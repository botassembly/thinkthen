//! The door's exact reply bytes, pinned before the door moved to serde types.
//!
//! One driver run asks every verb through `thinkthen_call` on the shared
//! backend's full arm, with prices set, so every optional facts key prints.
//! It also asks a null answer, a per-record annotate failure, the usage
//! counters, the attempts, and a failed call with its message and error
//! facts. Elapsed times, request digests, and the
//! loopback origin vary, so they print as 0 and `ORIGIN`.

use conformance_backend::Backend;
use serde_json::json;

use crate::cases::{Script, replies};
use crate::{compile, crate_dir, run, text};

const SETTINGS: &str =
    r#"{"cache":false,"max_retries":0,"usd_per_million_input":"0.25","usd_per_million_output":"0.25"}"#;

const FACTS: &str = r#""facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":1,"model":"jev-1.13.0","output_tokens":1,"records":1,"requests_sent":1,"seconds":0}"#;

#[test]
fn every_door_reply_keeps_its_bytes() {
    let backend = Backend::start().expect("the conformance backend");
    let full = format!("{}/arm/full/v1", backend.origin());
    let broken = format!("{}/arm/malformed/missing_answer/v1", backend.origin());
    let set = json!({"version":1,"questions":{
        "refund":{"decide":"Refund?","threshold":0.5},
        "team":{"choose":"Team?","options":["billing","other"],"threshold":0.5}}});
    let relation = json!({"relations":[{"name":"linked","source":"person","target":"person"}]});
    let asked = [
        json!({"decide":"Refund?","evidence":"Money back."}),
        json!({"decide":"Refund?","evidence":"Money back.","details":true}),
        json!({"decide":"Refund?","threshold":"0.8:0.95","evidence":"Money back."}),
        json!({"choose":"Team?","options":["billing","other"],"evidence":"Money back."}),
        json!({"score":"Severe?","levels":["low","high"],"evidence":"Money back."}),
        json!({"tag":"Labels?","labels":["billing","urgent"],"threshold":0.5,"evidence":"Money back."}),
        json!({"decide":"Refund?","records":["one","two"]}),
        json!({"filter":"Refund?","records":["one","two"]}),
        json!({"rank":"Refund?","records":["one","two"]}),
        json!({"find":"Which asks?","units":["one","two"]}),
        json!({"annotate":set,"records":["one"]}),
        json!({"version":1,"recognize":{"kinds":{"person":null}},"evidence":"Ada"}),
        json!({"version":1,"relate":relation,"records":[{"name":"Ada","kind":"person"},{"name":"Bea","kind":"person"}]}),
        json!({"decide":"Refund?","evidence":"Money back.","attempts":true}),
        json!({"usage":true}),
    ];
    let mut script = Script::default();
    script.ask("settings", &[&full, SETTINGS]);
    for request in &asked {
        script.ask("call", &[&full, &request.to_string()]);
    }
    script.ask("settings", &[&broken, SETTINGS]);
    script.ask("call", &[&broken, &json!({"annotate":set,"records":["one"]}).to_string()]);
    script.ask("call", &[&broken, &asked[0].to_string()]);
    script.ask("facts", &[&broken, "-"]);
    let output = run(&compile(&crate_dir().join("tests/c/driver.c")), "", &script.0);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let said: Vec<String> = replies(&output.stdout)
        .expect("framed replies")
        .into_iter()
        .map(|(code, body)| format!("{code} {}", steady(&body.replace(backend.origin(), "ORIGIN"))))
        .collect();
    let expected: Vec<String> = GOLDEN.iter().map(|line| line.replace("{FACTS}", FACTS)).collect();
    assert_eq!(said, expected);
}

/// The reply with each elapsed time and request digest printed as 0. A
/// digest covers the request, which names the loopback port.
fn steady(body: &str) -> String {
    const VARYING: [&str; 5] = ["\"seconds\":", "\"wall_ms\":", "\"server_ms\":", "\"request_sha256\":\"", "\"requests\":[\""];
    let mut out = String::new();
    let mut rest = body;
    while let Some(at) = VARYING.iter().filter_map(|key| rest.find(key).map(|at| at + key.len())).min() {
        out.push_str(&rest[..at]);
        out.push('0');
        rest = rest[at..].trim_start_matches(|c: char| c.is_ascii_hexdigit() || matches!(c, '.' | '-'));
    }
    out + rest
}

const GOLDEN: [&str; 20] = [
    r#"0 "#,
    r#"0 {"value":true,{FACTS}}"#,
    r#"0 {"value":{"schema":"thinkthen.result/1","value":true,"question":{"verb":"decide","text":"Refund?"},"answer":{"kind":"yes_no","probability":0.9},"threshold":0.5,"meta":{"tool":"thinkthen 0.0.1","question_sha256":"6c0b2c1ba8577c1d9d8df9ee08c9cce188ec4283e097cd37f5944518f2ce8317","url":"ORIGIN/arm/full/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":1,"output_tokens":1},"requests_sent":1,"cached":false,"requests":["0"],"failed_questions":0}},{FACTS}}"#,
    r#"0 {"value":null,{FACTS}}"#,
    r#"0 {"value":"billing",{FACTS}}"#,
    r#"0 {"value":0.1,{FACTS}}"#,
    r#"0 {"value":["billing","urgent"],{FACTS}}"#,
    r#"0 {"value":[true,true],"facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":1,"model":"jev-1.13.0","output_tokens":1,"records":2,"requests_sent":1,"seconds":0}}"#,
    r#"0 {"value":["one","two"],"facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":1,"model":"jev-1.13.0","output_tokens":1,"records":2,"requests_sent":1,"seconds":0}}"#,
    r#"0 {"value":["one","two"],"facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":1,"model":"jev-1.13.0","output_tokens":1,"records":2,"requests_sent":1,"seconds":0}}"#,
    r#"0 {"value":"one",{FACTS}}"#,
    r#"0 {"value":[{"refund":true,"team":"billing"}],{FACTS}}"#,
    r#"0 {"value":{"entities":[]},"facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":2,"model":"jev-1.13.0","output_tokens":2,"records":1,"requests_sent":2,"seconds":0}}"#,
    r#"0 {"value":{"edges":[{"relation":"linked","source":{"name":"Ada","kind":"person"},"target":{"name":"Bea","kind":"person"},"probability":0.9},{"relation":"linked","source":{"name":"Bea","kind":"person"},"target":{"name":"Ada","kind":"person"},"probability":0.9}]},{FACTS}}"#,
    r#"0 {"value":true,{FACTS},"attempts":[{"ordinal":1,"request_sha256":"0","wall_ms":0,"outcome":"ok","status":200}]}"#,
    r#"0 {"requests_sent":15,"retries":0,"input_tokens":15,"output_tokens":15,"cache_answers":0}"#,
    r#"0 "#,
    r#"0 {"value":[{"refund":true,"team":{"failed":{"kind":"backend","cause":"missing_answer"}}}],"facts":{"cache_answers":0,"model":"jev-1.13.0","records":1,"requests_sent":1,"seconds":0}}"#,
    r#"2 the reply was refused: the response carries no answer for question `q1`"#,
    r#"0 {"cache_answers":0,"records":0,"requests_sent":1,"seconds":0}"#,
];
