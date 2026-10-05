//! Source bounds refuse whole sets and keep relation endpoint provenance.
use crate::harness::Listener;
use crate::input_sources::{folder, text};
use crate::source_files::{answer, call};
use serde_json::Value;
use std::{fs, io};

#[test]
fn relation_source_rows_are_bounded_before_deduplication_and_before_reading_the_tail()
-> io::Result<()> {
    let place = folder("relation-source-count")?;
    let path = place.join("names.txt");
    let listener = Listener::answering(answer)?;
    for (ada, bea) in [(255, 1), (10_000, 10_000)] {
        let mut original = format!("{}{}", "Ada\n".repeat(ada), "Bea\n".repeat(bea)).into_bytes();
        original.push(0xff);
        fs::write(&path, original)?;
        let output = call(
            &listener,
            &["relate", "met"],
            &[path.to_str().unwrap()],
            &["--unit", "line"],
        )?;
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(
            text(&output.stderr),
            "thinkthen: source relate takes at most 255 source records\n"
        );
        assert!(output.stdout.is_empty());
        assert!(listener.requests().is_empty());
    }
    Ok(())
}

#[test]
fn oversized_whitespace_is_refused_before_skipping_and_before_the_next_file() -> io::Result<()> {
    let place = folder("source-whitespace-limit")?;
    let path = place.join("first.txt");
    let next = place.join("next.txt");
    fs::write(
        &path,
        format!("{}\nuseful tail\n", " ".repeat(16 * 1024 * 1024 + 1)),
    )?;
    fs::write(&next, "Ada\nBea\n")?;
    let listener = Listener::answering(answer)?;
    for command in [
        vec!["decide", "Q?"],
        vec!["recognize", "person"],
        vec!["relate", "met"],
    ] {
        let output = call(
            &listener,
            &command,
            &[path.to_str().unwrap(), next.to_str().unwrap()],
            &["--unit", "line"],
        )?;
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        assert_eq!(
            text(&output.stderr).lines().next(),
            Some(
                "thinkthen: the record is over 16 MiB, which is far past what a backend reads in one request"
            )
        );
        assert!(output.stdout.is_empty());
        assert!(listener.requests().is_empty());
    }
    Ok(())
}

#[test]
fn recognition_relation_endpoints_map_unicode_crlf_spans_in_every_output_shape() -> io::Result<()> {
    let place = folder("source-recognition-endpoints")?;
    let path = place.join("names.txt");
    fs::write(&path, "café 😀\r\nAda at\r\nAcme\r\n")?;
    let listener = Listener::answering(answer)?;
    for extra in [
        vec!["--unit", "file"],
        vec!["--window", "4"],
        vec!["--unit", "file", "--details"],
    ] {
        let output = call(
            &listener,
            &[
                "recognize",
                "person",
                "organization",
                "--relation",
                "works_for=person:organization",
            ],
            &[path.to_str().unwrap()],
            &extra,
        )?;
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        let row: Value = serde_json::from_slice(&output.stdout).unwrap();
        let edge = &row["value"]["relations"][0];
        for (endpoint, first, start, end) in [("source", 2, 8, 11), ("target", 3, 16, 20)] {
            assert_eq!(edge[endpoint]["file"], path.to_str().unwrap());
            assert_eq!(edge[endpoint]["first_line"], first);
            assert_eq!(edge[endpoint]["last_line"], first);
            assert_eq!(edge[endpoint]["start"], start);
            assert_eq!(edge[endpoint]["end"], end);
        }
    }
    Ok(())
}

#[test]
fn relation_source_original_jsonl_evidence_is_bounded_before_any_request() -> io::Result<()> {
    let place = folder("source-relation-evidence")?;
    let first = place.join("first.txt");
    let second = place.join("second.txt");
    let tail = place.join("tail.txt");
    fs::write(
        &first,
        serde_json::json!({"name":"Ada", "kind":"person", "private":"x".repeat(8 * 1024 * 1024)})
            .to_string(),
    )?;
    fs::write(
        &second,
        serde_json::json!({"name":"Bea", "kind":"person", "private":"y".repeat(8 * 1024 * 1024)})
            .to_string(),
    )?;
    fs::write(&tail, [0xff])?;
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        &["relate", "met"],
        &[
            first.to_str().unwrap(),
            second.to_str().unwrap(),
            tail.to_str().unwrap(),
        ],
        &["--jsonl"],
    )?;
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        text(&output.stderr),
        "thinkthen: source relate input exceeds 16 MiB\n"
    );
    assert!(output.stdout.is_empty());
    assert!(listener.requests().is_empty());
    Ok(())
}

#[test]
fn relation_source_expansion_counts_escaped_evidence_and_withholds_every_output_shape()
-> io::Result<()> {
    let place = folder("source-relation-expansion")?;
    let path = place.join("names.txt");
    // Escaped control bytes grow in serialized endpoint evidence and names.
    let suffix = "\u{0001}".repeat(200);
    fs::write(
        &path,
        format!(
            "{}{}",
            format!("PRIVATE_INPUT_MARKER-Ada{suffix}\n").repeat(65),
            format!("Bea{suffix}\n").repeat(65)
        ),
    )?;
    let listener = Listener::answering(answer)?;
    for extra in [vec!["--unit", "line"], vec!["--unit", "line", "--details"]] {
        let output = call(
            &listener,
            &["relate", "met"],
            &[path.to_str().unwrap()],
            &extra,
        )?;
        assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
        assert_eq!(
            text(&output.stderr),
            "thinkthen: source relate output exceeds 16 MiB\n"
        );
        assert!(output.stdout.is_empty());
        assert!(!listener.requests().is_empty());
    }
    Ok(())
}

#[test]
fn all_255_source_occurrences_keep_every_duplicate_endpoint_pair() -> io::Result<()> {
    let place = folder("source-relation-admitted-boundary")?;
    let path = place.join("names.txt");
    fs::write(&path, format!("{}Bea\n", "Ada\n".repeat(254)))?;
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        &["relate", "met"],
        &[path.to_str().unwrap()],
        &["--unit", "line"],
    )?;
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let edges: Vec<Value> = text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(edges.len(), 508);
    for endpoint in ["source", "target"] {
        let mut ada_lines: Vec<u64> = edges
            .iter()
            .filter(|edge| edge[endpoint]["name"] == "Ada")
            .map(|edge| edge[endpoint]["first_line"].as_u64().unwrap())
            .collect();
        ada_lines.sort_unstable();
        assert_eq!(ada_lines, (1..=254).collect::<Vec<_>>());
        assert!(
            edges
                .iter()
                .filter(|edge| edge[endpoint]["name"] == "Bea")
                .all(|edge| edge[endpoint]["first_line"] == 255)
        );
    }
    Ok(())
}

#[test]
fn oversized_whitespace_keeps_a_completed_stream_prefix_and_sends_no_tail() -> io::Result<()> {
    let place = folder("source-whitespace-prefix")?;
    let first = place.join("first.txt");
    let next = place.join("next.txt");
    fs::write(
        &first,
        format!(
            "good\n{}\nPRIVATE_INPUT_MARKER\n",
            " ".repeat(16 * 1024 * 1024 + 1)
        ),
    )?;
    fs::write(&next, "unread next file")?;
    let listener = Listener::answering(answer)?;
    let output = call(
        &listener,
        &["decide", "Q?"],
        &[first.to_str().unwrap(), next.to_str().unwrap()],
        &["--unit", "line", "--batch", "1"],
    )?;
    assert_eq!(output.status.code(), Some(2));
    let rows: Vec<Value> = text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["input"], "good");
    assert_eq!(rows[0]["first_line"], 1);
    assert!(!text(&output.stderr).contains("PRIVATE_INPUT_MARKER"));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(
        body["questions"]["q1"]["instructions"],
        "The text is \"good\". Q?"
    );
    Ok(())
}
