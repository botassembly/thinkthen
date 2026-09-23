//! The Python surface: `import thinkthen as tt`.
//!
//! One engine under ten surfaces, per ADR 0017. This shim binds
//! `thinkthen-contract` and never the engine beneath it: it converts
//! Python arguments into a contract question, calls one engine function,
//! converts the result, and maps the six error kinds to Python's own
//! exceptions. No rule, no retry, and no sending lives here.
//!
//! The ruled shape, from the ADR and the one-shape page:
//!
//! - the same names everywhere, `tt.` prefixed: `decide`, `choose`,
//!   `score`, `tag`, `filter`, `rank`, `find`, `annotate`, plus
//!   `decide_many` as decide's bulk spelling;
//! - `None` is "not sure";
//! - a list crosses into the engine once and runs at the engine's width;
//! - a failure raises Python's own error and never reads as `False`;
//! - every entry point takes `deadline` in seconds from the moment of the
//!   call, and every raised error carries `retryable`.
//!
//! While a bulk call waits, the engine runs this shim's poll on the
//! calling thread; the poll re-takes the interpreter lock and runs
//! `PyErr_CheckSignals`. When a signal fires the poll sets the cancel
//! token, no new request starts, the sent requests finish, and the call
//! raises `Cancelled` (a subclass of both `KeyboardInterrupt`, so one
//! `except KeyboardInterrupt` catches both paths, and `ThinkThenError`, so
//! one `except ThinkThenError` does too). A signal handler that raises
//! anything else — a `SystemExit`, its own error — has that error raised
//! when the call stops, never `Cancelled`. Worst case is one tick plus one
//! in-flight round, per the experiment 211 proof.
//!
//! A caller can also stop a call from any thread: `tt.CancelToken` is a
//! handle, `token.cancel()` sets it, and every verb takes `token=`. A
//! token given to a bulk call is heard at the engine's ticks; a token
//! given to a column verb is heard between rows. An interrupt never fires
//! the caller's token, so a token shared by several calls is only stopped
//! by its own `cancel`.

mod arrow;
mod generated;

use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use pyo3::create_exception;
use pyo3::exceptions::PyKeyboardInterrupt;
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyAny, PyDict, PyTuple, PyType};

use thinkthen_contract::Annotated;
use thinkthen_contract::Answer;
use thinkthen_contract::Cancel;
use thinkthen_contract::Connector;use thinkthen_contract::Details;
use thinkthen_contract::Edge as ContractEdge;
use thinkthen_contract::EngineConfig;
use thinkthen_contract::Failed;
use thinkthen_contract::Engine;
use thinkthen_contract::Error;
use thinkthen_contract::ErrorKind;
use thinkthen_contract::Kind;
use thinkthen_contract::Options;
use thinkthen_contract::Question as ContractQuestion;
use thinkthen_contract::QuestionSet;
use thinkthen_contract::Recognize;
use thinkthen_contract::Relate;
use thinkthen_contract::RelationRule;
use thinkthen_contract::Scored;
use thinkthen_standin::StandinConnector;

// The exceptions, one a kind. `Cancelled` subclasses `KeyboardInterrupt`,
// the host's own cancel gesture, so both interrupt paths land in one
// `except KeyboardInterrupt` handler; pyo3 gives an exception one base.
create_exception!(
    thinkthen._thinkthen,
    ThinkThenError,
    pyo3::exceptions::PyException,
    "the base error every thinkthen failure raises"
);
create_exception!(
    thinkthen._thinkthen,
    UsageError,
    ThinkThenError,
    "the request was wrong, and nothing was sent"
);
create_exception!(
    thinkthen._thinkthen,
    BackendError,
    ThinkThenError,
    "the wire failed or refused the request"
);
create_exception!(
    thinkthen._thinkthen,
    DeadlineError,
    ThinkThenError,
    "the caller's own budget ran out; it says nothing about the backend's health"
);
create_exception!(
    thinkthen._thinkthen,
    LocalError,
    ThinkThenError,
    "a local file or recording the caller named failed"
);
create_exception!(
    thinkthen._thinkthen,
    DefectError,
    ThinkThenError,
    "the engine broke its own contract"
);

/// The cancel gesture's own error, a subclass of both `KeyboardInterrupt`
/// (the host's gesture) and `ThinkThenError` (this package's base), so
/// either `except` catches it. pyo3's `create_exception!` gives an
/// exception one base, so the class is built here with two.
static CANCELLED: PyOnceLock<Py<PyType>> = PyOnceLock::new();

/// The `Cancelled` class, built once per process.
fn cancelled_type(py: Python<'_>) -> Bound<'_, PyType> {
    CANCELLED
        .get_or_init(py, || {
            let namespace = PyDict::new(py);
            namespace
                .set_item("__doc__", "the wait was cancelled; no new request started and the sent requests finished")
                .expect("a class dict takes the doc");
            namespace
                .set_item("__module__", "thinkthen._thinkthen")
                .expect("a class dict takes the module");
            let bases = PyTuple::new(
                py,
                [
                    py.get_type::<PyKeyboardInterrupt>().into_any(),
                    py.get_type::<ThinkThenError>().into_any(),
                ],
            )
            .expect("a tuple takes two bases");
            py.get_type::<PyType>()
                .call1(("Cancelled", bases, namespace))
                .expect("the class type builds a class")
                .cast_into::<PyType>()
                .expect("the class type builds a class")
                .unbind()
        })
        .bind(py)
        .clone()
}

/// One cancel error of the `Cancelled` class, with the given words.
fn cancelled_error(py: Python<'_>, message: impl Into<String>) -> PyErr {
    PyErr::from_type(cancelled_type(py), message.into())
}

/// The caller's own cancel handle: any thread holding it can stop a call
/// that was handed it, and the call raises `Cancelled` with no new request
/// started. One token may ride several calls; only `cancel` sets it, and
/// an interrupt never fires it.
#[pyclass(frozen)]
struct CancelToken {
    inner: Cancel,
}

#[pymethods]
impl CancelToken {
    #[new]
    fn new() -> Self {
        Self { inner: Cancel::new() }
    }

    /// Ask every call that was handed this token to stop.
    fn cancel(&self) {
        self.inner.cancel();
    }

    /// Whether `cancel` has been called.
    #[getter]
    fn cancelled(&self) -> bool {
        self.inner.is_cancelled()
    }

    fn __repr__(&self) -> String {
        format!("CancelToken(cancelled={})", self.inner.is_cancelled())
    }
}

/// One built question, from `tt.question` or a question file.
#[pyclass(from_py_object)]
#[derive(Clone)]
struct Question {
    inner: ContractQuestion,
}

#[pymethods]
impl Question {
    /// The digest of this question with its threshold, 64 hex figures.
    /// Parts and files give the same digest when they give the same
    /// question.
    fn digest(&self) -> String {
        self.inner.digest()
    }
}

/// One loaded question set, from a file this surface's `annotate` reads.
#[pyclass(from_py_object)]
#[derive(Clone)]
struct QuestionSetHolder {
    inner: QuestionSet,
}

/// The engine this process shares, built through the contract's connector
/// on first use so a host can set its variables after the import. The one
/// line that will name the real engine's connector is the `StandinConnector`
/// name below.
fn engine() -> &'static Arc<dyn Engine> {
    static ENGINE: OnceLock<Arc<dyn Engine>> = OnceLock::new();
    ENGINE.get_or_init(|| {
        StandinConnector
            .connect(&EngineConfig::from_env())
            .expect("the stand-in connector always connects")
    })
}

/// Run one engine step under the contract's shared panic guard: a panic
/// from anywhere beneath the step comes back as the defect kind carrying
/// the panic's own words, so a host hears an exception instead of losing
/// its process to an unwinding engine.
fn guarded<T>(step: impl FnOnce() -> Result<T, Error>) -> Result<T, Error> {
    thinkthen_contract::catch_panic("the Python door", step)
}

/// Bridge a caller's token to the call's own signal token: the engine
/// watches one token, and both gestures — the caller's `token=` and the
/// interpreter's Ctrl-C — must land on it. The bridge polls the caller's
/// token until it fires or the call ends, which `Drop` guarantees by
/// joining the watcher before the options it bridged for go away.
struct TokenBridge {
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl TokenBridge {
    fn watch(caller: Cancel, signal: &Cancel) -> Self {
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let armed = signal.clone();
        let flag = stop.clone();
        let handle = std::thread::spawn(move || {
            while !flag.load(std::sync::atomic::Ordering::SeqCst) {
                if caller.is_cancelled() {
                    armed.cancel();
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        });
        Self { stop, handle: Some(handle) }
    }
}

impl Drop for TokenBridge {
    fn drop(&mut self) {
        self.stop.store(true, std::sync::atomic::Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

/// Run one engine step with the panic guard armed and the interpreter's
/// signals heard: the step runs on a worker thread holding its own cancel
/// token, while the calling thread polls `py.check_signals` — Python's
/// rule that handlers run on the thread that holds the interpreter — and
/// cancels that token the moment a signal arrives, so the engine stops
/// before its next request instead of sending again, and the raise the
/// handler made comes back promptly instead of after the call.
fn step<T: Send + 'static>(
    py: Python<'_>,
    run: impl FnOnce(&Cancel) -> Result<T, Error> + Send + 'static,
) -> Result<T, PyErr> {
    let signal = Cancel::new();
    let armed = signal.clone();
    let mut worker = Some(std::thread::spawn(move || guarded(move || run(&armed))));
    let outcome = loop {
        if let Err(raised) = py.check_signals() {
            // Cancel first, then answer the raise without joining: the
            // promise is exact — no new request starts, and a request
            // already sent finishes in its own time on the worker. The
            // closure owns its captures, so the thread outliving this
            // step is safe, and joining here would hold the caller's
            // interrupt hostage to the in-flight send.
            signal.cancel();
            drop(worker.take());
            break Err(raised);
        }
        match &worker {
            Some(handle) if !handle.is_finished() => {}
            _ => break Ok(()),
        }
        // The poll nap releases the interpreter so other threads breathe.
        py.detach(|| std::thread::sleep(std::time::Duration::from_millis(20)));
    };
    match outcome {
        Err(raised) => Err(raised),
        Ok(()) => match worker.take().expect("the worker is joined once").join() {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => Err(python_error(py, error)),
            Err(_) => Err(python_error(py, Error::defect("the engine step did not come back"))),
        },
    }
}

/// The options a call takes: a deadline from a budget in seconds,
/// converted by the contract's one checked door, the caller's cancel
/// token bridged onto the call's signal token, and the signal token the
/// interpreter's own poll arms.
///
/// No deadline is spelled `None` (or by omitting `deadline=`), and `-1`
/// is the one spelled sentinel for the same thing — the ruling every
/// surface shares. Every other negative is refused as a usage error, so
/// a budget computed as `end - now` that lands below -1 can never
/// quietly disable the deadline. Zero stays what the contract settled: a
/// spent deadline that sends nothing.
fn call_options<'a>(
    deadline: Option<Seconds>,
    caller: Option<Cancel>,
    signal: &'a Cancel,
) -> Result<(Options<'a>, Option<TokenBridge>), Error> {
    // A token already cancelled before the call spends nothing: the
    // refusal happens here, before any request is built or sent.
    if let Some(held) = &caller {
        if held.is_cancelled() {
            return Err(Error {
                kind: ErrorKind::Cancelled,
                message: "the token was already cancelled; no request was sent".into(),
                retryable: false,
            });
        }
    }
    let options = Options::new()
        .with_deadline_seconds(deadline.map(|held| held.0))?
        .cancel(signal);
    let bridge = caller.map(|held| TokenBridge::watch(held, signal));
    Ok((options, bridge))
}

/// A `deadline=` in seconds, a real number. Python's bool is an int, so
/// `True` would run as one second and `False` as a spent deadline; ADR
/// 0031 refuses a bool (Python's or NumPy's) or any
/// non-number as the usage kind, as Node does.
#[derive(Clone, Copy, Debug)]
struct Seconds(f64);

impl<'a, 'py> FromPyObject<'a, 'py> for Seconds {
    type Error = PyErr;

    fn extract(held: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        // NumPy 1 names its bool `bool_`, NumPy 2 `bool`.
        let named_bool = held.get_type().name().is_ok_and(|name| name == "bool" || name == "bool_");
        match held.extract::<f64>() {
            Ok(seconds) if !named_bool => Ok(Self(seconds)),
            _ => Err(python_error(held.py(), Error::usage(DEADLINE_REFUSAL))),
        }
    }
}

/// The one sentence a non-number deadline is refused with.
const DEADLINE_REFUSAL: &str =
    "deadline is seconds from now, a number; no deadline is spelled None or -1";

/// Map a contract error to the Python exception of its kind, carrying the
/// kind's name and the retry signal on the instance.
fn python_error(py: Python<'_>, error: Error) -> PyErr {
    let Error { kind, message, retryable } = error;
    let raised = match kind {
        ErrorKind::Usage => UsageError::new_err(message),
        ErrorKind::Backend => BackendError::new_err(message),
        ErrorKind::Deadline => DeadlineError::new_err(message),
        ErrorKind::Local => LocalError::new_err(message),
        ErrorKind::Cancelled => cancelled_error(py, message),
        ErrorKind::Defect => DefectError::new_err(message),
    };
    let value = raised.value(py);
    let _ = value.setattr("kind", format!("{kind:?}").to_lowercase());
    let _ = value.setattr("retryable", retryable);
    raised
}

/// A decide question from text alone, under the grammar's default cut.
fn text_question(py: Python<'_>, text: &str) -> PyResult<ContractQuestion> {
    let body = serde_json::json!({ "decide": text });
    ContractQuestion::from_json(&body.to_string()).map_err(|error| python_error(py, error))
}

/// A question argument for the decide family: text alone or a built
/// question.
fn settle_question(py: Python<'_>, object: &Bound<'_, PyAny>) -> PyResult<ContractQuestion> {
    settle_for(py, object, "decide")
}

/// The question door with the verb's own word: a built question of a
/// different kind is refused here as usage, because the engine would
/// otherwise answer it and surface the mismatch as a backend error about
/// the answer's shape (review-4's wrong-question-type finding).
fn settle_for(
    py: Python<'_>,
    object: &Bound<'_, PyAny>,
    verb: &str,
) -> PyResult<ContractQuestion> {
    if let Ok(question) = object.extract::<Question>() {
        use thinkthen_contract::QuestionKind;
        let wanted = match verb {
            "choose" => QuestionKind::Choose,
            "score" => QuestionKind::Score,
            "tag" => QuestionKind::Tag,
            _ => QuestionKind::Decide,
        };
        if question.inner.kind() != wanted {
            return Err(UsageError::new_err(format!(
                "this question is a {} question; it answers {}, and handing it to {} would read the wrong answer shape",
                question.inner.kind(),
                question.inner.kind(),
                verb
            )));
        }
        return Ok(question.inner);
    }
    if let Ok(text) = object.extract::<String>() {
        return text_question(py, &text);
    }
    Err(UsageError::new_err(
        "the question is text or a built question from tt.question",
    ))
}

/// A question argument for a verb with members: text plus this call's
/// members, or a built question.
///
/// A built question carries its members once. Handing over a built
/// question and also naming `options`, `labels`, or `levels` is
/// ambiguous, and the call refuses with a usage error naming both
/// rather than guessing which one wins (the settled rule of the
/// contract's `Question` docs).
fn settle_verb_question(
    py: Python<'_>,
    object: &Bound<'_, PyAny>,
    verb: &str,
    key: &str,
    members: Option<Vec<String>>,
) -> PyResult<ContractQuestion> {
    let asked = settle_for(py, object, verb)?;
    if let Ok(question) = object.extract::<Question>() {
        if members.is_some() {
            return Err(UsageError::new_err(format!(
                "the question is built and the call also names {key}: the question carries its {key} once, so drop one"
            )));
        }
        let _ = question;
        return Ok(asked);
    }
    let text = object.extract::<String>().map_err(|_| {
        UsageError::new_err(format!(
            "a {verb} question is text plus its {key}, or a built question"
        ))
    })?;
    let list = members.ok_or_else(|| {
        UsageError::new_err(format!("a {verb} question needs its {key} beside the text"))
    })?;
    let mut body = serde_json::Map::new();
    body.insert(verb.to_owned(), serde_json::json!(text));
    body.insert(key.to_owned(), serde_json::json!(list));
    let held = serde_json::Value::Object(body).to_string();
    ContractQuestion::from_json(&held).map_err(|error| python_error(py, error))
}

/// A question set argument: a built set or a path to a question file.
fn settle_set(py: Python<'_>, object: &Bound<'_, PyAny>) -> PyResult<QuestionSet> {
    if let Ok(set) = object.extract::<QuestionSetHolder>() {
        return Ok(set.inner);
    }
    if let Ok(path) = object.extract::<String>() {
        QuestionSet::from_file(std::path::Path::new(&path))
            .map_err(|error| python_error(py, error))
    } else {
        Err(UsageError::new_err(
            "the set is a question file's path or a loaded set",
        ))
    }
}

/// The bare value of an answer: `True`, `False`, or `None` when unsure.
fn bare(py: Python<'_>, answer: Answer) -> Py<PyAny> {
    match answer.value() {
        Some(value) => pyo3::types::PyBool::new(py, value).to_owned().unbind().into_any(),
        None => py.None(),
    }
}

/// A plain string or `None`.
fn plain(py: Python<'_>, value: Option<String>) -> Py<PyAny> {
    match value {
        Some(text) => pyo3::types::PyString::new(py, &text).unbind().into_any(),
        None => py.None(),
    }
}

/// Run one bulk verb with the signal poll. `call` receives the armed
/// options and the poll, all inside `detach`, so the interpreter lock is
/// free while the engine works.
///
/// The engine is armed with this shim's own token; the poll bridges a
/// caller's token to it at the engine's ticks, so a shared token's
/// `cancel` stops exactly the calls it was handed, and an interrupt never
/// fires the caller's token. A signal whose handler raises something
/// other than `KeyboardInterrupt` — a `SystemExit`, the handler's own
/// error — is carried out of the poll and raised when the call stops.
fn bulk<T: Send>(
    py: Python<'_>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
    call: impl FnOnce(Options<'_>, Option<&mut dyn FnMut()>) -> Result<T, Error> + Send,
) -> PyResult<T> {
    if let Some(held) = token {
        if held.get().inner.is_cancelled() {
            return Err(cancelled_error(
                py,
                "the token was already cancelled; no request was sent",
            ));
        }
    }
    let flagged = Arc::new(AtomicBool::new(false));
    // The batch spine keeps its own war token and its own poll below: the
    // options carry one clone, the poll cancels the other, and the
    // caller's token is bridged by the poll loop that already watches it.
    let war = Cancel::new();
    let armed_token = war.clone();
    let (armed, _no_bridge) =
        call_options(deadline, None, &armed_token).map_err(|error| python_error(py, error))?;
    let caller = token.map(|held| held.get().inner.clone());
    let escaped: Arc<Mutex<Option<PyErr>>> = Arc::new(Mutex::new(None));
    let result = {
        let flagged = Arc::clone(&flagged);
        let escaped = Arc::clone(&escaped);
        py.detach(move || {
            let mut poll = move || {
                if flagged.load(Ordering::Relaxed) {
                    return;
                }
                if caller.as_ref().is_some_and(Cancel::is_cancelled) {
                    war.cancel();
                    return;
                }
                Python::attach(|py| match py.check_signals() {
                    Ok(()) => {}
                    Err(signal) if signal.is_instance_of::<PyKeyboardInterrupt>(py) => {
                        flagged.store(true, Ordering::Relaxed);
                        war.cancel();
                    }
                    Err(signal) => {
                        if let Ok(mut slot) = escaped.lock() {
                            *slot = Some(signal);
                        }
                        war.cancel();
                    }
                });
            };
            guarded(|| call(armed, Some(&mut poll as &mut dyn FnMut())))
        })
    };
    let pending = escaped.lock().ok().and_then(|mut slot| slot.take());
    match result {
        Ok(value) => match pending {
            Some(signal) => Err(signal),
            None => Ok(value),
        },
        Err(error) => {
            if let Some(signal) = pending {
                return Err(signal);
            }
            if error.kind == ErrorKind::Cancelled && flagged.load(Ordering::Relaxed) {
                let raised = cancelled_error(
                    py,
                    "interrupted: no new request started, and every sent request finished",
                );
                let _ = raised.value(py).setattr("retryable", false);
                Err(raised)
            } else {
                Err(python_error(py, error))
            }
        }
    }
}

/// Build a question from parts, through the one file grammar so parts and
/// files give the same digest.
#[pyfunction(signature = (*, decide = None, choose = None, score = None, tag = None, threshold = None, options = None, labels = None, levels = None, true_ = None, false_ = None, model = None, file = None))]
#[allow(clippy::too_many_arguments)]
fn question(
    py: Python<'_>,
    decide: Option<String>,
    choose: Option<String>,
    score: Option<String>,
    tag: Option<String>,
    threshold: Option<&Bound<'_, PyAny>>,
    options: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    levels: Option<Vec<String>>,
    true_: Option<String>,
    false_: Option<String>,
    model: Option<String>,
    file: Option<String>,
) -> PyResult<Question> {
    // One question, one verb: a call carrying two verb texts would drop
    // the second silently in the chain below, so the pair is refused
    // here (review-4, item 15: `question(decide=..., choose=...)`
    // dropped the choose).
    let verbs: Vec<(&str, bool)> = [
        ("decide", decide.is_some()),
        ("choose", choose.is_some()),
        ("score", score.is_some()),
        ("tag", tag.is_some()),
    ]
    .into_iter()
    .filter(|(_, held)| *held)
    .collect();
    if verbs.len() > 1 {
        return Err(UsageError::new_err(format!(
            "question() takes one verb; '{}' and '{}' are both given, and the second would be dropped",
            verbs[0].0,
            verbs[1].0
        )));
    }
    if let Some(path) = file {
        // A file carries its own whole question set; anything beside it
        // would be silently dropped, so every other argument refuses.
        let stray = [
            ("decide", decide.is_some()),
            ("choose", choose.is_some()),
            ("score", score.is_some()),
            ("tag", tag.is_some()),
            ("threshold", threshold.is_some()),
            ("options", options.is_some()),
            ("labels", labels.is_some()),
            ("levels", levels.is_some()),
            ("true", true_.is_some()),
            ("false", false_.is_some()),
            ("model", model.is_some()),
        ];
        if let Some((name, _)) = stray.iter().find(|(_, held)| *held) {
            return Err(UsageError::new_err(format!(
                "question(file=...) takes nothing beside the file; '{name}' would be dropped"
            )));
        }
        let inner = ContractQuestion::from_file(std::path::Path::new(&path))
            .map_err(|error| python_error(py, error))?;
        return Ok(Question { inner });
    }
    // An argument that does not belong to the chosen verb was silently
    // dropped before; now it refuses by name, because a dropped argument
    // is a misshapen question the caller believed they asked.
    let refuse_stray = |verb: &str, allowed: &[(&str, bool)]| -> PyResult<()> {
        for (name, held) in allowed {
            if *held {
                return Err(UsageError::new_err(format!(
                    "'{name}' does not belong to a {verb} question; it would be dropped"
                )));
            }
        }
        Ok(())
    };
    let mut body = serde_json::Map::new();
    if let Some(text) = decide {
        refuse_stray(
            "decide",
            &[
                ("options", options.is_some()),
                ("labels", labels.is_some()),
                ("levels", levels.is_some()),
            ],
        )?;
        body.insert("decide".into(), serde_json::json!(text));
        if let Some(yes) = true_ {
            body.insert("true".into(), serde_json::json!(yes));
        }
        if let Some(no) = false_ {
            body.insert("false".into(), serde_json::json!(no));
        }
    } else if let Some(text) = choose {
        refuse_stray(
            "choose",
            &[
                ("labels", labels.is_some()),
                ("levels", levels.is_some()),
                ("true", true_.is_some()),
                ("false", false_.is_some()),
            ],
        )?;
        body.insert("choose".into(), serde_json::json!(text));
        body.insert(
            "options".into(),
            serde_json::json!(options.ok_or_else(|| {
                UsageError::new_err("a choose question needs its options beside the text")
            })?),
        );
    } else if let Some(text) = score {
        refuse_stray(
            "score",
            &[
                ("options", options.is_some()),
                ("labels", labels.is_some()),
                ("true", true_.is_some()),
                ("false", false_.is_some()),
            ],
        )?;
        body.insert("score".into(), serde_json::json!(text));
        body.insert(
            "levels".into(),
            serde_json::json!(levels.ok_or_else(|| {
                UsageError::new_err("a score question needs its levels beside the text")
            })?),
        );
    } else if let Some(text) = tag {
        refuse_stray(
            "tag",
            &[
                ("options", options.is_some()),
                ("levels", levels.is_some()),
                ("true", true_.is_some()),
                ("false", false_.is_some()),
            ],
        )?;
        body.insert("tag".into(), serde_json::json!(text));
        body.insert(
            "labels".into(),
            serde_json::json!(labels.ok_or_else(|| {
                UsageError::new_err("a tag question needs its labels beside the text")
            })?),
        );
    } else {
        return Err(UsageError::new_err(
            "question takes exactly one of decide, choose, score, or tag",
        ));
    }
    if let Some(mark) = threshold {
        if let Ok(cut) = mark.extract::<f64>() {
            body.insert("threshold".into(), serde_json::json!(cut));
        } else if let Ok((low, high)) = mark.extract::<(f64, f64)>() {
            body.insert("threshold".into(), serde_json::json!(format!("{low}:{high}")));
        } else {
            return Err(UsageError::new_err(
                "threshold is a cut (a number) or a band (a low, high pair)",
            ));
        }
    }
    if let Some(name) = model {
        body.insert("model".into(), serde_json::json!(name));
    }
    let text = serde_json::Value::Object(body).to_string();
    let inner = ContractQuestion::from_json(&text).map_err(|error| python_error(py, error))?;
    Ok(Question { inner })
}

/// Ask once. `None` is "not sure".
///
/// A Polars column in returns a column out: the column crosses zero-copy
/// through the Arrow stream form, the engine runs the whole column 32
/// wide in Rust, and the answers come back as a boolean column whose
/// `None` rows are "not sure" — the wrapper rebuilds the host's own
/// `Series` from it. A plain string stays one answer.
#[pyfunction(signature = (question, evidence, *, deadline = None, token = None))]
fn decide(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: &Bound<'_, PyAny>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Py<PyAny>> {
    let asked = settle_question(py, question)?;
    if arrow::is_arrow(evidence)? {
        let column = arrow::series_column(evidence)?;
        let references: Vec<&str> = column.texts.clone();
        let judgments = bulk(py, deadline, token, |armed, poll| {
            engine().decide_many_opts(&asked, &references, armed, poll)
        })?;
        drop(column);
        let values: Vec<Option<bool>> =
            judgments.iter().map(|one| one.answer.value()).collect();
        return Ok(Py::new(py, arrow::ArrowSeries::bools("answer", &values))?
            .into_any());
    }
    let evidence: String = evidence.extract().map_err(|_| {
        UsageError::new_err(
            "the evidence is text or a Polars column (an Arrow stream); a frame goes to annotate with on=",
        )
    })?;
    let caller = token.map(|held| held.get().inner.clone());
    let answer = step(py, move |signal| {
        let (options, _bridge) = call_options(deadline, caller.clone(), signal)?;
        engine().decide_opts(&asked, &evidence, options)
    })?;
    Ok(bare(py, answer))
}

/// Ask of every record, once, keeping every judgment in input order.
/// A list or tuple crosses as strings; an object answering
/// `__arrow_c_stream__` (a Polars Series) crosses zero-copy as Arrow.
#[pyfunction(signature = (question, records, *, deadline = None, token = None))]
fn decide_many(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    records: &Bound<'_, PyAny>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Vec<Py<PyAny>>> {
    let asked = settle_question(py, question)?;
    if arrow::is_arrow(records)? {
        let column = arrow::series_column(records)?;
        let references: Vec<&str> = column.texts.clone();
        let judgments = bulk(py, deadline, token, |armed, poll| {
            engine().decide_many_opts(&asked, &references, armed, poll)
        })?;
        drop(column);
        return judgments
            .into_iter()
            .map(|one| Ok(bare(py, one.answer)))
            .collect();
    }
    let records: Vec<String> = records.extract()?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let judgments = bulk(py, deadline, token, |armed, poll| {
        engine().decide_many_opts(&asked, &references, armed, poll)
    })?;
    judgments.into_iter().map(|one| Ok(bare(py, one.answer))).collect()
}

/// Pick the option the evidence fits best. `None` is unresolved.
#[pyfunction(signature = (question, evidence, *, options = None, deadline = None, token = None))]
fn choose(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: String,
    options: Option<Vec<String>>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Py<PyAny>> {
    let asked = settle_verb_question(py, question, "choose", "options", options)?;
    let caller = token.map(|held| held.get().inner.clone());
    let picked = step(py, move |signal| {
        let (options, _bridge) = call_options(deadline, caller.clone(), signal)?;
        engine().choose_opts(&asked, &evidence, options)
    })?;
    Ok(plain(py, picked))
}

/// Place the evidence on the question's levels: the number, 0 to K-1.
///
/// A Polars column in returns a number column out. The stand-in carries
/// no bulk score in the contract, so the column runs one call a record
/// through this shim; the decide column next door takes the batch spine.
/// The public wrapper takes `levels` beside the text, as the deck draws.
#[pyfunction(signature = (question, evidence, *, levels = None, deadline = None, token = None))]
fn score(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: &Bound<'_, PyAny>,
    levels: Option<Vec<String>>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Py<PyAny>> {
    let asked = settle_verb_question(py, question, "score", "levels", levels)?;
    if arrow::is_arrow(evidence)? {
        let column = arrow::series_column(evidence)?;
        let texts: Vec<String> = column.texts.iter().map(|text| (*text).to_owned()).collect();
        drop(column);
        // One deadline for the whole column operation: the budget starts
        // when the call does, and every row's call reads the same instant,
        // so a column cannot outlive the caller's budget row after row.
        // The caller's token is read between rows, so a controller thread
        // stops the column before the next row is sent.
        let caller = token.map(|held| held.get().inner.clone());
        let values = step(py, move |signal| {
            let (options, _bridge) = call_options(deadline, caller.clone(), signal)?;
            let mut values = Vec::with_capacity(texts.len());
            for text in &texts {
                if options.cancel_token().is_some_and(Cancel::is_cancelled) {
                    return Err(Error::cancelled());
                }
                values.push(engine().score_opts(&asked, text, options)?.value);
            }
            Ok::<Vec<f64>, Error>(values)
        })?;
        return Ok(Py::new(py, arrow::ArrowSeries::numbers("score", &values))?.into_any());
    }
    let evidence: String = evidence.extract().map_err(|_| {
        UsageError::new_err(
            "the evidence is text or a Polars column (an Arrow stream); a frame goes to annotate with on=",
        )
    })?;
    let caller = token.map(|held| held.get().inner.clone());
    let value = step(py, move |signal| {
        let (options, _bridge) = call_options(deadline, caller.clone(), signal)?;
        Ok(engine().score_opts(&asked, &evidence, options)?.value)
    })?;
    Ok(pyo3::types::PyFloat::new(py, value).unbind().into_any())
}

/// Name the labels that held, in the question's order.
#[pyfunction(signature = (question, evidence, *, labels = None, deadline = None, token = None))]
fn tag(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: String,
    labels: Option<Vec<String>>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Vec<String>> {
    let asked = settle_verb_question(py, question, "tag", "labels", labels)?;
    let caller = token.map(|held| held.get().inner.clone());
    step(py, move |signal| {
        let (options, _bridge) = call_options(deadline, caller.clone(), signal)?;
        engine().tag_opts(&asked, &evidence, options)
    })
}

/// Keep the records whose evidence reached the mark, in order. The caller
/// gets back its own records.
#[pyfunction(signature = (question, records, *, deadline = None, token = None))]
#[allow(clippy::needless_pass_by_value)]
fn filter(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    records: Vec<String>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Vec<String>> {
    let asked = settle_question(py, question)?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let kept = bulk(py, deadline, token, |armed, poll| {
        engine().filter_opts(&asked, &references, armed, poll)
    })?;
    Ok(kept.into_iter().map(|place| records[place].clone()).collect())
}

/// Order the records most likely yes first, ties in input order.
///
/// The answer is the ruled pair per record — the place in the input and
/// the probability — in ranked order (the contract's `Ranked`; no surface
/// returns the bare unit alone). The wrapper fills the record and slices
/// `top`; the settled shape takes `top` there.
#[pyfunction(signature = (question, records, *, deadline = None, token = None))]
#[allow(clippy::needless_pass_by_value)]
fn rank(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    records: Vec<String>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Vec<(usize, f64)>> {
    let asked = settle_question(py, question)?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let ranked = bulk(py, deadline, token, |armed, poll| {
        engine().rank_opts(&asked, &references, armed, poll)
    })?;
    Ok(ranked.into_iter().map(|one| (one.index, one.probability)).collect())
}

/// Pick the unit that best answers the question. `None` is nothing fits.
///
/// The answer is the ruled pair — the unit's place in the input and the
/// probability — or `None` when nothing fits (case 25's null). The
/// wrapper fills the unit text.
#[pyfunction(signature = (question, units, *, deadline = None, token = None))]
#[allow(clippy::needless_pass_by_value)]
fn find(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    units: Vec<String>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Option<(usize, f64)>> {
    let asked = settle_question(py, question)?;
    let caller = token.map(|held| held.get().inner.clone());
    let found = step(py, move |signal| {
        let references: Vec<&str> = units.iter().map(String::as_str).collect();
        let (options, _bridge) = call_options(deadline, caller.clone(), signal)?;
        engine().find_opts(&asked, &references, options)
    })?;
    Ok(found.index.map(|place| (place, found.probability)))
}

/// The failed-question marker as Python data: the ruled shape
/// `{"failed": {"kind": KIND, "cause": CAUSE}}`, never `None`.
///
/// The words come from the contract's own serialization, so the two
/// spellings cannot drift apart; the conformance case
/// `74-annotate-preserves-good-answers` pins the marker in a row.
fn failed_marker(py: Python<'_>, failed: &Failed) -> PyResult<Py<PyAny>> {
    let held = serde_json::to_value(failed)
        .map_err(|error| UsageError::new_err(format!("the failed marker: {error}")))?;
    let inner = PyDict::new(py);
    inner.set_item("kind", held["kind"].as_str().unwrap_or("backend"))?;
    inner.set_item("cause", held["cause"].as_str().unwrap_or(""))?;
    let outer = PyDict::new(py);
    outer.set_item("failed", inner)?;
    Ok(outer.into_any().unbind())
}

/// One record's fields as a plain dictionary, in the set's name order.
///
/// A field whose logical question failed while its neighbours answered
/// carries the failed marker instead of a bare value, so good answers and
/// the failure ride one row (0054).
fn annotated_row(py: Python<'_>, fields: Vec<(String, Annotated)>) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    for (name, field) in fields {
        let value = match field {
            Annotated::Decision(answer) => bare(py, answer),
            Annotated::Choice(picked) => plain(py, picked),
            Annotated::Score(Scored { value, .. }) => {
                pyo3::types::PyFloat::new(py, value).unbind().into_any()
            }
            Annotated::Tags(held) => held
                .into_pyobject(py)?
                .unbind()
                .into_any(),
            Annotated::Failed(failed) => failed_marker(py, &failed)?,
        };
        dict.set_item(name, value)?;
    }
    Ok(dict.into_any().unbind())
}

/// Ask every question in the set of every record, one dictionary a record
/// in the set's name order. The wrapper in `thinkthen/__init__.py` hands a
/// data frame's column over and attaches the new columns on the way back.
#[pyfunction(signature = (set, records, *, deadline = None, token = None))]
#[allow(clippy::needless_pass_by_value)]
fn annotate_rows(
    py: Python<'_>,
    set: &Bound<'_, PyAny>,
    records: Vec<String>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Vec<Py<PyAny>>> {
    let loaded = settle_set(py, set)?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let rows = bulk(py, deadline, token, |armed, poll| {
        engine().annotate_opts(&loaded, &references, armed, poll)
    })?;
    rows.into_iter().map(|fields| annotated_row(py, fields)).collect()
}

/// Annotate a frame's `on` column: the caller's own columns come back
/// aliased and one new column a question is appended, all through the
/// Arrow stream form. `type(records)(frame)` builds the host's frame.
#[pyfunction(signature = (set, records, on, *, deadline = None, token = None))]
fn annotate_stream(
    py: Python<'_>,
    set: &Bound<'_, PyAny>,
    records: &Bound<'_, PyAny>,
    on: String,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<arrow::ArrowFrame> {
    let loaded = settle_set(py, set)?;
    let frame = arrow::frame_column(records, &on)?;
    let references: Vec<&str> = frame.texts.clone();
    let rows = bulk(py, deadline, token, |armed, poll| {
        engine().annotate_opts(&loaded, &references, armed, poll)
    })?;
    let names: Vec<String> = loaded.names().to_vec();
    let out = arrow::build_frame(frame, &names, &rows)?;
    Ok(arrow::ArrowFrame::new(out))
}

/// One judgment plus the audit trail, with the sends that produced it and
/// the recording digests of the logical requests (0053).
///
/// `requests` is always a list, one element for a one-request result, in
/// construction order; a retry adds no element. `failed_questions` is
/// always present, including zero.
#[pyfunction(signature = (question, evidence, *, deadline = None, token = None))]
fn details(
    py: Python<'_>,
    question: &Bound<'_, PyAny>,
    evidence: String,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Py<PyAny>> {
    let asked = settle_question(py, question)?;
    let caller = token.map(|held| held.get().inner.clone());
    let Details { probability, answer, nearest, model, digest, sends, requests, failed_questions } =
        step(py, move |signal| {
            let (options, _bridge) = call_options(deadline, caller.clone(), signal)?;
            engine().details_opts(&asked, &evidence, options)
        })?;
    let dict = PyDict::new(py);
    dict.set_item("probability", probability)?;
    dict.set_item("answer", bare(py, answer))?;
    // The nearest level's name, a score question's own field (ADR 0017
    // pick 6); `None` on every other verb, so no surface carries a
    // private field for it.
    dict.set_item("nearest", nearest)?;
    dict.set_item("model", model)?;
    dict.set_item("digest", digest)?;
    dict.set_item("sends", sends)?;
    dict.set_item("requests", requests)?;
    dict.set_item("failed_questions", failed_questions)?;
    Ok(dict.into_any().unbind())
}

/// One name `recognize` found: the user's own kind word and where the
/// name sits in the text the user gave. `strength` is the settled field
/// name for the number on a name, computed from several of the model's
/// numbers; the marketing vocabulary page
/// (the product vocabulary page, "The words for
/// numbers") reserves the vendor's own summary word, and `probability`,
/// for reported numbers, so this computed number carries neither word.
#[pyclass(frozen, skip_from_py_object)]
#[derive(Clone)]
struct Entity {
    #[pyo3(get)]
    id: u64,
    #[pyo3(get)]
    text: String,
    #[pyo3(get)]
    kind: String,
    #[pyo3(get)]
    start: u64,
    #[pyo3(get)]
    end: u64,
    #[pyo3(get)]
    strength: f64,
}

#[pymethods]
impl Entity {
    fn __repr__(&self) -> String {
        format!(
            "Entity(id={}, text={:?}, kind={:?}, start={}, end={}, strength={})",
            self.id, self.text, self.kind, self.start, self.end, self.strength
        )
    }
}

/// One relation between two names, by entity id. The ends are spelled
/// `source` and `target` on every surface; the number is the model's own
/// probability for the picked relation.
#[pyclass(frozen, skip_from_py_object)]
#[derive(Clone)]
struct Relation {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    source: u64,
    #[pyo3(get)]
    target: u64,
    #[pyo3(get)]
    probability: f64,
}

#[pymethods]
impl Relation {
    fn __repr__(&self) -> String {
        format!(
            "Relation(name={:?}, source={}, target={}, probability={})",
            self.name, self.source, self.target, self.probability
        )
    }
}

/// What `recognize` returned: the names, and the relations when a rule
/// was given.
#[pyclass(frozen, skip_from_py_object)]
#[derive(Clone)]
struct Recognized {
    entities: Vec<Entity>,
    relations: Vec<Relation>,
}

#[pymethods]
impl Recognized {
    #[getter]
    fn entities(&self) -> Vec<Entity> {
        self.entities.clone()
    }

    #[getter]
    fn relations(&self) -> Vec<Relation> {
        self.relations.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Recognized(entities={}, relations={})",
            self.entities.len(),
            self.relations.len()
        )
    }
}

/// One edge `relate` found, by record numbers counted from 1 in input
/// order. An `either` edge prints once, with the lower number in
/// `source`.
#[pyclass(frozen, skip_from_py_object)]
#[derive(Clone)]
struct Edge {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    source: u64,
    #[pyo3(get)]
    target: u64,
    #[pyo3(get)]
    probability: f64,
    #[pyo3(get)]
    source_kind: Option<String>,
    #[pyo3(get)]
    target_kind: Option<String>,
}

#[pymethods]
impl Edge {
    fn __repr__(&self) -> String {
        format!(
            "Edge(name={:?}, source={}, target={}, probability={})",
            self.name, self.source, self.target, self.probability
        )
    }
}

/// The ruled question-file section for one function: the named section
/// when the file nests one, with the file's top-level thresholds carried
/// into the section when it lacks them, else the whole object. Reading
/// the file is this surface's own; every grammar check is the core's.
fn spec_from_file(path: &str, section: &str) -> Result<String, Error> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| Error::local(format!("{path}: {error}")))?;
    let mut value: serde_json::Value = serde_json::from_str(&text)
        .map_err(|error| Error::usage(format!("the question file {path} is not JSON: {error}")))?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| Error::usage(format!("the question file {path} is not a JSON object")))?;
    let spec = match object.get(section).and_then(serde_json::Value::as_object) {
        Some(section) => {
            let mut spec = section.clone();
            for key in ["threshold", "relation_threshold"] {
                if !spec.contains_key(key) {
                    if let Some(carried) = object.get(key) {
                        spec.insert(key.to_owned(), carried.clone());
                    }
                }
            }
            serde_json::Value::Object(spec)
        }
        None => value,
    };
    Ok(serde_json::to_string(&spec).expect("a JSON value serializes"))
}

/// One end of a relation rule: `"*"` is the any kind, any other string a
/// kind of the user's own.
fn end_kind(end: &str) -> Kind {
    if end == "*" {
        Kind::Any
    } else {
        Kind::named(end)
    }
}

/// The pair a relation value must be: from, then to.
fn ends_pair(name: &str, ends: &Bound<'_, PyAny>) -> PyResult<(Kind, Kind)> {
    let pair: Vec<String> = ends.extract().map_err(|_| {
        UsageError::new_err(format!(
            "the relation {name} takes a pair of ends, from then to; each is a kind or \"*\""
        ))
    })?;
    if pair.len() != 2 {
        return Err(UsageError::new_err(format!(
            "the relation {name} takes a pair of ends, from then to, and {} came",
            pair.len()
        )));
    }
    Ok((end_kind(&pair[0]), end_kind(&pair[1])))
}

/// Build a recognize ask from the ruled keyword shape: kinds as a list or
/// a question file path, relations as a name-to-pair mapping, and the two
/// bars. One parser in the core checks every rule; nothing is checked
/// again here.
fn build_recognize(
    py: Python<'_>,
    kinds: Option<&Bound<'_, PyAny>>,
    relations: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    relation_threshold: Option<f64>,
) -> PyResult<Recognize> {
    let mut ask = match kinds {
        None => Recognize::new(),
        Some(bound) => {
            if let Ok(path) = bound.extract::<String>() {
                let spec = spec_from_file(&path, "recognize").map_err(|error| python_error(py, error))?;
                Recognize::from_json(&spec).map_err(|error| python_error(py, error))?
            } else {
                let names: Vec<String> = bound.extract().map_err(|_| {
                    UsageError::new_err("kinds takes a list of kind names or a question file path")
                })?;
                if names.is_empty() {
                    Recognize::new()
                } else {
                    Recognize::new().kinds(names)
                }
            }
        }
    };
    if let Some(bound) = relations {
        let dict = bound.cast::<PyDict>().map_err(|_| {
            UsageError::new_err(
                "relations takes a dictionary, one entry per rule: the name to a (source, target) pair",
            )
        })?;
        for (name, ends) in dict.iter() {
            let name: String = name
                .extract()
                .map_err(|_| UsageError::new_err("a relation name is a string"))?;
            let (from, to) = ends_pair(&name, &ends)?;
            let rule = RelationRule::new(&name, from, to).map_err(|error| python_error(py, error))?;
            // The core's one rule check, the same one the file grammar
            // runs: a named end outside the asked kinds is a usage error
            // here, so the real engine can never be handed one.
            rule.check_kinds(&ask.kinds).map_err(|error| python_error(py, error))?;
            ask.relations.push(rule);
        }
    }
    if let Some(value) = threshold {
        ask = ask.threshold(value).map_err(|error| python_error(py, error))?;
    }
    if let Some(value) = relation_threshold {
        ask = ask.relation_threshold(value).map_err(|error| python_error(py, error))?;
    }
    Ok(ask)
}

/// Build a relate ask from the ruled keyword shape: rules as bare names,
/// as name-to-pair mappings, or as a question file path; `either` names
/// the both-ways rules; one bar.
fn build_relate(
    py: Python<'_>,
    relations: Option<&Bound<'_, PyAny>>,
    either: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
) -> PyResult<Relate> {
    let mut ask = Relate::new();
    if let Some(bound) = relations {
        if let Ok(path) = bound.extract::<String>() {
            let spec = spec_from_file(&path, "relate").map_err(|error| python_error(py, error))?;
            ask = Relate::from_json(&spec).map_err(|error| python_error(py, error))?;
        } else {
            let entries = bound.cast::<pyo3::types::PyList>().map_err(|_| {
                UsageError::new_err(
                    "relations takes a list of names or name-to-pair mappings, or a question file path",
                )
            })?;
            for entry in entries.iter() {
                if let Ok(name) = entry.extract::<String>() {
                    let rule = RelationRule::new(&name, Kind::Any, Kind::Any)
                        .map_err(|error| python_error(py, error))?;
                    ask.relations.push(rule);
                } else if let Ok(dict) = entry.cast::<PyDict>() {
                    let held = |key: &str| -> PyResult<String> {
                        dict.get_item(key)?
                            .and_then(|value| value.extract::<String>().ok())
                            .ok_or_else(|| {
                                UsageError::new_err(format!(
                                    "a relation mapping needs its {key} as a string"
                                ))
                            })
                    };
                    let name = held("name")?;
                    let source = held("source")?;
                    let target = held("target")?;
                    let either = dict
                        .get_item("either")?
                        .and_then(|value| value.extract::<bool>().ok())
                        .unwrap_or(false);
                    let rule = RelationRule::new(&name, end_kind(&source), end_kind(&target))
                        .map_err(|error| python_error(py, error))?
                        .either(either);
                    ask.relations.push(rule);
                } else {
                    return Err(UsageError::new_err(
                        "a relation entry is a name or a mapping with name, source, target",
                    ));
                }
            }
        }
    }
    if let Some(bound) = either {
        let names: Vec<String> = bound.extract().map_err(|_| {
            UsageError::new_err("either takes a list of relation names")
        })?;
        for name in names {
            let rule = RelationRule::new(&name, Kind::Any, Kind::Any)
                .map_err(|error| python_error(py, error))?
                .either(true);
            ask.relations.push(rule);
        }
    }
    if let Some(value) = threshold {
        ask = ask.threshold(value).map_err(|error| python_error(py, error))?;
    }
    Ok(ask)
}

/// Carry the contract's recognized answer into this surface's records.
fn recognized_record(found: thinkthen_contract::Recognized) -> Recognized {
    Recognized {
        entities: found
            .entities
            .into_iter()
            .map(|entity| Entity {
                id: entity.id,
                text: entity.text,
                kind: entity.kind,
                start: entity.start as u64,
                end: entity.end as u64,
                strength: entity.strength,
            })
            .collect(),
        relations: found
            .relations
            .into_iter()
            .map(|relation| Relation {
                name: relation.name,
                source: relation.source,
                target: relation.target,
                probability: relation.probability,
            })
            .collect(),
    }
}

/// Carry one contract edge into this surface's record.
fn edge_record(edge: ContractEdge) -> Edge {
    Edge {
        name: edge.name,
        source: edge.source,
        target: edge.target,
        probability: edge.probability,
        source_kind: edge.source_kind,
        target_kind: edge.target_kind,
    }
}

/// Find every name in one text and say what kind it is. Relations are the
/// second answer, on when a rule is given. `kinds` is a list of the user's
/// own kind words or a path to a question file; `relations` is a mapping
/// of rule name to a (from, to) pair, each end a kind or the one-character
/// string `"*"`. Offsets count Python string positions, so
/// `text[start:end]` is the name.
#[pyfunction(signature = (text, *, kinds = None, relations = None, threshold = None, relation_threshold = None, deadline = None, token = None))]
#[allow(clippy::too_many_arguments)]
fn recognize(
    py: Python<'_>,
    text: String,
    kinds: Option<&Bound<'_, PyAny>>,
    relations: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    relation_threshold: Option<f64>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Recognized> {
    let asked = build_recognize(py, kinds, relations, threshold, relation_threshold)?;
    let caller = token.map(|held| held.get().inner.clone());
    let found = step(py, move |signal| {
        let (options, _bridge) = call_options(deadline, caller.clone(), signal)?;
        engine().recognize_opts(&asked, &text, options)
    })?;
    Ok(recognized_record(found))
}

/// `recognize` over a frame's `on` column: one row per name, with the
/// source row's number counted from 1, in a long frame this surface
/// builds whole. No relation rules here; ask them of the text form.
#[allow(clippy::too_many_arguments, reason = "the pyfunction signature is the public API shape")]
#[pyfunction(signature = (records, on, *, kinds = None, threshold = None, relation_threshold = None, deadline = None, token = None))]
fn recognize_stream(
    py: Python<'_>,
    records: &Bound<'_, PyAny>,
    on: String,
    kinds: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    relation_threshold: Option<f64>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<arrow::ArrowFrame> {
    let asked = build_recognize(py, kinds, None, threshold, relation_threshold)?;
    let frame = arrow::frame_column(records, &on)?;
    let references: Vec<&str> = frame.texts.clone();
    let rows = bulk(py, deadline, token, |armed, mut poll| {
        let mut row_col: Vec<i64> = Vec::new();
        let mut text_col: Vec<String> = Vec::new();
        let mut kind_col: Vec<String> = Vec::new();
        let mut start_col: Vec<i64> = Vec::new();
        let mut end_col: Vec<i64> = Vec::new();
        let mut strength_col: Vec<f64> = Vec::new();
        for (place, text) in references.iter().enumerate() {
            if let Some(poll) = poll.as_mut() {
                poll();
            }
            let found = engine().recognize_opts(&asked, text, armed)?;
            for entity in found.entities {
                row_col.push(place as i64 + 1);
                text_col.push(entity.text);
                kind_col.push(entity.kind);
                start_col.push(entity.start as i64);
                end_col.push(entity.end as i64);
                strength_col.push(entity.strength);
            }
        }
        Ok((row_col, text_col, kind_col, start_col, end_col, strength_col))
    })?;
    drop(frame);
    let (row_col, text_col, kind_col, start_col, end_col, strength_col) = rows;
    let table = arrow::build_table(&[
        ("row", arrow::TableValue::Counts(row_col)),
        ("text", arrow::TableValue::Texts(text_col)),
        ("kind", arrow::TableValue::Texts(kind_col)),
        ("start", arrow::TableValue::Counts(start_col)),
        ("end", arrow::TableValue::Counts(end_col)),
        ("strength", arrow::TableValue::Numbers(strength_col)),
    ])?;
    Ok(arrow::ArrowFrame::new(table))
}

/// Say how the records relate to each other: one pick-one question per
/// legal pair, every record crossing at once. More than 255 records is a
/// usage error before anything happens. `relations` is a list of names or
/// name-to-pair mappings, or a question file path; `either` names the
/// both-ways rules.
#[pyfunction(signature = (records, *, relations = None, either = None, threshold = None, deadline = None, token = None))]
#[allow(clippy::needless_pass_by_value)]
fn relate(
    py: Python<'_>,
    records: Vec<String>,
    relations: Option<&Bound<'_, PyAny>>,
    either: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<Vec<Edge>> {
    let asked = build_relate(py, relations, either, threshold)?;
    let references: Vec<&str> = records.iter().map(String::as_str).collect();
    let edges = bulk(py, deadline, token, |armed, mut poll| {
        if let Some(poll) = poll.as_mut() {
            poll();
        }
        thinkthen_contract::relate_checked(engine().as_ref(), &asked, &references, armed)
    })?;
    Ok(edges.into_iter().map(edge_record).collect())
}

/// `relate` over a frame's `on` column: a frame of edges, one row per
/// edge, the record numbers counted from 1 in input order.
#[allow(clippy::too_many_arguments, reason = "the pyfunction signature is the public API shape")]
#[pyfunction(signature = (records, on, *, relations = None, either = None, threshold = None, deadline = None, token = None))]
fn relate_stream(
    py: Python<'_>,
    records: &Bound<'_, PyAny>,
    on: String,
    relations: Option<&Bound<'_, PyAny>>,
    either: Option<&Bound<'_, PyAny>>,
    threshold: Option<f64>,
    deadline: Option<Seconds>,
    token: Option<&Bound<'_, CancelToken>>,
) -> PyResult<arrow::ArrowFrame> {
    let asked = build_relate(py, relations, either, threshold)?;
    let frame = arrow::frame_column(records, &on)?;
    let references: Vec<&str> = frame.texts.clone();
    let edges = bulk(py, deadline, token, |armed, mut poll| {
        if let Some(poll) = poll.as_mut() {
            poll();
        }
        thinkthen_contract::relate_checked(engine().as_ref(), &asked, &references, armed)
    })?;
    drop(frame);
    let mut name_col: Vec<String> = Vec::with_capacity(edges.len());
    let mut source_col: Vec<i64> = Vec::with_capacity(edges.len());
    let mut target_col: Vec<i64> = Vec::with_capacity(edges.len());
    let mut probability_col: Vec<f64> = Vec::with_capacity(edges.len());
    for edge in edges {
        name_col.push(edge.name);
        source_col.push(edge.source as i64);
        target_col.push(edge.target as i64);
        probability_col.push(edge.probability);
    }
    let table = arrow::build_table(&[
        ("name", arrow::TableValue::Texts(name_col)),
        ("source", arrow::TableValue::Counts(source_col)),
        ("target", arrow::TableValue::Counts(target_col)),
        ("probability", arrow::TableValue::Numbers(probability_col)),
    ])?;
    Ok(arrow::ArrowFrame::new(table))
}

/// The counters since the last reset. `requests` counts sends, so a
/// retried send shows twice, the same as the bill.
#[pyfunction]
fn usage(py: Python<'_>) -> PyResult<Py<PyAny>> {
    let counted = guarded(|| Ok::<_, Error>(engine().usage()))
        .map_err(|error| python_error(py, error))?;
    let dict = PyDict::new(py);
    let _ = dict.set_item("requests", counted.requests);
    let _ = dict.set_item("cache_answers", counted.cache_answers);
    let _ = dict.set_item("tokens", counted.tokens);
    Ok(dict.into_any().unbind())
}

#[pymodule]
fn _thinkthen(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = module.py();
    module.add_class::<Question>()?;
    module.add_class::<QuestionSetHolder>()?;
    module.add_class::<CancelToken>()?;
    module.add_class::<Entity>()?;
    module.add_class::<Relation>()?;
    module.add_class::<Recognized>()?;
    module.add_class::<Edge>()?;
    module.add_class::<arrow::ArrowFrame>()?;
    module.add_function(wrap_pyfunction!(arrow::_arrow_probe, module)?)?;
    module.add_function(wrap_pyfunction!(arrow::_probe_frame_rebuild, module)?)?;
    module.add_function(wrap_pyfunction!(annotate_stream, module)?)?;
    module.add_function(wrap_pyfunction!(recognize_stream, module)?)?;
    module.add_function(wrap_pyfunction!(relate_stream, module)?)?;
    generated::register(module)?;
    module.add("ThinkThenError", py.get_type::<ThinkThenError>())?;
    module.add("UsageError", py.get_type::<UsageError>())?;
    module.add("BackendError", py.get_type::<BackendError>())?;
    module.add("DeadlineError", py.get_type::<DeadlineError>())?;
    module.add("LocalError", py.get_type::<LocalError>())?;
    module.add("Cancelled", cancelled_type(py))?;
    module.add("DefectError", py.get_type::<DefectError>())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use thinkthen_contract::Error;

    #[test]
    fn the_defect_kind_maps_to_defect_error() {
        Python::initialize();
        Python::attach(|py| {
            let raised = python_error(py, Error::defect("the engine broke its own contract"));
            let name = raised.get_type(py).name().unwrap().to_string();
            assert_eq!(name, "DefectError");
            let value = raised.value(py);
            let kind: String = value.getattr("kind").unwrap().extract().unwrap();
            assert_eq!(kind, "defect");
            assert!(!raised.value(py).getattr("retryable").unwrap().extract::<bool>().unwrap());
        });
    }

    #[test]
    fn a_panicking_step_comes_back_as_the_defect_kind() {
        let caught = guarded(|| -> Result<(), Error> { panic!("the engine broke") });
        let error = caught.expect_err("a panic is an error, not an unwind past this door");
        assert_eq!(error.kind, ErrorKind::Defect);
        assert!(error.message.contains("the engine broke"), "{}", error.message);
        // The message comes from the contract's shared boundary, which
        // names the door the panic crossed; the pre-fix local guard said
        // "the engine panicked" and never named it.
        assert!(
            error.message.contains("the Python door"),
            "the contract's boundary names this door: {}",
            error.message
        );
        assert!(!error.retryable);
    }

    #[test]
    fn the_deadline_conversion_refuses_what_cannot_be_a_budget() {
        // One spelling everywhere (third review, item 21): -1 is the
        // no-deadline sentinel and crosses; NaN, an infinity, every other
        // negative, and an oversized budget are usage errors at this
        // door, never arithmetic that ends the host.
        let signal = Cancel::new();
        for refused in [f64::NAN, f64::INFINITY, -2.0, -0.5, 1e300] {
            let error = match call_options(Some(Seconds(refused)), None, &signal) {
                Ok(_) => panic!("{refused} must be refused"),
                Err(error) => error,
            };
            assert_eq!(error.kind, ErrorKind::Usage, "{refused}");
        }
        // `None` and the -1 sentinel both spell no deadline; zero is
        // a spent one the engine answers with the deadline kind.
        assert!(call_options(None, None, &signal).is_ok());
        assert!(call_options(Some(Seconds(-1.0)), None, &signal).is_ok());
        assert!(call_options(Some(Seconds(0.0)), None, &signal).is_ok());
    }
}
