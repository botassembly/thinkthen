#![cfg(test)]
//! Saved domains and ranks retain their canonical construction behavior.
use serde_json::Value;
use thinkthen::{OffsetUnit, Request, Settings};
fn execute(engine: &thinkthen::Engine, value: &Value) -> Result<Vec<Value>, thinkthen::Error> {
    let request = Request::from_json(&value.to_string())?;
    let call = engine.open_request(&request, OffsetUnit::CodePoints)?;
    call.sender().close();
    call.rows()
        .map(|row| row.map(|row| serde_json::to_value(row).unwrap()))
        .collect()
}
#[test]
fn saved_domains_and_rank_levels_share_the_same_contract() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../../../construction/options.json")).unwrap();
    for case in cases {
        let engine = Settings::from_json(&case["client"].to_string())
            .unwrap()
            .engine(|_| None)
            .unwrap();
        let actual = execute(&engine, &case["request"]);
        if let Some(code) = case["expected_error"].as_str() {
            assert_eq!(
                actual.unwrap_err().code().as_str(),
                code,
                "{}",
                case["name"]
            );
        } else {
            let actual = actual.unwrap();
            assert_eq!(
                serde_json::json!(actual),
                case["expected"],
                "{}",
                case["name"]
            );
            assert_eq!(
                actual,
                execute(&engine, &case["equivalent_request"]).unwrap(),
                "{}",
                case["name"]
            );
        }
    }
}
