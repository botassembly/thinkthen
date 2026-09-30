//! The backend and recording routes every command is driven over.

use super::{DAMAGED, EVIDENCE, HOSTILE};
use crate::harness::Canned;

/// A good reply giving `answer` to each question the verb asked. `recognize`
/// asks one question for each of the evidence's five pieces.
pub(super) fn good(answer: &str) -> String {
    let asked = if answer.contains("BEGIN") { 5 } else { 1 };
    let answers: Vec<String> = (1..=asked)
        .map(|place| format!(r#""q{place}":{{{answer}}}"#))
        .collect();
    format!(
        r#"{{"model":"jev-1.13.0","answers":{{{}}},"#,
        answers.join(",")
    ) + r#""usage":{"input_tokens":9,"output_tokens":3}}"#
}

/// An error body that quotes the evidence back, as a real backend may.
///
/// Nothing the tool prints may carry it, so every message names the status and
/// never the body.
fn quoting() -> String {
    format!(r#"{{"error":{{"message":"refused: {EVIDENCE}"}}}}"#)
}

/// A reply that is JSON no adapter reads, quoting the evidence back.
///
/// A JSON reader names the value it stopped on, so a reply shaped like this one
/// is what would carry the evidence into a diagnostic.
fn unreadable() -> String {
    format!(r#"{{"model":"jev-1.13.0","answers":"{EVIDENCE}"}}"#)
}

/// What the loopback backend answers one run with.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Answers {
    /// A good reply for the one question the verb asked.
    Good,
    /// A status that fails at once, with a body that quotes the evidence.
    Status(u16),
    /// A rate limit that lifts, then a good reply.
    RateLimit,
    /// A rate limit that never lifts, past the one retry the run allows.
    RateLimited,
    /// A reply that is JSON no adapter reads, quoting the evidence back.
    Unreadable,
    /// Nothing at all, because the run must not reach the listener.
    Nothing,
}

impl Answers {
    /// The responses the listener serves, in order, for this verb.
    pub(crate) fn script(self, answer: &str) -> Vec<Canned> {
        match self {
            Self::Good => vec![Canned::ok(&good(answer))],
            Self::Status(status) => vec![Canned::status(status, &quoting())],
            Self::RateLimit => vec![Canned::status(429, &quoting()), Canned::ok(&good(answer))],
            Self::RateLimited => vec![
                Canned::status(429, &quoting()),
                Canned::status(429, &quoting()),
            ],
            Self::Unreadable => vec![Canned::ok(&unreadable())],
            Self::Nothing => Vec::new(),
        }
    }
}

/// One way a run can end, driven over every verb and both views.
///
/// `adds` may hold `{dir}`, which becomes this run's own folder, and `{closed}`,
/// which becomes the address of a port nothing listens on.
pub(crate) struct Route {
    pub(crate) named: &'static str,
    pub(crate) adds: &'static [&'static str],
    pub(crate) answers: Answers,
    /// How many requests the listener must see, which pins "sends nothing".
    pub(crate) requests: usize,
    /// The exit code the run earns, which pins that the path really ran.
    pub(crate) code: i32,
    /// Whether a recording is written into the folder before the run.
    pub(crate) primed: bool,
    /// What every entry the priming run wrote is overwritten with, if anything.
    pub(crate) damage: Option<&'static str>,
    /// The part of the message that names this refusal and no other one.
    ///
    /// Two routes can share an exit code, so a route that would otherwise pass
    /// on a neighbour's refusal pins the sentence it means.
    pub(crate) says: Option<&'static str>,
    /// What names the refusal when the damaged file is a question fixture.
    pub(crate) fixture_says: Option<&'static str>,
    /// Whether the run carries the key at all.
    pub(crate) keyed: bool,
}

pub(crate) const PATHS: [Route; 17] = [
    route("a success", &[], Answers::Good, 1, 0),
    route("a plan", &["--plan"], Answers::Nothing, 0, 0),
    route("a record run", &["--record", "{dir}"], Answers::Good, 1, 0),
    route("a cache", &["--cache", "{dir}"], Answers::Good, 1, 0),
    // The listener answers the priming run alone. The replay that follows it
    // finds no answer waiting, so the one request the listener counted is the
    // priming one and the replay opened no connection at all.
    Route {
        named: "a replay",
        adds: &["--replay", "{dir}"],
        answers: Answers::Good,
        requests: 1,
        code: 0,
        primed: true,
        damage: None,
        says: None,
        fixture_says: None,
        keyed: true,
    },
    // The entry is damaged after it is written, so the reply the run reads is
    // untrusted bytes holding the evidence that was recorded.
    Route {
        named: "a damaged entry",
        adds: &["--replay", "{dir}"],
        answers: Answers::Good,
        requests: 1,
        code: 5,
        primed: true,
        damage: Some(DAMAGED),
        says: Some("the file is not a recording entry: the JSON at line 1 column 105 is not one"),
        fixture_says: Some(
            "the entry `thinkthen.jsonl` was refused: line 1 is not a question entry",
        ),
        keyed: true,
    },
    // The entry parses and every field of it is hostile text, so the refusal
    // comes after the reading rather than during it.
    Route {
        named: "a hostile entry",
        adds: &["--replay", "{dir}"],
        answers: Answers::Good,
        requests: 1,
        code: 5,
        primed: true,
        damage: Some(HOSTILE),
        says: Some(
            "the entry names a schema this version does not read, \
             and this version reads `thinkthen.recording/1`",
        ),
        fixture_says: Some("records a different question, so the file was damaged or hand-edited"),
        keyed: true,
    },
    route(
        "a replay miss",
        &["--replay", "{dir}"],
        Answers::Nothing,
        0,
        5,
    ),
    route(
        "a refused address",
        &["--url", "http://example.com/v1"],
        Answers::Nothing,
        0,
        2,
    ),
    route(
        "a failed request",
        &["--url", "{closed}"],
        Answers::Nothing,
        0,
        4,
    ),
    route("a refused request", &[], Answers::Status(400), 1, 4),
    route("a rate limit that lifts", &[], Answers::RateLimit, 2, 0),
    route(
        "a rate limit that stays",
        &["--max-retries", "1"],
        Answers::RateLimited,
        2,
        4,
    ),
    route(
        "an exhausted backend failure",
        &["--max-retries", "0"],
        Answers::Status(500),
        1,
        4,
    ),
    route("an unreadable answer", &[], Answers::Unreadable, 1, 4),
    // Recording preflight refuses an unusable folder before a key or request.
    route(
        "a recording folder that cannot be made",
        &["--record", "/dev/null/x"],
        Answers::Nothing,
        0,
        5,
    ),
    // A loopback backend takes a run with no key, so this run names an
    // address the rules cannot prove is this machine. Nothing listens there.
    Route {
        named: "a run with no key",
        adds: &["--url", "https://127.0.0.2:9/v1"],
        answers: Answers::Nothing,
        requests: 0,
        code: 4,
        primed: false,
        damage: None,
        says: Some("`THINKTHEN_API_KEY` is unset or blank, so no key is sent"),
        fixture_says: None,
        keyed: false,
    },
];

/// One route with the two uncommon fields at their usual values.
const fn route(
    named: &'static str,
    adds: &'static [&'static str],
    answers: Answers,
    requests: usize,
    code: i32,
) -> Route {
    Route {
        named,
        adds,
        answers,
        requests,
        code,
        primed: false,
        damage: None,
        says: None,
        fixture_says: None,
        keyed: true,
    }
}
