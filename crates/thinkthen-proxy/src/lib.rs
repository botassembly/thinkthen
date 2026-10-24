//! Immutable, single-tenant SQLite records behind a Rust service API.
//!
//! Admission uses the existing request and typed answer contract. This crate
//! performs no backend execution, authentication or network transport.

mod store;
pub use store::Service;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Maximum encoded bytes in each admitted content or gold-provenance field.
pub const RECORD_BYTES: usize = 1024 * 1024;
/// Maximum gold records returned in a history page.
pub const HISTORY_PAGE: usize = 100;

/// A service refusal with a stable code and no retained caller text in messages.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The shared function contract refused the request or answer.
    #[error("the question or answer does not match the admitted contract")]
    Question,
    /// A store operation failed, including a configured size boundary.
    #[error("the record store could not complete the operation")]
    Storage,
    /// A correction did not name the current gold answer.
    #[error("the gold answer changed; read its current identifier before correcting it")]
    Conflict,
}

impl Error {
    /// Stable service error code.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Question => "proxy.question",
            Self::Storage => "proxy.storage",
            Self::Conflict => "proxy.conflict",
        }
    }
}

macro_rules! content_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name(String);
        impl $name {
            /// Canonical content hash.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}
content_id!(
    QuestionId,
    "Identity of an immutable essential ask, function and fixed domain."
);
content_id!(WordingId, "Identity of one explicitly linked wording.");
content_id!(
    EvidenceId,
    "Identity of exact typed evidence, independent of its source identifier."
);

/// Identity of one immutable curated assertion and its audit event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoldId(i64);

/// A question and the first wording explicitly admitted for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admission {
    /// Identity shared by explicitly equivalent wordings.
    pub question: QuestionId,
    /// Identity of this wording.
    pub wording: WordingId,
}

/// Materialized selected evidence; request pointers are not applied by the store.
/// Text and a structured JSON string remain different values.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum Evidence {
    /// UTF-8 text without normalization.
    Text(String),
    /// Structured JSON with significant array order and canonical object order.
    Json(Value),
}

/// Authorship remains explicit for people and agents.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ActorKind {
    /// A person submitted the assertion.
    Person,
    /// An agent submitted the assertion.
    Agent,
}

/// Curator-supplied provenance for a gold assertion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribution {
    /// Stable actor identifier; credentials never belong here.
    pub actor: String,
    /// Whether the actor is a person or agent.
    pub kind: ActorKind,
    /// Why the answer was set or corrected.
    pub reason: String,
}

/// A retained gold assertion; a correction never rewrites this value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gold {
    /// Identifier used for an expected-current correction.
    pub id: GoldId,
    /// Question this assertion answers.
    pub question: QuestionId,
    /// Exact evidence this assertion judges.
    pub evidence: EvidenceId,
    /// Existing typed answer's serialized representation.
    pub answer: Value,
    /// Immutable authorship and reason.
    pub attribution: Attribution,
    /// Store-recorded milliseconds since the Unix epoch.
    pub recorded_at: i64,
    /// Prior assertion, if this is a correction.
    pub supersedes: Option<GoldId>,
}

impl From<rusqlite::Error> for Error {
    fn from(_: rusqlite::Error) -> Self {
        Self::Storage
    }
}
impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::Storage
    }
}
