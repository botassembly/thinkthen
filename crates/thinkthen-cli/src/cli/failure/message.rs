//! Authored question names are shown only in an explicit human diagnostic.

pub(crate) struct QuestionSet(String);

impl From<String> for QuestionSet {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl std::fmt::Display for QuestionSet {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::fmt::Debug for QuestionSet {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("QuestionSet(<names withheld>)")
    }
}
