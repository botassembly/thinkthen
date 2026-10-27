#![cfg(test)]
//! Validated numeric values preserve the caller's contract through serialization.

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use thinkthen::{Code, Cut, Error, Family, Probability, Threshold};

#[test]
fn numeric_values_round_trip_without_accepting_invalid_boundaries() {
    for value in [
        0.0,
        f64::MIN_POSITIVE,
        0.12345678901234567,
        0.9421819849031113,
        f64::from_bits(1),
        1.0,
    ] {
        let probability = Probability::new(value).unwrap();
        let json = serde_json::to_string(&probability).unwrap();
        let parsed: Probability = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.get(), value);
        assert_eq!(Cut::new(value).is_ok(), value > 0.0);
    }
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.1, 1.1] {
        assert!(Probability::new(value).is_none());
        assert!(Cut::new(value).is_err());
    }
    for json in ["-0.1", "1.1", "null", "\"0.7\""] {
        assert!(serde_json::from_str::<Probability>(json).is_err());
    }
    for (value, json) in [
        (
            Threshold::from(Cut::new(0.7123456789012345).unwrap()),
            "0.7123456789012345",
        ),
        (
            Threshold::band(0.2, 0.8).unwrap(),
            "{\"low\":0.2,\"high\":0.8}",
        ),
    ] {
        assert_eq!(serde_json::to_string(&value).unwrap(), json);
        assert_eq!(serde_json::from_str::<Threshold>(json).unwrap(), value);
    }
    for json in [
        "0",
        "\"0.2:0.8\"",
        "[0.2,0.8]",
        "{\"low\":0.8,\"high\":0.2}",
        "{\"low\":0.2,\"high\":0.2}",
        "{\"low\":0.2,\"high\":0.8,\"extra\":1}",
    ] {
        assert!(serde_json::from_str::<Threshold>(json).is_err(), "{json}");
    }
    for (low, high) in [(0.0, 0.8), (0.8, 0.2), (0.2, 0.2), (0.2, f64::NAN)] {
        assert_eq!(
            Threshold::band(low, high).unwrap_err().code(),
            Code::UsageThreshold
        );
    }
}

#[test]
fn error_registry_preserves_wire_identity_and_error_metadata() {
    let mut names = std::collections::BTreeSet::new();
    let mut integers = std::collections::BTreeSet::new();
    for code in Code::ALL {
        assert!(names.insert(code.as_str()));
        assert!(integers.insert(code.abi()));
        assert_eq!(Code::from_abi(code.abi()), Some(*code));
        let json = serde_json::to_string(code).unwrap();
        assert_eq!(serde_json::from_str::<Code>(&json).unwrap(), *code);
        assert_eq!(
            code.as_str().split('.').next().unwrap(),
            code.family().as_str()
        );
    }
    for (code, name, abi, family, row) in [
        (
            Code::UsageThreshold,
            "usage.threshold",
            1001,
            Family::Usage,
            false,
        ),
        (
            Code::RecordMissing,
            "record.missing",
            2002,
            Family::Record,
            true,
        ),
        (
            Code::RecordInvalid,
            "record.invalid",
            2004,
            Family::Record,
            true,
        ),
        (
            Code::BackendStatus,
            "backend.status",
            3002,
            Family::Backend,
            true,
        ),
        (
            Code::LocalCancelled,
            "local.cancelled",
            4001,
            Family::Local,
            false,
        ),
        (
            Code::InternalBug,
            "internal.bug",
            5001,
            Family::Internal,
            false,
        ),
    ] {
        assert_eq!(
            (code.as_str(), code.abi(), code.family(), code.is_record()),
            (name, abi, family, row)
        );
    }
    assert!(Code::from_abi(-1).is_none());
    assert!(Code::from_abi(0).is_none());
    let error: Error =
        serde_json::from_str(r#"{"code":"backend.status","message":"busy","http_status":429}"#)
            .unwrap();
    assert_eq!(error.code(), Code::BackendStatus);
    assert_eq!(error.message(), "busy");
    assert_eq!(error.http_status(), Some(429));
    assert_eq!(serde_json::to_value(&error).unwrap()["http_status"], 429);
    assert!(serde_json::from_str::<Error>(r#"{"code":"unknown.error","message":"bad"}"#).is_err());
}

#[test]
fn transport_errors_keep_their_cause_and_status_without_serializing_the_cause() {
    use loopback::{KEY, Loopback, Reply, UnavailableBackend};
    use std::error::Error as _;
    use std::time::Duration;
    use thinkthen::{Decide, Key, Outcome, Record, Retry, Settings};

    {
        let status = 429;
        let server = Loopback::start(move |_| Reply::json(status, &serde_json::json!({}))).unwrap();
        let engine = Settings::default()
            .url(server.base())
            .api_key(Key::new(KEY).unwrap())
            .retry(Retry::new(0, Duration::ZERO, Duration::from_secs(5)).unwrap())
            .engine(|_| None)
            .unwrap();
        let row = engine
            .run(&Decide::new("Ready?").unwrap(), Record::text("yes"))
            .unwrap();
        let Outcome::Failed { error } = row.outcome() else {
            panic!("expected failed row")
        };
        assert_eq!(error.code(), Code::BackendStatus);
        assert_eq!(error.http_status(), Some(status));
        assert!(!serde_json::to_string(error).unwrap().contains("source"));
    }
    let server = UnavailableBackend::new().unwrap();
    let base = server.base();
    let engine = Settings::default()
        .url(base)
        .api_key(Key::new(KEY).unwrap())
        .retry(Retry::new(0, Duration::ZERO, Duration::from_secs(5)).unwrap())
        .engine(|_| None)
        .unwrap();
    let row = engine
        .run(&Decide::new("Ready?").unwrap(), Record::text("yes"))
        .unwrap();
    let error = row.error().unwrap();
    assert_eq!(error.code(), Code::BackendTransport);
    assert!(error.source().is_some());
    assert!(
        error
            .to_string()
            .starts_with("the systemone backend could not be reached: ")
    );
}
