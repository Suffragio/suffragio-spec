use tonic::{Request, Response, Status};

use crate::app::{hash_bytes, now_timestamp, AppState};
use crate::ballot;
use crate::proto::{
    tally_engine_server::{TallyEngine, TallyEngineServer},
    CloseVotingWindowRequest, CloseVotingWindowResponse, ComputeResultsRequest, ComputeResultsResponse,
    ElectionState, GetOfficialResultsPackageRequest, GetOfficialResultsPackageResponse,
    GetResultsRequest, GetResultsResponse, GetTallySnapshotRequest, GetTallySnapshotResponse,
    PublishResultsRequest, PublishResultsResponse, TallyEvent, WatchEventsRequest,
};

pub struct TallyEngineService {
    state: AppState,
}

impl TallyEngineService {
    pub fn new(state: AppState) -> TallyEngineServer<Self> {
        TallyEngineServer::new(Self { state })
    }
}

#[tonic::async_trait]
impl TallyEngine for TallyEngineService {
    type WatchEventsStream =
        tokio_stream::wrappers::ReceiverStream<std::result::Result<TallyEvent, Status>>;

    async fn close_voting_window(
        &self,
        request: Request<CloseVotingWindowRequest>,
    ) -> Result<Response<CloseVotingWindowResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get_mut(&election_id.value).ok_or_else(|| Status::not_found("election"))?;
        let state = ElectionState::try_from(config.state).map_err(|_| Status::internal("bad state"))?;
        if state != ElectionState::Voting {
            return Err(Status::failed_precondition("voting not open"));
        }
        config.state = ElectionState::Closed as i32;
        inner.tally_closed.insert(election_id.value.clone());
        Ok(Response::new(CloseVotingWindowResponse {}))
    }

    async fn compute_results(
        &self,
        request: Request<ComputeResultsRequest>,
    ) -> Result<Response<ComputeResultsResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let config = inner.elections.get(&election_id.value).cloned().ok_or_else(|| Status::not_found("election"))?;
        let state = ElectionState::try_from(config.state).map_err(|_| Status::internal("bad state"))?;
        if state != ElectionState::Closed {
            return Err(Status::failed_precondition("voting not closed"));
        }
        let formula = config.formula_script.as_ref().ok_or_else(|| Status::failed_precondition("no formula"))?;
        let script = if !formula.inline_script.is_empty() {
            formula.inline_script.clone()
        } else {
            inner
                .formula_scripts
                .get(&formula.catalog_script_id)
                .cloned()
                .ok_or_else(|| Status::not_found("catalog script"))?
        };
        let computed_hash = hash_bytes(&script);
        if computed_hash != formula.content_hash {
            return Err(Status::failed_precondition("formula content_hash mismatch"));
        }
        let log = inner.vote_logs.get(&election_id.value).cloned().unwrap_or_default();
        if !req.log_head_hash.is_empty() && log.head_hash != req.log_head_hash {
            return Err(Status::failed_precondition("log head mismatch"));
        }

        let mut ballots = Vec::new();
        for entry in log.entries.iter() {
            let cid = entry
                .constituency_id
                .as_ref()
                .map(|c| c.value.clone())
                .unwrap_or_default();
            let filled = ballot::decode(&entry.ballot)
                .map_err(|e| Status::internal(format!("decode ballot: {e}")))?;
            ballots.push((cid, filled));
        }
        drop(inner);

        let results = crate::tally::run_formula(
            &script,
            &election_id.value,
            ballots,
            0,
            formula.content_hash.clone(),
            req.log_head_hash,
        )
        .map_err(|e| Status::internal(format!("tally error: {e}")))?;

        self.state.inner.write().await.results.insert(election_id.value.clone(), results.clone());
        Ok(Response::new(ComputeResultsResponse { results: Some(results) }))
    }

    async fn publish_results(
        &self,
        request: Request<PublishResultsRequest>,
    ) -> Result<Response<PublishResultsResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let package = req.package.as_ref().ok_or_else(|| Status::invalid_argument("package"))?;
        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get_mut(&election_id.value).ok_or_else(|| Status::not_found("election"))?;
        config.state = ElectionState::ResultsPublished as i32;
        inner.tally_published.insert(election_id.value.clone());
        inner
            .official_packages
            .insert(election_id.value.clone(), package.clone());
        Ok(Response::new(PublishResultsResponse {}))
    }

    async fn get_results(
        &self,
        request: Request<GetResultsRequest>,
    ) -> Result<Response<GetResultsResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let published = inner.tally_published.contains(&election_id.value);
        let results = inner.results.get(&election_id.value).cloned();
        Ok(Response::new(GetResultsResponse { results, published }))
    }

    async fn get_official_results_package(
        &self,
        request: Request<GetOfficialResultsPackageRequest>,
    ) -> Result<Response<GetOfficialResultsPackageResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let package = inner.official_packages.get(&election_id.value).cloned();
        Ok(Response::new(GetOfficialResultsPackageResponse { package }))
    }

    async fn get_tally_snapshot(
        &self,
        request: Request<GetTallySnapshotRequest>,
    ) -> Result<Response<GetTallySnapshotResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let closed = inner.tally_closed.contains(&election_id.value);
        let published = inner.tally_published.contains(&election_id.value);
        let log_head_hash = inner
            .vote_logs
            .get(&election_id.value)
            .map(|l| l.head_hash.clone())
            .unwrap_or_default();
        Ok(Response::new(GetTallySnapshotResponse {
            cursor: None,
            closed,
            published,
            log_head_hash,
            captured_at: Some(now_timestamp()),
        }))
    }

    async fn watch_events(
        &self,
        _request: Request<WatchEventsRequest>,
    ) -> Result<Response<Self::WatchEventsStream>, Status> {
        let (_tx, rx) = tokio::sync::mpsc::channel::<std::result::Result<TallyEvent, Status>>(1);
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
