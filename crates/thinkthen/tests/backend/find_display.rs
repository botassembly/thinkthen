//! Find views preserve the aggregate request and physical candidate identity.

use crate::harness::{Canned, Listener, spawn};
use crate::input_sources::{folder, text};
use serde_json::{Value, json};
use std::{fs, io};

pub(super) const PICKED: &str = r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"u002","confidence":0.98,"probabilities":{"u001":0.01,"u002":0.99}}}}"#;

pub(super) fn call(
    listener: &Listener,
    flags: &[&str],
    input: &[u8],
) -> io::Result<std::process::Output> {
    spawn(
        &[
            &[
                "find",
                "Which unit answers?",
                "--url",
                listener.base(),
                "--model",
                "local-1",
                "--no-cache",
            ][..],
            flags,
        ]
        .concat(),
        &[],
        input,
    )
}

pub(super) fn body(listener: &Listener) -> Vec<u8> {
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    requests
        .into_iter()
        .flat_map(|request| request.body)
        .collect()
}

#[test]
fn every_view_keeps_original_bytes_score_location_and_exact_aggregate_request() -> io::Result<()> {
    let input = "first  \r\nβeta  ";
    let cases: &[(&[&str], &str)] = &[
        (&[], "βeta  \n"),
        (&["-n"], "2:βeta  \n"),
        (&["--scores"], "0.99 βeta  \n"),
        (&["--scores", "-n"], "0.99 2:βeta  \n"),
        (&["--around", "0"], "--\nβeta  \n"),
        (&["-n", "--around", "0"], "--\n2:βeta  \n"),
        (&["--scores", "--around", "0"], "-- 0.99\nβeta  \n"),
        (&["-n", "--scores", "--around", "0"], "-- 0.99\n2:βeta  \n"),
        (
            &["-n", "--scores", "--around", "1"],
            "-- 0.99\n1-first  \n2:βeta  \n",
        ),
    ];
    let listener = Listener::answering(|_| Canned::ok(PICKED))?;
    assert_eq!(
        call(&listener, &[], input.as_bytes())?.stdout,
        "βeta  \n".as_bytes()
    );
    let ordinary = body(&listener);
    let place = folder("find-display-views")?;
    let file = place.join("input");
    fs::write(&file, input)?;
    for named in [false, true] {
        for (flags, expected) in cases {
            let flags = if named {
                [*flags, &["--input", file.to_str().expect("path")]].concat()
            } else {
                flags.to_vec()
            };
            let output = call(&listener, &flags, input.as_bytes())?;
            assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
            assert_eq!(output.stdout, expected.as_bytes());
            assert_eq!(body(&listener), ordinary);
        }
    }
    let maximum = usize::MAX.to_string();
    assert_eq!(
        call(&listener, &["-n", "--around", &maximum], input.as_bytes())?.stdout,
        "--\n1-first  \n2:βeta  \n".as_bytes()
    );
    assert_eq!(body(&listener), ordinary);
    Ok(())
}

#[test]
fn duplicate_winner_and_ties_use_candidate_indices_and_details_add_only_position() -> io::Result<()>
{
    let cases = [
        (PICKED, false, 0, b"0.99 2:same\n".as_slice()),
        (
            r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"u002","probabilities":{"u001":0.5,"u002":0.5}}}}"#,
            false,
            0,
            b"0.5 1:same\n".as_slice(),
        ),
        (
            r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"none","probabilities":{"u001":0.1,"u002":0.1,"none":0.8}}}}"#,
            true,
            3,
            b"".as_slice(),
        ),
        (
            r#"{"model":"local-1","answers":{"q1":{"type":"choice","choice":"u001","probabilities":{"u001":0.5,"u002":0.0,"none":0.5}}}}"#,
            true,
            3,
            b"".as_slice(),
        ),
    ];
    for (reply, none, status, expected) in cases {
        let listener = Listener::answering(move |_| Canned::ok(reply))?;
        let extra = if none { vec!["--none"] } else { vec![] };
        let output = call(
            &listener,
            &[&extra[..], &["-n", "--scores"]].concat(),
            b"same\nsame\n",
        )?;
        assert_eq!(output.status.code(), Some(status));
        assert_eq!(output.stdout, expected);
        body(&listener);
        if none {
            assert!(
                call(
                    &listener,
                    &["--none", "--around", "1", "--scores"],
                    b"same\nsame\n"
                )?
                .stdout
                .is_empty()
            );
            body(&listener);
        }
        let output = call(
            &listener,
            &[&extra[..], &["--details"]].concat(),
            b"same\nsame\n",
        )?;
        assert_eq!(output.status.code(), Some(status));
        let details: Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(details["threshold"], Value::Null);
        if none {
            assert_eq!(details["value"], Value::Null);
            assert!(details.get("position").is_none());
        } else {
            let first = if reply == PICKED { 2 } else { 1 };
            assert_eq!(
                details["position"],
                json!({"file":null,"first":first,"last":first})
            );
            assert_eq!(details["value"], "same");
            assert_eq!(
                details["answer"]["probabilities"]["u002"],
                if reply == PICKED {
                    json!(0.99)
                } else {
                    json!(0.5)
                }
            );
        }
        body(&listener);
    }
    Ok(())
}

#[test]
fn pointed_jsonl_keeps_complete_rows_for_view_and_only_pointed_evidence_in_request()
-> io::Result<()> {
    let input = b"{ \"body\": \"first\", \"secret\": \"source-only-A\" }  \r\n{\"body\":\"second\",\"secret\":\"source-only-B\"}\n";
    let listener = Listener::answering(|_| Canned::ok(PICKED))?;
    let flags = ["--jsonl", "--field", "/body"];
    assert_eq!(call(&listener, &flags, input)?.status.code(), Some(0));
    let ordinary = body(&listener);
    let output = call(
        &listener,
        &[&flags[..], &["-n", "--around", "1"]].concat(),
        input,
    )?;
    assert_eq!(output.stdout, b"--\n1-{ \"body\": \"first\", \"secret\": \"source-only-A\" }  \n2:{\"body\":\"second\",\"secret\":\"source-only-B\"}\n");
    let request = body(&listener);
    assert_eq!(request, ordinary);
    let request: Value = serde_json::from_slice(&request)?;
    assert_eq!(
        request["state"],
        r#"[{"id":"u001","evidence":"first"},{"id":"u002","evidence":"second"}]"#
    );
    assert_eq!(
        request["questions"]["q1"]["criteria"],
        json!({"u001":null,"u002":null})
    );
    assert!(!text(&ordinary).contains("source-only"));
    let output = call(
        &listener,
        &["--jsonl", "--details"],
        b"\"first\"\n\"second\"\n",
    )?;
    let details: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(details["position"], json!({"file":null,"first":2,"last":2}));
    body(&listener);
    Ok(())
}

#[test]
fn named_file_mutation_after_capture_preserves_selected_bytes_context_and_position()
-> io::Result<()> {
    for view in [
        vec![],
        vec!["-n", "--scores", "--around", "1"],
        vec!["--details"],
    ] {
        let place = folder("find-display-immutable")?;
        let file = place.join("input");
        fs::write(&file, "first\nsecond\n")?;
        let changed = file.clone();
        let listener = Listener::answering(move |_| {
            fs::write(&changed, "wrong\nchanged\n").expect("fixture mutation");
            Canned::ok(PICKED)
        })?;
        let output = call(
            &listener,
            &[&view[..], &["--input", file.to_str().expect("path")]].concat(),
            b"",
        )?;
        assert_eq!(output.status.code(), Some(0));
        if view.contains(&"--details") {
            let details: Value = serde_json::from_slice(&output.stdout)?;
            assert_eq!(details["value"], "second");
            assert_eq!(
                details["position"],
                json!({"file":file.to_string_lossy(),"first":2,"last":2})
            );
        } else {
            assert_eq!(
                output.stdout,
                if view.is_empty() {
                    b"second\n".as_slice()
                } else {
                    b"-- 0.99\n1-first\n2:second\n".as_slice()
                }
            );
        }
        let request: Value = serde_json::from_slice(&body(&listener))?;
        assert_eq!(
            request["state"],
            r#"[{"id":"u001","evidence":"first"},{"id":"u002","evidence":"second"}]"#
        );
        assert_eq!(fs::read_dir(place)?.count(), 1);
    }
    Ok(())
}
