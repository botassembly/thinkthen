//! Wire adapters. Each turns asks into a request body and reads the reply into answers.

mod openai;
mod stub;
mod systemone;

use serde_json::value::RawValue;

use crate::core::ask::{Ask, AskAnswer};
use crate::core::error::Error;

/// A wire format a backend speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum Adapter {
    /// The System One decisions API.
    #[serde(rename = "systemone")]
    SystemOne,
    /// The OpenAI decisions API.
    #[serde(rename = "openai")]
    OpenAi,
}

/// A decoded reply, read one question at a time.
#[derive(Debug)]
pub(crate) enum Reply {
    SystemOne(systemone::Reply),
    OpenAi(openai::Reply),
    Stub,
}

impl Adapter {
    /// The path this adapter posts to, under the backend's base address.
    #[must_use]
    pub const fn path(self) -> &'static str {
        match self {
            Self::SystemOne => "systemone",
            Self::OpenAi => "decisions",
        }
    }

    /// Whether this adapter's error body reports an oversized request.
    #[must_use]
    pub fn request_too_large(self, body: &[u8]) -> bool {
        match self {
            Self::SystemOne => systemone::request_too_large(body),
            Self::OpenAi => false,
        }
    }

    /// The wire form of one ask, admitted against the backend's model bounds.
    pub(crate) fn question(
        self,
        ask: &Ask,
        limits: crate::core::Limits,
    ) -> Result<Box<RawValue>, Error> {
        if let Some((count, limit)) = ask.option_count().zip(limits.options)
            && count > limit.get()
        {
            return Err(Error::new(
                crate::core::ErrorKind::BackendLimit,
                format!("the question exceeds the backend option limit of {limit}"),
            ));
        }
        match self {
            Self::SystemOne => Ok(systemone::question(ask)),
            Self::OpenAi => openai::question(ask),
        }
    }

    /// The bytes `question` adds to a request body at `place`.
    pub(crate) fn entry_len(self, place: usize, question: &RawValue) -> usize {
        match self {
            Self::SystemOne => systemone::entry_len(place, question),
            Self::OpenAi => openai::entry_len(place, question),
        }
    }

    /// The request body that asks `questions` of `model`.
    pub(crate) fn encode(self, state: &str, model: &str, questions: &[&RawValue]) -> Vec<u8> {
        match self {
            Self::SystemOne => systemone::encode(state, model, questions),
            Self::OpenAi => openai::encode(state, model, questions),
        }
    }

    /// The reply read from `body`, or `backend.reply` when it is not a reply at all.
    pub(crate) fn decode(self, body: &[u8]) -> Result<Reply, Error> {
        match self {
            Self::SystemOne => systemone::decode(body).map(Reply::SystemOne),
            Self::OpenAi => openai::decode(body).map(Reply::OpenAi),
        }
    }
}

impl Reply {
    /// The answer to `ask`, sent at `place`. A missing or garbled answer fails only this ask.
    pub(crate) fn read(
        &self,
        place: usize,
        ask: &Ask,
        stub_key: &[u8; 32],
    ) -> Result<AskAnswer, Error> {
        if let Ask::WithState { question, .. } = ask {
            return self.read(place, question, stub_key);
        }
        match self {
            Self::SystemOne(reply) => reply.read(place, ask),
            Self::OpenAi(reply) => reply.read(place, ask),
            Self::Stub => stub::answer(stub_key, ask),
        }
    }
}
