//! The JSON document one judgment prints.

use serde::Serialize;

use crate::adapter::Adapter;
use crate::answer::Answer;
use crate::assessment::Assessment;
use crate::question::Question;
use crate::text::{BackendName, ModelName, Url};

/// The schema string a version one result carries.
pub const SCHEMA: &str = "thinkthen.result/1";

/// What the backend reported it spent on the judgment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Usage {
    input_tokens: u64,
    output_tokens: u64,
}

impl Usage {
    /// Take the token counts the backend reported.
    #[must_use]
    pub const fn new(input_tokens: u64, output_tokens: u64) -> Self {
        Self {
            input_tokens,
            output_tokens,
        }
    }
}

/// Who answered, how, and at what cost.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Meta {
    backend: Option<BackendName>,
    url: Url,
    adapter: Adapter,
    model: ModelName,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<Usage>,
}

impl Meta {
    /// Name the backend, the URL that answered, the adapter, the model, and the usage.
    ///
    /// `backend` is `None` for an ad-hoc backend, and the field is then `null`.
    /// `usage` is `None` when the backend reported none, and the field is then
    /// absent from the JSON.
    #[must_use]
    pub const fn new(
        backend: Option<BackendName>,
        url: Url,
        adapter: Adapter,
        model: ModelName,
        usage: Option<Usage>,
    ) -> Self {
        Self {
            backend,
            url,
            adapter,
            model,
            usage,
        }
    }
}

/// One judgment, in the shape `specification/result.md` prints.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DecisionResult {
    schema: &'static str,
    question: Question,
    answer: Answer,
    assessment: Assessment,
    meta: Meta,
}

impl DecisionResult {
    /// Gather one judgment into the document the tool prints.
    #[must_use]
    pub const fn new(
        question: Question,
        answer: Answer,
        assessment: Assessment,
        meta: Meta,
    ) -> Self {
        Self {
            schema: SCHEMA,
            question,
            answer,
            assessment,
            meta,
        }
    }

    /// Read the assessment back.
    #[must_use]
    pub const fn assessment(&self) -> Assessment {
        self.assessment
    }
}

#[cfg(test)]
mod tests {
    use super::{DecisionResult, Meta, SCHEMA, Usage};
    use crate::adapter::Adapter;
    use crate::answer::Answer;
    use crate::assessment::assess;
    use crate::pass_mark::PassMark;
    use crate::policy::Policy;
    use crate::probability::Probability;
    use crate::question::Question;
    use crate::text::{BackendName, Condition, ModelName, Url};

    /// The example in `specification/result.md`, on the one line it prints on.
    const COMPACT: &str = concat!(
        r#"{"schema":"thinkthen.result/1","#,
        r#""question":{"verb":"if","condition":"asks for a refund"},"#,
        r#""answer":{"kind":"yes_no","probability":0.92},"#,
        r#""assessment":{"status":"accepted","value":true,"min_prob":0.9},"#,
        r#""meta":{"backend":"jev","url":"https://api.typesafe.ai/v1/systemone","#,
        r#""adapter":"systemone","model":"jev-1.13.0","#,
        r#""usage":{"input_tokens":312,"output_tokens":48}}}"#,
    );

    fn example() -> DecisionResult {
        let condition = Condition::new("asks for a refund").expect("not empty");
        let probability = Probability::new(0.92).expect("a probability");
        let mark = PassMark::new(0.9).expect("a pass mark");
        DecisionResult::new(
            Question::new_if(condition),
            Answer::new_yes_no(probability),
            assess(Answer::new_yes_no(probability), Policy::Symmetric(mark)),
            Meta::new(
                Some(BackendName::new("jev").expect("not empty")),
                Url::new("https://api.typesafe.ai/v1/systemone").expect("not empty"),
                Adapter::SystemOne,
                ModelName::new("jev-1.13.0").expect("not empty"),
                Some(Usage::new(312, 48)),
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
    fn usage_is_absent_and_an_ad_hoc_backend_is_null() {
        let meta = Meta::new(
            None,
            Url::new("http://127.0.0.1:8080/v1").expect("not empty"),
            Adapter::SystemOne,
            ModelName::new("local-1").expect("not empty"),
            None,
        );
        let rendered = serde_json::to_string(&meta).expect("meta serializes");
        assert_eq!(
            rendered,
            r#"{"backend":null,"url":"http://127.0.0.1:8080/v1","adapter":"systemone","model":"local-1"}"#
        );
    }

    #[test]
    fn an_unassessed_result_prints_a_null_value_and_a_null_mark() {
        let probability = Probability::new(0.92).expect("a probability");
        let assessment = assess(Answer::new_yes_no(probability), Policy::Unassessed);
        let rendered = serde_json::to_string(&assessment).expect("an assessment serializes");
        assert_eq!(
            rendered,
            r#"{"status":"unassessed","value":null,"min_prob":null}"#
        );
    }
}
