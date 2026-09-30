//! The facade's controls, each held against a counted loopback listener.
//!
//! The shared-case runner proves each function's result through the facade
//! under replay. These tests prove what that runner cannot see: which thread
//! sends, what is never sent again, and what is never sent at all.

use conformance_backend::Rendezvous;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::{fs, path::PathBuf};

use conformance_backend::{Backend as Loopback, Canned, Listener};
use nix::sys::pthread::{pthread_kill, pthread_self};
use nix::sys::signal::Signal;

use crate::core::adapters::built_in::DecodeError;
use crate::core::pack::{self, Ask};
use crate::core::{
    AnswerOutcome, Backend, BackendProfile, DEFAULT_MODEL, Evidence, Find, ModelName, Plan,
    Question, QuestionText, RecognizeSpec, RelateSpec,
};
use crate::engine::error::{Error, Kind, TransportKind};
use crate::engine::facade::{Asks, Bound, Engine, Execution, Key, Settings, Storage, relations};
use crate::engine::pipeline::{self, Answered, Asker, Failed, Flow, Packing};
use crate::engine::{Cancel, Deadline};

mod contract_tests;

const TEST_KEY: &str = "sk-facade-test-7f3a";

fn settings(base: &str) -> Settings {
    Settings {
        backend: Backend::resolve(Some(base), None, DEFAULT_MODEL).expect("backend"),
        profile: None,
        timeout: Duration::from_secs(30),
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
        width: None,
        per_minute: None,
        storage: Storage::default(),
        key: Arc::new(|| Ok(Key::of(TEST_KEY))),
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
    relate_limited(engine, base, None, cancel).map(|execution| execution.answered)
}

fn relate_limited(
    engine: &Engine,
    base: &str,
    profile: Option<&BackendProfile>,
    cancel: &Cancel,
) -> Result<Execution, Error> {
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
    engine.relate(prepared, &entities, 0.5, cancel)
}

/// Ask a two-question plan in one request, as the pipeline packs one
/// record's annotate questions.
fn annotate(engine: &Engine, cancel: &Cancel) -> Result<usize, Error> {
    let plan = Plan::authored(
        evidence("Refund the card."),
        ModelName::new("jev-latest").expect("model"),
        vec![decide("Is this a refund?"), decide("Is this urgent?")],
    )
    .expect("plan");
    let mut asks = Asks::default();
    asks.add(engine.backend(), &plan)?;
    let mut answered = 0;
    engine.ask_each(&asks, Bound::WHOLE, cancel, |_, reply| {
        answered += reply.reply.outcomes().len();
        Ok(())
    })?;
    Ok(answered)
}

/// Each text's yes probability, asked one text a request.
struct OneEach<'a>(&'a Engine);

impl Asker for OneEach<'_> {
    type Input = &'static str;
    type Row = Option<f64>;
    type Error = Error;

    fn label(&self, _text: &&'static str) -> usize {
        1
    }

    fn asks(&self, text: &&'static str) -> Result<Vec<Ask>, Error> {
        let model = self.0.backend().model().clone();
        let plan = Plan::authored(evidence(text), model, vec![decide("Is this a refund?")])
            .map_err(|_| Error::Defect("a plan of one question"))?;
        pack::asks(self.0.backend().url(), &plan).map_err(|_| Error::Defect("an unwritable plan"))
    }

    fn row(&self, _text: &'static str, answers: Vec<Answered>) -> Result<Option<f64>, Error> {
        let [answered] = answers.as_slice() else {
            return Err(Error::Defect("one answer"));
        };
        let stored = [answered.answer.as_deref().map_err(DecodeError::cause)];
        let question = [decide("Is this a refund?")];
        match pack::read(&question, &stored, &answered.answered_by)?.as_slice() {
            [AnswerOutcome::Answered(answer)] => Ok(answer.yes()),
            _ => Err(Error::Defect("a failed answer")),
        }
    }
}

/// Ask each text in its own request over the question pipeline and keep
/// the rows it emits in input order, and the failure it stopped at.
fn bulk(
    engine: &Engine,
    texts: &[&'static str],
    cancel: &Cancel,
) -> (Vec<Option<f64>>, Option<Error>) {
    let (mut rows, mut failure) = (Vec::new(), None);
    let packing = Packing {
        inputs: Some(1),
        questions: None,
        sized: true,
        context: false,
        detailed: false,
        continues: false,
    };
    let host = pipeline::eager(texts.to_vec(), |row| match row {
        Ok(row) => {
            rows.push(row);
            Flow::Continue
        }
        Err(Failed::Asker(error) | Failed::Engine { error, .. } | Failed::Stopped(error)) => {
            failure = Some(error);
            Flow::Stop
        }
        Err(Failed::Pack { .. }) => {
            failure = Some(Error::Defect("a question passed a limit"));
            Flow::Stop
        }
    });
    if let Err(error) = engine.ask_all(&OneEach(engine), packing, host, cancel) {
        failure = Some(error);
    }
    (rows, failure)
}

/// One facade call a table row makes.
type Call<'a> = &'a dyn Fn() -> Result<(), Error>;

/// A folder that is removed when the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("thinkthen-facade-{name}-{}", std::process::id()));
        let _absent = fs::remove_dir_all(&path);
        Self(path)
    }

    /// The text of every file under the folder, read lossily.
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
                    // The live store is binary; a key in it is still ASCII.
                    .map(|path| {
                        String::from_utf8_lossy(&fs::read(path).expect("file")).into_owned()
                    }),
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
fn a_host_signal_on_the_calling_thread_never_fails_a_single_send() {
    let handled = Arc::new(AtomicBool::new(false));
    let _handler =
        signal_hook::flag::register(signal_hook::consts::signal::SIGUSR1, Arc::clone(&handled))
            .expect("a no-op host handler");
    let loopback = Loopback::start().expect("loopback");
    let engine = engine(&format!("{}/arm/held/v1", loopback.origin()));
    let (published, caller) = channel();
    // A socket read with a timeout is not restarted after a signal, so a
    // send on the calling thread would fail here.
    let signal = |caller| pthread_kill(caller, Signal::SIGUSR1).expect("signal delivered");

    let (answer, found) = thread::scope(|scope| {
        let call = scope.spawn(|| {
            published.send(pthread_self()).expect("caller published");
            let answer = ask(&engine, "Please refund me.", &Cancel::default());
            let found = engine.find(&find(), &Cancel::default());
            (answer, found)
        });
        let caller = caller.recv().expect("caller");
        assert_eq!(loopback.wait(1), 1, "the judgment is held");
        signal(caller);
        loopback.round();
        assert_eq!(loopback.wait(2), 2, "the find is held");
        signal(caller);
        loopback.release();
        call.join().expect("call")
    });

    assert_eq!(answer.expect("the answer"), Some(0.9));
    assert_eq!(found.expect("the find").answered.requests_sent, 1);
    assert_eq!(loopback.count(), 2);
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
                .recognize(&recognize_spec(), "Ada lives in Paris.", 600_000, &cancel)
                .map(drop)
        }),
        ("relate", &|| relate(&engine, &base, &cancel).map(drop)),
        ("records", &|| {
            bulk(&engine, &["Refund me."], &cancel)
                .1
                .map_or(Ok(()), Err)
        }),
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
                .recognize(&recognize_spec(), "Ada lives in Paris.", 600_000, cancel)
                .map(|_| 1),
            relate_limited(engine, listener.base(), profile, cancel).map(|_| 1),
        ];
        for call in calls {
            let error = call.expect_err(name);
            assert_eq!(error.kind(), kind, "{name}: {error:?}");
        }
        assert_eq!(engine.usage().expect("usage").requests_sent, 0, "{name}");
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
    let first = Arc::new(Rendezvous::new(2));
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

    let (rows, failure) = thread::scope(|scope| {
        let run = scope.spawn(|| bulk(&engine, &["first", "unsure", "broken"], &cancel));
        for _ in 0..2 {
            later.recv().expect("a later item answered first");
        }
        first.wait();
        run.join().expect("batch")
    });

    assert_eq!(rows, [Some(0.9), Some(0.5)], "good outcomes in input order");
    let cause = failure.expect("the broken reply stops the run");
    assert_eq!(cause.kind(), Kind::Backend, "{cause:?}");
    let sent = listener.count();
    let usage = engine.usage().expect("usage");
    assert_eq!((sent, usage.requests_sent), (3, 3));
    assert!(!engine.recording());
    assert_eq!(
        listener.count(),
        sent,
        "reading results and usage sent nothing"
    );
}

#[test]
fn relate_fails_when_no_answer_is_usable_and_keeps_a_partial_result() {
    // Each reply answers the first question it names when `first` is set,
    // and gives every other question a probability out of range.
    let listener = |first: bool| {
        Listener::answering(move |body| Canned::ok(&failing_reply(body, first))).expect("listener")
    };
    let cancel = Cancel::default();

    let failing = listener(false);
    let Err(error) = relate_limited(&engine(failing.base()), failing.base(), None, &cancel) else {
        panic!("no usable answer fails the call")
    };
    assert!(matches!(error, Error::Reply(_)), "{error:?}");
    assert_eq!(error.kind(), Kind::Backend);

    let partial = listener(true);
    let execution = relate_limited(&engine(partial.base()), partial.base(), None, &cancel)
        .expect("a partial result");
    assert_eq!(execution.answered, 1);
    assert_eq!(execution.failed, 1, "the failed answer stays counted");
    assert_eq!(execution.logical.len(), 2);
    assert_eq!(
        execution.edges.len(),
        1,
        "the one good answer draws its edge"
    );
}

/// A reply whose yes/no probabilities are out of range, but for the first
/// when asked.
fn failing_reply(body: &[u8], first: bool) -> String {
    let request: serde_json::Value = serde_json::from_slice(body).expect("request");
    let names = request["questions"].as_object().expect("questions").keys();
    let answers = names
        .enumerate()
        .map(|(place, name)| {
            let yes = if first && place == 0 { 0.9 } else { 1.5 };
            (
                name.clone(),
                serde_json::json!({"type": "noul", "noul": yes}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    serde_json::json!({"model": "jev-latest", "answers": answers}).to_string()
}

#[test]
fn a_model_copy_keeps_the_rate_a_forked_child_may_rebuild_from() {
    let rate = std::num::NonZeroU32::new(600);
    let mut paced = settings("http://127.0.0.1:9/v1");
    paced.backend = paced.backend.with_per_minute(rate);
    let engine = Engine::new(paced).expect("engine");
    let copy = engine
        .with_model(ModelName::new("other-1").expect("a model"))
        .expect("a copy");
    assert_eq!(copy.backend().per_minute(), rate);
}
