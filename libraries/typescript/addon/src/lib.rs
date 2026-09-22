//! The Node-API door over the `thinkthen` contract, run today by the
//! stand-in engine. The wrapper (`index.js`) shapes arguments and raises
//! failures as the host's own error class; this file converts, calls one
//! engine function on a worker thread, and converts back. No rule, no
//! retry, and no sending lives here.
//!
//! The twelve calls cross through one `call` door with a small op name and a
//! JSON payload, because the repeated parts across surfaces belong to the
//! wrapper and the contract, not to nine copies of Node-API scaffolding.
//! Failures come back as data in a `{ err: { kind, retryable, message } }`
//! envelope, because a Node-API error object cannot carry the kind and the
//! retry signal as fields; the wrapper raises them as `ThinkThenError`.
//!
//! Every call runs as a Node-API async task: `compute` blocks a libuv
//! worker thread, never the JavaScript thread, and the promise resolves on
//! the event loop. A bulk call blocks one worker while the engine's own
//! scoped threads run at the process width, which is the ADR's Node shape.

use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use napi::bindgen_prelude::AsyncTask;
use napi::{Env, JsString, Task};
use napi_derive::napi;
use thinkthen_contract as tt;
use thinkthen_standin::StandinConnector;

/// The one engine the door holds, built through the contract's connector:
/// pointing a surface at the real engine's connector is the one line the
/// merge changes.
static ENGINE: OnceLock<Arc<dyn tt::Engine>> = OnceLock::new();

fn engine() -> &'static Arc<dyn tt::Engine> {
    ENGINE.get_or_init(|| {
        let connector = StandinConnector;
        tt::Connector::connect(&connector, &tt::EngineConfig::from_env())
            .expect("the stand-in connector always connects")
    })
}

/// Run a call so a panic never crosses into the host process: the defect
/// kind rides back as data in the failure envelope.
fn guarded<T>(call: impl FnOnce() -> Result<T, tt::Error>) -> Result<T, tt::Error> {
    panic::catch_unwind(AssertUnwindSafe(call))
        .unwrap_or_else(|_| Err(tt::Error::defect("the engine panicked inside the Node shim")))
}

/// The call's deadline instant, through the contract's one checked
/// conversion: a NaN, a negative other than the `NO_DEADLINE` sentinel, or
/// an oversized budget comes back as the usage kind instead of the panic
/// the unchecked conversion raised inside the host process.
fn deadline_of(deadline_ms: Option<f64>) -> Result<Option<Instant>, tt::Error> {
    let budget = deadline_ms.map(tt::deadline_from_millis).transpose()?.flatten();
    Ok(budget.map(|held| Instant::now() + held))
}

/// A cancel token the wrapper holds. The same token rides into the task,
/// so an `AbortSignal` listener can stop a running batch from JavaScript.
#[napi]
pub struct CancelHandle {
    token: tt::Cancel,
}

#[napi]
impl CancelHandle {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self { token: tt::Cancel::new() }
    }

    /// Ask every wait that sees this token to stop.
    #[napi]
    pub fn cancel(&self) {
        self.token.cancel();
    }

    /// Whether `cancel` has been called.
    #[napi(getter)]
    pub fn cancelled(&self) -> bool {
        self.token.is_cancelled()
    }
}

/// The counters since the last reset, as the contract's JSON.
#[napi]
pub fn usage() -> String {
    let held = panic::catch_unwind(|| engine().usage()).unwrap_or_default();
    serde_json::to_string(&held).expect("the counters serialize")
}

/// The verbs the door carries. The wrapper names them; the contract owns
/// their rules.
#[derive(Clone, Copy)]
enum Op {
    Decide,
    DecideMany,
    Choose,
    Score,
    Tag,
    Filter,
    Rank,
    Find,
    Annotate,
    Details,
    Recognize,
    Relate,
}

impl Op {
    fn parse(name: &str) -> napi::Result<Self> {
        Ok(match name {
            "decide" => Self::Decide,
            "decide_many" => Self::DecideMany,
            "choose" => Self::Choose,
            "score" => Self::Score,
            "tag" => Self::Tag,
            "filter" => Self::Filter,
            "rank" => Self::Rank,
            "find" => Self::Find,
            "annotate" => Self::Annotate,
            "details" => Self::Details,
            "recognize" => Self::Recognize,
            "relate" => Self::Relate,
            other => {
                return Err(napi::Error::new(
                    napi::Status::InvalidArg,
                    format!("the door knows no op {other}"),
                ));
            }
        })
    }
}

/// One engine call, run on a libuv worker thread.
pub struct CallTask {
    op: Op,
    /// The question's file-grammar JSON, or the set's path or JSON for
    /// `annotate`.
    spec: Option<String>,
    /// One evidence text, or a JSON array of records.
    payload: String,
    token: Option<tt::Cancel>,
    deadline: Option<Instant>,
    /// A failure the door itself found (a refused budget), carried back in
    /// the same envelope the engine's failures use.
    door_error: Option<tt::Error>,
}

fn records(payload: &str) -> Result<Vec<String>, tt::Error> {
    serde_json::from_str::<Vec<String>>(payload)
        .map_err(|error| tt::Error::usage(format!("the records are not a JSON array of strings: {error}")))
}

/// The engine takes borrowed records, so the owned strings the door parsed
/// lend their text for the call's length.
fn borrowed(records: &[String]) -> Vec<&str> {
    records.iter().map(String::as_str).collect()
}

fn question(spec: &Option<String>) -> Result<tt::Question, tt::Error> {
    let text = spec
        .as_deref()
        .ok_or_else(|| tt::Error::usage("the call names no question"))?;
    tt::Question::from_json(text)
}

fn set(spec: &Option<String>) -> Result<tt::QuestionSet, tt::Error> {
    let text = spec
        .as_deref()
        .ok_or_else(|| tt::Error::usage("the call names no question set"))?;
    if text.trim_start().starts_with('{') {
        tt::QuestionSet::from_json(text)
    } else {
        tt::QuestionSet::from_file(std::path::Path::new(text))
    }
}

fn recognize_spec(spec: &Option<String>) -> Result<tt::Recognize, tt::Error> {
    let text = spec
        .as_deref()
        .ok_or_else(|| tt::Error::usage("the call names no recognize spec"))?;
    tt::Recognize::from_json(text)
}

fn relate_spec(spec: &Option<String>) -> Result<tt::Relate, tt::Error> {
    let text = spec
        .as_deref()
        .ok_or_else(|| tt::Error::usage("the call names no relate spec"))?;
    tt::Relate::from_json(text)
}

impl CallTask {
    fn run(&self, engine: &dyn tt::Engine, options: tt::Options<'_>) -> Result<serde_json::Value, tt::Error> {
        match self.op {
            Op::Decide => {
                let question = question(&self.spec)?;
                let answer = engine.decide_opts(&question, &self.payload, options)?;
                serde_json::to_value(answer).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::DecideMany => {
                let question = question(&self.spec)?;
                let records = records(&self.payload)?;
                let held = borrowed(&records);
                let judgments = engine.decide_many_opts(&question, &held, options, None)?;
                serde_json::to_value(&judgments).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Choose => {
                let question = question(&self.spec)?;
                let pick = engine.choose_opts(&question, &self.payload, options)?;
                serde_json::to_value(pick).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Score => {
                let question = question(&self.spec)?;
                let scored = engine.score_opts(&question, &self.payload, options)?;
                serde_json::to_value(scored).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Tag => {
                let question = question(&self.spec)?;
                let held = engine.tag_opts(&question, &self.payload, options)?;
                serde_json::to_value(held).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Filter => {
                let question = question(&self.spec)?;
                let records = records(&self.payload)?;
                let held = borrowed(&records);
                let kept = engine.filter_opts(&question, &held, options, None)?;
                serde_json::to_value(kept).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Rank => {
                let question = question(&self.spec)?;
                let records = records(&self.payload)?;
                let held = borrowed(&records);
                let ranked = engine.rank_opts(&question, &held, options, None)?;
                serde_json::to_value(&ranked).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Find => {
                let question = question(&self.spec)?;
                let units = records(&self.payload)?;
                let held = borrowed(&units);
                let found = engine.find_opts(&question, &held, options)?;
                serde_json::to_value(found).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Annotate => {
                let set = set(&self.spec)?;
                let records = records(&self.payload)?;
                let held = borrowed(&records);
                let annotated = engine.annotate_opts(&set, &held, options, None)?;
                serde_json::to_value(&annotated).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Details => {
                let question = question(&self.spec)?;
                let details = engine.details_opts(&question, &self.payload, options)?;
                serde_json::to_value(details).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Recognize => {
                let ask = recognize_spec(&self.spec)?;
                let found = engine.recognize_opts(&ask, &self.payload, options)?;
                serde_json::to_value(found).map_err(|error| tt::Error::defect(error.to_string()))
            }
            Op::Relate => {
                let ask = relate_spec(&self.spec)?;
                let records = records(&self.payload)?;
                let held = borrowed(&records);
                // `relate_checked` carries the 255-record limit at the
                // contract's own door, so this surface inherits it.
                let edges = tt::relate_checked(engine, &ask, &held, options)?;
                serde_json::to_value(&edges).map_err(|error| tt::Error::defect(error.to_string()))
            }
        }
    }
}

impl Task for CallTask {
    type Output = String;
    type JsValue = JsString;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        if let Some(error) = self.door_error.take() {
            return Ok(failure_envelope(error));
        }
        let mut options = tt::Options::new();
        if let Some(token) = &self.token {
            options = options.cancel(token);
        }
        if let Some(at) = self.deadline {
            options = options.deadline(at);
        }
        let held = engine();
        let run = guarded(|| self.run(held.as_ref(), options));
        Ok(match run {
            Ok(value) => serde_json::json!({ "ok": value }).to_string(),
            Err(error) => failure_envelope(error),
        })
    }

    fn resolve(&mut self, env: Env, output: String) -> napi::Result<JsString> {
        env.create_string(&output)
    }
}

/// A contract failure as the envelope the wrapper raises: the kind and the
/// retry signal ride as data, because a Node-API error object cannot carry
/// them as fields.
fn failure_envelope(error: tt::Error) -> String {
    serde_json::json!({
        "err": {
            "kind": error.kind,
            "retryable": error.retryable,
            "message": error.message,
        }
    })
    .to_string()
}

/// One call on a worker thread. `op` names the verb; `spec` is the
/// question's file-grammar JSON (or, for `annotate`, a set's path or
/// JSON); `payload` is one evidence text or a JSON array of records;
/// `cancel` carries the wrapper's `AbortSignal`; `deadlineMs` bounds the
/// whole call. A budget of zero is legal and spent immediately: the
/// contract returns the deadline kind naming the budget. Minus one
/// milliseconds is the no-deadline sentinel; any other negative and every
/// oversized or NaN budget is refused with the usage kind.
#[allow(clippy::needless_pass_by_value)]
#[napi]
pub fn call(
    op: String,
    spec: Option<String>,
    payload: String,
    cancel: Option<&CancelHandle>,
    deadline_ms: Option<f64>,
) -> napi::Result<AsyncTask<CallTask>> {
    let op = Op::parse(&op)?;
    let token = cancel.map(|handle| handle.token.clone());
    let (deadline, door_error) = match deadline_of(deadline_ms) {
        Ok(deadline) => (deadline, None),
        Err(error) => (None, Some(error)),
    };
    Ok(AsyncTask::new(CallTask { op, spec, payload, token, deadline, door_error }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defect_kind_maps_into_the_failure_envelope() {
        let envelope = failure_envelope(tt::Error::defect("the engine broke its own contract"));
        let parsed: serde_json::Value = serde_json::from_str(&envelope).expect("the envelope is JSON");
        assert_eq!(parsed["err"]["kind"], "defect");
        assert_eq!(parsed["err"]["retryable"], false);
        assert!(parsed["err"]["message"]
            .as_str()
            .unwrap()
            .contains("the engine broke its own contract"));
    }

    #[test]
    fn a_panic_inside_the_shim_becomes_the_defect_kind() {
        let held = guarded(|| -> Result<(), tt::Error> { panic!("a shim bug") });
        match held {
            Err(error) => {
                assert_eq!(error.kind, tt::ErrorKind::Defect);
                assert!(!error.retryable);
            }
            Ok(()) => panic!("the panic must not read as a value"),
        }
    }

    #[test]
    fn a_hostile_budget_is_a_usage_error_not_a_panic() {
        for held in [f64::NAN, f64::INFINITY, -5.0, f64::MAX] {
            let failure = match deadline_of(Some(held)) {
                Err(error) => error,
                Ok(_) => panic!("the budget {held} must be refused"),
            };
            assert_eq!(failure.kind, tt::ErrorKind::Usage, "the budget {held} is refused as usage");
        }
        assert!(deadline_of(Some(-1.0)).expect("the sentinel").is_none(), "minus one means no deadline");
        assert!(deadline_of(Some(0.0)).expect("zero is spent, not refused").is_some());
        assert!(deadline_of(None).expect("no budget").is_none());
    }
}
