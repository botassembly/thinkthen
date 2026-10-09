//! Ordinary native preparation; no compatibility execution or host question parser.
use super::{admission::Invocation, tools::Tool};
use crate::{Error, LoadedQuestion, Question, QuestionSet, RankSet, RecognizeQuestionFile, Relate};

#[derive(Debug)]
pub(super) enum PreparedQuestion {
    Atomic(LoadedQuestion),
    Annotate(QuestionSet),
    RankSet(RankSet),
    Rank(Question),
    Find(crate::FindQuestionFile),
    Recognize(RecognizeQuestionFile),
    Relate(Relate),
    Dynamic(crate::RecordChooseQuestion),
    FindPrepared(Question, crate::RecordReading),
    RecognizePrepared(crate::Recognize, crate::RecordReading),
}
impl Invocation {
    pub(super) fn question(&self) -> Result<PreparedQuestion, Error> {
        if let Some(reference) = &self.arguments.question_reference {
            let text = match crate::public::named_question::Reference::resolve(reference)? {
                crate::public::named_question::Reference::Name(name) => {
                    crate::public::named_question::named_text(&name)?
                }
                crate::public::named_question::Reference::Path(path) => {
                    crate::read_question_file(&path)
                        .map_err(|_| Error::local("the question file could not be read"))?
                }
            };
            self.file_role(&text)?;
            return self
                .saved(&text)
                .map_err(|e| Error::local(e.detail().message()));
        }
        if let Some(name) = &self.arguments.question_name {
            let text = crate::public::named_question::named_text(name)?;
            self.file_role(&text)?;
            return self
                .saved(&text)
                .map_err(|e| Error::local(e.detail().message()));
        }
        if let Some(path) = &self.arguments.question_file {
            let text = crate::read_question_file(path).map_err(|reason| match reason {
                crate::QuestionFileError::TooLarge => {
                    Error::local("the question file is too large")
                }
                _ => Error::local("the question file could not be read"),
            })?;
            self.file_role(&text)?;
            return self
                .saved(&text)
                .map_err(|e| Error::local(e.detail().message()));
        }
        let raw = self
            .arguments
            .question
            .as_ref()
            .ok_or_else(|| Error::usage("missing question"))?;
        // Decode only the transport string carrier. Structured grammar stays native.
        if raw.get().starts_with('"') {
            let text: String =
                serde_json::from_str(raw.get()).map_err(|_| Error::usage("invalid question"))?;
            return self.text_question(&text);
        }
        self.saved(raw.get())
    }

    fn file_role(&self, text: &str) -> Result<(), Error> {
        use crate::core::QuestionRole as Role;
        let role = match self.tool {
            Tool::Annotate => Role::Set,
            Tool::Find => Role::Find,
            Tool::Recognize => Role::Recognize,
            Tool::Relate => Role::Relate,
            Tool::Rank if !Role::Set.differs(text) => Role::Set,
            Tool::Rank => Role::Rank,
            Tool::Choose => Role::Choose,
            _ => Role::Atomic,
        };
        if role.differs(text) {
            return Err(Error::usage("the question file uses another function"));
        }
        Ok(())
    }
    fn saved(&self, text: &str) -> Result<PreparedQuestion, Error> {
        match self.tool {
            Tool::Annotate => QuestionSet::from_json(text).map(PreparedQuestion::Annotate),
            Tool::Rank => {
                // Native role discrimination chooses the existing atomic or set loader.
                if crate::core::QuestionRole::Set.differs(text) {
                    Question::rank_from_json(text).map(PreparedQuestion::Rank)
                } else {
                    RankSet::from_json(text).map(PreparedQuestion::RankSet)
                }
            }
            Tool::Find => crate::FindQuestionFile::from_json(text).map(PreparedQuestion::Find),
            Tool::Recognize => {
                RecognizeQuestionFile::from_json(text).map(PreparedQuestion::Recognize)
            }
            Tool::Relate => Relate::from_records_json(text).map(PreparedQuestion::Relate),
            Tool::Choose
                if self.arguments.options.options_field.is_some()
                    || self.arguments.inputs.as_ref().is_some_and(|inputs| {
                        !inputs.is_empty() && inputs.iter().all(|input| input.options.is_some())
                    }) =>
            {
                crate::RecordChooseQuestion::from_json(text).map(PreparedQuestion::Dynamic)
            }
            _ => Question::from_json(text).and_then(|q| self.atomic(q)),
        }
    }
    fn text_question(&self, text: &str) -> Result<PreparedQuestion, Error> {
        match self.tool {
            Tool::Decide | Tool::Filter => {
                self.atomic(LoadedQuestion::Question(Question::decide(text)?.cut()))
            }
            Tool::Rank => Question::rank(text).map(PreparedQuestion::Rank),
            Tool::Find => Ok(PreparedQuestion::Atomic(LoadedQuestion::Question(
                Question::find(text)?,
            ))),
            _ => Err(Error::usage(
                "this function requires the ordinary JSON question grammar",
            )),
        }
    }
    fn atomic(&self, question: LoadedQuestion) -> Result<PreparedQuestion, Error> {
        use crate::QuestionKind as Kind;
        let valid = match self.tool {
            Tool::Decide => question.kind() == Kind::Decide,
            Tool::Filter => question.kind() == Kind::Decide,
            Tool::Choose => question.kind() == Kind::Choose,
            Tool::Tag => question.kind() == Kind::Tag,
            Tool::Score => question.kind() == Kind::Score,
            _ => false,
        };
        if !valid {
            return Err(Error::usage("question kind does not match the named tool"));
        }
        Ok(PreparedQuestion::Atomic(question))
    }
    pub(super) fn source(&self) -> Result<Option<crate::SourceItems>, Error> {
        self.arguments
            .source
            .as_ref()
            .map(|s| {
                crate::SourceItems::bounded_images(
                    &s.paths,
                    s.options(),
                    super::protocol::MAX_MESSAGE,
                )
            })
            .transpose()
    }
    pub(super) fn attachments(&self) -> Result<Option<crate::ImageEvidence>, Error> {
        if self.arguments.images.is_empty() {
            return Ok(None);
        }
        let options = crate::InputReaderOptions {
            reading: crate::ReaderOptions {
                unit: crate::SourceUnit::File,
                window: None,
            },
            media: crate::ReaderMedia::Image,
        };
        let images = crate::SourceItems::bounded_images(
            &self.arguments.images,
            options,
            super::protocol::MAX_MESSAGE,
        )?
        .map(|item| match item? {
            crate::SourceItem::Image(image) => Ok(image.record),
            crate::SourceItem::Text(_) => Err(Error::defect("image reader returned text")),
        })
        .collect::<Result<Vec<_>, Error>>()?;
        crate::ImageEvidence::new(self.arguments.evidence.clone(), images).map(Some)
    }
}
