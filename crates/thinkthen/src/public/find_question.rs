//! Native whole-set find preparation over the one question-file reader/parser.
use crate::core::{self, QuestionFile};
use crate::public::question::Kind;
use crate::public::{Error, Question, RecordReading};
use std::path::Path;

/// One saved find question and its ordinary selected-record reading.
#[derive(Clone, Debug)]
pub struct FindQuestionFile {
    question: Question,
    reading: RecordReading,
    selected: bool,
}
impl FindQuestionFile {
    /// Admit saved find text/model/profile and evidence pointers through the shared grammar.
    /// # Errors
    /// Refuses authored candidates, cuts, batch and invalid question/field values.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let file = QuestionFile::parse_find(text).map_err(Error::refused)?;
        let fields = file
            .on
            .iter()
            .map(core::Pointer::as_str)
            .collect::<Vec<_>>();
        let reading = RecordReading::new(&fields, None, None)?;
        let selected = fields.iter().any(|field| !field.is_empty());
        let mut question = Question::yes_no(file.text, None, None, None, Kind::Find);
        question.model = file.model;
        question.profile = file.profile;
        Ok(Self {
            question,
            reading,
            selected,
        })
    }
    /// Load the ordinary saved find grammar using the shared capped reader.
    /// # Errors
    /// Returns safe local file/content failures before input execution.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = crate::public::question_file::load_text(path.as_ref(), "question file")?;
        Self::from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }
    /// The actual prepared question; its candidates remain the execution inputs.
    #[must_use]
    pub const fn question(&self) -> &Question {
        &self.question
    }
    /// Existing record composition with the authored whole pointer list.
    #[must_use]
    pub const fn reading(&self) -> &RecordReading {
        &self.reading
    }
    /// Move the question and reading together into a native consumer.
    #[must_use]
    pub fn into_parts(self) -> (Question, RecordReading) {
        (self.question, self.reading)
    }
}
impl Question {
    /// Admit a saved find reading; candidates remain the actual input set.
    /// # Errors
    /// Refuses invalid grammar, cuts, authored candidates or non-root evidence pointers.
    pub fn find_from_json(text: &str) -> Result<Self, Error> {
        let file = FindQuestionFile::from_json(text)?;
        if file.selected {
            return Err(Error::usage(
                "a library find question reads selected units whole; select evidence with RecordReading",
            ));
        }
        Ok(file.question)
    }
    /// Load a saved find reading with the ordinary bounded question-file reader.
    /// # Errors
    /// Local errors preserve the existing file/content boundary without echoing contents.
    pub fn load_find(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = crate::public::question_file::load_text(path.as_ref(), "question file")?;
        Self::find_from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }
}

pub(crate) fn profiled(find: core::Find, question: &Question) -> core::Find {
    find.with_profile(question.profile.clone())
}
