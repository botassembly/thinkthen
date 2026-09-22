//! The compiled binary with several requests in flight, and the cache that resumes.

use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::harness::{Canned, Listener, spawn};

/// The question every case on this page asks.
const QUESTION: &str = "Does this report a payment failure?";

/// The numbers the answers take, one per record, so a row names its record.
const ODDS: [&str; 8] = [
    "0.11", "0.22", "0.33", "0.44", "0.55", "0.66", "0.77", "0.88",
];

/// This many records, each naming its own place.
fn records(count: usize) -> String {
    (1..=count)
        .map(|place| format!("{{\"id\":\"R-{place}\",\"body\":\"record {place}\"}}\n"))
        .collect()
}

/// The place the record in this request body names, counted from one.
fn ordinal(body: &[u8]) -> usize {
    String::from_utf8_lossy(body)
        .split_once("record ")
        .and_then(|(_, rest)| rest.split(['"', '\\', ' ']).next().map(str::to_owned))
        .and_then(|digits| digits.parse().ok())
        .unwrap_or(0)
}

/// The answer this record's place earns, with the probability only it carries.
fn answered(place: usize) -> String {
    let odds = ODDS.get(place.saturating_sub(1)).copied().unwrap_or("0.5");
    format!(
        concat!(
            r#"{{"model":"jev-1.13.0","answers":{{"q1":{{"type":"noul","noul":{odds}}}}},"#,
            r#""usage":{{"input_tokens":88,"output_tokens":12}}}}"#,
        ),
        odds = odds
    )
}

/// The answer a record earns, later records sooner than earlier ones.
fn later_sooner(body: &[u8]) -> Canned {
    let place = ordinal(body);
    let waiting = 20 * (9 - place.min(8)) as u64;
    Canned::ok(&answered(place)).after(waiting)
}

/// The answer a record earns on a run whose third record fails at the backend.
fn stopping_at_three(body: &[u8]) -> Canned {
    if ordinal(body) == 3 {
        return Canned::status(500, "{}").after(10);
    }
    later_sooner(body)
}

/// A listener that answers each record, later records sooner than earlier ones.
fn out_of_order() -> io::Result<Listener> {
    Listener::answering(later_sooner)
}

/// Run `decide` against one URL over the records on standard input.
fn decide(base: &str, arguments: &[&str], input: &str) -> io::Result<Output> {
    let asked = ["decide", QUESTION, "--url", base, "--model", "local-1"];
    spawn(
        &[&asked[..], arguments].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
}

/// What the run printed on standard output.
fn printed(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn wrapped(count: usize) -> String {
    (1..=count)
        .map(|place| {
            format!(
                "{{\"input\":{{\"id\":\"R-{place}\",\"body\":\"record {place}\"}},\"value\":false}}\n"
            )
        })
        .collect()
}

/// What the run said on standard error.
fn said(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A folder this test owns, removed and remade so each run starts empty.
fn folder(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

/// How many entries the folder holds.
fn entries(folder: &Path) -> usize {
    fs::read_dir(folder).map_or(0, |entries| {
        entries
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|value| value == "json")
            })
            .count()
    })
}

#[test]
fn a_backend_that_answers_out_of_order_still_prints_in_input_order() {
    let listener = out_of_order().expect("a loopback listener");
    let output = decide(
        listener.base(),
        &["--jsonl", "--field", "/body", "--details", "--jobs", "4"],
        &records(6),
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let rows = printed(&output);
    for (place, row) in rows.lines().enumerate() {
        assert!(row.contains(&format!(r#""id":"R-{}""#, place + 1)), "{row}");
        assert!(
            row.contains(&format!(r#""probability":{}"#, ODDS[place])),
            "{row}"
        );
    }
    assert_eq!(rows.lines().count(), 6);
    // The listener read them in another order than it answered them in.
    assert!(listener.peak() > 1, "peak {}", listener.peak());
}

#[test]
fn no_more_requests_are_in_flight_than_the_jobs_asked_for() {
    for jobs in ["1", "2", "4"] {
        let listener = Listener::answering(|body| Canned::ok(&answered(ordinal(body))).after(40))
            .expect("a loopback listener");
        let output = decide(
            listener.base(),
            &["--jsonl", "--field", "/body", "--jobs", jobs],
            &records(8),
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{jobs} jobs");
        assert_eq!(printed(&output).lines().count(), 8, "{jobs} jobs");
        let peak = listener.peak();
        let asked: usize = jobs.parse().expect("a number of jobs");
        // The bound is the claim. A loaded machine can fall short of it, so
        // the run asks for the bound and for more than one where one is due.
        assert!(peak <= asked, "{jobs} jobs reached {peak} in flight");
        assert!(
            asked == 1 || peak > 1,
            "{jobs} jobs reached {peak} in flight"
        );
    }
}

#[test]
fn every_number_of_jobs_prints_the_bytes_that_one_job_prints() {
    // The property the whole parallel path rests on: output never depends on
    // --jobs, on standard output or on standard error, finished or stopped.
    let refused = format!("{}{{\"id\":\"R-4\",\"note\":\"no body\"}}\n", records(3));
    let runs: [(&str, &[&str]); 3] = [
        (&records(6), &[]),
        (&refused, &[]),
        (&records(6), &["--max-retries", "0"]),
    ];

    for (input, extra) in runs {
        let reply: fn(&[u8]) -> Canned = if extra.contains(&"--max-retries") {
            stopping_at_three
        } else {
            later_sooner
        };
        let mut first: Option<(Option<i32>, String, String)> = None;
        for jobs in ["1", "2", "3", "5", "8", "32"] {
            let listener = Listener::answering(reply).expect("a loopback listener");
            let arguments = [&["--jsonl", "--field", "/body", "--jobs", jobs][..], extra].concat();
            let output =
                decide(listener.base(), &arguments, input).expect("the compiled binary runs");
            let seen = (output.status.code(), printed(&output), said(&output));

            match first.as_ref() {
                None => first = Some(seen),
                Some(one) => assert_eq!(&seen, one, "{jobs} jobs over {input}"),
            }
        }
    }
}

#[test]
fn a_stop_keeps_what_finished_after_it_and_a_rerun_pays_for_the_rest_alone() {
    let cache = folder("cache-resume");
    let named = cache.to_string_lossy().into_owned();
    // One listener answers both runs, because a recording entry is named by
    // the URL it was taken at. The third record fails once and answers after.
    let failed = Arc::new(AtomicBool::new(false));
    let listener = Listener::answering({
        let failed = Arc::clone(&failed);
        move |body| {
            let place = ordinal(body);
            if place == 3 && !failed.swap(true, Ordering::SeqCst) {
                return Canned::status(500, "{}").after(30);
            }
            Canned::ok(&answered(place))
        }
    })
    .expect("a loopback listener");

    let output = decide(
        listener.base(),
        &[
            "--jsonl",
            "--field",
            "/body",
            "--cache",
            &named,
            "--jobs",
            "4",
            "--max-retries",
            "0",
        ],
        &records(4),
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(4));
    assert_eq!(printed(&output), wrapped(2));
    let message = said(&output);
    assert!(message.contains("status 500"), "{message}");
    assert!(
        message.contains("stopped at record 3; 2 records finished"),
        "{message}"
    );
    assert_eq!(listener.requests().len(), 4);
    // Record 4 answered after the run had stopped, and it was billed, so it
    // is recorded and the rerun does not pay for it twice.
    assert_eq!(entries(&cache), 3);

    let output = decide(
        listener.base(),
        &[
            "--jsonl", "--field", "/body", "--cache", &named, "--jobs", "4",
        ],
        &records(4),
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(printed(&output), wrapped(4));
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(entries(&cache), 4);
}

#[test]
fn jobs_acts_in_record_mode_alone_and_inside_its_range() {
    let listener = Listener::answering(|body| Canned::ok(&answered(ordinal(body))))
        .expect("a loopback listener");

    let output =
        decide(listener.base(), &["--jobs", "2"], "record 1").expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(2));
    assert!(said(&output).contains("--jobs"), "{}", said(&output));
    assert!(listener.requests().is_empty());

    for outside in ["0", "33", "many"] {
        let output = decide(
            listener.base(),
            &["--jsonl", "--field", "/body", "--jobs", outside],
            &records(1),
        )
        .expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(2), "{outside}");
        assert!(said(&output).contains("--jobs"), "{}", said(&output));
    }

    let output = decide(
        listener.base(),
        &["--jsonl", "--field", "/body", "--jobs", "32"],
        &records(2),
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(printed(&output), wrapped(2));
}

#[test]
fn one_process_reuses_the_connections_it_opens() {
    for (jobs, most) in [("1", 1_usize), ("4", 4)] {
        let listener = Listener::answering(|body| Canned::ok(&answered(ordinal(body))))
            .expect("a loopback listener");
        let output = decide(
            listener.base(),
            &["--jsonl", "--field", "/body", "--jobs", jobs],
            &records(6),
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{jobs} jobs");
        assert_eq!(printed(&output).lines().count(), 6, "{jobs} jobs");
        let opened = listener.connections();
        assert!(
            (1..=most).contains(&opened),
            "{jobs} jobs opened {opened} connections"
        );
    }
}

#[test]
fn a_reader_that_closes_the_pipe_stops_the_reading_and_the_scheduling() {
    let listener = Listener::answering(|body| Canned::ok(&answered(ordinal(body))).after(30))
        .expect("a loopback listener");
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .args([
            "decide",
            QUESTION,
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--jsonl",
            "--field",
            "/body",
            "--jobs",
            "4",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the compiled binary runs");

    let mut input = child.stdin.take().expect("a pipe to standard input");
    input
        .write_all(records(24).as_bytes())
        .expect("the records are written");
    drop(input);

    // One row is read and the pipe is closed, as `head -1` closes it.
    let mut reader = BufReader::new(child.stdout.take().expect("a pipe from standard output"));
    let mut row = String::new();
    reader.read_line(&mut row).expect("one row");
    drop(reader);

    let status = child.wait().expect("the compiled binary ends");
    assert_eq!(status.code(), Some(0));
    assert_eq!(row, wrapped(1));
    // The tool learns of the closed pipe from the write that fails, so it
    // stops within one round of requests. The count stays near --jobs and
    // never reaches the input. A tool that kept scheduling would ask, and
    // pay, for all 24.
    let sent = listener.requests().len();
    assert!(sent <= 12, "the run kept scheduling and sent {sent}");
}

#[test]
fn records_that_are_byte_for_byte_alike_write_one_entry_and_race_with_nobody() {
    // Identical records make identical requests, so every worker misses the
    // same digest and every worker writes the same entry at once.
    let folder = folder("same-digest");
    let named = folder.to_string_lossy().into_owned();
    let listener =
        Listener::answering(|_| Canned::ok(&answered(1)).after(20)).expect("a loopback listener");
    let same: String = (0..16)
        .map(|_| "{\"id\":\"R-1\",\"body\":\"record 1\"}\n".to_owned())
        .collect();

    let output = decide(
        listener.base(),
        &[
            "--jsonl", "--field", "/body", "--record", &named, "--jobs", "8",
        ],
        &same,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    assert_eq!(printed(&output).lines().count(), 16);
    assert_eq!(entries(&folder), 1);
}

#[test]
fn equal_cache_misses_send_once_at_every_supported_width() {
    let same: String = (0..16)
        .map(|_| "{\"id\":\"R-1\",\"body\":\"record 1\"}\n".to_owned())
        .collect();
    for jobs in ["1", "4", "32"] {
        let cache = folder(&format!("same-cache-digest-{jobs}"));
        let named = cache.to_string_lossy();
        let listener = Listener::answering(|_| Canned::ok(&answered(1)).after(20))
            .expect("a loopback listener");
        let output = decide(
            listener.base(),
            &[
                "--jsonl",
                "--field",
                "/body",
                "--details",
                "--cache",
                &named,
                "--jobs",
                jobs,
            ],
            &same,
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{}", said(&output));
        let rows = printed(&output);
        assert_eq!(rows.lines().count(), 16);
        assert_eq!(rows.matches(r#""replayed":false"#).count(), 1);
        assert_eq!(rows.matches(r#""replayed":true"#).count(), 15);
        assert_eq!(listener.requests().len(), 1, "jobs {jobs}");
        assert_eq!(entries(&cache), 1);
    }
}

#[test]
fn different_cache_digests_do_not_share_a_lock() {
    let cache = folder("different-cache-digests");
    let named = cache.to_string_lossy();
    let listener = Listener::answering(|body| Canned::ok(&answered(ordinal(body))).after(50))
        .expect("a loopback listener");
    let output = decide(
        listener.base(),
        &[
            "--jsonl", "--field", "/body", "--cache", &named, "--jobs", "2",
        ],
        &records(2),
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    assert_eq!(listener.requests().len(), 2);
    assert_eq!(listener.peak(), 2);
}
