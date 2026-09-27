use super::{DecisionResult, Meta, RequestMeta, SCHEMA, Usage};
use crate::core::answer::{Answer, Value};
use crate::core::probability::Probability;
use crate::core::question::{Labels, Question};
use crate::core::records::{Framing, Reading};
use crate::core::text::{ModelName, QuestionText, Url};
use crate::core::threshold::Threshold;

/// The digest of the example question, which `result.md` prints too.
const DIGEST: &str = "982f744e7565001cab74fab677df4bf339916fa48b14ee909fde153869a89888";
const REQUEST: &str = "6b1f31aa3cf47e4e6a7f2b3d9ce06df13bc3340e6713473b434f9bbc263b91c4";

/// The example in `specification/result.md`, on the one line it prints on.
const COMPACT: &str = concat!(
    r#"{"schema":"thinkthen.result/1","value":true,"#,
    r#""question":{"verb":"decide","text":"Does this ask for a refund?"},"#,
    r#""answer":{"kind":"yes_no","probability":0.92},"threshold":0.5,"#,
    r#""meta":{"tool":"thinkthen 0.4.0","question_sha256":"982f744e7565001cab74fab677df4bf339916fa48b14ee909fde153869a89888","#,
    r#""url":"https://api.typesafe.ai/v1/systemone","#,
    r#""model":"jev-1.13.0","#,
    r#""usage":{"input_tokens":312,"output_tokens":48},"requests_sent":1,"cached":false,"#,
    r#""requests":["6b1f31aa3cf47e4e6a7f2b3d9ce06df13bc3340e6713473b434f9bbc263b91c4"],"failed_questions":0}}"#,
);

#[test]
fn usage_addition_refuses_either_counter_overflow() {
    let largest = Usage::new(u64::MAX, u64::MAX);
    assert_eq!(largest.checked_plus(Usage::new(1, 0)), None);
    assert_eq!(largest.checked_plus(Usage::new(0, 1)), None);
    assert_eq!(
        Usage::new(2, 3).checked_plus(Usage::new(5, 7)),
        Some(Usage::new(7, 10))
    );
}

fn example() -> DecisionResult {
    let text = QuestionText::new("Does this ask for a refund?").expect("not empty");
    let probability = Probability::new(0.92).expect("a probability");
    let answer = Answer::new_yes_no(probability);
    let threshold = Threshold::default();
    DecisionResult::new(
        answer.read(Some(threshold)).0,
        Question::Decide {
            text,
            yes: None,
            no: None,
        },
        answer,
        Some(threshold),
        Meta::new(
            "0.4.0",
            DIGEST.to_owned(),
            Url::new("https://api.typesafe.ai/v1/systemone").expect("not empty"),
            ModelName::new("jev-1.13.0").expect("not empty"),
            Some(Usage::new(312, 48)),
            RequestMeta::new(false, 1, vec![REQUEST.to_owned()]),
        ),
    )
}

#[test]
fn the_schema_constant_is_the_string_a_result_carries() {
    assert_eq!(SCHEMA, "thinkthen.result/1");
    let rendered = serde_json::to_string(&example()).expect("a result serializes");
    assert!(
        rendered.starts_with(&format!(r#"{{"schema":"{SCHEMA}""#)),
        "{rendered}"
    );
}

#[test]
fn a_result_serializes_in_the_order_the_specification_prints() {
    let rendered = serde_json::to_string(&example()).expect("a result serializes");
    assert_eq!(rendered, COMPACT);
}

#[test]
fn meta_names_the_tool_and_drops_the_usage_a_backend_never_reported() {
    let meta = Meta::new(
        "0.4.0",
        DIGEST.to_owned(),
        Url::new("http://127.0.0.1:8080/v1/systemone").expect("not empty"),
        ModelName::new("local-1").expect("not empty"),
        None,
        RequestMeta::new(true, 0, vec![REQUEST.to_owned()]),
    );
    let rendered = serde_json::to_string(&meta).expect("meta serializes");
    assert_eq!(
        rendered,
        concat!(
            r#"{"tool":"thinkthen 0.4.0","question_sha256":"982f744e7565001cab74fab677df4bf339916fa48b14ee909fde153869a89888","#,
            r#""url":"http://127.0.0.1:8080/v1/systemone","#,
            r#""model":"local-1","requests_sent":0,"cached":true,"requests":["6b1f31aa3cf47e4e6a7f2b3d9ce06df13bc3340e6713473b434f9bbc263b91c4"],"failed_questions":0}"#,
        )
    );
}

#[test]
fn an_unresolved_result_prints_a_null_value_and_the_band_that_left_it_open() {
    let text = QuestionText::new("Does this ask for a refund?").expect("not empty");
    let answer = Answer::new_yes_no(Probability::new(0.5).expect("a probability"));
    let threshold: Threshold = "0.1:0.9".parse().expect("a band");
    let result = DecisionResult::new(
        answer.read(Some(threshold)).0,
        Question::Decide {
            text,
            yes: None,
            no: None,
        },
        answer.clone(),
        Some(threshold),
        Meta::new(
            "0.4.0",
            DIGEST.to_owned(),
            Url::new("http://127.0.0.1:8080/v1/systemone").expect("not empty"),
            ModelName::new("local-1").expect("not empty"),
            None,
            RequestMeta::new(false, 1, vec![REQUEST.to_owned()]),
        ),
    );
    let rendered = serde_json::to_string(&result).expect("a result serializes");
    assert!(rendered.contains(r#""value":null,"#), "{rendered}");
    assert!(rendered.contains(r#""threshold":"0.1:0.9","#), "{rendered}");
}

#[test]
fn a_record_row_carries_the_whole_record_under_input() {
    let reading = Reading::new(Framing::Jsonl, Vec::new()).expect("a framing");
    let line = br#"{"id":"T-91","body":"Payouts have failed for 3 days."}"#;
    let record = reading.record(line).expect("a record");
    let rendered = serde_json::to_string(&example().with_input(record)).expect("a row");
    assert!(
        rendered.starts_with(concat!(
            r#"{"schema":"thinkthen.result/1","value":true,"#,
            r#""input":{"id":"T-91","body":"Payouts have failed for 3 days."},"question":"#,
        )),
        "{rendered}"
    );
}

#[test]
fn a_verb_that_takes_no_rule_prints_a_null_threshold() {
    let text = QuestionText::new("How much disruption does this report?").expect("not empty");
    let named = [("None.", None), ("Blocked.", None)].map(|(n, d)| (n.to_owned(), d));
    let levels = Labels::levels(named.into()).expect("two levels");
    let answer = Answer::new_yes_no(Probability::new(0.25).expect("a probability"));
    let result = DecisionResult::new(
        Value::Score(0.25),
        Question::Score { text, levels },
        answer,
        None,
        Meta::new(
            "0.4.0",
            DIGEST.to_owned(),
            Url::new("http://127.0.0.1:8080/v1/systemone").expect("not empty"),
            ModelName::new("local-1").expect("not empty"),
            None,
            RequestMeta::new(false, 1, vec![REQUEST.to_owned()]),
        ),
    );
    let rendered = serde_json::to_string(&result).expect("a result serializes");
    assert!(rendered.contains(r#""value":0.25,"#), "{rendered}");
    assert!(rendered.contains(r#""threshold":null,"#), "{rendered}");
}
