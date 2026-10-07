//! Saved recognition selection uses the ordinary question and record readers.
use crate::core;
use crate::public::{Error, Recognize, RecordReading};
use std::path::Path;

/// An actual recognition reading and its saved selected-record pointers.
#[derive(Clone, Debug)]
pub struct RecognizeQuestionFile {
    question: Recognize,
    reading: RecordReading,
}
impl RecognizeQuestionFile {
    /// Admit the ordinary saved grammar, retaining every authored `on` pointer.
    /// # Errors
    /// Refuses invalid kinds, relations, cuts, settings or pointer combinations.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let question = Recognize(core::RecognizeSpec::parse(text).map_err(Error::refused)?);
        let fields = question
            .0
            .on
            .iter()
            .map(core::Pointer::as_str)
            .collect::<Vec<_>>();
        let reading = RecordReading::new(&fields, None, None)?;
        Ok(Self { question, reading })
    }
    /// Load a selected recognition reading through the ordinary bounded file reader.
    /// # Errors
    /// Unreadable and invalid saved content returns Local before input execution.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = crate::public::question_file::load_text(path.as_ref(), "recognize file")?;
        Self::from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }
    /// The actual model, profile, kinds, relations and resolved cuts.
    #[must_use]
    pub const fn question(&self) -> &Recognize {
        &self.question
    }
    /// The shared native record reading for selected evidence.
    #[must_use]
    pub const fn reading(&self) -> &RecordReading {
        &self.reading
    }
    /// Move the prepared question and record reading into a consumer together.
    #[must_use]
    pub fn into_parts(self) -> (Recognize, RecordReading) {
        (self.question, self.reading)
    }
}
