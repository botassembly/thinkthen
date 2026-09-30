//! The request body a question's two homes send, side by side on the wire.

use std::fs;
use std::path::PathBuf;

use crate::harness::{Canned, Listener, spawn};

/// The response a backend gives to the one yes/no question these cases ask.
const DECIDED: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// The response a backend gives to the one pick these cases ask.
const PICKED: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"billing","#,
    r#""probabilities":{"billing":0.8,"shipping":0.2}}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// Write one question file under the test target directory and name its path.
fn written(name: &str, text: &str) -> String {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("asked");
    let _ = fs::create_dir_all(&folder);
    let path = folder.join(format!("{name}.json"));
    let _ = fs::write(&path, text);
    format!("@{}", path.display())
}

/// Run one command against a listener that answers once, and read the body sent.
fn sent(arguments: &[&str], answer: &str) -> Option<Vec<u8>> {
    let listener = Listener::serving(vec![Canned::ok(answer)]).ok()?;
    let asked = ["--url", listener.base(), "--model", "local-1"];
    let output = spawn(
        &[arguments, &asked[..]].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"Refund me please.",
    )
    .ok()?;
    let printed = String::from_utf8_lossy(&output.stdout).into_owned();
    let said = String::from_utf8_lossy(&output.stderr).into_owned();
    assert_eq!(output.status.code(), Some(0), "{said}");
    // Every case on this page runs with a key set, so each one is also a
    // secrecy case over the paths the question file opened.
    assert!(!printed.contains("sk-test-value"), "{printed}");
    assert!(!said.contains("sk-test-value"), "{said}");
    let body = listener.requests().into_iter().next().map(|one| one.body);
    // Two cases that both reached no listener would compare equal, so a miss
    // fails here instead of passing as an agreement.
    assert!(body.is_some(), "no request reached the listener");
    body
}

/// Read a request body back as text, and say so when no request arrived.
///
/// A case that reaches the listener pins the body it sent. A case that never
/// reached it fails on this sentence instead of on an empty string.
fn text(body: Option<Vec<u8>>) -> String {
    body.map_or_else(
        || "no request reached the listener".to_owned(),
        |bytes| String::from_utf8_lossy(&bytes).into_owned(),
    )
}

#[test]
fn the_two_texts_travel_with_the_question_and_come_from_either_home() {
    let file = written(
        "refund",
        concat!(
            r#"{"decide":"asks for a refund","#,
            r#""true":"The writer asks for money back.","#,
            r#""false":"The writer asks for anything else."}"#,
        ),
    );
    let typed = sent(
        &[
            "decide",
            "asks for a refund",
            "--true",
            "The writer asks for money back.",
            "--false",
            "The writer asks for anything else.",
        ],
        DECIDED,
    );

    assert_eq!(
        text(typed.clone()),
        concat!(
            r#"{"state":"Each question quotes the text it asks about.","model":"local-1","#,
            r#""questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me please.\". asks for a refund","#,
            r#""criteria":{"true":"The writer asks for money back.","#,
            r#""false":"The writer asks for anything else."}}}}"#,
        )
    );
    assert_eq!(typed, sent(&["decide", &file], DECIDED));
}

#[test]
fn one_text_alone_sends_one_field_and_nothing_for_the_other() {
    let body = text(sent(
        &[
            "decide",
            "asks for a refund",
            "--true",
            "The writer asks for money back.",
        ],
        DECIDED,
    ));
    assert!(
        body.contains(r#""criteria":{"true":"The writer asks for money back."}}"#),
        "{body}"
    );
    assert!(!body.contains(r#""false""#), "{body}");
}

#[test]
fn a_question_with_no_new_option_sends_the_request_it_always_sent() {
    let file = written("plain", r#"{"decide":"asks for a refund"}"#);
    let typed = sent(&["decide", "asks for a refund"], DECIDED);

    assert_eq!(
        text(typed.clone()),
        concat!(
            r#"{"state":"Each question quotes the text it asks about.","model":"local-1","#,
            r#""questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me please.\". asks for a refund"}}}"#,
        )
    );
    assert_eq!(typed, sent(&["decide", &file], DECIDED));
}

#[test]
fn a_description_per_option_travels_and_comes_from_either_home() {
    let file = written(
        "teams",
        concat!(
            r#"{"choose":"which team owns this","#,
            r#""options":{"billing":"Money and invoices.","shipping":"Parcels and dates."}}"#,
        ),
    );
    let typed = sent(
        &[
            "choose",
            "which team owns this",
            "--option",
            "billing=Money and invoices.",
            "--option",
            "shipping=Parcels and dates.",
        ],
        PICKED,
    );

    assert_eq!(
        text(typed.clone()),
        concat!(
            r#"{"state":"Each question quotes the text it asks about.","model":"local-1","#,
            r#""questions":{"q1":{"type":"choice","instructions":"The text is \"Refund me please.\". which team owns this","#,
            r#""criteria":{"billing":"Money and invoices.","#,
            r#""shipping":"Parcels and dates."}}}}"#,
        )
    );
    assert_eq!(typed, sent(&["choose", &file], PICKED));
}

#[test]
fn an_option_with_no_description_still_sends_a_null_under_its_label() {
    let file = written(
        "bare",
        r#"{"choose":"which team owns this","options":["billing","shipping"]}"#,
    );
    let typed = sent(
        &["choose", "which team owns this", "billing", "shipping"],
        PICKED,
    );

    assert!(
        text(typed.clone()).contains(r#""criteria":{"billing":null,"shipping":null}"#),
        "{}",
        text(typed.clone())
    );
    assert_eq!(typed, sent(&["choose", &file], PICKED));
}

#[test]
fn a_placement_reads_its_levels_from_either_home_and_sends_one_body() {
    let answered = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"score","score":0.4,"#,
        r#""legend":{"0":"None.","1":"Some.","2":"Blocked."},"#,
        r#""probabilities":{"0":0.7,"1":0.2,"2":0.1}}},"#,
        r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
    );
    let file = written(
        "levels",
        r#"{"score":"how much disruption","levels":["None.","Some.","Blocked."]}"#,
    );
    let typed = sent(
        &["score", "how much disruption", "None.", "Some.", "Blocked."],
        answered,
    );

    assert!(
        text(typed.clone()).contains(r#""criteria":["None.","Some.","Blocked."]"#),
        "{}",
        text(typed.clone())
    );
    assert_eq!(typed, sent(&["score", &file], answered));
}

#[test]
fn a_score_map_sends_descriptions_in_order_and_reports_the_names() {
    let file = written(
        "levels-map",
        concat!(
            r#"{"score":"how much disruption","levels":{"#,
            r#""None.":{"what":"No impact."},"Some.":"Partial.","Blocked.":null}}"#,
        ),
    );
    let answered = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"score","score":0.4,"#,
        r#""legend":{"0":"None.","1":"Some.","2":"Blocked."},"#,
        r#""probabilities":{"0":0.7,"1":0.2,"2":0.1}}},"#,
        r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
    );
    let listener = Listener::serving(vec![Canned::ok(answered)]).expect("a loopback listener");
    let output = spawn(
        &[
            "score",
            &file,
            "--details",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        String::from_utf8_lossy(&requests[0].body),
        concat!(
            r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"score","#,
            r#""instructions":"The text is \"Refund me please.\". how much disruption","#,
            r#""criteria":[{"what":"No impact."},"Partial.",{}]}}}"#,
        )
    );
    let printed = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(
        printed.contains(r#""levels":["None.","Some.","Blocked."]"#),
        "{printed}"
    );
    assert!(printed.contains(r#""level":"None.""#), "{printed}");
    assert!(
        printed.contains(r#""probabilities":{"None.":0.7,"Some.":0.2,"Blocked.":0.1}"#),
        "{printed}"
    );
    assert!(printed.contains(r#""value":0.4"#), "{printed}");
}

#[test]
fn a_typed_setting_beside_a_file_reaches_the_wire() {
    let file = written(
        "override",
        r#"{"decide":"asks for a refund","true":"The writer asks for money back."}"#,
    );
    let body = text(sent(&["decide", &file, "--true", "Money back."], DECIDED));
    assert!(
        body.contains(r#""criteria":{"true":"Money back."}"#),
        "{body}"
    );
}
