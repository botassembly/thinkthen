//! Native whole-set find preparation over the one question-file reader/parser.
use crate::core::{self, QuestionFile};
use crate::public::question::Kind;
use crate::public::{Error, Question};
use std::path::Path;
impl Question {
    /// Admit a saved find reading; candidates remain the actual input set.
    /// # Errors
    /// Refuses invalid grammar, cuts, authored candidates or non-root evidence pointers.
    pub fn find_from_json(text: &str) -> Result<Self, Error> {
        let file = QuestionFile::parse_find(text).map_err(Error::refused)?;
        if file.on.iter().any(|pointer| !pointer.as_str().is_empty()) {
            return Err(Error::usage(
                "a library find question reads selected units whole; select evidence with RecordReading",
            ));
        }
        let mut question = Self::yes_no(file.text, None, None, None, Kind::Find);
        question.model = file.model;
        question.profile = file.profile;
        Ok(question)
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
