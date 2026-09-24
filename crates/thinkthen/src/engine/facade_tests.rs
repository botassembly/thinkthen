//! The facade's controls, each held against a counted loopback listener.
//!
//! The shared-case runner proves each function's result through the facade
//! under replay. These tests prove what that runner cannot see: which thread
//! sends, what is never sent again, what is never sent at all, and the width.

use std::net::TcpListener;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use std::{fs, path::PathBuf};

use conformance_backend::{Backend as Loopback, Canned, Listener};
use nix::sys::pthread::{pthread_kill, pthread_self};
use nix::sys::signal::Signal;

use crate::core::{
    Backend, BackendProfile, Evidence, Find, ModelName, Plan, Question, QuestionSet, QuestionText,
    RecognizeSpec, RelateSpec,
};
use crate::engine::error::{Error, Kind, TransportKind};
use crate::engine::facade::{
    Completed, Engine, Input, InputPort, Key, RunOutcome, Settings, Storage, relations,
};
use crate::engine::{Cancel, Deadline, Width, Widths, workers};

const TEST_KEY: &str = "sk-facade-test-7f3a";

const CASES: &str = include_str!("../../../../conformance/cases.json");

fn settings(base: &str) -> Settings {
    Settings {
        backend: Backend::resolve(Some(base), None, "jev-latest").expect("backend"),
        profile: None,
        timeout: Duration::from_secs(5),
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
        width: None,
        storage: Storage::default(),
        key: || Ok(Key::of(TEST_KEY)),
        usage: Arc::default(),
    }
}

pub(super) fn engine(base: &str) -> Engine {
    Engine::new(settings(base)).expect("engine")
}

/// An engine that retries three times, so a resend would show in the count.
fn retrying(base: &str) -> Engine {
    Engine::new(Settings {
        max_retries: 3,
        ..settings(base)
    })
    .expect("engine")
}

/// An engine whose calls share one gate of this width.
fn narrow(base: &str, width: u64) -> Engine {
    let widths: &'static Widths = Box::leak(Box::default());
    widths
        .select(Some(Width::new(width).expect("width")))
        .expect("a fresh gate");
    engine(base).gated(widths)
}

fn decide(text: &str) -> Question {
    Question::Decide {
        text: QuestionText::new(text).expect("question"),
        yes: None,
        no: None,
    }
}

fn evidence(text: &str) -> Evidence {
    Evidence::new(text).expect("evidence")
}

fn ask(engine: &Engine, text: &str, cancel: &Cancel) -> Result<Option<f64>, Error> {
    engine
        .judge(
            &decide("Does this ask for a refund?"),
            None,
            evidence(text),
            cancel,
        )
        .map(|judged| judged.answer.yes())
}

fn find() -> Find {
    Find::new(
        QuestionText::new("Which passage answers?").expect("question"),
        &[evidence("First passage."), evidence("Second passage.")],
        ModelName::new("jev-latest").expect("model"),
        true,
    )
    .expect("find")
}

fn recognize_spec() -> RecognizeSpec {
    RecognizeSpec::parse(
        r#"{"version":1,"recognize":{"kinds":{"person":"A person's name.","place":"A place name."},"relations":[{"name":"lives_in","source":"person","target":"place"}]}}"#,
    )
    .expect("spec")
}

/// Relate two given alerts, the relation planner's smallest case.
fn relate(engine: &Engine, base: &str, cancel: &Cancel) -> Result<usize, Error> {
    relate_limited(engine, base, None, cancel)
}

fn relate_limited(
    engine: &Engine,
    base: &str,
    profile: Option<&BackendProfile>,
    cancel: &Cancel,
) -> Result<usize, Error> {
    let spec = RelateSpec::parse(
        r#"{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}}"#,
    )
    .expect("spec");
    let entities = spec
        .admit(&[
            ("Checkout fails.".to_owned(), "alert".to_owned()),
            ("The disk is full.".to_owned(), "alert".to_owned()),
        ])
        .expect("entities");
    let backend = Backend::resolve(Some(base), None, "jev-latest").expect("backend");
    let prepared = relations(&entities, &spec, &backend, profile)?;
    engine
        .relate(prepared, &entities, 0.5, cancel)
        .map(|execution| execution.answered)
}

/// Ask a two-question group through the split-request path annotate uses.
fn annotate(engine: &Engine, cancel: &Cancel) -> Result<usize, Error> {
    let plan = Plan::new(
        evidence("Refund the card."),
        ModelName::new("jev-latest").expect("model"),
        vec![decide("Is this a refund?"), decide("Is this urgent?")],
    )
    .expect("plan");
    let mut answered = 0;
    engine.ask_chunks(engine.split(&plan)?, cancel, |reply| {
        answered += reply.reply.outcomes().len();
        Ok::<(), Error>(())
    })?;
    Ok(answered)
}

/// A bulk run's outcome and the rows it emitted, in emission order.
type Bulk = (Result<RunOutcome<Error>, Error>, Vec<Option<f64>>);

/// One facade call a table row makes.
type Call<'a> = &'a dyn Fn() -> Result<(), Error>;

/// Judge each text over the record scheduler; each item asks under `each`.
fn bulk(engine: &Engine, texts: &[&'static str], batch: &Cancel, each: &Cancel) -> Bulk {
    let mut rows = Vec::new();
    let outcome = engine.records(
        false,
        batch,
        reader(texts.to_vec()),
        &|text: &&str| {
            ask(engine, text, each).map(|yes| Completed {
                value: yes,
                replayed: false,
                partial_failure: false,
            })
        },
        |row| {
            rows.push(row);
            Ok(true)
        },
    );
    (outcome, rows)
}

/// Start a reader that hands one item per request, then the end.
fn reader<T: Send + 'static, R: Send + 'static>(
    items: Vec<T>,
) -> impl FnOnce(Receiver<()>, InputPort<T, R, Error>) {
    move |requests, events| {
        thread::spawn(move || feed(items, &requests, &events));
    }
}

fn feed<T, R>(items: Vec<T>, requests: &Receiver<()>, events: &InputPort<T, R, Error>) {
    let mut inputs = items
        .into_iter()
        .map(Input::Item)
        .chain(std::iter::once(Input::End));
    while requests.recv().is_ok() {
        let Some(input) = inputs.next() else { return };
        if events.send(input).is_err() {
            return;
        }
    }
}

/// A folder that is removed when the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("thinkthen-facade-{name}-{}", std::process::id()));
        let _absent = fs::remove_dir_all(&path);
        Self(path)
    }

    /// The text of every file under the folder.
    fn files(&self) -> Vec<String> {
        let mut folders = vec![self.0.clone()];
        let mut files = Vec::new();
        while let Some(folder) = folders.pop() {
            let (inner, plain): (Vec<_>, Vec<_>) = fs::read_dir(folder)
                .into_iter()
                .flatten()
                .flatten()
                .map(|entry| entry.path())
                .partition(|path| path.is_dir());
            folders.extend(inner);
            files.extend(
                plain
                    .into_iter()
                    .map(|path| fs::read_to_string(path).expect("file")),
            );
        }
        files
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _removed = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn a_single_live_call_sends_on_a_worker_through_a_host_signal() {
    let handled = Arc::new(AtomicBool::new(false));
    let _handler =
        signal_hook::flag::register(signal_hook::consts::signal::SIGUSR1, Arc::clone(&handled))
            .expect("a no-op host handler");
    let loopback = Loopback::start().expect("loopback");
    let engine = engine(&format!("{}/arm/held/v1", loopback.origin()));
    let (published, caller) = channel();

    let (answer, found) = thread::scope(|scope| {
        let call = scope.spawn(|| {
            published.send(pthread_self()).expect("caller published");
            let answer = ask(&engine, "Please refund me.", &Cancel::default());
            let found = engine
                .find(&find(), &Cancel::default())
                .map(|found| found.answered);
            (answer, found, thread::current().id())
        });
        let caller = caller.recv().expect("caller");
        assert_eq!(loopback.wait(1), 1, "the single send is held");
        for _ in 0..10 {
            pthread_kill(caller, Signal::SIGUSR1).expect("signal delivered");
        }
        loopback.release();
        let (answer, found, caller) = call.join().expect("call");
        let sends = workers::SENDS.lock().expect("sends").clone();
        let mine = sends
            .iter()
            .filter(|(asked, _)| *asked == caller)
            .collect::<Vec<_>>();
        assert_eq!(mine.len(), 2, "one live attempt for each call");
        assert!(
            mine.iter().all(|(asked, sent)| asked != sent),
            "the calling thread never sends"
        );
        (answer, found)
    });

    assert_eq!(answer.expect("the answer"), Some(0.9));
    assert_eq!(found.expect("the find").requests_sent, 1);
    assert_eq!(loopback.count(), 2);
}

#[test]
fn a_refused_connection_fails_at_once_and_is_not_retryable() {
    let closed = TcpListener::bind("127.0.0.1:0").expect("port");
    let base = format!("http://{}/v1", closed.local_addr().expect("address"));
    drop(closed);
    let engine = retrying(&base);

    let error = ask(&engine, "Refund me.", &Cancel::default()).expect_err("refused");

    assert!(
        matches!(error, Error::Transport(TransportKind::Refused)),
        "{error:?}"
    );
    assert_eq!(error.kind(), Kind::Backend);
    assert!(!error.retryable());
    assert_eq!(engine.usage().requests_sent, 1, "one attempt and no retry");
}

#[test]
fn a_close_after_the_body_is_never_resent_on_any_path() {
    // Each connection serves one scripted reply and closes, so a resend
    // would take the next close and show in the count.
    let listener = Listener::serving((0..24).map(|_| Canned::close_without_reply()).collect())
        .expect("listener");
    let base = listener.base().to_owned();
    let engine = retrying(&base);
    let cancel = Cancel::default();
    let calls: [(&str, Call<'_>); 6] = [
        ("judge", &|| ask(&engine, "Refund me.", &cancel).map(drop)),
        ("find", &|| engine.find(&find(), &cancel).map(drop)),
        ("annotate", &|| annotate(&engine, &cancel).map(drop)),
        ("recognize", &|| {
            engine
                .recognize(&recognize_spec(), "Ada lives in Paris.", &cancel)
                .map(drop)
        }),
        ("relate", &|| relate(&engine, &base, &cancel).map(drop)),
        (
            "records",
            &|| match bulk(&engine, &["Refund me."], &cancel, &cancel).0? {
                RunOutcome::Stopped { cause, .. } => Err(cause),
                RunOutcome::Complete => Ok(()),
            },
        ),
    ];

    // `requests` hands each request over once, so the test keeps the total.
    let mut seen = 0;
    for (sent, (name, call)) in calls.into_iter().enumerate() {
        let error = call().expect_err(name);
        assert!(
            matches!(error, Error::Transport(TransportKind::PrematureClose)),
            "{name}: {error:?}"
        );
        assert_eq!(error.kind(), Kind::Backend, "{name}");
        seen += listener.requests().len();
        assert_eq!(seen, sent + 1, "{name} sent once and never again");
    }
}

#[test]
fn after_a_cancelled_batch_the_next_call_sends_only_its_own_request() {
    let loopback = Loopback::start().expect("loopback");
    let engine = narrow(&format!("{}/arm/held/v1", loopback.origin()), 1);
    let batch = Cancel::default();
    // Each item asks under a token the batch cancel does not reach, so only
    // the scheduler's stop checkpoint keeps a queued item from sending.
    let each = Cancel::default();

    let (outcome, rows) = thread::scope(|scope| {
        let run = scope.spawn(|| bulk(&engine, &["one", "two", "three"], &batch, &each));
        assert_eq!(loopback.wait(1), 1, "the first item is held");
        batch.fire();
        loopback.release();
        run.join().expect("batch")
    });

    assert!(
        matches!(
            outcome,
            Ok(RunOutcome::Stopped {
                cause: Error::Cancelled,
                ..
            })
        ),
        "{outcome:?}"
    );
    assert!(rows.len() <= 1, "{rows:?}");
    assert_eq!(loopback.count(), 1, "no queued item sent after the cancel");
    ask(&engine, "four", &Cancel::default()).expect("the next call");
    assert_eq!(
        loopback.count(),
        2,
        "the next call sent only its own request"
    );
}

#[test]
fn mixed_concurrent_calls_never_pass_the_one_width() {
    let loopback = Loopback::start().expect("loopback");
    let base = format!("{}/arm/held/v1", loopback.origin());
    let engine = narrow(&base, 2);
    let cancel = Cancel::default();

    let answered = thread::scope(|scope| {
        let calls = [
            scope.spawn(|| ask(&engine, "Refund me.", &cancel).map(|_| 1)),
            scope.spawn(|| engine.find(&find(), &cancel).map(|_| 1)),
            scope.spawn(|| annotate(&engine, &cancel)),
            scope.spawn(|| {
                engine
                    .recognize(&recognize_spec(), "Ada lives in Paris.", &cancel)
                    .map(|recognition| recognition.meta.requests.len())
            }),
            scope.spawn(|| relate(&engine, &base, &cancel)),
            scope.spawn(|| match bulk(&engine, &["a", "b", "c"], &cancel, &cancel) {
                (Ok(RunOutcome::Complete), rows) => Ok(rows.len()),
                (outcome, _) => panic!("{outcome:?}"),
            }),
        ];
        assert_eq!(loopback.wait(2), 2, "two attempts hold the width");
        loopback.release();
        calls.map(|call| call.join().expect("call").expect("answered"))
    });

    assert!(answered.iter().all(|count| *count > 0), "{answered:?}");
    assert_eq!(loopback.peak(), 2, "the gate held every call to width 2");
}

#[test]
fn local_refusals_send_nothing_and_store_nothing() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let folder = Scratch::new("refusals");
    let recording = || Settings {
        storage: Storage {
            record: Some(folder.0.clone()),
            ..Storage::default()
        },
        ..settings(listener.base())
    };
    let fired = Cancel::default();
    fired.fire();
    let spent = Cancel::default().with_deadline(Deadline::after(Duration::ZERO));
    let profile = BackendProfile::parse(
        r#"{"schema":"thinkthen.backend-profile/1","name":"edge","max_evidence_bytes":4}"#,
    )
    .expect("profile");
    let limited = Engine::new(Settings {
        profile: Some(profile.clone()),
        ..recording()
    })
    .expect("engine");
    let engine = Engine::new(recording()).expect("engine");
    let unlimited = Cancel::default();
    let table = [
        ("pre-fired cancel", &engine, None, &fired, Kind::Cancelled),
        ("spent deadline", &engine, None, &spent, Kind::Deadline),
        (
            "impossible profile limit",
            &limited,
            Some(&profile),
            &unlimited,
            Kind::Usage,
        ),
    ];

    for (name, engine, profile, cancel, kind) in table {
        let calls: [Result<usize, Error>; 5] = [
            ask(engine, "Refund me.", cancel).map(|_| 1),
            engine.find(&find(), cancel).map(|_| 1),
            annotate(engine, cancel),
            engine
                .recognize(&recognize_spec(), "Ada lives in Paris.", cancel)
                .map(|_| 1),
            relate_limited(engine, listener.base(), profile, cancel),
        ];
        for call in calls {
            let error = call.expect_err(name);
            assert_eq!(error.kind(), kind, "{name}: {error:?}");
        }
        assert_eq!(engine.usage().requests_sent, 0, "{name}");
    }
    assert_eq!(listener.count(), 0, "no refusal reached the listener");
    assert_eq!(
        folder.files(),
        Vec::<String>::new(),
        "no refusal wrote a file"
    );
}

#[test]
fn completed_results_keep_input_order_and_reading_them_sends_nothing() {
    let first = Arc::new(Barrier::new(2));
    let (answered, later) = channel();
    let listener = {
        let first = Arc::clone(&first);
        Listener::answering(move |body| {
            let reply = |yes: f64| {
                Canned::ok(&format!(
                    r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":{yes}}}}}}}"#
                ))
            };
            let text = String::from_utf8_lossy(body);
            if text.contains("first") {
                reply(0.9).after_release(Arc::clone(&first))
            } else if text.contains("unsure") {
                reply(0.5).notifying(answered.clone())
            } else if text.contains("broken") {
                Canned::ok(r#"{"model":"jev-latest","answers":{}}"#).notifying(answered.clone())
            } else {
                reply(0.1).notifying(answered.clone())
            }
        })
        .expect("listener")
    };
    let engine = engine(listener.base());
    let cancel = Cancel::default();

    let (outcome, rows) = thread::scope(|scope| {
        let run = scope.spawn(|| bulk(&engine, &["first", "unsure", "broken"], &cancel, &cancel));
        for _ in 0..2 {
            later.recv().expect("a later item answered first");
        }
        first.wait();
        run.join().expect("batch")
    });

    assert_eq!(rows, [Some(0.9), Some(0.5)], "good outcomes in input order");
    let Ok(RunOutcome::Stopped { at, cause, .. }) = outcome else {
        panic!("{outcome:?}")
    };
    assert_eq!((at, cause.kind()), (3, Kind::Backend), "{cause:?}");
    let sent = listener.count();
    let usage = engine.usage();
    assert_eq!((sent, usage.requests_sent), (3, 3));
    assert!(!engine.recording());
    assert_eq!(
        listener.count(),
        sent,
        "reading results and usage sent nothing"
    );
}

#[test]
fn bulk_and_one_question_annotate_answers_match_the_shared_cases() {
    let document: serde_json::Value = serde_json::from_str(CASES).expect("cases");
    let case = |id: &str| {
        document["cases"]
            .as_array()
            .expect("cases")
            .iter()
            .find(|case| case["id"] == id)
            .expect("case")
            .clone()
    };
    let loopback = Loopback::start().expect("loopback");
    let cancel = Cancel::default();

    let many = case("27-decide-many");
    let bulk_engine = engine(&format!("{}/case/27-decide-many/v1", loopback.origin()));
    let texts = many["exchanges"]
        .as_array()
        .expect("exchanges")
        .iter()
        .map(|exchange| {
            &*exchange["evidence"]
                .as_str()
                .expect("text")
                .to_owned()
                .leak()
        })
        .collect::<Vec<&'static str>>();
    let question = decide(many["question"]["decide"].as_str().expect("question"));
    let mut rows = Vec::new();
    let outcome = bulk_engine.records(
        false,
        &cancel,
        reader(texts),
        &|text: &&str| {
            bulk_engine
                .judge(&question, None, evidence(text), &cancel)
                .map(|judged| Completed {
                    value: judged.answer.yes(),
                    replayed: false,
                    partial_failure: false,
                })
        },
        |row| {
            rows.push(row);
            Ok(true)
        },
    );
    assert!(matches!(outcome, Ok(RunOutcome::Complete)), "{outcome:?}");
    let details = many["expect"]["success"]["answers"]
        .as_array()
        .expect("answers")
        .iter()
        .map(|answer| answer["details"]["answer"]["probability"].as_f64())
        .collect::<Vec<_>>();
    assert_eq!(
        rows, details,
        "each bulk row carries the details probability"
    );

    for id in [
        "37-annotate-choose-one",
        "38-annotate-score-one",
        "39-annotate-tag-one",
    ] {
        let annotate = case(id);
        let set = QuestionSet::parse(&annotate["question_set"].to_string()).expect("set");
        let [named] = set.questions() else {
            panic!("{id} asks one question")
        };
        let engine = engine(&format!("{}/case/{id}/v1", loopback.origin()));
        let text = annotate["exchanges"][0]["evidence"].as_str().expect("text");
        let scalar = engine
            .judge(named.question(), named.threshold(), evidence(text), &cancel)
            .expect("scalar");
        assert_eq!(
            serde_json::to_value(&scalar.value).expect("value"),
            annotate["expect"]["success"]["answers"][0]["bare"],
            "{id}"
        );
    }
}

#[test]
fn only_the_one_accessor_reads_the_retained_state() {
    let sources = [
        include_str!("facade.rs"),
        include_str!("facade/recognize.rs"),
        include_str!("facade/relate.rs"),
    ];
    let reads = sources
        .iter()
        .map(|source| {
            source
                .match_indices("self.state")
                .filter(|(at, _)| !source[at + "self.state".len()..].starts_with('('))
                .count()
        })
        .sum::<usize>();
    assert_eq!(reads, 1, "only `Engine::state` names the state field");
}

#[test]
fn no_key_reaches_a_result_an_error_a_recording_or_a_count() {
    let listener = Listener::serving(vec![
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#),
        Canned::close_without_reply(),
    ])
    .expect("listener");
    let folder = Scratch::new("secrecy");
    let engine = Engine::new(Settings {
        storage: Storage {
            record: Some(folder.0.clone()),
            ..Storage::default()
        },
        ..settings(listener.base())
    })
    .expect("engine");
    let cancel = Cancel::default();

    let judged = engine
        .judge(&decide("Refund?"), None, evidence("Refund me."), &cancel)
        .expect("judged");
    let failed = ask(&engine, "broken", &cancel).expect_err("closed");
    let refused = Engine::new(Settings {
        key: || Err(Error::NoKey("THINKTHEN_API_KEY")),
        ..settings(listener.base())
    })
    .and_then(|engine| ask(&engine, "no key here", &cancel))
    .expect_err("no key");
    let seen = [
        format!("{:?} {:?}", judged.answer, judged.answered.reply),
        format!("{failed:?}"),
        format!("{refused:?}"),
        format!("{:?}", engine.usage()),
        folder.files().concat(),
    ];

    assert!(listener.requests().iter().all(|request| {
        request
            .header("authorization")
            .is_some_and(|value| value.contains(TEST_KEY))
    }));
    for text in seen {
        assert!(!text.contains(TEST_KEY), "{text}");
        assert!(!text.contains("sk-facade"), "{text}");
    }
}
