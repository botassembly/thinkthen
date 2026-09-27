//! `thinkthen diff`: show which saved answers changed between two runs or two cuts.
//!
//! The command reads the paths it is handed and nothing else, sends no
//! request, and reads no setting. `sdlc/scripts/policy.py` holds it to that.

use std::fmt::Write as _;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::Args;

use crate::cli::measure::{Cause, Match, Refusal, lines, rule, write};
use crate::core::Pointer;
use crate::core::Json;
use crate::core::measure::answer::{self, Identity};
use crate::core::measure::diff::{self as compare, Effect, ItemChange, Last, Row, Side, Summary};
use crate::core::measure::key::Key;
use crate::core::measure::{MeasureError, places, rounded, rounded_line};
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
    /// How a recognize name matches a name on the other side: the same places and kind, or overlapping places and the same kind.
    #[arg(long = "match", value_enum)]
    matching: Option<Match>,
    /// Print the results for a person instead of JSON lines.
    #[arg(long)]
    table: bool,
}

/// Compare, write JSON lines or the table and flush, then warn on standard error.
pub(crate) fn run(arguments: &DiffArguments, writer: impl Write) -> Result<(), Failure> {
    let (changes, summary) = compare_all(arguments).map_err(Failure::Measure)?;
    let text = if arguments.table {
        table(&changes, &summary)
    } else {
        let mut text = String::new();
        for change in &changes {
            text.push_str(&rounded_line(change).map_err(Failure::Render)?);
            text.push('\n');
        }
        text.push_str(&rounded_line(&Last { summary: &summary }).map_err(Failure::Render)?);
        text.push('\n');
        text
    };
    write(writer, &text)?;
    let mut stderr = std::io::stderr().lock();
    let _unwritten = stderr
        .write_all(warnings(&summary).as_bytes())
        .and_then(|()| stderr.flush());
    Ok(())
}

/// The warnings a result that exits 0 may still need. Neither changes standard output.
fn warnings(summary: &Summary) -> String {
    let mut out = String::new();
    if summary.records == 0 {
        out.push_str("thinkthen: diff: warning: no answer paired; check that both runs hold the same record ids and answer names\n");
    }
    if summary.digests_differ > 0 {
        let _ = writeln!(
            out,
            "thinkthen: diff: warning: the question digest differs in {} of {} paired answers. A different question, threshold, or profile gives a different digest.",
            summary.digests_differ, summary.records
        );
    }
    out
}

fn compare_all(arguments: &DiffArguments) -> Result<(Vec<Row>, Summary), Refusal> {
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
    let matching = arguments.matching.map(Into::into);
    compare::diff(a, b, key.as_ref(), [shown_a, shown_b], compared, matching).map_err(|cause| {
        let mixed = matches!(
            cause,
            MeasureError::PairsVerbs(_) | MeasureError::MixesVerbs(_) | MeasureError::MixesSets(_)
        );
        let role = if mixed && second.is_some() {
            "second run"
        } else {
            "first run"
        };
        refusal(role, Cause::Measure(cause))
    })
}

/// The prototype's table, word for word, and the item rows of `recognize` and `relate`.
fn table(rows: &[Row], summary: &Summary) -> String {
    let mut out = String::new();
    for row in rows {
        let change = match row {
            Row::Answer(change) => change,
            Row::Items(change) => {
                item_rows(&mut out, change);
                continue;
            }
        };
        let name = change
            .name
            .as_ref()
            .map_or_else(String::new, |name| format!("/{name}"));
        let [pa, pb] = change.probability.map(|p| places(p.map(rounded), 2));
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
    if let Some(items) = &summary.items {
        let _ = write!(
            out,
            "; items gained {}, lost {}, changed kind {}",
            items.items_gained, items.items_lost, items.items_changed_kind
        );
        let on = summary.mcnemar_on.unwrap_or_default();
        if let (Some(total), Some(a), Some(b), Some(xa), Some(xb)) = (
            items.key_items,
            summary.right_a,
            summary.right_b,
            items.extra_a,
            items.extra_b,
        ) {
            let _ = write!(out, "; {on} matched {a} -> {b} of {total}; extras {xa} -> {xb}");
        }
    } else if let (Some(gained), Some(lost), Some(right_a), Some(right_b), Some(labeled)) = (
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
        let _ = write!(out, "; McNemar p {} on {on}", places(Some(rounded(p)), 3));
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

/// One `recognize` or `relate` row: `ID[/NAME]  +G -L ~K`, then its gained, lost, and changed-kind items.
fn item_rows(out: &mut String, change: &ItemChange) {
    let name = change
        .name
        .as_ref()
        .map_or_else(String::new, |name| format!("/{name}"));
    let _ = writeln!(
        out,
        "{}{name}  +{} -{} ~{}",
        change.id,
        change.gained.len(),
        change.lost.len(),
        change.changed_kind.len()
    );
    for (sign, items) in [('+', &change.gained), ('-', &change.lost)] {
        for item in items {
            let _ = writeln!(out, "  {sign} {}", item_line(item));
        }
    }
    for moved in &change.changed_kind {
        let (from, to) = (item_text(&moved.from), item_text(&moved.to));
        let place = |item: &Json| [text(item, "start"), text(item, "end")];
        let to = if place(&moved.from) == place(&moved.to) {
            text(&moved.to, "kind")
        } else {
            to
        };
        let _ = writeln!(out, "  ~ {from} -> {to}");
    }
}

/// A name as `TEXT [START,END) KIND strength S`, or an edge as `RELATION SOURCE (KIND) -> TARGET (KIND) p P`.
fn item_line(item: &Json) -> String {
    let score = |name: &str, digits| {
        let value = match item.member(name) {
            Some(Json::Number(number)) => number.as_f64(),
            _ => None,
        };
        places(value.map(rounded), digits)
    };
    match item.member("relation") {
        Some(_) => format!("{} p {}", item_text(item), score("probability", 2)),
        None => format!("{} strength {}", item_text(item), score("strength", 4)),
    }
}

/// A name as `TEXT [START,END) KIND`, or an edge as `RELATION SOURCE (KIND) -> TARGET (KIND)`.
fn item_text(item: &Json) -> String {
    let end = |side: &str| {
        let side = item.member(side).unwrap_or(&Json::Null);
        format!("{} ({})", text(side, "name"), text(side, "kind"))
    };
    match item.member("relation") {
        Some(_) => format!(
            "{} {} -> {}",
            text(item, "relation"),
            end("source"),
            end("target")
        ),
        None => format!(
            "{} [{},{}) {}",
            text(item, "text"),
            text(item, "start"),
            text(item, "end"),
            text(item, "kind")
        ),
    }
}

/// A member as a table writes it: a string as is, anything else as JSON text.
fn text(item: &Json, name: &str) -> String {
    match item.member(name) {
        Some(Json::String(held)) => held.clone(),
        Some(Json::Number(number)) => number.to_string(),
        _ => "-".to_owned(),
    }
}
