//! `thinkthen audit`: grade saved `decide` and `choose` answers against an answer key.
//!
//! The command reads the two paths it is handed and nothing else, sends no
//! request, and reads no setting. `sdlc/scripts/policy.py` holds it to that.

use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Args, ValueEnum};

use crate::cli::measure::{Cause, Refusal, lines, rule, write};
use crate::core::Pointer;
use crate::core::measure::answer::{self, Identity};
use crate::core::measure::audit::{self as grade, By, Row, Settings, Suggested};
use crate::core::measure::key::Key;
use crate::core::measure::{places, python_float_text, rounded, rounded_line};
use crate::failure::Failure;

/// The command line of `audit`. Its help is on the `Audit` command.
#[derive(Args, Debug)]
pub(crate) struct AuditArguments {
    /// Saved answer lines, or - for standard input.
    results: PathBuf,
    /// The answer key as JSON lines, or - for standard input.
    key: PathBuf,
    /// Group the answers by question or by verb.
    #[arg(long, value_enum, default_value = "question")]
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
    /// Print the results for a person instead of JSON lines.
    #[arg(long)]
    table: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Group {
    Question,
    Verb,
}

fn target(text: &str) -> Result<f64, String> {
    text.parse::<f64>()
        .ok()
        .filter(|value| (0.0..=1.0).contains(value))
        .ok_or_else(|| "the target is an agreement from 0 to 1".to_owned())
}

/// Grade, then write JSON lines or the table and flush.
pub(crate) fn run(arguments: &AuditArguments, writer: impl Write) -> Result<(), Failure> {
    let rows = grade_all(arguments).map_err(Failure::Measure)?;
    let mut text = String::new();
    for row in &rows {
        if arguments.table {
            table(row, &mut text);
        } else {
            text.push_str(&rounded_line(row).map_err(Failure::Render)?);
            text.push('\n');
        }
    }
    write(writer, &text)
}

fn grade_all(arguments: &AuditArguments) -> Result<Vec<Row>, Refusal> {
    let refusal = |role, cause| Refusal {
        command: "audit",
        role,
        cause,
    };
    let dash = Path::new("-");
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
    let key = Key::read(&key_lines).map_err(|e| refusal("key", Cause::Measure(e)))?;
    let settings = Settings {
        by: match arguments.by {
            Group::Question => By::Question,
            Group::Verb => By::Verb,
        },
        rule,
        shown,
        seed: arguments.seed,
        target: arguments.target,
    };
    grade::audit(&answers, &key, &settings).map_err(|e| refusal("results", Cause::Measure(e)))
}

/// A float as the table writes a number: three places, or `-` for none.
fn three(value: Option<f64>) -> String {
    places(value.map(rounded), 3)
}

/// The prototype's table for one group, word for word.
fn table(row: &Row, out: &mut String) {
    let mut line = |text: String| {
        out.push_str(&text);
        out.push('\n');
    };
    let verb = row.verb.unwrap_or("None");
    line(format!(
        "{}  ({verb}, {} rows, {} labeled, {} failed, rule {})",
        row.group,
        row.rows,
        row.labeled,
        row.failed,
        row.threshold.text()
    ));
    let [low, high] = row
        .interval
        .map_or([None, None], |[l, h]| [Some(l), Some(h)]);
    line(format!(
        "  agreement {} (95% {} to {}): {} right, {} wrong, {} unresolved, {} tied",
        three(row.agreement),
        three(low),
        three(high),
        row.right,
        row.wrong,
        row.unresolved,
        row.tied
    ));
    if row.verb == Some("decide") {
        line(format!(
            "  said yes, key no: {}   said no, key yes: {}   yes recall {}   mean p(yes) {}   AUC {}",
            row.false_yes.unwrap_or_default(),
            row.false_no.unwrap_or_default(),
            three(row.yes_recall),
            three(row.mean_probability),
            three(row.auc)
        ));
    } else if let Some(pairs) = row.disagreements.as_ref().filter(|pairs| !pairs.is_empty()) {
        let pairs: Vec<String> = pairs
            .iter()
            .map(|d| format!("key {} said {} x{}", d.key, d.said, d.count))
            .collect();
        line(format!("  disagreements: {}", pairs.join(", ")));
    }
    if let Some(calibration) = &row.calibration {
        let [low, high] = calibration.interval;
        line(format!(
            "  calibration error {} (95% {} to {}); {}",
            three(Some(calibration.error)),
            three(Some(low)),
            three(Some(high)),
            calibration.note
        ));
    }
    if let Some(text) = row
        .suggested
        .as_ref()
        .map(|suggested| suggested_line(row, suggested))
    {
        line(text);
    }
    for (place, point) in row.coverage.iter().flatten().enumerate() {
        if place == 0 {
            line(format!(
                "  {:>5}  {:<10} {:>8} {:>8} {:>8}",
                "cut", "threshold", "answered", "coverage", "accuracy"
            ));
        }
        line(format!(
            "  {:>5}  {:<10} {:>8} {:>8} {:>8}",
            python_float_text(rounded(point.cut)),
            point.threshold.text(),
            point.answered,
            three(point.coverage),
            three(point.accuracy)
        ));
    }
}

/// The suggested cut's line, or the sentence that says no cut reaches the target.
fn suggested_line(row: &Row, suggested: &Suggested) -> String {
    let (Some(cut), Some(tune), Some(held)) = (suggested.cut, &suggested.tune, &suggested.held)
    else {
        return format!("  suggested cut: none reaches {}", suggested.objective);
    };
    let extra = if row.verb == Some("decide") {
        let [run, at] = [&held.at_run, &held.at_cut].map(|c| three(c.yes_recall));
        format!(", yes recall {run} -> {at}")
    } else {
        let [run, at] = [&held.at_run, &held.at_cut].map(|c| three(c.coverage));
        format!(", coverage {run} -> {at}")
    };
    format!(
        "  suggested cut {} ({}; {} split, tuned on {}, checked on {} held out): held agreement {} as run -> {} at the cut{extra}",
        python_float_text(rounded(cut)),
        suggested.objective,
        suggested.split,
        tune.n,
        held.n,
        three(held.at_run.agreement),
        three(held.at_cut.agreement)
    )
}
