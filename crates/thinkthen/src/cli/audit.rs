//! `thinkthen audit`: grade saved answers against an answer key and suggest a bar.
//!
//! The command reads the paths it is handed and nothing else, sends no
//! request, and reads no setting. Only `--write` changes a file, through the
//! `write` module. `sdlc/scripts/policy.py` holds it to that.

mod write;

use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Args, ValueEnum};

use crate::cli::measure::{BY, Cause, Refusal, lines, rule, write};
use crate::core::Pointer;
use crate::core::measure::answer::{self, Identity};
use crate::core::measure::audit::{self as grade, By, Counts, Pooled, Row, Settings, Suggested};
use crate::core::measure::group;
use crate::core::measure::key::Key;
use crate::core::measure::optimize::{Bar, Measure, Steady};
use crate::core::measure::{places, python_float_text, rounded, rounded_line};
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
            table(row, &mut text);
        } else {
            text.push_str(&rounded_line(row).map_err(Failure::Render)?);
            text.push('\n');
        }
    }
    if let Some(pooled) = &pooled {
        if arguments.table {
            text.push_str(&pooled_line(pooled));
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
    let key = Key::read(&key_lines).map_err(|e| refusal("key", Cause::Measure(e)))?;
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
        "  agreement {} (95% {} to {}): {} right, {} wrong, {} unresolved, {} tied{}",
        three(row.agreement),
        three(low),
        three(high),
        row.right,
        row.wrong,
        row.unresolved,
        row.tied,
        ties_line(row)
    ));
    if row.true_yes.is_some() {
        line(format!(
            "  said yes, key no: {}   said no, key yes: {}   yes recall {}   mean p(yes) {}   AUC {}",
            row.false_yes.unwrap_or_default(),
            row.false_no.unwrap_or_default(),
            three(row.yes_recall),
            three(row.mean_probability),
            three(row.auc)
        ));
        line(format!(
            "  precision {}   f1 {}",
            three(row.precision),
            three(row.f1)
        ));
    } else if let Some(pairs) = row.disagreements.as_ref().filter(|pairs| !pairs.is_empty()) {
        let pairs: Vec<String> = pairs
            .iter()
            .map(|d| format!("key {} said {} x{}", d.key, d.said, d.count))
            .collect();
        line(format!("  disagreements: {}", pairs.join(", ")));
    }
    let added = [
        ("r-precision", row.r_precision),
        ("mean level distance", row.mean_level_distance),
    ];
    for (name, value) in added.into_iter().filter(|(_, value)| value.is_some()) {
        line(format!("  {name} {}", three(value)));
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
    if let Some(suggested) = &row.suggested {
        line(suggested_line(row, suggested));
        if let Some(Some(steady)) = &suggested.steady {
            line(steady_line(steady, suggested.seed) + &crossed_line(suggested));
        }
        if let (true, Some(held)) = (row.true_yes.is_some(), &suggested.held) {
            line(held_line(&held.at_cut));
        }
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

/// The ties line of a `choose` or `find` row with ties, after a line break, or nothing.
fn ties_line(row: &Row) -> String {
    let ties = row
        .tied_holding_key
        .zip(row.tie_share)
        .filter(|_| row.tied > 0);
    ties.map_or_else(String::new, |(holding, share)| {
        format!(
            "\n  ties holding the key: {holding} of {}, share {}",
            row.tied,
            three(Some(share))
        )
    })
}

/// The line of `suggested.crossed`, after a line break, or nothing.
fn crossed_line(suggested: &Suggested) -> String {
    let Some(Some(crossed)) = &suggested.crossed else {
        return String::new();
    };
    let [first, second] = crossed.cuts.map(|cut| python_float_text(rounded(cut)));
    format!(
        "\n  crossed: cuts {first} and {second}, each checked on the other part: agreement {}, {} right of {} answered",
        three(crossed.held.agreement),
        crossed.held.right,
        crossed.held.answered
    )
}

/// The suggested cut's line, or the sentence that says no cut reaches the target.
fn suggested_line(row: &Row, suggested: &Suggested) -> String {
    if let (Some(cuts), Some(tune), Some(held)) = (suggested.cuts, &suggested.tune, &suggested.held)
    {
        return format!(
            "  suggested level cuts {} ({}; {} split, tuned on {}, checked on {} held out): held exact levels {} as run -> {} at the cuts",
            bar_text(Bar::Levels(cuts)),
            suggested.objective,
            suggested.split,
            tune.n,
            held.n,
            held.at_run.right,
            held.at_cut.right
        );
    }
    let (Some(cut), Some(tune), Some(held)) = (suggested.cut, &suggested.tune, &suggested.held)
    else {
        // No steady count means the measure does not apply to this verb.
        if suggested.steady.is_none() {
            return format!("  suggested cut: none; {}", suggested.objective);
        }
        return format!("  suggested cut: none reaches {}", suggested.objective);
    };
    let extra = if row.true_yes.is_some() {
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

/// The four measures at the suggested cut on the held part.
fn held_line(at: &Counts) -> String {
    format!(
        "  at the suggested cut on the held part: accuracy {}, precision {}, recall {}, f1 {}",
        three(Measure::Accuracy.of(at)),
        three(at.precision),
        three(at.yes_recall),
        three(at.f1)
    )
}

/// A bar as the table writes it: a cut, or level cuts joined by commas.
fn bar_text(bar: Bar) -> String {
    let hundredths = match bar {
        Bar::Cut(k) => vec![k],
        Bar::Levels(cuts) => cuts.hundredths().to_vec(),
    };
    let texts: Vec<String> = hundredths
        .iter()
        .map(|k| python_float_text(f64::from(*k) / 100.0))
        .collect();
    texts.join(", ")
}

/// How steady the bar stays, and the seed or the key split behind it.
fn steady_line(steady: &Steady, seed: Option<u64>) -> String {
    let count = |bar: Bar| {
        steady
            .counts
            .iter()
            .find(|tally| tally.cut == bar)
            .map_or(0, |tally| tally.count)
    };
    let low = steady.counts.first().map_or(steady.cut, |tally| tally.cut);
    let high = steady.counts.last().map_or(steady.cut, |tally| tally.cut);
    let seed = seed.map_or_else(|| "key split".to_owned(), |seed| format!("seed {seed}"));
    let bracket = |bar: Bar| match bar {
        Bar::Cut(_) => bar_text(bar),
        Bar::Levels(_) => format!("[{}]", bar_text(bar)),
    };
    format!(
        "  steady: {} on {} of {} splits, range {} to {}; beat the run's rule on {} of {} held parts ({seed})",
        bracket(steady.cut),
        count(steady.cut),
        steady.splits,
        bracket(low),
        bracket(high),
        steady.better,
        steady.splits
    )
}

/// The pooled line as the table writes it.
fn pooled_line(pooled: &Pooled) -> String {
    let [error, low, high] = pooled.calibration.as_ref().map_or([None; 3], |c| {
        [Some(c.error), Some(c.interval[0]), Some(c.interval[1])]
    });
    format!(
        "  every verb pooled: calibration error {} (95% {} to {}) over {} answers",
        three(error),
        three(low),
        three(high),
        pooled.answers
    )
}
