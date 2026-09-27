//! Pair two runs, or two cuts on one run, and say which answers changed.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::core::json::Json;
use crate::core::measure::answer::{Answer, Rule, Said, Shown, Verb};
use crate::core::measure::items::{Item, Matching, What, kind_changed, matches, pair};
use crate::core::measure::key::{Key, Outcome, Want, outcome};
use crate::core::measure::{MeasureError, mcnemar};

/// Which McNemar count a pair of outcomes adds to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Discordant {
    /// The first side was wrong, tied, or not sure, and the second is right.
    OtherToRight,
    /// The first side was right, and the second is wrong, tied, or not sure.
    RightToOther,
}

/// The one place the McNemar rule lives.
pub(crate) const fn discordant(a: Outcome, b: Outcome) -> Option<Discordant> {
    match (a, b) {
        (Outcome::Right, Outcome::Right) => None,
        (_, Outcome::Right) => Some(Discordant::OtherToRight),
        (Outcome::Right, _) => Some(Discordant::RightToOther),
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
    pub(crate) probability: [Option<f64>; 2],
    pub(crate) key: Option<String>,
    pub(crate) effect: Option<Effect>,
}

/// One changed row: an answer that moved, or a record whose names or edges changed.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum Row {
    Answer(Change),
    Items(ItemChange),
}

/// One `recognize` or `relate` record whose items or key matches changed.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct ItemChange {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) lost: Vec<Json>,
    pub(crate) gained: Vec<Json>,
    pub(crate) changed_kind: Vec<KindChange>,
    pub(crate) key: Option<KeyMatches>,
}

/// A name that kept its place and changed kind: A's item and B's.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct KindChange {
    pub(crate) from: Json,
    pub(crate) to: Json,
}

/// The key items each side matched, and each side's extras.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct KeyMatches {
    pub(crate) matched: [usize; 2],
    pub(crate) extra: [usize; 2],
}

/// The summary members only `recognize` and `relate` pairs print.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ItemCounts {
    pub(crate) key_items: Option<usize>,
    pub(crate) items_gained: usize,
    pub(crate) items_lost: usize,
    pub(crate) items_changed_kind: usize,
    pub(crate) extra_a: Option<usize>,
    pub(crate) extra_b: Option<usize>,
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
    pub(crate) mcnemar_p: Option<f64>,
    /// Present for `recognize` and `relate` pairs only.
    #[serde(flatten)]
    pub(crate) items: Option<ItemCounts>,
    pub(crate) compare: &'static str,
    pub(crate) a: Shown,
    pub(crate) b: Shown,
    /// Pairs whose two lines carry different question digests. It never prints in the summary.
    #[serde(skip)]
    pub(crate) digests_differ: usize,
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
    digests_differ: usize,
    moves: BTreeMap<(String, String), usize>,
    /// Items gained, lost, and changed in kind.
    items: [usize; 3],
    key_items: usize,
    extra: [usize; 2],
}

/// Compare side `a` with side `b`, which is `a` again for two cuts.
///
/// # Errors
///
/// Returns [`MeasureError`] for a rule an answer cannot take, a
/// `choose` key value that is not text, a verb mix, or `--match` over other verbs.
pub(crate) fn diff(
    a: Side<'_>,
    b: Side<'_>,
    key: Option<&Key>,
    shown: [Shown; 2],
    compare: &'static str,
    matching: Option<Matching>,
) -> Result<(Vec<Row>, Summary), MeasureError> {
    let by_b = paired(b.answers);
    let seen_a = paired(a.answers);
    let mut tally = Tally::default();
    let mut changes = Vec::new();
    let mut first = None;
    for x in a.answers.iter().filter(|answer| !answer.failed) {
        let Some(y) = by_b.get(&(&x.name, x.id.as_str())) else {
            continue;
        };
        // A diff holds one class of verb: `recognize`, `relate`, or the others, as the first pair.
        let class = x.verb.set().then_some(x.verb);
        match *first.get_or_insert(class) {
            _ if x.verb != y.verb && (x.verb.set() || y.verb.set()) => {
                Err(MeasureError::PairsVerbs(y.line))
            }
            held if held != class && held.is_some() && class.is_some() => {
                Err(MeasureError::MixesSets(y.line))
            }
            held if held != class => Err(MeasureError::MixesVerbs(y.line)),
            None if matching.is_some() => Err(MeasureError::MatchNotSet),
            _ => Ok(()),
        }?;
        tally.records += 1;
        tally.digests_differ += usize::from(matches!(
            (&x.digest, &y.digest),
            (Some(p), Some(q)) if p != q
        ));
        let sides = [x.said(a.rule)?, y.said(b.rule)?];
        changes.extend(match class {
            Some(_) => {
                items(x, y, sides, key, matching.unwrap_or_default(), &mut tally)?.map(Row::Items)
            }
            None => answer(x, y, sides, key, &mut tally)?.map(Row::Answer),
        });
    }
    let only = |one: &BTreeMap<Pair<'_>, _>, other: &BTreeMap<Pair<'_>, _>| {
        one.keys().filter(|pair| !other.contains_key(*pair)).count()
    };
    let (only_a, only_b) = (only(&seen_a, &by_b), only(&by_b, &seen_a));
    let keyed = |count| key.map(|_| count);
    let set = first.flatten();
    let (mcnemar_on, mcnemar_p) = if key.is_some() {
        let [to_right, from_right] = tally.discordant;
        let on = match set {
            Some(Verb::Recognize) => "key names",
            Some(_) => "key edges",
            None => "right answers",
        };
        (Some(on), Some(mcnemar(to_right, from_right)))
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
        items: set.map(|_| ItemCounts {
            key_items: keyed(tally.key_items),
            items_gained: tally.items[0],
            items_lost: tally.items[1],
            items_changed_kind: tally.items[2],
            extra_a: keyed(tally.extra[0]),
            extra_b: keyed(tally.extra[1]),
        }),
        compare,
        a: shown_a,
        b: shown_b,
        digests_differ: tally.digests_differ,
    };
    Ok((changes, summary))
}

/// One `decide` or `choose` pair against the key. Gives its row when the answer moved.
///
/// # Errors
///
/// Returns [`MeasureError`] for a key value the answer cannot take.
fn answer(
    x: &Answer,
    y: &Answer,
    [from, to]: [Said<'_>; 2],
    key: Option<&Key>,
    tally: &mut Tally,
) -> Result<Option<Change>, MeasureError> {
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
        return Ok(None);
    }
    let (from, to) = (from.text().to_owned(), to.text().to_owned());
    *tally.moves.entry((from.clone(), to.clone())).or_default() += 1;
    tally.yes[0] += usize::from(from == "yes" && to == "no");
    tally.yes[1] += usize::from(from == "no" && to == "yes");
    let effect = outcomes.map(|(oa, ob)| effect(oa, ob));
    tally.effects[0] += usize::from(effect == Some(Effect::Gained));
    tally.effects[1] += usize::from(effect == Some(Effect::Lost));
    Ok(Some(Change {
        id: x.id.clone(),
        name: x.name.clone(),
        from,
        to,
        probability: [x.confidence(), y.confidence()],
        key: want.as_ref().map(|want| want.text().to_owned()),
        effect,
    }))
}

/// Pair one record's kept items side against side, and each side against its key value.
/// Gives the record's row when an item was lost, gained, or changed in kind, or a key
/// item matched on one side only.
///
/// # Errors
///
/// Returns [`MeasureError::KeyUnknown`] for a key item either side's question lacks.
fn items(
    x: &Answer,
    y: &Answer,
    sides: [Said<'_>; 2],
    key: Option<&Key>,
    matching: Matching,
    tally: &mut Tally,
) -> Result<Option<ItemChange>, MeasureError> {
    let [from, to] = sides.map(|said| match said {
        Said::Items(items) => items,
        _ => &[],
    });
    let to_all: Vec<&Item> = to.iter().collect();
    let (_, unpaired_a, unpaired_b) = pair(from, &to_all, |a, b| matches(a, &b.what, matching));
    let (mut moved, lost, gained) =
        pair(unpaired_a, &unpaired_b, |a, b| kind_changed(a, b, matching));
    let (lost, gained) = (printed(lost), printed(gained));
    moved.sort_by_key(|(a, _)| a.place);
    let changed_kind: Vec<KindChange> = moved
        .into_iter()
        .map(|(a, b)| KindChange {
            from: a.printed.clone(),
            to: b.printed.clone(),
        })
        .collect();
    tally.items[0] += gained.len();
    tally.items[1] += lost.len();
    tally.items[2] += changed_kind.len();
    let wants = match key {
        Some(key) => [key.want(x)?, key.want(y)?],
        None => [None, None],
    };
    let key = match wants {
        [Some(Want::Items(want_a, _)), Some(Want::Items(want_b, _))] => {
            let (hit_a, hit_b) = (hits(from, &want_a, matching), hits(to, &want_b, matching));
            let only = |one: &[usize], other: &[usize]| {
                one.iter().filter(|place| !other.contains(place)).count()
            };
            let (only_a, only_b) = (only(&hit_a, &hit_b), only(&hit_b, &hit_a));
            let matched = [hit_a.len(), hit_b.len()];
            let extra = [from.len() - matched[0], to.len() - matched[1]];
            tally.labeled += 1;
            tally.key_items += want_a.len();
            let add = |[p, q]: [usize; 2], [r, t]: [usize; 2]| [p + r, q + t];
            tally.right = add(tally.right, matched);
            tally.effects = add(tally.effects, [only_b, only_a]);
            tally.discordant = add(tally.discordant, [only_b, only_a]);
            tally.extra = add(tally.extra, extra);
            Some((KeyMatches { matched, extra }, only_a + only_b))
        }
        _ => None,
    };
    let changed = key.is_some_and(|(_, one_side)| one_side > 0)
        || !(lost.is_empty() && gained.is_empty() && changed_kind.is_empty());
    Ok(changed.then(|| ItemChange {
        id: x.id.clone(),
        name: x.name.clone(),
        lost,
        gained,
        changed_kind,
        key: key.map(|(matches, _)| matches),
    }))
}

/// The places of the key items a side's kept items match, one match each.
fn hits(kept: &[Item], wanted: &[What], matching: Matching) -> Vec<usize> {
    let places: Vec<usize> = (0..wanted.len()).collect();
    let fits = |item: &Item, place| {
        wanted
            .get(place)
            .is_some_and(|want| matches(item, want, matching))
    };
    pair(kept, &places, fits)
        .0
        .into_iter()
        .map(|(_, place)| place)
        .collect()
}

/// Items as printed, in their line's output order.
fn printed(mut items: Vec<&Item>) -> Vec<Json> {
    items.sort_by_key(|item| item.place);
    items.into_iter().map(|item| item.printed.clone()).collect()
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

#[cfg(test)]
#[path = "diff_tests.rs"]
mod tests;
