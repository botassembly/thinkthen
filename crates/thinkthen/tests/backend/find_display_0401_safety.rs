//! Find display refuses before sending and reuses ordinary recording identities.

use crate::find_display_0401::{PICKED, body, call};
use crate::harness::{Canned, Listener};
use crate::intake_0401::{folder, text};
use crate::support::stored;
use std::{fs, io};

#[test]
fn invalid_flags_empty_input_and_invalid_neighbors_send_nothing() -> io::Result<()> {
    let listener = Listener::answering(|_| Canned::ok(PICKED))?;
    for value in ["-1", "", "+1", "1.0", "١", "18446744073709551616"] {
        let output = call(&listener, &["--around", value], b"first\nsecond\n")?;
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(
            text(&output.stderr).contains("--around takes an ASCII whole number of at least 0")
        );
    }
    for flags in [vec!["-n"], vec!["--scores"], vec!["--around", "0"]] {
        let output = call(
            &listener,
            &[&flags[..], &["--details"]].concat(),
            b"first\nsecond\n",
        )?;
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(text(&output.stderr).contains("text display flags cannot accompany --details"));
        let output = call(&listener, &flags, b"")?;
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stdout.is_empty());
    }
    for (flags, input) in [
        (vec![], b"first\n\n".as_slice()),
        (
            vec!["-n", "--scores", "--around", "1"],
            b"first\n\n".as_slice(),
        ),
        (vec!["--around", "1"], b" \t\r\n".as_slice()),
        (vec!["--jsonl"], b"\"first\"\nnot json\n".as_slice()),
        (
            vec!["--jsonl", "--around", "1"],
            b"\"first\"\nnot json\n".as_slice(),
        ),
    ] {
        let output = call(&listener, &flags, input)?;
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        assert!(output.stdout.is_empty());
        let at = if input.starts_with(b" ") { 1 } else { 2 };
        assert!(
            text(&output.stderr).contains(&format!("record {at}")),
            "{}",
            text(&output.stderr)
        );
    }
    assert_eq!(listener.connections(), 0);
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn snapshot_counts_original_crlf_bytes_and_retains_ordinary_lazy_bound() -> io::Result<()> {
    const LIMIT: usize = 16 * 1024 * 1024;
    let listener = Listener::answering(|_| Canned::ok(PICKED))?;
    let mut source = vec![b'a'; LIMIT / 2 - 2];
    source.extend_from_slice(b"\r\n");
    source.extend(vec![b'b'; LIMIT / 2 - 2]);
    source.extend_from_slice(b"\r\n");
    assert_eq!(source.len(), LIMIT);
    let plain = call(&listener, &["--plan"], &source)?;
    assert_eq!(plain.status.code(), Some(0), "{}", text(&plain.stderr));
    let around = call(
        &listener,
        &["--plan", "--around", "1", "-n", "--scores"],
        &source,
    )?;
    assert_eq!(around.status.code(), Some(0), "{}", text(&around.stderr));
    assert_eq!(around.stdout, plain.stdout);
    source.push(b'x');
    for flags in [
        vec!["--around", "0"],
        vec!["--around", "0", "--plan"],
        vec![],
        vec!["--scores"],
    ] {
        let output = call(&listener, &flags, &source)?;
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        let expected = if flags.contains(&"--around") {
            "--around reads at most 16 MiB across all input sources"
        } else {
            "`find` reads at most 16 MiB across all units"
        };
        assert!(
            text(&output.stderr).contains(expected),
            "{}",
            text(&output.stderr)
        );
    }
    assert_eq!(listener.connections(), 0);
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn views_share_recording_keys_and_replay_without_sending_or_a_key() -> io::Result<()> {
    let listener = Listener::answering(|_| Canned::ok(PICKED))?;
    let place = folder("find-display-replay")?;
    let plain = place.join("plain");
    let shown = place.join("shown");
    let input = b"first\nsecond\n";
    assert_eq!(
        call(
            &listener,
            &["--record", plain.to_str().expect("path")],
            input
        )?
        .stdout,
        b"second\n"
    );
    let ordinary = body(&listener);
    assert_eq!(
        call(
            &listener,
            &[
                "--record",
                shown.to_str().expect("path"),
                "-n",
                "--scores",
                "--around",
                "1"
            ],
            input
        )?
        .stdout,
        b"-- 0.99\n1-first\n2:second\n"
    );
    assert_eq!(body(&listener), ordinary);
    assert_eq!(stored(&plain)?, stored(&shown)?);
    for (flags, expected) in [
        (vec![], b"second\n".as_slice()),
        (vec!["-n"], b"2:second\n".as_slice()),
        (vec!["--scores"], b"0.99 second\n".as_slice()),
        (
            vec!["-n", "--scores", "--around", "1"],
            b"-- 0.99\n1-first\n2:second\n".as_slice(),
        ),
    ] {
        let output = call(
            &listener,
            &[&flags[..], &["--replay", plain.to_str().expect("path")]].concat(),
            input,
        )?;
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        assert_eq!(output.stdout, expected);
    }
    assert!(listener.requests().is_empty());
    assert_eq!(listener.connections(), 2);
    Ok(())
}

#[test]
fn backend_refusal_never_prints_a_delimiter_or_provider_body() -> io::Result<()> {
    let listener = Listener::answering(|_| Canned::status(400, "secret provider response"))?;
    let output = call(
        &listener,
        &["-n", "--scores", "--around", "1"],
        b"first\nsecond\n",
    )?;
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert!(!text(&output.stderr).contains("secret provider response"));
    body(&listener);
    Ok(())
}

#[cfg(unix)]
#[test]
fn invalid_utf8_named_path_preserves_access_and_lossy_details_location() -> io::Result<()> {
    use crate::child::ChildEnvironment as _;
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    let place = folder("find-display-path")?;
    let file = place.join(OsString::from_vec(b"file-\xff".to_vec()));
    fs::write(&file, b"first\nsecond\n")?;
    let listener = Listener::answering(|_| Canned::ok(PICKED))?;
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .clear_environment()
        .home(place.join("home"))
        .args([
            "find",
            "Which unit answers?",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
            "--details",
            "--input",
        ])
        .arg(&file);
    let output = command.output()?;
    assert_eq!(output.status.code(), Some(0));
    let details: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(
        details["position"],
        serde_json::json!({"file":file.to_string_lossy(),"first":2,"last":2})
    );
    body(&listener);
    Ok(())
}

#[test]
fn closed_group_output_keeps_completed_find_outcome() -> io::Result<()> {
    use crate::harness::{finish, start};
    let listener = Listener::answering(|_| Canned::ok(PICKED).after(100))?;
    let mut input = vec![b'a'; 128 * 1024];
    input.push(b'\n');
    input.extend(vec![b'b'; 128 * 1024]);
    input.push(b'\n');
    let mut child = start(
        &[
            "find",
            "Which unit answers?",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--no-cache",
            "--around",
            "1",
            "--scores",
            "-n",
        ],
        &[],
        &input,
    )?;
    drop(child.stdout.take().expect("stdout pipe"));
    let output = finish(child, "find group after pipe closes")?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert!(output.stderr.is_empty());
    body(&listener);
    Ok(())
}
