//! What a backend said, once an adapter has read its response.

use crate::answer::Answer;
use crate::result::Usage;
use crate::text::ModelName;

/// The model that answered, one answer per planned question, and the usage.
#[derive(Clone, Debug, PartialEq)]
pub struct Reply {
    model: ModelName,
    answers: Vec<Answer>,
    usage: Option<Usage>,
}

impl Reply {
    /// Gather what one adapter read from one response body.
    pub(crate) const fn new(model: ModelName, answers: Vec<Answer>, usage: Option<Usage>) -> Self {
        Self {
            model,
            answers,
            usage,
        }
    }

    /// Read the model the backend reported.
    #[must_use]
    pub const fn model(&self) -> &ModelName {
        &self.model
    }

    /// Read the answers back, one per planned question, in plan order.
    #[must_use]
    pub fn answers(&self) -> &[Answer] {
        &self.answers
    }

    /// Read the usage the backend reported, or `None` when it reported none.
    #[must_use]
    pub const fn usage(&self) -> Option<Usage> {
        self.usage
    }
}
