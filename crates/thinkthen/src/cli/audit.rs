//! `thinkthen audit`: grade saved answers against an answer key and suggest a bar.
//!
//! The command reads the paths it is handed and nothing else, sends no
//! request, and reads no setting. Only `--write` changes a file, through the
//! `write` module. `sdlc/scripts/policy.py` holds it to that.

mod table;
mod write;

use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Args, ValueEnum};

use crate::cli::measure::{BY, Cause, Refusal, lines, rule, write};
use crate::core::Pointer;
use crate::core::measure::answer::{self, Identity};
use crate::core::measure::audit::{self as grade, By, Pooled, Row, Settings};
use crate::core::measure::group;
use crate::core::measure::items::Matching;
use crate::core::measure::key::Key;
use crate::core::measure::optimize::Measure;
use crate::core::measure::rounded_line;
use crate::failure::Failure;

/// The command line of `audit`. Its help is on the `Audit` command.
#[derive(Args, Debug)]
pub(crate) struct AuditArguments {
    /// Saved answer lines, or - for standard input.
    results: PathBuf,
    /// The answer key as JSON lines, or - for standard input.
    key: PathBuf,
    /// Group the answers by question, by verb, or a JSON pointer such as /category into each line's input.
    #[arg(
        long,
        value_name = "question|verb|POINTER",
        default_value = "question",
        value_parser = group
    )]
    by: Group,
    /// Rescore each answer from its saved probabilities under this cut or band.
    #[arg(long, value_name = "T|LOW:HIGH", allow_negative_numbers = true)]
    threshold: Option<String>,
    /// The JSON pointer to the record id inside each line's input.
    #[arg(
        long,
        value_name = "POINTER",
        default_value = "/id",
        allow_hyphen_values = true
    )]
    id: String,
    /// The seed of the split and the bootstrap.
    #[arg(long, value_name = "N", default_value_t = 0)]
    seed: u64,
    /// The agreement a choose suggested cut must reach, from 0 to 1.
    #[arg(long, value_name = "A", default_value = "0.9", value_parser = target)]
    target: f64,
    /// The measure the suggested bar maximizes: accuracy, precision, recall, or f1.
    #[arg(long, value_enum, default_value = "accuracy")]
    optimize: Optimize,
    /// Write the steady bar into the question file or set the results came from.
    #[arg(long, value_name = "QUESTIONS")]
    write: Option<PathBuf>,
    /// How a recognize name matches a key name: the same places and kind, or overlapping places and the same kind.
    #[arg(long = "match", value_parser = ["strict", "overlap"], default_value = "strict")]
    matching: String,
    /// Add the coverage curve at every distinct confidence to each group.
    #[arg(long)]
    curve: bool,
    /// Add one last line with the calibration of every verb pooled.
    #[arg(long)]
    pooled: bool,
    /// Print the results for a person instead of JSON lines.
    #[arg(long)]
    table: bool,
}

#[derive(Clone, Debug)]
enum Group {
    Question,
    Verb,
    Field(String),
}

fn group(text: &str) -> Result<Group, String> {
    match text {
        "question" => Ok(Group::Question),
        "verb" => Ok(Group::Verb),
        _ if text.starts_with('/') => Ok(Group::Field(text.to_owned())),
        _ => Err(BY.to_owned()),
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Optimize {
    Accuracy,
    Precision,
    Recall,
    F1,
}

fn target(text: &str) -> Result<f64, String> {
    text.parse::<f64>()
        .ok()
        .filter(|value| (0.0..=1.0).contains(value))
        .ok_or_else(|| "the target is an agreement from 0 to 1".to_owned())
}

/// Grade, write the bar `--write` names and report it, then write JSON lines or the table.
pub(crate) fn run(arguments: &AuditArguments, writer: impl Write) -> Result<(), Failure> {
    let (rows, pooled, report) = grade_all(arguments).map_err(Failure::Measure)?;
    write(std::io::stderr().lock(), &report)?;
    let mut text = String::new();
    for row in &rows {
        if arguments.table {
            table::table(row, &mut text);
        } else {
            text.push_str(&rounded_line(row).map_err(Failure::Render)?);
            text.push('\n');
        }
    }
    if let Some(pooled) = &pooled {
        if arguments.table {
            text.push_str(&table::pooled_line(pooled));
        } else {
            text.push_str(&rounded_line(pooled).map_err(Failure::Render)?);
        }
        text.push('\n');
    }
    write(writer, &text)
}

/// The rows, the pooled line when asked, and the `--write` report.
type Graded = (Vec<Row>, Option<Pooled>, String);

fn grade_all(arguments: &AuditArguments) -> Result<Graded, Refusal> {
    let refusal = |role, cause| Refusal {
        command: "audit",
        role,
        cause,
    };
    let dash = Path::new("-");
    if let Some(path) = &arguments.write {
        let refused = [
            (path == dash, Cause::WriteDash),
            (arguments.threshold.is_some(), Cause::WriteThreshold),
            (matches!(arguments.by, Group::Verb), Cause::WriteByVerb),
            (
                matches!(arguments.by, Group::Field(_)),
                Cause::WriteByPointer,
            ),
        ];
        if let Some((_, cause)) = refused.into_iter().find(|(beside, _)| *beside) {
            return Err(refusal("question", cause));
        }
    }
    if arguments.curve && arguments.table {
        return Err(refusal("results", Cause::CurveTable));
    }
    if arguments.results == dash && arguments.key == dash {
        return Err(refusal("results", Cause::TwoStandardInputs));
    }
    let pointer =
        Pointer::new(arguments.id.as_str()).map_err(|_| refusal("results", Cause::Pointer))?;
    let (rule, shown) = rule("--threshold", arguments.threshold.as_deref())
        .map_err(|cause| refusal("results", cause))?;
    let results = lines(&arguments.results).map_err(|cause| refusal("results", cause))?;
    let key_lines = lines(&arguments.key).map_err(|cause| refusal("key", cause))?;
    let answers = answer::read(&results, &pointer, Identity::Question)
        .map_err(|e| refusal("results", Cause::Measure(e)))?;
    let mut key = Key::read(&key_lines).map_err(|e| refusal("key", Cause::Measure(e)))?;
    if arguments.matching == "overlap" {
        key.1 = Matching::Overlap;
    }
    let by = match &arguments.by {
        Group::Question => By::Question,
        Group::Verb => By::Verb,
        Group::Field(text) => {
            let field = Pointer::new(text).map_err(|_| refusal("results", Cause::ByPointer))?;
            By::Field(
                group::values(&results, &field)
                    .map_err(|e| refusal("results", Cause::Measure(e)))?,
            )
        }
    };
    let settings = Settings {
        by,
        rule,
        shown,
        seed: arguments.seed,
        target: arguments.target,
        optimize: match arguments.optimize {
            Optimize::Accuracy => Measure::Accuracy,
            Optimize::Precision => Measure::Precision,
            Optimize::Recall => Measure::Recall,
            Optimize::F1 => Measure::F1,
        },
        curve: arguments.curve,
    };
    let rows = grade::audit(&answers, &key, &settings)
        .map_err(|e| refusal("results", Cause::Measure(e)))?;
    if !key_lines.is_empty()
        && rows.iter().any(|row| row.rows > 0)
        && rows.iter().all(|row| row.labeled == 0)
    {
        return Err(refusal("key", Cause::NoneLabeled));
    }
    let report = match &arguments.write {
        Some(path) => {
            write::bars(path, &answers, &rows).map_err(|cause| refusal("question", cause))?
        }
        None => String::new(),
    };
    let pooled = arguments
        .pooled
        .then(|| grade::pooled(&answers, &key, &settings))
        .transpose()
        .map_err(|e| refusal("results", Cause::Measure(e)))?;
    Ok((rows, pooled, report))
}
