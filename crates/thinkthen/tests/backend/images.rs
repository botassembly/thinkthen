//! The branch-only image spike through the compiled command and final bodies.
use crate::harness::{Canned, Listener, spawn};
use crate::support::{encoded_decide, keys, stored};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const PNG: &[u8] = include_bytes!("../fixtures/images/pixel.png");
const PNG_URL: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAADElEQVR4nGP4z8AAAAMBAQDJ/pLvAAAAAElFTkSuQmCC";
const YES: &str = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":10,"output_tokens":2}}"#;
const CHOICE: &str = r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"planet","confidence":0.9,"probabilities":{"planet":0.9,"vehicle":0.05,"building":0.05}}}}"#;
const SCORE: &str = r#"{"model":"local-1","answers":{"q1":{"type":"score","score":1.9,"confidence":0.9,"legend":{"0":"none","1":"part","2":"entire"},"probabilities":{"0":0.0,"1":0.1,"2":0.9}}}}"#;

fn folder(name: &str) -> PathBuf {
    crate::recordings::folder(&format!("image-0034-{name}"))
}
fn file(root: &Path, name: &str, bytes: &[u8]) -> PathBuf {
    fs::create_dir_all(root).expect("owned folder");
    let path = root.join(name);
    fs::write(&path, bytes).expect("fixture");
    path
}
fn run(
    listener: &Listener,
    backend: &str,
    verb: &str,
    labels: &[&str],
    flags: &[&str],
    key: bool,
) -> Output {
    let args = [
        vec![verb, "asks for a refund"],
        labels.to_vec(),
        vec![
            "--backend",
            backend,
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--max-retries",
            "0",
        ],
        flags.to_vec(),
    ]
    .concat();
    let env = if key {
        vec![("THINKTHEN_API_KEY", "sk-image-fixture")]
    } else {
        vec![]
    };
    spawn(&args, &env, b"Refund me please.").expect("compiled binary")
}
fn succeeds(output: &Output) {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn images_reach_all_three_verbs_on_each_explicit_backend_and_plan() {
    let root = folder("verbs");
    let image = file(&root, "not-a-png-extension", PNG);
    let image = image.to_str().expect("path");
    for backend in ["liquid", "openrouter", "llamacpp"] {
        for (verb, labels, answer, expected) in [
            ("decide", vec![], YES, "true\n"),
            (
                "choose",
                vec!["planet", "vehicle", "building"],
                CHOICE,
                "\"planet\"\n",
            ),
            ("score", vec!["none", "part", "entire"], SCORE, "1.9\n"),
        ] {
            let listener = Listener::serving(vec![Canned::ok(answer)]).expect("listener");
            let out = run(
                &listener,
                backend,
                verb,
                &labels,
                &["--image", image, "--no-cache"],
                true,
            );
            succeeds(&out);
            assert_eq!(String::from_utf8_lossy(&out.stdout), expected);
            let requests = listener.requests();
            assert_eq!(requests.len(), 1);
            let body: Value = serde_json::from_slice(&requests[0].body).expect("body");
            assert_eq!(body["images"], json!([PNG_URL]));
            assert_eq!(
                body["state"],
                "Each question quotes the text it asks about."
            );
            assert_eq!(
                body["questions"]["q1"]["instructions"],
                "The text is \"Refund me please.\". asks for a refund"
            );
            assert!(!String::from_utf8_lossy(&requests[0].body).contains("not-a-png-extension"));
            if verb == "score" {
                assert_eq!(
                    body["questions"]["q1"]["criteria"],
                    json!(["none", "part", "entire"])
                );
            }
            if verb == "choose" {
                let sent = String::from_utf8_lossy(&requests[0].body);
                assert!(
                    sent.contains(r#""criteria":{"planet":null,"vehicle":null,"building":null}"#)
                );
            }
            let plan = run(
                &listener,
                backend,
                verb,
                &labels,
                &["--image", image, "--plan"],
                false,
            );
            succeeds(&plan);
            let first = String::from_utf8_lossy(&plan.stdout);
            assert!(first.contains(PNG_URL));
            assert_eq!(listener.connections(), 1, "plan sends nothing");
        }
    }
}

#[test]
fn text_bytes_and_key_stay_fixed_images_miss_and_relocation_replays_without_sends() {
    let root = folder("identity");
    let image = file(&root, "one.png", PNG);
    let relocated = file(&root, "elsewhere.png", PNG);
    let cache = root.join("answers");
    let listener = Listener::serving(vec![Canned::ok(YES), Canned::ok(YES), Canned::ok(YES)])
        .expect("listener");
    let cached = ["--cache", cache.to_str().expect("cache"), "--details"];
    let text = run(&listener, "llamacpp", "decide", &[], &cached, true);
    succeeds(&text);
    let sent = listener.requests();
    assert_eq!(
        sent[0].body,
        encoded_decide("Refund me please.", "local-1", "asks for a refund")
    );
    let text_result: Value = serde_json::from_slice(&text.stdout).expect("details");
    assert_eq!(
        text_result["meta"]["requests"],
        json!(keys(listener.url(), &sent[0].body))
    );
    let attached = [
        cached.to_vec(),
        vec!["--image", image.to_str().expect("path")],
    ]
    .concat();
    let out = run(&listener, "llamacpp", "decide", &[], &attached, true);
    succeeds(&out);
    assert_eq!(listener.connections(), 2, "text cache cannot answer image");
    let first_result: Value = serde_json::from_slice(&out.stdout).expect("details");
    assert_ne!(
        first_result["meta"]["requests"],
        text_result["meta"]["requests"]
    );
    let replay = [
        "--replay",
        cache.to_str().expect("cache"),
        "--image",
        relocated.to_str().expect("path"),
        "--details",
    ];
    let out = run(&listener, "llamacpp", "decide", &[], &replay, false);
    succeeds(&out);
    assert_eq!(
        listener.connections(),
        2,
        "identical bytes moved to another filename"
    );
    let replay_result: Value = serde_json::from_slice(&out.stdout).expect("details");
    assert_eq!(
        first_result["meta"]["requests"],
        replay_result["meta"]["requests"]
    );
    // Change pixels using another valid PNG, not its filename.
    fs::write(&image, include_bytes!("../fixtures/images/other.png")).expect("changed pixels");
    let out = run(&listener, "llamacpp", "decide", &[], &attached, true);
    succeeds(&out);
    assert_eq!(
        listener.connections(),
        3,
        "changed bytes require a live answer"
    );
    let changed: Value = serde_json::from_slice(&out.stdout).expect("details");
    assert_ne!(
        changed["meta"]["requests"],
        first_result["meta"]["requests"]
    );
    assert_eq!(stored(&cache).expect("exported fixture").len(), 3);
    let portable = run(
        &listener,
        "llamacpp",
        "decide",
        &[],
        &[
            "--replay",
            cache.to_str().expect("cache"),
            "--image",
            image.to_str().expect("path"),
        ],
        false,
    );
    succeeds(&portable);
    assert_eq!(listener.connections(), 3);
}

#[test]
fn a_live_record_keeps_actual_image_exchange_and_keyless_replay_for_every_verb() {
    for (verb, labels, response) in [
        ("decide", vec![], YES),
        ("choose", vec!["planet", "vehicle", "building"], CHOICE),
        ("score", vec!["none", "part", "entire"], SCORE),
    ] {
        let root = folder(&format!("record-{verb}"));
        let image = file(&root, "one.png", PNG);
        let recording = root.join("recording");
        let listener = Listener::serving(vec![Canned::ok(response)]).expect("listener");
        let out = run(
            &listener,
            "llamacpp",
            verb,
            &labels,
            &[
                "--image",
                image.to_str().expect("path"),
                "--record",
                recording.to_str().expect("path"),
            ],
            true,
        );
        succeeds(&out);
        let paths: Vec<_> = fs::read_dir(recording.join("exchanges"))
            .expect("exchanges")
            .map(|entry| entry.expect("entry").path())
            .collect();
        assert_eq!(paths.len(), 1);
        let saved = fs::read_to_string(&paths[0]).expect("actual exchange");
        assert!(!saved.contains("sk-image-fixture"));
        #[derive(serde::Deserialize)]
        struct SavedBodies {
            request: Box<serde_json::value::RawValue>,
            response: Box<serde_json::value::RawValue>,
        }
        let saved: SavedBodies = serde_json::from_str(&saved).expect("JSON bodies");
        let requests = listener.requests();
        assert_eq!(saved.request.get().as_bytes(), requests[0].body);
        assert_eq!(saved.response.get(), response);
        let export = spawn(
            &["cache", "convert", recording.to_str().expect("path")],
            &[],
            b"",
        )
        .expect("export");
        succeeds(&export);
        let portable = root.join("portable");
        fs::create_dir_all(&portable).expect("portable folder");
        fs::copy(
            recording.join("thinkthen.jsonl"),
            portable.join("thinkthen.jsonl"),
        )
        .expect("portable fixture");
        let replay = run(
            &listener,
            "llamacpp",
            verb,
            &labels,
            &[
                "--image",
                image.to_str().expect("path"),
                "--replay",
                portable.to_str().expect("path"),
            ],
            false,
        );
        succeeds(&replay);
        assert_eq!(replay.stdout, out.stdout);
        assert_eq!(listener.connections(), 1, "replay never sends");
    }
}

fn malformed_headers() -> Vec<Vec<u8>> {
    let mut malformed = Vec::new();
    // Header fields must be legal even when the container still has image data.
    for (offset, value) in [(24, 3), (24, 1), (25, 1), (26, 1), (27, 1), (28, 2)] {
        let mut bytes = PNG.to_vec();
        bytes[offset] = value;
        malformed.push(bytes);
    }
    let jpeg = include_bytes!("../fixtures/images/pixel.jpg");
    let frame = jpeg
        .windows(2)
        .position(|pair| pair == [0xff, 0xc0])
        .expect("frame");
    let scan = jpeg
        .windows(2)
        .position(|pair| pair == [0xff, 0xda])
        .expect("scan");
    for (offset, value) in [
        (frame + 9, 0),
        (frame + 9, 4),
        (frame + 9, 2),
        (frame + 11, 0),
        (frame + 12, 4),
        (frame + 13, jpeg[frame + 10]),
        (scan + 4, 0),
        (scan + 4, 4),
        (scan + 4, 2),
        (scan + 5, 99),
        (scan + 6, 0x44),
        (scan + 7, jpeg[scan + 5]),
    ] {
        let mut bytes = jpeg.to_vec();
        bytes[offset] = value;
        malformed.push(bytes);
    }
    malformed
}

#[test]
fn invalid_files_and_modes_send_nothing() {
    let root = folder("invalid");
    let image = file(&root, "one.png", PNG);
    let invalid = file(&root, "fake.png", b"not an image");
    let oversized = file(&root, "large.png", &vec![0; 32769]);
    let truncated = file(&root, "truncated.png", &PNG[..33]);
    let listener = Listener::serving(vec![]).expect("counting listener");
    for (path, flags, diagnostic) in [
        (&invalid, vec![], "JPEG or PNG bytes"),
        (&oversized, vec![], "32768 bytes"),
        (&truncated, vec![], "JPEG or PNG bytes"),
        (&root, vec![], "regular file"),
        (&root.join("missing.png"), vec![], "regular file"),
        (&image, vec!["--lines"], "one document"),
        (&image, vec!["--window", "2"], "one document"),
        (
            &image,
            vec!["--input", root.to_str().expect("path")],
            "regular file",
        ),
    ] {
        let flags = [vec!["--image", path.to_str().expect("path")], flags].concat();
        let out = run(&listener, "llamacpp", "decide", &[], &flags, true);
        assert_eq!(out.status.code(), Some(2));
        assert!(
            String::from_utf8_lossy(&out.stderr).contains(diagnostic),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(listener.connections(), 0);
    }
    for (index, bytes) in malformed_headers().iter().enumerate() {
        let path = file(&root, &format!("malformed-{index}"), bytes);
        let out = run(
            &listener,
            "llamacpp",
            "decide",
            &[],
            &["--image", path.to_str().expect("path")],
            true,
        );
        assert_eq!(out.status.code(), Some(2), "malformed header {index}");
        assert!(String::from_utf8_lossy(&out.stderr).contains("JPEG or PNG bytes"));
        assert_eq!(listener.connections(), 0, "malformed header {index}");
    }
    let out = run(
        &listener,
        "ollama",
        "decide",
        &[],
        &["--image", image.to_str().expect("path")],
        true,
    );
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("supported image backend"));
    for verb in [
        "tag",
        "filter",
        "rank",
        "find",
        "recognize",
        "relate",
        "annotate",
    ] {
        let out = run(
            &listener,
            "llamacpp",
            verb,
            &["x", "y"],
            &["--image", image.to_str().expect("path")],
            true,
        );
        assert_eq!(out.status.code(), Some(2), "{verb}");
        assert!(String::from_utf8_lossy(&out.stderr).contains("unexpected argument '--image'"));
    }
    assert_eq!(listener.connections(), 0);
    assert!(
        listener.requests().is_empty(),
        "invalid input reached no request parser"
    );
}

#[test]
fn jpeg_is_sent_as_jpeg_and_large_encoded_image_requests_fail_before_send() {
    let root = folder("jpeg-size");
    let jpeg = file(
        &root,
        "pixel.bin",
        include_bytes!("../fixtures/images/pixel.jpg"),
    );
    let listener = Listener::serving(vec![Canned::ok(YES)]).expect("listener");
    let out = run(
        &listener,
        "llamacpp",
        "decide",
        &[],
        &["--image", jpeg.to_str().expect("path"), "--no-cache"],
        true,
    );
    succeeds(&out);
    let requests = listener.requests();
    let body: Value = serde_json::from_slice(&requests[0].body).expect("body");
    let image = body["images"][0].as_str().expect("data URL");
    assert!(image.starts_with("data:image/jpeg;base64,/9j/"));
    assert!(image.ends_with("E//Z"));
    let oversized = spawn(
        &[
            "decide",
            "Visible?",
            "--backend",
            "llamacpp",
            "--url",
            listener.base(),
            "--image",
            jpeg.to_str().expect("path"),
            "--no-cache",
        ],
        &[],
        &vec![b'x'; 65536],
    )
    .expect("command");
    assert_eq!(oversized.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&oversized.stderr).contains("spike limit of 65536 bytes"));
    assert_eq!(listener.connections(), 1);
    assert!(listener.requests().is_empty());
}
