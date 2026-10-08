//! Effective-last legacy routing preserves original strict canonical bytes.
use super::{compile, crate_dir, run, text};
use super::cases::{Script, replies};
use conformance_backend::{Canned, Listener};

#[test]
fn canonical_namespace_routes_original_bytes_and_legacy_schema_stays_ignored() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)).unwrap();
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let canonical = r#"{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Fits?"},"input":{"kind":"text","text":"Alpha."}}}"#;
    let mut script = Script::default();
    script.ask("settings", &[listener.base(), r#"{"cache":false,"model":"fixed"}"#]);
    for request in [
        canonical.to_owned(),
        canonical.replace("thinkthen.request/1", "thinkthen.request/2"),
        canonical.replace("\"schema\":", "\"schema\":\"unrelated\",\"schema\":"),
        r#"{"schema":"thinkthen.request/2","schema":"unrelated","decide":"Fits?","evidence":"Alpha."}"#.to_owned(),
        r#"{"schema":"unrelated","decide":"old","decide":"Fits?","evidence":"old","evidence":"Alpha."}"#.to_owned(),
        r#"{"recognize":{},"records":["Alpha."],"evidence":"Alpha."}"#.to_owned(),
    ] { script.ask("call", &[listener.base(), &request]); }
    let output = run(&driver, listener.base(), &script.0);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let replies = replies(&output.stdout).unwrap();
    assert_eq!(replies.iter().map(|r| r.0).collect::<Vec<_>>(), vec![0,0,1,1,0,0,1]);
    assert!(replies[6].1.contains("recognize takes no records key"));
    let canonical: serde_json::Value = serde_json::from_str(&replies[1].1).unwrap();
    assert!(canonical.to_string().contains("thinkthen.result/2"));
    let mut legacy_a: serde_json::Value = serde_json::from_str(&replies[4].1).unwrap();
    let mut legacy_b: serde_json::Value = serde_json::from_str(&replies[5].1).unwrap();
    legacy_a["facts"].as_object_mut().unwrap().remove("seconds");
    legacy_b["facts"].as_object_mut().unwrap().remove("seconds");
    assert_eq!(legacy_a, legacy_b, "legacy effective-last members preserve value and facts");
    assert_eq!(listener.count(), 3);
    let requests = listener.requests();
    assert_eq!(requests[0].body, requests[1].body);
    assert_eq!(requests[1].body, requests[2].body);
}
