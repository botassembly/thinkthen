//! The request writer against its fixtures.

use super::{Request, RequestQuestion, drops_detail, encode};
use crate::core::adapters::built_in::DEFAULT_MODEL;
use crate::core::adapters::systemone::tests::{
    disruption_plan, plan_for, tag_plan, team_plan, urgency_plan,
};
use crate::core::digest::question_sha256;
use crate::core::json::Json;
use crate::core::plan::{Descriptions, Plan};
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
    let plan = Plan::authored(
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
    Plan::authored(
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

/// (question file, verb, authored questions, text questions) for each
/// description form: a string, an object with `what` and other fields, an
/// object without a usable `what`, a list, an empty object, null, and absent.
fn forms() -> [(String, Verb, String, String); 8] {
    const ALL: &str = r#"{"s":"Text.","o":{"what":"Obj","not_for":"x","examples":["e"]},"n":{"k":1},"l":["x",1],"e":{},"z":null}"#;
    let tag = |label: &str, criteria: &str| {
        format!(
            r#""{label}":{{"type":"noul","instructions":"Which?\n\nDetermine whether the label \"{label}\" applies to this item."{criteria}}}"#
        )
    };
    let text_tag = [
        tag("s", r#","criteria":{"true":"Text."}"#).replacen(r#""s""#, r#""q1""#, 1),
        tag("o", r#","criteria":{"true":"Obj"}"#).replacen(r#""o""#, r#""q2""#, 1),
        tag("n", r#","criteria":{"true":"{\"k\":1}"}"#).replacen(r#""n""#, r#""q3""#, 1),
        tag("l", r#","criteria":{"true":"[\"x\",1]"}"#).replacen(r#""l""#, r#""q4""#, 1),
        tag("e", "").replacen(r#""e""#, r#""q5""#, 1),
        tag("z", "").replacen(r#""z""#, r#""q6""#, 1),
    ]
    .join(",");
    let authored_tag = r#""q1":{"type":"noul","instructions":["Which?",{"label":"s","description":"Text."}],"criteria":{"true":"Text."}},"q2":{"type":"noul","instructions":["Which?",{"label":"o","description":{"what":"Obj","not_for":"x","examples":["e"]}}],"criteria":{"true":{"what":"Obj","not_for":"x","examples":["e"]}}},"q3":{"type":"noul","instructions":["Which?",{"label":"n","description":{"k":1}}],"criteria":{"true":{"k":1}}},"q4":{"type":"noul","instructions":["Which?",{"label":"l","description":["x",1]}],"criteria":{"true":["x",1]}},"q5":{"type":"noul","instructions":["Which?",{"label":"e","description":{}}],"criteria":{"true":{}}},"q6":{"type":"noul","instructions":["Which?",{"label":"z"}]}"#;
    let plain_tag = r#""q1":{"type":"noul","instructions":"Which?\n\nDetermine whether the label \"a\" applies to this item."},"q2":{"type":"noul","instructions":"Which?\n\nDetermine whether the label \"b\" applies to this item."}"#;
    // (question file, verb, authored questions, text questions)
    [
        (
            r#"{"decide":"Refund?","true":"Money back.","false":{"what":"No money","not_for":"credit","examples":["denied"]}}"#.to_owned(),
            Verb::Decide,
            r#""q1":{"type":"noul","instructions":"Refund?","criteria":{"true":"Money back.","false":{"what":"No money","not_for":"credit","examples":["denied"]}}}"#.to_owned(),
            r#""q1":{"type":"noul","instructions":"Refund?","criteria":{"true":"Money back.","false":"No money"}}"#.to_owned(),
        ),
        (
            r#"{"decide":"Refund?","true":{"not_for":"credit"},"false":["a","b"]}"#.to_owned(),
            Verb::Decide,
            r#""q1":{"type":"noul","instructions":"Refund?","criteria":{"true":{"not_for":"credit"},"false":["a","b"]}}"#.to_owned(),
            r#""q1":{"type":"noul","instructions":"Refund?","criteria":{"true":"{\"not_for\":\"credit\"}","false":"[\"a\",\"b\"]"}}"#.to_owned(),
        ),
        (
            r#"{"decide":"Refund?","true":{},"false":null}"#.to_owned(),
            Verb::Decide,
            r#""q1":{"type":"noul","instructions":"Refund?","criteria":{"true":{}}}"#.to_owned(),
            r#""q1":{"type":"noul","instructions":"Refund?"}"#.to_owned(),
        ),
        (
            format!(r#"{{"choose":"Which?","options":{ALL}}}"#),
            Verb::Choose,
            format!(r#""q1":{{"type":"choice","instructions":"Which?","criteria":{ALL}}}"#),
            r#""q1":{"type":"choice","instructions":"Which?","criteria":{"s":"Text.","o":"Obj","n":"{\"k\":1}","l":"[\"x\",1]","e":null,"z":null}}"#.to_owned(),
        ),
        (
            r#"{"choose":"Which?","options":["a","b"]}"#.to_owned(),
            Verb::Choose,
            r#""q1":{"type":"choice","instructions":"Which?","criteria":{"a":null,"b":null}}"#.to_owned(),
            r#""q1":{"type":"choice","instructions":"Which?","criteria":{"a":null,"b":null}}"#.to_owned(),
        ),
        (
            format!(r#"{{"tag":"Which?","labels":{ALL}}}"#),
            Verb::Tag,
            authored_tag.to_owned(),
            text_tag,
        ),
        (
            r#"{"tag":"Which?","labels":["a","b"]}"#.to_owned(),
            Verb::Tag,
            plain_tag.to_owned(),
            plain_tag.to_owned(),
        ),
        (
            r#"{"score":"How?","levels":{"s":"Text.","o":{"what":"Obj","not_for":"x","examples":["e"]},"n":{"what":"  ","k":1},"l":["x",1],"e":{},"z":null}}"#.to_owned(),
            Verb::Score,
            r#""q1":{"type":"score","instructions":"How?","criteria":["Text.",{"what":"Obj","not_for":"x","examples":["e"]},{"what":"  ","k":1},["x",1],{},{}]}"#.to_owned(),
            r#""q1":{"type":"score","instructions":"How?","criteria":["Text.","Obj","{\"what\":\"  \",\"k\":1}","[\"x\",1]","e","z"]}"#.to_owned(),
        ),
    ]
}

/// Each description form under each backend form (ADR 0115 section 3): a
/// string, an object with `what` and other fields, an object without a usable
/// `what`, a list, an empty object, null, and absent. `Authored` keeps today's
/// bytes for every backend but `ollama`; `Text` is the Ollama workaround.
#[test]
fn each_description_travels_as_its_backend_form_writes_it() {
    for (index, (file, verb, authored, text)) in forms().into_iter().enumerate() {
        let plan = file_plan(&file, verb);
        for (form, questions) in [
            (Descriptions::Authored, authored),
            (Descriptions::Text, text),
        ] {
            let plan = Plan::new(
                plan.evidence().clone(),
                plan.model().clone(),
                form,
                plan.questions().to_vec(),
            )
            .expect("a plan");
            let body = String::from_utf8(encode(&plan).expect("a request")).expect("UTF-8");
            assert_eq!(
                body,
                format!(
                    r#"{{"state":"Refund me please.","model":"{DEFAULT_MODEL}","questions":{{{questions}}}}}"#
                ),
                "{index} {form:?}"
            );
            let dropped = form == Descriptions::Text && ![2, 4, 6].contains(&index);
            assert_eq!(drops_detail(&plan), dropped, "{index} {form:?}");
        }
    }
}
