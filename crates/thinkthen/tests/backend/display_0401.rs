//! Exact filter/rank text bytes and stable source locations.

use crate::harness::{Canned, Listener, spawn};
use crate::intake_0401::{answer, folder, text};
use serde_json::{Value, json};
use std::{fs, io};

pub(super) fn call(
    listener: &Listener,
    verb: &str,
    flags: &[&str],
    input: &[u8],
) -> io::Result<std::process::Output> {
    spawn(
        &[
            &[
                verb,
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
            ][..],
            flags,
        ]
        .concat(),
        &[],
        input,
    )
}

#[test]
fn every_flag_combination_preserves_lines_and_uses_shortest_scores() -> io::Result<()> {
    let input = b"alpha  \r\n\n\xce\xb2eta";
    let cases: &[(&[&str], &str)] = &[
        (&[], "alpha  \nβeta\n"),
        (&["-n"], "1:alpha  \n3:βeta\n"),
        (&["--scores"], "0.9 alpha  \n0.9 βeta\n"),
        (&["-n", "--scores"], "0.9 1:alpha  \n0.9 3:βeta\n"),
        (&["--around", "0"], "--\nalpha  \n--\nβeta\n"),
        (&["-n", "--around", "0"], "--\n1:alpha  \n--\n3:βeta\n"),
        (
            &["--scores", "--around", "0"],
            "-- 0.9\nalpha  \n-- 0.9\nβeta\n",
        ),
        (
            &["-n", "--scores", "--around", "0"],
            "-- 0.9\n1:alpha  \n-- 0.9\n3:βeta\n",
        ),
    ];
    for verb in ["filter", "rank"] {
        for (flags, expected) in cases {
            let listener = Listener::answering(answer)?;
            let output = call(&listener, verb, flags, input)?;
            assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
            assert_eq!(output.stdout, expected.as_bytes(), "{verb} {flags:?}");
            assert_eq!(listener.requests().len(), 2);
        }
    }
    Ok(())
}

#[test]
fn groups_repeat_overlaps_preserve_blank_neighbors_and_clamp_edges() -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        "filter",
        &["-n", "--around", "1"],
        b"first\r\n\r\nlast\r\n",
    )?;
    assert_eq!(output.stdout, b"--\n1:first\n2-\n--\n2-\n3:last\n");
    let output = call(
        &listener,
        "rank",
        &["-n", "--around", "18446744073709551615", "--top", "2"],
        b"a\nb\nc",
    )?;
    assert_eq!(output.stdout, b"--\n1:a\n2-b\n3-c\n--\n1-a\n2:b\n3-c\n");
    Ok(())
}

#[test]
fn windows_prefix_first_record_line_and_mark_every_selected_physical_line() -> io::Result<()> {
    for verb in ["filter", "rank"] {
        let listener = Listener::answering(answer)?;
        let input = b"\r\n\r\nhit\r\n\r\nend";
        let output = call(&listener, verb, &["--window", "2", "-n", "--scores"], input)?;
        assert_eq!(output.stdout, b"0.9 3:hit\r\n\n0.9 5:end\n");
        let output = call(
            &listener,
            verb,
            &["--window", "2", "-n", "--around", "1"],
            input,
        )?;
        assert_eq!(output.stdout, b"--\n2-\n3:hit\n4:\n5-end\n--\n4-\n5:end\n");
    }
    Ok(())
}

#[test]
fn multiple_and_repeated_paths_keep_occurrence_locations() -> io::Result<()> {
    let place = folder("display-paths")?;
    let one = place.join("one.txt");
    let two = place.join("two.txt");
    fs::write(&one, "one\n")?;
    fs::write(&two, "two\n")?;
    let one = one.to_str().expect("path");
    let two = two.to_str().expect("path");
    for verb in ["filter", "rank"] {
        let listener = Listener::answering(answer)?;
        let output = call(
            &listener,
            verb,
            &["-n", "--around", "1", one, two, one],
            b"",
        )?;
        assert_eq!(
            output.stdout,
            format!("--\n{one}:1:one\n--\n{two}:1:two\n--\n{one}:1:one\n").as_bytes()
        );
        assert_eq!(listener.requests().len(), 3);
        let output = call(&listener, verb, &["-n", one], b"")?;
        assert_eq!(output.stdout, b"1:one\n");
    }
    Ok(())
}

#[test]
fn jsonl_spacing_survives_and_neighbors_never_become_evidence() -> io::Result<()> {
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        "filter",
        &["--jsonl", "--field", "/text", "-n", "--around", "1"],
        b"{ \"text\": \"hit\" }  \r\n\r\nnot json\n",
    )?;
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"--\n1:{ \"text\": \"hit\" }  \n2-\n");
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body: Value = serde_json::from_slice(&requests.first().expect("one observed request").body)
        .expect("observed request JSON");
    assert_eq!(
        body,
        json!({
            "state": "Each question quotes the text it asks about.",
            "model": "local-1",
            "questions": {"q1": {"type": "noul", "instructions": "The text is \"hit\". Is it clear?"}}
        })
    );
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        "rank",
        &["--jsonl", "--field", "/text", "-n", "--scores"],
        b"{ \"text\": \"hit\" }  \n",
    )?;
    assert_eq!(output.stdout, b"0.9 1:{ \"text\": \"hit\" }  \n");
    Ok(())
}

#[test]
fn filter_membership_and_rank_top_use_original_positions() -> io::Result<()> {
    let respond = |body: &[u8]| {
        if text(body).contains("skip") {
            Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.1}}}"#)
        } else {
            answer(body)
        }
    };
    let listener = Listener::answering(respond)?;
    let output = call(
        &listener,
        "filter",
        &["-n", "--scores", "--around", "1"],
        b"skip\nhit\nlast\n",
    )?;
    assert_eq!(
        output.stdout,
        b"-- 0.9\n1-skip\n2:hit\n3-last\n-- 0.9\n2-hit\n3:last\n"
    );
    let output = call(
        &listener,
        "rank",
        &["-n", "--scores", "--top", "1"],
        b"skip\nhit\nlast\n",
    )?;
    assert_eq!(output.stdout, b"0.9 2:hit\n");
    Ok(())
}

#[test]
fn batched_and_split_views_keep_stable_ties_and_file_local_top_positions() -> io::Result<()> {
    let place = folder("display-batches")?;
    let one = place.join("one");
    let two = place.join("two");
    let profile = place.join("profile.json");
    fs::write(&one, "one\n\nlast\n")?;
    fs::write(&two, "two\nend\n")?;
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"split","max_questions":1}"#,
    )?;
    let one = one.to_str().expect("path");
    let two = two.to_str().expect("path");
    for verb in ["filter", "rank"] {
        for split in [false, true] {
            let listener = Listener::answering(answer)?;
            let mut flags = vec![
                verb,
                "Is it clear?",
                "--url",
                listener.base(),
                "--model",
                "local-1",
                "--no-cache",
                "--batch",
                "max",
                "--jobs",
                "2",
                "-n",
                "--scores",
                "--around",
                "0",
                one,
                two,
            ];
            if split {
                flags.extend(["--profile", profile.to_str().expect("path")]);
            }
            if verb == "rank" {
                flags.extend(["--top", "2"]);
            }
            let output = spawn(&flags, &[], b"")?;
            assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
            let mut expected = format!("-- 0.9\n{one}:1:one\n-- 0.9\n{one}:3:last\n");
            if verb == "filter" {
                expected.push_str(&format!("-- 0.9\n{two}:1:two\n-- 0.9\n{two}:2:end\n"));
            }
            assert_eq!(output.stdout, expected.as_bytes());
            assert_eq!(listener.requests().len(), if split { 4 } else { 1 });
        }
    }
    Ok(())
}
