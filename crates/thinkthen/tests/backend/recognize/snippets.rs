//! Caller snippet widths change marked questions and request windows.
use super::*;

#[test]
fn snippet_width_clips_both_stages_and_keeps_default_requests() {
    let input =
        b"one two three four five six seven Ada eight nine ten eleven twelve thirteen fourteen";
    let mut defaults = None;
    for width in [None, Some("6"), Some("0"), Some("8"), Some("4294967295")] {
        let listener = Listener::answering(automatic).unwrap();
        let mut args = vec!["person"];
        if let Some(width) = width {
            args.extend(["--snippet-pieces", width]);
        }
        let output = run(&listener, &args, input);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let requests = listener.requests();
        assert_eq!(requests.len(), 2);
        let bodies: Vec<_> = requests.iter().map(|r| r.body.clone()).collect();
        if width.is_none() {
            defaults = Some(bodies.clone());
        }
        if width == Some("6") {
            assert_eq!(defaults.as_ref().unwrap(), &bodies);
        }
        let stage: Vec<Value> = bodies
            .iter()
            .map(|b| serde_json::from_slice(b).unwrap())
            .collect();
        let snippets: Vec<_> = stage[0]["questions"]
            .as_object()
            .unwrap()
            .values()
            .map(|q| marked(q["instructions"].as_str().unwrap()))
            .collect();
        let ada = snippets
            .iter()
            .find(|(_, inside, _)| *inside == "Ada")
            .unwrap();
        match width {
            Some("0") => assert_eq!(*ada, ("", "Ada", "")),
            Some("8" | "4294967295") => assert_eq!(
                *ada,
                (
                    "one two three four five six seven ",
                    "Ada",
                    " eight nine ten eleven twelve thirteen fourteen"
                )
            ),
            _ => assert_eq!(
                *ada,
                (
                    "two three four five six seven ",
                    "Ada",
                    " eight nine ten eleven twelve thirteen"
                )
            ),
        }
        let kind = stage[1]["questions"]
            .as_object()
            .unwrap()
            .values()
            .next()
            .unwrap();
        assert_eq!(marked(kind["instructions"].as_str().unwrap()), *ada);
        if width == Some("0") {
            assert_eq!(stage[1]["state"], "Ada");
        } else if matches!(width, Some("8" | "4294967295")) {
            assert_eq!(stage[1]["state"], std::str::from_utf8(input).unwrap());
        }
        assert_eq!(
            snippets
                .iter()
                .find(|(_, inside, _)| *inside == "one")
                .unwrap()
                .0,
            ""
        );
        assert_eq!(
            snippets
                .iter()
                .find(|(_, inside, _)| *inside == "fourteen")
                .unwrap()
                .2,
            ""
        );
    }
}

#[test]
fn malformed_cli_and_saved_snippet_widths_send_nothing() {
    let listener = Listener::answering(automatic).unwrap();
    for width in ["-1", "1.5", "4294967296"] {
        let output = run(&listener, &[&format!("--snippet-pieces={width}")], ADA);
        assert_eq!(output.status.code(), Some(2));
    }
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("snippet-width-invalid");
    fs::create_dir_all(&root).unwrap();
    let file = root.join("question.json");
    for width in ["-1", "1.5", "4294967296", "null"] {
        fs::write(
            &file,
            format!(r#"{{"version":1,"recognize":{{"snippet_pieces":{width}}}}}"#),
        )
        .unwrap();
        let operand = format!("@{}", file.display());
        let output = run(&listener, &[&operand], ADA);
        assert_eq!(output.status.code(), Some(5));
    }
    assert_eq!(listener.requests().len(), 0);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn snippet_width_plan_keys_and_replay_follow_actual_windows() {
    let input =
        b"one two three four five six seven Ada eight nine ten eleven twelve thirteen fourteen";
    let listener = Listener::answering(automatic).unwrap();
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("snippet-width-replay");
    fs::create_dir_all(&root).unwrap();
    let mut keys = Vec::new();
    for width in ["0", "6", "15", "4294967295"] {
        let directory = root.join(width);
        let directory = directory.to_string_lossy();
        let args = ["person", "--snippet-pieces", width];
        let plan = plan_json(&run(
            &listener,
            &[args.as_slice(), &["--plan"]].concat(),
            input,
        ));
        let before = listener.requests().len();
        let output = run(
            &listener,
            &[args.as_slice(), &["--details", "--record", &directory]].concat(),
            input,
        );
        assert!(output.status.success());
        let sent = listener.requests();
        let body = &sent[before].body;
        assert_eq!(
            plan["requests"][0]["body_utf8"],
            String::from_utf8_lossy(body).as_ref()
        );
        let url = format!("{}/systemone", listener.base());
        assert_eq!(
            plan["requests"][0]["digest"],
            crate::support::digest(&url, body)
        );
        let expected: Vec<_> = sent[before..]
            .iter()
            .flat_map(|r| crate::support::keys(&url, &r.body))
            .collect();
        assert_eq!(
            json(&output)["meta"]["requests"],
            Value::from(expected.clone())
        );
        keys.push(expected);
        let before = listener.requests().len();
        let replay = local(
            &listener,
            &[
                args.as_slice(),
                &["--details", "--no-cache", "--replay", &directory],
            ]
            .concat(),
            None,
            input,
        );
        assert!(
            replay.status.success(),
            "{}",
            String::from_utf8_lossy(&replay.stderr)
        );
        assert_eq!(json(&replay)["entities"], json(&output)["entities"]);
        assert_eq!(listener.requests().len(), before);
    }
    assert_ne!(keys[0], keys[1]);
    assert_ne!(keys[1], keys[2]);
    assert_eq!(keys[2], keys[3]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn zero_snippet_width_retains_each_edge_candidate_without_surrounding_pieces() {
    let listener = Listener::answering(automatic).unwrap();
    let output = run(
        &listener,
        &["person", "--snippet-pieces", "0"],
        b"one Ada, two",
    );
    assert!(output.status.success());
    let sent = listener.requests();
    assert_eq!(sent.len(), 2);
    let edge = questions(&sent[1].body)
        .into_iter()
        .find(|q| q["criteria"].get("Ada,").is_some())
        .unwrap();
    assert_eq!(
        marked(edge["instructions"].as_str().unwrap()),
        ("", "Ada", "")
    );
    assert_eq!(edge["criteria"]["Ada"], "...[[Ada]]...");
    assert_eq!(edge["criteria"]["Ada,"], "...[[Ada,]]...");
}

#[test]
fn snippet_cache_reuses_only_the_same_generated_windows() {
    let listener = Listener::answering(automatic).unwrap();
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("snippet-width-cache");
    let cache = root.to_string_lossy();
    let input =
        b"one two three four five six seven Ada eight nine ten eleven twelve thirteen fourteen";
    for (width, expected) in [("0", 2), ("6", 2), ("6", 0), ("15", 2), ("4294967295", 0)] {
        let output = local(
            &listener,
            &["person", "--snippet-pieces", width, "--cache", &cache],
            Some("fake"),
            input,
        );
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(listener.requests().len(), expected);
    }
    fs::remove_dir_all(root).unwrap();
}
