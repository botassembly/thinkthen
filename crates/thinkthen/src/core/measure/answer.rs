//! One graded answer read from a saved result line, and what it says under a rule.

use std::collections::BTreeSet;

use serde::{Serialize, Serializer};

use crate::core::json::Json;
use crate::core::measure::{MeasureError, python_float_text, record_id, rounded};
use crate::core::pointer::Pointer;
use crate::core::probability::Probability;
use crate::core::threshold::{Outcome as Judged, Threshold};

/// The two verbs audit grades.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Verb {
    /// A yes or no answer.
    Decide,
    /// One option out of a list.
    Choose,
}

impl Verb {
    /// The verb's name as a result line writes it.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Decide => "decide",
            Self::Choose => "choose",
        }
    }
}

/// The rule answers are read under: each as it ran, or a threshold over its probabilities.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Rule {
    /// Each answer as it was printed.
    AsRun,
    /// A cut or a band applied to the saved probabilities.
    Threshold(Threshold),
}

/// A rule as the output prints it: `"as run"`, a cut as a number, or a band as text.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Shown {
    AsRun,
    Cut(f64),
    Band(String),
}

impl Serialize for Shown {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AsRun => serializer.serialize_str("as run"),
            Self::Cut(cut) => serializer.serialize_f64(*cut),
            Self::Band(band) => serializer.serialize_str(band),
        }
    }
}

impl Shown {
    /// The rule as a table writes it, a cut in Python's float text.
    pub(crate) fn text(&self) -> String {
        match self {
            Self::AsRun => "as run".to_owned(),
            Self::Cut(cut) => python_float_text(rounded(*cut)),
            Self::Band(band) => band.clone(),
        }
    }
}

/// What one answer says under a rule. An option named `tied` stays an option.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Said<'a> {
    /// A `decide` yes.
    Yes,
    /// A `decide` no.
    No,
    /// A `choose` option.
    Option(&'a str),
    /// No answer under the rule: a band's middle, a missed cut, or a null value.
    Unresolved,
    /// Two or more options share the top probability.
    Tied,
}

impl Said<'_> {
    /// The answer as the output names it.
    pub(crate) const fn text(&self) -> &str {
        match self {
            Self::Yes => "yes",
            Self::No => "no",
            Self::Option(option) => option,
            Self::Unresolved => "unresolved",
            Self::Tied => "tied",
        }
    }
}

/// Which answers count as one record twice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Identity {
    /// One answer name, record id, and question text: one question in `audit`.
    Question,
    /// One answer name and record id: one answer in a `diff` run.
    Answer,
}

/// The top of a `choose` distribution.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Top {
    /// The largest probability.
    pub(crate) top: f64,
    /// The first option, in member order, that holds it.
    pub(crate) pick: String,
    /// True when two or more options hold it.
    pub(crate) tied: bool,
}

/// One record's answer to one question.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Answer {
    /// The one-based line the answer came from.
    pub(crate) line: usize,
    /// The record id.
    pub(crate) id: String,
    /// The member name under `answers`, for an `annotate` line.
    pub(crate) name: Option<String>,
    /// The question text, when the line carries it.
    pub(crate) text: Option<String>,
    /// The line's question digest: `meta.question_sha256`, or `meta.questions_sha256` on an `annotate` line.
    pub(crate) digest: Option<String>,
    /// The verb, named by the line or read from its value.
    pub(crate) verb: Verb,
    /// True when the answer failed and leaves every measure.
    pub(crate) failed: bool,
    /// The printed value: a boolean, an option, or nothing.
    value: Option<Printed>,
    /// `p(yes)` for `decide`.
    pub(crate) probability: Option<Probability>,
    /// The distribution's top for `choose`.
    pub(crate) top: Option<Top>,
}

/// A printed value audit can read.
#[derive(Clone, Debug, PartialEq)]
enum Printed {
    Bool(bool),
    Option(String),
}

/// Read every answer that numbered result lines hold, with the record id at the pointer.
///
/// # Errors
///
/// Returns [`MeasureError`] for a line without an id, another verb, a
/// probability that is not one, or a record repeated under the identity.
pub(crate) fn read(
    lines: &[(usize, Json)],
    id: &Pointer,
    identity: Identity,
) -> Result<Vec<Answer>, MeasureError> {
    let mut read = Vec::new();
    for (line, row) in lines {
        let found = row.member("input").and_then(|input| id.resolve(input));
        let record = found.and_then(record_id).ok_or(MeasureError::NoId(*line))?;
        read.extend(answers(*line, &record, row)?);
    }
    refuse_repeats(&read, identity)?;
    Ok(read)
}

/// Read every answer one result line holds: the line itself, or each member of its `answers`.
fn answers(line: usize, id: &str, row: &Json) -> Result<Vec<Answer>, MeasureError> {
    let digest = |key| {
        row.member("meta")
            .and_then(|meta| meta.member(key))
            .and_then(Json::as_str)
    };
    match row.member("answers") {
        Some(Json::Object(members)) => members
            .iter()
            .map(|(name, entry)| {
                Answer::read(line, id, Some(name), digest("questions_sha256"), entry)
            })
            .collect(),
        Some(_) => Err(MeasureError::Ungradable(line)),
        None => Ok(vec![Answer::read(
            line,
            id,
            None,
            digest("question_sha256"),
            row,
        )?]),
    }
}

/// Refuse one record twice under the identity, failed answers included.
///
/// # Errors
///
/// Returns [`MeasureError::Repeated`] at the line of the second one.
fn refuse_repeats(answers: &[Answer], identity: Identity) -> Result<(), MeasureError> {
    let mut seen = BTreeSet::new();
    for answer in answers {
        let text = match identity {
            Identity::Question => answer.text.as_ref(),
            Identity::Answer => None,
        };
        if !seen.insert((&answer.name, &answer.id, text)) {
            return Err(MeasureError::Repeated(answer.line));
        }
    }
    Ok(())
}

impl Answer {
    fn read(
        line: usize,
        id: &str,
        name: Option<&str>,
        digest: Option<&str>,
        entry: &Json,
    ) -> Result<Self, MeasureError> {
        let value = entry.member("value");
        let failed = entry.member("failure").is_some()
            || value.and_then(|held| held.member("failed")).is_some()
            || (entry.member("answer").is_none() && value.is_none());
        let question = entry.member("question");
        let verb = match question
            .and_then(|held| held.member("verb"))
            .and_then(Json::as_str)
        {
            Some("decide") => Verb::Decide,
            Some("choose") => Verb::Choose,
            Some(other) if !other.is_empty() => return Err(MeasureError::Ungradable(line)),
            _ if matches!(value, None | Some(Json::Null | Json::Bool(_))) => Verb::Decide,
            _ => Verb::Choose,
        };
        let printed = match value {
            Some(Json::Bool(_)) if verb == Verb::Choose && !failed => {
                return Err(MeasureError::Ungradable(line));
            }
            Some(Json::Bool(held)) => Some(Printed::Bool(*held)),
            Some(Json::String(text)) => Some(Printed::Option(text.clone())),
            None | Some(Json::Null) => None,
            Some(_) if failed || verb == Verb::Decide => None,
            Some(_) => return Err(MeasureError::Ungradable(line)),
        };
        let answer = entry.member("answer");
        let probability = answer
            .and_then(|held| held.member("probability"))
            .filter(|held| **held != Json::Null)
            .map(|held| probability(line, held))
            .transpose()?;
        let top = answer
            .and_then(|held| held.member("probabilities"))
            .filter(|held| **held != Json::Null)
            .map(|held| top(line, held))
            .transpose()?;
        Ok(Self {
            line,
            id: id.to_owned(),
            name: name.map(str::to_owned),
            digest: digest.map(str::to_owned),
            text: question
                .and_then(|held| held.member("text"))
                .and_then(Json::as_str)
                .map(str::to_owned),
            verb,
            failed,
            value: printed,
            probability,
            top,
        })
    }

    /// True when the answer saved the probabilities a rule reads.
    pub(crate) const fn has_probability(&self) -> bool {
        match self.verb {
            Verb::Decide => self.probability.is_some(),
            Verb::Choose => self.top.is_some(),
        }
    }

    /// `p(yes)` for `decide`, or the top probability for `choose`.
    pub(crate) fn confidence(&self) -> Option<f64> {
        match self.verb {
            Verb::Decide => self.probability.map(Probability::as_f64),
            Verb::Choose => self.top.as_ref().map(|top| top.top),
        }
    }

    /// What this answer says under the rule.
    ///
    /// # Errors
    ///
    /// Returns [`MeasureError`] for a rule over an answer without
    /// probabilities, or a band over an untied `choose` answer.
    pub(crate) fn said(&self, rule: Rule) -> Result<Said<'_>, MeasureError> {
        if rule != Rule::AsRun && !self.has_probability() {
            return Err(MeasureError::NeedsProbabilities);
        }
        if self.verb == Verb::Decide {
            return Ok(match rule {
                Rule::AsRun => match self.value {
                    Some(Printed::Bool(true)) => Said::Yes,
                    Some(Printed::Bool(false)) => Said::No,
                    _ => Said::Unresolved,
                },
                Rule::Threshold(threshold) => match self.probability.map(|p| threshold.judge(p)) {
                    Some(Judged::Yes) => Said::Yes,
                    Some(Judged::No) => Said::No,
                    _ => Said::Unresolved,
                },
            });
        }
        if self.top.as_ref().is_some_and(|top| top.tied) {
            return Ok(Said::Tied);
        }
        match rule {
            Rule::AsRun => Ok(match &self.value {
                Some(Printed::Option(option)) => Said::Option(option),
                _ => Said::Unresolved,
            }),
            Rule::Threshold(threshold) => {
                let Some(cut) = threshold.cut_value() else {
                    return Err(MeasureError::BandOnChoose);
                };
                Ok(match &self.top {
                    Some(top) if top.top >= cut => Said::Option(&top.pick),
                    _ => Said::Unresolved,
                })
            }
        }
    }
}

fn probability(line: usize, value: &Json) -> Result<Probability, MeasureError> {
    let Json::Number(number) = value else {
        return Err(MeasureError::Probability(line));
    };
    number
        .as_f64()
        .and_then(|held| Probability::new(held).ok())
        .ok_or(MeasureError::Probability(line))
}

fn top(line: usize, value: &Json) -> Result<Top, MeasureError> {
    let Json::Object(members) = value else {
        return Err(MeasureError::Probability(line));
    };
    let mut found: Option<Top> = None;
    for (option, held) in members {
        let p = probability(line, held)?.as_f64();
        match &mut found {
            Some(top) if p == top.top => top.tied = true,
            Some(top) if p < top.top => {}
            _ => {
                found = Some(Top {
                    top: p,
                    pick: option.clone(),
                    tied: false,
                });
            }
        }
    }
    found.ok_or(MeasureError::Probability(line))
}
