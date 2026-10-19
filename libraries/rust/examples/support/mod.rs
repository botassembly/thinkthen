#![allow(
    dead_code,
    reason = "each standalone recipe uses its own subset of shared input construction"
)]
use thinkthen::{
    InputEvidence, QuestionInput, RequestArguments, RequestDefinition, RequestInput, RequestItem,
    RequestOptions, RequestOriginal, RequestQuestion,
};

pub(super) fn arguments(
    question: impl Into<RequestDefinition>,
    input: RequestInput,
) -> RequestArguments {
    RequestArguments {
        question: RequestQuestion::Definition {
            value: question.into(),
        },
        input,
        options: RequestOptions::default(),
    }
}
pub(super) fn text(value: &str) -> RequestInput {
    RequestInput::Text {
        text: value.into(),
        images: vec![],
    }
}
pub(super) fn records<T: InputEvidence>(values: impl IntoIterator<Item = T>) -> RequestInput {
    RequestInput::Records {
        items: values
            .into_iter()
            .map(|value| {
                let original = match value.question_input() {
                    QuestionInput::Text(text) => Some(RequestOriginal::Text { text }),
                    QuestionInput::Record(value) => Some(RequestOriginal::Json {
                        value: value.original().clone(),
                    }),
                    QuestionInput::Images(_) => None,
                };
                RequestItem {
                    original,
                    context: None,
                    options: None,
                    seed_spans: None,
                    examples: None,
                    images: vec![],
                }
            })
            .collect(),
    }
}
pub(super) fn original(value: &QuestionInput) -> Result<&str, Box<dyn std::error::Error>> {
    match value {
        QuestionInput::Text(text) => Ok(text),
        QuestionInput::Record(record) => record.original().literal().ok_or_else(|| "expected text original".into()),
        _ => Err("expected text original".into()),
    }
}
