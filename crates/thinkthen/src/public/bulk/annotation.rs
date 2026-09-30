//! Grouped annotation of each input record on the question pipeline, by ADR
//! 0111 section 5. Every group's questions share the fixed state, so records
//! and groups pack together, and each row gathers its questions back.

use super::{Values, evidence};
use crate::core::{self, Json, pack, pack::Ask, quoted_plan};
use crate::engine::facade::{self, Annotation, GroupAnswer, QuestionAnswer};
use crate::engine::pipeline::{Answered, Asker, Failed};
use crate::public::asking::{Text, packed};
use crate::public::error::Error;
use crate::public::results::{ObservedQuestion, Written};

/// What one question set needs of each text beside the text.
pub(crate) struct Annotating {
    set: core::QuestionSet,
    groups: Vec<Vec<usize>>,
    backend: core::Backend,
    profile: Option<core::BackendProfile>,
}

impl Annotating {
    pub(crate) fn new(engine: &facade::Engine, set: core::QuestionSet) -> Self {
        Self {
            groups: set.groups(),
            set,
            backend: engine.backend().clone(),
            profile: engine.profile().cloned(),
        }
    }

    fn question(&self, place: usize) -> Result<core::Question, Error> {
        self.set
            .questions()
            .get(place)
            .map(|named| named.question().clone())
            .ok_or_else(|| Error::defect("a group points outside its set"))
    }
}

impl Asker for Annotating {
    type Input = Text;
    type Row = Annotation;
    type Error = Error;

    fn label(&self, text: &Text) -> usize {
        text.at
    }

    fn asks(&self, text: &Text) -> Result<Vec<Ask>, Error> {
        let record = record(&self.set, &text.text)?;
        let mut asks = Vec::new();
        for places in &self.groups {
            let evidence = self
                .set
                .group_evidence(places, &record)
                .map_err(|error| match error {
                    core::PartError::Record(error) => Error::refused(error),
                    core::PartError::Reading(_) => {
                        Error::defect("a checked question set could not read its parts")
                    }
                })?;
            let questions = places
                .iter()
                .map(|&place| self.question(place))
                .collect::<Result<Vec<_>, _>>()?;
            let plan = quoted_plan(
                self.backend.model().clone(),
                evidence,
                None,
                questions,
                self.profile.as_ref(),
            )
            .map_err(|error| match error {
                core::BatchError::Profile(_) => Error::refused(error),
                _ => Error::defect("an annotate group asks nothing"),
            })?;
            asks.extend(
                pack::asks(self.backend.url(), &plan)
                    .map_err(|_| Error::defect("a request could not be written as JSON"))?,
            );
        }
        Ok(asks)
    }

    fn row(&self, _text: Text, answers: Vec<Answered>) -> Result<Annotation, Error> {
        let mut rest = answers.as_slice();
        let mut groups = Vec::with_capacity(self.groups.len());
        for places in &self.groups {
            let mut questions = Vec::with_capacity(places.len());
            for &place in places {
                let question = self.question(place)?;
                let (own, after) = rest
                    .split_at_checked(pack::wire_count(&question))
                    .ok_or_else(|| Error::defect("an annotate question lost its answers"))?;
                rest = after;
                questions.push(QuestionAnswer::read(place, question, own).map_err(Error::from)?);
            }
            groups.push(GroupAnswer::of_questions(questions));
        }
        facade::assemble(&self.set, groups, self.backend.model()).map_err(Error::from)
    }
}

/// The public error for one annotate row with no annotation.
pub(crate) fn failure(failed: Failed<Error>) -> Error {
    match failed {
        Failed::Asker(error) => error,
        Failed::Pack { error, .. } => packed(error),
        Failed::Engine { error, .. } | Failed::Stopped(error) => Error::from(error),
    }
}

/// Render a row once all its questions arrive.
pub(crate) fn rendered(
    set: &core::QuestionSet,
    engine: &facade::Engine,
    annotation: Annotation,
    observing: bool,
) -> Result<Values, Error> {
    let observed = if observing {
        let model = annotation
            .model
            .as_ref()
            .unwrap_or(engine.backend().model());
        annotation
            .details
            .iter()
            .zip(&annotation.receipts)
            .map(|((name, entry), receipt)| {
                let detail = ObservedQuestion::from_annotated(
                    entry,
                    set.profile(),
                    engine.backend(),
                    model,
                    (receipt.usage, receipt.requests_sent, receipt.replayed),
                )?;
                Ok((name.clone(), detail))
            })
            .collect::<Result<Vec<_>, Error>>()?
    } else {
        Vec::new()
    };
    let json = Written::of(&core::NamedValues::new(annotation.values.clone()))?;
    Ok(Values {
        values: annotation.values,
        json,
        observed,
    })
}

/// One record as its groups read it. Only a part group parses the text, once,
/// as the command reads a whole document, so a root-only set sends it as given.
pub(crate) fn record(set: &core::QuestionSet, text: &str) -> Result<core::BatchRecord, Error> {
    let evidence = evidence(text)?;
    if set.first_part().is_none() {
        let value = Json::String(text.to_owned());
        return Ok(core::BatchRecord { evidence, value });
    }
    let usage = |error: core::RecordError| Error::usage(error.to_string());
    let reading = core::Reading::new(core::Framing::Document, Vec::new())
        .map_err(|_| Error::defect("a document reading takes no pointer"))?;
    let held = reading.annotation_record(text.as_bytes()).map_err(usage)?;
    let value = reading.batch_record(&held).map_err(usage)?.value;
    Ok(core::BatchRecord { evidence, value })
}
