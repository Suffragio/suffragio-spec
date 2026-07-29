use rsa::BigUint;
use serde::Serialize;
use tonic::{Request, Response, Status};

use crate::app::{hash_bytes, now_timestamp, AppState};
use crate::ballot;
use crate::crypto::SUITE_ID;
use crate::proto::{
    vote_broadcast_queue_server::{VoteBroadcastQueue, VoteBroadcastQueueServer},
    GetLogHeadRequest, GetLogHeadResponse, GetVoteQueueSnapshotRequest, GetVoteQueueSnapshotResponse,
    ReportLogHeadRequest, ReportLogHeadResponse, SignedVote, StreamVotesRequest, SubmitVoteRequest,
    SubmitVoteResponse, VoteQueueEvent, WatchEventsRequest,
};

pub struct VoteBroadcastQueueService {
    state: AppState,
}

impl VoteBroadcastQueueService {
    pub fn new(state: AppState) -> VoteBroadcastQueueServer<Self> {
        VoteBroadcastQueueServer::new(Self { state })
    }
}

#[derive(Serialize)]
struct CanonicalVote {
    election_id: String,
    constituency_id: String,
    ballot: Vec<u8>,
    signature: Vec<u8>,
    key_id: String,
    suite_id: String,
}

#[tonic::async_trait]
impl VoteBroadcastQueue for VoteBroadcastQueueService {
    type StreamVotesStream =
        tokio_stream::wrappers::ReceiverStream<std::result::Result<SignedVote, Status>>;
    type WatchEventsStream =
        tokio_stream::wrappers::ReceiverStream<std::result::Result<VoteQueueEvent, Status>>;

    async fn submit_vote(
        &self,
        request: Request<SubmitVoteRequest>,
    ) -> Result<Response<SubmitVoteResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let constituency_id = req.constituency_id.as_ref().ok_or_else(|| Status::invalid_argument("constituency_id"))?;
        let suite_id = req.suite_id.as_ref().ok_or_else(|| Status::invalid_argument("suite_id"))?;
        if suite_id.value != SUITE_ID {
            return Err(Status::invalid_argument("unsupported suite_id"));
        }

        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get(&election_id.value).cloned().ok_or_else(|| Status::not_found("election"))?;
        let state = crate::proto::ElectionState::try_from(config.state).map_err(|_| Status::internal("bad state"))?;
        if state != crate::proto::ElectionState::Voting {
            return Err(Status::failed_precondition("voting not open"));
        }

        let bsa_key = config
            .bsa_public_keys
            .iter()
            .find(|k| k.key_id == req.key_id)
            .ok_or_else(|| Status::not_found("key_id"))?;
        if bsa_key.suite_id.as_ref().map(|s| s.value.as_str()) != Some(SUITE_ID) {
            return Err(Status::invalid_argument("key suite mismatch"));
        }

        let pub_key = crate::crypto::deserialize_public_key(&bsa_key.public_key)
            .map_err(|e| Status::internal(format!("bad public key: {e}")))?;
        let signature = BigUint::from_bytes_be(&req.signature);
        if !crate::crypto::verify(&pub_key, &req.ballot, &election_id.value, &signature) {
            return Err(Status::permission_denied("invalid blind signature"));
        }

        let ballot = ballot::decode(&req.ballot).map_err(|e| Status::invalid_argument(format!("ballot cbor: {e}")))?;
        let template = config
            .ballot_templates
            .iter()
            .find(|t| {
                t.constituency_id.as_ref().map(|c| c.value.as_str()) == Some(&constituency_id.value)
            })
            .ok_or_else(|| Status::not_found("ballot template for constituency"))?;
        ballot::validate(&ballot, template).map_err(|e| Status::invalid_argument(format!("ballot invalid: {e}")))?;

        let log = inner.vote_logs.entry(election_id.value.clone()).or_default();
        let duplicate = log.entries.iter().any(|e| e.ballot == req.ballot && e.signature == req.signature);
        if duplicate {
            return Err(Status::already_exists("duplicate vote"));
        }

        let canonical = CanonicalVote {
            election_id: election_id.value.clone(),
            constituency_id: constituency_id.value.clone(),
            ballot: req.ballot.clone(),
            signature: req.signature.clone(),
            key_id: req.key_id.clone(),
            suite_id: suite_id.value.clone(),
        };
        let canonical_bytes = cbor4ii::serde::to_vec(Vec::new(), &canonical)
            .map_err(|e| Status::internal(format!("canonical encode: {e}")))?;
        let prev_hash = log.head_hash.clone();
        let mut preimage = prev_hash.clone();
        preimage.extend_from_slice(&canonical_bytes);
        let entry_hash = hash_bytes(&preimage);
        let sequence = log.sequence + 1;

        let mut vote = SignedVote {
            election_id: Some(election_id.clone()),
            constituency_id: Some(constituency_id.clone()),
            ballot: req.ballot,
            signature: req.signature,
            key_id: req.key_id,
            suite_id: Some(suite_id.clone()),
            sequence,
            entry_hash: entry_hash.clone(),
            prev_hash: prev_hash.clone(),
            received_at: None,
        };
        if config.publish_received_at {
            vote.received_at = Some(now_timestamp());
        }

        log.entries.push(vote);
        log.head_hash = entry_hash.clone();
        log.sequence = sequence;

        Ok(Response::new(SubmitVoteResponse {
            sequence,
            entry_hash,
            prev_hash,
        }))
    }

    async fn stream_votes(
        &self,
        request: Request<StreamVotesRequest>,
    ) -> Result<Response<Self::StreamVotesStream>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let log = inner.vote_logs.get(&election_id.value).cloned().unwrap_or_default();
        let after = req.after_sequence;
        let (tx, rx) = tokio::sync::mpsc::channel::<std::result::Result<SignedVote, Status>>(128);
        for vote in log.entries.into_iter().skip(after as usize) {
            let _ = tx.send(Ok(vote)).await;
        }
        drop(tx);
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    async fn get_log_head(
        &self,
        request: Request<GetLogHeadRequest>,
    ) -> Result<Response<GetLogHeadResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let log = inner.vote_logs.get(&election_id.value).cloned().unwrap_or_default();
        Ok(Response::new(GetLogHeadResponse {
            head_hash: log.head_hash,
            sequence: log.sequence,
            observed_at: Some(now_timestamp()),
        }))
    }

    async fn report_log_head(
        &self,
        _request: Request<ReportLogHeadRequest>,
    ) -> Result<Response<ReportLogHeadResponse>, Status> {
        Ok(Response::new(ReportLogHeadResponse {}))
    }

    async fn get_vote_queue_snapshot(
        &self,
        request: Request<GetVoteQueueSnapshotRequest>,
    ) -> Result<Response<GetVoteQueueSnapshotResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let log = inner.vote_logs.get(&election_id.value).cloned().unwrap_or_default();
        Ok(Response::new(GetVoteQueueSnapshotResponse {
            cursor: None,
            head_hash: log.head_hash,
            sequence: log.sequence,
            captured_at: Some(now_timestamp()),
        }))
    }

    async fn watch_events(
        &self,
        _request: Request<WatchEventsRequest>,
    ) -> Result<Response<Self::WatchEventsStream>, Status> {
        let (_tx, rx) =
            tokio::sync::mpsc::channel::<std::result::Result<VoteQueueEvent, Status>>(1);
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
