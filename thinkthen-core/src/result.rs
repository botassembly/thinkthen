//! The JSON document one judgment prints.

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use crate::answer::{Answer, Value};
use crate::question::Question;
use crate::records::Record;
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
    pub const fn new(input_tokens: u64, output_tokens: u64) -> Self {
        Self {
            input_tokens,
            output_tokens,
        }
    }

    /// Add the counts from two replies when both totals fit.
    #[must_use]
    pub const fn checked_plus(self, other: Self) -> Option<Self> {
        let Some(input_tokens) = self.input_tokens.checked_add(other.input_tokens) else {
            return None;
        };
        let Some(output_tokens) = self.output_tokens.checked_add(other.output_tokens) else {
            return None;
        };
        Some(Self {
            input_tokens,
            output_tokens,
        })
    }
}

/// One named answer inside an annotated detailed row.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnnotatedAnswer {
    value: Value,
    question: Question,
    answer: Answer,
    threshold: Option<Threshold>,
    request: String,
}

impl AnnotatedAnswer {
    /// Gather the complete answer and the request that produced it.
    #[must_use]
    pub const fn new(
        value: Value,
        question: Question,
        answer: Answer,
        threshold: Option<Threshold>,
        request: String,
    ) -> Self {
        Self {
            value,
            question,
            answer,
            threshold,
            request,
        }
    }
}

/// Aggregate metadata for one annotated record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AnnotateMeta {
    tool: String,
    questions_sha256: String,
    url: Url,
    model: ModelName,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<Usage>,
    replayed: bool,
}

impl AnnotateMeta {
    /// Gather the shared facts about every request behind one row.
    #[must_use]
    pub fn new(
        version: &str,
        questions_sha256: String,
        url: Url,
        model: ModelName,
        usage: Option<Usage>,
        replayed: bool,
    ) -> Self {
        Self {
            tool: crate::version_line(version),
            questions_sha256,
            url,
            model,
            usage,
            replayed,
        }
    }
}

/// The detailed result from applying a question set to one record.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnnotateResult {
    schema: &'static str,
    input: Record,
    value: NamedValues,
    answers: NamedAnswers,
    meta: AnnotateMeta,
}

impl AnnotateResult {
    /// Gather one complete annotation row.
    #[must_use]
    pub const fn new(
        input: Record,
        values: Vec<(String, Value)>,
        answers: Vec<(String, AnnotatedAnswer)>,
        meta: AnnotateMeta,
    ) -> Self {
        Self {
            schema: SCHEMA,
            input,
            value: NamedValues(values),
            answers: NamedAnswers(answers),
            meta,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct NamedValues(Vec<(String, Value)>);
impl Serialize for NamedValues {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(name, value)| (name, value)))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct NamedAnswers(Vec<(String, AnnotatedAnswer)>);
impl Serialize for NamedAnswers {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (name, answer) in &self.0 {
            map.serialize_entry(name, answer)?;
        }
        map.end()
    }
}

/// Who answered, how, at what cost, from a backend or from a recording.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Meta {
    tool: String,
    question_sha256: String,
    url: Url,
    model: ModelName,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<Usage>,
    replayed: bool,
}

impl Meta {
    /// Name the tool, who answered, at what cost, and whether a recording did.
    ///
    /// `version` is the binary's own version, and `tool` is the identity line
    /// the tool prints of itself, so a saved row names what made it.
    /// `question_sha256` names the exact question that produced the row, and
    /// `specification/question-file.md` writes out the form it digests.
    /// `usage` is `None` when the backend reported none, and the field is then
    /// absent from the JSON. `replayed` is always present.
    #[must_use]
    pub fn new(
        version: &str,
        question_sha256: String,
        url: Url,
        model: ModelName,
        usage: Option<Usage>,
        replayed: bool,
    ) -> Self {
        Self {
            tool: crate::version_line(version),
            question_sha256,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Record>,
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
            input: None,
            question,
            answer,
            threshold,
            meta,
        }
    }

    /// Carry the whole record this row answered, as a record row does.
    ///
    /// `input` holds the record as it arrived, including the parts no pointer
    /// sent. A single document is not a record stream, so it carries none.
    #[must_use]
    pub fn with_input(mut self, record: Record) -> Self {
        self.input = Some(record);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{DecisionResult, Meta, SCHEMA, Usage};
    use crate::answer::{Answer, Value};
    use crate::probability::Probability;
    use crate::question::{Labels, Question};
    use crate::records::{Framing, Reading};
    use crate::text::{ModelName, QuestionText, Url};
    use crate::threshold::Threshold;

    /// The digest of the example question, which `result.md` prints too.
    const DIGEST: &str = "982f744e7565001cab74fab677df4bf339916fa48b14ee909fde153869a89888";

    /// The example in `specification/result.md`, on the one line it prints on.
    const COMPACT: &str = concat!(
        r#"{"schema":"thinkthen.result/1","value":true,"#,
        r#""question":{"verb":"decide","text":"Does this ask for a refund?"},"#,
        r#""answer":{"kind":"yes_no","probability":0.92},"threshold":0.5,"#,
        r#""meta":{"tool":"thinkthen 0.4.0","question_sha256":"982f744e7565001cab74fab677df4bf339916fa48b14ee909fde153869a89888","#,
        r#""url":"https://api.typesafe.ai/v1/systemone","#,
        r#""model":"jev-1.13.0","#,
        r#""usage":{"input_tokens":312,"output_tokens":48},"replayed":false}}"#,
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
    fn meta_names_the_tool_and_drops_the_usage_a_backend_never_reported() {
        let meta = Meta::new(
            "0.4.0",
            DIGEST.to_owned(),
            Url::new("http://127.0.0.1:8080/v1/systemone").expect("not empty"),
            ModelName::new("local-1").expect("not empty"),
            None,
            true,
        );
        let rendered = serde_json::to_string(&meta).expect("meta serializes");
        assert_eq!(
            rendered,
            concat!(
                r#"{"tool":"thinkthen 0.4.0","question_sha256":"982f744e7565001cab74fab677df4bf339916fa48b14ee909fde153869a89888","#,
                r#""url":"http://127.0.0.1:8080/v1/systemone","#,
                r#""model":"local-1","replayed":true}"#,
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
                false,
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
        let levels =
            Labels::levels(vec!["None.".to_owned(), "Blocked.".to_owned()]).expect("two levels");
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
                false,
            ),
        );
        let rendered = serde_json::to_string(&result).expect("a result serializes");
        assert!(rendered.contains(r#""value":0.25,"#), "{rendered}");
        assert!(rendered.contains(r#""threshold":null,"#), "{rendered}");
    }
}
