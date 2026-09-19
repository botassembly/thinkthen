//! The JSON document one judgment prints.

use serde::Serialize;

use crate::answer::{Answer, Value};
use crate::question::Question;
use crate::text::{ModelName, Url};
use crate::threshold::Threshold;

/// The schema string a version one result carries.
pub(crate) const SCHEMA: &str = "thinkthen.result/1";

/// What the backend reported it spent on the judgment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Usage {
    input_tokens: u64,
    output_tokens: u64,
}

impl Usage {
    /// Take the token counts the backend reported.
    #[must_use]
    pub(crate) const fn new(input_tokens: u64, output_tokens: u64) -> Self {
        Self {
            input_tokens,
            output_tokens,
        }
    }
}

/// Who answered, how, at what cost, from a backend or from a recording.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Meta {
    url: Url,
    model: ModelName,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<Usage>,
    replayed: bool,
}

impl Meta {
    /// Name who answered, at what cost, and whether a recording answered.
    ///
    /// `usage` is `None` when the backend reported none, and the field is then
    /// absent from the JSON. `replayed` is always present.
    #[must_use]
    pub const fn new(url: Url, model: ModelName, usage: Option<Usage>, replayed: bool) -> Self {
        Self {
            url,
            model,
            usage,
            replayed,
        }
    }
}

/// One judgment, in the shape `specification/result.md` prints.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DecisionResult {
    schema: &'static str,
    value: Value,
    question: Question,
    answer: Answer,
    threshold: Option<Threshold>,
    meta: Meta,
}

impl DecisionResult {
    /// Gather one judgment into the document the tool prints.
    ///
    /// `value` is the bare value the command would have printed, so a reader of
    /// the object and a reader of the bare line learn the same thing.
    /// `threshold` is `None` on a verb that takes no rule, and it prints `null`.
    #[must_use]
    pub const fn new(
        value: Value,
        question: Question,
        answer: Answer,
        threshold: Option<Threshold>,
        meta: Meta,
    ) -> Self {
        Self {
            schema: SCHEMA,
            value,
            question,
            answer,
            threshold,
            meta,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DecisionResult, Meta, SCHEMA, Usage};
    use crate::answer::{Answer, Value};
    use crate::probability::Probability;
    use crate::question::{Labels, Question};
    use crate::text::{ModelName, QuestionText, Url};
    use crate::threshold::Threshold;

    /// The example in `specification/result.md`, on the one line it prints on.
    const COMPACT: &str = concat!(
        r#"{"schema":"thinkthen.result/1","value":true,"#,
        r#""question":{"verb":"decide","text":"Does this ask for a refund?"},"#,
        r#""answer":{"kind":"yes_no","probability":0.92},"threshold":0.5,"#,
        r#""meta":{"url":"https://api.typesafe.ai/v1/systemone","#,
        r#""model":"jev-1.13.0","#,
        r#""usage":{"input_tokens":312,"output_tokens":48},"replayed":false}}"#,
    );

    fn example() -> DecisionResult {
        let text = QuestionText::new("Does this ask for a refund?").expect("not empty");
        let probability = Probability::new(0.92).expect("a probability");
        let answer = Answer::new_yes_no(probability);
        let threshold = Threshold::default();
        DecisionResult::new(
            answer.read(Some(threshold)).0,
            Question::Decide { text },
            answer,
            Some(threshold),
            Meta::new(
                Url::new("https://api.typesafe.ai/v1/systemone").expect("not empty"),
                ModelName::new("jev-1.13.0").expect("not empty"),
                Some(Usage::new(312, 48)),
                false,
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
    fn meta_holds_four_fields_and_drops_the_usage_a_backend_never_reported() {
        let meta = Meta::new(
            Url::new("http://127.0.0.1:8080/v1/systemone").expect("not empty"),
            ModelName::new("local-1").expect("not empty"),
            None,
            true,
        );
        let rendered = serde_json::to_string(&meta).expect("meta serializes");
        assert_eq!(
            rendered,
            r#"{"url":"http://127.0.0.1:8080/v1/systemone","model":"local-1","replayed":true}"#
        );
    }

    #[test]
    fn an_unresolved_result_prints_a_null_value_and_the_band_that_left_it_open() {
        let text = QuestionText::new("Does this ask for a refund?").expect("not empty");
        let answer = Answer::new_yes_no(Probability::new(0.5).expect("a probability"));
        let threshold: Threshold = "0.1:0.9".parse().expect("a band");
        let result = DecisionResult::new(
            answer.read(Some(threshold)).0,
            Question::Decide { text },
            answer.clone(),
            Some(threshold),
            Meta::new(
                Url::new("http://127.0.0.1:8080/v1/systemone").expect("not empty"),
                ModelName::new("local-1").expect("not empty"),
                None,
                false,
            ),
        );
        let rendered = serde_json::to_string(&result).expect("a result serializes");
        assert!(rendered.contains(r#""value":null,"#), "{rendered}");
        assert!(rendered.contains(r#""threshold":"0.1:0.9","#), "{rendered}");
    }

    #[test]
    fn a_verb_that_takes_no_rule_prints_a_null_threshold() {
        let text = QuestionText::new("How much disruption does this report?").expect("not empty");
        let levels =
            Labels::levels(vec!["None.".to_owned(), "Blocked.".to_owned()]).expect("two levels");
        let answer = Answer::new_yes_no(Probability::new(0.25).expect("a probability"));
        let result = DecisionResult::new(
            Value::Score(0.25),
            Question::Score { text, levels },
            answer,
            None,
            Meta::new(
                Url::new("http://127.0.0.1:8080/v1/systemone").expect("not empty"),
                ModelName::new("local-1").expect("not empty"),
                None,
                false,
            ),
        );
        let rendered = serde_json::to_string(&result).expect("a result serializes");
        assert!(rendered.contains(r#""value":0.25,"#), "{rendered}");
        assert!(rendered.contains(r#""threshold":null,"#), "{rendered}");
    }
}
