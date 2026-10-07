//! The compiled binary with several requests in flight, and the cache that resumes.

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup or a missing field should stop the boundary test"
)]

use crate::child::ChildEnvironment as _;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Duration;

use conformance_backend::Rendezvous;

use crate::harness::{Canned, Gathering, Listener, finish, spawn_one as spawn};

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
            r#"{{"model":"local-1","answers":{{"q1":{{"type":"noul","noul":{odds}}}}},"#,
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

/// Run `decide` against one URL over the records on standard input.
fn decide(base: &str, arguments: &[&str], input: &str) -> io::Result<Output> {
    let asked = ["decide", QUESTION, "--url", base, "--model", "local-1"];
    spawn(
        &[&asked[..], arguments].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
}

/// Start `decide` over JSON records without the cache, with standard input left open.
fn piped(base: &str, jobs: Option<&str>) -> io::Result<Child> {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let home = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "parallel-home-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .clear_environment()
        .home(home)
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .args([
            "decide",
            QUESTION,
            "--model",
            "local-1",
            "--no-cache",
            "--batch",
            "1",
        ])
        .args(["--url", base, "--jsonl", "--field", "/body"])
        .args(jobs.into_iter().flat_map(|jobs| ["--jobs", jobs]))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
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

/// How many answers the folder's store holds; each record here asks one question.
fn entries(folder: &Path) -> usize {
    if !folder.join("thinkthen.sqlite").exists() {
        return 0;
    }
    crate::support::stored(folder).expect("the store").len()
}

#[test]
fn a_backend_that_answers_out_of_order_still_prints_in_input_order() {
    // The first four requests wait until all are in flight, so the peak
    // passes one however loaded the machine is (ticket 0352).
    let gathering = Gathering::new(4);
    let listener = Listener::answering(move |body| {
        gathering.hold();
        later_sooner(body)
    })
    .expect("a loopback listener");
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
    for (jobs, asked, count) in [
        (Some("1"), 1, 8),
        (Some("2"), 2, 8),
        (Some("4"), 4, 8),
        (None, 8, 16),
    ] {
        // The first `asked` requests wait until all are in flight, so the peak
        // reaches the bound however loaded the machine is (ticket 0352).
        let gathering = Gathering::new(asked);
        let listener = Listener::answering(move |_| {
            gathering.hold();
            Canned::ok(&answered(1))
        })
        .expect("a loopback listener");
        let mut arguments = vec!["--jsonl", "--field", "/body", "--batch", "1"];
        arguments.extend(jobs.into_iter().flat_map(|jobs| ["--jobs", jobs]));
        let output =
            decide(listener.base(), &arguments, &records(count)).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{jobs:?} jobs");
        assert_eq!(printed(&output), wrapped(count), "{jobs:?} jobs");
        assert_eq!(listener.count(), count);
        let peak = listener.peak();
        assert_eq!(peak, asked, "{jobs:?} jobs reached {peak} in flight");
    }
}

/// The property the whole parallel path rests on: output never depends on
/// --jobs, on standard output or on standard error, finished or stopped.
/// Each input is its own test, so the three spread over the runner.
fn every_number_of_jobs_prints_what_one_job_prints(input: &str, extra: &[&str]) {
    let reply: fn(&[u8]) -> Canned = if extra.contains(&"--max-retries") {
        stopping_at_three
    } else {
        later_sooner
    };
    let mut first: Option<(Option<i32>, String, String)> = None;
    for jobs in ["1", "2", "3", "5", "8", "32"] {
        let listener = Listener::answering(reply).expect("a loopback listener");
        let arguments = [&["--jsonl", "--field", "/body", "--jobs", jobs][..], extra].concat();
        let output = decide(listener.base(), &arguments, input).expect("the compiled binary runs");
        let seen = (output.status.code(), printed(&output), said(&output));

        match first.as_ref() {
            None => first = Some(seen),
            Some(one) => assert_eq!(&seen, one, "{jobs} jobs over {input}"),
        }
    }
}

#[test]
fn every_number_of_jobs_prints_the_bytes_that_one_job_prints() {
    every_number_of_jobs_prints_what_one_job_prints(&records(6), &[]);
}

#[test]
fn every_number_of_jobs_prints_the_refusal_that_one_job_prints() {
    let refused = format!("{}{{\"id\":\"R-4\",\"note\":\"no body\"}}\n", records(3));
    every_number_of_jobs_prints_what_one_job_prints(&refused, &[]);
}

#[test]
fn every_number_of_jobs_prints_the_stop_that_one_job_prints() {
    every_number_of_jobs_prints_what_one_job_prints(&records(6), &["--max-retries", "0"]);
}

#[test]
fn a_stop_keeps_what_finished_after_it_and_a_rerun_pays_for_the_rest_alone() {
    let cache = folder("cache-resume");
    let named = cache.to_string_lossy().into_owned();
    // One listener answers both runs, because a recording entry is named by
    // the URL it was taken at. The third record fails once and answers after.
    // The first run's four replies wait until all four requests are in, so
    // record 4 is always sent before record 3 fails (ticket 0352).
    let failed = Arc::new(AtomicBool::new(false));
    let first_run = Arc::new(Rendezvous::new(4));
    let seen = Arc::new(AtomicUsize::new(0));
    let listener = Listener::answering({
        let failed = Arc::clone(&failed);
        move |body| {
            let place = ordinal(body);
            let reply = if place == 3 && !failed.swap(true, Ordering::SeqCst) {
                Canned::status(500, "{}")
            } else {
                Canned::ok(&answered(place))
            };
            if seen.fetch_add(1, Ordering::SeqCst) < 4 {
                reply.after_release(Arc::clone(&first_run))
            } else {
                reply
            }
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
fn a_run_opens_one_connection_for_each_job_and_reuses_it() {
    for (explicit, jobs) in [
        (true, 1_usize),
        (true, 4),
        (true, 16),
        (true, 32),
        (false, 8),
    ] {
        // Each round is held until all of its requests are in flight, so the
        // first opens `jobs` connections and the second needs them all again.
        let rounds = [(); 2].map(|()| Gathering::new(jobs));
        let (told, answers) = mpsc::channel();
        let listener = Listener::answering(move |body| {
            let place = ordinal(body);
            rounds[usize::from(place > jobs)].hold();
            Canned::ok(&answered(1)).notifying(told.clone())
        })
        .expect("a loopback listener");
        let mut child = piped(
            listener.base(),
            explicit.then_some(jobs.to_string()).as_deref(),
        )
        .expect("the compiled binary runs");
        let mut input = child.stdin.take().expect("a pipe to standard input");
        let all = records(2 * jobs);
        let lines: Vec<&str> = all.split_inclusive('\n').collect();
        input
            .write_all(lines[..jobs].concat().as_bytes())
            .expect("round one");
        for _ in 0..jobs {
            if let Err(error) = answers.recv_timeout(Duration::from_secs(30)) {
                drop(input);
                let output = finish(child, "unanswered parallel round").expect("bounded child");
                panic!("the first round is unanswered: {error}; {jobs} jobs; {output:?}");
            }
        }
        // Every connection now sits idle in the pool, where a small pool trims it.
        std::thread::sleep(Duration::from_millis(200));
        input
            .write_all(lines[jobs..].concat().as_bytes())
            .expect("round two");
        drop(input);
        let output = finish(child, "thinkthen over two rounds").expect("the compiled binary ends");

        assert_eq!(output.status.code(), Some(0), "{jobs} jobs");
        assert_eq!(printed(&output), wrapped(2 * jobs), "{jobs} jobs");
        assert_eq!(listener.count(), 2 * jobs, "{jobs} jobs");
        assert_eq!(listener.peak(), jobs, "{jobs} jobs");
        assert_eq!(listener.connections(), jobs, "{jobs} jobs");
    }
}

#[test]
fn a_reader_that_closes_the_pipe_stops_the_reading_and_the_scheduling() {
    let listener = Listener::answering(|body| Canned::ok(&answered(ordinal(body))).after(30))
        .expect("a loopback listener");
    let mut child = piped(listener.base(), Some("4")).expect("the compiled binary runs");

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

    let output = finish(child, "thinkthen after `head -1`").expect("the compiled binary ends");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
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
        // A row the call's in-flight map answers is not from the store, so
        // how many rows say `cached: false` depends on timing. The one send
        // is counted once across the rows all the same.
        let cached = rows.matches(r#""cached":true"#).count();
        assert!(cached < 16, "jobs {jobs}: {rows}");
        assert_eq!(rows.matches(r#""cached":false"#).count() + cached, 16);
        let sent = rows.matches(r#""requests_sent":1,"#).count();
        assert_eq!(rows.matches(r#""requests_sent":0,"#).count() + sent, 16);
        assert_eq!(sent, 1, "jobs {jobs}: the rows count one send");
        assert_eq!(listener.requests().len(), 1, "jobs {jobs}");
        assert_eq!(entries(&cache), 1);
    }
}

#[test]
fn different_cache_digests_do_not_share_a_lock() {
    let cache = folder("different-cache-digests");
    let named = cache.to_string_lossy();
    // Each request waits until both are in flight, so a lock shared across
    // digests holds the second request back and the peak stays at 1.
    let gathering = Gathering::new(2);
    let listener = Listener::answering(move |body| {
        gathering.hold();
        Canned::ok(&answered(ordinal(body)))
    })
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
