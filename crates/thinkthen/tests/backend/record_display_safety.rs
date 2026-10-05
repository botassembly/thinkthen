//! Display refusals, snapshot bounds and unchanged failure guarantees.

use crate::find_display::{PICKED, body, call as find_call};
use crate::harness::{Canned, Listener};
use crate::input_sources::{answer, folder, text};
use crate::record_display::call;
use crate::support::stored;
use std::{fs, io};

#[test]
fn display_usage_errors_and_late_open_errors_send_nothing() -> io::Result<()> {
    let cases: Vec<Vec<&str>> = vec![
        vec!["--around", "-1"],
        vec!["--around", ""],
        vec!["--around", "+1"],
        vec!["--around", "1.0"],
        vec!["--around", "١"],
        vec!["--around", "18446744073709551616"],
        vec!["-n", "--details"],
        vec!["--scores", "--details"],
        vec!["--around", "0", "--details"],
        vec!["-n", "--csv"],
        vec!["--scores", "--csv"],
        vec!["--around", "0", "--csv"],
        vec!["-n", "--tsv"],
        vec!["--scores", "--tsv"],
        vec!["--around", "0", "--tsv"],
    ];
    for verb in ["filter", "rank", "find"] {
        for flags in &cases {
            if verb == "find" && (flags.contains(&"--csv") || flags.contains(&"--tsv")) {
                continue;
            }
            let listener = Listener::answering(answer)?;
            let output = if verb == "find" {
                find_call(&listener, flags, b"secret input\n")?
            } else {
                call(&listener, verb, flags, b"secret input\n")?
            };
            assert_eq!(
                output.status.code(),
                Some(2),
                "{flags:?}: {}",
                text(&output.stderr)
            );
            assert!(output.stdout.is_empty());
            assert_eq!(listener.connections(), 0);
            assert!(!text(&output.stderr).contains("secret input"));
            let expected = if flags.contains(&"--details") {
                "text display flags cannot accompany --details"
            } else if flags.contains(&"--csv") || flags.contains(&"--tsv") {
                "text display flags need lines or JSONL, not CSV or TSV"
            } else {
                "--around takes an ASCII whole number of at least 0"
            };
            assert!(text(&output.stderr).contains(expected));
        }
        if verb == "find" {
            continue;
        }
        let place = folder("display-missing")?;
        let one = place.join("one");
        let missing = place.join("missing-secret-path");
        fs::write(&one, "good\n")?;
        let listener = Listener::answering(answer)?;
        let output = call(
            &listener,
            verb,
            &[
                "--around",
                "0",
                one.to_str().expect("path"),
                missing.to_str().expect("path"),
            ],
            b"",
        )?;
        assert_eq!(output.status.code(), Some(5));
        assert!(output.stdout.is_empty());
        assert_eq!(listener.connections(), 0);
        assert!(!text(&output.stderr).contains("missing-secret-path"));
    }
    Ok(())
}

#[test]
fn original_byte_budget_counts_blanks_endings_and_all_source_occurrences() -> io::Result<()> {
    const LIMIT: usize = 16 * 1024 * 1024;
    let place = folder("display-bound")?;
    let one = place.join("one");
    let two = place.join("two");
    let mut half = vec![b' '; LIMIT / 2 - 2];
    half.extend_from_slice(b"\r\n");
    fs::write(&one, &half)?;
    fs::write(&two, &half)?;
    let one = one.to_str().expect("path");
    let two = two.to_str().expect("path");
    let mut whole = half.clone();
    whole.extend_from_slice(&half);
    let full = place.join("full");
    fs::write(&full, &whole)?;
    let full = full.to_str().expect("path");
    for verb in ["filter", "rank"] {
        for flags in [
            vec!["--around", "0"],
            vec!["--around", "0", one, two],
            vec!["--around", "0", one, one],
            vec!["--around", "0", full],
        ] {
            let listener = Listener::answering(answer)?;
            let output = call(&listener, verb, &flags, &whole)?;
            assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
            assert!(output.stdout.is_empty());
            assert_eq!(listener.connections(), 0);
        }
        check_overflow_and_lazy_intake(verb, full, &mut whole)?;
        let listener = Listener::answering(answer)?;
        fs::write(two, [&half[..], b"\n"].concat())?;
        let output = call(&listener, verb, &["--around", "0", one, two], b"")?;
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(listener.connections(), 0);
        fs::write(two, &half)?;
        let output = call(&listener, verb, &["--around", "0"], b"")?;
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stdout.is_empty());
    }
    Ok(())
}

fn check_overflow_and_lazy_intake(verb: &str, full: &str, whole: &mut Vec<u8>) -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    whole.push(b'\n');
    let output = call(&listener, verb, &["--around", "0"], &*whole)?;
    fs::write(full, &*whole)?;
    let named = call(&listener, verb, &["--around", "0", full], b"")?;
    assert_eq!(named.status.code(), Some(2));
    let plan = call(&listener, verb, &["--around", "0", "--plan", full], b"")?;
    assert_eq!(plan.status.code(), Some(2));
    assert!(plan.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
    whole.pop();
    fs::write(full, &*whole)?;
    let mut beyond = whole.clone();
    beyond.extend_from_slice(b"hit\n");
    let lazy_listener = Listener::answering(answer)?;
    let lazy = call(&lazy_listener, verb, &["-n", "--scores"], &beyond)?;
    assert_eq!(lazy.status.code(), Some(0), "{}", text(&lazy.stderr));
    assert_eq!(lazy.stdout, b"0.9 3:hit\n");
    assert_eq!(lazy_listener.requests().len(), 1);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
    assert!(
        text(&output.stderr).contains("--around reads at most 16 MiB across all input sources")
    );
    Ok(())
}

#[test]
fn later_backend_failure_keeps_filter_groups_and_leaves_rank_empty() -> io::Result<()> {
    let views: &[(&[&str], &[u8])] = &[
        (&["-n"], b"1:first\n"),
        (&["--scores"], b"0.9 first\n"),
        (&["--around", "1"], b"--\nfirst\nfail\n"),
        (
            &["-n", "--scores", "--around", "1"],
            b"-- 0.9\n1:first\n2-fail\n",
        ),
    ];
    let respond = |body: &[u8]| {
        if text(body).contains("fail") {
            Canned::status(400, "secret provider body")
        } else {
            answer(body)
        }
    };
    for verb in ["filter", "rank"] {
        for (flags, prefix) in views {
            let listener = Listener::answering(respond)?;
            let output = call(&listener, verb, flags, b"first\nfail\n")?;
            assert_eq!(output.status.code(), Some(4), "{}", text(&output.stderr));
            assert_eq!(output.stdout, if verb == "filter" { *prefix } else { b"" });
            assert_eq!(listener.requests().len(), 2);
            assert!(!text(&output.stderr).contains("secret provider body"));
            if verb == "rank" {
                let output = call(
                    &listener,
                    verb,
                    &[&flags[..], &["--top", "1"]].concat(),
                    b"first\nfail\n",
                )?;
                assert_eq!(output.status.code(), Some(4));
                assert!(output.stdout.is_empty());
            }
        }
    }
    Ok(())
}

#[test]
fn invalid_flags_empty_input_and_invalid_neighbors_send_nothing() -> io::Result<()> {
    let listener = Listener::answering(|_| Canned::ok(PICKED))?;
    for flags in [vec!["-n"], vec!["--scores"], vec!["--around", "0"]] {
        let output = find_call(&listener, &flags, b"")?;
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
        let output = find_call(&listener, &flags, input)?;
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
    let plain = find_call(&listener, &["--plan"], &source)?;
    assert_eq!(plain.status.code(), Some(0), "{}", text(&plain.stderr));
    let around = find_call(
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
        let output = find_call(&listener, &flags, &source)?;
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
        find_call(
            &listener,
            &["--record", plain.to_str().expect("path")],
            input
        )?
        .stdout,
        b"second\n"
    );
    let ordinary = body(&listener);
    assert_eq!(
        find_call(
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
        let output = find_call(
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
    let output = find_call(
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
