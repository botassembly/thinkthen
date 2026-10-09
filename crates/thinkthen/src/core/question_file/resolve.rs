//! The one rule that settles what a run asks: the command line, then the file.
//!
//! Ian ruled the order on 2026-09-19. A single value typed beside a question
//! file replaces the file's value. A list typed beside it replaces the file's
//! whole list and never merges with it. Nothing here reaches outside the
//! process, so a library over this crate resolves a question the same way the
//! binary does.

use serde::{Serialize, Serializer};

use crate::core::adapters::built_in::DEFAULT_MODEL;
use crate::core::backend_profile::ProfileName;
use crate::core::pointer::Pointer;
use crate::core::question::{Labels, Question};
use crate::core::question_file::{
    Described, QuestionFile, QuestionFileError, Source, Verb, pointers,
};
use crate::core::text::{Meaning, ModelName, QuestionText};
use crate::core::threshold::Threshold;

/// What rule the caller acts on, where the verb alone does not say.
///
/// `filter` and `rank` ask the `decide` question of every record, so the verb
/// in the question file is `decide` for all three. `decide` takes a cut or a
/// band, `filter` keeps or drops and so takes a single cut, and `rank` orders
/// accepts an optional single probability cutoff.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum Cutting {
    /// Whatever the verb's own page allows.
    #[default]
    AsTheVerbAllows,
    /// One cut and never a band.
    OneCut,
    /// No rule in either home.
    NoRule,
    /// Optional rank cutoff, with no default and no band.
    RankCut,
}

/// What a caller typed beside the question, each value absent when untyped.
///
/// A caller with no command line leaves every field `None` and passes the
/// file alone.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Typed {
    /// What rule the caller acts on, where the verb alone does not say.
    pub(crate) cutting: Cutting,
    /// `--threshold`, as it was written.
    pub(crate) threshold: Option<String>,
    /// `--true`, the text that says what a yes means.
    pub(crate) yes: Option<String>,
    /// `--false`, the text that says what a no means.
    pub(crate) no: Option<String>,
    /// The options or the levels, as labels with their descriptions.
    pub(crate) labels: Option<Described>,
    /// `--model`.
    pub(crate) model: Option<String>,
    /// `--field`, which replaces the file's `on`.
    pub(crate) on: Option<Vec<String>>,
    /// True when each record carries its own options, as `--options` asks.
    ///
    /// The question is then built once per record, so no list is settled here
    /// and [`Resolved::question`] is `None`. Every other setting still
    /// resolves exactly once.
    pub(crate) options_from_record: bool,
}

/// The question that results, with the source of every setting.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Resolved {
    metadata: crate::core::declaration::QuestionMetadata,
    question: Option<Question>,
    text: QuestionText,
    threshold: Option<Threshold>,
    model: ModelName,
    on: Vec<Pointer>,
    sources: Sources,
    profile: Option<ProfileName>,
}

impl Resolved {
    pub(crate) const fn metadata(&self) -> &crate::core::declaration::QuestionMetadata {
        &self.metadata
    }
    /// The question to ask, or `None` when each record carries its own options.
    #[must_use]
    pub(crate) const fn question(&self) -> Option<&Question> {
        self.question.as_ref()
    }

    /// The question text, which every record is asked whatever its options are.
    #[must_use]
    pub(crate) const fn text(&self) -> &QuestionText {
        &self.text
    }

    /// The rule the answer is read under, or `None` on a verb that takes none.
    #[must_use]
    pub(crate) const fn threshold(&self) -> Option<Threshold> {
        self.threshold
    }

    /// The model the request names.
    #[must_use]
    pub(crate) const fn model(&self) -> &ModelName {
        &self.model
    }

    /// The pointers that say which part of a record is sent.
    #[must_use]
    pub(crate) fn on(&self) -> &[Pointer] {
        &self.on
    }

    /// Where each setting came from, which a plan prints under `from`.
    #[must_use]
    pub(crate) const fn sources(&self) -> &Sources {
        &self.sources
    }

    /// The profile this saved threshold was tuned under, when named.
    pub(crate) const fn profile(&self) -> Option<&ProfileName> {
        self.profile.as_ref()
    }
}

/// Where each setting of one question came from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Sources {
    verb: Option<Verb>,
    question: Source,
    yes: Source,
    no: Source,
    labels: Source,
    threshold: Source,
    takes_threshold: bool,
    on: Source,
    model: Source,
}

impl Sources {
    pub(crate) const fn for_find(on: Source, model: Source) -> Self {
        Self {
            verb: None,
            question: Source::File,
            yes: Source::Default,
            no: Source::Default,
            labels: Source::Default,
            threshold: Source::Default,
            takes_threshold: false,
            on,
            model,
        }
    }
    /// True when a question file settled the question, which a plan names.
    #[must_use]
    pub(crate) const fn question_is_from_file(&self) -> bool {
        matches!(self.question, Source::File)
    }

    pub(crate) const fn model_is_default(&self) -> bool {
        matches!(self.model, Source::Default)
    }

    #[must_use]
    pub(crate) const fn with_configuration_model(mut self) -> Self {
        self.model = Source::Configuration;
        self
    }
}

impl Serialize for Sources {
    /// Write one entry per setting the verb takes, in the order its page lists.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut named: Vec<(&str, Source)> = vec![("question", self.question)];
        match self.verb {
            Some(Verb::Decide) => named.extend([("true", self.yes), ("false", self.no)]),
            Some(Verb::Choose) => named.push(("options", self.labels)),
            Some(Verb::Tag) => named.push(("labels", self.labels)),
            Some(Verb::Score) => named.push(("levels", self.labels)),
            None => {}
        }
        if self.takes_threshold {
            named.push(("threshold", self.threshold));
        }
        named.extend([("on", self.on), ("model", self.model)]);
        serializer.collect_map(named)
    }
}

/// Settle what one run asks, with the command line first and the file second.
///
/// Ian ruled the order on 2026-09-19: the command line, then the file, then
/// the default. A single value typed beside a file replaces the file's value.
/// A list typed beside a file replaces the file's whole list and never merges
/// with it. The question text always comes from the file when a file was
/// named. `text` carries the question a caller typed, and it is `None` when a
/// file carries it instead.
///
/// # Errors
///
/// Returns [`QuestionFileError`] when the command does not match the file,
/// when a value fails a check the verb makes, and when no question was given
/// at all. Every message names the source of the value at fault.
pub(crate) fn resolve(
    verb: Verb,
    text: Option<&str>,
    file: Option<&QuestionFile>,
    typed: &Typed,
) -> Result<Resolved, QuestionFileError> {
    if let Some(file) = file
        && file.verb != verb
    {
        return Err(QuestionFileError::VerbMismatch {
            asked: verb,
            held: file.verb,
        });
    }
    let (question_text, question_source) = match file {
        Some(file) => (file.text.clone(), Source::File),
        None => (
            QuestionText::new(text.unwrap_or_default()).map_err(|error| {
                QuestionFileError::Blank {
                    origin: Source::CommandLine,
                    key: verb.word(),
                    error,
                }
            })?,
            Source::CommandLine,
        ),
    };
    let (yes, yes_source) = meaning_of(
        "true",
        typed.yes.as_deref(),
        file.and_then(|f| f.yes.clone()),
    )?;
    let (no, no_source) = meaning_of(
        "false",
        typed.no.as_deref(),
        file.and_then(|f| f.no.clone()),
    )?;
    let (threshold, threshold_source) = threshold_of(verb, typed, file)?;
    let (model, model_source) = model_of(typed, file)?;
    let (on, on_source) = on_of(typed, file)?;
    let (question, labels_source) = resolved_question(verb, &question_text, yes, no, typed, file)?;
    Ok(Resolved {
        metadata: file.map(|file| file.metadata.clone()).unwrap_or_default(),
        question,
        text: question_text,
        threshold,
        model,
        on,
        profile: file.and_then(|held| held.profile.clone()),
        sources: Sources {
            verb: Some(verb),
            question: question_source,
            yes: yes_source,
            no: no_source,
            labels: labels_source,
            threshold: threshold_source,
            takes_threshold: verb != Verb::Score
                && typed.cutting != Cutting::NoRule
                && (typed.cutting != Cutting::RankCut || threshold.is_some()),
            on: on_source,
            model: model_source,
        },
    })
}

/// Build the verb's question and name where its list came from.
fn resolved_question(
    verb: Verb,
    text: &QuestionText,
    yes: Option<Meaning>,
    no: Option<Meaning>,
    typed: &Typed,
    file: Option<&QuestionFile>,
) -> Result<(Option<Question>, Source), QuestionFileError> {
    Ok(match verb {
        Verb::Decide => (
            Some(Question::Decide {
                text: text.clone(),
                yes,
                no,
            }),
            Source::Default,
        ),
        Verb::Choose if typed.options_from_record => (None, Source::CommandLine),
        Verb::Choose => {
            let (options, source) = labels_of(verb, typed, file)?;
            (
                Some(Question::Choose {
                    text: text.clone(),
                    options,
                }),
                source,
            )
        }
        Verb::Tag => {
            let (labels, source) = labels_of(verb, typed, file)?;
            (
                Some(Question::Tag {
                    text: text.clone(),
                    labels,
                }),
                source,
            )
        }
        Verb::Score => {
            let (levels, source) = labels_of(verb, typed, file)?;
            (
                Some(Question::Score {
                    text: text.clone(),
                    levels,
                }),
                source,
            )
        }
    })
}

/// The model the request names, and where it came from.
fn model_of(
    typed: &Typed,
    file: Option<&QuestionFile>,
) -> Result<(ModelName, Source), QuestionFileError> {
    let blank = |origin| {
        move |error| QuestionFileError::Blank {
            origin,
            key: "model",
            error,
        }
    };
    match (
        typed.model.as_deref(),
        file.and_then(|held| held.model.clone()),
    ) {
        (Some(typed), _) => Ok((
            ModelName::new(typed).map_err(blank(Source::CommandLine))?,
            Source::CommandLine,
        )),
        (None, Some(held)) => Ok((held, Source::File)),
        (None, None) => Ok((
            ModelName::new(DEFAULT_MODEL).map_err(blank(Source::Default))?,
            Source::Default,
        )),
    }
}

/// The pointers that say what part of a record is sent, and where they came from.
fn on_of(
    typed: &Typed,
    file: Option<&QuestionFile>,
) -> Result<(Vec<Pointer>, Source), QuestionFileError> {
    match (typed.on.as_deref(), file.and_then(|held| held.on.clone())) {
        (Some(typed), _) => {
            let written: Vec<&str> = typed.iter().map(String::as_str).collect();
            Ok((
                pointers(&written, Source::CommandLine, "field")?,
                Source::CommandLine,
            ))
        }
        (None, Some(held)) => Ok((held, Source::File)),
        (None, None) => Ok((Vec::new(), Source::Default)),
    }
}

/// The text for what yes or what no means, and where it came from.
fn meaning_of(
    key: &'static str,
    typed: Option<&str>,
    held: Option<Meaning>,
) -> Result<(Option<Meaning>, Source), QuestionFileError> {
    match (typed, held) {
        (Some(typed), _) => Ok((
            Some(
                Meaning::new(typed).map_err(|error| QuestionFileError::Blank {
                    origin: Source::CommandLine,
                    key,
                    error,
                })?,
            ),
            Source::CommandLine,
        )),
        (None, Some(held)) => Ok((Some(held), Source::File)),
        (None, None) => Ok((None, Source::Default)),
    }
}

/// The rule the answer is read under, and where it came from.
fn threshold_of(
    verb: Verb,
    typed: &Typed,
    file: Option<&QuestionFile>,
) -> Result<(Option<Threshold>, Source), QuestionFileError> {
    if typed.threshold.is_some() && verb == Verb::Score {
        return Err(QuestionFileError::RuleOnScore(Source::CommandLine));
    }
    if typed.threshold.is_some() && typed.cutting == Cutting::NoRule {
        return Err(QuestionFileError::RuleOnRank(Source::CommandLine));
    }
    let (rule, source) = match (typed.threshold.as_deref(), file.and_then(|f| f.threshold)) {
        (Some(typed), _) => (
            Some(
                typed
                    .parse()
                    .map_err(|error| QuestionFileError::Threshold {
                        origin: Source::CommandLine,
                        error,
                    })?,
            ),
            Source::CommandLine,
        ),
        (None, Some(held)) => (Some(held), Source::File),
        (None, None) => match verb {
            Verb::Decide | Verb::Tag => (Some(Threshold::default()), Source::Default),
            _ => (None, Source::Default),
        },
    };
    // Rank has no default cutoff. Score and set routes admit no rule.
    if matches!(typed.cutting, Cutting::NoRule | Cutting::RankCut) && source == Source::Default {
        return Ok((None, source));
    }
    if let Some(rule) = rule {
        // `score` answers with a number and no rule cuts it. `choose` cuts on
        // one winning probability, which a band has no second side for.
        if verb == Verb::Score {
            return Err(QuestionFileError::RuleOnScore(source));
        }
        if verb == Verb::Choose && !rule.is_cut() {
            return Err(QuestionFileError::BandOnChoose(source));
        }
        if verb == Verb::Tag && !rule.is_cut() {
            return Err(QuestionFileError::BandOnTag(source));
        }
        match typed.cutting {
            Cutting::AsTheVerbAllows => {}
            Cutting::OneCut if !rule.is_cut() => {
                return Err(QuestionFileError::BandOnFilter(source));
            }
            Cutting::OneCut => {}
            Cutting::NoRule => return Err(QuestionFileError::RuleOnRank(source)),
            Cutting::RankCut if !rule.is_cut() => {
                return Err(QuestionFileError::BandOnRank(source));
            }
            Cutting::RankCut => {}
        }
    }
    Ok((rule, source))
}

/// The options or the levels the verb needs, and where they came from.
fn labels_of(
    verb: Verb,
    typed: &Typed,
    file: Option<&QuestionFile>,
) -> Result<(Labels, Source), QuestionFileError> {
    let key = match verb {
        Verb::Choose => "options",
        Verb::Tag => "labels",
        Verb::Decide | Verb::Score => "levels",
    };
    let from_file = file.and_then(|held| held.labels.clone());
    let (listed, source) = match (typed.labels.clone(), from_file) {
        (Some(typed), _) => (typed, Source::CommandLine),
        (None, Some(held)) => (held, Source::File),
        // Neither home holds a list, so neither one is at fault and the
        // message names no source. The verb needs a list, and that is all a
        // reader can act on.
        (None, None) => (Vec::new(), Source::Default),
    };
    let built = match verb {
        Verb::Choose => Labels::described(listed),
        Verb::Tag => Labels::tags(listed),
        Verb::Decide | Verb::Score => Labels::levels(listed),
    };
    built
        .map(|labels| (labels, source))
        .map_err(|error| QuestionFileError::Labels {
            origin: source,
            key,
            error,
        })
}

#[cfg(test)]
mod tests;
