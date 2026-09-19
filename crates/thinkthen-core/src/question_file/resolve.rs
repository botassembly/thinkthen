//! The one rule that settles what a run asks: the command line, then the file.
//!
//! Ian ruled the order on 2026-09-19. A single value typed beside a question
//! file replaces the file's value. A list typed beside it replaces the file's
//! whole list and never merges with it. Nothing here reaches outside the
//! process, so a library over this crate resolves a question the same way the
//! binary does.

use serde::{Serialize, Serializer};

use crate::backend::DEFAULT_MODEL;
use crate::pointer::Pointer;
use crate::question::{Labels, Question};
use crate::question_file::{QuestionFile, QuestionFileError, Source, Verb, pointers};
use crate::text::{Meaning, ModelName, QuestionText};
use crate::threshold::Threshold;

/// What a caller typed beside the question, each value absent when untyped.
///
/// A caller with no command line leaves every field `None` and passes the
/// file alone.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Typed {
    /// `--threshold`, as it was written.
    pub threshold: Option<String>,
    /// `--true`, the text that says what a yes means.
    pub yes: Option<String>,
    /// `--false`, the text that says what a no means.
    pub no: Option<String>,
    /// The options or the levels, as labels with their descriptions.
    pub labels: Option<Vec<(String, Option<String>)>>,
    /// `--model`.
    pub model: Option<String>,
    /// `--field`, which replaces the file's `on`.
    pub on: Option<Vec<String>>,
    /// True when each record carries its own options, as `--options` asks.
    ///
    /// The question is then built once per record, so no list is settled here
    /// and [`Resolved::question`] is `None`. Every other setting still
    /// resolves exactly once.
    pub options_from_record: bool,
}

/// The question that results, with the source of every setting.
#[derive(Clone, Debug, PartialEq)]
pub struct Resolved {
    question: Option<Question>,
    text: QuestionText,
    threshold: Option<Threshold>,
    model: ModelName,
    on: Vec<Pointer>,
    sources: Sources,
}

impl Resolved {
    /// The question to ask, or `None` when each record carries its own options.
    #[must_use]
    pub const fn question(&self) -> Option<&Question> {
        self.question.as_ref()
    }

    /// The question text, which every record is asked whatever its options are.
    #[must_use]
    pub const fn text(&self) -> &QuestionText {
        &self.text
    }

    /// The rule the answer is read under, or `None` on a verb that takes none.
    #[must_use]
    pub const fn threshold(&self) -> Option<Threshold> {
        self.threshold
    }

    /// The model the request names.
    #[must_use]
    pub const fn model(&self) -> &ModelName {
        &self.model
    }

    /// The pointers that say which part of a record is sent.
    #[must_use]
    pub fn on(&self) -> &[Pointer] {
        &self.on
    }

    /// Where each setting came from, which a plan prints under `from`.
    #[must_use]
    pub const fn sources(&self) -> &Sources {
        &self.sources
    }
}

/// Where each setting of one question came from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sources {
    verb: Verb,
    question: Source,
    yes: Source,
    no: Source,
    labels: Source,
    threshold: Source,
    on: Source,
    model: Source,
}

impl Serialize for Sources {
    /// Write one entry per setting the verb takes, in the order its page lists.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut named: Vec<(&str, Source)> = vec![("question", self.question)];
        match self.verb {
            Verb::Decide => named.extend([("true", self.yes), ("false", self.no)]),
            Verb::Choose => named.push(("options", self.labels)),
            Verb::Score => named.push(("levels", self.labels)),
        }
        if self.verb != Verb::Score {
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
pub fn resolve(
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
    let (model, model_source) = match (typed.model.as_deref(), file.and_then(|f| f.model.clone())) {
        (Some(typed), _) => (
            ModelName::new(typed).map_err(|error| QuestionFileError::Blank {
                origin: Source::CommandLine,
                key: "model",
                error,
            })?,
            Source::CommandLine,
        ),
        (None, Some(held)) => (held, Source::File),
        (None, None) => (
            ModelName::new(DEFAULT_MODEL).map_err(|error| QuestionFileError::Blank {
                origin: Source::Default,
                key: "model",
                error,
            })?,
            Source::Default,
        ),
    };
    let (on, on_source) = match (typed.on.as_deref(), file.and_then(|f| f.on.clone())) {
        (Some(typed), _) => {
            let written: Vec<&str> = typed.iter().map(String::as_str).collect();
            (
                pointers(&written, Source::CommandLine, "field")?,
                Source::CommandLine,
            )
        }
        (None, Some(held)) => (held, Source::File),
        (None, None) => (Vec::new(), Source::Default),
    };
    let (question, labels_source) = match verb {
        Verb::Decide => (
            Some(Question::Decide {
                text: question_text.clone(),
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
                    text: question_text.clone(),
                    options,
                }),
                source,
            )
        }
        Verb::Score => {
            let (levels, source) = labels_of(verb, typed, file)?;
            (
                Some(Question::Score {
                    text: question_text.clone(),
                    levels,
                }),
                source,
            )
        }
    };
    Ok(Resolved {
        question,
        text: question_text,
        threshold,
        model,
        on,
        sources: Sources {
            verb,
            question: question_source,
            yes: yes_source,
            no: no_source,
            labels: labels_source,
            threshold: threshold_source,
            on: on_source,
            model: model_source,
        },
    })
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
            Verb::Decide => (Some(Threshold::default()), Source::Default),
            _ => (None, Source::Default),
        },
    };
    if verb == Verb::Choose
        && let Some(rule) = rule
        && !rule.is_cut()
    {
        return Err(QuestionFileError::BandOnChoose(source));
    }
    Ok((rule, source))
}

/// The options or the levels the verb needs, and where they came from.
fn labels_of(
    verb: Verb,
    typed: &Typed,
    file: Option<&QuestionFile>,
) -> Result<(Labels, Source), QuestionFileError> {
    let key = if verb == Verb::Choose {
        "options"
    } else {
        "levels"
    };
    let from_file = file.and_then(|held| held.labels.clone());
    let (listed, source) = match (typed.labels.clone(), from_file) {
        (Some(typed), _) => (typed, Source::CommandLine),
        (None, Some(held)) => (held, Source::File),
        (None, None) => (
            Vec::new(),
            if file.is_some() {
                Source::File
            } else {
                Source::CommandLine
            },
        ),
    };
    let built = match verb {
        Verb::Choose => Labels::described(listed),
        _ => Labels::levels(listed.into_iter().map(|(name, _)| name).collect()),
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
mod tests {
    use super::{Typed, resolve};
    use crate::question::{LabelsError, Question};
    use crate::question_file::{QuestionFile, QuestionFileError as Refused, Source, Verb};
    use crate::render::json_line;

    fn file(text: &str) -> QuestionFile {
        QuestionFile::parse(text).expect("a question file")
    }

    fn typed_labels(values: &[&str]) -> Option<Vec<(String, Option<String>)>> {
        Some(
            values
                .iter()
                .map(|name| ((*name).to_owned(), None))
                .collect(),
        )
    }

    #[test]
    fn a_file_of_each_verb_holds_the_settings_it_names() {
        let decide = file(
            r#"{"decide":"Does this ask for a refund?","true":"Money back.","false":"Anything else.","threshold":"0.2:0.8","on":"/body","model":"jev-1.13.0"}"#,
        );
        assert_eq!(decide.verb(), Verb::Decide);
        let resolved = resolve(Verb::Decide, None, Some(&decide), &Typed::default())
            .expect("a resolved question");
        assert_eq!(
            json_line(resolved.question().expect("a question")).expect("a question is writable"),
            concat!(
                r#"{"verb":"decide","text":"Does this ask for a refund?","#,
                r#""true":"Money back.","false":"Anything else."}"#,
            )
        );
        assert_eq!(resolved.model().as_str(), "jev-1.13.0");
        assert_eq!(resolved.on().len(), 1);
        assert_eq!(
            resolved.threshold().map(|rule| rule.to_string()),
            Some("0.2:0.8".to_owned())
        );

        let choose = file(
            r#"{"choose":"Which team?","options":{"billing":"Money.","other":null},"threshold":0.8}"#,
        );
        let resolved =
            resolve(Verb::Choose, None, Some(&choose), &Typed::default()).expect("a resolved pick");
        assert!(matches!(resolved.question(), Some(Question::Choose { .. })));
        assert!(resolved.threshold().expect("a cut").is_cut());

        let score = file(r#"{"score":"How much?","levels":["None.","Some."]}"#);
        let resolved = resolve(Verb::Score, None, Some(&score), &Typed::default())
            .expect("a resolved placement");
        assert!(matches!(resolved.question(), Some(Question::Score { .. })));
        assert_eq!(resolved.threshold(), None);
    }

    #[test]
    fn a_command_that_does_not_match_the_file_is_refused_by_name() {
        let held = file(r#"{"choose":"Which team?","options":["a","b"]}"#);
        assert_eq!(
            resolve(Verb::Decide, None, Some(&held), &Typed::default()),
            Err(Refused::VerbMismatch {
                asked: Verb::Decide,
                held: Verb::Choose,
            })
        );
        assert_eq!(
            resolve(Verb::Decide, None, Some(&held), &Typed::default())
                .expect_err("a refusal")
                .to_string(),
            "the command is `decide` and the question file holds a `choose` question"
        );
    }

    #[test]
    fn the_command_line_beats_the_file_and_the_file_beats_the_default() {
        let held = file(
            r#"{"decide":"Does this ask for a refund?","true":"Money back.","threshold":"0.2:0.8","on":"/body","model":"jev-1.13.0"}"#,
        );
        let typed = Typed {
            threshold: Some("0.9".to_owned()),
            yes: Some("Cash back.".to_owned()),
            model: Some("jev-1.14.0".to_owned()),
            on: Some(vec!["/note".to_owned()]),
            ..Typed::default()
        };
        let resolved =
            resolve(Verb::Decide, None, Some(&held), &typed).expect("a resolved question");
        assert_eq!(
            json_line(resolved.question().expect("a question")).expect("a question is writable"),
            r#"{"verb":"decide","text":"Does this ask for a refund?","true":"Cash back."}"#
        );
        assert_eq!(resolved.model().as_str(), "jev-1.14.0");
        assert_eq!(resolved.on()[0].as_str(), "/note");
        assert_eq!(
            json_line(resolved.sources()).expect("sources are writable"),
            concat!(
                r#"{"question":"file","true":"command line","false":"default","#,
                r#""threshold":"command line","on":"command line","model":"command line"}"#,
            )
        );

        let alone = resolve(Verb::Decide, None, Some(&held), &Typed::default())
            .expect("a resolved question");
        assert_eq!(
            json_line(alone.sources()).expect("sources are writable"),
            concat!(
                r#"{"question":"file","true":"file","false":"default","#,
                r#""threshold":"file","on":"file","model":"file"}"#,
            )
        );
    }

    #[test]
    fn a_typed_list_replaces_the_files_whole_list_and_never_merges_with_it() {
        let held = file(
            r#"{"choose":"Which team?","options":{"billing":"Money.","shipping":"Parcels.","other":null}}"#,
        );
        let typed = Typed {
            labels: typed_labels(&["urgent", "later"]),
            ..Typed::default()
        };
        let resolved = resolve(Verb::Choose, None, Some(&held), &typed).expect("a resolved pick");
        assert_eq!(
            json_line(resolved.question().expect("a question")).expect("a question is writable"),
            r#"{"verb":"choose","text":"Which team?","options":["urgent","later"]}"#
        );
        assert_eq!(
            json_line(resolved.sources()).expect("sources are writable"),
            concat!(
                r#"{"question":"file","options":"command line","threshold":"default","#,
                r#""on":"default","model":"default"}"#,
            )
        );
    }

    #[test]
    fn a_question_with_no_file_reads_every_setting_from_the_command_line() {
        let typed = Typed {
            labels: typed_labels(&["bug", "other"]),
            threshold: Some("0.8".to_owned()),
            ..Typed::default()
        };
        let resolved =
            resolve(Verb::Choose, Some("Which kind?"), None, &typed).expect("a resolved pick");
        assert_eq!(resolved.model().as_str(), "jev-latest");
        assert!(resolved.on().is_empty());
        assert_eq!(
            json_line(resolved.sources()).expect("sources are writable"),
            concat!(
                r#"{"question":"command line","options":"command line","#,
                r#""threshold":"command line","on":"default","model":"default"}"#,
            )
        );
    }

    #[test]
    fn a_band_on_a_pick_is_refused_from_either_source() {
        let held = file(r#"{"choose":"Which team?","options":["a","b"],"threshold":"0.1:0.9"}"#);
        assert_eq!(
            resolve(Verb::Choose, None, Some(&held), &Typed::default()),
            Err(Refused::BandOnChoose(Source::File))
        );
        let typed = Typed {
            labels: typed_labels(&["a", "b"]),
            threshold: Some("0.1:0.9".to_owned()),
            ..Typed::default()
        };
        let refusal =
            resolve(Verb::Choose, Some("Which team?"), None, &typed).expect_err("a refused band");
        assert_eq!(refusal, Refused::BandOnChoose(Source::CommandLine));
        assert_eq!(
            refusal.to_string(),
            "--threshold: `choose` takes a single cut and never a band"
        );
        assert_eq!(refusal.origin(), Source::CommandLine);
    }

    #[test]
    fn a_list_the_verb_does_not_take_names_the_source_it_came_from() {
        let held = file(r#"{"choose":"Which team?","options":["only"]}"#);
        let refusal = resolve(Verb::Choose, None, Some(&held), &Typed::default())
            .expect_err("a refused list");
        assert_eq!(
            refusal,
            Refused::Labels {
                origin: Source::File,
                key: "options",
                error: LabelsError::OptionCount,
            }
        );
        assert_eq!(
            refusal.to_string(),
            "the question file's `options`: `choose` takes 2 to 255 options"
        );
        assert_eq!(refusal.origin(), Source::File);

        let typed = Typed {
            labels: typed_labels(&["only"]),
            ..Typed::default()
        };
        let refusal =
            resolve(Verb::Choose, Some("Which team?"), None, &typed).expect_err("a refused list");
        assert_eq!(refusal.to_string(), "`choose` takes 2 to 255 options");
        assert_eq!(refusal.origin(), Source::CommandLine);
    }

    #[test]
    fn a_label_holding_a_control_character_is_refused_from_a_file_too() {
        let held = file(r#"{"choose":"Which team?","options":["other","bug\nrm -rf /"]}"#);
        let refusal = resolve(Verb::Choose, None, Some(&held), &Typed::default())
            .expect_err("a refused list");
        assert_eq!(
            refusal,
            Refused::Labels {
                origin: Source::File,
                key: "options",
                error: LabelsError::Control,
            }
        );
        let said = refusal.to_string();
        assert!(!said.contains("rm -rf"), "{said}");
        assert!(!said.contains('\n'), "{said:?}");
    }

    #[test]
    fn options_that_each_record_carries_settle_every_other_setting_once() {
        let held = file(r#"{"choose":"Which code fits?","options":["a","b"],"threshold":0.8}"#);
        let typed = Typed {
            options_from_record: true,
            ..Typed::default()
        };
        let resolved = resolve(Verb::Choose, None, Some(&held), &typed).expect("a resolved pick");
        assert_eq!(resolved.question(), None);
        assert_eq!(resolved.text().as_str(), "Which code fits?");
        assert!(resolved.threshold().expect("a cut").is_cut());
        assert_eq!(
            json_line(resolved.sources()).expect("sources are writable"),
            concat!(
                r#"{"question":"file","options":"command line","threshold":"file","#,
                r#""on":"default","model":"default"}"#,
            )
        );
    }

    #[test]
    fn a_file_may_be_read_with_no_command_line_at_all() {
        let held = file(r#"{"score":"How much?","levels":["None.","Some.","Blocked."]}"#);
        let resolved = resolve(Verb::Score, None, Some(&held), &Typed::default())
            .expect("a resolved placement");
        assert_eq!(
            json_line(resolved.question().expect("a question")).expect("a question is writable"),
            r#"{"verb":"score","text":"How much?","levels":["None.","Some.","Blocked."]}"#
        );
        assert_eq!(
            json_line(resolved.sources()).expect("sources are writable"),
            r#"{"question":"file","levels":"file","on":"default","model":"default"}"#
        );
    }

    #[test]
    fn a_typed_question_and_the_same_question_in_a_file_ask_one_thing() {
        let held = file(r#"{"decide":"Does this ask for a refund?","threshold":0.8}"#);
        let from_file = resolve(Verb::Decide, None, Some(&held), &Typed::default())
            .expect("a resolved question");
        let typed = Typed {
            threshold: Some("0.8".to_owned()),
            ..Typed::default()
        };
        let from_line = resolve(
            Verb::Decide,
            Some("Does this ask for a refund?"),
            None,
            &typed,
        )
        .expect("a resolved question");
        assert_eq!(from_file.question(), from_line.question());
        assert_eq!(from_file.text(), from_line.text());
        assert_eq!(from_file.threshold(), from_line.threshold());
    }
}
