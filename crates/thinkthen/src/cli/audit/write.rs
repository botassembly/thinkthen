//! `audit --write`: put the steady bar into the question file the results came from.
//!
//! The module reads the one file `--write` names, checks each result line's
//! question digest against it, and writes it back whole with one
//! `std::fs::write`. `sdlc/scripts/policy.py` allows that call here alone.

use std::collections::BTreeSet;
use std::path::Path;

use crate::cli::measure::Cause;
use crate::core::measure::Line;
use crate::core::measure::answer::{Answer, Verb};
use crate::core::measure::audit::{Kind, Row};
use crate::core::measure::optimize::Bar;
use crate::core::measure::rows::number;
use crate::core::measure::splice::splice;
use crate::core::{
    BlankTextError, Json, ModelName, QuestionFile, QuestionSet, RecognizeSpec, RelateSpec, Typed,
    question_sha256_with_profile, recognize_sha256, resolve,
};

/// What `--write` does with one question's bar.
enum Choice {
    /// Write this threshold text.
    Write(String),
    /// Keep the file as it is, for the reason given.
    Keep(String),
}

/// Write each steady bar into the file and return the report, one line per bar.
/// A single file that gains a bar also records the one model every results line names.
///
/// # Errors
///
/// Returns [`Cause::Unreadable`] or [`Cause::NotQuestions`] for a file a run
/// would not accept, [`Cause::Digest`] for a line another question asked, and
/// [`Cause::Unwritable`] when the new text cannot be written.
pub(super) fn bars(
    path: &Path,
    results: &[Line],
    answers: &[Answer],
    rows: &[Row],
) -> Result<String, Cause> {
    let bytes = std::fs::read(path).map_err(|_| Cause::Unreadable)?;
    let text = String::from_utf8(bytes).map_err(|_| Cause::NotQuestions)?;
    let json = Json::parse(&text).map_err(|_| Cause::NotQuestions)?;
    let set = json.member("questions").is_some();
    let digests: Vec<String> = if set {
        let parsed = QuestionSet::parse(&text).map_err(|_| Cause::NotQuestions)?;
        parsed.sha256().ok().into_iter().collect()
    } else if json.member("recognize").is_some() {
        let spec = RecognizeSpec::parse(&text).map_err(|_| Cause::NotQuestions)?;
        recognize_sha256(&spec).ok().into_iter().collect()
    } else if json.member("relate").is_some() {
        // A `--lines` run nulls the fields, so either digest names this file.
        let spec = RelateSpec::parse(&text).map_err(|_| Cause::NotQuestions)?;
        [false, true]
            .into_iter()
            .filter_map(|lines| spec.question(lines).sha256().ok())
            .collect()
    } else {
        let file = QuestionFile::parse(&text).map_err(|_| Cause::NotQuestions)?;
        let resolved = resolve(file.verb(), None, Some(&file), &Typed::default())
            .map_err(|_| Cause::NotQuestions)?;
        let digest = resolved.question().and_then(|question| {
            question_sha256_with_profile(question, resolved.threshold(), resolved.profile()).ok()
        });
        digest.into_iter().collect()
    };
    let checked = answers
        .iter()
        .filter(|answer| !matches!(answer.verb, Verb::Rank | Verb::Find));
    for answer in checked {
        if !answer
            .digest
            .as_ref()
            .is_some_and(|held| digests.contains(held))
        {
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
    if new != text && !set {
        match model(results) {
            Ok(model) => {
                let value =
                    serde_json::to_string(model.as_str()).map_err(|_| Cause::NotQuestions)?;
                new = splice(&new, &["model"], &value)
                    .ok_or(Cause::NotQuestions)?
                    .0;
            }
            Err(reason) => report.push_str(&format!(
                "{}: audit: kept the model for the question; {reason}\n",
                crate::core::NAME
            )),
        }
    }
    if new != text {
        std::fs::write(path, new).map_err(|_| Cause::Unwritable)?;
    }
    Ok(report)
}

/// The one model every results line names in `meta.model`, trimmed as a
/// later run reads it, or why no model is written.
fn model(results: &[Line]) -> Result<ModelName, &'static str> {
    let named = results
        .iter()
        .map(|(_, line)| line.member("meta")?.member("model")?.as_str())
        .collect::<Option<BTreeSet<_>>>()
        .ok_or("a result names no model")?;
    let mut named = named.into_iter();
    match (named.next(), named.next()) {
        (Some(model), None) => ModelName::new(model).map_err(|error| match error {
            BlankTextError::ModelControl => {
                "a result names a model with a control character or white space but a plain space"
            }
            _ => "a result names a blank model",
        }),
        _ => Err("the results name more than one model"),
    }
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
