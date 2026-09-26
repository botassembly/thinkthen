//! `audit --write`: put the steady bar into the question file the results came from.
//!
//! The module reads the one file `--write` names, checks each result line's
//! question digest against it, and writes it back whole with one
//! `std::fs::write`. `sdlc/scripts/policy.py` allows that call here alone.

use std::path::Path;

use crate::cli::measure::Cause;
use crate::core::measure::answer::{Answer, Verb};
use crate::core::measure::audit::{Kind, Row};
use crate::core::measure::optimize::Bar;
use crate::core::measure::rows::number;
use crate::core::measure::splice::splice;
use crate::core::{Json, QuestionFile, QuestionSet, Typed, question_sha256_with_profile, resolve};

/// What `--write` does with one question's bar.
enum Choice {
    /// Write this threshold text.
    Write(String),
    /// Keep the file as it is, for the reason given.
    Keep(String),
}

/// Write each steady bar into the file and return the report, one line per bar.
///
/// # Errors
///
/// Returns [`Cause::Unreadable`] or [`Cause::NotQuestions`] for a file a run
/// would not accept, [`Cause::Digest`] for a line another question asked, and
/// [`Cause::Unwritable`] when the new text cannot be written.
pub(super) fn bars(path: &Path, answers: &[Answer], rows: &[Row]) -> Result<String, Cause> {
    let bytes = std::fs::read(path).map_err(|_| Cause::Unreadable)?;
    let text = String::from_utf8(bytes).map_err(|_| Cause::NotQuestions)?;
    let set = Json::parse(&text)
        .map_err(|_| Cause::NotQuestions)?
        .member("questions")
        .is_some();
    let digest = if set {
        let parsed = QuestionSet::parse(&text).map_err(|_| Cause::NotQuestions)?;
        parsed.sha256().ok()
    } else {
        let file = QuestionFile::parse(&text).map_err(|_| Cause::NotQuestions)?;
        let resolved = resolve(file.verb(), None, Some(&file), &Typed::default())
            .map_err(|_| Cause::NotQuestions)?;
        resolved.question().and_then(|question| {
            question_sha256_with_profile(question, resolved.threshold(), resolved.profile()).ok()
        })
    };
    let checked = answers
        .iter()
        .filter(|answer| !matches!(answer.verb, Verb::Rank | Verb::Find));
    for answer in checked {
        if digest.is_none() || answer.digest != digest {
            return Err(Cause::Digest(answer.line));
        }
    }
    let mut new = text.clone();
    let mut report = String::new();
    for row in rows.iter().filter(|row| row.kind != Kind::Label) {
        let (target, place) = match (&row.name, set) {
            (Some(name), true) => (
                format!("question {name}"),
                vec!["questions", name.as_str(), "threshold"],
            ),
            _ => ("the question".to_owned(), vec!["threshold"]),
        };
        let line = match decide(row) {
            Choice::Keep(reason) => reason.replace("TARGET", &target),
            Choice::Write(value) => {
                let (spliced, old) = splice(&new, &place, &value).ok_or(Cause::NotQuestions)?;
                new = spliced;
                let old = old.unwrap_or_else(|| "absent".to_owned());
                format!("wrote threshold {value} for {target}; it was {old}")
            }
        };
        report.push_str(&format!("{}: audit: {line}\n", crate::core::NAME));
    }
    if new != text {
        std::fs::write(path, new).map_err(|_| Cause::Unwritable)?;
    }
    Ok(report)
}

/// Whether one row's steady bar is written, and why it is kept when it is not.
fn decide(row: &Row) -> Choice {
    let keep = |reason: &str| Choice::Keep(reason.to_owned());
    match row.verb {
        Some("rank" | "find") => {
            return keep("kept TARGET unchanged; audit suggests no bar for rank or find");
        }
        Some("score") => return keep("kept TARGET unchanged; a score question takes no threshold"),
        _ if row.band => return keep("kept the band for TARGET; audit suggests a single cut"),
        _ => {}
    }
    let steady = row
        .suggested
        .as_ref()
        .and_then(|suggested| suggested.steady.clone().flatten());
    let Some(steady) = steady else {
        return keep("kept the bar for TARGET; audit found no cut to suggest");
    };
    if steady.better * 2 <= steady.splits {
        return Choice::Keep(format!(
            "kept the bar for TARGET; the steady bar beat it on {} of {} held parts",
            steady.better, steady.splits
        ));
    }
    match steady.cut {
        Bar::Cut(k) => Choice::Write(number(k)),
        Bar::Levels(_) => keep("kept TARGET unchanged; a score question takes no threshold"),
    }
}
