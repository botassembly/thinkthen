#![cfg(test)]
//! Shared option mappings compare complete equivalent calls, including identity.

use serde_json::Value;
use thinkthen::{Cancel, Client, SdkRequest};

fn execute(client: &Client, request: &Value) -> Result<Vec<Value>, thinkthen::Error> {
    let request = SdkRequest::from_json(&request.to_string())?;
    let mut rows = Vec::new();
    request.execute(client, Cancel::default(), |row| {
        rows.push(serde_json::to_value(row).unwrap());
    })?;
    Ok(rows)
}

#[test]
fn saved_domains_and_rank_levels_share_the_same_contract() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../../../construction/options.json")).unwrap();
    for case in cases {
        let client = Client::from_json(&case["client"].to_string(), |_| None).unwrap();
        let actual = execute(&client, &case["request"]);
        if let Some(code) = case["expected_error"].as_str() {
            assert_eq!(actual.unwrap_err().kind().code(), code, "{}", case["name"]);
        } else {
            let actual = actual.unwrap();
            assert_eq!(
                actual.len(),
                case["request"]["records"].as_array().unwrap().len()
            );
            assert_eq!(
                actual,
                execute(&client, &case["equivalent_request"]).unwrap(),
                "{}",
                case["name"]
            );
        }
    }
}
