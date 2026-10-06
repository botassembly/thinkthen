//! Ordered image and grouped-question measurement conditions.
use super::images::{PNG, PNG_URL, YES, file, folder, run, succeeds};
use crate::harness::{Canned, Listener, spawn};
use serde_json::{Value, json};
use std::fs;

fn decoded(url: &Value) -> Option<Vec<u8>> {
    let payload = url.as_str()?.split_once(',')?.1;
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();
    for chunk in payload.as_bytes().chunks(4) {
        let [a, b, c, d] = chunk else { return None };
        let value = |byte| alphabet.iter().position(|held| *held == byte).unwrap_or(0) as u8;
        result.push((value(*a) << 2) | (value(*b) >> 4));
        if *c != b'=' {
            result.push((value(*b) << 4) | (value(*c) >> 2));
        }
        if *d != b'=' {
            result.push((value(*c) << 6) | value(*d));
        }
    }
    Some(result)
}

#[test]
fn ordered_pairs_keep_full_bytes_and_replay_only_the_same_condition() {
    let root = folder("ordered-0036");
    let one = file(&root, "one", PNG);
    let two = file(&root, "two", include_bytes!("../fixtures/images/pixel.jpg"));
    let cache = root.join("recording");
    let listener = Listener::serving(vec![Canned::ok(YES), Canned::ok(YES)]).expect("listener");
    let one = one.to_str().expect("path");
    let two = two.to_str().expect("path");
    let cache = cache.to_str().expect("path");
    let output = run(
        &listener,
        "llamacpp",
        "decide",
        &[],
        &["--image", one, "--image", two, "--record", cache],
        true,
    );
    succeeds(&output);
    let requests = listener.requests();
    let body: Value = serde_json::from_slice(&requests[0].body).expect("body");
    assert_eq!(body["images"][0], PNG_URL);
    assert_eq!(decoded(&body["images"][0]), Some(PNG.to_vec()));
    assert_eq!(
        decoded(&body["images"][1]),
        Some(include_bytes!("../fixtures/images/pixel.jpg").to_vec())
    );
    let reversed = run(
        &listener,
        "llamacpp",
        "decide",
        &[],
        &["--image", two, "--image", one, "--replay", cache],
        false,
    );
    assert_eq!(reversed.status.code(), Some(5));
    assert_eq!(listener.connections(), 1);
    for _ in 0..2 {
        let replay = run(
            &listener,
            "llamacpp",
            "decide",
            &[],
            &["--image", one, "--image", two, "--replay", cache],
            false,
        );
        succeeds(&replay);
        assert_eq!(replay.stdout, output.stdout);
    }
    assert_eq!(listener.connections(), 1);
    let reversed = run(
        &listener,
        "llamacpp",
        "decide",
        &[],
        &["--image", two, "--image", one, "--cache", cache],
        true,
    );
    succeeds(&reversed);
    let reversed_body: Value = serde_json::from_slice(&listener.requests()[0].body).expect("body");
    assert_eq!(
        reversed_body["images"],
        json!([body["images"][1], body["images"][0]])
    );
}

#[test]
fn grouped_questions_share_images_and_cannot_reuse_scalar_or_other_cohort() {
    let root = folder("grouped-0036");
    let image = file(&root, "one", PNG);
    let set = file(&root, "questions.json", br#"{"version":1,"questions":{"refund":{"decide":"asks for a refund"},"other":{"decide":"Other?"}}}"#);
    let cache = root.join("recording");
    let both = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}}}"#;
    let listener = Listener::serving(vec![Canned::ok(YES), Canned::ok(both)]).expect("listener");
    let image = image.to_str().expect("path");
    let cache = cache.to_str().expect("path");
    succeeds(&run(
        &listener,
        "llamacpp",
        "decide",
        &[],
        &["--image", image, "--cache", cache],
        true,
    ));
    let args = [
        "annotate",
        set.to_str().expect("path"),
        "--image",
        image,
        "--backend",
        "llamacpp",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--max-retries",
        "0",
    ];
    let output = spawn(
        &[args.as_slice(), &["--record", cache]].concat(),
        &[("THINKTHEN_API_KEY", "sk-image-fixture")],
        b"Refund me please.",
    )
    .expect("command");
    succeeds(&output);
    let row: Value = serde_json::from_slice(&output.stdout).expect("row");
    assert_eq!(row["refund"], true);
    assert_eq!(row["other"], false);
    let requests = listener.requests();
    let body: Value = serde_json::from_slice(&requests[1].body).expect("body");
    assert_eq!(body["questions"].as_object().expect("questions").len(), 2);
    assert_eq!(body["images"], json!([PNG_URL]));
    let exchanges = fs::read_dir(format!("{cache}/exchanges")).expect("exchanges");
    let saved = exchanges
        .map(|entry| fs::read_to_string(entry.expect("entry").path()).expect("saved"))
        .collect::<Vec<_>>();
    assert!(saved.iter().any(|saved| {
        let value: Value = serde_json::from_str(saved).expect("exchange");
        value["request"] == body
            && value["response"] == serde_json::from_str::<Value>(both).expect("reply")
    }));
    assert!(
        saved
            .iter()
            .all(|saved| !saved.contains("sk-image-fixture"))
    );
    succeeds(&spawn(&["cache", "convert", cache], &[], b"").expect("export"));
    let portable = root.join("portable");
    fs::create_dir_all(&portable).expect("portable");
    fs::copy(
        format!("{cache}/thinkthen.jsonl"),
        portable.join("thinkthen.jsonl"),
    )
    .expect("portable fixture");
    let portable = portable.to_str().expect("path");
    for _ in 0..2 {
        let replay = spawn(
            &[args.as_slice(), &["--replay", portable]].concat(),
            &[],
            b"Refund me please.",
        )
        .expect("replay");
        succeeds(&replay);
        assert_eq!(replay.stdout, output.stdout);
    }
    fs::write(
        &set,
        br#"{"version":1,"questions":{"refund":{"decide":"asks for a refund"}}}"#,
    )
    .expect("different cohort");
    let miss = spawn(
        &[args.as_slice(), &["--replay", portable]].concat(),
        &[],
        b"Refund me please.",
    )
    .expect("miss");
    assert_eq!(miss.status.code(), Some(5));
    assert_eq!(listener.connections(), 2);
}

#[test]
fn full_mebibyte_pairs_are_preserved_and_total_body_overflow_sends_zero() {
    let root = folder("bounds-0036");
    // Legal JPEG comments pad the fixture to exactly the admitted bound.
    let original = include_bytes!("../fixtures/images/pixel.jpg");
    let mut bytes = original[..2].to_vec();
    let mut left = 1_048_576 - original.len();
    while left > 0 {
        let mut size = left.min(65_537);
        if left - size > 0 && left - size < 4 {
            size -= 4;
        }
        bytes.extend_from_slice(&[0xff, 0xfe]);
        bytes.extend_from_slice(&u16::try_from(size - 2).expect("segment").to_be_bytes());
        bytes.resize(bytes.len() + size - 4, b'x');
        left -= size;
    }
    bytes.extend_from_slice(&original[2..]);
    assert_eq!(bytes.len(), 1_048_576);
    let image = file(&root, "full.jpg", &bytes);
    let image = image.to_str().expect("path");
    let listener = Listener::serving(vec![Canned::ok(YES)]).expect("listener");
    let output = run(
        &listener,
        "llamacpp",
        "decide",
        &[],
        &["--image", image, "--image", image, "--no-cache"],
        true,
    );
    succeeds(&output);
    let requests = listener.requests();
    assert!(requests[0].body.len() > 2_790_000);
    assert!(requests[0].body.len() <= 2_800_000);
    let body: Value = serde_json::from_slice(&requests[0].body).expect("body");
    assert_eq!(body["images"][0], body["images"][1]);
    assert_eq!(decoded(&body["images"][0]), Some(bytes.clone()));
    // The carrier's base64 is complete, including both the first and last bytes.
    assert_eq!(
        body["images"][0].as_str().expect("URL").len(),
        23 + bytes.len().div_ceil(3) * 4
    );
    let zero = Listener::serving(vec![]).expect("counting listener");
    let oversized = spawn(
        &[
            "decide",
            "Visible?",
            "--backend",
            "llamacpp",
            "--url",
            zero.base(),
            "--image",
            image,
            "--image",
            image,
            "--no-cache",
        ],
        &[],
        &vec![b'x'; 5000],
    )
    .expect("command");
    assert_eq!(oversized.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&oversized.stderr).contains("spike limit of 2800000 bytes"));
    let count = run(
        &zero,
        "llamacpp",
        "decide",
        &[],
        &["--image", image, "--image", image, "--image", image],
        true,
    );
    assert_eq!(count.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&count.stderr).contains("at most two ordered images"));
    assert_eq!(zero.connections(), 0);
}
