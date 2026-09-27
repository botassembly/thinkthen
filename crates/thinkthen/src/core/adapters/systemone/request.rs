//! Writing one plan as the request body the backend reads.

#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
use serde::Deserialize;
use serde::{Serialize, Serializer};
use serde_json::value::RawValue;

use crate::core::adapters::systemone::{EncodeError, wire_name};
use crate::core::json::Json;
use crate::core::plan::Plan;
use crate::core::question::{Labels, Question};
use crate::core::text::{Description, QuestionText};

/// The body one request carries.
///
/// `state` is a string for the text evidence a run has always sent, and the
/// object or list itself when a pointer selection made one, so the JSON is
/// never folded into a sentence or written twice.
///
/// It lives only inside [`encode_raw`], which writes it at once, so nothing
/// outside a test can print it and it derives `Debug` only in tests.
#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, serde::Deserialize, PartialEq))]
pub(crate) struct Request {
    state: Json,
    model: String,
    questions: Questions,
}

/// The wire questions in request order.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct Questions(Vec<(String, RequestQuestion)>);

#[cfg(test)]
impl Questions {
    fn get(&self, name: &str) -> Option<&RequestQuestion> {
        self.0
            .iter()
            .find(|(held, _)| held == name)
            .map(|(_, question)| question)
    }
}

impl Serialize for Questions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(name, question)| (name, question)))
    }
}

#[cfg(test)]
impl<'de> Deserialize<'de> for Questions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        BTreeMap::<String, RequestQuestion>::deserialize(deserializer)
            .map(|questions| Self(questions.into_iter().collect()))
    }
}

/// One named question inside a request, in the shape its verb asks for.
#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, serde::Deserialize, PartialEq))]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum RequestQuestion {
    /// A yes/no question, which the vendor calls `noul`.
    Noul {
        instructions: Json,
        #[serde(skip_serializing_if = "Option::is_none")]
        criteria: Option<NoulCriteria>,
    },
    /// A pick, with the options as the keys of `criteria`.
    Choice {
        instructions: Json,
        criteria: Criteria,
    },
    /// A placement, with the levels as the `criteria` array, lowest first.
    Score {
        instructions: Json,
        criteria: Vec<Json>,
    },
}

/// What a yes means and what a no means, as the vendor's `criteria` object.
///
/// The tool's own words for these two are `--true` and `--false`, and this
/// module alone knows they travel here. A question that names neither carries
/// no `criteria` at all, so its request is byte for byte the request of the
/// version before the two texts existed.
#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, serde::Deserialize, PartialEq))]
pub(crate) struct NoulCriteria {
    #[serde(rename = "true", skip_serializing_if = "Option::is_none")]
    yes: Option<Json>,
    #[serde(rename = "false", skip_serializing_if = "Option::is_none")]
    no: Option<Json>,
}

/// The options of a pick, as a map from each option to its description.
///
/// The command line carries labels alone, so every description is `null`
/// there. `--options` reads a map from a record, and each value travels as the
/// description. The keys keep the order they were given, because option order
/// moves the odds.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) struct Criteria(Vec<(String, Option<Json>)>);

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
        while let Some((option, described)) = map.next_entry::<String, Option<Json>>()? {
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
pub(crate) fn encode(plan: &Plan) -> Result<Vec<u8>, EncodeError> {
    encode_raw(plan).map(|raw| raw.get().as_bytes().to_owned())
}

/// Write the plan as the request body the plan document embeds.
pub(crate) fn encode_raw(plan: &Plan) -> Result<Box<RawValue>, EncodeError> {
    let request = Request {
        state: plan.evidence().as_json(),
        model: plan.model().as_str().to_owned(),
        questions: questions(plan)?,
    };
    serde_json::value::to_raw_value(&request).map_err(|error| EncodeError::of(&error))
}

/// Expand logical tag questions into one wire yes/no question per label.
fn questions(plan: &Plan) -> Result<Questions, EncodeError> {
    let mut written = Vec::new();
    for question in plan.questions() {
        match question {
            Question::Tag { text, labels } => {
                // A string question over only string descriptions keeps the
                // sentence every older request carried. One structured value
                // turns every label into the array form instead, so no JSON is
                // ever interpolated into a sentence.
                let sentence = tag_sentence(text, labels);
                for (label, description) in labels.descriptions() {
                    let instructions = tag_instructions(text, label, description, sentence)?;
                    written.push((
                        wire_name(written.len()),
                        RequestQuestion::Noul {
                            instructions,
                            criteria: description.map(|description| NoulCriteria {
                                yes: Some(description.as_json().clone()),
                                no: None,
                            }),
                        },
                    ));
                }
            }
            _ => {
                let Some(question) = RequestQuestion::asking(question) else {
                    return Err(EncodeError::of(&"a tag question was not expanded"));
                };
                written.push((wire_name(written.len()), question));
            }
        }
    }
    Ok(Questions(written))
}

/// The sentence a string-only tag question keeps, or none when the text or one
/// description is structured.
fn tag_sentence<'a>(text: &'a QuestionText, labels: &Labels) -> Option<&'a str> {
    text.as_json().as_str().filter(|_| {
        labels
            .descriptions()
            .all(|(_, described)| described.is_none_or(|held| held.as_json().as_str().is_some()))
    })
}

/// The instruction one tag label asks: the sentence a string-only question has
/// always sent, or the array form a structured value requires.
fn tag_instructions(
    text: &QuestionText,
    label: &str,
    description: Option<&Description>,
    sentence: Option<&str>,
) -> Result<Json, EncodeError> {
    match sentence {
        Some(sentence) => {
            let label = serde_json::to_string(label).map_err(|error| EncodeError::of(&error))?;
            Ok(Json::String(format!(
                "{sentence}\n\nDetermine whether the label {label} applies to this item."
            )))
        }
        None => {
            let mut members = vec![("label".to_owned(), Json::String(label.to_owned()))];
            if let Some(description) = description {
                members.push(("description".to_owned(), description.as_json().clone()));
            }
            Ok(Json::Array(vec![
                text.as_json().clone(),
                Json::Object(members),
            ]))
        }
    }
}

impl RequestQuestion {
    /// Write one question in the shape its verb asks for.
    fn asking(question: &Question) -> Option<Self> {
        Some(match question {
            Question::Decide { text, yes, no } => Self::Noul {
                instructions: text.as_json().clone(),
                criteria: (yes.is_some() || no.is_some()).then(|| NoulCriteria {
                    yes: yes.as_ref().map(|meaning| meaning.as_json().clone()),
                    no: no.as_ref().map(|meaning| meaning.as_json().clone()),
                }),
            },
            Question::Choose { text, options } => Self::Choice {
                instructions: text.as_json().clone(),
                criteria: Criteria(
                    options
                        .descriptions()
                        .map(|(name, described)| {
                            (name.clone(), described.map(|held| held.as_json().clone()))
                        })
                        .collect(),
                ),
            },
            Question::Score { text, levels } => Self::Score {
                instructions: text.as_json().clone(),
                criteria: levels
                    .descriptions()
                    .map(|(name, described)| {
                        described.map_or_else(
                            || Json::String(name.clone()),
                            |held| match held.as_json() {
                                Json::Null => Json::Object(Vec::new()),
                                held => held.clone(),
                            },
                        )
                    })
                    .collect(),
            },
            Question::Tag { .. } => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, RequestQuestion, encode};
    use crate::core::adapters::built_in::DEFAULT_MODEL;
    use crate::core::adapters::systemone::tests::{
        disruption_plan, plan_for, tag_plan, team_plan, urgency_plan,
    };
    use crate::core::json::Json;
    use crate::core::plan::Plan;
    use crate::core::question_file::{QuestionFile, Typed, Verb, resolve};
    use crate::core::text::{Evidence, ModelName, QuestionText};
    use proptest::collection::vec;
    use proptest::prelude::{Strategy, any};
    use proptest::{prop_assert_eq, proptest};

    const DECIDE: &str = include_str!(
        "../../../../../../specification/fixtures/systemone/decide-urgent.request.json"
    );
    const CHOOSE: &str =
        include_str!("../../../../../../specification/fixtures/systemone/choose-team.request.json");
    const SCORE: &str = include_str!(
        "../../../../../../specification/fixtures/systemone/score-disruption.request.json"
    );

    /// The bytes one plan writes, read back as the request they spell.
    fn written(plan: &crate::core::plan::Plan) -> Request {
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
    fn a_tag_expands_labels_in_order_and_quotes_the_label_inside_the_instruction() {
        let bytes = encode(&tag_plan()).expect("a plan is writable");
        let text = String::from_utf8(bytes).expect("a request is text");
        let written: Request = serde_json::from_str(&text).expect("request JSON");
        assert_eq!(
            written.questions.get("q1"),
            Some(&RequestQuestion::Noul {
                instructions: Json::String(
                    "Which topics?\n\nDetermine whether the label \"bill\\\\\\\"ing\" applies to this item."
                        .to_owned()
                ),
                criteria: None,
            })
        );
        assert!(
            text.find(r#""q1""#).expect("q1") < text.find(r#""q2""#).expect("q2"),
            "{text}"
        );
    }

    #[test]
    fn a_structured_state_writes_the_json_the_selection_made() {
        let evidence = Evidence::structured(
            Json::parse(r#"{"query":"Why is signing in slow?","passage":"A lagging replica."}"#)
                .expect("a JSON object"),
        )
        .expect("an object is structured evidence");
        let plan = Plan::new(
            evidence,
            ModelName::new("local-1").expect("a model name"),
            vec![crate::core::Question::Decide {
                text: QuestionText::new("The passage answers the query.").expect("not blank"),
                yes: None,
                no: None,
            }],
        )
        .expect("a plan of one question");
        let text = String::from_utf8(encode(&plan).expect("a plan is writable"))
            .expect("a request is text");
        assert_eq!(
            text,
            concat!(
                r#"{"state":{"query":"Why is signing in slow?","passage":"A lagging replica."},"#,
                r#""model":"local-1","questions":{"q1":{"type":"noul","#,
                r#""instructions":"The passage answers the query."}}}"#,
            )
        );
    }

    /// A plan over one question file the test writes out.
    fn file_plan(text: &str, verb: Verb) -> Plan {
        let file = QuestionFile::parse(text).expect("a question file");
        let resolved =
            resolve(verb, None, Some(&file), &Typed::default()).expect("a resolved question");
        Plan::new(
            Evidence::new("Refund me please.").expect("not blank"),
            resolved.model().clone(),
            vec![resolved.question().expect("a question").clone()],
        )
        .expect("a plan")
    }

    #[test]
    fn a_structured_decide_writes_the_object_and_both_criteria() {
        let bytes = encode(&file_plan(
            r#"{"decide":{"ask":"Refund?"},"true":{"means":"Money back."},"false":null}"#,
            Verb::Decide,
        ))
        .expect("a plan is writable");
        let text = String::from_utf8(bytes).expect("a request is text");
        assert!(
            text.contains(r#""instructions":{"ask":"Refund?"}"#),
            "{text}"
        );
        assert!(
            text.contains(r#""criteria":{"true":{"means":"Money back."},"false":null}"#),
            "{text}"
        );
    }

    #[test]
    fn a_structured_choice_writes_descriptions_in_member_order() {
        let bytes = encode(&file_plan(
            r#"{"choose":"Which?","options":{"a":{"k":1},"b":null}}"#,
            Verb::Choose,
        ))
        .expect("a plan is writable");
        let text = String::from_utf8(bytes).expect("a request is text");
        assert!(
            text.contains(r#""criteria":{"a":{"k":1},"b":null}"#),
            "{text}"
        );
    }

    #[test]
    fn one_structured_tag_value_expands_every_label_into_the_array_form() {
        let bytes = encode(&file_plan(
            r#"{"tag":"Which?","labels":{"a":{"d":1},"b":null}}"#,
            Verb::Tag,
        ))
        .expect("a plan is writable");
        let text = String::from_utf8(bytes).expect("a request is text");
        assert_eq!(
            text,
            format!(
                r#"{{"state":"Refund me please.","model":"{DEFAULT_MODEL}","questions":{{"q1":{{"type":"noul","instructions":["Which?",{{"label":"a","description":{{"d":1}}}}],"criteria":{{"true":{{"d":1}}}}}},"q2":{{"type":"noul","instructions":["Which?",{{"label":"b"}}]}}}}}}"#
            )
        );
    }

    #[test]
    fn the_names_carry_the_order_and_the_key_order_carries_nothing() {
        let plan = plan_for("Help!", &["is urgent", "asks for a refund"]);
        let written = written(&plan);
        let named = |name: &str| match written.questions.get(name) {
            Some(RequestQuestion::Noul { instructions, .. }) => instructions.clone(),
            _ => panic!("one yes/no question per name"),
        };
        assert_eq!(named("q1").as_str(), Some("is urgent"));
        assert_eq!(named("q2").as_str(), Some("asks for a refund"));
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
            prop_assert_eq!(written.state.as_str(), Some(state.as_str()));
            let question = written.questions.get("q1").expect("one named question");
            let RequestQuestion::Noul {
                instructions: sent,
                ..
            } = question
            else {
                panic!("a decide plan writes a yes/no question");
            };
            prop_assert_eq!(sent.as_str(), Some(instructions.as_str()));
        }
    }
}
