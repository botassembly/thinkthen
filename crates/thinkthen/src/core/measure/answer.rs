//! One graded answer read from a saved result line, and what it says under a rule.

use std::collections::BTreeSet;

use serde::{Serialize, Serializer};

use crate::core::json::Json;
use crate::core::measure::levels::{LevelCuts, level_at};
use crate::core::measure::{MeasureError, python_float_text, record_id, rounded};
use crate::core::pointer::Pointer;
use crate::core::probability::Probability;
use crate::core::threshold::{Outcome as Judged, Threshold};

#[path = "verbs.rs"]
mod verbs;

use verbs::{names, probability, top, verb};

/// The verbs audit grades. diff reads only `decide` and `choose`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Verb {
    /// A yes or no answer, from `decide` or `filter`.
    Decide,
    /// One option out of a list.
    Choose,
    /// One label of a `tag` answer, read as yes or no.
    Tag,
    /// A place on named levels.
    Score,
    /// The best unit of one input, or `none`.
    Find,
    /// A ranked `decide` row, which makes no selection.
    Rank,
}

impl Verb {
    /// The verb's name as a result line or a row writes it.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Decide => "decide",
            Self::Choose => "choose",
            Self::Tag => "tag",
            Self::Score => "score",
            Self::Find => "find",
            Self::Rank => "rank",
        }
    }

    /// True for a verb whose answer reads as yes or no.
    pub(crate) const fn yes_no(self) -> bool {
        matches!(self, Self::Decide | Self::Tag | Self::Rank)
    }
}

/// The rule answers are read under: each as it ran, or a threshold over its probabilities.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Rule {
    /// Each answer as it was printed.
    AsRun,
    /// A cut or a band applied to the saved probabilities.
    Threshold(Threshold),
    /// Level cuts over a `score` number.
    Levels(LevelCuts),
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
    /// A yes.
    Yes,
    /// A no.
    No,
    /// An option, a level, or a unit.
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

/// Which answers count as one record twice, which also names the caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Identity {
    /// One answer name, record id, question text, and label: one question in `audit`,
    /// which reads every verb it grades.
    Question,
    /// One answer name and record id: one answer in a `diff` run, which reads
    /// `decide` and `choose` alone.
    Answer,
}

/// The top of a distribution.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Top {
    /// The largest probability.
    pub(crate) top: f64,
    /// The first option, in member order, that holds it.
    pub(crate) pick: String,
    /// True when two or more options hold it.
    pub(crate) tied: bool,
    /// Every option that holds it, in member order.
    pub(crate) holders: Vec<String>,
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
    /// The label a `tag` answer stands for.
    pub(crate) label: Option<String>,
    /// The verb, named by the line or read from its value.
    pub(crate) verb: Verb,
    /// True when the answer failed and leaves every measure.
    pub(crate) failed: bool,
    /// The printed value: a boolean, an option, or nothing.
    value: Option<Printed>,
    /// `p(yes)` for a yes/no answer.
    pub(crate) probability: Option<Probability>,
    /// The distribution's top for `choose` and `find`.
    pub(crate) top: Option<Top>,
    /// The labels, levels, or units the answer could name.
    pub(crate) options: Vec<String>,
    /// The number a `score` answer printed.
    pub(crate) number: Option<f64>,
    /// The line's question digest: `meta.question_sha256`, or `meta.questions_sha256` on an `annotate` line.
    pub(crate) digest: Option<String>,
    /// True when the line ran under a band.
    pub(crate) band: bool,
}

/// A printed value audit can read.
#[derive(Clone, Debug, PartialEq)]
enum Printed {
    Bool(bool),
    Option(String),
    /// A `find` answer that selected a unit.
    Found,
}

/// Read every answer that numbered result lines hold, with the record id at the pointer.
///
/// # Errors
///
/// Returns [`MeasureError`] for a line without an id, a verb the caller does
/// not grade, a probability that is not one, or a record repeated under the identity.
pub(crate) fn read(
    lines: &[(usize, Json)],
    id: &Pointer,
    identity: Identity,
) -> Result<Vec<Answer>, MeasureError> {
    let mut read = Vec::new();
    for (line, row) in lines {
        let found = row.member("input").and_then(|input| id.resolve(input));
        let record = match found.and_then(record_id) {
            Some(record) => record,
            None if identity == Identity::Question
                && row.member("input").is_none()
                && verb_named(row) == Some("find") =>
            {
                line.to_string()
            }
            None => return Err(MeasureError::NoId(*line)),
        };
        read.extend(answers(*line, &record, row, identity)?);
    }
    refuse_repeats(&read, identity)?;
    Ok(read)
}

fn verb_named(entry: &Json) -> Option<&str> {
    entry
        .member("question")
        .and_then(|held| held.member("verb"))
        .and_then(Json::as_str)
}

/// Read every answer one result line holds: the line itself, or each member of its `answers`.
fn answers(
    line: usize,
    id: &str,
    row: &Json,
    identity: Identity,
) -> Result<Vec<Answer>, MeasureError> {
    let named = if row.member("answers").is_some() {
        "questions_sha256"
    } else {
        "question_sha256"
    };
    let digest = row
        .member("meta")
        .and_then(|meta| meta.member(named))
        .and_then(Json::as_str)
        .map(str::to_owned);
    let read = |name: Option<&str>, entry: &Json| {
        Answer::read(line, id, name, entry, identity, digest.clone())
    };
    match row.member("answers") {
        Some(Json::Object(members)) => {
            let mut all = Vec::new();
            for (name, entry) in members {
                all.extend(read(Some(name), entry)?);
            }
            Ok(all)
        }
        Some(_) => Err(MeasureError::Ungradable(line)),
        None => read(None, row),
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
        if !seen.insert((&answer.name, &answer.id, text, &answer.label)) {
            return Err(MeasureError::Repeated(answer.line));
        }
    }
    Ok(())
}

impl Answer {
    /// Every answer one entry holds: one, or one per label of a `tag` answer.
    fn read(
        line: usize,
        id: &str,
        name: Option<&str>,
        entry: &Json,
        identity: Identity,
        digest: Option<String>,
    ) -> Result<Vec<Self>, MeasureError> {
        let value = entry.member("value");
        let failed = entry.member("failure").is_some()
            || value.and_then(|held| held.member("failed")).is_some()
            || (entry.member("answer").is_none() && value.is_none());
        let question = entry.member("question");
        let audit = identity == Identity::Question;
        let verb = verb(line, entry, audit, failed)?;
        let answer = entry.member("answer");
        let distribution = answer
            .and_then(|held| held.member("probabilities"))
            .filter(|held| **held != Json::Null);
        let mut read = Self {
            line,
            id: id.to_owned(),
            name: name.map(str::to_owned),
            text: question
                .and_then(|held| held.member("text"))
                .and_then(Json::as_str)
                .map(str::to_owned),
            label: None,
            verb,
            failed,
            value: None,
            probability: None,
            top: None,
            options: Vec::new(),
            number: None,
            digest,
            band: matches!(entry.member("threshold"), Some(Json::String(_))),
        };
        match verb {
            Verb::Tag if !failed => return read.labels(question, value, distribution),
            Verb::Score if !failed => {
                read.options = names(question.and_then(|held| held.member("levels")))
                    .filter(|levels| levels.len() >= 2)
                    .ok_or(MeasureError::Ungradable(line))?;
                read.number = match value {
                    Some(Json::Number(number)) => number.as_f64(),
                    _ => None,
                };
            }
            Verb::Find if !failed => {
                read.top = distribution.map(|held| top(line, held)).transpose()?;
                read.options = distribution
                    .and_then(|held| match held {
                        Json::Object(members) => {
                            Some(members.iter().map(|(unit, _)| unit.clone()).collect())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                if !matches!(value, None | Some(Json::Null)) {
                    read.value = Some(Printed::Found);
                }
            }
            Verb::Tag | Verb::Score | Verb::Find => {}
            Verb::Decide | Verb::Choose | Verb::Rank => {
                read.value = match value {
                    Some(Json::Bool(_)) if verb == Verb::Choose && !failed => {
                        return Err(MeasureError::Ungradable(line));
                    }
                    Some(Json::Bool(held)) => Some(Printed::Bool(*held)),
                    Some(Json::String(text)) => Some(Printed::Option(text.clone())),
                    None | Some(Json::Null) => None,
                    Some(_) if failed || verb != Verb::Choose => None,
                    Some(_) => return Err(MeasureError::Ungradable(line)),
                };
                read.probability = answer
                    .and_then(|held| held.member("probability"))
                    .filter(|held| **held != Json::Null)
                    .map(|held| probability(line, held))
                    .transpose()?;
                read.top = distribution.map(|held| top(line, held)).transpose()?;
            }
        }
        Ok(vec![read])
    }

    /// True when the answer saved what a rule reads.
    pub(crate) const fn has_probability(&self) -> bool {
        match self.verb {
            Verb::Decide | Verb::Tag | Verb::Rank => self.probability.is_some(),
            Verb::Choose | Verb::Find => self.top.is_some(),
            Verb::Score => self.number.is_some(),
        }
    }

    /// `p(yes)` for a yes/no answer, or the top probability for a pick.
    pub(crate) fn confidence(&self) -> Option<f64> {
        match self.verb {
            Verb::Decide | Verb::Tag | Verb::Rank => self.probability.map(Probability::as_f64),
            Verb::Choose | Verb::Find => self.top.as_ref().map(|top| top.top),
            Verb::Score => None,
        }
    }

    /// The zero-based place of a level name among this answer's levels.
    pub(crate) fn level(&self, name: &str) -> Option<usize> {
        self.options.iter().position(|level| level == name)
    }

    /// What this answer says under the rule.
    ///
    /// # Errors
    ///
    /// Returns [`MeasureError`] for a rule over an answer without
    /// probabilities, a band over an untied `choose` answer, or a threshold
    /// over a `score` or `find` answer.
    pub(crate) fn said(&self, rule: Rule) -> Result<Said<'_>, MeasureError> {
        if matches!(rule, Rule::Threshold(_)) && matches!(self.verb, Verb::Score | Verb::Find) {
            return Err(MeasureError::NoRule);
        }
        if rule != Rule::AsRun && !self.has_probability() {
            return Err(MeasureError::NeedsProbabilities);
        }
        match self.verb {
            Verb::Decide | Verb::Tag | Verb::Rank => Ok(match rule {
                Rule::Threshold(threshold) => match self.probability.map(|p| threshold.judge(p)) {
                    Some(Judged::Yes) => Said::Yes,
                    Some(Judged::No) => Said::No,
                    _ => Said::Unresolved,
                },
                Rule::AsRun | Rule::Levels(_) => match self.value {
                    Some(Printed::Bool(true)) => Said::Yes,
                    Some(Printed::Bool(false)) => Said::No,
                    _ => Said::Unresolved,
                },
            }),
            Verb::Score => {
                let cuts = match rule {
                    Rule::Levels(cuts) => cuts,
                    _ => LevelCuts::midpoints(self.options.len()),
                };
                Ok(self
                    .number
                    .and_then(|number| self.options.get(level_at(number, &cuts)))
                    .map_or(Said::Unresolved, |level| Said::Option(level)))
            }
            Verb::Find => Ok(match (&self.top, &self.value) {
                (None, _) => Said::Unresolved,
                (Some(top), Some(Printed::Found)) => Said::Option(&top.pick),
                (Some(top), _) if top.tied => Said::Tied,
                (Some(top), _) => Said::Option(&top.pick),
            }),
            Verb::Choose => {
                if self.top.as_ref().is_some_and(|top| top.tied) {
                    return Ok(Said::Tied);
                }
                let cut = match rule {
                    Rule::Threshold(threshold) => {
                        Some(threshold.cut_value().ok_or(MeasureError::BandOnChoose)?)
                    }
                    Rule::AsRun | Rule::Levels(_) => None,
                };
                Ok(match (cut, &self.value, &self.top) {
                    (None, Some(Printed::Option(option)), _) => Said::Option(option),
                    (Some(cut), _, Some(top)) if top.top >= cut => Said::Option(&top.pick),
                    _ => Said::Unresolved,
                })
            }
        }
    }
}
