use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::proto::{ElectionConfig, SignedVote};

#[derive(Clone, Debug, Default)]
pub struct VoterRecord {
    pub constituency_id: String,
    pub revoked: bool,
}

#[derive(Clone, Debug)]
pub struct TokenRecord {
    pub constituency_id: String,
    pub expires_at: Option<prost_types::Timestamp>,
    pub consumed: bool,
}

#[derive(Clone, Debug, Default)]
pub struct VoteLog {
    pub entries: Vec<SignedVote>,
    pub head_hash: Vec<u8>,
    pub sequence: u64,
}

#[derive(Default)]
pub struct AppStateInner {
    pub elections: HashMap<String, ElectionConfig>,
    pub rolls: HashMap<String, HashMap<String, VoterRecord>>,
    pub tokens: HashMap<String, HashMap<String, TokenRecord>>,
    pub vote_logs: HashMap<String, VoteLog>,
    pub results: HashMap<String, crate::proto::ElectionResults>,
    pub official_packages: HashMap<String, crate::proto::OfficialResultsPackage>,
    pub tally_closed: HashSet<String>,
    pub tally_published: HashSet<String>,
    pub formula_catalog: HashMap<String, crate::proto::FormulaScriptMetadata>,
    pub formula_scripts: HashMap<String, Vec<u8>>,
    pub nodes: Vec<crate::proto::NodeInfo>,
    pub idempotency: HashMap<String, Vec<u8>>,
    pub bsa_private_keys: HashMap<String, HashMap<String, rsa::RsaPrivateKey>>,
}

#[derive(Clone, Default)]
pub struct AppState {
    pub inner: Arc<RwLock<AppStateInner>>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn serve(self, addr: std::net::SocketAddr) -> anyhow::Result<()> {
        let registry = crate::services::election_registry::ElectionRegistryService::new(self.clone());
        let registration =
            crate::services::registration_eligibility::RegistrationEligibilityService::new(
                self.clone(),
            );
        let bsa = crate::services::blind_signature::BlindSignatureAuthorityService::new(self.clone());
        let queue = crate::services::vote_queue::VoteBroadcastQueueService::new(self.clone());
        let tally = crate::services::tally::TallyEngineService::new(self.clone());
        let catalog = crate::services::formula_catalog::FormulaCatalogService::new(self.clone());
        let discovery = crate::services::discovery::DiscoveryService::new(self.clone());

        tonic::transport::Server::builder()
            .add_service(registry)
            .add_service(registration)
            .add_service(bsa)
            .add_service(queue)
            .add_service(tally)
            .add_service(catalog)
            .add_service(discovery)
            .serve(addr)
            .await?;
        Ok(())
    }
}

pub fn now_timestamp() -> prost_types::Timestamp {
    let now = chrono::Utc::now();
    prost_types::Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}

pub fn timestamp_to_chrono(ts: &prost_types::Timestamp) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::from_timestamp(ts.seconds, ts.nanos as u32)
}

pub fn election_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn token_value() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn hash_bytes(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
