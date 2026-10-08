//! One admitted example contract, shared by native calls, files and record projections.
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use thiserror::Error;

use crate::core::{Json, RecognizeSpec};

mod brackets;
mod render;
#[cfg(test)]
mod tests;

/// A tagged recognition example, as bracket text or literal text with scalar spans.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub enum RecognitionExample {
    /// Inline `[[KIND|TEXT]]` tags; backslash escapes bracket syntax characters.
    Brackets(String),
    /// Original text, declared entity spans and an optional broader vocabulary.
    Spans(RecognitionExampleText),
}

/// Literal example text and its entity annotations.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RecognitionExampleText {
    /// Exact decoded original text.
    pub text: String,
    /// Nonoverlapping entity spans, using Unicode-scalar offsets.
    pub entities: Vec<RecognitionExampleEntity>,
    /// Allowed kinds; omission uses the recognition's kinds, or generic ENTITY.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present_kinds"
    )]
    #[cfg_attr(test, schemars(with = "Vec<String>"))]
    pub kinds: Option<Vec<String>>,
}

/// One entity's inclusive start, exclusive end and declared kind.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RecognitionExampleEntity {
    /// Inclusive Unicode-scalar position in the original text.
    pub start: usize,
    /// Exclusive Unicode-scalar position in the original text.
    pub end: usize,
    /// A label in this example's effective vocabulary.
    pub kind: String,
}

macro_rules! withheld_debug {
    ($($type:ty),+) => {$ (
        impl fmt::Debug for $type {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(concat!(stringify!($type), "(<withheld>)"))
            }
        }
    )+};
}
withheld_debug!(
    RecognitionExample,
    RecognitionExampleText,
    RecognitionExampleEntity
);

fn present_kinds<'de, D: Deserializer<'de>>(decoder: D) -> Result<Option<Vec<String>>, D::Error> {
    Vec::<String>::deserialize(decoder).map(Some)
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub(crate) enum ExampleError {
    #[error("recognition examples use bracket text or closed text/entities objects")]
    Shape,
    #[error("recognition example bracket tags or escapes are invalid")]
    Brackets,
    #[error("recognition example text is blank or contains no pieces")]
    Blank,
    #[error("recognition example kinds are nonempty, distinct printable labels")]
    Vocabulary,
    #[error("recognition example entity names an undeclared kind")]
    Kind,
    #[error("recognition example entity edges must coincide with piece edges")]
    Edge,
    #[error("recognition example entity spans overlap")]
    Overlap,
    #[error("recognition example questions could not be rendered")]
    Render,
}

pub(crate) fn render_examples(
    spec: &RecognizeSpec,
    examples: &[RecognitionExample],
) -> Result<Vec<String>, ExampleError> {
    examples
        .iter()
        .map(|example| render::one(spec, example))
        .collect()
}

/// Read a selected list using the same serde contract as canonical Request.
pub(crate) fn examples_of(value: &Json) -> Result<Vec<RecognitionExample>, ExampleError> {
    let Json::Array(_) = value else {
        return Err(ExampleError::Shape);
    };
    let text = serde_json::to_string(value).map_err(|_| ExampleError::Shape)?;
    serde_json::from_str(&text).map_err(|_| ExampleError::Shape)
}

pub(crate) fn selected_examples(
    record: &crate::core::Record,
    pointer: &crate::core::Pointer,
) -> Result<Option<Vec<RecognitionExample>>, ExampleError> {
    let value = record.json().ok_or(ExampleError::Shape)?;
    pointer.resolve(value).map(examples_of).transpose()
}

pub(crate) fn example_file(text: &str) -> Result<Vec<RecognitionExample>, ExampleError> {
    let lines: Vec<_> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let jsonl = lines
        .first()
        .is_some_and(|line| matches!(line.trim_start().chars().next(), Some('{' | '"')));
    lines
        .into_iter()
        .map(|line| {
            if jsonl {
                serde_json::from_str(line).map_err(|_| ExampleError::Shape)
            } else {
                Ok(RecognitionExample::Brackets(line.to_owned()))
            }
        })
        .collect()
}
