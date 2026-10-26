#![cfg(test)]
//! SQL shorthand admits canonical questions without reference resolution.
use thinkthen_sql::request;
#[test]
fn plain_and_flat_questions_share_identity_and_required_lists() {
    let plain = request(
        "choose",
        "Ready?",
        Some(r#"{"choices":["one","two"],"threshold":{"low":0.2,"high":0.8}}"#),
    )
    .unwrap();
    let canonical = thinkthen::Request::from_json(r#"{"question":{"function":"choose","question":"Ready?","choices":["one","two"],"threshold":{"low":0.2,"high":0.8}}}"#).unwrap();
    assert_eq!(
        serde_json::to_value(plain).unwrap(),
        serde_json::to_value(canonical).unwrap()
    );
    assert!(request("decide", "{literal", None).is_ok());
    for (function, question, options) in [
        ("choose", "Ready?", None),
        (
            "decide",
            "Ready?",
            Some(r#"{"threshold":0.4,"threshold":0.7}"#),
        ),
        (
            "tag",
            "Ready?",
            Some(r#"{"labels":["one"],"threshold":{"low":0.2,"high":0.8}}"#),
        ),
    ] {
        assert!(request(function, question, options).is_err());
    }
}
#[test]
fn sql_policy_and_forbidden_fields_precede_environment_access() {
    for field in ["url", "api_key", "key_source"] {
        let settings = format!("{{\"{field}\":null}}");
        let error =
            thinkthen_sql::client(&settings, |_| panic!("invalid settings read environment"))
                .unwrap_err();
        assert_eq!(error.code().as_str(), "usage.settings");
    }
    for (settings, policy, code) in [
        (r#"{"backend":"stub"}"#, None, "usage.sql-selection"),
        ("{}", Some(""), "usage.sql-selection"),
        ("{}", Some("null"), "usage.sql-policy"),
        (
            r#"{"backend":"stub"}"#,
            Some(r#"{"stub":"stub"}"#),
            "usage.sql-policy",
        ),
    ] {
        let error = thinkthen_sql::client_with_policy(settings, policy, |_| {
            panic!("denied settings read environment")
        })
        .unwrap_err();
        assert_eq!(error.code().as_str(), code);
    }
    thinkthen_sql::client_with_policy(
        r#"{"backend":"stub","model":"ignored"}"#,
        Some("stub:*"),
        |_| panic!("stub read environment"),
    )
    .unwrap();
}

#[test]
fn unrestricted_databases_select_backends_without_postgresql_policy() {
    thinkthen_sql::client(r#"{"backend":"stub","model":"ignored"}"#, |_| {
        panic!("stub read environment")
    })
    .unwrap();
    thinkthen_sql::client(r#"{"backend":"openai","model":"custom-model"}"#, |name| {
        (name == "OPENAI_API_KEY").then(|| "synthetic-key".into())
    })
    .unwrap();
}
