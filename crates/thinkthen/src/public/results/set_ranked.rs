//! The additive set-rank result owns each original only once.
use std::fmt;

/// One original record, attributed to the member that first selected it.
/// Its probability belongs to that member; probabilities across members
/// need not decrease along the merged order. Debug withholds input and name.
#[derive(Clone, PartialEq)]
pub struct SetRanked<T> {
    index: usize,
    input: T,
    probability: f64,
    question_name: String,
}

impl<T> fmt::Debug for SetRanked<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SetRanked")
            .field("index", &self.index)
            .field("probability", &self.probability)
            .finish_non_exhaustive()
    }
}

impl<T> SetRanked<T> {
    pub(crate) fn new(index: usize, input: T, probability: f64, question_name: String) -> Self {
        Self {
            index,
            input,
            probability,
            question_name,
        }
    }
    /// Zero-based original input position.
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }
    /// The original record, borrowed.
    #[must_use]
    pub fn input(&self) -> &T {
        &self.input
    }
    /// The original record, moved out.
    #[must_use]
    pub fn into_input(self) -> T {
        self.input
    }
    /// The selecting member's yes probability.
    #[must_use]
    pub fn probability(&self) -> f64 {
        self.probability
    }
    /// The selecting member's saved name.
    #[must_use]
    pub fn question_name(&self) -> &str {
        &self.question_name
    }
}
