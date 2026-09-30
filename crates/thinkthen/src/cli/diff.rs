//! `thinkthen diff`: show which saved answers changed between two runs or two cuts.
//!
//! The command reads the paths it is handed and nothing else, sends no
//! request, and reads no setting. `sdlc/scripts/policy.py` holds it to that.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::Args;

use crate::cli::measure::{Cause, Match, Refusal, lines, rule, write};
use crate::core::Pointer;
use crate::core::measure::MeasureError::{MixesSets, MixesVerbs, PairsVerbs};
use crate::core::measure::answer::{self, Identity};
use crate::core::measure::diff::{
    self as compare, Change, Effect, ItemChange, KindChange, Last, Row, Side, Summary,
};
use crate::core::measure::items::number;
use crate::core::measure::key::Key;
use crate::core::measure::{places, rounded, rounded_line};
use crate::core::{BatchSetting, Json};
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
    /// How a recognize name matches a name on the other side: the same places and kind, or overlapping places and the same kind. The default is strict.
    #[arg(long = "match", value_enum)]
    matching: Option<Match>,
    /// Print the results for a person instead of JSON lines.
    #[arg(long)]
    table: bool,
}

/// Compare, write JSON lines or the table and flush, then warn on standard error.
pub(crate) fn run(arguments: &DiffArguments, writer: impl Write) -> Result<(), Failure> {
    let (changes, summary, batch_settings) = compare_all(arguments).map_err(Failure::Measure)?;
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
    let mut notices = warnings(&summary);
    if batch_settings.len() > 1 {
        let _ = writeln!(
            notices,
            "thinkthen: diff: warning: the runs used different batch settings ({}); batching moves answers, so some changes may come from it",
            BatchSetting::listed(&batch_settings)
        );
    }
    let _unwritten = stderr
        .write_all(notices.as_bytes())
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

type Compared = (Vec<Row>, Summary, BTreeSet<BatchSetting>);

fn compare_all(arguments: &DiffArguments) -> Result<Compared, Refusal> {
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
                let settings = BatchSetting::in_results(&read)
                    .map_err(|(line, member)| Cause::BatchSetting(line, member))?;
                answer::read(&read, &pointer, Identity::Answer)
                    .map(|answers| (answers, settings))
                    .map_err(Cause::Measure)
            })
            .map_err(|cause| refusal(role, cause))
    };
    let (first, mut batch_settings) = run("first run", &arguments.a)?;
    let second = match &arguments.b {
        Some(path) => {
            let (answers, settings) = run("second run", path)?;
            batch_settings.extend(settings);
            Some(answers)
        }
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
    let (shown, matching) = ([shown_a, shown_b], arguments.matching.map(Into::into));
    compare::diff(a, b, key.as_ref(), shown, compared, matching)
        .map(|(rows, summary)| (rows, summary, batch_settings))
        .map_err(|cause| {
            let mixed = matches!(cause, PairsVerbs(_) | MixesVerbs(_) | MixesSets(_));
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
        let (Row::Answer(Change { name, .. }) | Row::Items(ItemChange { name, .. })) = row;
        let name = name
            .as_ref()
            .map_or_else(String::new, |name| format!("/{name}"));
        let change = match row {
            Row::Answer(change) => change,
            Row::Items(change) => {
                item_rows(&mut out, change, &name);
                continue;
            }
        };
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
        if let (Some(total), Some(on)) = (items.key_items, summary.mcnemar_on) {
            let _ = write!(
                out,
                "; {on} matched {} -> {} of {total}; extras {} -> {}",
                summary.right_a.unwrap_or_default(),
                summary.right_b.unwrap_or_default(),
                items.extra_a.unwrap_or_default(),
                items.extra_b.unwrap_or_default()
            );
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
fn item_rows(out: &mut String, change: &ItemChange, name: &str) {
    let (lost, gained) = (&change.lost, &change.gained);
    let _ = writeln!(
        out,
        "{}{name}  +{} -{} ~{}",
        change.id,
        gained.len(),
        lost.len(),
        change.changed_kind.len()
    );
    for (sign, item) in gained
        .iter()
        .map(|item| ('+', item))
        .chain(lost.iter().map(|item| ('-', item)))
    {
        let (label, member, digits) = match item.member("relation") {
            Some(_) => ("p", "probability", 2),
            None => ("strength", "strength", 4),
        };
        let score = places(number(item.member(member)).map(rounded), digits);
        let _ = writeln!(out, "  {sign} {} {label} {score}", item_text(item));
    }
    for KindChange { from, to } in &change.changed_kind {
        let place = |item: &Json| [text(item, "start"), text(item, "end")];
        let to = match place(from) == place(to) {
            true => text(to, "kind"),
            false => item_text(to),
        };
        let _ = writeln!(out, "  ~ {} -> {to}", item_text(from));
    }
}

/// A name as `TEXT [START,END) KIND`, or an edge as `RELATION SOURCE (KIND) -> TARGET (KIND)`.
fn item_text(item: &Json) -> String {
    let end = |side| {
        let side = item.member(side).unwrap_or(&Json::Null);
        format!("{} ({})", text(side, "name"), text(side, "kind"))
    };
    let [words, start, end_at, kind] =
        ["text", "start", "end", "kind"].map(|name| text(item, name));
    match item.member("relation") {
        Some(_) => format!(
            "{} {} -> {}",
            text(item, "relation"),
            end("source"),
            end("target")
        ),
        None => format!("{words} [{start},{end_at}) {kind}"),
    }
}

/// A member as a table writes it: a string as is, a number as its JSON text.
fn text(item: &Json, name: &str) -> String {
    match item.member(name) {
        Some(Json::String(held)) => held.clone(),
        Some(Json::Number(number)) => number.to_string(),
        _ => "-".to_owned(),
    }
}
