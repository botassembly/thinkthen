//! CLI attachments and whole-file image items use native ordered image semantics.
use crate::harness::{Canned, Listener, spawn};
use serde_json::{Value, json};
use std::{fs, path::PathBuf, process::Output};

mod identity;

const REPLY: &str =
    include_str!("../../../../specification/fixtures/images/liquid-decide-reply.json");
pub(super) fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/images")
        .join(name)
}
fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}
#[expect(
    clippy::expect_used,
    reason = "invalid shared fixtures or loopback setup stop this behavioral test"
)]
fn call(listener: &Listener, args: &[&str], input: &[u8]) -> Output {
    let mut arguments = args.to_vec();
    arguments.extend([
        "--backend",
        "liquid",
        "--model",
        "d1",
        "--url",
        listener.base(),
        "--no-cache",
        "--max-retries",
        "0",
    ]);
    spawn(&arguments, &[("THINKTHEN_API_KEY", "sk-image-test")], input)
        .expect("valid saved fixture or loopback configuration")
}
#[expect(
    clippy::expect_used,
    reason = "invalid shared fixtures or loopback setup stop this behavioral test"
)]
fn data_url(name: &str) -> String {
    use base64::Engine as _;
    let bytes = fs::read(fixture(name)).expect("valid saved fixture or loopback configuration");
    format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    )
}
#[test]
fn attachments_keep_original_order_duplicates_text_and_typed_details() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let red = fixture("red.png");
    let blue = fixture("blue.png");
    let output = call(
        &listener,
        &[
            "decide",
            "Is red visible?",
            "--image",
            red.to_str().unwrap(),
            "--image",
            blue.to_str().unwrap(),
            "--image",
            red.to_str().unwrap(),
            "--details",
            "--facts",
        ],
        b"Compare originals.",
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let row: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["value"], true);
    assert_eq!(row["input"]["text"], "Compare originals.");
    assert_eq!(row["input"]["images"].as_array().unwrap().len(), 3);
    assert!(row["position"].get("first").is_none());
    assert_eq!(row["position"]["images"], json!([red, blue, red]));
    let requests = listener.requests();
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(
        body,
        json!({"state":"Compare originals.","model":"d1","questions":{"q1":{"type":"noul","instructions":"Is red visible?"}},"images":[data_url("red.png"),data_url("blue.png"),data_url("red.png")]})
    );
    assert_eq!(listener.count(), 1);
    assert!(!text(&output.stderr).contains("sk-image-test"));
    assert!(text(&output.stderr).contains("\"input_tokens\":123"));
}
#[test]
fn whole_image_files_are_separate_located_rows_and_later_failure_keeps_the_first() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let red = fixture("red.png");
    let blue = fixture("blue.png");
    let args = [
        "decide",
        "Is red visible?",
        "--input",
        red.to_str().unwrap(),
        "--input",
        blue.to_str().unwrap(),
        "--unit",
        "file",
        "--media",
        "image",
        "--details",
    ];
    let output = call(&listener, &args, b"ignored");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let rows: Vec<Value> = text(&output.stdout)
        .lines()
        .map(|row| serde_json::from_str(row).unwrap())
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["file"], json!(red));
    assert_eq!(rows[1]["file"], json!(blue));
    for row in &rows {
        assert!(row.get("first_line").is_none());
        assert!(row.get("last_line").is_none());
        assert!(row["position"].get("first").is_none());
    }
    assert_eq!(listener.count(), 2);
    for request in listener.requests() {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body["state"], "");
        assert_eq!(body["images"].as_array().unwrap().len(), 1);
    }
    let missing = fixture("no-such-image.png");
    let output = call(
        &listener,
        &[
            "decide",
            "Is red visible?",
            "--input",
            red.to_str().unwrap(),
            "--input",
            missing.to_str().unwrap(),
            "--unit",
            "file",
            "--media",
            "image",
            "--details",
        ],
        b"",
    );
    // Manifest enumeration retains its established eager failure behavior.
    assert_eq!(output.status.code(), Some(5));
    let malformed = fixture("malformed-pixels.png");
    let output = call(
        &listener,
        &[
            "decide",
            "Is red visible?",
            "--input",
            red.to_str().unwrap(),
            "--input",
            malformed.to_str().unwrap(),
            "--unit",
            "file",
            "--media",
            "image",
            "--details",
        ],
        b"",
    );
    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout).lines().count(), 1);
    assert_eq!(listener.count(), 3);
}
#[test]
fn unsupported_functions_contradictory_modes_and_malformed_attachments_send_nothing() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    for (function, args) in [
        ("tag", vec!["tag", "Q?", "red"]),
        ("filter", vec!["filter", "Q?"]),
        ("rank", vec!["rank", "Q?"]),
        ("annotate", vec!["annotate", "missing.json"]),
        ("find", vec!["find", "Q?"]),
        ("recognize", vec!["recognize", "Q?", "thing"]),
        ("relate", vec!["relate", "missing.json"]),
    ] {
        let mut args = args;
        args.extend(["--image", "does-not-exist.jpg"]);
        let output = call(&listener, &args, b"PRIVATE_IMAGE_CAPTION");
        assert_eq!(
            output.status.code(),
            Some(2),
            "{function}: {}",
            text(&output.stderr)
        );
        assert_eq!(
            text(&output.stderr),
            format!("thinkthen: {function} accepts text only; images are unsupported\n")
        );
        assert!(output.stdout.is_empty());
    }
    let red = fixture("red.png");
    let bad = fixture("progressive-app14-one-component.jpg");
    for args in [
        vec!["decide", "Q?", "--image", red.to_str().unwrap(), "--lines"],
        vec![
            "decide",
            "Q?",
            "--input",
            red.to_str().unwrap(),
            "--media",
            "image",
            "--unit",
            "line",
        ],
        vec![
            "decide",
            "Q?",
            "--image",
            red.to_str().unwrap(),
            "--image",
            bad.to_str().unwrap(),
        ],
    ] {
        let output = call(&listener, &args, b"PRIVATE_IMAGE_CAPTION");
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        assert!(output.stdout.is_empty());
        assert!(!text(&output.stderr).contains("PRIVATE_IMAGE_CAPTION"));
    }
    let mut excessive = vec!["decide", "Q?"];
    for _ in 0..9 {
        excessive.extend(["--image", "does-not-exist.jpg"]);
    }
    let output = call(&listener, &excessive, b"PRIVATE_IMAGE_CAPTION");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        text(&output.stderr),
        "thinkthen: image evidence requires 1 to 8 images\n"
    );
    assert!(output.stdout.is_empty());
    assert_eq!(listener.count(), 0);
}

#[test]
fn choose_and_score_and_image_only_decide_use_each_hosted_protocol() {
    let red = fixture("red.png");
    for (backend, model) in [("liquid", "d1"), ("perplexity", "pplx-decider-v1-27b")] {
        for (verb, mut args, response, value, questions) in [
            (
                "decide",
                vec!["decide", "Is red visible?"],
                REPLY,
                json!(true),
                json!({"q1":{"type":"noul","instructions":"Is red visible?"}}),
            ),
            (
                "choose",
                vec!["choose", "Which color?", "red", "blue"],
                include_str!("../../../../specification/fixtures/images/liquid-choose-reply.json"),
                json!("red"),
                json!({"q1":{"type":"choice","instructions":"Which color?","criteria":{"red":null,"blue":null}}}),
            ),
            (
                "score",
                vec!["score", "How red?", "none", "all"],
                include_str!("../../../../specification/fixtures/images/liquid-score-reply.json"),
                json!(0.8),
                json!({"q1":{"type":"score","instructions":"How red?","criteria":["none","all"]}}),
            ),
        ] {
            // The Perplexity fixtures preserve the literal returned model.
            let response = if backend == "perplexity" {
                response.replace("\"d1\"", "\"pplx-decider-v1-27b\"")
            } else {
                response.to_owned()
            };
            let listener = Listener::answering(move |_| Canned::ok(&response)).unwrap();
            args.extend([
                "--backend",
                backend,
                "--model",
                model,
                "--url",
                listener.base(),
                "--no-cache",
                "--image",
                red.to_str().unwrap(),
            ]);
            let output = spawn(&args, &[("THINKTHEN_API_KEY", "sk-image-test")], b"").unwrap();
            assert_eq!(
                output.status.code(),
                Some(0),
                "{verb}: {}",
                text(&output.stderr)
            );
            assert_eq!(
                serde_json::from_slice::<Value>(&output.stdout).unwrap(),
                value
            );
            let actual: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
            let expected = if backend == "liquid" {
                json!({"model":model,"state":"","questions":questions,"images":[data_url("red.png")]})
            } else {
                json!({"model":model,"state":[{"type":"image_url","image_url":{"url":data_url("red.png")}}],"questions":questions})
            };
            assert_eq!(actual, expected);
            assert_eq!(listener.count(), 1);
        }
    }
}

#[test]
fn cli_above_spike_body_ignores_soft_default_but_respects_explicit_limit() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let large = fixture("above-spike.png");
    let output = call(
        &listener,
        &[
            "decide",
            "Is red visible?",
            "--image",
            large.to_str().unwrap(),
        ],
        b"",
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let requests = listener.requests();
    assert!(requests[0].body.len() > 96_000);
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["images"], json!([data_url("above-spike.png")]));
    let output = call(
        &listener,
        &[
            "decide",
            "Is red visible?",
            "--image",
            large.to_str().unwrap(),
            "--max-request-bytes",
            "96000",
        ],
        b"",
    );
    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    assert_eq!(listener.count(), 1);
}
