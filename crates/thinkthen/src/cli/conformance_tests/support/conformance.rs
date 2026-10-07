//! Fixture-only shapes and checks kept apart from production semantics.

use crate::core::{Answer, FindAnswer, Value};
use serde::Deserialize;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::value::RawValue;
use std::fmt;

#[derive(Deserialize)]
pub(super) struct Document {
    schema: String,
    pub(super) backend_url: String,
    case_count: usize,
    error_kinds: Vec<String>,
    pub(super) cases: Vec<Case>,
}

impl Document {
    pub(super) fn validate_header(&self, verbs: &[&str], errors: &[&str]) -> Result<(), String> {
        if self.schema != "thinkthen.conformance/1"
            || self.backend_url != "https://api.typesafe.ai/v1/systemone"
            || self.case_count != self.cases.len()
        {
            return Err("schema or declared case count is wrong".to_owned());
        }
        let mut declared = self
            .error_kinds
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        declared.sort_unstable();
        if declared != errors || verbs.len() != 8 {
            return Err("declared error kinds are wrong".to_owned());
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Case {
    pub(super) id: String,
    provenance: Option<CaseProvenance>,
    pub(super) verb: String,
    pub(super) question: Option<Box<RawValue>>,
    pub(super) question_set: Option<Box<RawValue>>,
    /// The one record an annotate case asks, when its set reads parts of it.
    pub(super) record: Option<Box<RawValue>>,
    pub(super) question_form: Option<QuestionForm>,
    pub(super) text: Option<String>,
    pub(super) evidence: Option<String>,
    pub(super) entities: Option<Box<RawValue>>,
    pub(super) operation: Option<Injection>,
    pub(super) exchanges: Vec<Exchange>,
    #[serde(rename = "runtime_exchanges")]
    _runtime_exchanges: Option<Vec<Box<RawValue>>>,
    pub(super) expect: Expect,
}

/// Where a case numbered 26 or above came from, and any ruling it waits on.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CaseProvenance {
    branch: Option<String>,
    pending: Option<String>,
}

/// How a rule-breaking question reaches a surface: as JSON text or as a named file.
#[derive(Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(super) enum QuestionForm {
    Text,
    File,
}

impl Case {
    pub(super) fn validate_shape(&self) -> Result<(), String> {
        let named = self
            .provenance
            .as_ref()
            .is_some_and(|held| held.branch.is_some() || held.pending.is_some());
        let number = self
            .id
            .split('-')
            .next()
            .and_then(|lead| lead.parse::<u32>().ok())
            .ok_or_else(|| format!("{} has no leading number", self.id))?;
        if number >= 26 && !named {
            return Err(format!("{} names no provenance", self.id));
        }
        if (self.verb == "recognize") != self.text.is_some()
            || (self.verb == "relate") != self.entities.is_some()
        {
            return Err(format!("{} has the wrong input shape", self.id));
        }
        match (&self.expect.success, &self.expect.error) {
            (Some(success), None) => self.validate_success(success)?,
            (None, Some(_)) => self.validate_fault()?,
            _ => return Err(format!("{} has more or less than one outcome", self.id)),
        }
        if self.record.is_some() && self.verb != "annotate" {
            return Err(format!("{} names a record outside annotate", self.id));
        }
        if self.verb == "annotate" {
            if self.question_set.is_none() || self.question.is_some() {
                return Err(format!("{} has the wrong question-set shape", self.id));
            }
        } else if self.question.is_none() || self.question_set.is_some() {
            return Err(format!("{} has the wrong question shape", self.id));
        }
        Ok(())
    }

    fn validate_success(&self, success: &Success) -> Result<(), String> {
        if self.exchanges.is_empty() && self.verb != "filter" {
            return Err(format!("{} has no successful exchange", self.id));
        }
        if self.operation.is_some() || self.question_form.is_some() {
            return Err(format!("{} carries a fault injection on success", self.id));
        }
        let expected: &[&str] = match self.verb.as_str() {
            "decide" => &["single", "decide_many"],
            verb @ ("filter" | "rank" | "find" | "annotate" | "recognize" | "relate") => &[verb],
            _ => &["single"],
        };
        if !expected.contains(&success.kind.as_str())
            || (success.kind == "single" && self.exchanges.len() != 1)
            || (success.counters.is_some() && success.kind != "single")
        {
            return Err(format!("{} has success kind `{}`", self.id, success.kind));
        }
        let takes_operation = matches!(self.verb.as_str(), "filter" | "rank" | "find");
        if success.operation.is_some() != takes_operation {
            return Err(format!("{} has the wrong operation shape", self.id));
        }
        Ok(())
    }

    fn validate_fault(&self) -> Result<(), String> {
        let injected = self
            .operation
            .as_ref()
            .is_some_and(|held| !held.injection.is_empty());
        if !self.exchanges.is_empty()
            || injected == self.question_form.is_some()
            || self.question_form.is_some() != self.evidence.is_some()
        {
            return Err(format!("{} is not a schema-only fault", self.id));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub(super) struct Injection {
    pub(super) injection: String,
}

#[derive(Deserialize)]
pub(super) struct Exchange {
    pub(super) provenance: Provenance,
    pub(super) evidence: String,
    pub(super) request: String,
    pub(super) response: Box<RawValue>,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(super) enum Provenance {
    Captured { path: String },
    SyntheticContract,
}

#[derive(Deserialize)]
pub(super) struct Expect {
    pub(super) success: Option<Success>,
    pub(super) error: Option<ExpectedError>,
}

#[derive(Deserialize)]
pub(super) struct ExpectedError {
    pub(super) kind: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Success {
    pub(super) kind: String,
    pub(super) answers: Vec<ExpectedAnswer>,
    pub(super) operation: Option<Box<RawValue>>,
    #[serde(default)]
    pub(super) failed_questions: usize,
    pub(super) counters: Option<Counters>,
}

/// The counter differences a run of `calls` identical calls leaves behind.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Counters {
    pub(super) calls: usize,
    pub(super) requests: u64,
    pub(super) cache_answers: u64,
}

#[derive(Deserialize)]
pub(super) struct ExpectedAnswer {
    pub(super) exchange: usize,
    pub(super) name: String,
    pub(super) bare: Box<RawValue>,
    pub(super) details: Details,
}

#[derive(Deserialize)]
pub(super) struct Details {
    pub(super) answer: Option<Box<RawValue>>,
    pub(super) failure: Option<Box<RawValue>>,
    pub(super) model: String,
    pub(super) question_sha256: String,
    pub(super) requests: Vec<String>,
}

#[derive(Deserialize)]
struct FilterExpected {
    indexes: Vec<usize>,
}

pub(super) fn filter(raw: &RawValue, values: &[Value]) -> Result<(), String> {
    let expected: FilterExpected =
        serde_json::from_str(raw.get()).map_err(|error| error.to_string())?;
    let actual = values
        .iter()
        .enumerate()
        .filter_map(|(place, value)| matches!(value, Value::YesNo(Some(true))).then_some(place))
        .collect::<Vec<_>>();
    (actual == expected.indexes)
        .then_some(())
        .ok_or_else(|| "filter indexes differ".to_owned())
}

#[derive(Deserialize)]
struct RankExpected {
    ranking: Vec<Ranked>,
}

#[derive(Deserialize, PartialEq)]
struct Ranked {
    index: usize,
    probability: f64,
}

pub(super) fn rank(raw: &RawValue, odds: &[f64], order: &[usize]) -> Result<(), String> {
    let expected: RankExpected =
        serde_json::from_str(raw.get()).map_err(|error| error.to_string())?;
    let actual = order
        .iter()
        .map(|place| Ranked {
            index: *place,
            probability: odds.get(*place).copied().unwrap_or_default(),
        })
        .collect::<Vec<_>>();
    (actual == expected.ranking)
        .then_some(())
        .ok_or_else(|| "rank order differs".to_owned())
}

#[derive(Deserialize)]
struct FindExpected {
    selected: Option<usize>,
    probabilities: Vec<Found>,
}

#[derive(Deserialize, PartialEq)]
struct Found {
    index: Option<usize>,
    probability: f64,
}

#[derive(Deserialize)]
struct ChoiceDetails {
    probabilities: OrderedProbabilities,
}

struct OrderedProbabilities(Vec<(String, f64)>);

impl<'de> Deserialize<'de> for OrderedProbabilities {
    fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        decoder.deserialize_map(ProbabilityEntries)
    }
}

struct ProbabilityEntries;

impl<'de> Visitor<'de> for ProbabilityEntries {
    type Value = OrderedProbabilities;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("probabilities in question order")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
        let mut entries = Vec::new();
        while let Some(entry) = map.next_entry::<String, f64>()? {
            entries.push(entry);
        }
        Ok(OrderedProbabilities(entries))
    }
}

pub(super) fn find(raw: &RawValue, found: &FindAnswer, answer: &Answer) -> Result<(), String> {
    let expected: FindExpected =
        serde_json::from_str(raw.get()).map_err(|error| error.to_string())?;
    let encoded = serde_json::to_string(answer).map_err(|error| error.to_string())?;
    let details: ChoiceDetails =
        serde_json::from_str(&encoded).map_err(|error| error.to_string())?;
    let count = details.probabilities.0.len();
    let actual = details
        .probabilities
        .0
        .iter()
        .enumerate()
        .map(|(place, (_, probability))| Found {
            index: (place + 1 < count).then_some(place),
            probability: *probability,
        })
        .collect::<Vec<_>>();
    (expected.selected == found.selected() && expected.probabilities == actual)
        .then_some(())
        .ok_or_else(|| "find selection differs".to_owned())
}

const PRIVATE_KEYS: [&str; 14] = [
    "accesstoken",
    "apikey",
    "apitoken",
    "authtoken",
    "authorization",
    "bearertoken",
    "clientsecret",
    "credential",
    "credentials",
    "header",
    "headers",
    "thinkthenapikey",
    "token",
    "xapikey",
];

fn private_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect::<String>();
    PRIVATE_KEYS.contains(&normalized.as_str())
}

pub(super) fn privacy(text: &str) -> Result<(), String> {
    let mut decoder = serde_json::Deserializer::from_str(text);
    Keys.deserialize(&mut decoder)
        .map_err(|error| error.to_string())
}

struct Keys;

impl<'de> DeserializeSeed<'de> for Keys {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, decoder: D) -> Result<(), D::Error> {
        decoder.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Keys {
    type Value = ();
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("JSON without credential or header fields")
    }
    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<(), M::Error> {
        while let Some(key) = map.next_key::<String>()? {
            if private_key(&key) {
                return Err(serde::de::Error::custom(
                    "a credential or header field is forbidden",
                ));
            }
            map.next_value_seed(Keys)?;
        }
        Ok(())
    }
    fn visit_seq<S: SeqAccess<'de>>(self, mut sequence: S) -> Result<(), S::Error> {
        while sequence.next_element_seed(Keys)?.is_some() {}
        Ok(())
    }
    fn visit_bool<E: serde::de::Error>(self, _: bool) -> Result<(), E> {
        Ok(())
    }
    fn visit_i64<E: serde::de::Error>(self, _: i64) -> Result<(), E> {
        Ok(())
    }
    fn visit_u64<E: serde::de::Error>(self, _: u64) -> Result<(), E> {
        Ok(())
    }
    fn visit_f64<E: serde::de::Error>(self, _: f64) -> Result<(), E> {
        Ok(())
    }
    fn visit_str<E: serde::de::Error>(self, _: &str) -> Result<(), E> {
        Ok(())
    }
    fn visit_string<E: serde::de::Error>(self, _: String) -> Result<(), E> {
        Ok(())
    }
    fn visit_none<E: serde::de::Error>(self) -> Result<(), E> {
        Ok(())
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<(), E> {
        Ok(())
    }
}

impl Provenance {
    pub(super) fn validate(&self, exchange: &Exchange) -> Result<(), String> {
        match self {
            Self::SyntheticContract => Ok(()),
            Self::Captured { path } => {
                let committed = path.starts_with("demos/") || path.starts_with("specification/");
                if !committed || path.contains("..") || !path.contains("/recording") {
                    return Err(format!("unknown captured recording `{path}`"));
                }
                let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
                let text = std::fs::read_to_string(format!("{root}{path}/thinkthen.jsonl"))
                    .map_err(|_| format!("unknown captured recording `{path}`"))?;
                captured(&text, exchange)
            }
        }
    }
}

/// Whether a committed fixture holds each question of a captured exchange
/// with the answer and model its response carries, by ADR 0111 section 9.
fn captured(fixture: &str, exchange: &Exchange) -> Result<(), String> {
    #[derive(Deserialize)]
    struct Response<'a> {
        model: String,
        #[serde(borrow)]
        answers: std::collections::BTreeMap<String, &'a serde_json::value::RawValue>,
    }
    #[derive(Deserialize)]
    struct Line {
        key: Option<String>,
        answer: Option<String>,
        answered_by: Option<String>,
    }
    let response: Response<'_> =
        serde_json::from_str(exchange.response.get()).map_err(|error| error.to_string())?;
    let mut held = std::collections::HashMap::new();
    for line in fixture.lines() {
        let line: Line = serde_json::from_str(line).map_err(|error| error.to_string())?;
        if let (Some(key), Some(answer), Some(model)) = (line.key, line.answer, line.answered_by) {
            held.insert(key, (answer, model));
        }
    }
    let url = crate::core::Url::new("https://api.typesafe.ai/v1/systemone").map_err(|_| "URL")?;
    for (place, key) in legacy_question_keys(&url, &exchange.request)
        .iter()
        .enumerate()
    {
        let (answer, model) = held
            .get(key)
            .ok_or_else(|| format!("the fixture holds no answer for question `{key}`"))?;
        let sent = response
            .answers
            .get(&format!("q{}", place + 1))
            .ok_or("captured response lacks an answer")?;
        if !super::same_json(answer, sent.get())? || *model != response.model {
            return Err("captured exchange differs from its fixture".to_owned());
        }
    }
    Ok(())
}

/// Original v1 question keys validate historical fixture bytes before rekeying.
fn legacy_question_keys(url: &crate::core::Url, body: &str) -> Vec<String> {
    #[derive(Deserialize)]
    struct Parts<'a> {
        #[serde(borrow)]
        state: &'a serde_json::value::RawValue,
        #[serde(borrow)]
        model: &'a serde_json::value::RawValue,
        #[serde(borrow)]
        questions: std::collections::BTreeMap<String, &'a serde_json::value::RawValue>,
    }
    let parts: Parts<'_> = serde_json::from_str(body).expect("a request body");
    let mut questions: Vec<_> = parts
        .questions
        .into_iter()
        .map(|(name, question)| (name[1..].parse::<usize>().expect("a qN name"), question))
        .collect();
    questions.sort_by_key(|(place, _)| *place);
    questions
        .into_iter()
        .map(|(_, question)| {
            crate::core::pack::QuestionKey::of(
                url,
                parts.model.get(),
                parts.state.get(),
                question.get(),
            )
            .hex()
        })
        .collect()
}
