//! `audit --table`: the prototype's table, word for word, and the lines later tickets added.

use crate::core::measure::audit::{Counts, Pooled, Row, Suggested};
use crate::core::measure::optimize::{Bar, Measure, Steady};
use crate::core::measure::{places, python_float_text, rounded};

/// A float as the table writes a number: three places, or `-` for none.
fn three(value: Option<f64>) -> String {
    places(value.map(rounded), 3)
}

/// The prototype's table for one group, word for word.
pub(super) fn table(row: &Row, out: &mut String) {
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
    let set = matches!(row.verb, Some("recognize" | "relate"));
    line(summary_line(row, set));
    if row.true_yes.is_some() && !set {
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
        line(suggested_line(row, suggested, set));
        if let Some(Some(steady)) = &suggested.steady {
            line(steady_line(steady, suggested.seed) + &crossed_line(suggested, set));
        }
        if let (true, Some(held)) = (row.true_yes.is_some(), &suggested.held) {
            line(held_line(&held.at_cut, set));
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

/// The first counts line: matched, extra, and missed for names and edges, or agreement for the rest.
fn summary_line(row: &Row, set: bool) -> String {
    let [low, high] = row
        .interval
        .map_or([None, None], |[l, h]| [Some(l), Some(h)]);
    if set {
        return format!(
            "  matched {}, extra {}, missed {}: precision {}   recall {}   f1 {}",
            row.right,
            row.false_yes.unwrap_or_default(),
            row.false_no.unwrap_or_default(),
            three(row.precision),
            three(row.yes_recall),
            three(row.f1)
        );
    }
    format!(
        "  agreement {} (95% {} to {}): {} right, {} wrong, {} not sure, {} tied{}",
        three(row.agreement),
        three(low),
        three(high),
        row.right,
        row.wrong,
        row.unsure,
        row.tied,
        ties_line(row)
    )
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
fn crossed_line(suggested: &Suggested, set: bool) -> String {
    let Some(Some(crossed)) = &suggested.crossed else {
        return String::new();
    };
    let [first, second] = crossed.cuts.map(|cut| python_float_text(rounded(cut)));
    let held = &crossed.held;
    let tail = if set {
        format!("f1 {}", three(held.f1))
    } else {
        let agreement = three(held.agreement);
        format!(
            "agreement {agreement}, {} right of {} answered",
            held.right, held.answered
        )
    };
    format!("\n  crossed: cuts {first} and {second}, each checked on the other part: {tail}")
}

/// The suggested cut's line, or the sentence that says no cut reaches the target.
fn suggested_line(row: &Row, suggested: &Suggested, set: bool) -> String {
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
    let (name, [run, at]) = if set {
        ("f1", [held.at_run.f1, held.at_cut.f1])
    } else {
        ("agreement", [held.at_run.agreement, held.at_cut.agreement])
    };
    let extra = if set {
        String::new()
    } else if row.true_yes.is_some() {
        let [run, at] = [&held.at_run, &held.at_cut].map(|c| three(c.yes_recall));
        format!(", yes recall {run} -> {at}")
    } else {
        let [run, at] = [&held.at_run, &held.at_cut].map(|c| three(c.coverage));
        format!(", coverage {run} -> {at}")
    };
    format!(
        "  suggested cut {} ({}; {} split, tuned on {}, checked on {} held out): held {name} {} as run -> {} at the cut{extra}",
        python_float_text(rounded(cut)),
        suggested.objective,
        suggested.split,
        tune.n,
        held.n,
        three(run),
        three(at)
    )
}

/// The four measures at the suggested cut on the held part.
fn held_line(at: &Counts, set: bool) -> String {
    let accuracy = if set {
        String::new()
    } else {
        format!("accuracy {}, ", three(Measure::Accuracy.of(at)))
    };
    format!(
        "  at the suggested cut on the held part: {accuracy}precision {}, recall {}, f1 {}",
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
pub(super) fn pooled_line(pooled: &Pooled) -> String {
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
