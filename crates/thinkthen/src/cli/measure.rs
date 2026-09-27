//! Read the named inputs of the measuring commands and say why one was refused.
//!
//! The module opens only the paths it is handed, or standard input for `-`,
//! and hands the bytes to the pure core. `sdlc/scripts/policy.py` holds it to that.

use std::fmt;
use std::io::{ErrorKind, Read as _, Write};
use std::path::Path;

use crate::core::measure::answer::{Rule, Shown};
use crate::core::measure::{Line, MeasureError, json_lines};
use crate::core::{Threshold, ThresholdError};
use crate::failure::Failure;

/// Why a measuring command stopped, told with the command name and the input's role.
///
/// No failure carries a record, an id, a key value, or a path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Refusal {
    /// `audit` or `diff`.
    pub(crate) command: &'static str,
    /// The input the failure is in, such as `results` or `key`.
    pub(crate) role: &'static str,
    /// What was refused.
    pub(crate) cause: Cause,
}

/// The sentence a `--by` value that is neither a word nor a pointer gets.
pub(crate) const BY: &str = "--by takes question, verb, or a JSON pointer such as /category";

/// What a measuring command refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Cause {
    /// The input could not be read.
    Unreadable,
    /// Two inputs name standard input.
    TwoStandardInputs,
    /// `diff` has neither a second run nor `--compare-threshold`.
    NoSecondRun,
    /// `--id` is not a JSON pointer.
    Pointer,
    /// A threshold option is not a rule.
    Rule(&'static str, ThresholdError),
    /// The core refused the input's contents.
    Measure(MeasureError),
    /// `--write` names standard input.
    WriteDash,
    /// `--write` beside `--threshold`.
    WriteThreshold,
    /// `--write` beside `--by verb`.
    WriteByVerb,
    /// `--write` beside a `--by` pointer.
    WriteByPointer,
    /// `--by` starts with `/` and is not a JSON pointer.
    ByPointer,
    /// `--curve` beside `--table`.
    CurveTable,
    /// The file `--write` names is not a question file or set a run accepts.
    NotQuestions,
    /// A results line was not asked from the file `--write` names.
    Digest(usize),
    /// The file `--write` names could not be written.
    Unwritable,
    /// The key has lines, and no answer that did not fail has a label in it.
    NoneLabeled,
}

impl Refusal {
    /// The exit code: 5 for a file that cannot be read, used, or written, else 2.
    pub(crate) const fn code(&self) -> u8 {
        if matches!(
            self.cause,
            Cause::Unreadable | Cause::NotQuestions | Cause::Unwritable
        ) {
            5
        } else {
            2
        }
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            command,
            role,
            cause,
        } = self;
        write!(formatter, "{command}: ")?;
        match cause {
            Cause::Unreadable => write!(formatter, "cannot read the {role} file"),
            Cause::TwoStandardInputs => formatter.write_str("only one input may be standard input"),
            Cause::NoSecondRun => {
                formatter.write_str("diff needs a second run or --compare-threshold")
            }
            Cause::Pointer => formatter.write_str("--id takes a JSON pointer such as /id or ''"),
            Cause::Rule(option, error) => write!(formatter, "{option}: {error}"),
            Cause::Measure(error) => said(formatter, command, role, *error),
            Cause::WriteDash => formatter.write_str("--write needs a file path"),
            Cause::WriteThreshold => {
                formatter.write_str("--write reads each answer as it ran; drop --threshold")
            }
            Cause::WriteByVerb => formatter.write_str("--write grades by question; drop --by verb"),
            Cause::WriteByPointer => {
                formatter.write_str("--write grades by question; drop the --by pointer")
            }
            Cause::ByPointer => formatter.write_str(BY),
            Cause::CurveTable => formatter.write_str("--curve prints JSON lines; drop --table"),
            Cause::NotQuestions => {
                formatter.write_str("--write names a file that is not a valid question file")
            }
            Cause::Digest(line) => write!(
                formatter,
                "results line {line} was not asked from the question file; --write needs --details lines from that file"
            ),
            Cause::Unwritable => formatter.write_str("cannot write the question file"),
            Cause::NoneLabeled => formatter.write_str(
                "no answer has a label in the key; check that --id points at the key's ids and that its values fit the verb",
            ),
        }
    }
}

fn said(
    formatter: &mut fmt::Formatter<'_>,
    command: &str,
    role: &str,
    error: MeasureError,
) -> fmt::Result {
    match error {
        MeasureError::NotObject(line) => {
            write!(formatter, "{role} line {line} is not a JSON object")
        }
        MeasureError::NoGroup(line) => write!(
            formatter,
            "{role} line {line} has no string or integer value at the --by pointer"
        ),
        MeasureError::NoId(line) => write!(
            formatter,
            "{role} line {line} has no string or integer id at the --id pointer"
        ),
        MeasureError::Ungradable(line) if command == "audit" => write!(
            formatter,
            "{role} line {line} holds an answer audit cannot grade; audit grades decide, filter, choose, tag, score, rank, find, recognize, and relate"
        ),
        MeasureError::Ungradable(line) => write!(
            formatter,
            "{role} line {line} holds an answer diff cannot grade; diff grades decide, choose, recognize and relate"
        ),
        MeasureError::NoQuestion(line) => write!(
            formatter,
            "{role} line {line} holds an answer without its question; save it with --details"
        ),
        MeasureError::KeyUnknown(line) => write!(
            formatter,
            "key line {line} names a level, label, or unit the question does not have"
        ),
        MeasureError::NoRule => formatter.write_str("score and find answers take no --threshold"),
        MeasureError::Probability(line) => write!(
            formatter,
            "{role} line {line} holds a probability outside 0 to 1 or an empty distribution"
        ),
        MeasureError::Repeated(line) => {
            let one = if command == "diff" {
                "answer"
            } else {
                "question"
            };
            write!(
                formatter,
                "{role} line {line} repeats a record for one {one}"
            )
        }
        MeasureError::KeyLine(line) => write!(
            formatter,
            "key line {line} needs a new id, a value, and a part of tune or held when present"
        ),
        MeasureError::ChooseKey(line) => {
            write!(
                formatter,
                "key line {line} gives a choose value that is not text"
            )
        }
        MeasureError::MixedParts => {
            formatter.write_str("the key gives a part on some labeled records and not on others")
        }
        MeasureError::TwoVerbs => {
            formatter.write_str("one question holds both decide and choose answers")
        }
        MeasureError::NeedsProbabilities => formatter
            .write_str("--threshold needs probabilities; rerun the question with --details"),
        MeasureError::SetCut => formatter.write_str(
            "recognize and relate take a single --threshold at or above the cut they ran with",
        ),
        MeasureError::KeyItems(line) => write!(
            formatter,
            "key line {line} gives recognize or relate a value unlike the command's own"
        ),
        MeasureError::BandOnChoose => {
            formatter.write_str("choose takes a single cut; a band applies to decide")
        }
        MeasureError::PairsVerbs(line) => write!(
            formatter,
            "{role} line {line} pairs recognize or relate with another verb"
        ),
        MeasureError::MixesVerbs(line) => write!(
            formatter,
            "{role} line {line} mixes recognize or relate with other verbs"
        ),
        MeasureError::MixesSets(line) => {
            write!(formatter, "{role} line {line} mixes recognize with relate")
        }
        MeasureError::MatchNotSet => formatter.write_str("--match applies to recognize and relate"),
    }
}

/// A threshold option as the rule answers are read under and the rule the output prints.
///
/// # Errors
///
/// Returns [`Cause::Rule`] naming the option when the text is not a rule.
pub(crate) fn rule(option: &'static str, text: Option<&str>) -> Result<(Rule, Shown), Cause> {
    let Some(text) = text else {
        return Ok((Rule::AsRun, Shown::AsRun));
    };
    let threshold: Threshold = text.parse().map_err(|error| Cause::Rule(option, error))?;
    let shown = threshold
        .cut_value()
        .map_or_else(|| Shown::Band(text.to_owned()), Shown::Cut);
    Ok((Rule::Threshold(threshold), shown))
}

/// Read one named input and split it into numbered JSON lines.
///
/// # Errors
///
/// Returns [`Cause::Unreadable`] or the core's refusal of a line.
pub(crate) fn lines(path: &Path) -> Result<Vec<Line>, Cause> {
    read(path).and_then(|bytes| json_lines(&bytes).map_err(Cause::Measure))
}

/// Write the whole output and flush. A reader that closed early is not a failure.
///
/// # Errors
///
/// Returns [`Failure::Output`] when standard output refuses the bytes.
pub(crate) fn write(mut writer: impl Write, text: &str) -> Result<(), Failure> {
    match writer
        .write_all(text.as_bytes())
        .and_then(|()| writer.flush())
    {
        Err(error) if error.kind() != ErrorKind::BrokenPipe => Err(Failure::Output(error)),
        _ => Ok(()),
    }
}

/// Read one named input whole: a path, or standard input for `-`.
///
/// # Errors
///
/// Returns [`Cause::Unreadable`] when the bytes cannot be read.
fn read(path: &Path) -> Result<Vec<u8>, Cause> {
    if path == Path::new("-") {
        let mut bytes = Vec::new();
        return std::io::stdin()
            .lock()
            .read_to_end(&mut bytes)
            .map(|_| bytes)
            .map_err(|_| Cause::Unreadable);
    }
    std::fs::read(path).map_err(|_| Cause::Unreadable)
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub(crate) enum Match {
    Strict,
    Overlap,
}

impl From<Match> for crate::core::measure::items::Matching {
    fn from(held: Match) -> Self {
        match held {
            Match::Strict => Self::Strict,
            Match::Overlap => Self::Overlap,
        }
    }
}
