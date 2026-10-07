//! Released question/1 and versioned question/2 input digests.
use crate::core::adapters::built_in;
use crate::core::{Url, hex};
use sha2::{Digest as _, Sha256};
use std::fmt;

/// The SHA-256 of the adapter, the address, the model, the state and one
/// question, each as sent and joined by one line feed.
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

    /// Hash the five parts. `model`, `state` and `question` are compact JSON,
    /// so none holds a raw line feed, and the address refuses control bytes.
    pub(crate) fn of(url: &Url, model: &str, state: &str, question: &str) -> Self {
        Self::of_parts(Sha256::new(), url, model, state, question)
    }

    pub(crate) fn images_of(url: &Url, model: &str, state: &str, question: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(b"thinkthen.image-question/1\n");
        Self::of_parts(hasher, url, model, state, question)
    }

    fn of_parts(mut hasher: Sha256, url: &Url, model: &str, state: &str, question: &str) -> Self {
        for (place, part) in [built_in::NAME, url.as_str(), model, state, question]
            .into_iter()
            .enumerate()
        {
            if place > 0 {
                hasher.update(b"\n");
            }
            hasher.update(part.as_bytes());
        }
        Self(hasher.finalize().into())
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
