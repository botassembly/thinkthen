//! Fixture-only shapes and checks kept apart from production semantics.

use serde::Deserialize;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::value::RawValue;
use std::fmt;
use thinkthen_core::{Answer, FindAnswer, Value};

#[derive(Deserialize)]
pub(crate) struct Document {
    schema: String,
    case_count: usize,
    error_kinds: Vec<String>,
    pub(crate) cases: Vec<Case>,
}

impl Document {
    pub(crate) fn validate_header(&self, verbs: &[&str], errors: &[&str]) -> Result<(), String> {
        if self.schema != "thinkthen.conformance/1" || self.case_count != self.cases.len() {
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
pub(crate) struct Case {
    pub(crate) id: String,
    pub(crate) verb: String,
    pub(crate) question: Option<Box<RawValue>>,
    pub(crate) question_set: Option<Box<RawValue>>,
    pub(crate) operation: Option<Injection>,
    pub(crate) exchanges: Vec<Exchange>,
    pub(crate) expect: Expect,
}

impl Case {
    pub(crate) fn validate_shape(&self) -> Result<(), String> {
        match (&self.expect.success, &self.expect.error) {
            (Some(success), None) => self.validate_success(success)?,
            (None, Some(_)) => self.validate_fault()?,
            _ => return Err(format!("{} has more or less than one outcome", self.id)),
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
        if self.exchanges.is_empty() {
            return Err(format!("{} has no successful exchange", self.id));
        }
        if self.operation.is_some() {
            return Err(format!("{} carries a fault injection on success", self.id));
        }
        let expected = match self.verb.as_str() {
            "filter" => "filter",
            "rank" => "rank",
            "find" => "find",
            "annotate" => "annotate",
            _ => "single",
        };
        if success.kind != expected {
            return Err(format!("{} has success kind `{}`", self.id, success.kind));
        }
        let takes_operation = matches!(self.verb.as_str(), "filter" | "rank" | "find");
        if success.operation.is_some() != takes_operation {
            return Err(format!("{} has the wrong operation shape", self.id));
        }
        Ok(())
    }

    fn validate_fault(&self) -> Result<(), String> {
        let injection = self
            .operation
            .as_ref()
            .ok_or_else(|| format!("{} has no injection", self.id))?;
        if !self.exchanges.is_empty() || injection.injection.is_empty() {
            return Err(format!("{} is not a schema-only fault", self.id));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub(crate) struct Injection {
    pub(crate) injection: String,
}

#[derive(Deserialize)]
pub(crate) struct Exchange {
    pub(crate) provenance: Provenance,
    pub(crate) evidence: String,
    pub(crate) request: String,
    pub(crate) response: Box<RawValue>,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum Provenance {
    Captured { path: String },
    SyntheticContract,
}

#[derive(Deserialize)]
pub(crate) struct Expect {
    pub(crate) success: Option<Success>,
    pub(crate) error: Option<ExpectedError>,
}

#[derive(Deserialize)]
pub(crate) struct ExpectedError {
    pub kind: String,
}

#[derive(Deserialize)]
pub(crate) struct Success {
    pub(crate) kind: String,
    pub(crate) answers: Vec<ExpectedAnswer>,
    pub(crate) operation: Option<Box<RawValue>>,
}

#[derive(Deserialize)]
pub(crate) struct ExpectedAnswer {
    pub(crate) exchange: usize,
    pub(crate) name: String,
    pub(crate) bare: Box<RawValue>,
    pub(crate) details: Details,
}

#[derive(Deserialize)]
pub(crate) struct Details {
    pub(crate) answer: Box<RawValue>,
    pub(crate) model: String,
    pub(crate) question_sha256: String,
}

#[derive(Deserialize)]
pub(crate) struct Recording {
    pub(crate) request: Box<RawValue>,
    pub(crate) response: Box<RawValue>,
}

#[derive(Deserialize)]
struct FilterExpected {
    indexes: Vec<usize>,
}

pub(crate) fn filter(raw: &RawValue, values: &[Value]) -> Result<(), String> {
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

pub(crate) fn rank(raw: &RawValue, odds: &[f64], order: &[usize]) -> Result<(), String> {
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

pub(crate) fn find(raw: &RawValue, found: &FindAnswer, answer: &Answer) -> Result<(), String> {
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

pub(crate) fn privacy(text: &str) -> Result<(), String> {
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
