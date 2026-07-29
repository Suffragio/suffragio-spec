use tonic::{Request, Response, Status};

use crate::app::{now_timestamp, AppState};
use crate::proto::{
    discovery_server::{Discovery, DiscoveryServer},
    AnnounceNodeRequest, AnnounceNodeResponse, DiscoverElectionsRequest, DiscoverElectionsResponse,
    DiscoveryEvent, ElectionSummary, GetDiscoverySnapshotRequest, GetDiscoverySnapshotResponse,
    WatchEventsRequest,
};

pub struct DiscoveryService {
    state: AppState,
}

impl DiscoveryService {
    pub fn new(state: AppState) -> DiscoveryServer<Self> {
        DiscoveryServer::new(Self { state })
    }
}

#[tonic::async_trait]
impl Discovery for DiscoveryService {
    type WatchEventsStream =
        tokio_stream::wrappers::ReceiverStream<std::result::Result<DiscoveryEvent, Status>>;

    async fn announce_node(
        &self,
        request: Request<AnnounceNodeRequest>,
    ) -> Result<Response<AnnounceNodeResponse>, Status> {
        let req = request.into_inner();
        let node = req.node.as_ref().ok_or_else(|| Status::invalid_argument("node"))?;
        let mut inner = self.state.inner.write().await;
        inner.nodes.push(node.clone());
        Ok(Response::new(AnnounceNodeResponse {
            tracker: node.tracker.clone(),
        }))
    }

    async fn discover_elections(
        &self,
        _request: Request<DiscoverElectionsRequest>,
    ) -> Result<Response<DiscoverElectionsResponse>, Status> {
        let inner = self.state.inner.read().await;
        let elections: Vec<ElectionSummary> = inner
            .elections
            .values()
            .map(|e| ElectionSummary {
                election_id: e.election_id.clone(),
                title: e.title.clone(),
                constituencies: e.constituencies.clone(),
                voting_window: e.voting_window.clone(),
                published: e.published,
                tracker: None,
                state: e.state,
            })
            .collect();
        Ok(Response::new(DiscoverElectionsResponse {
            elections,
            next_page_token: String::new(),
            tracker: None,
        }))
    }

    async fn get_discovery_snapshot(
        &self,
        _request: Request<GetDiscoverySnapshotRequest>,
    ) -> Result<Response<GetDiscoverySnapshotResponse>, Status> {
        let inner = self.state.inner.read().await;
        let nodes = inner.nodes.clone();
        Ok(Response::new(GetDiscoverySnapshotResponse {
            cursor: None,
            nodes,
            captured_at: Some(now_timestamp()),
        }))
    }

    async fn watch_events(
        &self,
        _request: Request<WatchEventsRequest>,
    ) -> Result<Response<Self::WatchEventsStream>, Status> {
        let (_tx, rx) =
            tokio::sync::mpsc::channel::<std::result::Result<DiscoveryEvent, Status>>(1);
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
