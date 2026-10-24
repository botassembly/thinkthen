#![cfg(test)]
//! SQL shorthand is admitted once before any database wrapper drives the call.

use std::time::Duration;
use thinkthen::Client;
use thinkthen_sql::{SqlCall, SqlEvent, SqlRecord, request};

fn answer(question: &str, options: Option<&str>) -> String {
    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    let request = request("decide", question, options).unwrap();
    let mut call = SqlCall::open(&client, &request).unwrap();
    let mut row = None;
    loop {
        match call.poll(Duration::from_millis(20), || false).unwrap() {
            Some(SqlEvent::NeedsChunk) => {
                call.send_chunk(vec![SqlRecord::Text("evidence".to_owned())])
                    .unwrap();
                call.end_input();
            }
            Some(SqlEvent::Row(value)) => row = Some(value.json),
            Some(SqlEvent::Finished) => break,
            None => {}
        }
    }
    assert!(call.final_error().is_none());
    row.unwrap()
}

#[test]
fn plain_and_tagged_sql_questions_share_admission_and_identity() {
    let tagged = r#" {"type":"inline","value":"Ready?"}"#;
    assert_eq!(answer("Ready?", None), answer(tagged, Some("{}")));
    let toolbox =
        r#"{"type":"toolbox","value":{"questions":{"ready":{"decide":"Ready?"}}},"name":"ready"}"#;
    assert_eq!(answer("Ready?", None), answer(toolbox, None));
    // A literal leading brace travels through an explicitly tagged inline value.
    assert!(!answer(r#"{"type":"inline","value":"{literal question?"}"#, None).is_empty());
}

#[test]
fn malformed_tagged_text_never_falls_back_and_references_keep_engine_errors() {
    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    for (question, options) in [
        ("{not JSON", None),
        ("  {", None),
        ("Ready?", Some("bad")),
        ("Ready?", Some("")),
        ("Ready?", Some(r#"{},"records":["injected"]"#)),
        ("Ready?", Some(r#"{"on":"/wrong","on":"/body"}"#)),
        (
            r#"{"type":"inline","type":"reference","value":"Ready?"}"#,
            None,
        ),
    ] {
        assert_eq!(
            request("decide", question, options)
                .unwrap_err()
                .kind()
                .code(),
            "usage.question"
        );
    }
    let reference = request("decide", r#"{"type":"reference","name":"ready"}"#, None).unwrap();
    assert_eq!(
        SqlCall::open(&client, &reference)
            .unwrap_err()
            .kind()
            .code(),
        "usage.reference"
    );
    let request = request("decide", "Ready?", Some(r#"{"threshold":"bad"}"#)).unwrap();
    assert_eq!(
        SqlCall::open(&client, &request).unwrap_err().kind().code(),
        "usage.threshold"
    );
}

#[test]
fn sql_cannot_route_process_credentials_before_client_admission() {
    use std::cell::RefCell;
    for settings in [
        r#"{"url":"http://127.0.0.1"}"#,
        r#"{"key_source":"OTHER_KEY"}"#,
        r#"{"url":null}"#,
        r#"{"key_source":null}"#,
        r#"{"adapter":"stub","url":null,"key_source":null}"#,
    ] {
        let error = thinkthen_sql::client(settings, |_| {
            panic!("rejected SQL settings read the environment")
        })
        .unwrap_err();
        assert_eq!(error.kind().code(), "usage.settings");
    }
    let names = RefCell::new(Vec::new());
    thinkthen_sql::client("{}", |name| {
        names.borrow_mut().push(name.to_owned());
        (name == "THINKTHEN_API_KEY").then(|| "synthetic-key".to_owned())
    })
    .unwrap();
    assert!(
        names
            .borrow()
            .iter()
            .any(|name| name == "THINKTHEN_API_KEY")
    );
    assert!(
        !names
            .borrow()
            .iter()
            .any(|name| name == "OPENAI_API_KEY" || name == "OTHER_KEY")
    );
}

#[test]
fn backend_policy_precedes_credentials_and_uses_effective_models() {
    use thinkthen_sql::client_with_policy;
    for (settings, policy, code) in [
        (r#"{"adapter":"stub"}"#, None, "usage.sql-selection"),
        (r#"{"adapter":"openai"}"#, None, "usage.sql-selection"),
        (r#"{"model":"other"}"#, None, "usage.sql-selection"),
        ("{}", Some("{}"), "usage.sql-selection"),
        (
            "{}",
            Some(r#"{"systemone":["other"]}"#),
            "usage.sql-selection",
        ),
        (
            r#"{"adapter":"stub","adapter":"systemone"}"#,
            Some(r#"{"stub":["stub"]}"#),
            "usage.settings",
        ),
        (
            r#"{"adapter":"stub"}"#,
            Some(r#"{"stub":"stub"}"#),
            "usage.sql-policy",
        ),
        (r#"{"adapter":"stub"}"#, Some("null"), "usage.sql-policy"),
    ] {
        let error = client_with_policy(settings, policy, |_| {
            panic!("denied backend policy read the environment")
        })
        .unwrap_err();
        assert_eq!(error.kind().code(), code);
    }
    // The stub ignores caller model overrides; enforce the effective model.
    client_with_policy(
        r#"{"adapter":"stub","model":"ignored","batch":16}"#,
        Some(r#"{"stub":["stub"]}"#),
        |_| panic!("stub read the environment"),
    )
    .unwrap();
    for (settings, policy) in [
        ("{}", None),
        (r#"{"adapter":"systemone","model":"jev-1.13.0"}"#, None),
        (
            r#"{"adapter":"openai"}"#,
            Some(r#"{"openai":["gpt-6-luna"]}"#),
        ),
        (r#"{"model":"custom"}"#, Some(r#"{"systemone":["custom"]}"#)),
    ] {
        client_with_policy(settings, policy, |name| {
            matches!(name, "THINKTHEN_API_KEY" | "OPENAI_API_KEY")
                .then(|| "synthetic-key".to_owned())
        })
        .unwrap();
    }
}
