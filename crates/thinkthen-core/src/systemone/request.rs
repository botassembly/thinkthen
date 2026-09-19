//! Writing one plan as the request body the backend reads.

use std::collections::BTreeMap;

use serde::{Serialize, Serializer};
use serde_json::value::RawValue;

use crate::plan::Plan;
use crate::question::Question;
use crate::systemone::{EncodeError, wire_name};

/// The body one request carries.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize, PartialEq))]
pub(crate) struct Request {
    state: String,
    model: String,
    questions: BTreeMap<String, RequestQuestion>,
}

/// One named question inside a request, in the shape its verb asks for.
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize, PartialEq))]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum RequestQuestion {
    /// A yes/no question, which the vendor calls `noul`.
    Noul { instructions: String },
    /// A pick, with the options as the keys of `criteria`.
    Choice {
        instructions: String,
        criteria: Criteria,
    },
    /// A placement, with the levels as the `criteria` array, lowest first.
    Score {
        instructions: String,
        criteria: Vec<String>,
    },
}

/// The options of a pick, as a map from each option to its description.
///
/// The command line carries labels alone, so every description is `null`
/// there. `--options` reads a map from a record, and each value travels as the
/// description. The keys keep the order they were given, because option order
/// moves the odds.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct Criteria(Vec<(String, Option<String>)>);

impl Serialize for Criteria {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(option, described)| (option, described)))
    }
}

/// The visitor that reads a `criteria` map back, keeping its document order.
#[cfg(test)]
struct Keys;

#[cfg(test)]
impl<'de> serde::de::Visitor<'de> for Keys {
    type Value = Criteria;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a map from each option to its description")
    }

    fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<Criteria, M::Error> {
        let mut options = Vec::new();
        while let Some((option, described)) = map.next_entry::<String, Option<String>>()? {
            options.push((option, described));
        }
        Ok(Criteria(options))
    }
}

#[cfg(test)]
impl<'de> serde::Deserialize<'de> for Criteria {
    /// Read the keys back in document order, so a fixture pins that order.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Keys)
    }
}

/// Write the plan as the request body the backend reads.
///
/// # Errors
///
/// Returns [`EncodeError`] when the body cannot be written as JSON.
pub fn encode(plan: &Plan) -> Result<Vec<u8>, EncodeError> {
    encode_raw(plan).map(|raw| raw.get().as_bytes().to_owned())
}

/// Write the plan as the request body the plan document embeds.
pub(crate) fn encode_raw(plan: &Plan) -> Result<Box<RawValue>, EncodeError> {
    let request = Request {
        state: plan.evidence().as_str().to_owned(),
        model: plan.model().as_str().to_owned(),
        questions: plan
            .questions()
            .iter()
            .enumerate()
            .map(|(place, question)| (wire_name(place), RequestQuestion::asking(question)))
            .collect(),
    };
    serde_json::value::to_raw_value(&request).map_err(|error| EncodeError::of(&error))
}

impl RequestQuestion {
    /// Write one question in the shape its verb asks for.
    fn asking(question: &Question) -> Self {
        match question {
            Question::Decide { text } => Self::Noul {
                instructions: text.as_str().to_owned(),
            },
            Question::Choose { text, options } => Self::Choice {
                instructions: text.as_str().to_owned(),
                criteria: Criteria(
                    options
                        .descriptions()
                        .map(|(name, described)| (name.clone(), described.map(str::to_owned)))
                        .collect(),
                ),
            },
            Question::Score { text, levels } => Self::Score {
                instructions: text.as_str().to_owned(),
                criteria: levels.names().cloned().collect(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, RequestQuestion, encode};
    use crate::systemone::tests::{disruption_plan, plan_for, team_plan, urgency_plan};
    use proptest::collection::vec;
    use proptest::prelude::{Strategy, any};
    use proptest::{prop_assert_eq, proptest};

    const DECIDE: &str =
        include_str!("../../../../specification/fixtures/systemone/decide-urgent.request.json");
    const CHOOSE: &str =
        include_str!("../../../../specification/fixtures/systemone/choose-team.request.json");
    const SCORE: &str =
        include_str!("../../../../specification/fixtures/systemone/score-disruption.request.json");

    /// The bytes one plan writes, read back as the request they spell.
    fn written(plan: &crate::plan::Plan) -> Request {
        let bytes = encode(plan).expect("a plan is writable");
        serde_json::from_slice(&bytes).expect("a systemone request")
    }

    /// The fixture a case compares against.
    fn fixture(text: &str) -> Request {
        serde_json::from_str(text).expect("a systemone request")
    }

    #[test]
    fn each_verb_writes_the_request_its_fixture_shows() {
        let cases = [
            (urgency_plan(), DECIDE),
            (team_plan(), CHOOSE),
            (disruption_plan(), SCORE),
        ];
        for (plan, text) in cases {
            assert_eq!(written(&plan), fixture(text));
        }
    }

    #[test]
    fn a_pick_writes_its_options_as_criteria_in_the_order_they_were_typed() {
        let bytes = encode(&team_plan()).expect("a plan is writable");
        let text = String::from_utf8(bytes).expect("a request is text");
        assert!(
            text.contains(
                r#""criteria":{"billing":null,"shipping":null,"account":null,"other":null}"#
            ),
            "{text}"
        );
    }

    #[test]
    fn the_names_carry_the_order_and_the_key_order_carries_nothing() {
        let plan = plan_for("Help!", &["is urgent", "asks for a refund"]);
        let written = written(&plan);
        let named = |name: &str| match written.questions.get(name) {
            Some(RequestQuestion::Noul { instructions }) => instructions.clone(),
            _ => panic!("one yes/no question per name"),
        };
        assert_eq!(named("q1"), "is urgent");
        assert_eq!(named("q2"), "asks for a refund");
    }

    fn texts() -> impl Strategy<Value = String> {
        vec(any::<char>(), 1..24)
            .prop_map(|chars| chars.into_iter().collect::<String>())
            .prop_filter("text that is not blank", |text| !text.trim().is_empty())
    }

    proptest! {
        #[test]
        fn any_evidence_and_question_reach_the_wire_unchanged(
            state in texts(),
            instructions in texts(),
        ) {
            let written = written(&plan_for(&state, &[&instructions]));
            prop_assert_eq!(&written.state, &state);
            let question = written.questions.get("q1").expect("one named question");
            let RequestQuestion::Noul { instructions: sent } = question else {
                panic!("a decide plan writes a yes/no question");
            };
            prop_assert_eq!(sent, &instructions);
        }
    }
}
