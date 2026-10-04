//! Display refusals, snapshot bounds and unchanged failure guarantees.

use crate::display_0401::call;
use crate::harness::{Canned, Listener};
use crate::intake_0401::{answer, folder, text};
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
    for verb in ["filter", "rank"] {
        for flags in &cases {
            let listener = Listener::answering(answer)?;
            let output = call(&listener, verb, flags, b"secret input\n")?;
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
