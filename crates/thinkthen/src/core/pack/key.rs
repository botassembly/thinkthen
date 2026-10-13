//! Current question input digests.
use crate::core::adapters::built_in;
use crate::core::{Url, hex};
use std::fmt;

/// Current question identities include
/// both requested and reported models with length-prefixed UTF-8 parts.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct QuestionKey([u8; 32]);

impl QuestionKey {
    /// Frame the validated canonical parts; models are compact JSON strings.
    pub(crate) fn complete(
        url: &Url,
        requested: &str,
        answered: &str,
        state: &str,
        question: &str,
    ) -> Self {
        Self::framed(
            "thinkthen.question-key/2",
            url,
            requested,
            answered,
            state,
            question,
        )
    }

    pub(crate) fn complete_for(
        api: crate::core::adapters::ApiType,
        url: &Url,
        requested: &str,
        answered: &str,
        state: &str,
        question: &str,
    ) -> Self {
        Self(crate::core::identity::framing::digest(
            "thinkthen.question-key/2",
            &[
                api.name().as_bytes(),
                url.as_str().as_bytes(),
                requested.as_bytes(),
                answered.as_bytes(),
                state.as_bytes(),
                question.as_bytes(),
            ],
        ))
    }

    /// Explicit image inputs retain a distinct versioned key domain.
    pub(crate) fn complete_image(
        url: &Url,
        requested: &str,
        answered: &str,
        state: &str,
        question: &str,
    ) -> Self {
        Self::framed(
            "thinkthen.image-question-key/2",
            url,
            requested,
            answered,
            state,
            question,
        )
    }

    fn framed(
        domain: &str,
        url: &Url,
        requested: &str,
        answered: &str,
        state: &str,
        question: &str,
    ) -> Self {
        let bytes = crate::core::identity::framing::digest(
            domain,
            &[
                built_in::NAME.as_bytes(),
                url.as_str().as_bytes(),
                requested.as_bytes(),
                answered.as_bytes(),
                state.as_bytes(),
                question.as_bytes(),
            ],
        );
        Self(bytes)
    }

    pub(crate) const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// The key as 64 lowercase hex figures.
    pub(crate) fn hex(&self) -> String {
        hex(&self.0)
    }

    /// Read 64 lowercase hex figures back.
    pub(crate) fn parse(text: &str) -> Option<Self> {
        let digits = text.as_bytes();
        if digits.len() != 64 {
            return None;
        }
        let figure = |byte: u8| match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        };
        let mut bytes = [0; 32];
        for (slot, pair) in bytes.iter_mut().zip(digits.chunks(2)) {
            let [high, low] = pair else { return None };
            *slot = (figure(*high)? << 4) | figure(*low)?;
        }
        Some(Self(bytes))
    }
}

impl fmt::Debug for QuestionKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.hex())
    }
}
