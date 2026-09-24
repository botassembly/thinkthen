//! `thinkthen diff`: show which saved answers changed between two runs or two cuts.
//!
//! The command reads the paths it is handed and nothing else, sends no
//! request, and reads no setting. `sdlc/scripts/policy.py` holds it to that.

use std::fmt::Write as _;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use clap::Args;

use crate::cli::measure::{Cause, Refusal, lines, rule};
use crate::core::measure::answer::{self, Identity};
use crate::core::measure::diff::{self as compare, Change, Effect, Last, Side, Summary};
use crate::core::measure::key::Key;
use crate::core::measure::{rounded, three_places};
use crate::core::{Pointer, json_line};
use crate::failure::Failure;

/// The command line of `diff`. Its help is on the `Diff` command.
#[derive(Args, Debug)]
pub(crate) struct DiffArguments {
    /// The first run's saved answer lines, or - for standard input.
    a: PathBuf,
    /// The second run's saved answer lines, or - for standard input.
    b: Option<PathBuf>,
    /// An answer key as JSON lines, or - for standard input.
    #[arg(long, value_name = "KEY")]
    key: Option<PathBuf>,
    /// Rescore the first run from its saved probabilities under this cut or band.
    #[arg(long, value_name = "T|LOW:HIGH", allow_negative_numbers = true)]
    threshold: Option<String>,
    /// Rescore the second side under this cut or band instead of --threshold.
    #[arg(long, value_name = "T|LOW:HIGH", allow_negative_numbers = true)]
    compare_threshold: Option<String>,
    /// The JSON pointer to the record id inside each line's input.
    #[arg(
        long,
        value_name = "POINTER",
        default_value = "/id",
        allow_hyphen_values = true
    )]
    id: String,
    /// Print the results for a person instead of JSON lines.
    #[arg(long)]
    table: bool,
}

/// Compare, then write JSON lines or the table and flush.
pub(crate) fn run(arguments: &DiffArguments, mut writer: impl Write) -> Result<(), Failure> {
    let (changes, summary) = compare_all(arguments).map_err(Failure::Measure)?;
    let text = if arguments.table {
        table(&changes, &summary)
    } else {
        let mut text = String::new();
        for change in &changes {
            text.push_str(&json_line(change).map_err(Failure::Render)?);
            text.push('\n');
        }
        text.push_str(&json_line(&Last { summary: &summary }).map_err(Failure::Render)?);
        text.push('\n');
        text
    };
    match writer
        .write_all(text.as_bytes())
        .and_then(|()| writer.flush())
    {
        Err(error) if error.kind() != ErrorKind::BrokenPipe => Err(Failure::Output(error)),
        _ => Ok(()),
    }
}

fn compare_all(arguments: &DiffArguments) -> Result<(Vec<Change>, Summary), Refusal> {
    let refusal = |role, cause| Refusal {
        command: "diff",
        role,
        cause,
    };
    if arguments.b.is_none() && arguments.compare_threshold.is_none() {
        return Err(refusal("first run", Cause::NoSecondRun));
    }
    let inputs = [
        Some(&arguments.a),
        arguments.b.as_ref(),
        arguments.key.as_ref(),
    ];
    if inputs
        .iter()
        .flatten()
        .filter(|path| **path == Path::new("-"))
        .count()
        > 1
    {
        return Err(refusal("first run", Cause::TwoStandardInputs));
    }
    let pointer =
        Pointer::new(arguments.id.as_str()).map_err(|_| refusal("first run", Cause::Pointer))?;
    let (rule_a, shown_a) = rule("--threshold", arguments.threshold.as_deref())
        .map_err(|cause| refusal("first run", cause))?;
    let (rule_b, shown_b) = match arguments.compare_threshold.as_deref() {
        None => (rule_a, shown_a.clone()),
        Some(text) => {
            rule("--compare-threshold", Some(text)).map_err(|cause| refusal("second run", cause))?
        }
    };
    let key = match &arguments.key {
        None => None,
        Some(path) => Some(
            lines(path)
                .and_then(|read| Key::read(&read).map_err(Cause::Measure))
                .map_err(|cause| refusal("key", cause))?,
        ),
    };
    let run = |role, path: &Path| {
        lines(path)
            .and_then(|read| {
                answer::read(&read, &pointer, Identity::Answer).map_err(Cause::Measure)
            })
            .map_err(|cause| refusal(role, cause))
    };
    let first = run("first run", &arguments.a)?;
    let second = match &arguments.b {
        Some(path) => Some(run("second run", path)?),
        None => None,
    };
    let a = Side {
        answers: &first,
        rule: rule_a,
    };
    let b = Side {
        answers: second.as_deref().unwrap_or(&first),
        rule: rule_b,
    };
    let compared = if second.is_some() { "runs" } else { "cuts" };
    compare::diff(a, b, key.as_ref(), [shown_a, shown_b], compared)
        .map_err(|cause| refusal("first run", Cause::Measure(cause)))
}

/// A probability as the table writes it: two places, or `-` for none.
fn two(value: Option<f64>) -> String {
    value.map_or_else(|| "-".to_owned(), |value| format!("{value:.2}"))
}

/// The prototype's table, word for word.
fn table(changes: &[Change], summary: &Summary) -> String {
    let mut out = String::new();
    for change in changes {
        let name = change
            .name
            .as_ref()
            .map_or_else(String::new, |name| format!("/{name}"));
        let [pa, pb] = change.probability.map(|p| two(p.map(rounded)));
        let _ = write!(
            out,
            "{}{name}  {} -> {}  p {pa} -> {pb}",
            change.id, change.from, change.to
        );
        if let Some(key) = &change.key {
            let effect = change.effect.map_or("None", Effect::name);
            let _ = write!(out, "  key {key}: {effect}");
        }
        out.push('\n');
    }
    let (a, b) = (summary.a.text(), summary.b.text());
    if summary.compare == "cuts" {
        let _ = write!(out, "{a} -> {b}");
    } else if a == "as run" && b == "as run" {
        out.push_str("A -> B");
    } else {
        let _ = write!(out, "A -> B (at {a} and {b})");
    }
    let _ = write!(out, ": {} of {} changed", summary.changed, summary.records);
    for step in &summary.moves {
        let _ = write!(out, "; {} -> {} {}", step.from, step.to, step.count);
    }
    if let (Some(gained), Some(lost), Some(right_a), Some(right_b), Some(labeled)) = (
        summary.gained,
        summary.lost,
        summary.right_a,
        summary.right_b,
        summary.labeled,
    ) {
        let _ = write!(
            out,
            "; gained {gained}, lost {lost} ({right_a} -> {right_b} right of {labeled})"
        );
    }
    if let (Some(p), Some(on)) = (summary.mcnemar_p, summary.mcnemar_on) {
        let _ = write!(
            out,
            "; McNemar p {} on {on}",
            three_places(Some(rounded(p)))
        );
    }
    if summary.only_a > 0 || summary.only_b > 0 {
        let _ = write!(
            out,
            "; only in A {}, only in B {}",
            summary.only_a, summary.only_b
        );
    }
    out.push('\n');
    out
}
