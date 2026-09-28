//! The one door every call passes. It reads the op, the question, and the
//! payload, calls the public engine, and writes one envelope with a value,
//! final facts and ordered question details, or a named failure. It holds no rule of its own
//! beyond the host's deadline spelling.

mod diagnostics;
mod result;

use std::io::Read;
use std::num::NonZeroUsize;
use std::panic::{AssertUnwindSafe, catch_unwind};

use serde_json::{Map, Value, json};
use thinkthen::{
    BatchSetting, CallOptions, CancelToken, Engine, EngineBuilder, Error, ErrorKind, Facts,
    Question, Recognize, Relate,
};

/// The most milliseconds a deadline takes: 4,294,967,295 seconds (ADR 0041).
const MOST_MILLIS: f64 = 4_294_967_295_000.0;

/// A failure as the envelope carries it.
#[derive(Debug)]
pub(crate) struct Failure {
    kind: ErrorKind,
    retryable: bool,
    message: String,
    facts: Option<Facts>,
}

impl Failure {
    fn of(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            retryable: false,
            message: message.into(),
            facts: None,
        }
    }

    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Usage, message)
    }

    pub(crate) fn local(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Local, message)
    }

    fn defect(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Defect, message)
    }

    /// The envelope for this failure. The kind comes from the engine's one table.
    pub(crate) fn envelope(&self) -> String {
        self.envelope_with(&Value::Array(Vec::new()))
    }

    fn envelope_with(&self, details: &Value) -> String {
        let mut error = json!({
            "kind": self.kind.name(),
            "retryable": self.retryable,
            "message": self.message,
        });
        if let Some(facts) = &self.facts {
            result::put(&mut error, "facts", result::facts(facts));
            result::put(&mut error, "details", details.clone());
        }
        json!({ "err": error }).to_string()
    }
}

impl From<Error> for Failure {
    fn from(error: Error) -> Self {
        Self {
            kind: error.kind(),
            retryable: error.retryable(),
            message: error.to_string(),
            facts: error.facts().cloned(),
        }
    }
}

type Answered = Result<String, Failure>;

/// Run one body and write its envelope. The binding's one panic guard turns a
/// panic into `defect`.
pub(crate) fn guarded(body: impl FnOnce() -> Answered) -> String {
    diagnostics::owned(|| match caught(body) {
        Ok(raw) => format!("{{\"ok\":{raw}}}"),
        Err(failure) => failure.envelope(),
    })
}

/// Return the original validated JSON as an envelope for a named question.
pub(crate) fn question_file(path: &str) -> String {
    guarded(|| {
        let source = bounded_file(path, "question")?;
        Question::from_json(&source)
            .map_err(|_| Failure::local("the question file has invalid question content"))?;
        serde_json::to_string(&source)
            .map_err(|_| Failure::defect("the question could not be encoded"))
    })
}

/// Validate one named recognition or relation plan and retain its source.
pub(crate) fn plan_file(path: &str, verb: &str) -> String {
    guarded(|| {
        let source = bounded_file(path, "plan")?;
        match verb {
            "recognize" => {
                Recognize::from_json(&source)
                    .map_err(|_| Failure::local("the plan file has invalid plan content"))?;
            }
            "relate" => {
                Relate::from_json(&source)
                    .map_err(|_| Failure::local("the plan file has invalid plan content"))?;
            }
            _ => return Err(Failure::usage("the plan file needs recognize or relate")),
        }
        serde_json::to_string(&source).map_err(|_| Failure::defect("the plan could not be encoded"))
    })
}

fn bounded_file(path: &str, role: &str) -> Result<String, Failure> {
    const LIMIT: u64 = 1_048_576;
    let unreadable = || Failure::local(format!("the {role} file could not be read"));
    let file = std::fs::File::open(path).map_err(|_| unreadable())?;
    let mut bytes = Vec::new();
    file.take(LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| unreadable())?;
    if bytes.len() as u64 > LIMIT {
        return Err(Failure::local(format!("the {role} file is too large")));
    }
    String::from_utf8(bytes).map_err(|_| Failure::local(format!("the {role} file is not UTF-8")))
}

/// The binding's one panic guard: a panic in `body` becomes `defect`, so no
/// panic crosses into Node.
pub(crate) fn caught<T>(body: impl FnOnce() -> Result<T, Failure>) -> Result<T, Failure> {
    diagnostics::owned(|| match catch_unwind(AssertUnwindSafe(body)) {
        Ok(value) => value,
        Err(payload) => {
            std::mem::forget(payload);
            Err(Failure::of(
                ErrorKind::Defect,
                "defect: the Node binding panicked",
            ))
        }
    })
}

/// One call as it crossed from JavaScript.
#[derive(Debug)]
pub(crate) struct Call {
    pub(crate) op: String,
    pub(crate) spec: Option<String>,
    pub(crate) payload: String,
    pub(crate) deadline_ms: Option<f64>,
    pub(crate) batch: Option<String>,
    pub(crate) context: Option<String>,
}

/// The host's deadline: `None` and `-1` are no deadline, `0` is spent, and
/// any other value must be a whole number of milliseconds in range. The
/// check runs before any cast, so the cast is exact.
pub(crate) fn deadline_of(value: Option<f64>) -> Result<Option<i64>, Failure> {
    match value {
        None => Ok(None),
        Some(-1.0) => Ok(None),
        Some(held)
            if held.is_finite() && held.fract() == 0.0 && (0.0..=MOST_MILLIS).contains(&held) =>
        {
            #[expect(
                clippy::cast_possible_truncation,
                reason = "the value is whole and at most 4294967295000"
            )]
            Ok(Some(held as i64))
        }
        Some(held) => Err(Failure::usage(format!(
            "deadlineMs {} is not a deadline; use -1, null, or a whole number of milliseconds from 0 to 4294967295000",
            js_number(held)
        ))),
    }
}

/// A number as JavaScript's `String()` prints it.
fn js_number(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_owned();
    }
    if value.is_infinite() {
        return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_owned();
    }
    let size = value.abs();
    if size >= 1e21 || (size != 0.0 && size < 1e-6) {
        let written = format!("{value:e}");
        return match written.split_once('e') {
            Some((digits, power)) if !power.starts_with('-') => format!("{digits}e+{power}"),
            _ => written,
        };
    }
    format!("{value}")
}

/// Answer one call on this thread and write its envelope.
pub(crate) fn answer(engine: Option<&Engine>, call: &Call, token: &CancelToken) -> String {
    let observed = result::Observed::default();
    let capture = |event: thinkthen::RecordObservation<'_>| observed.capture(event);
    let answered = caught(|| {
        let deadline = deadline_of(call.deadline_ms)?;
        let engine = match engine {
            Some(engine) => engine,
            None => thinkthen::default_engine()?,
        };
        let mut options = CallOptions::new().cancel(token).observe(&capture);
        if let Some(millis) = deadline {
            options = options.deadline_millis(millis)?;
        }
        if let Some(batch) = &call.batch {
            let value: Value = serde_json::from_str(batch)
                .map_err(|_| Failure::usage("options.batch is max or a positive whole number"))?;
            options = options.batch(batch_of(&value)?);
        }
        if let Some(context) = &call.context {
            options = options.context(context);
        }
        result::run(engine, call, options)
    });
    let details = observed.snapshot();
    match answered {
        Ok(finished) => format!(
            "{{\"ok\":{{\"value\":{},\"facts\":{},\"details\":{details}}}}}",
            finished.value,
            result::facts(&finished.facts),
        ),
        Err(failure) => failure.envelope_with(&details),
    }
}

fn batch_of(value: &Value) -> Result<BatchSetting, Failure> {
    if value == "max" {
        return Ok(BatchSetting::Max);
    }
    let count = value
        .as_u64()
        .and_then(|held| usize::try_from(held).ok())
        .and_then(NonZeroUsize::new)
        .ok_or_else(|| Failure::usage("options.batch is max or a positive whole number"))?;
    Ok(BatchSetting::Records(count))
}

/// Build an engine from `EngineBuilder::from_env()` and the given options.
pub(crate) fn engine(options: &str) -> Result<Engine, Failure> {
    caught(|| built(options))
}

fn built(options: &str) -> Result<Engine, Failure> {
    let given: Map<String, Value> = serde_json::from_str(options)
        .map_err(|_| Failure::usage("new Engine takes one options object"))?;
    let mut builder = EngineBuilder::from_env()?;
    for (key, value) in &given {
        builder = setting(builder, key, value)?;
    }
    Ok(builder.build()?)
}

fn setting(builder: EngineBuilder, key: &str, value: &Value) -> Result<EngineBuilder, Failure> {
    let text = || {
        value
            .as_str()
            .ok_or_else(|| Failure::usage(format!("options.{key} is a string")))
    };
    let whole = || {
        value
            .as_u64()
            .ok_or_else(|| Failure::usage(format!("options.{key} is a whole number")))
    };
    Ok(match (key, value) {
        ("baseUrl", _) => builder.base_url(text()?)?,
        ("model", _) => builder.model(text()?)?,
        ("batch", _) => builder.batch(batch_of(value)?),
        ("throttle", _) => builder.throttle(u8::try_from(whole()?).unwrap_or(u8::MAX))?,
        ("maxRequests", _) => {
            builder.max_requests(Some(usize::try_from(whole()?).unwrap_or(usize::MAX)))?
        }
        ("maxRequestBytes", _) => builder.max_request_bytes(
            usize::try_from(whole()?)
                .map_err(|_| Failure::usage("options.maxRequestBytes is a whole number"))?,
        )?,
        ("cache", Value::Bool(false)) => builder.no_cache(),
        ("cache", Value::String(folder)) => builder.cache_at(folder)?,
        ("cache", _) => return Err(Failure::usage("options.cache is false or a folder path")),
        ("timeoutSeconds", _) => builder.timeout(std::time::Duration::from_secs(whole()?))?,
        ("maxRetries", _) => builder.max_retries(
            u32::try_from(whole()?)
                .map_err(|_| Failure::usage("options.maxRetries is a whole number"))?,
        ),
        ("record", _) => builder.record(text()?)?,
        ("replay", _) => builder.replay(text()?)?,
        ("profile", _) => builder.profile(text()?)?,
        _ => return Err(Failure::usage(format!("new Engine takes no option {key}"))),
    })
}

/// The engine's counters as one JSON object.
pub(crate) fn usage(engine: Option<&Engine>) -> String {
    guarded(|| {
        let counters = match engine {
            Some(engine) => engine.usage(),
            None => thinkthen::usage()?,
        };
        Ok(json!({
            "requests_sent": counters.requests_sent(),
            "retries": counters.retries(),
            "cache_answers": counters.cache_answers(),
            "input_tokens": counters.input_tokens(),
            "output_tokens": counters.output_tokens(),
        })
        .to_string())
    })
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use thinkthen::ErrorKind;

    use super::{Failure, deadline_of, guarded};

    fn kind(envelope: &str) -> Value {
        serde_json::from_str::<Value>(envelope).unwrap()["err"]["kind"].clone()
    }

    /// R1-10: a panic below the door is `defect`, and the next call answers.
    #[test]
    fn a_panic_is_a_defect_and_the_next_call_answers() {
        let planted = guarded(|| panic!("planted"));
        assert_eq!(
            planted,
            r#"{"err":{"kind":"defect","message":"defect: the Node binding panicked","retryable":false}}"#
        );
        assert_eq!(guarded(|| Ok("7".to_owned())), r#"{"ok":7}"#);
    }

    /// R1-31: each of the six kinds keeps its conformance word.
    #[test]
    fn each_kind_keeps_its_word() {
        let table = [
            (ErrorKind::Usage, "usage"),
            (ErrorKind::Backend, "backend"),
            (ErrorKind::Local, "local"),
            (ErrorKind::Cancelled, "cancelled"),
            (ErrorKind::Deadline, "deadline"),
            (ErrorKind::Defect, "defect"),
        ];
        for (held, word) in table {
            assert_eq!(kind(&Failure::of(held, "m").envelope()), word);
        }
    }

    /// R1-11: a native caller that skips the wrapper still meets the check.
    #[test]
    fn a_hostile_deadline_is_usage_with_its_number() {
        let table = [
            (f64::NAN, "NaN"),
            (f64::INFINITY, "Infinity"),
            (1e300, "1e+300"),
            (f64::MAX, "1.7976931348623157e+308"),
            (-2.0, "-2"),
            (0.5, "0.5"),
            (1e-7, "1e-7"),
            (4_294_967_296_000.0, "4294967296000"),
        ];
        for (held, written) in table {
            let refused = deadline_of(Some(held)).unwrap_err();
            assert_eq!(kind(&refused.envelope()), "usage");
            assert_eq!(
                refused.message,
                format!(
                    "deadlineMs {written} is not a deadline; use -1, null, or a whole number of milliseconds from 0 to 4294967295000"
                )
            );
        }
        assert_eq!(deadline_of(Some(-1.0)).unwrap(), None);
        assert_eq!(deadline_of(None).unwrap(), None);
        assert_eq!(deadline_of(Some(0.0)).unwrap(), Some(0));
        assert_eq!(
            deadline_of(Some(4_294_967_295_000.0)).unwrap(),
            Some(4_294_967_295_000)
        );
    }
}
