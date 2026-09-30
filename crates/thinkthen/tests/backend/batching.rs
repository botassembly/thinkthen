//! Records of a stream share requests on `decide`, `filter` and `rank`, by
//! ADR 0048 items 1 to 6 and ADR 0055.
//!
//! The loopback answers each wire question with a probability made from the
//! line the question quotes, so a row's answer follows its record whatever
//! the batch.

use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Map, Value, json};

use crate::harness::{Canned, Gathering, Listener, finish, spawn};

mod ceiling;
mod choose;
mod context;
mod portable;
mod tag_score;
mod tiers;
mod too_large;
mod warning;

pub(super) const QUESTION: &str = "It names a place.";
const QUOTED: &str = "Each question quotes the text it asks about.";
pub(super) const KEY: (&str, &str) = ("THINKTHEN_API_KEY", "sk-test-value");

pub(super) fn lines(places: impl Iterator<Item = usize>) -> String {
    places.map(|place| format!("line {place}\n")).collect()
}

/// The number after `line ` in a text, when it holds one.
fn place(text: &str) -> Option<usize> {
    let (_, rest) = text.split_once("line ")?;
    rest.split(|c: char| !c.is_ascii_digit())
        .next()?
        .parse()
        .ok()
}

fn odds(place: usize) -> f64 {
    f64::from(u32::try_from(place * 37 % 100).unwrap_or(0)) / 100.0
}

/// The row a line prints without `--details`.
fn row(place: usize) -> String {
    format!(
        "{{\"input\":\"line {place}\",\"value\":{}}}\n",
        odds(place) >= 0.5
    )
}

fn rows(places: impl Iterator<Item = usize>) -> String {
    places.map(row).collect()
}

/// Each wire question's name and the line it asks about. A batch of one
/// sends the line as the evidence and the question unquoted.
pub(super) fn places(body: &[u8]) -> Vec<(String, usize)> {
    let request: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
    let state = request["state"].as_str().unwrap_or_default();
    let Some(questions) = request["questions"].as_object() else {
        return Vec::new();
    };
    questions
        .iter()
        .map(|(name, question)| {
            let text = question["instructions"].as_str().unwrap_or_default();
            (
                name.clone(),
                place(text).or_else(|| place(state)).unwrap_or(0),
            )
        })
        .collect()
}

fn first(body: &[u8]) -> usize {
    places(body).iter().map(|&(_, at)| at).min().unwrap_or(0)
}

/// A reply answering every question but the one about line `skip`, with the
/// usage given.
fn reply(body: &[u8], skip: Option<usize>, usage: Option<(u64, u64)>) -> Canned {
    let answers: Map<String, Value> = places(body)
        .into_iter()
        .filter(|&(_, at)| Some(at) != skip)
        .map(|(name, at)| (name, json!({"type": "noul", "noul": odds(at)})))
        .collect();
    let mut reply = json!({"model": "jev-1.13.0", "answers": answers});
    if let Some((input, output)) = usage {
        reply["usage"] = json!({"input_tokens": input, "output_tokens": output});
    }
    Canned::ok(&reply.to_string())
}

pub(super) fn answering(body: &[u8]) -> Canned {
    reply(body, None, Some((88, 12)))
}

/// `decide QUESTION --lines` at the loopback, with the extra arguments.
fn decide(base: &str, extra: &[&str], environment: &[(&str, &str)], input: &str) -> Output {
    let fixed = [
        "decide",
        QUESTION,
        "--lines",
        "--url",
        base,
        "--model",
        "jev-1.13.0",
    ];
    let arguments = [&fixed[..], extra].concat();
    spawn(
        &arguments,
        &[environment, &[KEY]].concat(),
        input.as_bytes(),
    )
    .expect("the command runs")
}

pub(super) fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// A fresh folder under the target folder.
pub(super) fn folder(name: &str) -> String {
    static FOLDERS: AtomicUsize = AtomicUsize::new(0);
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "batching-{name}-{}-{}",
        std::process::id(),
        FOLDERS.fetch_add(1, Ordering::Relaxed)
    ));
    let _removed = fs::remove_dir_all(&path);
    path.to_string_lossy().into_owned()
}

/// The parsed `--details` rows.
pub(super) fn details(output: &Output) -> Vec<Value> {
    text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("a JSON row"))
        .collect()
}

#[test]
fn order_holds_across_jobs() {
    let input = lines(1..=306);
    let mut runs = Vec::new();
    for jobs in [1, 8] {
        let gathering = (jobs > 1).then(|| Gathering::new(jobs));
        let listener = Listener::answering(move |body| {
            if let Some(gathering) = &gathering {
                gathering.hold();
            }
            let later = 310_usize.saturating_sub(first(body)) / 10;
            answering(body).after(u64::try_from(later).unwrap_or(0))
        })
        .expect("a loopback listener");
        let jobs_text = jobs.to_string();
        let output = decide(
            listener.base(),
            &["--batch", "10", "--no-cache", "--jobs", &jobs_text],
            &[],
            &input,
        );
        let mut sizes: Vec<usize> = listener
            .requests()
            .iter()
            .map(|request| places(&request.body).len())
            .collect();
        sizes.sort_unstable();
        assert_eq!(sizes, [&[6][..], &[10; 30]].concat(), "at --jobs {jobs}");
        if jobs > 1 {
            assert_eq!(listener.peak(), jobs, "batches in flight");
        }
        runs.push(output);
    }
    assert_eq!(text(&runs[0].stdout), rows(1..=306));
    assert_eq!(text(&runs[1].stdout), text(&runs[0].stdout));
    assert_eq!(text(&runs[1].stderr), text(&runs[0].stderr));
    assert_eq!(runs[1].status.code(), Some(0));
}

#[test]
fn replay_answers_every_batch() {
    let input = lines(1..=25);
    let listener = Listener::answering(answering).expect("a loopback listener");
    let base = listener.base();
    let recording = folder("recording");
    let recorded = decide(
        base,
        &["--batch", "10", "--record", &recording],
        &[],
        &input,
    );
    assert_eq!(
        recorded.status.code(),
        Some(0),
        "{}",
        text(&recorded.stderr)
    );
    assert_eq!(listener.count(), 3);

    let replayed = decide(
        base,
        &["--batch", "10", "--replay", &recording],
        &[],
        &input,
    );
    assert_eq!(listener.count(), 3, "a replay sends nothing");
    assert_eq!(text(&replayed.stdout), text(&recorded.stdout));
    assert_eq!(text(&replayed.stderr), text(&recorded.stderr));
    assert_eq!(text(&replayed.stdout), rows(1..=25));

    // Each question is stored alone, so another batch setting replays too.
    let rebatched = decide(base, &["--batch", "5", "--replay", &recording], &[], &input);
    assert_eq!(text(&rebatched.stdout), rows(1..=25));
    assert_eq!(
        listener.count(),
        3,
        "a replay at another batch setting sends nothing"
    );

    let missed = decide(
        base,
        &["--batch", "10", "--replay", &recording],
        &[],
        &lines(1..=26),
    );
    assert_eq!(missed.status.code(), Some(5));
    assert_eq!(text(&missed.stdout), rows(1..=25));
    // The key holds the loopback's port, so the line is pinned around it.
    let stopped = text(&missed.stderr);
    let (head, tail) = stopped.split_once("question `").expect("a key");
    assert_eq!(
        (
            head,
            tail.split_once('`').map(|(key, tail)| (key.len(), tail))
        ),
        (
            "thinkthen: the decide request: the replay folder holds no answer for ",
            Some((
                64,
                concat!(
                    "; the key is the SHA-256 of the adapter, address, model, shared state and question as sent\n",
                    "thinkthen: stopped at record 26; 25 records finished, 25 records from a recording\n"
                )
            ))
        )
    );
    assert_eq!(listener.count(), 3);

    let cache = folder("cache");
    let cached = decide(base, &["--batch", "10", "--cache", &cache], &[], &input);
    let again = decide(base, &["--batch", "10", "--cache", &cache], &[], &input);
    assert_eq!(listener.count(), 6, "the second cached run sends nothing");
    assert_eq!(text(&again.stdout), text(&cached.stdout));
}

/// A pause row: its name, its extra arguments and its environment.
type Mode<'a> = (&'a str, &'a [&'a str], &'a [(&'a str, &'a str)]);

#[test]
fn a_table_shares_one_request_and_filter_prints_its_kept_rows() {
    let listener = Listener::answering(answering).expect("a loopback listener");
    let fixed = [
        "filter",
        QUESTION,
        "--csv",
        "--no-cache",
        "--url",
        listener.base(),
    ];
    let arguments = [&fixed[..], &["--model", "jev-1.13.0"]].concat();
    let table = b"body\nline 1\nline 2\nline 3\n";
    let output = spawn(&arguments, &[KEY], table).expect("the command runs");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let sent = listener.requests();
    assert_eq!(sent.len(), 1, "three rows share one request");
    assert_eq!(places(&sent[0].body).len(), 3);
    assert_eq!(text(&output.stdout), "{\"body\":\"line 2\"}\n");
}

#[test]
fn a_pause_sends_the_open_batch() {
    let typed = folder("typed");
    let named = folder("named");
    let modes: [Mode<'_>; 3] = [
        ("no folder", &["--no-cache"], &[]),
        ("the default cache", &[], &[("THINKTHEN_CACHE", &named)]),
        ("a typed folder", &["--cache", &typed], &[]),
    ];
    for (name, extra, environment) in modes {
        let listener = Listener::answering(answering).expect("a loopback listener");
        let fixed = ["decide", QUESTION, "--lines", "--url", listener.base()];
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command
            .env_clear()
            .env("HOME", folder("home"))
            .env(KEY.0, KEY.1)
            .envs(environment.iter().copied())
            .args(fixed)
            .args(["--model", "jev-1.13.0"])
            .args(extra)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().expect("the command starts");
        let mut writer = child.stdin.take().expect("an input pipe");
        writer
            .write_all(lines(1..=3).as_bytes())
            .expect("three records are written");
        writer.flush().expect("the records reach the pipe");
        let deadline = Instant::now() + Duration::from_secs(5);
        while listener.count() == 0 && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(10));
        }
        let sent = listener.requests();
        drop(writer);
        let output = finish(child, name).expect("the command ends");
        assert_eq!(sent.len(), 1, "{name}: a request while the pipe stays open");
        assert_eq!(places(&sent[0].body).len(), 3, "{name}");
        assert_eq!(text(&output.stdout), rows(1..=3), "{name}");
    }
}

/// Equal questions in one call are asked once, by ADR 0111 section 2: ten
/// thousand lines of five values send one request of five questions, and
/// every row reads its question's answer.
#[test]
fn equal_records_ask_their_question_once() {
    let repeats: String = (0..10_000)
        .map(|at| format!("line {}\n", at % 5 + 1))
        .collect();
    let listener = Listener::answering(answering).expect("a loopback listener");
    let output = decide(listener.base(), &["--no-cache"], &[], &repeats);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let bodies = listener.requests();
    assert_eq!(bodies.len(), 1);
    assert!(text(&bodies[0].body).contains(QUOTED));
    let mut asked: Vec<usize> = places(&bodies[0].body).iter().map(|&(_, at)| at).collect();
    asked.sort_unstable();
    assert_eq!(asked, [1, 2, 3, 4, 5]);
    let expected: String = (0..10_000).map(|at| row(at % 5 + 1)).collect();
    assert_eq!(text(&output.stdout), expected);
}

/// Thirty lines at `--jobs 1` and no retry, in batches of `batch`: what printed,
/// what the run said, and its exit code.
fn failing(
    batch: &str,
    answer: impl Fn(&[u8]) -> Canned + Send + Sync + 'static,
) -> (String, String, Option<i32>) {
    let listener = Listener::answering(answer).expect("a loopback listener");
    let fixed = ["--jobs", "1", "--max-retries", "0", "--no-cache"];
    let arguments = [&fixed[..], &["--batch", batch]].concat();
    let output = decide(listener.base(), &arguments, &[], &lines(1..=30));
    (
        text(&output.stdout),
        text(&output.stderr),
        output.status.code(),
    )
}

#[test]
fn a_failed_batch_stops_at_its_first_record() {
    let unavailable = |body: &[u8]| -> Canned {
        if places(body).iter().any(|&(_, at)| at == 11) {
            Canned::status(503, "{}")
        } else {
            answering(body)
        }
    };
    let cause = "the backend answered with status 503: the backend failed after the allowed attempts; try again later or change --max-retries";
    assert_eq!(
        failing("10", unavailable),
        (
            rows(1..=10),
            format!(
                "thinkthen: stopped at record 11; the request for records 11 to 20 failed: {cause}; 10 records finished\n"
            ),
            Some(4)
        )
    );
    assert_eq!(
        failing("10", |body| reply(body, Some(13), Some((88, 12)))),
        (
            rows(1..=12),
            "thinkthen: stopped at record 13; the reply for records 11 to 20 gave record 13 no usable answer; 12 records finished\n".to_owned(),
            Some(4)
        )
    );
    assert_eq!(
        failing("1", unavailable),
        (
            rows(1..=10),
            format!("thinkthen: {cause}\nthinkthen: stopped at record 11; 10 records finished\n"),
            Some(4)
        )
    );
}

#[test]
fn each_row_carries_its_share() {
    let song = "The text is the title of a song by the Beatles.";
    let input = "Come Together\nBecause\nCome Together\n";
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specification/fixtures/systemone/batch-duplicate.request.json"
    );
    let expected = fs::read_to_string(fixture).expect("the fixture");
    for usage in [Some((100, 10)), None] {
        let listener =
            Listener::answering(move |body| reply(body, None, usage)).expect("a loopback listener");
        let arguments = [
            "decide",
            song,
            "--lines",
            "--batch",
            "3",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "jev-latest",
        ];
        let output = spawn(&arguments, &[KEY], input.as_bytes()).expect("the command runs");
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        let sent = listener.requests();
        assert_eq!(sent.len(), 1);
        assert_eq!(text(&sent[0].body), expected.trim_end_matches('\n'));
        let printed = details(&output);
        assert!(printed.iter().all(|row| row["meta"].get("batch").is_none()));
        let shares: Vec<(Value, Value)> = printed
            .iter()
            .map(|row| {
                (
                    row["meta"]["usage"].clone(),
                    row["meta"]["requests_sent"].clone(),
                )
            })
            .collect();
        // The request asks two questions, so each takes half, the remainder
        // to the earliest. The copy reads the first question's answer and
        // usage share, and counts no send.
        let wanted = match usage {
            Some(_) => vec![
                (json!({"input_tokens": 50, "output_tokens": 5}), json!(1)),
                (json!({"input_tokens": 50, "output_tokens": 5}), json!(0)),
                (json!({"input_tokens": 50, "output_tokens": 5}), json!(0)),
            ],
            None => vec![
                (Value::Null, json!(1)),
                (Value::Null, json!(0)),
                (Value::Null, json!(0)),
            ],
        };
        assert_eq!(shares, wanted, "usage {usage:?}");
    }
}

/// A failed request stops the run sending, as one job did, even while an
/// earlier record's request is still out: that request finishes, and no
/// later record's request goes (ticket 0304 slice 4).
#[test]
fn a_failed_request_sends_no_later_request() {
    let listener = Listener::answering(|body| {
        if first(body) == 2 {
            Canned::status(500, "{}")
        } else {
            answering(body).after(200)
        }
    })
    .expect("a loopback listener");
    let options = [
        "--batch",
        "1",
        "--jobs",
        "2",
        "--max-retries",
        "0",
        "--no-cache",
    ];
    let output = decide(listener.base(), &options, &[], &lines(1..=6));
    assert_eq!(
        output.status.code(),
        Some(4),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.count(), 2, "line 1's request and line 2's");
    assert_eq!(text(&output.stdout), row(1));
}
