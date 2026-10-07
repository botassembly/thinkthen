//! Framed captions retain originals, independent controls and every attachment.
use super::{REPLY, call, data_url, fixture, text};
use crate::harness::{Canned, Listener};
use serde_json::{Value, json};

#[test]
fn framed_json_captions_keep_context_originals_and_repeated_image_order() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let red = fixture("red.png");
    let blue = fixture("blue.png");
    let originals = [
        json!({"caption":"First caption","context":"First reference","extra":false}),
        json!({"caption":"Second caption","context":"Second reference","extra":null}),
    ];
    let input = originals
        .iter()
        .map(|record| format!("{record}\n"))
        .collect::<String>();
    let output = call(
        &listener,
        &[
            "decide",
            "Is red visible?",
            "--jsonl",
            "--field",
            "/caption",
            "--context-field",
            "/context",
            "--image",
            red.to_str().unwrap(),
            "--image",
            blue.to_str().unwrap(),
            "--image",
            red.to_str().unwrap(),
            "--details",
        ],
        input.as_bytes(),
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let rows: Vec<Value> = text(&output.stdout)
        .lines()
        .map(|row| serde_json::from_str(row).unwrap())
        .collect();
    assert_eq!(rows.len(), 2);
    for (row, original) in rows.iter().zip(&originals) {
        assert_eq!(&row["input"], original);
        assert_eq!(row["images"].as_array().unwrap().len(), 3);
    }
    let sent = listener.requests();
    assert_eq!(sent.len(), 2);
    let mut bodies: Vec<Value> = sent
        .iter()
        .map(|request| serde_json::from_slice(&request.body).unwrap())
        .collect();
    bodies.sort_by_key(|body| body["state"]["text"].as_str().unwrap().to_owned());
    for (body, state) in bodies.iter().zip([
        json!({"context":"First reference","text":"First caption"}),
        json!({"context":"Second reference","text":"Second caption"}),
    ]) {
        assert_eq!(
            body,
            &json!({"state":state,"model":"d1","questions":{"q1":{"type":"noul","instructions":"Is red visible?"}},"images":[data_url("red.png"),data_url("blue.png"),data_url("red.png")]})
        );
    }
}

#[test]
fn framed_choose_keeps_distinct_ordered_option_lists_with_the_same_images() {
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"d1","answers":{"q1":{"type":"choice","choice":"red","probabilities":{"red":0.8,"blue":0.2}},"q2":{"type":"choice","choice":"blue","probabilities":{"blue":0.8,"red":0.2}}}}"#)
    })
    .unwrap();
    let red = fixture("red.png");
    let input = b"{\"caption\":\"Compare originals.\",\"options\":[\"red\",\"blue\"]}\n{\"caption\":\"Compare originals.\",\"options\":[\"blue\",\"red\"]}\n";
    let output = call(
        &listener,
        &[
            "choose",
            "Which color?",
            "--options",
            "/options",
            "--jsonl",
            "--field",
            "/caption",
            "--image",
            red.to_str().unwrap(),
            "--details",
        ],
        input,
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout).lines().count(), 2);
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    let raw = text(&sent[0].body);
    assert!(
        raw.contains(r#""criteria":{"red":null,"blue":null}"#),
        "{raw}"
    );
    assert!(
        raw.contains(r#""criteria":{"blue":null,"red":null}"#),
        "{raw}"
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&sent[0].body).unwrap()["images"],
        json!([data_url("red.png")])
    );
}

#[test]
fn explicit_attachment_media_validates_every_original_before_sending() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let red = fixture("red.png");
    let jpeg = fixture("red.jpg");
    for arguments in [
        vec![
            "--image-media",
            "image/jpeg",
            "--image",
            red.to_str().unwrap(),
        ],
        vec![
            "--image-media",
            "image/png",
            "--image",
            red.to_str().unwrap(),
            "--image",
            jpeg.to_str().unwrap(),
        ],
        vec![
            "--image-media",
            "image/gif",
            "--image",
            red.to_str().unwrap(),
        ],
        vec!["--image-media", "image/png"],
    ] {
        let output = call(
            &listener,
            &[&["decide", "Is red visible?"], arguments.as_slice()].concat(),
            b"Caption",
        );
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        assert!(output.stdout.is_empty());
        assert!(listener.requests().is_empty());
    }
    let output = call(
        &listener,
        &[
            "decide",
            "Is red visible?",
            "--image-media",
            "image/png",
            "--image",
            red.to_str().unwrap(),
            "--image",
            red.to_str().unwrap(),
            "--details",
        ],
        b"Caption",
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(
        body["images"],
        json!([data_url("red.png"), data_url("red.png")])
    );
}
