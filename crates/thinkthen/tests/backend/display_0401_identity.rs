//! Immutable source bytes, wire/cache/replay parity and saved score labels.

use crate::display_0401::call;
use crate::harness::{Canned, Listener, spawn};
use crate::intake_0401::{answer, folder, text};
use crate::support::stored;
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::{fs, io};

#[test]
fn file_mutation_changes_neither_later_requests_nor_neighbors_and_never_spools() -> io::Result<()> {
    let place = folder("display-immutable")?;
    let file = place.join("input");
    let scratch = place.join("scratch");
    fs::create_dir_all(&scratch)?;
    fs::write(&file, "first\nsecond\nlast\n")?;
    let changed = file.clone();
    let count = Arc::new(AtomicUsize::new(0));
    let seen = count.clone();
    let listener = Listener::answering(move |body| {
        if seen.fetch_add(1, Ordering::SeqCst) == 0 {
            fs::write(&changed, "changed\nwrong\nnew\n").expect("mutation fixture");
        }
        answer(body)
    })?;
    let output = spawn(
        &[
            "filter",
            "Is it clear?",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
            "--batch",
            "1",
            "--jobs",
            "1",
            "-n",
            "--around",
            "1",
            file.to_str().expect("path"),
        ],
        &[("TMPDIR", scratch.to_str().expect("scratch"))],
        b"",
    )?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(
        output.stdout,
        b"--\n1:first\n2-second\n--\n1-first\n2:second\n3-last\n--\n2-second\n3:last\n"
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    for (request, expected) in requests.iter().zip(["first", "second", "last"]) {
        assert!(text(&request.body).contains(expected));
        assert!(!text(&request.body).contains("changed"));
        assert!(!text(&request.body).contains("wrong"));
    }
    assert_eq!(fs::read_dir(scratch)?.count(), 0);
    Ok(())
}

#[test]
fn display_changes_neither_request_bytes_nor_stored_keys_and_replay_sends_nothing() -> io::Result<()>
{
    for verb in ["filter", "rank"] {
        let listener = Listener::answering(answer)?;
        let plain = crate::recordings::folder(&format!("display-wire-{verb}-plain"));
        let shown = crate::recordings::folder(&format!("display-wire-{verb}-shown"));
        let fixed = [
            verb,
            "Is it clear?",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
            "--batch",
            "2",
            "--jobs",
            "1",
        ];
        let input = b"first\n\nlast\r\n";
        let output = spawn(
            &[&fixed[..], &["--record", plain.to_str().expect("path")]].concat(),
            &[],
            input,
        )?;
        assert_eq!(output.status.code(), Some(0));
        let before = listener
            .requests()
            .into_iter()
            .map(|request| request.body)
            .collect::<Vec<_>>();
        let view = ["-n", "--scores", "--around", "1"];
        let output = spawn(
            &[
                &fixed[..],
                &view,
                &["--record", shown.to_str().expect("path")],
            ]
            .concat(),
            &[],
            input,
        )?;
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        let requests = listener.requests();
        assert_eq!(requests.len(), before.len());
        assert_eq!(
            requests
                .iter()
                .map(|request| &request.body)
                .collect::<Vec<_>>(),
            before.iter().collect::<Vec<_>>()
        );
        let keys = |path| -> io::Result<Vec<String>> {
            Ok(stored(path)?
                .iter()
                .map(|row| row["key"].to_string())
                .collect())
        };
        assert_eq!(keys(&plain)?, keys(&shown)?);
        let count = listener.connections();
        let replay = spawn(
            &[
                &fixed[..],
                &view,
                &["--replay", plain.to_str().expect("path")],
            ]
            .concat(),
            &[],
            input,
        )?;
        assert_eq!(replay.status.code(), Some(0), "{}", text(&replay.stderr));
        assert_eq!(replay.stdout, output.stdout);
        assert_eq!(listener.connections(), count);
    }
    Ok(())
}

#[test]
fn saved_score_labels_use_weighted_levels_and_cache_hits_without_sends() -> io::Result<()> {
    let place = folder("display-saved-score")?;
    let question = place.join("question.json");
    fs::write(
        &question,
        r#"{"score":"Quality?","levels":{"low":"Low","middle":"Middle","high":"High"}}"#,
    )?;
    let question = format!("@{}", question.display());
    let cache = crate::recordings::folder("display-score-cache");
    let listener = Listener::answering(|_| {
        Canned::ok(&json!({"model":"local-1","answers":{"q1":{"type":"score","score":0.0,"confidence":0.1,"legend":{"0":"low","1":"middle","2":"high"},"probabilities":{"0":0.1,"1":0.3,"2":0.6}}}}).to_string())
    })?;
    let fixed = [
        "rank",
        &question,
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--batch",
        "1",
        "--jobs",
        "1",
        "--cache",
        cache.to_str().expect("path"),
    ];
    let plain = spawn(&fixed, &[], b"a\nb\n")?;
    assert_eq!(plain.stdout, b"a\nb\n");
    let count = listener.connections();
    let shown = spawn(
        &[
            &fixed[..],
            &["-n", "--scores", "--around", "0", "--top", "1"],
        ]
        .concat(),
        &[],
        b"a\nb\n",
    )?;
    assert_eq!(shown.status.code(), Some(0), "{}", text(&shown.stderr));
    assert_eq!(shown.stdout, b"-- 1.5\n1:a\n");
    assert_eq!(listener.connections(), count);
    Ok(())
}

#[test]
fn provider_context_remains_independent_from_display_neighbors() -> io::Result<()> {
    let place = folder("display-context")?;
    let context = place.join("context");
    fs::write(&context, "Provider-only context")?;
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        "filter",
        &[
            "--context",
            context.to_str().expect("path"),
            "--around",
            "1",
        ],
        b"hit\n\n",
    )?;
    assert_eq!(output.stdout, b"--\nhit\n\n");
    assert_eq!(listener.requests().len(), 1);
    assert!(
        listener
            .requests()
            .iter()
            .all(|request| text(&request.body).contains("Provider-only context"))
    );
    Ok(())
}

#[cfg(unix)]
#[test]
fn invalid_utf8_paths_keep_access_and_lossy_display_and_neighbor_bytes() -> io::Result<()> {
    use crate::child::ChildEnvironment as _;
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    let place = folder("display-invalid-path")?;
    let file = place.join(OsString::from_vec(b"file-\xff".to_vec()));
    fs::write(&file, b"hit\n\xff\n")?;
    let listener = Listener::answering(answer)?;
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command.clear_environment().home(place.join("home")).args(&[
        "filter",
        "Is it clear?",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--no-cache",
        "--batch",
        "1",
        "--jobs",
        "1",
        "-n",
        "--around",
        "1",
    ]);
    command.arg(&file).arg(&file);
    let output = command.output()?;
    assert_eq!(output.status.code(), Some(5));
    let mut expected = format!(
        "--\n{}:1:hit\n{}:2-",
        file.to_string_lossy(),
        file.to_string_lossy()
    )
    .into_bytes();
    expected.extend_from_slice(b"\xff\n");
    assert_eq!(output.stdout, expected);
    assert_eq!(listener.requests().len(), 1);
    Ok(())
}

#[test]
fn saved_score_backend_failure_keeps_rank_empty_under_every_text_view() -> io::Result<()> {
    let place = folder("display-score-failure")?;
    let question = place.join("question.json");
    fs::write(
        &question,
        r#"{"score":"Quality?","levels":{"low":"Low","high":"High"}}"#,
    )?;
    let question = format!("@{}", question.display());
    for view in [
        vec!["-n"],
        vec!["--scores"],
        vec!["--around", "1"],
        vec!["-n", "--scores", "--around", "0", "--top", "1"],
    ] {
        let listener = Listener::answering(|body| {
            if text(body).contains("fail") {
                Canned::status(400, "secret response")
            } else {
                answer(body)
            }
        })?;
        let output = spawn(
            &[
                &[
                    "rank",
                    &question,
                    "--url",
                    listener.base(),
                    "--model",
                    "local-1",
                    "--no-cache",
                    "--batch",
                    "1",
                    "--jobs",
                    "1",
                ][..],
                &view,
            ]
            .concat(),
            &[],
            b"first\nfail\n",
        )?;
        assert_eq!(output.status.code(), Some(4), "{}", text(&output.stderr));
        assert!(output.stdout.is_empty());
        assert_eq!(listener.connections(), 2);
        assert!(!text(&output.stderr).contains("secret response"));
    }
    Ok(())
}
