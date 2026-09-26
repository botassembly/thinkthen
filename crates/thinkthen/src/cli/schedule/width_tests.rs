//! The command's width setup and the one cap every live path shares.
//!
//! Width state belongs to the process, so each proof runs in a child copy of
//! this test binary where no unrelated test shares the gate.

use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::{env, fs};

use clap::Parser as _;
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};

use crate::args::{Cli, Command as Verb};
use crate::edge::Environment;
use crate::engine::http::{Client, Exchange, Key};
use crate::engine::{Cancel, WIDTH_CHILD, Width, Widths, process_width};
use crate::failure::{Failure, report};

mod facade_tests;

const CHILD: &str = "THINKTHEN_TEST_WIDTH_CHILD";
const QUIET: Duration = Duration::from_millis(150);

/// Run one ignored test of this file alone in a fresh copy of the binary.
fn in_child(name: &str) {
    in_child_at(&format!("cli::schedule::width_tests::{name}"));
}

/// Run the ignored test at `path` alone in a fresh copy of the binary. A
/// child still running at the test deadline is killed and fails the test.
pub(crate) fn in_child_at(path: &str) {
    let name = path.replace("::", "-");
    let home = env::temp_dir().join(format!("thinkthen-width-{name}-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).expect("child home");
    let mut command = Command::new(env::current_exe().expect("test binary"));
    command
        .args([
            "--exact",
            path,
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .env_clear()
        .env(CHILD, "1")
        .env("HOME", &home)
        .env("XDG_CACHE_HOME", home.join("cache"))
        .env("XDG_CONFIG_HOME", home.join("config"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let child = command.spawn().expect("child");
    let output = crate::test_deadline::finish(child, path).expect("child");
    let _removed = fs::remove_dir_all(&home);
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("1 passed"),
        "the child ran its test"
    );
}

/// Only a child started by [`in_child`] runs the body of an ignored test.
pub(crate) fn child() -> bool {
    let chosen = env::var_os(CHILD).is_some();
    WIDTH_CHILD.store(chosen, Ordering::Release);
    chosen
}

/// Standard input that counts every read, so a refusal proves none happened.
struct Counted(Cursor<Vec<u8>>, Arc<AtomicUsize>);

impl Read for Counted {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.1.fetch_add(1, Ordering::SeqCst);
        self.0.read(buffer)
    }
}

/// Run one command line in this process, as `main` would, but on this input.
fn dispatch(
    arguments: &[&str],
    environment: &Environment,
    input: impl Read + Send + 'static,
) -> Result<ExitCode, Failure> {
    let cli = Cli::try_parse_from(["thinkthen"].iter().chain(arguments)).expect("command line");
    let mut output = Vec::new();
    match &cli.command {
        Some(Verb::Decide(held)) => crate::judge::decide(held, environment, input, &mut output),
        Some(Verb::Choose(held)) => crate::judge::choose(held, environment, input, &mut output),
        Some(Verb::Annotate(held)) => crate::annotate::run(held, environment, input, &mut output),
        Some(Verb::Find(held)) => crate::cli::find::run(held, environment, input, &mut output),
        Some(Verb::Recognize(held)) => {
            crate::cli::recognize::run(held, environment, input, &mut output)
        }
        Some(Verb::Relate(held)) => crate::cli::relate::run(held, environment, input, &mut output),
        _ => panic!("no arm for {arguments:?}"),
    }
}

/// Answer every question: yes at 0.9, and the first option of any choice.
fn generic(body: &[u8]) -> String {
    let request: Value = serde_json::from_slice(body).expect("request");
    let mut answers = serde_json::Map::new();
    for (name, question) in request["questions"].as_object().expect("questions") {
        let answer = if question["type"] == "noul" {
            json!({"type": "noul", "noul": 0.9})
        } else {
            let options = question["criteria"].as_object().expect("criteria");
            let first = options.keys().next().expect("an option").clone();
            let odds: serde_json::Map<_, _> = options
                .keys()
                .map(|key| (key.clone(), json!(f64::from(u8::from(*key == first)))))
                .collect();
            json!({"type": question["type"], "choice": first, "probabilities": odds})
        };
        answers.insert(name.clone(), answer);
    }
    json!({"model": request["model"], "answers": answers}).to_string()
}

fn requests_counted(environment: &Environment) -> u64 {
    environment.usage().finish();
    let path = environment.usage_path().expect("a usage folder");
    crate::engine::usage::read(path, &crate::engine::usage::month_now())
        .expect("usage totals")
        .total
        .requests_sent
}

#[test]
fn the_command_selects_only_an_explicit_jobs_and_refuses_a_later_different_one() {
    in_child("command_setup_child");
}

#[test]
#[ignore = "runs alone in a child process"]
fn command_setup_child() {
    if !child() {
        return;
    }
    let listener = Listener::answering(|body| Canned::ok(&generic(body))).expect("listener");
    let environment = Environment::read().expect("environment");
    let decide = |jobs: Option<&'static str>, reads: &Arc<AtomicUsize>| {
        let mut line = vec![
            "decide",
            "Is it late?",
            "--url",
            listener.base(),
            "--no-cache",
        ];
        line.extend(["--lines", "--model", "local-1"]);
        line.extend(jobs.iter().flat_map(|jobs| ["--jobs", jobs]));
        let input = Counted(Cursor::new(b"one\ntwo\n".to_vec()), Arc::clone(reads));
        dispatch(&line, &environment, input)
    };

    let reads = Arc::new(AtomicUsize::new(0));
    assert_eq!(decide(None, &reads).expect("omitted"), ExitCode::SUCCESS);
    assert_eq!(
        process_width().selected(),
        None,
        "an omitted --jobs selects nothing"
    );
    assert_eq!(
        decide(Some("4"), &reads).expect("explicit"),
        ExitCode::SUCCESS
    );
    assert_eq!(process_width().selected(), Width::new(4).ok());
    assert_eq!(
        decide(Some("4"), &reads).expect("the same width"),
        ExitCode::SUCCESS
    );

    let before = (
        listener.connections(),
        listener.count(),
        environment.cancel().keys(),
        requests_counted(&environment),
    );
    assert_eq!(before.1, 6);
    assert_eq!(before.3, 6);
    let unread = Arc::new(AtomicUsize::new(0));
    let refused = decide(Some("8"), &unread).expect_err("a different width");
    let mut said = Vec::new();
    assert_eq!(report(&refused, &mut said), ExitCode::from(2));
    assert_eq!(
        String::from_utf8(said).expect("text"),
        "thinkthen: throttle 4 is already active for this process; use throttle 4 or drop the throttle argument\n"
    );
    assert_eq!(unread.load(Ordering::SeqCst), 0, "no input was read");
    let after = (
        listener.connections(),
        listener.count(),
        environment.cancel().keys(),
        requests_counted(&environment),
    );
    assert_eq!(after, before, "no connection, request, key, or count");
    assert_eq!(process_width().selected(), Width::new(4).ok());
}

#[test]
fn the_width_setup_maps_each_jobs_value_to_one_selection() {
    let widths: &'static Widths = Box::leak(Box::default());
    assert_eq!(super::width_in(widths, None).expect("omitted"), 4);
    assert_eq!(widths.selected(), None);
    assert_eq!(super::width_in(widths, Some(4)).expect("explicit"), 4);
    assert_eq!(widths.selected(), Width::new(4).ok());
    assert!(matches!(
        super::width_in(widths, Some(8)),
        Err(Failure::WidthActive(active)) if Some(active.0) == Width::new(4).ok()
    ));
    assert!(matches!(
        super::width_in(Box::leak(Box::default()), Some(0)),
        Err(Failure::Usage(
            "a width is a whole number from 1 through 32"
        ))
    ));
}

/// A listener that holds every answer until the test lets one go, and says
/// so each time it starts holding one.
struct Held {
    listener: Listener,
    release: Sender<()>,
    holding: Receiver<String>,
}

fn held(busy_once: &'static str) -> Held {
    let (release, released) = channel::<()>();
    let released = Mutex::new(released);
    let (holding_send, holding) = channel();
    let holding_send = Mutex::new(holding_send);
    let busy = Mutex::new(false);
    let listener = Listener::answering(move |body| {
        let text = String::from_utf8_lossy(body).into_owned();
        let mut said_busy = busy.lock().expect("busy flag");
        if text.contains(busy_once) && !*said_busy {
            *said_busy = true;
            return Canned::status(503, "busy").asking("retry-after-ms", "10");
        }
        drop(said_busy);
        let _held = holding_send.lock().expect("sender").send(text);
        // Each token the test sends lets exactly one held answer go.
        let _token = released.lock().expect("release").recv();
        Canned::ok(&generic(body))
    })
    .expect("listener");
    Held {
        listener,
        release,
        holding,
    }
}

/// Let answers go one at a time, and fail if more than `cap` are ever held.
/// Return the most held at once and every request body that was held.
fn drain(held: &Held, cap: usize, finished: &AtomicUsize, runs: usize) -> (usize, Vec<String>) {
    let give_up = Instant::now() + Duration::from_secs(120);
    let (mut holding, mut most, mut seen) = (0, 0, Vec::new());
    loop {
        assert!(Instant::now() < give_up, "the run deadlocked");
        match held.holding.recv_timeout(QUIET) {
            Ok(body) => {
                holding += 1;
                most = most.max(holding);
                assert!(
                    holding <= cap,
                    "{holding} requests in flight over a cap of {cap}"
                );
                seen.push(body);
            }
            Err(RecvTimeoutError::Timeout) if holding > 0 => {
                held.release.send(()).expect("release");
                holding -= 1;
            }
            Err(RecvTimeoutError::Timeout) if finished.load(Ordering::SeqCst) == runs => break,
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => panic!("the listener stopped"),
        }
    }
    (most, seen)
}

#[test]
fn every_live_path_and_every_engine_share_one_cap() {
    in_child("shared_cap_child");
}

#[test]
#[ignore = "runs alone in a child process"]
fn shared_cap_child() {
    if !child() {
        return;
    }
    two_engines_share_the_cap();
    every_command_path_shares_the_cap();
}

/// One bare attempt through `client`, as a later library engine would send.
fn send_once(client: &Client, url: &str, finished: &AtomicUsize) {
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url,
        body: br#"{"model":"local-1","questions":{"q1":{"type":"noul"}}}"#,
        key: &key,
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
    };
    let sent = client.post_observed(&exchange, &Cancel::default(), || ());
    finished.fetch_add(1, Ordering::SeqCst);
    assert!(sent.is_ok(), "{:?}", sent.err());
}

/// Two clients built with different settings still send under one cap.
fn two_engines_share_the_cap() {
    let held = held("never busy");
    let url = held.listener.url();
    let finished = AtomicUsize::new(0);
    let clients = [5, 7].map(|timeout| {
        Client::new(
            Duration::from_secs(timeout),
            false,
            crate::engine::process_width(),
        )
    });
    let (most, _) = thread::scope(|scope| {
        for client in clients.iter().flat_map(|client| [client; 4]) {
            let finished = &finished;
            scope.spawn(move || send_once(client, url, finished));
        }
        drain(&held, 4, &finished, 8)
    });
    assert_eq!(most, 4, "two engines reach the one cap and no more");
}

fn file(home: &Path, name: &str, text: &str) -> String {
    let path = home.join(name);
    fs::write(&path, text).expect("fixture file");
    path.display().to_string()
}

/// Every command path at once, each marked so its requests can be told apart.
fn every_command_path_shares_the_cap() {
    let home = PathBuf::from(env::var_os("HOME").expect("home"));
    let set = file(
        &home,
        "set.json",
        r#"{"version":1,"questions":{"a":{"decide":"first?"},"b":{"decide":"second?"}}}"#,
    );
    let profile = file(
        &home,
        "one.json",
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    );
    let held = held("retried-mark");
    let base = held.listener.base().to_owned();
    let with = |line: &[&str]| -> Vec<String> {
        let mut line: Vec<String> = line.iter().map(|word| (*word).to_owned()).collect();
        line.extend(["--url", &base, "--model", "local-1", "--no-cache"].map(str::to_owned));
        line
    };
    let runs: Vec<(Vec<String>, &str)> = vec![
        (with(&["decide", "Direct?"]), "direct-mark"),
        (with(&["decide", "Direct?"]), "retried-mark"),
        (
            with(&["decide", "Record?", "--lines"]),
            "record-mark 1\nrecord-mark 2\nrecord-mark 3\nrecord-mark 4\nrecord-mark 5\n",
        ),
        (
            with(&["choose", "Which?", "red", "blue", "--lines"]),
            "chosen-mark 1\nchosen-mark 2\n",
        ),
        (
            with(&["annotate", &set, "--lines"]),
            "grouped-mark 1\ngrouped-mark 2\ngrouped-mark 3\n",
        ),
        (
            with(&["annotate", &set, "--profile", &profile]),
            "split-mark",
        ),
        (
            with(&["find", "Which is late?", "--lines"]),
            "found-mark 1\nfound-mark 2\n",
        ),
        (
            with(&["recognize", "--profile", &profile]),
            "Recognized Ada met Acme.",
        ),
        (
            with(&["relate", "linked=person:organization"]),
            r#"[{"name":"RelatedAda","kind":"person"},{"name":"Acme","kind":"organization"},{"name":"Beta","kind":"organization"}]"#,
        ),
    ];
    let finished = AtomicUsize::new(0);
    let (most, seen) = thread::scope(|scope| {
        for (line, input) in &runs {
            let finished = &finished;
            scope.spawn(move || {
                let environment = Environment::read().expect("environment");
                let words: Vec<&str> = line.iter().map(String::as_str).collect();
                let result = dispatch(&words, &environment, Cursor::new(input.as_bytes().to_vec()));
                finished.fetch_add(1, Ordering::SeqCst);
                assert!(result.is_ok(), "{line:?}: {result:?}");
            });
        }
        drain(&held, 4, &finished, runs.len())
    });
    assert_eq!(most, 4, "the paths together reach the one cap and no more");
    let marks = [
        "direct-mark",
        "retried-mark",
        "record-mark",
        "chosen-mark",
        "grouped-mark",
        "split-mark",
        "found-mark",
        "Recognized",
        "RelatedAda",
    ];
    for mark in marks {
        assert!(
            seen.iter().any(|body| body.contains(mark)),
            "{mark} sent nothing"
        );
    }
    assert!(
        seen.iter()
            .filter(|body| body.contains("split-mark"))
            .count()
            == 2,
        "the split set sends one request per question"
    );
}
