//! Pair two runs, or two cuts on one run, and say which answers changed.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::core::measure::answer::{Answer, Rule, Shown, Verb};
use crate::core::measure::key::{Key, Outcome, outcome};
use crate::core::measure::{MeasureError, mcnemar, six};

/// Which McNemar count a pair of outcomes adds to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Discordant {
    /// The first side was not right and the second is.
    OtherToRight,
    /// The first side was right and the second is not.
    RightToOther,
}

/// The one place the McNemar rule lives.
pub(crate) const fn discordant(a: Outcome, b: Outcome) -> Option<Discordant> {
    match (a, b) {
        (Outcome::Wrong, Outcome::Right) => Some(Discordant::OtherToRight),
        (Outcome::Right, Outcome::Wrong) => Some(Discordant::RightToOther),
        _ => None,
    }
}

/// What a changed answer did against the key.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Effect {
    /// Wrong became right.
    Gained,
    /// Right became wrong.
    Lost,
    /// Any other change between graded answers.
    Changed,
    /// Tied or not sure became right or wrong.
    Resolved,
    /// Right or wrong became tied or not sure.
    Withdrawn,
}

impl Effect {
    /// The effect as the output names it.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Gained => "gained",
            Self::Lost => "lost",
            Self::Changed => "changed",
            Self::Resolved => "resolved",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// The effect of a changed answer, from its two outcomes.
const fn effect(a: Outcome, b: Outcome) -> Effect {
    use Outcome::{Right, Tied, Unresolved, Wrong};
    match (a, b) {
        (Wrong, Right) => Effect::Gained,
        (Right, Wrong) => Effect::Lost,
        (Tied | Unresolved, Right | Wrong) => Effect::Resolved,
        (Right | Wrong, Tied | Unresolved) => Effect::Withdrawn,
        _ => Effect::Changed,
    }
}

/// One answer that changed between the two sides.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Change {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) from: String,
    pub(crate) to: String,
    #[serde(serialize_with = "six")]
    pub(crate) probability: [Option<f64>; 2],
    pub(crate) key: Option<String>,
    pub(crate) effect: Option<Effect>,
}

/// One kind of move and how many answers made it.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct Move {
    pub(crate) from: String,
    pub(crate) to: String,
    pub(crate) count: usize,
}

/// The counts over every pair, printed as the last line.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Summary {
    pub(crate) records: usize,
    pub(crate) changed: usize,
    pub(crate) only_a: usize,
    pub(crate) only_b: usize,
    pub(crate) moves: Vec<Move>,
    pub(crate) labeled: Option<usize>,
    pub(crate) right_a: Option<usize>,
    pub(crate) right_b: Option<usize>,
    pub(crate) gained: Option<usize>,
    pub(crate) lost: Option<usize>,
    pub(crate) mcnemar_on: Option<&'static str>,
    #[serde(serialize_with = "six")]
    pub(crate) mcnemar_p: Option<f64>,
    pub(crate) compare: &'static str,
    pub(crate) a: Shown,
    pub(crate) b: Shown,
}

/// The last output line, which holds the summary.
#[derive(Serialize)]
pub(crate) struct Last<'a> {
    pub(crate) summary: &'a Summary,
}

/// One side of the comparison: its answers and the rule they are read under.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Side<'a> {
    pub(crate) answers: &'a [Answer],
    pub(crate) rule: Rule,
}

/// The pair key: the answer name and the record id.
type Pair<'a> = (&'a Option<String>, &'a str);

/// Every answer that did not fail, by its pair key.
fn paired(answers: &[Answer]) -> BTreeMap<Pair<'_>, &Answer> {
    answers
        .iter()
        .filter(|answer| !answer.failed)
        .map(|answer| ((&answer.name, answer.id.as_str()), answer))
        .collect()
}

/// Running counts while the pairs are walked.
#[derive(Default)]
struct Tally {
    records: usize,
    labeled: usize,
    right: [usize; 2],
    effects: [usize; 2],
    discordant: [usize; 2],
    yes: [usize; 2],
    moves: BTreeMap<(String, String), usize>,
}

/// Compare side `a` with side `b`, which is `a` again for two cuts.
///
/// # Errors
///
/// Returns [`MeasureError`] for a rule an answer cannot take, or a
/// `choose` key value that is not text.
pub(crate) fn diff(
    a: Side<'_>,
    b: Side<'_>,
    key: Option<&Key>,
    shown: [Shown; 2],
    compare: &'static str,
) -> Result<(Vec<Change>, Summary), MeasureError> {
    let by_b = paired(b.answers);
    let seen_a = paired(a.answers);
    let mut tally = Tally::default();
    let mut changes = Vec::new();
    for x in a.answers.iter().filter(|answer| !answer.failed) {
        let Some(y) = by_b.get(&(&x.name, x.id.as_str())) else {
            continue;
        };
        tally.records += 1;
        let (from, to) = (x.said(a.rule)?, y.said(b.rule)?);
        let want = match key {
            Some(key) => key.want(x)?,
            None => None,
        };
        let outcomes = want
            .as_ref()
            .map(|want| (outcome(&from, want), outcome(&to, want)));
        if let Some((oa, ob)) = outcomes {
            tally.labeled += 1;
            tally.right[0] += usize::from(oa == Outcome::Right);
            tally.right[1] += usize::from(ob == Outcome::Right);
            match discordant(oa, ob) {
                Some(Discordant::OtherToRight) => tally.discordant[0] += 1,
                Some(Discordant::RightToOther) => tally.discordant[1] += 1,
                None => {}
            }
        }
        if from == to {
            continue;
        }
        let (from, to) = (from.text().to_owned(), to.text().to_owned());
        *tally.moves.entry((from.clone(), to.clone())).or_default() += 1;
        tally.yes[0] += usize::from(from == "yes" && to == "no");
        tally.yes[1] += usize::from(from == "no" && to == "yes");
        let effect = outcomes.map(|(oa, ob)| effect(oa, ob));
        tally.effects[0] += usize::from(effect == Some(Effect::Gained));
        tally.effects[1] += usize::from(effect == Some(Effect::Lost));
        changes.push(Change {
            id: x.id.clone(),
            name: x.name.clone(),
            from,
            to,
            probability: [x.confidence(), y.confidence()],
            key: want.as_ref().map(|want| want.text().to_owned()),
            effect,
        });
    }
    let only = |one: &BTreeMap<Pair<'_>, _>, other: &BTreeMap<Pair<'_>, _>| {
        one.keys().filter(|pair| !other.contains_key(*pair)).count()
    };
    let (only_a, only_b) = (only(&seen_a, &by_b), only(&by_b, &seen_a));
    let keyed = |count| key.map(|_| count);
    let (mcnemar_on, mcnemar_p) = if key.is_some() {
        let [to_right, from_right] = tally.discordant;
        (Some("right answers"), Some(mcnemar(to_right, from_right)))
    } else if a.answers.iter().all(|answer| answer.verb == Verb::Decide) {
        (
            Some("yes answers"),
            Some(mcnemar(tally.yes[0], tally.yes[1])),
        )
    } else {
        (None, None)
    };
    let [shown_a, shown_b] = shown;
    let summary = Summary {
        records: tally.records,
        changed: changes.len(),
        only_a,
        only_b,
        moves: sorted(tally.moves),
        labeled: keyed(tally.labeled),
        right_a: keyed(tally.right[0]),
        right_b: keyed(tally.right[1]),
        gained: keyed(tally.effects[0]),
        lost: keyed(tally.effects[1]),
        mcnemar_on,
        mcnemar_p,
        compare,
        a: shown_a,
        b: shown_b,
    };
    Ok((changes, summary))
}

/// The moves by count, largest first, then by `from` and `to` in code point order.
fn sorted(counts: BTreeMap<(String, String), usize>) -> Vec<Move> {
    let mut moves: Vec<Move> = counts
        .into_iter()
        .map(|((from, to), count)| Move { from, to, count })
        .collect();
    moves.sort_by(|m, n| {
        n.count
            .cmp(&m.count)
            .then_with(|| (&m.from, &m.to).cmp(&(&n.from, &n.to)))
    });
    moves
}
