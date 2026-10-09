//! Said and keyed `recognize` names and `relate` edges, and the greedy match between them.

use crate::core::json::Json;
use crate::core::measure::MeasureError;
use crate::core::measure::answer::{Rule, Verb};
use crate::core::probability::Probability;
use crate::core::recognize::{ENTITY, RecognitionMode};

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
    /// A decoded boundary with no classified kind.
    Span(u64, u64),
    /// An edge: its relation, and its source and target as name and kind.
    Edge(String, [(String, String); 2]),
}

/// One said item: what it is, its strength or probability, whether its relation has no
/// direction, its zero-based place in the line's output, and its object as printed.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Item {
    pub(crate) what: What,
    score: f64,
    loose: bool,
    pub(crate) place: usize,
    pub(crate) printed: Json,
}

/// What one line said, strongest first, and the cut it ran with.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Items {
    said: Vec<Item>,
    /// The line's `question.threshold`.
    pub(crate) cut: f64,
    /// The recognition value shape, including empty proposal lists.
    pub(crate) mode: RecognitionMode,
}

impl Items {
    /// The items a rule keeps, strongest first: every item as run, or those at or above a single cut.
    ///
    /// # Errors
    ///
    /// Returns [`MeasureError::SetCut`] for a band or a cut below the line's run cut.
    pub(crate) fn kept(&self, rule: Rule) -> Result<&[Item], MeasureError> {
        let floor = match rule {
            Rule::Threshold(threshold) => threshold
                .cut_value()
                .filter(|cut| *cut >= self.cut)
                .ok_or(MeasureError::SetCut)?,
            Rule::AsRun | Rule::Levels(_) => 0.0,
        };
        let kept = self.said.partition_point(|item| item.score >= floor);
        Ok(self.said.get(..kept).unwrap_or_default())
    }
}

/// A JSON number as a float, or nothing.
pub(crate) fn number(value: Option<&Json>) -> Option<f64> {
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
    let mode = if relate {
        RecognitionMode::Whole
    } else {
        mode(question).ok_or(ungradable)?
    };
    let value = entry.member("value").ok_or(ungradable)?;
    if !relate && self::mode(value) != Some(mode) {
        return Err(ungradable);
    }
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
    let said = list(verb, mode, value).ok_or(ungradable)?;
    let mut items = Vec::new();
    for (place, held) in said.iter().enumerate() {
        let what = entity_rule(verb, &names, what(verb, mode, held).ok_or(ungradable)?);
        let score = number(
            held.member(if relate || mode == RecognitionMode::BoundaryOnly {
                "probability"
            } else {
                "strength"
            }),
        )
        .and_then(|p| Probability::new(p).ok())
        .ok_or(MeasureError::Probability(line))?
        .as_f64();
        let loose = matches!(&what, What::Edge(relation, _) if either.iter().any(|name| name.as_str() == Some(relation)));
        let printed = held.clone();
        items.push(Item {
            what,
            score,
            loose,
            place,
            printed,
        });
    }
    items.sort_by(|a, b| b.score.total_cmp(&a.score));
    let meta = entry.member("meta");
    let lost = number(meta.and_then(|meta| meta.member("failed_questions")));
    let partial = lost.is_some_and(|lost| lost > 0.0);
    Ok((
        Items {
            said: items,
            cut,
            mode,
        },
        names,
        partial,
    ))
}

/// The list of edges a `relate` value is, or of names under a `recognize` value's `entities`.
fn list(verb: Verb, mode: RecognitionMode, value: &Json) -> Option<&Vec<Json>> {
    match (verb, value) {
        (Verb::Relate, Json::Array(list)) => Some(list),
        (Verb::Relate, _) => None,
        _ => list(
            Verb::Relate,
            mode,
            value.member(if mode == RecognitionMode::BoundaryOnly {
                "proposals"
            } else {
                "entities"
            })?,
        ),
    }
}

fn mode(value: &Json) -> Option<RecognitionMode> {
    match value.member("mode") {
        None => Some(RecognitionMode::Whole),
        Some(Json::String(mode)) => RecognitionMode::parse(mode),
        _ => None,
    }
}

fn text(value: Option<&Json>) -> Option<String> {
    value.and_then(Json::as_str).map(str::to_owned)
}

/// A name or an edge read from one object of the command's value; other members are ignored.
fn what(verb: Verb, mode: RecognitionMode, held: &Json) -> Option<What> {
    if verb == Verb::Relate {
        let end = |side: &Json| Some((text(side.member("name"))?, text(side.member("kind"))?));
        let ends = [end(held.member("source")?)?, end(held.member("target")?)?];
        return Some(What::Edge(text(held.member("relation"))?, ends));
    }
    let place = |name: &str| match held.member(name)? {
        Json::Number(number) => number.as_u64(),
        _ => None,
    };
    if mode == RecognitionMode::BoundaryOnly {
        return Some(What::Span(place("start")?, place("end")?));
    }
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
    mode: RecognitionMode,
) -> Result<Vec<What>, MeasureError> {
    if verb == Verb::Recognize && self::mode(value) != Some(mode) {
        return Err(MeasureError::KeyItems(line));
    }
    list(verb, mode, value)
        .ok_or(MeasureError::KeyItems(line))?
        .iter()
        .map(|held| {
            let what = what(verb, mode, held).ok_or(MeasureError::KeyItems(line))?;
            let what = entity_rule(verb, names, what);
            let known = match &what {
                What::Span(..) => true,
                What::Name(named, ..) | What::Edge(named, _) => {
                    names.contains(named) || (verb == Verb::Recognize && names.is_empty())
                }
            };
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

/// Matched, extra, and missed under the rule.
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
    let kept = items.kept(rule)?;
    let wanted: Vec<&What> = key.iter().collect();
    let hit = pair(kept, &wanted, |item, want| matches(item, want, matching))
        .0
        .len();
    Ok([hit, kept.len() - hit, key.len() - hit])
}

/// The matched pairs, the unmatched said items, and the unmatched wanted items.
type Paired<'a, T> = (Vec<(&'a Item, T)>, Vec<&'a Item>, Vec<T>);

/// One match each: the said items are taken in order, and each takes the first
/// open wanted item, in order, that fits it.
pub(crate) fn pair<'a, T: Copy>(
    said: impl IntoIterator<Item = &'a Item>,
    wanted: &[T],
    fits: impl Fn(&Item, T) -> bool,
) -> Paired<'a, T> {
    let mut open: Vec<Option<T>> = wanted.iter().copied().map(Some).collect();
    let (mut matched, mut unmatched) = (Vec::new(), Vec::new());
    for item in said {
        let found = open.iter_mut().find_map(|slot| {
            slot.filter(|want| fits(item, *want))
                .and_then(|_| slot.take())
        });
        match found {
            Some(want) => matched.push((item, want)),
            None => unmatched.push(item),
        }
    }
    (matched, unmatched, open.into_iter().flatten().collect())
}

/// True when a said item matches a wanted name or edge under the rule.
pub(crate) fn matches(item: &Item, want: &What, matching: Matching) -> bool {
    match (&item.what, want) {
        (What::Name(kind, start, end), What::Name(key_kind, key_start, key_end)) => {
            kind == key_kind
                && match matching {
                    Matching::Strict => (start, end) == (key_start, key_end),
                    Matching::Overlap => start < key_end && key_start < end,
                }
        }
        (What::Span(start, end), What::Span(key_start, key_end)) => match matching {
            Matching::Strict => (start, end) == (key_start, key_end),
            Matching::Overlap => start < key_end && key_start < end,
        },
        (What::Edge(relation, [source, target]), What::Edge(key_relation, [from, to])) => {
            relation == key_relation
                && ((source, target) == (from, to)
                    || (item.loose && (target, source) == (from, to)))
        }
        _ => false,
    }
}

/// True when two names sit at places that match under the rule and differ in kind.
pub(crate) fn kind_changed(a: &Item, b: &Item, matching: Matching) -> bool {
    match (&a.what, &b.what) {
        (What::Name(kind, ..), What::Name(other, start, end)) => {
            kind != other && matches(a, &What::Name(kind.clone(), *start, *end), matching)
        }
        _ => false,
    }
}
