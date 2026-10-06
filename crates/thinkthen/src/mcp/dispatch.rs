//! Native input preparation, reusable by the pending complete dispatch adapter.

use super::{admission::Invocation, tools::Tool};
use crate::{Error, LoadedQuestion, Question, QuestionSet, RankSet, Recognize, Relate};

#[derive(Debug)]
pub(super) enum PreparedQuestion {
    Atomic(LoadedQuestion),
    Annotate(QuestionSet),
    RankSet(RankSet),
    Recognize(Recognize),
    Relate(Relate),
}

impl Invocation {
    /// Use the existing loaders. Inline @ prefixes remain literal text.
    /// Call on the dispatch worker, never the protocol reader.
    pub(super) fn question(&self) -> Result<PreparedQuestion, Error> {
        if let Some(path) = &self.arguments.question_file {
            return match self.tool {
                Tool::Annotate => QuestionSet::load(path).map(PreparedQuestion::Annotate),
                Tool::Rank => RankSet::load(path).map(PreparedQuestion::RankSet),
                Tool::Recognize => Recognize::load(path).map(PreparedQuestion::Recognize),
                Tool::Relate => Relate::load(path).map(PreparedQuestion::Relate),
                _ => Question::load(path).and_then(|question| self.atomic(question)),
            };
        }
        let raw = self
            .arguments
            .question
            .as_ref()
            .ok_or_else(|| Error::usage("missing question"))?;
        let value: serde_json::Value =
            serde_json::from_str(raw.get()).map_err(|_| Error::usage("invalid question"))?;
        if let Some(text) = value.as_str() {
            return self.text_question(text);
        }
        match self.tool {
            Tool::Annotate => QuestionSet::from_json(raw.get()).map(PreparedQuestion::Annotate),
            Tool::Rank => RankSet::from_json(raw.get()).map(PreparedQuestion::RankSet),
            Tool::Recognize => Recognize::from_json(raw.get()).map(PreparedQuestion::Recognize),
            Tool::Relate => Relate::from_json(raw.get()).map(PreparedQuestion::Relate),
            _ => Question::from_json(raw.get()).and_then(|q| self.atomic(q)),
        }
    }

    fn text_question(&self, text: &str) -> Result<PreparedQuestion, Error> {
        let question = match self.tool {
            Tool::Decide | Tool::Filter => Question::decide(text)?.cut(),
            Tool::Rank => Question::rank(text)?,
            Tool::Find => {
                let question = Question::find(text)?;
                if self.arguments.options.none {
                    question.offering_none()?
                } else {
                    question
                }
            }
            _ => {
                return Err(Error::usage(
                    "this function requires the ordinary JSON question grammar",
                ));
            }
        };
        Ok(PreparedQuestion::Atomic(LoadedQuestion::Question(question)))
    }

    fn atomic(&self, question: LoadedQuestion) -> Result<PreparedQuestion, Error> {
        use crate::QuestionKind as Kind;
        let expected = match self.tool {
            Tool::Decide | Tool::Filter => Kind::Decide,
            Tool::Choose => Kind::Choose,
            Tool::Tag => Kind::Tag,
            Tool::Score => Kind::Score,
            _ => {
                return Err(Error::usage(
                    "this function requires its set or plan grammar",
                ));
            }
        };
        if question.kind() != expected {
            return Err(Error::usage("question kind does not match the named tool"));
        }
        Ok(PreparedQuestion::Atomic(question))
    }

    /// Reuse native enumeration, bounded decode, order and original locations.
    pub(super) fn source(&self) -> Result<Option<crate::SourceItems>, Error> {
        self.arguments
            .source
            .as_ref()
            .map(|source| crate::read_inputs(&source.paths, source.options()))
            .transpose()
    }

    /// The shared reader decodes explicit attachments in authored order, with
    /// duplicates intact. Names remain positions, never model evidence.
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
        let images = crate::read_inputs(&self.arguments.images, options)?
            .map(|item| match item? {
                crate::SourceItem::Image(image) => Ok(image.record),
                crate::SourceItem::Text(_) => Err(Error::defect("image reader returned text")),
            })
            .collect::<Result<Vec<_>, Error>>()?;
        crate::ImageEvidence::new(self.arguments.evidence.clone(), images).map(Some)
    }
}
