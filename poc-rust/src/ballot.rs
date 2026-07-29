use std::collections::{BTreeMap, HashSet};

use anyhow::{anyhow, bail, Context};
use serde::Deserialize;

use crate::proto::BallotTemplate;

pub type FilledBallot = BTreeMap<String, Vec<String>>;

pub fn encode(ballot: &FilledBallot) -> anyhow::Result<Vec<u8>> {
    cbor4ii::serde::to_vec(Vec::new(), ballot).map_err(|e| anyhow!("cbor encode: {e}"))
}

pub fn decode(bytes: &[u8]) -> anyhow::Result<FilledBallot> {
    cbor4ii::serde::from_slice(bytes).context("cbor decode ballot")
}

#[derive(Debug, Deserialize)]
struct DslDoc {
    questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
struct Question {
    id: String,
    #[serde(rename = "type")]
    qtype: String,
    #[serde(default)]
    required: bool,
    #[serde(default)]
    options: Vec<Opt>,
}

#[derive(Debug, Deserialize)]
struct Opt {
    id: String,
}

pub fn validate(ballot: &FilledBallot, template: &BallotTemplate) -> anyhow::Result<()> {
    let doc: DslDoc = serde_json::from_str(&template.document_json)
        .context("parse ballot template DSL")?;
    let questions: std::collections::HashMap<String, Question> = doc
        .questions
        .into_iter()
        .map(|q| (q.id.clone(), q))
        .collect();

    let mut seen = HashSet::new();
    for (qid, selected) in ballot.iter() {
        if !seen.insert(qid.clone()) {
            bail!("duplicate question id in ballot: {qid}");
        }
        let q = questions
            .get(qid)
            .ok_or_else(|| anyhow!("unknown question id: {qid}"))?;
        let option_ids: HashSet<String> = q.options.iter().map(|o| o.id.clone()).collect();
        for opt in selected.iter() {
            if !option_ids.contains(opt) {
                bail!("unknown option id {opt} for question {qid}");
            }
        }
        match q.qtype.as_str() {
            "single_choice" | "yes_no" => {
                if selected.len() != 1 {
                    bail!("question {qid} expects exactly one choice");
                }
            }
            "multi_choice" => {
                if q.required && selected.is_empty() {
                    bail!("question {qid} is required");
                }
            }
            "ranked" => {
                if selected.len() != option_ids.len() {
                    bail!("question {qid} ranked vote must contain all options once");
                }
                let uniq: HashSet<_> = selected.iter().cloned().collect();
                if uniq.len() != selected.len() {
                    bail!("question {qid} ranked vote contains duplicates");
                }
            }
            other => bail!("unsupported question type: {other}"),
        }
    }

    for q in questions.values() {
        if q.required && !ballot.contains_key(&q.id) {
            bail!("missing required question: {}", q.id);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn template() -> BallotTemplate {
        BallotTemplate {
            constituency_id: Some(crate::proto::ConstituencyId {
                value: "district-1".to_string(),
            }),
            dsl_version: "suffragio-ballot-dsl/1".to_string(),
            document_json: r#"{
                "questions": [
                    {
                        "id": "q_president",
                        "type": "single_choice",
                        "required": true,
                        "options": [
                            {"id": "cand_a"},
                            {"id": "cand_b"}
                        ]
                    }
                ]
            }"#
            .to_string(),
        }
    }

    #[test]
    fn encode_decode_roundtrip() {
        let mut ballot = FilledBallot::new();
        ballot.insert("q_president".to_string(), vec!["cand_a".to_string()]);
        let encoded = encode(&ballot).unwrap();
        let decoded = decode(&encoded).unwrap();
        assert_eq!(ballot, decoded);
    }

    #[test]
    fn valid_single_choice() {
        let mut ballot = FilledBallot::new();
        ballot.insert("q_president".to_string(), vec!["cand_a".to_string()]);
        validate(&ballot, &template()).unwrap();
    }

    #[test]
    fn invalid_option() {
        let mut ballot = FilledBallot::new();
        ballot.insert("q_president".to_string(), vec!["cand_c".to_string()]);
        assert!(validate(&ballot, &template()).is_err());
    }

    #[test]
    fn missing_required_question() {
        let ballot = FilledBallot::new();
        assert!(validate(&ballot, &template()).is_err());
    }
}
