//! The arms a caller picks by the path of the base it gives.
//!
//! `/case/ID/…` answers the named case's exact request bodies. `/generic/…`
//! answers any well-formed request by one fixed rule. `/arm/NAME/…` serves one
//! wire fault. Anything else earns the drift status and a line on standard
//! error, so a caller that drifted fails loud.

use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, BufRead, Write};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde::de::{Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde_json::value::RawValue;

use crate::lifetime::Gate;
use crate::listener::{Canned, Listener, Recorded};

/// The status every unknown body, arm, or request earns, and no arm serves.
pub(crate) const DRIFT: u16 = 500;

/// The longest delay the delay arm serves, under the engine's request timeout.
const MOST_DELAY: u64 = 10_000;

/// How long a `wait` line waits for its count.
const WAIT_BOUND: Duration = Duration::from_secs(5);

/// The shared cases, compiled in so the binary needs no path.
const CASES: &str = include_str!("../../cases.json");

/// The six malformed replies, one per failure cause.
const CAUSES: [&str; 6] = [
    "missing_answer",
    "wrong_kind",
    "missing_probability",
    "invalid_probability",
    "invalid_distribution",
    "unexpected_probability",
];

/// Each case's request bodies and the response each one gets.
type Cases = BTreeMap<String, BTreeMap<Vec<u8>, String>>;

#[derive(Deserialize)]
struct Document {
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    exchanges: Vec<Exchange>,
}

#[derive(Deserialize)]
struct Exchange {
    request: String,
    response: Box<RawValue>,
}

#[derive(Deserialize)]
struct Request {
    model: String,
    questions: BTreeMap<String, Question>,
}

#[derive(Deserialize)]
struct Question {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    criteria: Criteria,
}

/// The option names in the order sent, or each level's number.
#[derive(Default)]
struct Criteria(Vec<String>);

/// The conformance backend: one loopback listener and the gate its held arm waits on.
#[derive(Debug)]
pub struct Backend {
    listener: Listener,
    gate: Arc<Gate>,
}

impl Backend {
    /// Bind 127.0.0.1 on an ephemeral port and serve every arm.
    pub fn start() -> io::Result<Self> {
        let document: Document = serde_json::from_str(CASES).map_err(io::Error::other)?;
        let mut cases = Cases::new();
        for case in document.cases {
            let bodies = case.exchanges.into_iter().map(|exchange| {
                (
                    exchange.request.into_bytes(),
                    exchange.response.get().to_owned(),
                )
            });
            cases.insert(case.id, bodies.collect());
        }
        let gate = Arc::new(Gate::default());
        let held = Arc::clone(&gate);
        let listener = Listener::routing(move |request| route(&cases, &held, request))?;
        Ok(Self { listener, gate })
    }

    /// The scheme, address, and port. A caller appends an arm's path.
    pub fn origin(&self) -> &str {
        self.listener.origin()
    }

    /// How many requests the backend has read so far.
    pub fn count(&self) -> usize {
        self.listener.count()
    }

    /// Let every held reply go, now and from here on.
    pub fn release(&self) {
        self.gate.release();
    }

    /// Let go every reply held now. A reply that arrives later holds again.
    pub fn round(&self) {
        self.gate.next_round();
    }

    /// The count once it reads at least `least`, or at 5 s, whichever comes first.
    pub fn wait(&self, least: usize) -> usize {
        self.wait_until(least, &AtomicBool::new(false))
            .unwrap_or_else(|| self.count())
    }

    fn wait_until(&self, least: usize, cancelled: &AtomicBool) -> Option<usize> {
        let deadline = Instant::now() + WAIT_BOUND;
        loop {
            if cancelled.load(Ordering::SeqCst) {
                return None;
            }
            let count = self.count();
            if count >= least || Instant::now() >= deadline {
                return Some(count);
            }
            thread::sleep(Duration::from_millis(5));
        }
    }
}

/// Print the port, answer `count`, `release`, `round`, and `wait N` lines, and
/// print the count at the end.
///
/// A `wait` answers on a thread of its own, so the lines behind it run at once.
/// The output is shared with those threads. Input close cancels and joins them.
pub fn run(input: impl BufRead, output: impl Write + Send + 'static) -> io::Result<()> {
    let backend = Arc::new(Backend::start()?);
    let output = Arc::new(Mutex::new(output));
    let port = backend.origin().rsplit(':').next().unwrap_or_default();
    say(&output, port)?;
    let cancelled = Arc::new(AtomicBool::new(false));
    let mut waits = Vec::new();
    let read = (|| -> io::Result<()> {
        for line in input.lines() {
            match line?.trim() {
                "count" => say(&output, backend.count())?,
                "release" => backend.release(),
                "round" => backend.round(),
                other => match other.strip_prefix("wait ").and_then(whole) {
                    Some(least) => {
                        let (backend, output, cancelled) = (
                            Arc::clone(&backend),
                            Arc::clone(&output),
                            Arc::clone(&cancelled),
                        );
                        waits.push(thread::spawn(move || {
                            if let Some(count) = backend.wait_until(least, &cancelled) {
                                let _ = say(&output, format!("wait {count}"));
                            }
                        }));
                    }
                    None => writeln!(io::stderr(), "conformance-backend: unknown line `{other}`")?,
                },
            }
        }
        Ok(())
    })();
    cancelled.store(true, Ordering::SeqCst);
    for wait in waits {
        let _ = wait.join();
    }
    read?;
    say(&output, backend.count())
}

/// Write one line under the output lock and flush it.
fn say(output: &Mutex<impl Write>, text: impl fmt::Display) -> io::Result<()> {
    let mut output = output
        .lock()
        .map_err(|_| io::Error::other("the output lock was poisoned"))?;
    writeln!(output, "{text}")?;
    output.flush()
}

/// A whole number: ASCII digits only, so `+1` and `-1` are not one.
fn whole<T: FromStr>(text: &str) -> Option<T> {
    let digits = !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit());
    digits.then(|| text.parse().ok()).flatten()
}

/// Pick the arm the request's path names.
fn route(cases: &Cases, gate: &Arc<Gate>, request: &Recorded) -> Canned {
    let path = request.line.split(' ').nth(1).unwrap_or_default();
    let mut parts = path.trim_start_matches('/').split('/');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("case"), Some(id), _) => cases
            .get(id)
            .and_then(|bodies| bodies.get(&request.body))
            .map_or_else(
                || drift(&format!("case `{id}` has no such body")),
                |body| Canned::ok(body),
            ),
        (Some("generic"), ..) => generic(&request.body, None, false),
        (Some("arm"), Some("full"), _) => generic(&request.body, None, true),
        (Some("arm"), Some("status"), code) => match code.and_then(whole::<u16>) {
            Some(code @ 401..=404) => Canned::status(code, "status arm"),
            _ => drift("the status arm takes 401, 402, 403, or 404"),
        },
        (Some("arm"), Some("reset"), _) => Canned::reset(),
        (Some("arm"), Some(status @ ("429" | "503")), _) => {
            let status = if status == "429" { 429 } else { 503 };
            Canned::status(status, "try again").asking("retry-after-ms", "10")
        }
        (Some("arm"), Some("refuse"), _) => Canned::status(422, "refused"),
        (Some("arm"), Some("held"), _) => {
            generic(&request.body, None, false).held_by(Arc::clone(gate))
        }
        (Some("arm"), Some("delay"), value) => match value.and_then(whole::<u64>) {
            None => drift("the delay arm needs a whole number of milliseconds"),
            Some(millis) if millis > MOST_DELAY => {
                drift("the delay arm allows at most 10000 milliseconds")
            }
            Some(millis) => generic(&request.body, None, false).after(millis),
        },
        (Some("arm"), Some("malformed"), Some(cause)) => generic(&request.body, Some(cause), false),
        _ => drift(&format!("no arm at `{path}`")),
    }
}

/// Answer with the drift status and say why on standard error.
pub(crate) fn drift(why: &str) -> Canned {
    let _ = writeln!(io::stderr(), "conformance-backend: {why}");
    Canned::status(DRIFT, why)
}

/// Answer every question by the fixed rule, breaking one when a cause is named.
///
/// The first option, level, or yes gets 0.9, and the rest share the remainder
/// in declared order. A broken answer lands on the last question that can
/// carry its cause; a distribution cause needs a choice or score question.
/// A `full` answer adds every optional field the tool reads: a confidence on
/// each choice and score answer, and the token counts.
fn generic(body: &[u8], cause: Option<&str>, full: bool) -> Canned {
    let Ok(request) = serde_json::from_slice::<Request>(body) else {
        return drift("the generic arm got no well-formed request");
    };
    if cause.is_some_and(|cause| !CAUSES.contains(&cause)) {
        return drift("the malformed arm names no failure cause");
    }
    let yes_no = |question: &Question| question.kind == "noul";
    let broken = cause.and_then(|cause| {
        let needs_levels = !matches!(
            cause,
            "missing_answer" | "wrong_kind" | "invalid_probability"
        );
        let mut numbered: Vec<_> = request.questions.iter().collect();
        numbered.sort_by_key(|(name, _)| name.trim_start_matches('q').parse::<usize>().ok());
        numbered
            .into_iter()
            .rev()
            .find(|(_, question)| !needs_levels || !yes_no(question))
            .map(|(name, _)| (name.clone(), cause))
    });
    if cause.is_some() && broken.is_none() {
        return drift("no question can carry that failure cause");
    }
    let mut answers = Vec::new();
    for (name, question) in &request.questions {
        let cause = broken
            .as_ref()
            .and_then(|(target, cause)| (target == name).then_some(*cause));
        let Some(answer) = answer(question, cause, full) else {
            if cause.is_none() {
                return drift(&format!(
                    "the generic arm cannot answer `{}`",
                    question.kind
                ));
            }
            continue;
        };
        answers.push(format!("{}:{answer}", quoted(name)));
    }
    let usage = if full {
        r#","usage":{"input_tokens":1,"output_tokens":1}"#
    } else {
        ""
    };
    Canned::ok(&format!(
        r#"{{"model":{},"answers":{{{}}}{usage}}}"#,
        quoted(&request.model),
        answers.join(",")
    ))
}

/// One answer by the fixed rule, or broken for the cause. `None` omits it.
fn answer(question: &Question, cause: Option<&str>, full: bool) -> Option<String> {
    let kind = question.kind.as_str();
    if kind == "noul" {
        return match cause {
            Some("missing_answer") => None,
            Some("wrong_kind") => Some(r#"{"type":"choice","probabilities":{}}"#.to_owned()),
            Some(_) => Some(r#"{"type":"noul","noul":1.5}"#.to_owned()),
            None => Some(r#"{"type":"noul","noul":0.9}"#.to_owned()),
        };
    }
    if kind != "choice" && kind != "score" {
        return None;
    }
    let keys = &question.criteria.0;
    let rest = keys.len().saturating_sub(1);
    let mut spread: Vec<(String, f64)> = keys
        .iter()
        .enumerate()
        .map(|(place, key)| {
            let share = match (place, rest) {
                (0, 0) => 1.0,
                (0, _) => 0.9,
                _ => 0.1 / rest as f64,
            };
            (key.clone(), share)
        })
        .collect();
    let first = spread.first_mut();
    match (cause, first) {
        (Some("missing_answer"), _) => return None,
        (Some("wrong_kind"), _) => return Some(r#"{"type":"noul","noul":0.9}"#.to_owned()),
        (Some("missing_probability"), Some(_)) => drop(spread.remove(0)),
        (Some("invalid_probability"), Some(first)) => first.1 = 1.5,
        (Some("invalid_distribution"), Some(first)) => first.1 = 0.5,
        (Some("unexpected_probability"), _) => spread.push(("unexpected".to_owned(), 0.0)),
        _ => {}
    }
    let entries: Vec<String> = spread
        .iter()
        .map(|(key, share)| format!("{}:{share}", quoted(key)))
        .collect();
    let confidence = if full { r#","confidence":0.9"# } else { "" };
    Some(format!(
        r#"{{"type":"{kind}","probabilities":{{{}}}{confidence}}}"#,
        entries.join(",")
    ))
}

/// A string as JSON text.
fn quoted(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_default()
}

impl<'de> Deserialize<'de> for Criteria {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Keys)
    }
}

/// Reads option names in the order sent, or counts the levels of a list.
struct Keys;

impl<'de> Visitor<'de> for Keys {
    type Value = Criteria;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object of options or a list of levels")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Criteria, A::Error> {
        let mut keys = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            map.next_value::<IgnoredAny>()?;
            keys.push(key);
        }
        Ok(Criteria(keys))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut levels: A) -> Result<Criteria, A::Error> {
        let mut keys = Vec::new();
        while levels.next_element::<IgnoredAny>()?.is_some() {
            keys.push(keys.len().to_string());
        }
        Ok(Criteria(keys))
    }
}
