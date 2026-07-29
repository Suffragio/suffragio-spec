use std::collections::BTreeMap;
use std::time::Duration;

use suffragio_poc::app::AppState;
use suffragio_poc::ballot::FilledBallot;
use suffragio_poc::crypto::SUITE_ID;
use suffragio_poc::proto::{
    blind_signature_authority_client::BlindSignatureAuthorityClient,
    election_registry_client::ElectionRegistryClient,
    registration_eligibility_client::RegistrationEligibilityClient,
    tally_engine_client::TallyEngineClient,
    vote_broadcast_queue_client::VoteBroadcastQueueClient,
    AddBsaPublicKeyRequest, BsaPublicKey, CloseVotingWindowRequest, ComputeResultsRequest,
    Constituency, ConstituencyId, CreateElectionRequest, CryptoSuiteId, DefineBallotTemplateRequest,
    FormulaScriptRef, IdentityProof, OfficialResultsPackage, PublishElectionRequest,
    PublishResultsRequest, RegisterVoterRollRequest, RequestBlindSignatureRequest,
    SetFormulaScriptRequest, SubmitVoteRequest, TransitionElectionStateRequest, VerifyIdentityRequest,
    VoterEntry,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "[::1]:50051".parse()?;
    let state = AppState::new();

    let server_state = state.clone();
    let server = tokio::spawn(async move {
        tracing::info!("Suffragio POC server listening on {}", addr);
        server_state.serve(addr).await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(200)).await;

    let mut registry = ElectionRegistryClient::connect("http://[::1]:50051").await?;
    let mut reg = RegistrationEligibilityClient::connect("http://[::1]:50051").await?;
    let mut bsa = BlindSignatureAuthorityClient::connect("http://[::1]:50051").await?;
    let mut queue = VoteBroadcastQueueClient::connect("http://[::1]:50051").await?;
    let mut tally = TallyEngineClient::connect("http://[::1]:50051").await?;

    let election = registry
        .create_election(CreateElectionRequest {
            title: "POC Presidential Election".to_string(),
            constituencies: vec![Constituency {
                id: Some(ConstituencyId {
                    value: "district-1".to_string(),
                }),
                name: "District 1".to_string(),
            }],
        })
        .await?
        .into_inner();
    let election_id = election.election_id.unwrap();
    tracing::info!("created election {}", election_id.value);

    registry
        .define_ballot_template(DefineBallotTemplateRequest {
            election_id: Some(election_id.clone()),
            template: Some(suffragio_poc::proto::BallotTemplate {
                constituency_id: Some(ConstituencyId {
                    value: "district-1".to_string(),
                }),
                dsl_version: "suffragio-ballot-dsl/1".to_string(),
                document_json: r#"{
                    "questions": [
                        {
                            "id": "q_president",
                            "type": "single_choice",
                            "required": true,
                            "label": "President",
                            "options": [
                                {"id": "cand_a", "label": "Candidate A"},
                                {"id": "cand_b", "label": "Candidate B"}
                            ]
                        }
                    ]
                }"#
                .to_string(),
            }),
        })
        .await?;

    let formula = r#"return function(ballots, election_id)
  local results = {}
  for _, b in ipairs(ballots) do
    local cid = b.constituency_id
    local choices = b.ballot["q_president"]
    if choices and #choices == 1 then
      local opt = choices[1]
      results[cid] = results[cid] or {}
      results[cid][opt] = (results[cid][opt] or 0) + 1
    end
  end
  local out = {}
  for cid, tally in pairs(results) do
    table.insert(out, { constituency_id = cid, tally_by_choice = tally })
  end
  return { constituency_results = out, total_votes_counted = #ballots }
end"#;
    let formula_hash = suffragio_poc::app::hash_bytes(formula.as_bytes());
    registry
        .set_formula_script(SetFormulaScriptRequest {
            election_id: Some(election_id.clone()),
            script: Some(FormulaScriptRef {
                content_hash: formula_hash.clone(),
                inline_script: formula.as_bytes().to_vec(),
                catalog_script_id: String::new(),
            }),
        })
        .await?;

    registry
        .add_bsa_public_key(AddBsaPublicKeyRequest {
            election_id: Some(election_id.clone()),
            key: Some(BsaPublicKey {
                key_id: "bsa-1".to_string(),
                suite_id: Some(CryptoSuiteId {
                    value: SUITE_ID.to_string(),
                }),
                public_key: vec![],
                not_before: None,
                not_after: None,
            }),
        })
        .await?;

    registry
        .publish_election(PublishElectionRequest {
            election_id: Some(election_id.clone()),
        })
        .await?;
    registry
        .transition_election_state(TransitionElectionStateRequest {
            election_id: Some(election_id.clone()),
            to_state: suffragio_poc::proto::ElectionState::Voting as i32,
        })
        .await?;
    tracing::info!("election open for voting");

    reg.register_voter_roll(RegisterVoterRollRequest {
        election_id: Some(election_id.clone()),
        voters: vec![VoterEntry {
            voter_id: "voter-42".to_string(),
            constituency_id: Some(ConstituencyId {
                value: "district-1".to_string(),
            }),
        }],
    })
    .await?;

    let token_resp = reg
        .verify_identity(VerifyIdentityRequest {
            election_id: Some(election_id.clone()),
            proof: Some(IdentityProof {
                adapter: "stub".to_string(),
                assertion: "voter-42".as_bytes().to_vec(),
            }),
            auth_session_ref: String::new(),
        })
        .await?
        .into_inner();
    let token = token_resp.token.unwrap();
    let constituency_id = token_resp.constituency_id.unwrap();
    tracing::info!("got eligibility token");

    let config = registry
        .get_election(suffragio_poc::proto::GetElectionRequest {
            election_id: Some(election_id.clone()),
        })
        .await?
        .into_inner()
        .election
        .unwrap();
    let bsa_key = config.bsa_public_keys.first().unwrap();
    let pub_key = suffragio_poc::crypto::deserialize_public_key(&bsa_key.public_key)?;

    let mut ballot: FilledBallot = BTreeMap::new();
    ballot.insert("q_president".to_string(), vec!["cand_a".to_string()]);
    let ballot_cbor = suffragio_poc::ballot::encode(&ballot)?;

    let mut rng = rand::thread_rng();
    let blinded = suffragio_poc::crypto::blind(&pub_key, &ballot_cbor, &election_id.value, &mut rng);

    let blind_sig = bsa
        .request_blind_signature(RequestBlindSignatureRequest {
            election_id: Some(election_id.clone()),
            token: Some(token.clone()),
            blinded_ballot: blinded.blinded.to_bytes_be(),
            key_id: bsa_key.key_id.clone(),
            suite_id: Some(CryptoSuiteId {
                value: SUITE_ID.to_string(),
            }),
        })
        .await?
        .into_inner();

    let signature = suffragio_poc::crypto::unblind(
        &pub_key,
        &rsa::BigUint::from_bytes_be(&blind_sig.blind_signature),
        &blinded.r,
    )?;
    assert!(suffragio_poc::crypto::verify(
        &pub_key,
        &ballot_cbor,
        &election_id.value,
        &signature
    ));
    tracing::info!("blind signature verified");

    let submit = queue
        .submit_vote(SubmitVoteRequest {
            election_id: Some(election_id.clone()),
            constituency_id: Some(constituency_id.clone()),
            ballot: ballot_cbor,
            signature: signature.to_bytes_be(),
            key_id: bsa_key.key_id.clone(),
            suite_id: Some(CryptoSuiteId {
                value: SUITE_ID.to_string(),
            }),
            prev_hash: vec![],
        })
        .await?
        .into_inner();
    tracing::info!("vote submitted at sequence {}", submit.sequence);

    tally
        .close_voting_window(CloseVotingWindowRequest {
            election_id: Some(election_id.clone()),
            authorizing_signatures: vec![],
        })
        .await?;

    let head = queue
        .get_log_head(suffragio_poc::proto::GetLogHeadRequest {
            election_id: Some(election_id.clone()),
        })
        .await?
        .into_inner();
    let results = tally
        .compute_results(ComputeResultsRequest {
            election_id: Some(election_id.clone()),
            log_head_hash: head.head_hash,
        })
        .await?
        .into_inner()
        .results
        .unwrap();
    tracing::info!("results: {:?}", results);

    tally
        .publish_results(PublishResultsRequest {
            election_id: Some(election_id.clone()),
            package: Some(OfficialResultsPackage {
                results: Some(results.clone()),
                log_head_hash: results.log_head_hash.clone(),
                formula_content_hash: results.formula_content_hash.clone(),
                formula_catalog_script_id: String::new(),
                signatures: vec![],
            }),
        })
        .await?;

    tracing::info!("POC complete");
    server.abort();
    Ok(())
}
