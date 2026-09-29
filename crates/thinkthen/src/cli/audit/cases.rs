//! A JSONL projection of the answers the existing audit reader and key grade.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::core::Json;
use crate::core::measure::MeasureError;
use crate::core::measure::answer::{Answer, Rule, Said, Verb};
use crate::core::measure::items::{self, Item};
use crate::core::measure::key::{Key, Outcome, Want, outcome};

#[derive(Serialize)]
pub(super) struct Case {
    case: bool,
    line: usize,
    id: String,
    name: Option<String>,
    question: Option<String>,
    label: Option<String>,
    verb: &'static str,
    input: Option<Json>,
    options: Option<Vec<String>>,
    said: Option<Json>,
    truth: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_value: Option<Json>,
    outcome: &'static str,
    probability: Option<f64>,
    probabilities: Option<Json>,
    top_two: Option<Vec<Choice>>,
    item_counts: Option<ItemCounts>,
    usage: Option<Usage>,
    usage_scope: Option<&'static str>,
}

#[derive(Serialize)]
struct Choice {
    option: String,
    probability: f64,
}

#[derive(Serialize)]
struct ItemCounts {
    matched: usize,
    extra: usize,
    missed: usize,
}

#[derive(Serialize)]
struct Usage {
    input_tokens: u64,
    output_tokens: u64,
}

/// Project each already parsed answer from its original saved result line.
pub(super) fn collect(
    results: &[(usize, Json)],
    answers: &[Answer],
    key: &Key,
    rule: Rule,
) -> Result<Vec<Case>, MeasureError> {
    let by_line: BTreeMap<usize, &Json> = results.iter().map(|(line, row)| (*line, row)).collect();
    answers
        .iter()
        .map(|answer| {
            let row = by_line
                .get(&answer.line)
                .ok_or(MeasureError::Ungradable(answer.line))?;
            let entry = match &answer.name {
                Some(name) => row
                    .member("answers")
                    .and_then(|members| members.member(name))
                    .ok_or(MeasureError::Ungradable(answer.line))?,
                None => *row,
            };
            case(answer, row, entry, key, rule)
        })
        .collect()
}

fn case(
    answer: &Answer,
    row: &Json,
    entry: &Json,
    key: &Key,
    rule: Rule,
) -> Result<Case, MeasureError> {
    let options = options(answer.verb, entry);
    let probabilities = entry
        .member("answer")
        .and_then(|saved| saved.member("probabilities"))
        .filter(|saved| valid_probabilities(saved))
        .cloned();
    let top_two = (answer.verb == Verb::Choose)
        .then(|| top_two(options.as_deref(), probabilities.as_ref()))
        .flatten();
    let usage = usage(row);
    let (said, truth, key_value, outcome, item_counts) = graded(answer, key, rule)?;
    Ok(Case {
        case: true,
        line: answer.line,
        id: answer.id.clone(),
        name: answer.name.clone(),
        question: answer.text.clone(),
        label: answer.label.clone(),
        verb: answer.verb.name(),
        input: row.member("input").cloned(),
        options,
        said,
        truth,
        key_value,
        outcome,
        probability: answer.probability.map(|probability| probability.as_f64()),
        probabilities,
        top_two,
        item_counts,
        usage_scope: usage.as_ref().map(|_| "result_line"),
        usage,
    })
}

type Grade = (
    Option<Json>,
    Option<Json>,
    Option<Json>,
    &'static str,
    Option<ItemCounts>,
);

fn graded(answer: &Answer, key: &Key, rule: Rule) -> Result<Grade, MeasureError> {
    if answer.failed {
        return Ok((None, None, key.saved_value(answer).cloned(), "failed", None));
    }
    let Some(want) = key.want(answer)? else {
        let said = answer.said(rule).ok().and_then(shown);
        return Ok((said, None, None, "unlabeled", None));
    };
    if let Want::Items(ref wanted, matching) = want {
        let items = answer
            .items
            .as_ref()
            .ok_or(MeasureError::Ungradable(answer.line))?;
        let kept = items.kept(rule)?;
        let [matched, extra, missed] = items::tally(items, wanted, matching, rule)?;
        return Ok((
            Some(item_shape(answer.verb, kept)),
            key.saved_value(answer).cloned(),
            None,
            "items",
            Some(ItemCounts {
                matched,
                extra,
                missed,
            }),
        ));
    }
    let said = answer.said(rule)?;
    let result = match outcome(&said, &want) {
        Outcome::Right => "right",
        Outcome::Wrong => "wrong",
        Outcome::Unresolved => "unsure",
        Outcome::Tied => "tied",
    };
    Ok((shown(said), truth(want), None, result, None))
}

fn shown(said: Said<'_>) -> Option<Json> {
    match said {
        Said::Yes => Some(Json::Bool(true)),
        Said::No => Some(Json::Bool(false)),
        Said::Option(text) => Some(Json::String(text.to_owned())),
        Said::Unresolved | Said::Tied | Said::Items(_) => None,
    }
}

fn truth(want: Want) -> Option<Json> {
    match want {
        Want::Yes => Some(Json::Bool(true)),
        Want::No => Some(Json::Bool(false)),
        Want::Option(text) => Some(Json::String(text)),
        Want::Items(..) => None,
    }
}

fn item_shape(verb: Verb, kept: &[Item]) -> Json {
    let items = Json::Array(kept.iter().map(|item| item.printed.clone()).collect());
    if verb == Verb::Recognize {
        Json::Object(vec![("entities".to_owned(), items)])
    } else {
        items
    }
}

fn options(verb: Verb, entry: &Json) -> Option<Vec<String>> {
    let field = match verb {
        Verb::Choose => "options",
        Verb::Tag => "labels",
        Verb::Score => "levels",
        _ => return None,
    };
    match entry.member("question")?.member(field)? {
        Json::Array(items) => items
            .iter()
            .map(|item| item.as_str().map(str::to_owned))
            .collect(),
        Json::Object(members) => Some(members.iter().map(|(name, _)| name.clone()).collect()),
        _ => None,
    }
}

fn valid_probabilities(saved: &Json) -> bool {
    let Json::Object(members) = saved else {
        return false;
    };
    !members.is_empty()
        && members.iter().all(|(_, value)| {
            matches!(value, Json::Number(number) if number.as_f64().is_some_and(|p| (0.0..=1.0).contains(&p)))
        })
}

fn top_two(options: Option<&[String]>, distribution: Option<&Json>) -> Option<Vec<Choice>> {
    let options = options?;
    let Json::Object(members) = distribution? else {
        return None;
    };
    if options.len() < 2 || options.len() != members.len() {
        return None;
    }
    let mut ranked = Vec::with_capacity(options.len());
    for option in options {
        let Json::Number(number) = members
            .iter()
            .find(|(name, _)| name == option)
            .map(|(_, value)| value)?
        else {
            return None;
        };
        if ranked.iter().any(|held: &Choice| held.option == *option) {
            return None;
        }
        ranked.push(Choice {
            option: option.clone(),
            probability: number.as_f64()?,
        });
    }
    ranked.sort_by(|a, b| b.probability.total_cmp(&a.probability));
    ranked.truncate(2);
    Some(ranked)
}

fn usage(row: &Json) -> Option<Usage> {
    let saved = row.member("meta")?.member("usage")?;
    let count = |name| match saved.member(name)? {
        Json::Number(number) => number.as_u64(),
        _ => None,
    };
    Some(Usage {
        input_tokens: count("input_tokens")?,
        output_tokens: count("output_tokens")?,
    })
}
