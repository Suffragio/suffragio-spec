use rsa::BigUint;
use tonic::{Request, Response, Status};

use crate::app::{now_timestamp, AppState};
use crate::crypto::SUITE_ID;
use crate::proto::{
    blind_signature_authority_server::{BlindSignatureAuthority, BlindSignatureAuthorityServer},
    BlindSignatureEvent, GetBlindSignatureSnapshotRequest, GetBlindSignatureSnapshotResponse,
    RequestBlindSignatureRequest, RequestBlindSignatureResponse, WatchEventsRequest,
};

pub struct BlindSignatureAuthorityService {
    state: AppState,
}

impl BlindSignatureAuthorityService {
    pub fn new(state: AppState) -> BlindSignatureAuthorityServer<Self> {
        BlindSignatureAuthorityServer::new(Self { state })
    }
}

#[tonic::async_trait]
impl BlindSignatureAuthority for BlindSignatureAuthorityService {
    type WatchEventsStream =
        tokio_stream::wrappers::ReceiverStream<std::result::Result<BlindSignatureEvent, Status>>;

    async fn request_blind_signature(
        &self,
        request: Request<RequestBlindSignatureRequest>,
    ) -> Result<Response<RequestBlindSignatureResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let token = req.token.as_ref().ok_or_else(|| Status::invalid_argument("token"))?;
        if req.suite_id.as_ref().map(|s| s.value.as_str()) != Some(SUITE_ID) {
            return Err(Status::invalid_argument("unsupported suite_id"));
        }

        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get(&election_id.value).ok_or_else(|| Status::not_found("election"))?.clone();
        let state = crate::proto::ElectionState::try_from(config.state).map_err(|_| Status::internal("bad state"))?;
        if state != crate::proto::ElectionState::Published
            && state != crate::proto::ElectionState::Voting
            && state != crate::proto::ElectionState::Closed
        {
            return Err(Status::failed_precondition("election not open for signing"));
        }

        let tokens = inner.tokens.get_mut(&election_id.value).ok_or_else(|| Status::not_found("tokens"))?;
        let record = tokens.get_mut(&token.value).ok_or_else(|| Status::not_found("token"))?;
        if record.consumed {
            return Err(Status::already_exists("token already consumed"));
        }
        if let Some(exp) = record.expires_at.as_ref() {
            if crate::app::timestamp_to_chrono(exp) <= Some(chrono::Utc::now()) {
                return Err(Status::deadline_exceeded("token expired"));
            }
        }
        record.consumed = true;

        let priv_map = inner
            .bsa_private_keys
            .get(&election_id.value)
            .ok_or_else(|| Status::failed_precondition("no BSA keys for election"))?;
        let priv_key = priv_map
            .get(&req.key_id)
            .ok_or_else(|| Status::not_found("key_id"))?;

        let blinded = BigUint::from_bytes_be(&req.blinded_ballot);
        let signature = crate::crypto::sign(priv_key, &blinded);

        Ok(Response::new(RequestBlindSignatureResponse {
            blind_signature: signature.to_bytes_be(),
            key_id: req.key_id,
        }))
    }

    async fn get_blind_signature_snapshot(
        &self,
        _request: Request<GetBlindSignatureSnapshotRequest>,
    ) -> Result<Response<GetBlindSignatureSnapshotResponse>, Status> {
        Ok(Response::new(GetBlindSignatureSnapshotResponse {
            cursor: None,
            captured_at: Some(now_timestamp()),
            signatures_issued_count: 0,
        }))
    }

    async fn watch_events(
        &self,
        _request: Request<WatchEventsRequest>,
    ) -> Result<Response<Self::WatchEventsStream>, Status> {
        let (_tx, rx) = tokio::sync::mpsc::channel::<
            std::result::Result<BlindSignatureEvent, Status>,
        >(1);
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
