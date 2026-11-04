#![cfg(test)]
//! A saved rank step streams its input instead of holding all of it first.
#![cfg(feature = "command")]
#![cfg(unix)]

#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::json;
use std::io::Write;
use std::process::Stdio;
use std::sync::mpsc;
use std::time::Duration;
mod support;

/// Records sent before the producer waits for the first backend request.
const FIRST: usize = 100;
/// Records sent after it, so the whole input is far larger than one batch.
const REST: usize = 2_000;

#[test]
fn rank_scores_records_before_a_long_input_ends() {
    let (asked, waited) = mpsc::channel::<()>();
    let server = Loopback::answering(move |question| {
        let _ = asked.send(());
        let p = if question["instructions"]
            .as_str()
            .unwrap()
            .contains("record 7 ")
        {
            0.9
        } else {
            0.1
        };
        json!({"type":"score","probabilities":{"0":1.0 - p,"1":p}})
    })
    .unwrap();
    let directory = tempfile::TempDir::new().unwrap();
    let entry = directory.path().join("rank.json");
    std::fs::write(&entry, br#"{"run":[{"question":{"function":"rank","question":"Priority?","levels":["low","high"],"top":1}},{"shell":["cat"]}]}"#).unwrap();
    let mut child = support::command()
        .args([
            "run",
            "--allow-shell",
            "--json",
            "--backend",
            "systemone",
            "--batch",
            "50",
            "--url",
            server.base(),
        ])
        .arg(&entry)
        .env("THINKTHEN_API_KEY", KEY)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    // The producer stops after the first records until a backend request arrives.
    // A host that holds every record before ranking never asks, so it waits out the deadline.
    let producer = std::thread::spawn(move || {
        for index in 0..FIRST {
            writeln!(input, "{}", json!(format!("record {index} body"))).unwrap();
        }
        let streamed = waited.recv_timeout(Duration::from_secs(20)).is_ok();
        for index in FIRST..FIRST + REST {
            writeln!(input, "{}", json!(format!("record {index} body"))).unwrap();
        }
        streamed
    });
    let output = child.wait_with_output().unwrap();
    assert!(
        producer.join().unwrap(),
        "no backend request arrived before the input ended"
    );
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let text = String::from_utf8(output.stdout).unwrap();
    let rows: Vec<_> = text.lines().collect();
    let answered: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| serde_json::from_str::<serde_json::Value>(row).unwrap())
        .filter(|row| row["answer"] == json!(1))
        .collect();
    assert_eq!(answered.len(), 1, "{rows:?}");
    assert_eq!(answered[0]["position"], json!(8), "{answered:?}");
    // The ranked row carries its original item, kept by the engine with its top state.
    assert_eq!(answered[0]["item"], json!("record 7 body"), "{answered:?}");
}
