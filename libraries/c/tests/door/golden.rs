//! The door's exact reply bytes, pinned before the door moved to serde types.
//!
//! One driver run asks every verb through `thinkthen_call` on the shared
//! backend's full arm, with prices set, so every optional facts key prints.
//! It also asks a null answer, a per-record annotate failure, the usage
//! counters, the attempts, and a failed call with its message and error
//! facts. Elapsed times, request digests, and the loopback origin vary, so
//! they print as 0 and `ORIGIN`.

use conformance_backend::Backend;
use serde_json::json;

use crate::cases::{Script, replies};
use crate::{compile, crate_dir, run, text};

const SETTINGS: &str = r#"{"cache":false,"max_retries":0,"usd_per_million_input":"0.25","usd_per_million_output":"0.25"}"#;

const FACTS: &str = r#""facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":1,"model":"jev-1.13.0","output_tokens":1,"records":1,"requests_sent":1,"seconds":0}"#;

#[test]
fn every_door_reply_keeps_its_bytes() {
    let backend = Backend::start().expect("the conformance backend");
    let full = format!("{}/arm/full/capture/v1", backend.origin());
    let broken = format!("{}/arm/malformed/missing_answer/v1", backend.origin());
    let set = json!({"version":1,"questions":{
        "refund":{"decide":"Refund?","threshold":0.5},
        "team":{"choose":"Team?","options":["billing","other"],"threshold":0.5}}});
    let relation = json!({"relations":[{"name":"linked","source":"person","target":"person"}]});
    let both_ways =
        json!({"relations":[{"name":"met","source":"person","target":"person","either":true}]});
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
        json!({"version":1,"relate":both_ways,"records":[{"name":"Ada","kind":"person"},{"name":"Bea","kind":"person"}]}),
        json!({"decide":"Refund?","evidence":"Money back.","attempts":true}),
        json!({"usage":true}),
    ];
    let mut script = Script::default();
    script.ask("settings", &[&full, SETTINGS]);
    for request in &asked {
        script.ask("call", &[&full, &request.to_string()]);
    }
    script.ask("settings", &[&broken, SETTINGS]);
    script.ask(
        "call",
        &[
            &broken,
            &json!({"annotate":set,"records":["one"]}).to_string(),
        ],
    );
    script.ask("call", &[&broken, &asked[0].to_string()]);
    script.ask("facts", &[&broken, "-"]);
    let output = run(
        &compile(&crate_dir().join("tests/c/driver.c")),
        "",
        &script.0,
    );
    assert!(output.status.success(), "{}", text(&output.stderr));
    let said: Vec<String> = replies(&output.stdout)
        .expect("framed replies")
        .into_iter()
        .map(|(code, body)| {
            format!(
                "{code} {}",
                steady(
                    &body
                        .replace(backend.origin(), "ORIGIN")
                        .replace("/arm/full/capture/", "/arm/full/")
                )
            )
        })
        .collect();
    let captured: serde_json::Value = serde_json::from_str(&backend.capture()).expect("capture");
    assert_eq!(captured["bodies"], json!(REQUESTS));
    assert_eq!(backend.count(), REQUESTS.len() + 2);
    // The malformed arm sends the same encoded annotation and scalar bodies.
    let failure_capture = Backend::start().expect("failure request capture");
    let failure_base = format!("{}/arm/full/capture/v1", failure_capture.origin());
    let mut failure_script = Script::default();
    failure_script.ask("settings", &[&failure_base, SETTINGS]);
    for request in [&asked[10], &asked[0]] {
        failure_script.ask("call", &[&failure_base, &request.to_string()]);
    }
    let failure_output = run(
        &compile(&crate_dir().join("tests/c/driver.c")),
        "",
        &failure_script.0,
    );
    assert!(
        failure_output.status.success(),
        "{}",
        text(&failure_output.stderr)
    );
    let captured: serde_json::Value =
        serde_json::from_str(&failure_capture.capture()).expect("capture");
    assert_eq!(captured["bodies"], json!([ANNOTATE, DECIDE]));
    assert_eq!(failure_capture.count(), 2);
    assert_eq!(said, expected());
}

fn expected() -> Vec<String> {
    GOLDEN
        .iter()
        .zip([
            &[][..], &[DECIDE], &[DECIDE], &[DECIDE], &[CHOOSE], &[SCORE], &[TAG],
            &[RECORDS], &[RECORDS], &[RECORDS], &[FIND], &[ANNOTATE], &[TOKEN, KIND],
            &[RELATE], &[EITHER], &[DECIDE], &[], &[], &[ANNOTATE], &[], &[DECIDE],
        ])
        .map(|(line, bodies)| {
            let largest = bodies.iter().map(|body| body.len()).max().unwrap_or(0);
            // Integer ceiling of the approved encoded-body 0.908 upper estimate.
            let estimated = (largest * 908).div_ceil(1000);
            let added = format!(
                r#""largest_request_bytes":{largest},"largest_request_estimated_input_tokens":{estimated},"token_estimate_method":"encoded-body-bytes-908-v1","seconds":0"#
            );
            line.replace("{FACTS}", FACTS)
                .replace("\"seconds\":0", &added)
                .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
        })
        .collect()
}

/// The reply with each elapsed time and request digest printed as 0. A
/// digest covers the request, which names the loopback port.
fn steady(body: &str) -> String {
    const VARYING: [&str; 5] = [
        "\"seconds\":",
        "\"wall_ms\":",
        "\"server_ms\":",
        "\"request_sha256\":\"",
        "\"requests\":[\"",
    ];
    let mut out = String::new();
    let mut rest = body;
    while let Some(at) = VARYING
        .iter()
        .filter_map(|key| rest.find(key).map(|at| at + key.len()))
        .min()
    {
        out.push_str(&rest[..at]);
        out.push('0');
        rest = rest[at..]
            .trim_start_matches(|c: char| c.is_ascii_hexdigit() || matches!(c, '.' | '-'));
    }
    out + rest
}

const GOLDEN: [&str; 21] = [
    r#"0 "#,
    r#"0 {"value":true,{FACTS}}"#,
    r#"0 {"value":{"schema":"thinkthen.result/1","value":true,"question":{"verb":"decide","text":"Refund?"},"answer":{"kind":"yes_no","probability":0.9},"threshold":0.5,"meta":{"tool":"thinkthen {VERSION}","question_sha256":"6c0b2c1ba8577c1d9d8df9ee08c9cce188ec4283e097cd37f5944518f2ce8317","url":"ORIGIN/arm/full/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":1,"output_tokens":1},"requests_sent":1,"cached":false,"requests":["0"],"failed_questions":0}},{FACTS}}"#,
    r#"0 {"value":null,{FACTS}}"#,
    r#"0 {"value":"billing",{FACTS}}"#,
    r#"0 {"value":0.1,{FACTS}}"#,
    r#"0 {"value":["billing","urgent"],{FACTS}}"#,
    r#"0 {"value":[true,true],"facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":1,"model":"jev-1.13.0","output_tokens":1,"records":2,"requests_sent":1,"seconds":0}}"#,
    r#"0 {"value":["one","two"],"facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":1,"model":"jev-1.13.0","output_tokens":1,"records":2,"requests_sent":1,"seconds":0}}"#,
    r#"0 {"value":[{"index":0,"record":"one","probability":0.9},{"index":1,"record":"two","probability":0.9}],"facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":1,"model":"jev-1.13.0","output_tokens":1,"records":2,"requests_sent":1,"seconds":0}}"#,
    r#"0 {"value":{"index":0,"unit":"one","probability":0.9},{FACTS}}"#,
    r#"0 {"value":[{"refund":true,"team":"billing"}],{FACTS}}"#,
    r#"0 {"value":{"entities":[]},"facts":{"cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":2,"model":"jev-1.13.0","output_tokens":2,"records":1,"requests_sent":2,"seconds":0}}"#,
    r#"0 {"value":{"edges":[{"relation":"linked","source":{"name":"Ada","kind":"person"},"target":{"name":"Bea","kind":"person"},"probability":0.9},{"relation":"linked","source":{"name":"Bea","kind":"person"},"target":{"name":"Ada","kind":"person"},"probability":0.9}]},{FACTS}}"#,
    r#"0 {"value":{"edges":[{"relation":"met","source":{"name":"Ada","kind":"person"},"target":{"name":"Bea","kind":"person"},"probability":0.9,"either":true}]},{FACTS}}"#,
    r#"0 {"value":true,{FACTS},"attempts":[{"ordinal":1,"request_sha256":"0","wall_ms":0,"outcome":"ok","status":200}]}"#,
    r#"0 {"requests_sent":16,"retries":0,"input_tokens":16,"output_tokens":16,"cache_answers":0}"#,
    r#"0 "#,
    r#"0 {"value":[{"refund":true,"team":{"failed":{"kind":"backend","cause":"missing_answer"}}}],"facts":{"cache_answers":0,"model":"jev-1.13.0","records":1,"requests_sent":1,"seconds":0}}"#,
    r#"2 the reply was refused: the response carries no answer for question `q1`"#,
    r#"0 {"cache_answers":0,"records":0,"requests_sent":1,"seconds":0}"#,
];

// Independently pinned encoded requests: wording, readings, order and framing.
const DECIDE: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Money back.\". Refund?"}}}"#;
const CHOOSE: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"The text is \"Money back.\". Team?","criteria":{"billing":null,"other":null}}}}"#;
const SCORE: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"score","instructions":"The text is \"Money back.\". Severe?","criteria":["low","high"]}}}"#;
const TAG: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Money back.\". Labels?\n\nDetermine whether the label \"billing\" applies to this item."},"q2":{"type":"noul","instructions":"The text is \"Money back.\". Labels?\n\nDetermine whether the label \"urgent\" applies to this item."}}}"#;
const RECORDS: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"one\". Refund?"},"q2":{"type":"noul","instructions":"The text is \"two\". Refund?"}}}"#;
const FIND: &str = r#"{"state":"[{\"id\":\"u001\",\"evidence\":\"one\"},{\"id\":\"u002\",\"evidence\":\"two\"}]","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"Which asks?","criteria":{"u001":null,"u002":null}}}}"#;
const ANNOTATE: &str = r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"one\". Refund?"},"q2":{"type":"choice","instructions":"The text is \"one\". Team?","criteria":{"billing":null,"other":null}}}}"#;
const TOKEN: &str = r#"{"state":"Ada","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"Tokens are split at spaces and at each punctuation mark. Where does the [[ ]] token stand in a name of one of these kinds: person? Other names, ordinary words, dates, numbers, and marks that are not part of a name's own spelling are OUT.\n\nSnippet: [[Ada]]","criteria":{"BEGIN":"first token of a name of two or more tokens","INSIDE":"a middle token of a name","END":"last token of a name of two or more tokens","SINGLE":"a one-token name","OUT":"not part of a name"}}}}"#;
const KIND: &str = r#"{"state":"Ada","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"In the text below, some words are wrapped in [[ ]]. Going by what they refer to in this text, which listed kind of name are they? Choose none of these when they are not a proper name, or when they name something that no listed kind covers.\n\nText: [[Ada]]","criteria":{"person":null,"none of these":"They are not a proper name, or no listed kind covers what they name."}}}}"#;
const RELATE: &str = r#"{"state":{"entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Bea","kind":"person"}]},"model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"Is it true that i1 linked i2?"},"q2":{"type":"noul","instructions":"Is it true that i2 linked i1?"}}}"#;
const EITHER: &str = r#"{"state":{"entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Bea","kind":"person"}]},"model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"Is it true that i1 met i2, or that i2 met i1?"}}}"#;

const REQUESTS: [&str; 16] = [
    DECIDE, DECIDE, DECIDE, CHOOSE, SCORE, TAG, RECORDS, RECORDS, RECORDS, FIND, ANNOTATE, TOKEN,
    KIND, RELATE, EITHER, DECIDE,
];
