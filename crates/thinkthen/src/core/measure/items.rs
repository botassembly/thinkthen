//! Said and keyed `recognize` names and `relate` edges, and the greedy match between them.

use crate::core::json::Json;
use crate::core::measure::MeasureError;
use crate::core::measure::answer::{Rule, Verb};
use crate::core::probability::Probability;
use crate::core::recognize::ENTITY;

/// How a said name matches a key name.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum Matching {
    /// The same start, end, and kind.
    #[default]
    Strict,
    /// The same kind, and places that overlap.
    Overlap,
}

/// One name or edge, as said or as keyed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum What {
    /// A name: its kind, start, and exclusive end.
    Name(String, u64, u64),
    /// An edge: its relation, and its source and target as name and kind.
    Edge(String, [(String, String); 2]),
}

/// One said item: what it is, its strength or probability, and whether its relation has no direction.
type Item = (What, f64, bool);

/// What one line said, strongest first, and the cut it ran with.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Items {
    said: Vec<Item>,
    /// The line's `question.threshold`.
    pub(crate) cut: f64,
}

/// A JSON number as a float, or nothing.
fn number(value: Option<&Json>) -> Option<f64> {
    match value? {
        Json::Number(number) => number.as_f64(),
        _ => None,
    }
}

/// The said items of a line, its kinds or relation names, and whether a `relate` line lost questions.
///
/// # Errors
///
/// [`MeasureError::Ungradable`] for a line without its run cut, kinds or relations, or a
/// value of the command's shape; [`MeasureError::Probability`] for a score outside 0 to 1.
pub(crate) fn read(
    line: usize,
    verb: Verb,
    entry: &Json,
) -> Result<(Items, Vec<String>, bool), MeasureError> {
    let ungradable = MeasureError::Ungradable(line);
    let question = entry.member("question").ok_or(ungradable)?;
    let cut = number(question.member("threshold")).ok_or(ungradable)?;
    let relate = verb == Verb::Relate;
    let (names, either): (Vec<String>, Vec<&Json>) =
        match question.member(if relate { "relations" } else { "kinds" }) {
            Some(Json::Array(rules)) if relate => (
                rules
                    .iter()
                    .map(|rule| text(rule.member("name")))
                    .collect::<Option<_>>()
                    .ok_or(ungradable)?,
                rules
                    .iter()
                    .filter(|rule| rule.member("either") == Some(&Json::Bool(true)))
                    .filter_map(|rule| rule.member("name"))
                    .collect(),
            ),
            Some(Json::Object(kinds)) if !relate => (
                kinds.iter().map(|(kind, _)| kind.clone()).collect(),
                Vec::new(),
            ),
            _ => return Err(ungradable),
        };
    let said = entry
        .member("value")
        .and_then(|value| list(verb, value))
        .ok_or(ungradable)?;
    let mut items = Vec::new();
    for held in said {
        let what = entity_rule(verb, &names, what(verb, held).ok_or(ungradable)?);
        let score = number(held.member(if relate { "probability" } else { "strength" }))
            .and_then(|p| Probability::new(p).ok())
            .ok_or(MeasureError::Probability(line))?
            .as_f64();
        let loose = matches!(&what, What::Edge(relation, _) if either.iter().any(|name| name.as_str() == Some(relation)));
        items.push((what, score, loose));
    }
    items.sort_by(|a, b| b.1.total_cmp(&a.1));
    let meta = entry.member("meta");
    let lost = number(meta.and_then(|meta| meta.member("failed_questions")));
    let partial = lost.is_some_and(|lost| lost > 0.0);
    Ok((Items { said: items, cut }, names, partial))
}

/// The list of edges a `relate` value is, or of names under a `recognize` value's `entities`.
fn list(verb: Verb, value: &Json) -> Option<&Vec<Json>> {
    match (verb, value) {
        (Verb::Relate, Json::Array(list)) => Some(list),
        (Verb::Relate, _) => None,
        _ => list(Verb::Relate, value.member("entities")?),
    }
}

fn text(value: Option<&Json>) -> Option<String> {
    value.and_then(Json::as_str).map(str::to_owned)
}

/// A name or an edge read from one object of the command's value; other members are ignored.
fn what(verb: Verb, held: &Json) -> Option<What> {
    if verb == Verb::Relate {
        let end = |side: &Json| Some((text(side.member("name"))?, text(side.member("kind"))?));
        let ends = [end(held.member("source")?)?, end(held.member("target")?)?];
        return Some(What::Edge(text(held.member("relation"))?, ends));
    }
    let place = |name: &str| match held.member(name)? {
        Json::Number(number) => number.as_u64(),
        _ => None,
    };
    Some(What::Name(
        text(held.member("kind"))?,
        place("start")?,
        place("end")?,
    ))
}

/// The key's items for a line whose question names these kinds or relations.
///
/// # Errors
///
/// Returns [`MeasureError::KeyItems`] for a value unlike the command's own, and
/// [`MeasureError::KeyUnknown`] for a kind or relation the question lacks.
pub(crate) fn key(
    line: usize,
    verb: Verb,
    value: &Json,
    names: &[String],
) -> Result<Vec<What>, MeasureError> {
    list(verb, value)
        .ok_or(MeasureError::KeyItems(line))?
        .iter()
        .map(|held| {
            let what = what(verb, held).ok_or(MeasureError::KeyItems(line))?;
            let what = entity_rule(verb, names, what);
            let (What::Name(named, ..) | What::Edge(named, _)) = &what;
            let known = names.contains(named) || (verb == Verb::Recognize && names.is_empty());
            known.then_some(what).ok_or(MeasureError::KeyUnknown(line))
        })
        .collect()
}

/// A `recognize` line with an empty kind set grades every said and key name as `ENTITY`.
fn entity_rule(verb: Verb, names: &[String], what: What) -> What {
    match what {
        What::Name(_, start, end) if verb == Verb::Recognize && names.is_empty() => {
            What::Name(ENTITY.to_owned(), start, end)
        }
        held => held,
    }
}

/// Matched, extra, and missed under the rule. Said items are taken strongest
/// first, and each takes the first unmatched key item it matches.
///
/// # Errors
///
/// Returns [`MeasureError::SetCut`] for a band or a cut below the line's run cut.
pub(crate) fn tally(
    items: &Items,
    key: &[What],
    matching: Matching,
    rule: Rule,
) -> Result<[usize; 3], MeasureError> {
    let floor = match rule {
        Rule::Threshold(threshold) => threshold
            .cut_value()
            .filter(|cut| *cut >= items.cut)
            .ok_or(MeasureError::SetCut)?,
        Rule::AsRun | Rule::Levels(_) => 0.0,
    };
    let mut open = vec![true; key.len()];
    let [mut hit, mut extra] = [0, 0];
    for item in items.said.iter().filter(|item| item.1 >= floor) {
        let mut found = key.iter().zip(open.iter_mut());
        if let Some((_, open)) = found.find(|(want, open)| **open && matches(item, want, matching))
        {
            *open = false;
            hit += 1;
        } else {
            extra += 1;
        }
    }
    Ok([hit, extra, key.len() - hit])
}

fn matches((what, _, loose): &Item, want: &What, matching: Matching) -> bool {
    match (what, want) {
        (What::Name(kind, start, end), What::Name(key_kind, key_start, key_end)) => {
            kind == key_kind
                && match matching {
                    Matching::Strict => (start, end) == (key_start, key_end),
                    Matching::Overlap => start < key_end && key_start < end,
                }
        }
        (What::Edge(relation, [source, target]), What::Edge(key_relation, [from, to])) => {
            relation == key_relation
                && ((source, target) == (from, to) || (*loose && (target, source) == (from, to)))
        }
        _ => false,
    }
}
