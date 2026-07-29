use std::collections::BTreeMap;

use anyhow::anyhow;
use mlua::{Lua, LuaOptions, LuaSerdeExt, StdLib};
use serde::{Deserialize, Serialize};

use crate::proto::{ConstituencyId, ConstituencyResult, ElectionId, ElectionResults};

#[derive(Serialize)]
struct LuaBallot {
    constituency_id: String,
    ballot: BTreeMap<String, Vec<String>>,
}

#[derive(Deserialize)]
struct LuaConstituencyResult {
    constituency_id: String,
    tally_by_choice: std::collections::HashMap<String, i64>,
}

#[derive(Deserialize)]
struct LuaResults {
    constituency_results: Vec<LuaConstituencyResult>,
    total_votes_counted: u64,
}

pub fn run_formula(
    script: &[u8],
    election_id: &str,
    ballots: Vec<(String, BTreeMap<String, Vec<String>>)>,
    invalid_rejected_at_submit: u64,
    formula_content_hash: Vec<u8>,
    log_head_hash: Vec<u8>,
) -> anyhow::Result<ElectionResults> {
    let lua = Lua::new_with(
        StdLib::TABLE | StdLib::STRING | StdLib::MATH,
        LuaOptions::default(),
    )
    .map_err(|e| anyhow!("create lua: {e}"))?;
    let _ = lua.set_memory_limit(8 * 1024 * 1024);

    let lua_ballots: Vec<LuaBallot> = ballots
        .into_iter()
        .map(|(constituency_id, ballot)| LuaBallot {
            constituency_id,
            ballot,
        })
        .collect();

    let globals = lua.globals();
    globals
        .set("ballots", lua.to_value(&lua_ballots).map_err(|e| anyhow!("serialize ballots: {e}"))?)
        .map_err(|e| anyhow!("set ballots: {e}"))?;
    globals
        .set("election_id", election_id)
        .map_err(|e| anyhow!("set election_id: {e}"))?;

    let chunk = lua.load(script);
    let func: mlua::Function = chunk.eval().map_err(|e| anyhow!("load lua formula: {e}"))?;

    let value: mlua::Value = func
        .call::<mlua::Value>((
            lua.to_value(&lua_ballots).map_err(|e| anyhow!("serialize ballots: {e}"))?,
            election_id,
        ))
        .map_err(|e| anyhow!("run lua formula: {e}"))?;

    let parsed: LuaResults = lua.from_value(value).map_err(|e| anyhow!("parse lua results: {e}"))?;

    let constituency_results = parsed
        .constituency_results
        .into_iter()
        .map(|r| ConstituencyResult {
            constituency_id: Some(ConstituencyId {
                value: r.constituency_id,
            }),
            tally_by_choice: r.tally_by_choice,
        })
        .collect();

    Ok(ElectionResults {
        election_id: Some(ElectionId {
            value: election_id.to_string(),
        }),
        constituency_results,
        total_votes_counted: parsed.total_votes_counted,
        invalid_rejected_at_submit,
        computed_at: Some(crate::app::now_timestamp()),
        formula_content_hash,
        log_head_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_plurality_tally() {
        let script = br#"return function(ballots, _election_id)
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

        let mut b1 = BTreeMap::new();
        b1.insert("q_president".to_string(), vec!["cand_a".to_string()]);
        let mut b2 = BTreeMap::new();
        b2.insert("q_president".to_string(), vec!["cand_a".to_string()]);
        let mut b3 = BTreeMap::new();
        b3.insert("q_president".to_string(), vec!["cand_b".to_string()]);

        let ballots = vec![
            ("district-1".to_string(), b1),
            ("district-1".to_string(), b2),
            ("district-1".to_string(), b3),
        ];

        let results = run_formula(script, "test", ballots, 0, vec![], vec![]).unwrap();
        assert_eq!(results.total_votes_counted, 3);
        assert_eq!(results.constituency_results.len(), 1);
        let tally = &results.constituency_results[0].tally_by_choice;
        assert_eq!(tally.get("cand_a").copied().unwrap_or(0), 2);
        assert_eq!(tally.get("cand_b").copied().unwrap_or(0), 1);
    }
}
