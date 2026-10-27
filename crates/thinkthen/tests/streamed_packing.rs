#![cfg(test)]
//! Feature: streamed records share the finite input request packing.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
#[cfg(feature = "command")]
mod support;
use loopback::{KEY, Loopback};
use thinkthen::{Decide, Engine, Key, Record, Settings};

fn engine(server: &Loopback) -> Engine {
    Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch(50_u64.try_into().unwrap())
        .engine(|_| None)
        .unwrap()
}

#[test]
fn pushed_records_pack_like_finite_input_and_keep_their_order() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = engine(&server);
    let question = Decide::new("Ready?").unwrap();
    let finite = engine
        .rows(
            &question,
            (0..500).map(|i| Record::text(format!("row {i}"))),
        )
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let finite_requests = server.count();
    assert_eq!(finite_requests, 10);
    let call = engine.open(&question).unwrap();
    let sender = call.sender();
    let producer = std::thread::spawn(move || {
        for i in 0..500 {
            sender.send(Record::text(format!("row {i}"))).unwrap();
        }
        sender.close();
    });
    let streamed = call.rows().collect::<Result<Vec<_>, _>>().unwrap();
    producer.join().unwrap();
    assert_eq!(streamed.len(), finite.len());
    for (i, row) in streamed.into_iter().enumerate() {
        assert_eq!(row.position(), i as u64 + 1);
        assert_eq!(row.into_result().unwrap(), Some(true));
    }
    assert_eq!(server.count() - finite_requests, finite_requests);
}

#[test]
fn an_idle_producer_gets_a_partial_request_before_it_closes_input() {
    let server = Loopback::noul(0.9).unwrap();
    let call = engine(&server)
        .open(&Decide::new("Ready?").unwrap())
        .unwrap();
    let sender = call.sender();
    sender.send(Record::text("first")).unwrap();
    let mut rows = call.rows();
    // The next send depends on the first response. Only the idle window can
    // make progress: input remains open and the batch is not full.
    let first = rows.next().unwrap().unwrap();
    assert_eq!(first.position(), 1);
    assert_eq!(server.count(), 1);
    sender.send(Record::text("second")).unwrap();
    sender.close();
    assert_eq!(rows.next().unwrap().unwrap().position(), 2);
    assert!(rows.next().is_none());
    assert_eq!(server.count(), 2);
}

#[cfg(feature = "command")]
mod hosts {
    use super::*;
    use std::io::Write;
    use std::process::Stdio;

    #[test]
    fn file_stdin_and_run_pack_individual_records_into_full_requests() {
        let directory = tempfile::TempDir::new().unwrap();
        let file = directory.path().join("records.csv");
        std::fs::write(
            &file,
            format!(
                "body\n{}",
                (0..500).map(|i| format!("row {i}\n")).collect::<String>()
            ),
        )
        .unwrap();
        let toolbox = directory.path().join("question.json");
        std::fs::write(
            &toolbox,
            r#"{"question":{"function":"decide","question":"Ready?"}}"#,
        )
        .unwrap();
        for host in ["file", "stdin", "run"] {
            let server = Loopback::noul(0.9).unwrap();
            let mut command = support::command();
            command
                .args([
                    "--backend",
                    "systemone",
                    "--url",
                    server.base(),
                    "--batch",
                    "50",
                ])
                .env("THINKTHEN_API_KEY", KEY);
            if host != "file" {
                command.arg("--json");
            }
            if host == "run" {
                command.args(["run", toolbox.to_str().unwrap(), "--from", "lines"]);
            } else {
                command.args(["decide", "Ready?"]);
                if host == "file" {
                    command.args([
                        file.to_str().unwrap(),
                        "--from",
                        "csv",
                        "--column",
                        "body",
                        "--into",
                        "answer",
                    ]);
                } else {
                    command.args(["--from", "lines"]);
                }
            }
            let mut child = command
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            let mut input = child.stdin.take().unwrap();
            let writer = std::thread::spawn(move || {
                if host != "file" {
                    for i in 0..500 {
                        writeln!(input, "row {i}").unwrap();
                        input.flush().unwrap();
                    }
                }
            });
            let output = child.wait_with_output().unwrap();
            writer.join().unwrap();
            assert!(
                output.status.success(),
                "{host}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            if host == "file" {
                let mut table =
                    csv::Reader::from_path(directory.path().join("records.thinkthen.csv")).unwrap();
                let answer = table
                    .headers()
                    .unwrap()
                    .iter()
                    .position(|name| name == "answer")
                    .unwrap();
                let rows = table.records().collect::<Result<Vec<_>, _>>().unwrap();
                assert_eq!(rows.len(), 500);
                for (i, row) in rows.iter().enumerate() {
                    assert_eq!(row.get(0), Some(format!("row {i}").as_str()));
                    assert_eq!(row.get(answer), Some("TRUE"));
                }
            } else {
                let rows = String::from_utf8(output.stdout)
                    .unwrap()
                    .lines()
                    .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(rows.len(), 500, "{host}");
                for (i, row) in rows.iter().enumerate() {
                    assert_eq!(row["position"], i + 1, "{host}");
                    assert_eq!(row["answer"], true, "{host}");
                }
            }
            assert_eq!(server.count(), 10, "{host}");
        }
    }
}
