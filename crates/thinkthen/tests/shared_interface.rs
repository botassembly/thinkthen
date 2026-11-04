#![cfg(test)]
//! Shared admission refuses blank evidence before any paid request.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{
    BackendName, Key, OffsetUnit, Request, Settings, questions::Decide, records::Record,
};

#[test]
fn omitted_backend_resolves_environment_before_credentials_and_explicit_selection_wins() {
    for settings in [Settings::default(), Settings::from_json("{}").unwrap()] {
        let engine = settings
            .engine(|name| {
                assert_eq!(name, "THINKTHEN_BACKEND", "stub read another setting");
                Some("stub".into())
            })
            .unwrap();
        assert_eq!(engine.resolved_settings().backend(), BackendName::Stub);
        assert_eq!(
            engine
                .run(&Decide::new("Ready?").unwrap(), Record::text("record"))
                .unwrap()
                .source()
                .backend(),
            BackendName::Stub
        );
        let error = settings
            .engine(|name| {
                assert_eq!(
                    name, "THINKTHEN_BACKEND",
                    "invalid backend read credentials"
                );
                Some("unknown-backend".into())
            })
            .unwrap_err();
        assert_eq!(error.code().as_str(), "usage.backend");
        let roundtrip = serde_json::to_string(&settings).unwrap();
        assert_eq!(
            Settings::from_json(&roundtrip)
                .unwrap()
                .engine(|name| {
                    assert_eq!(name, "THINKTHEN_BACKEND");
                    Some("stub".into())
                })
                .unwrap()
                .resolved_settings()
                .backend(),
            BackendName::Stub
        );
    }
    Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| panic!("explicit stub read environment"))
        .unwrap();
    assert_eq!(
        Settings::default()
            .engine(|_| None)
            .unwrap_err()
            .code()
            .as_str(),
        "usage.key-missing"
    );
}

#[test]
fn omitted_backend_environment_selects_the_actual_remote_route() {
    let server = Loopback::start(|_| {
        loopback::Reply::json(
            200,
            &json!({"answers":[{"type":"predicate","name":null,"probability":0.9}]}),
        )
    })
    .unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch(1_u64.try_into().unwrap())
        .engine(|name| {
            assert_eq!(
                name, "THINKTHEN_BACKEND",
                "direct synthetic key must avoid credential lookup"
            );
            Some("openai".into())
        })
        .unwrap();
    let row = engine
        .run(&Decide::new("Ready?").unwrap(), Record::text("record"))
        .unwrap();
    assert_eq!(row.source().backend(), BackendName::OpenAi);
    assert!(row.into_answer().unwrap());
    assert_eq!(server.requests().len(), 1);
    assert_eq!(server.requests()[0].path, "/v1/decisions");
}

#[test]
fn blank_records_fail_without_requests_and_empty_calls_return_nothing() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::SystemOne)
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    for records in [
        json!(["", "   ", "\n", "\u{2003}", {}, {"id": "only"}]),
        json!([]),
    ] {
        let request = Request::from_json(
            &json!({"question":{"function":"decide","question":"Ready?"},"records":records})
                .to_string(),
        )
        .unwrap();
        let rows = engine
            .open_request(&request, OffsetUnit::Utf8)
            .unwrap()
            .rows()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(rows.len(), records.as_array().unwrap().len());
        for row in rows {
            let row = serde_json::to_value(row).unwrap();
            assert_eq!(row["status"], "failed");
            assert_eq!(row["error"]["code"], "record.empty");
        }
    }
    let selected = Decide::new("Ready?")
        .unwrap()
        .field("/text".parse().unwrap());
    for (value, expected) in [
        (json!(""), "record.empty"),
        (json!("  "), "record.empty"),
        (json!({}), "record.empty"),
        (json!([]), "record.empty"),
        (json!(null), "record.missing"),
    ] {
        let record = Record::json(json!({"text":value}));
        if expected == "record.empty" {
            assert_eq!(
                engine.run(&selected, record).unwrap_err().code().as_str(),
                expected
            );
        } else {
            assert_eq!(
                engine
                    .run(&selected, record)
                    .unwrap()
                    .error()
                    .unwrap()
                    .code()
                    .as_str(),
                expected
            );
        }
    }
    let question = Decide::new("Ready?").unwrap();
    let error = engine.run(&question, Record::text(" ")).unwrap_err();
    assert_eq!(error.code().as_str(), "record.empty");
    assert!(
        server.requests().is_empty(),
        "blank records reached the backend"
    );
    let controls = engine
        .rows(
            &selected,
            [
                Record::json(json!({"text":"[]"})),
                Record::json(json!({"text":"{}"})),
                Record::json(json!({"text":false})),
                Record::json(json!({"text":0})),
            ],
        )
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(controls.len(), 4);
    assert!(
        controls
            .iter()
            .all(|row| row.status() == thinkthen::results::Status::Answered)
    );
    assert_eq!(server.requests().len(), 1);
}

#[test]
fn mixed_streams_continue_and_selected_empty_evidence_is_refused() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::SystemOne)
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    let request = Request::from_json(&json!({
        "question":{"function":"decide","question":"Ready?","field":"/body"},
        "records":[{"body":"first"},{"body":""},{"body":{}},{"body":"   "},{"body":[]},{"body":null},{"body":"second"}]
    }).to_string()).unwrap();
    let rows = engine
        .open_request(&request, OffsetUnit::Utf8)
        .unwrap()
        .rows()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 7);
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(row.position(), (index + 1) as u64);
        if [1, 2, 3, 4].contains(&index) {
            assert_eq!(row.error().unwrap().code().as_str(), "record.empty");
        } else if index == 5 {
            assert_eq!(row.error().unwrap().code().as_str(), "record.missing");
        } else {
            assert_eq!(row.status(), thinkthen::results::Status::Answered);
        }
    }
    assert_eq!(server.requests().len(), 1);
}
