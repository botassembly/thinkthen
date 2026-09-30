//! The request writer against its fixtures.

use super::{Request, RequestQuestion, encode};
use crate::core::adapters::built_in::DEFAULT_MODEL;
use crate::core::adapters::systemone::tests::{
    disruption_plan, plan_for, tag_plan, team_plan, urgency_plan,
};
use crate::core::digest::question_sha256;
use crate::core::json::Json;
use crate::core::plan::Plan;
use crate::core::question_file::{QuestionFile, Typed, Verb, resolve};
use crate::core::recording::Exchange;
use crate::core::text::{Evidence, ModelName, QuestionText, Url};
use crate::core::threshold::Threshold;
use proptest::collection::vec;
use proptest::prelude::{Strategy, any};
use proptest::{prop_assert_eq, proptest};

const DECIDE: &str =
    include_str!("../../../../../../specification/fixtures/systemone/decide-urgent.request.json");
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
        text.contains(r#""criteria":{"billing":null,"shipping":null,"account":null,"other":null}"#),
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
    let text =
        String::from_utf8(encode(&plan).expect("a plan is writable")).expect("a request is text");
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
fn null_noul_descriptions_leave_only_described_criteria() {
    let cases = [
        (
            r#"{"decide":"Refund?","true":{"means":"Money back."},"false":null}"#,
            r#"{"true":{"means":"Money back."}}"#,
        ),
        (
            r#"{"decide":"Refund?","true":null,"false":{"means":"Not money back."}}"#,
            r#"{"false":{"means":"Not money back."}}"#,
        ),
        (r#"{"decide":"Refund?","true":null,"false":null}"#, ""),
        (r#"{"decide":"Refund?"}"#, ""),
        (
            r#"{"decide":"Refund?","true":"Money back.","false":"Anything else."}"#,
            r#"{"true":"Money back.","false":"Anything else."}"#,
        ),
    ];
    for (question, criteria) in cases {
        let body = String::from_utf8(encode(&file_plan(question, Verb::Decide)).expect("request"))
            .expect("UTF-8");
        let suffix = if criteria.is_empty() {
            String::new()
        } else {
            format!(r#","criteria":{criteria}"#)
        };
        assert_eq!(
            body,
            format!(
                r#"{{"state":"Refund me please.","model":"{DEFAULT_MODEL}","questions":{{"q1":{{"type":"noul","instructions":"Refund?"{suffix}}}}}}}"#
            )
        );
    }
}

#[test]
fn null_and_absent_meanings_keep_distinct_question_names_but_share_one_exchange() {
    let absent = file_plan(r#"{"decide":"Refund?"}"#, Verb::Decide);
    let explicit = file_plan(r#"{"decide":"Refund?","true":null}"#, Verb::Decide);
    let cut = Some(Threshold::default());
    assert_ne!(
        question_sha256(&absent.questions()[0], cut).expect("absent digest"),
        question_sha256(&explicit.questions()[0], cut).expect("explicit digest")
    );
    let first = encode(&absent).expect("request");
    let second = encode(&explicit).expect("request");
    assert_eq!(first, second);
    let url = Url::new("http://127.0.0.1:9/v1/systemone").expect("URL");
    assert_eq!(
        Exchange::new(&url, &first).digest(),
        Exchange::new(&url, &second).digest()
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
