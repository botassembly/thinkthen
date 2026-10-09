//! Shared cases that cross the command itself: staged verbs, question forms, and counters.

use super::conformance_support::{Case, Counters, Document, QuestionForm, Success};
use super::{CASES, asked, same_json};
use crate::args::{Cli, Command};
use crate::core::recording::Exchange as Recorded;
use crate::core::{Backend, DEFAULT_MODEL, ModelName, Url};
use crate::edge::Environment;
use crate::engine::Cancel;
use crate::engine::facade::{Asks, Bound, Engine, Settings, Storage};
use crate::engine::http::Key;
use crate::engine::usage::{self, month_now};
use crate::failure::{Failure, report};
use crate::test_deadline::child::ChildEnvironment as _;
use clap::Parser as _;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::io::{Cursor, ErrorKind, Read as _, Write as _};
use std::net::{TcpListener, ToSocketAddrs as _};
use std::path::{Path, PathBuf};
use std::process::{self, ExitCode, Output};
use std::time::Duration;
use std::{fs, thread};

#[path = "../../../../../conformance/consumer/consumer/tests/public/keys.rs"]
mod shared_keys;

/// A case's temporary folder, removed when the case ends, pass or fail.
pub(super) struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _removed = fs::remove_dir_all(&self.0);
    }
}

fn folder(case: &Case) -> Scratch {
    let path = std::env::temp_dir().join(format!(
        "thinkthen-conformance-{}-{}",
        std::process::id(),
        case.id
    ));
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("case folder");
    Scratch(path)
}

/// The exact diagnostic the command prints for each rule-breaking question case.
const SENTENCES: [(&str, &str); 3] = [
    (
        "29-usage-json-text",
        "thinkthen: --threshold: a single cut is above zero and at most one\n",
    ),
    (
        "30-local-question-file",
        "thinkthen: the question file's `decide`: a question is text, not white space\n",
    ),
    (
        "31-usage-rank-blank-question",
        "thinkthen: a question is text, not white space\n",
    ),
];

fn question_file(case: &Case, folder: &Path) -> String {
    let path = folder.join("question.json");
    let question = case.question.as_ref().expect("case question");
    fs::write(&path, question.get()).expect("question file");
    format!("@{}", path.display())
}

/// Run one parsed command in process with the case's input on standard input.
fn dispatch(arguments: &[String], input: Vec<u8>) -> (Result<ExitCode, Failure>, Vec<u8>) {
    let cli = Cli::try_parse_from(arguments).expect("case command line");
    let environment = Environment::default();
    let input = Cursor::new(input);
    let mut output = Vec::new();
    let admitted = match crate::cli::request::admit(cli.command.as_ref().expect("command")) {
        Ok(admitted) => admitted,
        Err(error) => return (Err(error), output),
    };
    let result = match &cli.command {
        Some(Command::Decide(held)) => crate::judge::decide(
            held,
            &environment,
            admitted.expect("decide request"),
            input,
            &mut output,
        ),
        Some(Command::Rank(held)) => crate::judge::rank(
            held,
            &environment,
            admitted.expect("rank request"),
            input,
            &mut output,
        ),
        Some(Command::Recognize(held)) => crate::cli::recognize::run(
            held,
            &environment,
            crate::cli::request::admit(cli.command.as_ref().expect("command"))
                .expect("native header")
                .expect("recognize request"),
            input,
            &mut output,
        ),
        Some(Command::Relate(held)) => {
            crate::cli::relate::run(held, &environment, input, &mut output)
        }
        _ => panic!("no command runner arm for {arguments:?}"),
    };
    (result, output)
}

#[derive(Deserialize)]
struct Printed {
    value: Box<serde_json::value::RawValue>,
    meta: PrintedMeta,
}

#[derive(Deserialize)]
struct PrintedMeta {
    question_sha256: String,
    model: String,
    requests: Vec<String>,
}

/// Replay a recognize or relate case through the command with `--details`.
/// A case's scratch folder and, inside it, a replay folder that holds every
/// exchange of the case under the canonical address.
pub(super) fn replay(case: &Case) -> (Scratch, PathBuf) {
    let scratch = folder(case);
    let replay = scratch.0.join("replay");
    fs::create_dir_all(&replay).expect("replay folder");
    let url = Url::new("https://api.typesafe.ai/v1/systemone").expect("canonical URL");
    for exchange in &case.exchanges {
        let recorded = Recorded::new(&url, exchange.request.as_bytes());
        // The old entry's five fields, the request and response kept byte
        // for byte, since a question key reads the state's exact JSON.
        let entry = format!(
            r#"{{"schema":"thinkthen.recording/1","adapter":{},"url":{},"request":{},"response":{}}}"#,
            serde_json::to_string(crate::core::adapters::built_in::NAME).expect("adapter"),
            serde_json::to_string(url.as_str()).expect("address"),
            exchange.request,
            exchange.response.get(),
        );
        let name = format!("{}.json", recorded.digest().as_str());
        fs::write(replay.join(name), entry).expect("old entry");
    }
    // The shared cases hold whole exchanges, so `cache convert` turns them
    // into the question store's fixture, as it does a committed old folder.
    crate::engine::store::convert(&replay, false).expect("converted replay");
    for item in fs::read_dir(&replay).expect("replay folder") {
        let path = item.expect("replay item").path();
        if path
            .file_name()
            .is_some_and(|name| name != crate::engine::store::JSONL)
        {
            fs::remove_file(path).expect("old entry removed");
        }
    }
    (scratch, replay)
}

/// Map original exchange digests to independently framed v2 metadata keys.
pub(super) fn metadata_keys(case: &Case, url: &Url) -> BTreeMap<String, Vec<String>> {
    let exchanges = case
        .exchanges
        .iter()
        .map(|exchange| {
            serde_json::json!({
                "request":exchange.request,
                "response":serde_json::from_str::<serde_json::Value>(exchange.response.get()).expect("saved reply")
            })
        })
        .collect::<Vec<_>>();
    shared_keys::fixture_keys(url.as_str(), url.as_str(), &exchanges)
        .expect("independent v2 fixture keys")
        .into_iter()
        .map(|(digest, keys)| (digest, serde_json::from_value(keys).expect("question keys")))
        .collect()
}

pub(super) fn staged(case: &Case, success: &Success) {
    let (scratch, replay) = replay(case);
    let folder = scratch.0.clone();
    let input = match (&case.text, &case.entities) {
        (Some(text), None) => text.clone().into_bytes(),
        (None, Some(entities)) => entities.get().as_bytes().to_vec(),
        _ => panic!("{} has no staged input", case.id),
    };
    let question = question_file(case, &folder);
    let replayed = replay.display().to_string();
    let arguments = [
        "thinkthen",
        case.verb.as_str(),
        question.as_str(),
        "--url",
        "https://api.typesafe.ai/v1",
        "--replay",
        replayed.as_str(),
        "--details",
    ]
    .map(str::to_owned);
    let (result, output) = dispatch(&arguments, input);
    assert_eq!(
        result.expect("staged run"),
        ExitCode::SUCCESS,
        "{}",
        case.id
    );
    let printed: Printed = serde_json::from_slice(&output).expect("one detailed result");
    let [held] = success.answers.as_slice() else {
        panic!("{} has no whole-result answer", case.id);
    };
    assert!(
        same_json(printed.value.get(), held.bare.get()).expect("JSON"),
        "{}",
        case.id
    );
    assert_eq!(
        printed.meta.question_sha256, held.details.question_sha256,
        "{}",
        case.id
    );
    assert_eq!(printed.meta.model, held.details.model, "{}", case.id);
    let url = Url::new("https://api.typesafe.ai/v1/systemone").expect("canonical URL");
    let expected = metadata_keys(case, &url);
    let keys: Vec<String> = case
        .exchanges
        .iter()
        .flat_map(|exchange| {
            let digest = Recorded::new(&url, exchange.request.as_bytes()).digest();
            expected[digest.as_str()].clone()
        })
        .collect();
    assert_eq!(printed.meta.requests, keys, "{}", case.id);
}

/// Names a form child's job: a case id, or `probe` for a valid question.
const CHILD: &str = "THINKTHEN_TEST_FORM_CHILD";

/// Whether a variable could steer the command: any `THINKTHEN_` name but the job's.
fn steers(name: &std::ffi::OsStr) -> bool {
    name.to_str()
        .is_some_and(|name| name.starts_with("THINKTHEN_") && name != CHILD)
}

/// Run one ignored test of this module as a child of the test binary, with
/// an empty environment.
fn test_child(name: &str) -> process::Command {
    let mut command = process::Command::new(std::env::current_exe().expect("test binary"));
    command.clear_environment().args([
        "--ignored",
        "--exact",
        "--nocapture",
        &format!("cli::conformance_tests::command::{name}"),
    ]);
    command
}

/// Run a form child for one job with only its job in its environment.
fn child(job: &str) -> Output {
    let mut command = test_child("form_child");
    command.env(CHILD, job);
    crate::test_deadline::output(&mut command).expect("form child")
}

/// A loopback address that can synchronously prove no connection was queued.
fn quiet() -> (String, TcpListener) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("loopback listener");
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    let url = format!(
        "http://{}/v1",
        listener.local_addr().expect("loopback address")
    );
    (url, listener)
}

fn no_connection(listener: &TcpListener, label: &str) {
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock),
        "{label}: no connection reached the named address"
    );
}

fn say(line: &str) {
    let mut output = std::io::stdout().lock();
    writeln!(output, "form-child {line}").expect("write");
    output.flush().expect("flush");
}

/// Give a rule-breaking question in a child whose environment holds no key.
pub(super) fn form(case: &Case) {
    let output = child(&case.id);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success() && stdout.lines().any(|line| line == "form-child sees []"),
        "{}: {stdout}{}",
        case.id,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
#[ignore = "run as a child by the command runner"]
fn form_child() {
    let Some(job) = std::env::var_os(CHILD) else {
        return;
    };
    let seen = std::env::vars_os()
        .map(|(name, _)| name)
        .filter(|name| steers(name))
        .map(|name| name.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    say(&format!("sees [{}]", seen.join(" ")));
    if job == "probe" {
        probe();
    } else {
        let document: Document = serde_json::from_str(CASES).expect("shared document");
        let case = document
            .cases
            .iter()
            .find(|case| job == case.id.as_str())
            .expect("the named case");
        let kind = &case.expect.error.as_ref().expect("a fault").kind;
        rule_breaking(case, kind);
    }
    assert!(seen.is_empty(), "the child sees {seen:?}");
}

/// Ask a valid question at an address the rules cannot prove is this machine,
/// where a missing key still refuses, and report what went out. The short
/// spelling must resolve to the socket whose pending connections we inspect.
fn probe() {
    let (_, listener) = quiet();
    let bound = listener.local_addr().expect("bound address");
    let resolved = ("127.1", bound.port())
        .to_socket_addrs()
        .expect("the short address resolves")
        .collect::<Vec<_>>();
    assert_eq!(resolved, [bound], "the named address reaches this listener");
    let url = format!("https://127.1:{}/v1", bound.port());
    let arguments = [
        "thinkthen",
        "decide",
        "Does the writer ask for a refund?",
        "--url",
        url.as_str(),
        "--no-cache",
    ]
    .map(str::to_owned);
    let (result, _output) = dispatch(&arguments, b"I want a refund.\n".to_vec());
    let mut diagnostic = Vec::new();
    if let Err(failure) = result {
        let _code = report(&failure, &mut diagnostic);
    }
    no_connection(&listener, "probe");
    say("requests 0");
    say(&format!(
        "says {}",
        String::from_utf8_lossy(&diagnostic).trim_end()
    ));
}

/// The runner, started under a planted key and address, hides both from its child.
// Its probe needs the short spelling `127.1`, which Windows does not resolve.
#[cfg(unix)]
#[test]
fn the_runner_hides_a_key_and_an_address_from_its_children() {
    let (url, listener) = quiet();
    let output = crate::test_deadline::output(
        test_child("form_runner")
            .env("THINKTHEN_API_KEY", "test-key-not-real")
            .env("THINKTHEN_BASE_URL", &url),
    )
    .expect("form runner");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout
        .lines()
        .filter(|line| line.starts_with("form-child "))
        .collect::<Vec<_>>();
    assert_eq!(
        lines,
        [
            "form-child sees []",
            "form-child requests 0",
            "form-child says thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent",
        ]
    );
    assert!(output.status.success(), "{stdout}");
    no_connection(&listener, "form runner");
}

#[test]
#[ignore = "run under a planted key by the guard's test"]
fn form_runner() {
    if std::env::var("THINKTHEN_API_KEY").as_deref() != Ok("test-key-not-real") {
        return;
    }
    let output = child("probe");
    std::io::stdout().write_all(&output.stdout).expect("relay");
    assert!(output.status.success());
}

/// Give a rule-breaking question as typed text or as a named file and read the exit.
#[allow(
    clippy::disallowed_types,
    reason = "fixture-only question members become command-line text"
)]
fn rule_breaking(case: &Case, kind: &str) {
    let (url, listener) = quiet();
    let scratch = folder(case);
    let folder = scratch.0.clone();
    let mut arguments = vec!["thinkthen".to_owned(), case.verb.clone()];
    if case.question_form == Some(QuestionForm::File) {
        arguments.push(question_file(case, &folder));
    } else {
        let raw = case.question.as_ref().expect("case question");
        let members: BTreeMap<String, serde_json::Value> =
            serde_json::from_str(raw.get()).expect("question members");
        for (name, value) in members {
            let text = value
                .as_str()
                .map_or_else(|| value.to_string(), str::to_owned);
            if !["decide", "choose", "tag", "score"].contains(&name.as_str()) {
                arguments.push(format!("--{name}"));
                arguments.push(text);
            } else {
                arguments.insert(2, text);
            }
        }
    }
    arguments.extend(["--url".to_owned(), url, "--no-cache".to_owned()]);
    if case.verb == "rank" {
        arguments.push("--lines".to_owned());
    }
    let evidence = case.evidence.clone().expect("valid evidence");
    let (result, output) = dispatch(&arguments, evidence.into_bytes());
    let failure = result.expect_err("a rule-breaking question fails");
    let mut diagnostic = Vec::new();
    let code = report(&failure, &mut diagnostic);
    let wanted = ExitCode::from(if kind == "usage" { 2 } else { 5 });
    assert_eq!(code, wanted, "{}", case.id);
    let sentence = SENTENCES
        .iter()
        .find_map(|(id, sentence)| (*id == case.id).then_some(*sentence))
        .expect("a pinned sentence");
    assert_eq!(
        String::from_utf8_lossy(&diagnostic),
        sentence,
        "{}",
        case.id
    );
    assert!(output.is_empty(), "{}", case.id);
    no_connection(&listener, &case.id);
}

/// Answer one request on loopback, then refuse any later one.
fn serve_once(listener: TcpListener, request: Vec<u8>, response: Vec<u8>) {
    let (mut stream, _) = listener.accept().expect("one request");
    let mut received = Vec::new();
    let mut buffer = [0_u8; 4096];
    while !received.ends_with(&request) {
        let read = stream.read(&mut buffer).expect("request bytes");
        assert!(read > 0, "the request ended early");
        received.extend_from_slice(buffer.get(..read).expect("read bytes"));
    }
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        response.len()
    );
    stream.write_all(head.as_bytes()).expect("response head");
    stream.write_all(&response).expect("response body");
}

/// Repeat one call through a cache folder and read the process counters around it.
pub(super) fn counters(case: &Case, expected: &Counters) {
    let scratch = folder(case);
    let folder = scratch.0.clone();
    let cache = folder.join("cache");
    let totals = folder.join("usage");
    let listener = TcpListener::bind("127.0.0.1:0").expect("loopback listener");
    let address = listener.local_addr().expect("loopback address");
    let exchange = case.exchanges.first().expect("one exchange");
    let request = exchange.request.clone().into_bytes();
    let response = exchange.response.get().as_bytes().to_vec();
    let server = thread::spawn(move || serve_once(listener, request, response));
    let backend = Backend::from_parts(
        Url::new(format!("http://{address}/v1/systemone")).expect("loopback URL"),
        ModelName::new(DEFAULT_MODEL).expect("model"),
    );
    let process = std::sync::Arc::new(usage::Counters::new(Some(totals.clone())));
    let engine = Engine::new(Settings {
        backend,
        profile: None,
        timeout: Duration::from_secs(30),
        max_retries: 0,
        retry_wait: Duration::ZERO,
        width: None,
        per_minute: None,
        storage: Storage {
            record: Some(cache.clone()),
            replay: Some(cache),
            private_default: true,
            cache_answers: true,
            refresh_cache: false,
        },
        key: std::sync::Arc::new(|| Ok(Key::of("offline"))),
        usage: std::sync::Arc::clone(&process),
    })
    .expect("a caching engine");
    let before = usage::read(&totals, &month_now())
        .expect("totals before")
        .total;
    let plan = asked(case, 0, exchange).expect("case plan").plan;
    let mut asks = Asks::default();
    asks.add(engine.backend(), &plan).expect("case questions");
    for _ in 0..expected.calls {
        engine
            .ask_each(&asks, Bound::WHOLE, &Cancel::default(), |_, _| Ok(()))
            .expect("counted call");
    }
    server.join().expect("loopback server");
    process.finish();
    let after = usage::read(&totals, &month_now())
        .expect("totals after")
        .total;
    let requests = after.requests_sent - before.requests_sent;
    assert_eq!(requests, expected.requests, "{} requests", case.id);
    let cache_answers = after.cache_answers - before.cache_answers;
    assert_eq!(
        cache_answers, expected.cache_answers,
        "{} cache answers",
        case.id
    );
}
