//! One installed-header driver exercises dynamic JSON records and call facts.

use super::*;
use conformance_backend::{Canned, Listener};

#[test]
fn dynamic_records_wrap_facts_and_stop_without_partial_json() {
    let success = r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","choice":"first","probabilities":{"first":0.9,"second":0.1}},"q2":{"type":"choice","choice":"second","probabilities":{"first":0.2,"second":0.8}}},"usage":{"input_tokens":5,"output_tokens":3}}"#;
    let failed = r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","choice":"first","probabilities":{"first":0.9,"second":0.1}}},"usage":{"input_tokens":5,"output_tokens":3}}"#;
    let listener =
        Listener::serving(vec![Canned::ok(success), Canned::ok(failed)]).expect("listener");
    let base = listener.base();
    let settings = json!({"base_url":base,"cache":false,"max_retries":0}).to_string();
    let request = json!({
        "choose":"Which label?", "options": {"first":"First route", "second":"Second route"},
        "records":["alpha","beta"], "details":true, "call":{"batch":2}
    })
    .to_string();
    let mut script = Script::default();
    script.ask("settings", &[base, &settings]);
    script.ask("call", &[base, &request]);
    script.ask("call", &[base, &request]);
    script.ask("facts", &[base, "-"]);
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    assert_eq!(got.len(), 4);
    assert_eq!(got[0].0, 0);
    let wrapped: Value = serde_json::from_str(&got[1].1).expect("wrapped success");
    assert_eq!(got[1].0, 0);
    assert_eq!(wrapped["facts"]["records"], 2);
    assert_eq!(wrapped["facts"]["requests_sent"], 1);
    assert_eq!(wrapped["value"][0]["input"], "alpha");
    assert_eq!(wrapped["value"][1]["input"], "beta");
    assert_eq!(wrapped["value"][0]["value"], "first");
    assert_eq!(wrapped["value"][1]["value"], "second");
    assert_eq!(wrapped["value"][1]["meta"]["batch"]["position"], 2);
    assert_eq!(got[2].0, 2, "a failed second row has no partial JSON");
    assert!(!got[2].1.starts_with('{'));
    let stopped: Value = serde_json::from_str(&got[3].1).expect("failure facts");
    assert_eq!(stopped["records"], 1);
    assert_eq!(stopped["requests_sent"], 1);
    assert_eq!(listener.connections(), 2);
}

#[test]
#[expect(
    clippy::cognitive_complexity,
    reason = "one listener table checks maximal, batch-one, contextual and refused calls"
)]
fn call_batch_controls_request_count_and_refuses_scalar_context_before_send() {
    let listener = Listener::answering(|body| {
        let asked: Value = serde_json::from_slice(body).expect("request JSON");
        let questions = asked["questions"].as_object().expect("questions");
        let answers: serde_json::Map<String, Value> = questions
            .keys()
            .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&json!({"model":"jev-latest","answers":answers}).to_string())
    })
    .expect("listener");
    let base = listener.base();
    let settings = json!({"base_url":base,"cache":false}).to_string();
    let records = ["alpha", "beta", "gamma"];
    let maximal = json!({"decide":"Is it relevant?","records":records,"details":true}).to_string();
    let singles =
        json!({"decide":"Is it relevant?","records":records,"call":{"batch":1}}).to_string();
    let contextual = json!({"decide":"Is it relevant?","records":records,"details":true,"call":{"batch":3,"context":"Shared facts."}}).to_string();
    let refused =
        json!({"decide":"Is it relevant?","evidence":"alpha","call":{"context":"Shared."}})
            .to_string();
    let empty_calls = [
        json!({"decide":"Is it relevant?","evidence":"alpha","call":{}}),
        json!({"find":"Which?","units":["alpha"],"call":{}}),
        json!({"recognize":{"version":1,"recognize":{"kinds":{"person":"A person."}}},"evidence":"alpha","call":{}}),
        json!({"relate":{"version":1,"relate":{"relations":[{"name":"linked","source":"person","target":"person"}]}},"records":[{"name":"alpha","kind":"person"}],"call":{}}),
    ];
    let mut script = Script::default();
    script.ask("settings", &[base, &settings]);
    script.ask("call", &[base, &maximal]);
    script.ask("call", &[base, &singles]);
    script.ask("call", &[base, &contextual]);
    script.ask("call", &[base, &refused]);
    for request in &empty_calls {
        script.ask("call", &[base, &request.to_string()]);
    }
    script.ask("facts", &[base, "-"]);
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    assert_eq!(got.len(), 10);
    let maximal: Value = serde_json::from_str(&got[1].1).expect("maximal JSON");
    let singles: Value = serde_json::from_str(&got[2].1).expect("single JSON");
    let contextual: Value = serde_json::from_str(&got[3].1).expect("contextual JSON");
    assert_eq!(got[1].0, 0);
    assert_eq!(got[2].0, 0);
    assert_eq!(got[3].0, 0);
    assert_eq!(maximal["value"][0]["value"], true);
    assert_eq!(maximal["value"][2]["input"], "gamma");
    assert_eq!(singles["value"], json!([true, true, true]));
    assert_eq!(maximal["facts"]["requests_sent"], 1);
    assert_eq!(singles["facts"]["requests_sent"], 3);
    assert_eq!(contextual["facts"]["requests_sent"], 1);
    assert_eq!(contextual["value"][0]["input"], "alpha");
    assert_eq!(contextual["value"][1]["meta"]["batch"]["position"], 2);
    assert_eq!(
        contextual["value"][0]["meta"]["context_sha256"],
        "3fecbe9f8bf58be501d407aed3249b1a6b404b035b0aace505df40089de5a9c1"
    );
    assert_eq!(
        contextual["value"][0]["meta"]["question_sha256"],
        maximal["value"][0]["meta"]["question_sha256"]
    );
    assert_eq!(got[4].0, 1, "scalar context is a usage failure");
    for (reply, verb) in got[5..9]
        .iter()
        .zip(["decide", "find", "recognize", "relate"])
    {
        assert_eq!(reply.0, 1, "{verb} refuses an empty call object");
    }
    assert_eq!(
        got[9],
        (0, "null".to_owned()),
        "pre-send usage has no call facts"
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 5, "the refused context sent nothing");
    let sizes: Vec<usize> = requests
        .iter()
        .map(|request| {
            let body: Value = serde_json::from_slice(&request.body).expect("request body");
            body["questions"].as_object().expect("questions").len()
        })
        .collect();
    assert_eq!(sizes, [3, 1, 1, 1, 3]);
    assert!(
        String::from_utf8_lossy(&requests[4].body).contains("Shared facts."),
        "the shared context reached the request"
    );
}
