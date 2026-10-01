//! One installed-header driver exercises dynamic JSON records and call facts.

use super::*;
use conformance_backend::{Canned, Listener};

#[test]
fn direct_c_json_attempts_opt_in_keeps_default_envelope_and_frees_results() {
    let reply = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::answering(move |_| {
        Canned::ok(reply)
            .asking("x-envoy-upstream-service-time", "7")
            .asking("x-typesafe-request-id", "req-c-one")
    })
    .expect("loopback");
    let base = listener.base();
    let settings = json!({"base_url":base,"cache":false}).to_string();
    let default = json!({"decide":"Is it relevant?","evidence":"alpha"}).to_string();
    let opted = json!({"decide":"Is it relevant?","evidence":"beta","attempts":true}).to_string();
    let empty = json!({"filter":"Is it relevant?","records":[],"attempts":true}).to_string();
    let invalid =
        json!({"decide":"Is it relevant?","evidence":"beta","attempts":false}).to_string();
    let invalid_null =
        json!({"decide":"Is it relevant?","evidence":"beta","attempts":null}).to_string();
    let invalid_text =
        json!({"decide":"Is it relevant?","evidence":"beta","attempts":"true"}).to_string();
    let invalid_usage = json!({"usage":true,"attempts":true}).to_string();
    let mut script = Script::default();
    script.ask("settings", &[base, &settings]);
    for request in [
        &default,
        &opted,
        &empty,
        &invalid,
        &invalid_null,
        &invalid_text,
        &invalid_usage,
    ] {
        script.ask("call", &[base, request]);
    }
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    assert_eq!(got.len(), 8);
    let ordinary: Value = serde_json::from_str(&got[1].1).expect("default success");
    assert_eq!(ordinary.as_object().expect("object").len(), 2);
    let detailed: Value = serde_json::from_str(&got[2].1).expect("opted success");
    assert_eq!(detailed.as_object().expect("object").len(), 3);
    assert_eq!(detailed["attempts"][0]["ordinal"], 1);
    assert_eq!(detailed["attempts"][0]["outcome"], "ok");
    assert_eq!(detailed["attempts"][0]["status"], 200);
    assert_eq!(detailed["attempts"][0]["server_ms"], 7);
    assert_eq!(detailed["attempts"][0]["request_id"], "req-c-one");
    let empty: Value = serde_json::from_str(&got[3].1).expect("empty success");
    assert_eq!(empty["attempts"], json!([]));
    assert_eq!(got[4].0, 1, "false is not the opt-in");
    assert_eq!(got[5].0, 1, "null is not the opt-in");
    assert_eq!(got[6].0, 1, "a string is not the opt-in");
    assert_eq!(got[7].0, 1, "usage takes no attempt control");
    assert_eq!(listener.count(), 2, "invalid and empty calls send nothing");
}

#[test]
fn direct_c_attempt_member_is_available_on_each_of_ten_json_verbs() {
    let listener = Listener::answering(|_| Canned::ok(
        r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","choice":"u001","probabilities":{"u001":0.9,"u002":0.1}}}}"#,
    )).expect("loopback");
    let base = listener.base();
    let settings = json!({"base_url":base,"cache":false}).to_string();
    let cases = [
        (
            "decide",
            json!({"decide":"Q?","records":[],"attempts":true}),
        ),
        (
            "choose",
            json!({"choose":"Q?","options":["a","b"],"records":[],"attempts":true}),
        ),
        (
            "score",
            json!({"score":"Q?","levels":["low","high"],"records":[],"attempts":true}),
        ),
        (
            "tag",
            json!({"tag":"Q?","labels":["a","b"],"records":[],"attempts":true}),
        ),
        (
            "filter",
            json!({"filter":"Q?","records":[],"attempts":true}),
        ),
        ("rank", json!({"rank":"Q?","records":[],"attempts":true})),
        (
            "find",
            json!({"find":"Which?","units":["a","b"],"attempts":true}),
        ),
        (
            "annotate",
            json!({"annotate":{"version":1,"questions":{"one":{"decide":"Q?"}}},"records":[],"attempts":true}),
        ),
        (
            "recognize",
            json!({"version":1,"recognize":{"kinds":{"person":null}},"evidence":"","attempts":true}),
        ),
        (
            "relate",
            json!({"version":1,"relate":{"relations":[{"name":"linked","source":"person","target":"person"}]},"records":[],"attempts":true}),
        ),
    ];
    let mut script = Script::default();
    script.ask("settings", &[base, &settings]);
    for (_, request) in &cases {
        script.ask("call", &[base, &request.to_string()]);
    }
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    assert_eq!(got.len(), cases.len() + 1);
    for ((verb, _), reply) in cases.iter().zip(&got[1..]) {
        assert_eq!(reply.0, 0, "{verb}: {}", reply.1);
        let success: Value = serde_json::from_str(&reply.1).expect("success JSON");
        assert_eq!(success.as_object().expect("object").len(), 3, "{verb}");
        assert!(success["attempts"].is_array(), "{verb}");
    }
    assert_eq!(listener.count(), 1, "only find has a nonempty request");
}

#[test]
fn direct_c_staged_recognize_and_relate_keep_live_attempts() {
    let backend = Backend::start().expect("saved conformance backend");
    let base = format!("{}/generic/v1", backend.origin());
    let settings = json!({"base_url":base,"cache":false}).to_string();
    let recognize = json!({"version":1,"recognize":{"kinds":{"person":null}},
        "evidence":"Maria Chen arrived.","attempts":true})
    .to_string();
    let relate = json!({"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]},
        "records":[{"name":"Maria Chen","kind":"person"},{"name":"arrived.","kind":"person"}],"attempts":true}).to_string();
    let mut script = Script::default();
    script.ask("settings", &[&base, &settings]);
    script.ask("call", &[&base, &recognize]);
    script.ask("call", &[&base, &relate]);
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    assert_eq!(got.len(), 3);
    for (verb, reply) in ["recognize", "relate"].into_iter().zip(&got[1..]) {
        assert_eq!(reply.0, 0, "{verb}: {}", reply.1);
        let value: Value = serde_json::from_str(&reply.1).expect("success JSON");
        let attempts = value["attempts"].as_array().expect("opted live attempts");
        assert!(!attempts.is_empty(), "{verb} sent a real request");
        assert!(
            attempts.iter().all(|attempt| attempt["status"] == 200),
            "{verb}"
        );
    }
}

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
    let keys = |row: &Value| row["meta"]["requests"].as_array().map(Vec::len);
    assert_eq!(
        keys(&wrapped["value"][0]),
        Some(1),
        "one question key a row"
    );
    assert_ne!(
        wrapped["value"][0]["meta"]["requests"],
        wrapped["value"][1]["meta"]["requests"]
    );
    assert!(wrapped["value"][1]["meta"].get("batch").is_none());
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
    assert!(contextual["value"][1]["meta"]["requests"][0].is_string());
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

/// An annotate record read by an `on` question nests at most 127 levels. At
/// 128 the door refuses it by that limit, with the usage code, and sends nothing.
#[test]
fn direct_c_annotate_record_nests_at_most_127_levels() {
    let reply = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::answering(move |_| Canned::ok(reply)).expect("loopback");
    let base = listener.base();
    let settings = json!({"base_url":base,"cache":false}).to_string();
    let set = json!({"version":1,"questions":{"one":{"decide":"Q?","on":"/a"}}});
    let record = |depth: usize| {
        let inner = depth - 1;
        format!("{{\"a\":{}1{}}}", "[".repeat(inner), "]".repeat(inner))
    };
    let mut script = Script::default();
    script.ask("settings", &[base, &settings]);
    for depth in [127, 128] {
        let request = json!({"annotate":set,"records":[record(depth)]}).to_string();
        script.ask("call", &[base, &request]);
    }
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    assert_eq!(got.len(), 3);
    assert_eq!(got[1].0, 0, "{}", got[1].1);
    assert_eq!(
        got[2],
        (
            1,
            "the JSON nests more than 127 levels of arrays and objects, the most this tool reads"
                .to_owned()
        )
    );
    assert_eq!(listener.count(), 1, "the refused record sends nothing");
}

/// Debt 031: relate reads each record through the one record parser, so a
/// record 128 levels deep gets the depth sentence and the door keeps its own
/// two relate sentences for text that is not JSON and for a wrong shape.
#[test]
fn direct_c_relate_record_reads_through_the_one_parser() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("loopback");
    let base = listener.base();
    let settings = json!({"base_url":base,"cache":false}).to_string();
    let rule = r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]}}"#;
    let deep = |depth: usize| {
        let inner = depth - 1;
        let nested = format!("{}1{}", "[".repeat(inner), "]".repeat(inner));
        format!(r#"{{"name":"Ada","kind":"person","x":{nested}}}"#)
    };
    let shape = "a relate record is a JSON object with a string name and a string kind";
    let rows = [
        (deep(127), 0, r#"{"edges":[]}"#),
        (
            deep(128),
            1,
            "the JSON nests more than 127 levels of arrays and objects, the most this tool reads",
        ),
        ("not json".to_owned(), 1, "a relate record is not JSON"),
        (r#"["Ada","person"]"#.to_owned(), 1, shape),
        (r#"{"name":"Ada"}"#.to_owned(), 1, shape),
        (
            r#"{"name":"Ada","name":"Bea","kind":"person"}"#.to_owned(),
            1,
            "a JSON record holds each member name once, and one name arrived twice",
        ),
    ];
    let mut script = Script::default();
    script.ask("settings", &[base, &settings]);
    for (record, _, _) in &rows {
        script.ask("relate", &[base, rule, record]);
    }
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let got = replies(&output.stdout).expect("framed replies");
    let wanted: Vec<Reply> = rows
        .iter()
        .map(|(_, code, said)| (*code, (*said).to_owned()))
        .collect();
    assert_eq!(got[1..], wanted[..]);
    assert_eq!(
        listener.count(),
        0,
        "one record or a refused one sends nothing"
    );
}
