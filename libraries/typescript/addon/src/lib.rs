//! The Node-API door over the `thinkthen` contract, run today by the
//! stand-in engine. The wrapper (`index.js`) shapes arguments and raises
//! failures as the host's own error class; this file converts, calls one
//! engine function on a worker thread, and converts back. No rule, no
//! retry, and no sending lives here.
//!
//! The ten calls cross through one `call` door with a small op name and a
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

use std::sync::OnceLock;
use std::time::{Duration, Instant};

use napi::bindgen_prelude::AsyncTask;
use napi::{Env, JsString, Task};
use napi_derive::napi;
use thinkthen_contract::Engine;
use thinkthen_contract as tt;
use thinkthen_standin::BlockingEngine;

static ENGINE: OnceLock<BlockingEngine> = OnceLock::new();

fn engine() -> &'static BlockingEngine {
    ENGINE.get_or_init(BlockingEngine::from_env)
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
    serde_json::to_string(&engine().usage()).expect("the counters serialize")
}

/// Reset the counters.
#[napi]
pub fn reset_usage() {
    thinkthen_standin::reset_usage();
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
    Probe,
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
            "probe" => Self::Probe,
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

impl CallTask {
    fn run(&self, engine: &BlockingEngine, options: tt::Options<'_>) -> Result<serde_json::Value, tt::Error> {
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
            Op::Probe => {
                let text = engine.probe(&self.payload)?;
                serde_json::to_value(text).map_err(|error| tt::Error::defect(error.to_string()))
            }
        }
    }
}

impl Task for CallTask {
    type Output = String;
    type JsValue = JsString;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        let mut options = tt::Options::new();
        if let Some(token) = &self.token {
            options = options.cancel(token);
        }
        if let Some(at) = self.deadline {
            options = options.deadline(at);
        }
        let run = self.run(engine(), options);
        Ok(match run {
            Ok(value) => serde_json::json!({ "ok": value }).to_string(),
            Err(error) => serde_json::json!({
                "err": {
                    "kind": error.kind,
                    "retryable": error.retryable,
                    "message": error.message,
                }
            })
            .to_string(),
        })
    }

    fn resolve(&mut self, env: Env, output: String) -> napi::Result<JsString> {
        env.create_string(&output)
    }
}

/// One call on a worker thread. `op` names the verb; `spec` is the
/// question's file-grammar JSON (or, for `annotate`, a set's path or
/// JSON); `payload` is one evidence text or a JSON array of records;
/// `cancel` carries the wrapper's `AbortSignal`; `deadlineSec` bounds the
/// whole call.
#[allow(clippy::needless_pass_by_value)]
#[napi]
pub fn call(
    op: String,
    spec: Option<String>,
    payload: String,
    cancel: Option<&CancelHandle>,
    deadline_sec: Option<f64>,
) -> napi::Result<AsyncTask<CallTask>> {
    let op = Op::parse(&op)?;
    let token = cancel.map(|handle| handle.token.clone());
    let deadline = deadline_sec
        .filter(|seconds| *seconds > 0.0 && seconds.is_finite())
        .map(|seconds| Instant::now() + Duration::from_secs_f64(seconds));
    Ok(AsyncTask::new(CallTask { op, spec, payload, token, deadline }))
}
