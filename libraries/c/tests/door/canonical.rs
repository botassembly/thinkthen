//! Effective-last legacy routing preserves original strict canonical bytes.
use super::cases::{Script, replies};
use super::{compile, crate_dir, run, text};
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};

#[test]
fn canonical_namespace_routes_original_bytes_and_legacy_schema_stays_ignored() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let canonical = r#"{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Fits?"},"input":{"kind":"text","text":"Alpha."}}}"#;
    let mut script = Script::default();
    script.ask(
        "settings",
        &[listener.base(), r#"{"cache":false,"model":"fixed"}"#],
    );
    for request in [
        canonical.to_owned(),
        canonical.replace("thinkthen.request/1", "thinkthen.request/2"),
        canonical.replace("\"schema\":", "\"schema\":\"unrelated\",\"schema\":"),
        r#"{"schema":"thinkthen.request/2","schema":"unrelated","decide":"Fits?","evidence":"Alpha."}"#.to_owned(),
        r#"{"schema":"unrelated","decide":"old","decide":"Fits?","evidence":"old","evidence":"Alpha."}"#.to_owned(),
        r#"{"recognize":{},"records":["Alpha."],"evidence":"Alpha."}"#.to_owned(),
        r#"{"filter":"Fits?","threshold":"0.2:0.8","records":["Alpha."]}"#.to_owned(),
    ] { script.ask("call", &[listener.base(), &request]); }
    let output = run(&driver, listener.base(), &script.0);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let replies = replies(&output.stdout).unwrap();
    assert_eq!(
        replies.iter().map(|r| r.0).collect::<Vec<_>>(),
        vec![0, 0, 1, 1, 0, 0, 1, 1]
    );
    assert!(replies[6].1.contains("recognize takes no records key"));
    assert!(
        replies[7]
            .1
            .contains("filter keeps a record at a cut, not a band")
    );
    let canonical: serde_json::Value = serde_json::from_str(&replies[1].1).unwrap();
    assert!(canonical.to_string().contains("thinkthen.result/2"));
    let mut legacy_a: serde_json::Value = serde_json::from_str(&replies[4].1).unwrap();
    let mut legacy_b: serde_json::Value = serde_json::from_str(&replies[5].1).unwrap();
    legacy_a["facts"].as_object_mut().unwrap().remove("seconds");
    legacy_b["facts"].as_object_mut().unwrap().remove("seconds");
    assert_eq!(
        legacy_a, legacy_b,
        "legacy effective-last members preserve value and facts"
    );
    assert_eq!(listener.count(), 3);
    let requests = listener.requests();
    assert_eq!(requests[0].body, requests[1].body);
    assert_eq!(requests[1].body, requests[2].body);
}

#[test]
fn legacy_annotation_documents_keep_projection_cache_and_parser_boundaries() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .unwrap();
    let base = listener.base();
    // The unchanged request from the JVM Matrix.requests[11] reaches the same C door.
    let projected = r#"{"annotate":{"version":1,"questions":{"check":{"decide":"Is it?","on":"/body"}}},"records":["{\"body\":\"annotate-on\",\"hidden\":\"not-sent\"}"]}"#;
    let mut script = Script::default();
    script.ask("settings", &[base, r#"{"model":"fixed"}"#]);
    for _ in 0..2 {
        script.ask("call", &[base, projected]);
    }
    let set = json!({"version":1,"questions":{"check":{"decide":"Q?","on":"/a"}}});
    script.ask(
        "call",
        &[
            base,
            &json!({"annotate":set,"records":[r#"{"a":1,"a":2}"#]}).to_string(),
        ],
    );
    let plain = json!({"version":1,"questions":{"check":{"decide":"Q?"}}});
    script.ask(
        "call",
        &[
            base,
            &json!({"annotate":plain,"records":["{not json"]}).to_string(),
        ],
    );
    let canonical = json!({"schema":"thinkthen.request/1","call":{
        "function":"annotate","question":{"kind":"definition","value":set},
        "input":{"kind":"records","items":[{"original":{"kind":"text","text":r#"{"a":1}"#}}]}
    }});
    script.ask("call", &[base, &canonical.to_string()]);
    let mut text_document = canonical;
    text_document["call"]["input"]["items"][0]["original"]["text"] = json!("{not json");
    script.ask("call", &[base, &text_document.to_string()]);
    let output = run(
        &compile(&crate_dir().join("tests/c/driver.c")),
        base,
        &script.0,
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let got = replies(&output.stdout).unwrap();
    assert_eq!(
        got.iter().map(|reply| reply.0).collect::<Vec<_>>(),
        [0, 0, 0, 1, 0, 0, 1]
    );
    let first: Value = serde_json::from_str(&got[1].1).unwrap();
    let cached: Value = serde_json::from_str(&got[2].1).unwrap();
    assert_eq!(first["value"], json!([{"check":true}]));
    assert_eq!(cached["value"], first["value"]);
    assert_eq!(first["facts"]["requests_sent"], 1);
    assert_eq!(cached["facts"]["requests_sent"], 0);
    assert_eq!(cached["facts"]["cache_answers"], 1);
    assert_eq!(
        got[3].1,
        "a JSON record holds each member name once, and one name arrived twice"
    );
    assert_eq!(
        got[6].1,
        "question `check` reads `on`, and this record's evidence is text with no members"
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3, "cached and refused records send nothing");
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(
        body["questions"]["q1"]["instructions"],
        "The text is \"annotate-on\". Is it?"
    );
    assert!(!text(&requests[0].body).contains("not-sent"));
    assert!(!text(&requests[0].body).contains("hidden"));
    let fallback: Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(
        fallback["questions"]["q1"]["instructions"],
        "The text is \"{not json\". Q?"
    );
    let projected: Value = serde_json::from_str(&got[5].1).unwrap();
    assert_eq!(projected["value"][0]["input"], json!({"a":1}));
    assert_eq!(projected["value"][0]["value"], json!({"check":true}));
    assert_eq!(projected["facts"]["requests_sent"], 1);
    let body: Value = serde_json::from_slice(&requests[2].body).unwrap();
    assert_eq!(
        body["questions"]["q1"]["instructions"],
        "The text is \"1\". Q?"
    );
}
