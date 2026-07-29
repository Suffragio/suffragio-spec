use tonic::{Request, Response, Status};

use crate::app::{hash_bytes, now_timestamp, AppState};
use crate::proto::{
    formula_catalog_server::{FormulaCatalog, FormulaCatalogServer},
    FormulaCatalogEvent, FormulaScriptMetadata, GetFormulaCatalogSnapshotRequest,
    GetFormulaCatalogSnapshotResponse, GetScriptRequest, GetScriptResponse, ListScriptsRequest,
    ListScriptsResponse, PublishScriptRequest, PublishScriptResponse, WatchEventsRequest,
};

pub struct FormulaCatalogService {
    state: AppState,
}

impl FormulaCatalogService {
    pub fn new(state: AppState) -> FormulaCatalogServer<Self> {
        FormulaCatalogServer::new(Self { state })
    }
}

#[tonic::async_trait]
impl FormulaCatalog for FormulaCatalogService {
    type WatchEventsStream =
        tokio_stream::wrappers::ReceiverStream<std::result::Result<FormulaCatalogEvent, Status>>;

    async fn publish_script(
        &self,
        request: Request<PublishScriptRequest>,
    ) -> Result<Response<PublishScriptResponse>, Status> {
        let req = request.into_inner();
        let script_id = uuid::Uuid::new_v4().to_string();
        let content_hash = hash_bytes(&req.script);
        let metadata = FormulaScriptMetadata {
            script_id: script_id.clone(),
            title: req.title,
            description: req.description,
            tags: req.tags,
            content_hash: content_hash.clone(),
            published_at: Some(now_timestamp()),
        };
        let mut inner = self.state.inner.write().await;
        inner.formula_catalog.insert(script_id.clone(), metadata);
        inner.formula_scripts.insert(script_id.clone(), req.script);
        Ok(Response::new(PublishScriptResponse {
            script_id,
            content_hash,
        }))
    }

    async fn get_script(
        &self,
        request: Request<GetScriptRequest>,
    ) -> Result<Response<GetScriptResponse>, Status> {
        let req = request.into_inner();
        let inner = self.state.inner.read().await;
        let metadata = inner
            .formula_catalog
            .get(&req.script_id)
            .cloned()
            .ok_or_else(|| Status::not_found("script"))?;
        let script = inner
            .formula_scripts
            .get(&req.script_id)
            .cloned()
            .unwrap_or_default();
        if !req.expected_content_hash.is_empty() && req.expected_content_hash != metadata.content_hash {
            return Err(Status::not_found("hash mismatch"));
        }
        Ok(Response::new(GetScriptResponse {
            metadata: Some(metadata),
            script,
        }))
    }

    async fn list_scripts(
        &self,
        request: Request<ListScriptsRequest>,
    ) -> Result<Response<ListScriptsResponse>, Status> {
        let req = request.into_inner();
        let inner = self.state.inner.read().await;
        let query = req.query.to_lowercase();
        let tag = req.tag.to_lowercase();
        let scripts: Vec<_> = inner
            .formula_catalog
            .values()
            .filter(|m| {
                (query.is_empty()
                    || m.title.to_lowercase().contains(&query)
                    || m.description.to_lowercase().contains(&query))
                    && (tag.is_empty() || m.tags.iter().any(|t| t.to_lowercase() == tag))
            })
            .cloned()
            .collect();
        Ok(Response::new(ListScriptsResponse {
            scripts,
            next_page_token: String::new(),
        }))
    }

    async fn get_formula_catalog_snapshot(
        &self,
        _request: Request<GetFormulaCatalogSnapshotRequest>,
    ) -> Result<Response<GetFormulaCatalogSnapshotResponse>, Status> {
        let inner = self.state.inner.read().await;
        let scripts = inner.formula_catalog.values().cloned().collect();
        Ok(Response::new(GetFormulaCatalogSnapshotResponse {
            cursor: None,
            scripts,
            captured_at: Some(now_timestamp()),
        }))
    }

    async fn watch_events(
        &self,
        _request: Request<WatchEventsRequest>,
    ) -> Result<Response<Self::WatchEventsStream>, Status> {
        let (_tx, rx) = tokio::sync::mpsc::channel::<
            std::result::Result<FormulaCatalogEvent, Status>,
        >(1);
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
