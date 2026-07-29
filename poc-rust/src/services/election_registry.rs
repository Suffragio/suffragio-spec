use tonic::{Request, Response, Status};

use crate::app::{now_timestamp, AppState};
use crate::proto::{
    election_registry_server::{ElectionRegistry, ElectionRegistryServer},
    AddBsaPublicKeyRequest, AddBsaPublicKeyResponse, CreateElectionRequest, CreateElectionResponse,
    DefineBallotTemplateRequest, DefineBallotTemplateResponse, ElectionConfig, ElectionId,
    ElectionRegistryEvent, ElectionState, GetElectionRequest, GetElectionResponse,
    GetElectionSnapshotRequest, GetElectionSnapshotResponse, ListElectionsRequest,
    ListElectionsResponse, PublishElectionRequest, PublishElectionResponse, ScheduleElectionRequest,
    ScheduleElectionResponse, SetFormulaScriptRequest, SetFormulaScriptResponse,
    SetPublicTimestampsRequest, SetPublicTimestampsResponse, TransitionElectionStateRequest,
    TransitionElectionStateResponse, WatchEventsRequest,
};

pub struct ElectionRegistryService {
    state: AppState,
}

impl ElectionRegistryService {
    pub fn new(state: AppState) -> ElectionRegistryServer<Self> {
        ElectionRegistryServer::new(Self { state })
    }

    fn require_state(from: ElectionState, to: ElectionState) -> Result<(), Status> {
        let ok = matches!(
            (from, to),
            (ElectionState::Draft, ElectionState::Ready)
                | (ElectionState::Ready, ElectionState::Published)
                | (ElectionState::Published, ElectionState::Voting)
                | (ElectionState::Voting, ElectionState::Closed)
                | (ElectionState::Closed, ElectionState::Tallied)
                | (ElectionState::Tallied, ElectionState::ResultsPublished)
        );
        if !ok {
            return Err(Status::failed_precondition(format!(
                "invalid state transition: {from:?} -> {to:?}"
            )));
        }
        Ok(())
    }
}

#[tonic::async_trait]
impl ElectionRegistry for ElectionRegistryService {
    type WatchEventsStream =
        tokio_stream::wrappers::ReceiverStream<std::result::Result<ElectionRegistryEvent, Status>>;

    async fn create_election(
        &self,
        request: Request<CreateElectionRequest>,
    ) -> Result<Response<CreateElectionResponse>, Status> {
        let req = request.into_inner();
        let id = crate::app::election_id();
        let config = ElectionConfig {
            election_id: Some(ElectionId { value: id.clone() }),
            title: req.title,
            constituencies: req.constituencies,
            ballot_templates: vec![],
            formula_script: None,
            bsa_public_keys: vec![],
            voting_window: None,
            state: ElectionState::Draft as i32,
            published: false,
            publish_received_at: false,
        };
        self.state.inner.write().await.elections.insert(id.clone(), config);
        Ok(Response::new(CreateElectionResponse {
            election_id: Some(ElectionId { value: id }),
        }))
    }

    async fn define_ballot_template(
        &self,
        request: Request<DefineBallotTemplateRequest>,
    ) -> Result<Response<DefineBallotTemplateResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let template = req.template.as_ref().ok_or_else(|| Status::invalid_argument("template"))?;
        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get_mut(&election_id.value).ok_or_else(|| Status::not_found("election"))?;
        if config.state != ElectionState::Draft as i32 && config.state != ElectionState::Ready as i32 {
            return Err(Status::failed_precondition("ballot template immutable after PUBLISHED"));
        }
        config
            .ballot_templates
            .retain(|t| t.constituency_id != template.constituency_id);
        config.ballot_templates.push(template.clone());
        Ok(Response::new(DefineBallotTemplateResponse {}))
    }

    async fn set_formula_script(
        &self,
        request: Request<SetFormulaScriptRequest>,
    ) -> Result<Response<SetFormulaScriptResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let script = req.script.as_ref().ok_or_else(|| Status::invalid_argument("script"))?;
        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get_mut(&election_id.value).ok_or_else(|| Status::not_found("election"))?;
        if config.state != ElectionState::Draft as i32 {
            return Err(Status::failed_precondition("formula immutable after READY"));
        }
        config.formula_script = Some(script.clone());
        Ok(Response::new(SetFormulaScriptResponse {}))
    }

    async fn add_bsa_public_key(
        &self,
        request: Request<AddBsaPublicKeyRequest>,
    ) -> Result<Response<AddBsaPublicKeyResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let mut key = req.key.as_ref().ok_or_else(|| Status::invalid_argument("key"))?.clone();
        let election_id_val = election_id.value.clone();

        let priv_key = crate::crypto::generate_key().map_err(|e| Status::internal(e.to_string()))?;
        key.public_key = crate::crypto::serialize_public_key(&priv_key.to_public_key())
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get_mut(&election_id_val).ok_or_else(|| Status::not_found("election"))?;
        if config.state == ElectionState::Tallied as i32
            || config.state == ElectionState::ResultsPublished as i32
        {
            return Err(Status::failed_precondition("election ended"));
        }
        let key_id = key.key_id.clone();
        config.bsa_public_keys.push(key);
        let _ = config;
        inner
            .bsa_private_keys
            .entry(election_id_val)
            .or_default()
            .insert(key_id, priv_key);
        Ok(Response::new(AddBsaPublicKeyResponse {}))
    }

    async fn schedule_election(
        &self,
        request: Request<ScheduleElectionRequest>,
    ) -> Result<Response<ScheduleElectionResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let window = req.voting_window.as_ref().ok_or_else(|| Status::invalid_argument("voting_window"))?;
        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get_mut(&election_id.value).ok_or_else(|| Status::not_found("election"))?;
        if config.state != ElectionState::Draft as i32 && config.state != ElectionState::Ready as i32 {
            return Err(Status::failed_precondition("schedule immutable after PUBLISHED"));
        }
        config.voting_window = Some(window.clone());
        Ok(Response::new(ScheduleElectionResponse {}))
    }

    async fn set_public_timestamps(
        &self,
        request: Request<SetPublicTimestampsRequest>,
    ) -> Result<Response<SetPublicTimestampsResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get_mut(&election_id.value).ok_or_else(|| Status::not_found("election"))?;
        if config.state != ElectionState::Draft as i32 && config.state != ElectionState::Ready as i32 {
            return Err(Status::failed_precondition("immutable after PUBLISHED"));
        }
        config.publish_received_at = req.publish_received_at;
        Ok(Response::new(SetPublicTimestampsResponse {}))
    }

    async fn transition_election_state(
        &self,
        request: Request<TransitionElectionStateRequest>,
    ) -> Result<Response<TransitionElectionStateResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let to = ElectionState::try_from(req.to_state).map_err(|_| Status::invalid_argument("to_state"))?;
        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get_mut(&election_id.value).ok_or_else(|| Status::not_found("election"))?;
        let from = ElectionState::try_from(config.state).map_err(|_| Status::internal("bad state"))?;
        Self::require_state(from, to)?;
        config.state = to as i32;
        if to == ElectionState::Published {
            config.published = true;
        }
        Ok(Response::new(TransitionElectionStateResponse { state: to as i32 }))
    }

    async fn publish_election(
        &self,
        request: Request<PublishElectionRequest>,
    ) -> Result<Response<PublishElectionResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let mut inner = self.state.inner.write().await;
        let config = inner.elections.get_mut(&election_id.value).ok_or_else(|| Status::not_found("election"))?;
        let from = ElectionState::try_from(config.state).map_err(|_| Status::internal("bad state"))?;
        if from != ElectionState::Draft && from != ElectionState::Ready {
            return Err(Status::failed_precondition("already published"));
        }
        config.state = ElectionState::Published as i32;
        config.published = true;
        Ok(Response::new(PublishElectionResponse {}))
    }

    async fn get_election(
        &self,
        request: Request<GetElectionRequest>,
    ) -> Result<Response<GetElectionResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let config = inner.elections.get(&election_id.value).cloned().ok_or_else(|| Status::not_found("election"))?;
        Ok(Response::new(GetElectionResponse { election: Some(config) }))
    }

    async fn list_elections(
        &self,
        _request: Request<ListElectionsRequest>,
    ) -> Result<Response<ListElectionsResponse>, Status> {
        let inner = self.state.inner.read().await;
        let elections = inner.elections.values().cloned().collect();
        Ok(Response::new(ListElectionsResponse { elections, next_page_token: String::new() }))
    }

    async fn get_election_snapshot(
        &self,
        request: Request<GetElectionSnapshotRequest>,
    ) -> Result<Response<GetElectionSnapshotResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let config = inner.elections.get(&election_id.value).cloned().ok_or_else(|| Status::not_found("election"))?;
        Ok(Response::new(GetElectionSnapshotResponse {
            election: Some(config),
            cursor: None,
            captured_at: Some(now_timestamp()),
        }))
    }

    async fn watch_events(
        &self,
        _request: Request<WatchEventsRequest>,
    ) -> Result<Response<Self::WatchEventsStream>, Status> {
        let (_tx, rx) = tokio::sync::mpsc::channel::<
            std::result::Result<ElectionRegistryEvent, Status>,
        >(1);
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
