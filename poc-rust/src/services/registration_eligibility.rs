use tonic::{Request, Response, Status};

use crate::app::{now_timestamp, token_value, AppState, TokenRecord};
use crate::proto::{
    registration_eligibility_server::{RegistrationEligibility, RegistrationEligibilityServer},
    ConsumeEligibilityTokenRequest, ConsumeEligibilityTokenResponse, ConstituencyId,
    EligibilityToken, GetRegistrationSnapshotRequest, GetRegistrationSnapshotResponse,
    GetVoterStatusRequest, GetVoterStatusResponse, RegisterVoterRollRequest,
    RegisterVoterRollResponse, RegistrationEligibilityEvent, RevokeVotingRightsRequest,
    RevokeVotingRightsResponse, VerifyIdentityRequest, VerifyIdentityResponse, WatchEventsRequest,
};

pub struct RegistrationEligibilityService {
    state: AppState,
}

impl RegistrationEligibilityService {
    pub fn new(state: AppState) -> RegistrationEligibilityServer<Self> {
        RegistrationEligibilityServer::new(Self { state })
    }
}

fn expiry() -> prost_types::Timestamp {
    let t = chrono::Utc::now() + chrono::Duration::hours(1);
    prost_types::Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as i32,
    }
}

#[tonic::async_trait]
impl RegistrationEligibility for RegistrationEligibilityService {
    type WatchEventsStream =
        tokio_stream::wrappers::ReceiverStream<std::result::Result<RegistrationEligibilityEvent, Status>>;

    async fn register_voter_roll(
        &self,
        request: Request<RegisterVoterRollRequest>,
    ) -> Result<Response<RegisterVoterRollResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let mut inner = self.state.inner.write().await;
        let roll = inner.rolls.entry(election_id.value.clone()).or_default();
        let mut count = 0;
        for voter in req.voters.into_iter() {
            let cid = voter.constituency_id.as_ref().map(|c| c.value.clone()).unwrap_or_default();
            roll.insert(
                voter.voter_id,
                crate::app::VoterRecord {
                    constituency_id: cid,
                    revoked: false,
                },
            );
            count += 1;
        }
        Ok(Response::new(RegisterVoterRollResponse { registered_count: count }))
    }

    async fn verify_identity(
        &self,
        request: Request<VerifyIdentityRequest>,
    ) -> Result<Response<VerifyIdentityResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let proof = req.proof.as_ref().ok_or_else(|| Status::invalid_argument("proof"))?;
        let voter_id = String::from_utf8_lossy(&proof.assertion).to_string();
        if voter_id.is_empty() {
            return Err(Status::invalid_argument("empty voter identity"));
        }
        let mut inner = self.state.inner.write().await;
        let roll = inner.rolls.get(&election_id.value).ok_or_else(|| Status::not_found("roll"))?;
        let record = roll.get(&voter_id).ok_or_else(|| Status::permission_denied("not registered"))?;
        if record.revoked {
            return Err(Status::permission_denied("rights revoked"));
        }
        let token_val = token_value();
        let constituency_id = record.constituency_id.clone();
        let expires = expiry();
        inner
            .tokens
            .entry(election_id.value.clone())
            .or_default()
            .insert(
                token_val.clone(),
                TokenRecord {
                    constituency_id: constituency_id.clone(),
                    expires_at: Some(expires.clone()),
                    consumed: false,
                },
            );
        Ok(Response::new(VerifyIdentityResponse {
            token: Some(EligibilityToken { value: token_val }),
            constituency_id: Some(ConstituencyId { value: constituency_id }),
            expires_at: Some(expires),
        }))
    }

    async fn revoke_voting_rights(
        &self,
        request: Request<RevokeVotingRightsRequest>,
    ) -> Result<Response<RevokeVotingRightsResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let mut inner = self.state.inner.write().await;
        let roll = inner.rolls.entry(election_id.value.clone()).or_default();
        if let Some(rec) = roll.get_mut(&req.voter_id) {
            rec.revoked = true;
        }
        Ok(Response::new(RevokeVotingRightsResponse {}))
    }

    async fn consume_eligibility_token(
        &self,
        request: Request<ConsumeEligibilityTokenRequest>,
    ) -> Result<Response<ConsumeEligibilityTokenResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let token = req.token.as_ref().ok_or_else(|| Status::invalid_argument("token"))?;
        let mut inner = self.state.inner.write().await;
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
        Ok(Response::new(ConsumeEligibilityTokenResponse {
            constituency_id: Some(ConstituencyId { value: record.constituency_id.clone() }),
            expires_at: record.expires_at.clone(),
        }))
    }

    async fn get_voter_status(
        &self,
        request: Request<GetVoterStatusRequest>,
    ) -> Result<Response<GetVoterStatusResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let roll = inner.rolls.get(&election_id.value);
        let (registered, revoked, constituency_id) = match roll.and_then(|r| r.get(&req.voter_id)) {
            Some(rec) => (true, rec.revoked, rec.constituency_id.clone()),
            None => (false, false, String::new()),
        };
        Ok(Response::new(GetVoterStatusResponse {
            registered,
            eligible: registered && !revoked,
            revoked,
            constituency_id: Some(ConstituencyId { value: constituency_id }),
        }))
    }

    async fn get_registration_snapshot(
        &self,
        request: Request<GetRegistrationSnapshotRequest>,
    ) -> Result<Response<GetRegistrationSnapshotResponse>, Status> {
        let req = request.into_inner();
        let election_id = req.election_id.as_ref().ok_or_else(|| Status::invalid_argument("election_id"))?;
        let inner = self.state.inner.read().await;
        let roll = inner.rolls.get(&election_id.value);
        let tokens = inner.tokens.get(&election_id.value);
        let registered_count = roll.map(|r| r.len() as i32).unwrap_or(0);
        let revoked_count = roll.map(|r| r.values().filter(|v| v.revoked).count() as i32).unwrap_or(0);
        let tokens_issued_count = tokens.map(|t| t.len() as i32).unwrap_or(0);
        let tokens_consumed_count = tokens
            .map(|t| t.values().filter(|v| v.consumed).count() as i32)
            .unwrap_or(0);
        Ok(Response::new(GetRegistrationSnapshotResponse {
            cursor: None,
            captured_at: Some(now_timestamp()),
            registered_count,
            revoked_count,
            tokens_issued_count,
            tokens_consumed_count,
        }))
    }

    async fn watch_events(
        &self,
        _request: Request<WatchEventsRequest>,
    ) -> Result<Response<Self::WatchEventsStream>, Status> {
        let (_tx, rx) = tokio::sync::mpsc::channel::<
            std::result::Result<RegistrationEligibilityEvent, Status>,
        >(1);
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
