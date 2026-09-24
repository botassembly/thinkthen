//! Shared cases that cross the command itself: staged verbs, question forms, and counters.

use super::conformance_support::{Case, Counters, QuestionForm, Success};
use super::{asked, same_json};
use crate::args::{Cli, Command};
use crate::core::recording::{Entry, Exchange as Recorded};
use crate::core::{Backend, ModelName, Url};
use crate::edge::Environment;
use crate::engine::error::Error as EngineError;
use crate::engine::http::{Client, Key};
use crate::engine::request::{Transport, ask_profile};
use crate::engine::usage::{self, month_now};
use crate::failure::{Failure, report};
use crate::recorder::Recorder;
use clap::Parser as _;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::io::{Cursor, Read as _, Write as _};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;
use std::{fs, thread};

fn folder(case: &Case) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "thinkthen-conformance-{}-{}",
        std::process::id(),
        case.id
    ));
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("case folder");
    path
}

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
    let result = match &cli.command {
        Some(Command::Decide(held)) => crate::judge::decide(held, &environment, input, &mut output),
        Some(Command::Rank(held)) => crate::judge::rank(held, &environment, input, &mut output),
        Some(Command::Recognize(held)) => {
            crate::cli::recognize::run(held, &environment, input, &mut output)
        }
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
pub(super) fn staged(case: &Case, success: &Success) {
    let folder = folder(case);
    let replay = folder.join("replay");
    fs::create_dir_all(&replay).expect("replay folder");
    let url = Url::new("https://api.typesafe.ai/v1/systemone").expect("canonical URL");
    for exchange in &case.exchanges {
        let recorded = Recorded::new(&url, exchange.request.as_bytes());
        let entry = Entry::of(&recorded, exchange.response.get().as_bytes()).expect("entry");
        let text = entry.written().expect("entry text");
        fs::write(replay.join(recorded.digest().file_name()), text).expect("replay entry");
    }
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
    assert_eq!(printed.meta.requests, held.details.requests, "{}", case.id);
    fs::remove_dir_all(folder).expect("case folder removed");
}

/// Give a rule-breaking question as typed text or as a named file and read the exit.
#[allow(
    clippy::disallowed_types,
    reason = "fixture-only question members become command-line text"
)]
pub(super) fn form(case: &Case, kind: &str) {
    let folder = folder(case);
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
    arguments.push("--no-cache".to_owned());
    let (result, output) = dispatch(&arguments, Vec::new());
    let failure = result.expect_err("a rule-breaking question fails");
    let code = report(&failure, &mut Vec::new());
    let wanted = ExitCode::from(if kind == "usage" { 2 } else { 5 });
    assert_eq!(code, wanted, "{}", case.id);
    assert!(output.is_empty(), "{}", case.id);
    fs::remove_dir_all(folder).expect("case folder removed");
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
    let folder = folder(case);
    let cache = folder.join("cache");
    let totals = folder.join("usage");
    let listener = TcpListener::bind("127.0.0.1:0").expect("loopback listener");
    let address = listener.local_addr().expect("loopback address");
    let exchange = case.exchanges.first().expect("one exchange");
    let request = exchange.request.clone().into_bytes();
    let response = exchange.response.get().as_bytes().to_vec();
    let server = thread::spawn(move || serve_once(listener, request, response));
    let backend = Backend::from_parts(
        Url::new(&format!("http://{address}/v1/systemone")).expect("loopback URL"),
        ModelName::new("jev-latest").expect("model"),
    );
    let recorder = Recorder::of_private(Some(&cache), Some(&cache), false, true).expect("cache");
    let client = Client::new(Duration::from_secs(5), false);
    let process = usage::Counters::new(Some(totals.clone()));
    let before = usage::read(&totals, &month_now())
        .expect("totals before")
        .total;
    let plan = asked(case, 0, exchange).expect("case plan").plan;
    for _ in 0..expected.calls {
        let transport = Transport {
            client: &client,
            max_retries: 0,
            retry_wait: Duration::ZERO,
            usage: &process,
        };
        let cancel = crate::engine::Cancel::default();
        ask_profile::<EngineError>(&backend, &plan, None, &recorder, &cancel, transport, || {
            Ok(Key::of("offline"))
        })
        .expect("counted call");
    }
    server.join().expect("loopback server");
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
    fs::remove_dir_all(folder).expect("case folder removed");
}
