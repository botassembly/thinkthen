//! OpenAI Decisions text adapter. Transport and storage remain shared.
mod request;
mod response;
pub(crate) use super::systemone::request::Parts;
pub(crate) use request::{canonical_question, input, join, parts};
pub(crate) use response::{decode_observed, reported_model, split, stored, validate_exchange};
pub(crate) const NAME: &str = "openai-decisions";
pub(crate) const BUILT_IN: super::systemone::backends::BuiltIn =
    super::systemone::backends::BuiltIn {
        name: "openai",
        path: "decisions",
        base: "https://api.openai.com/v1",
        keys: &["OPENAI_API_KEY"],
        model: "gpt-6-luna",
        accounting: crate::core::backend::InputAccounting::EncodedBody,
        descriptions: crate::core::Descriptions::Authored,
    };

#[cfg(test)]
mod tests;
