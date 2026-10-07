//! Framed captions retain originals, independent controls and every attachment.
use super::{REPLY, call, data_url, fixture, text};
use crate::harness::{Canned, Listener};
use serde_json::{Value, json};

#[test]
fn scalar_image_captions_preserve_optional_blank_bytes_and_refuse_invalid_text() {
    let listener = Listener::answering(|_| Canned::ok(REPLY)).unwrap();
    let red = fixture("red.png");
    for caption in [b"".as_slice(), b"   ", b"\t\n", b"Caption\n"] {
        let output = call(
            &listener,
            &[
                "decide",
                "Red?",
                "--image",
                red.to_str().unwrap(),
                "--details",
            ],
            caption,
        );
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        let row: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            row["input"]["text"],
            if caption.is_empty() {
                Value::Null
            } else {
                json!(std::str::from_utf8(caption).unwrap())
            }
        );
        let body: Value =
            serde_json::from_slice(&listener.requests().last().unwrap().body).unwrap();
        assert_eq!(body["state"], json!(std::str::from_utf8(caption).unwrap()));
    }
    let before = listener.count();
    for (caption, exit) in [(vec![0xff], 5), (vec![b'x'; 16 * 1024 * 1024 + 1], 2)] {
        let output = call(
            &listener,
            &[
                "decide",
                "Red?",
                "--image",
                red.to_str().unwrap(),
                "--details",
            ],
            &caption,
        );
        assert_eq!(output.status.code(), Some(exit), "{}", text(&output.stderr));
        assert!(output.stdout.is_empty());
        assert_eq!(listener.count(), before);
    }
    let place = crate::batching::folder("image-caption-declaration");
    std::fs::create_dir_all(&place).unwrap();
    let path = format!("{place}/question.json");
    let operand = format!("@{path}");
    for (schema, caption, exit) in [
        (json!({"type":"string"}), b"   ".as_slice(), 0),
        (
            json!({"type":"object","properties":{"body":{"type":"string"}}}),
            b"   ".as_slice(),
            2,
        ),
        (json!({"type":"string"}), b"".as_slice(), 2),
    ] {
        std::fs::write(
            &path,
            json!({"decide":"Red?","item_schema":schema}).to_string(),
        )
        .unwrap();
        let before = listener.count();
        let output = call(
            &listener,
            &[
                "decide",
                &operand,
                "--image",
                red.to_str().unwrap(),
                "--details",
            ],
            caption,
        );
        assert_eq!(output.status.code(), Some(exit), "{}", text(&output.stderr));
        assert_eq!(listener.count() - before, usize::from(exit == 0));
        if exit != 0 {
            assert!(output.stdout.is_empty());
        }
    }
    std::fs::remove_dir_all(&place).unwrap();
}

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
