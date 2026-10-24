#![cfg(test)]
//! SQL settings and diagnostic contracts shared by all database hosts.

use serde_json::Value;

#[test]
fn sql_settings_preserve_codes_and_forbidden_field_precedence() {
    let cases: Vec<Value> = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/sql.json"
    ))
    .unwrap();
    for case in cases {
        let settings = case["settings"].as_str().unwrap();
        let variable = |_: &str| panic!("rejected SQL settings read a credential");
        let error = if let Some(policy) = case.get("policy") {
            thinkthen_sql::client_with_policy(settings, policy.as_str(), variable)
        } else {
            thinkthen_sql::client(settings, variable)
        }
        .unwrap_err();
        assert_eq!(error.kind().code(), case["code"], "{case}");
        if let Some(expected) = case.get("message") {
            assert_eq!(error.to_string(), expected.as_str().unwrap());
        }
    }
}
