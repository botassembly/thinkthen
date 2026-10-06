//! Image inputs cannot borrow an answer cached for an ordinary JSON envelope.
use super::{REPLY, data_url, fixture, text};
use crate::harness::{Canned, Listener, spawn};
use serde_json::{Value, json};
use std::fs;

#[test]
fn images_and_structured_evidence_have_distinct_keys_and_replay_their_own_answers() {
    let no = REPLY.replace("0.9", "0.1");
    let listener = Listener::answering(move |request| {
        let body: Value = serde_json::from_slice(request).unwrap();
        Canned::ok(if body.get("images").is_some() {
            REPLY
        } else {
            &no
        })
    })
    .unwrap();
    let parent = crate::batching::folder("image-structured-key-distinction");
    fs::create_dir_all(&parent).unwrap();
    let question = std::path::Path::new(&parent).join("question.json");
    fs::write(&question, r#"{"decide":{}}"#).unwrap();
    let question = format!("@{}", question.to_str().unwrap());
    let cache = std::path::Path::new(&parent).join("cache");
    let red = fixture("red.png");
    let envelope = json!({"schema":"thinkthen.image-state/1","text":"","images":[{
        "media":"image/png","base64":data_url("red.png").split_once(',').unwrap().1
    }]});
    let bytes = data_url("red.png");
    let bytes = bytes.split_once(',').unwrap().1;
    // Preserve envelope field order, exactly as the image identity serializes.
    let input = format!(r#"{{"evidence":{{"schema":"thinkthen.image-state/1","text":"","images":[{{"media":"image/png","base64":"{bytes}"}}]}}}}"#).into_bytes();
    for mode in ["--cache", "--replay"] {
        for (image, status, answer) in [(true, 0, true), (false, 1, false)] {
            let mut args = vec![
                "decide",
                &question,
                "--backend",
                "liquid",
                "--model",
                "d1",
                "--url",
                listener.base(),
                mode,
                cache.to_str().unwrap(),
                "--details",
            ];
            args.extend(if image {
                vec!["--image", red.to_str().unwrap()]
            } else {
                vec!["--jsonl", "--field", "/evidence"]
            });
            let output = spawn(
                &args,
                &[("THINKTHEN_API_KEY", "sk-image-test")],
                if image { b"" } else { &input },
            )
            .unwrap();
            // JSONL is a record run: its answer is in the row, exit0.
            assert_eq!(
                output.status.code(),
                Some(if image { status } else { 0 }),
                "{}",
                text(&output.stderr)
            );
            let row: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(row["value"], answer);
        }
        assert_eq!(listener.count(), 2, "strict replay sends nothing");
    }
    let requests = listener.requests();
    let image: Value = serde_json::from_slice(&requests[0].body).unwrap();
    let text: Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(image["questions"], text["questions"]);
    assert_eq!(text["state"], envelope);
    assert!(image.get("images").is_some());
    assert!(text.get("images").is_none());
    let connection = rusqlite::Connection::open(cache.join("thinkthen.sqlite")).unwrap();
    let counts: (i64, i64) = connection
        .query_row(
            "SELECT (SELECT COUNT(*) FROM states), (SELECT COUNT(*) FROM answers)",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert_eq!(counts, (2, 2));
    fs::remove_dir_all(parent).unwrap();
}
