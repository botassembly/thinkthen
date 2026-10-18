//! Grouped annotation of each input record on the question pipeline, by ADR
//! 0111 section 5. Every group's questions share the fixed state, so records
//! and groups pack together, and each row gathers its questions back.

use super::Values;
use crate::CompleteRecord;
use crate::core::{self, pack, pack::Ask, quoted_plan};
use crate::engine::facade::{self, Annotation, GroupAnswer, QuestionAnswer};
use crate::engine::pipeline::{Answered, Asker};
use crate::public::annotated::NamedAnnotation;
use crate::public::asking::Text;
use crate::public::engine::{Engine, Evidence, evidence};
use crate::public::error::Error;
use crate::public::options::CallOptions;
use crate::public::results::ObservedQuestion;
use crate::public::set::QuestionSet;

/// What one question set needs of each text beside the text.
pub(crate) struct Annotating {
    require_usable_groups: bool,
    set: core::QuestionSet,
    groups: Vec<Vec<usize>>,
    backend: core::Backend,
    profile: Option<core::BackendProfile>,
    context: Option<core::Json>,
    shared_context: Option<core::Evidence>,
}

impl Annotating {
    fn input_record(&self, text: &Text) -> Result<core::BatchRecord, Error> {
        match &text.input {
            crate::public::QuestionInput::Record(record) if record.images().is_empty() => {
                Ok(record.batch_record())
            }
            _ => record(
                &self.set,
                text.plain(crate::public::InputFunction::Annotate)?,
            ),
        }
    }

    pub(crate) fn with_typed_context(
        mut self,
        context: Option<&core::Evidence>,
        explicit: bool,
    ) -> Self {
        if explicit {
            self.context = context.map(core::Evidence::as_json);
        } else {
            self.shared_context = context.cloned();
        }
        self
    }

    pub(crate) fn new(engine: &facade::Engine, set: core::QuestionSet) -> Self {
        Self {
            require_usable_groups: false,
            groups: set.groups(),
            set,
            backend: engine.backend().clone(),
            profile: engine.profile().cloned(),
            context: None,
            shared_context: None,
        }
    }

    pub(crate) const fn require_usable_groups(mut self, required: bool) -> Self {
        self.require_usable_groups = required;
        self
    }

    pub(crate) fn asks_with_groups(
        &self,
        text: &Text,
        mut group_done: impl FnMut(usize, usize),
    ) -> Result<Vec<Ask>, Error> {
        let record = self.input_record(text)?;
        let mut asks = Vec::new();
        for (group, places) in self.groups.iter().enumerate() {
            let evidence =
                self.set
                    .group_evidence(places, &record)
                    .map_err(|error| match error {
                        core::PartError::Declaration(_) => {
                            Error::usage("the item does not match item_schema")
                        }
                        core::PartError::Record(error) => Error::refused(error),
                        core::PartError::Reading(error) => Error::refused(error),
                    })?;
            let questions = places
                .iter()
                .map(|&place| self.question(place))
                .collect::<Result<Vec<_>, _>>()?;
            let plan = quoted_plan(
                self.backend.asked(),
                evidence,
                self.shared_context.as_ref(),
                questions,
                self.profile.as_ref(),
            )
            .map_err(|error| match error {
                core::BatchError::Profile(limit) => {
                    Error::from(crate::engine::error::Error::ProfileLimit(limit))
                }
                core::BatchError::StructuredQuestionWithContext => Error::refused(error),
                _ => Error::defect("an annotate group asks nothing"),
            })?;
            let grouped = pack::asks_for(self.backend.api_type(), self.backend.url(), &plan)
                .map_err(|_| Error::defect("a request could not be written as JSON"))?;
            group_done(group, grouped.len());
            asks.extend(grouped);
        }
        if let Some(context) = &self.context {
            let model = pack::model_json(self.backend.model().as_str())
                .map_err(|_| Error::defect("an aggregate model could not be written"))?;
            for ask in &mut asks {
                ask.state = ask
                    .state
                    .with_context_value(context)
                    .map_err(|_| Error::defect("an aggregate context could not be written"))?;
                ask.key = ask.state.key(self.backend.url(), &model, &ask.question);
            }
        }
        Ok(asks)
    }

    fn question(&self, place: usize) -> Result<core::Question, Error> {
        self.set
            .questions()
            .get(place)
            .map(|named| named.question().clone())
            .ok_or_else(|| Error::defect("a group points outside its set"))
    }
}

pub(crate) fn admit_declarations(
    set: &core::QuestionSet,
    record: &core::BatchRecord,
) -> Result<(), Error> {
    for places in set.groups() {
        match set.group_evidence(&places, record) {
            Ok(_) | Err(core::PartError::Record(core::RecordError::Missed(_))) => {}
            Err(core::PartError::Declaration(_)) => {
                return Err(Error::usage("the item does not match item_schema"));
            }
            Err(core::PartError::Record(error)) => return Err(Error::refused(error)),
            Err(core::PartError::Reading(error)) => return Err(Error::refused(error)),
        }
    }
    Ok(())
}

impl Asker for Annotating {
    type Input = Text;
    type Row = Annotation;
    type Error = Error;
    fn validates_batches(&self) -> bool {
        self.set.questions().iter().any(|member| {
            member.metadata().item_schema.is_some() || member.metadata().context_schema.is_some()
        })
    }

    fn label(&self, text: &Text) -> usize {
        text.at
    }

    fn asks(&self, text: &Text) -> Result<Vec<Ask>, Error> {
        self.asks_with_groups(text, |_, _| {})
    }

    fn row(&self, _text: Text, answers: Vec<Answered>) -> Result<Annotation, Error> {
        let mut rest = answers.as_slice();
        let mut groups = Vec::with_capacity(self.groups.len());
        for places in &self.groups {
            let mut questions = Vec::with_capacity(places.len());
            let mut usable = false;
            let mut failed = None;
            for &place in places {
                let question = self.question(place)?;
                let (own, after) = rest
                    .split_at_checked(pack::wire_count(&question))
                    .ok_or_else(|| Error::defect("an annotate question lost its answers"))?;
                rest = after;
                let answer = QuestionAnswer::read(place, question, own).map_err(Error::from)?;
                usable |= matches!(answer.reply.outcomes(), [core::AnswerOutcome::Answered(_)]);
                failed = failed.or_else(|| first_failure(own));
                questions.push(answer);
            }
            if self.require_usable_groups && !usable {
                let (cause, (first, last)) = match failed {
                    Some((cause, span)) => (Some(cause), span),
                    None => (None, (0, 0)),
                };
                return Err(crate::public::asking::backend_failed().with_diagnostic(
                    crate::public::error::diagnostic::Diagnostic::PartialReply {
                        cause,
                        first,
                        last,
                    },
                ));
            }
            groups.push(GroupAnswer::of_questions(questions));
        }
        facade::assemble(&self.set, groups, self.backend.model()).map_err(Error::from)
    }
}

fn first_failure(
    own: &[Answered],
) -> Option<(core::adapters::built_in::DecodeError, (usize, usize))> {
    own.iter().find_map(|answered| {
        answered
            .answer
            .as_ref()
            .err()
            .map(|cause| (cause.clone(), answered.span))
    })
}

impl Engine {
    /// Each record with every value of the set, lazily, in input order. A
    /// member with an `on` pointer reads that part of the record's JSON text,
    /// and a record missing a part is refused before its first request. A
    /// question the backend failed reads [`Annotated::Failed`](crate::Annotated::Failed).
    pub fn annotate<'a, I>(
        &'a self,
        questions: &'a QuestionSet,
        records: I,
    ) -> Result<crate::AnnotationCall<I::Item>, Error>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        self.annotate_with(questions, records, CallOptions::new())
    }

    /// [`Engine::annotate`] under these controls.
    pub fn annotate_with<'a, I>(
        &'a self,
        questions: &'a QuestionSet,
        records: I,
        options: CallOptions<'a>,
    ) -> Result<crate::AnnotationCall<I::Item>, Error>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence,
    {
        crate::public::request::pull::annotation(
            self,
            questions.clone(),
            records
                .into_iter()
                .map(|row| Ok(crate::public::record_input::plain(row))),
            options,
            |original, ordinal, result| {
                Ok(Some(CompleteRecord {
                    original,
                    ordinal,
                    result,
                }))
            },
        )?
        .into_call()?
        .project(|rows| {
            let values = rows
                .iter()
                .map(|row| {
                    row.result
                        .canonical
                        .reading
                        .value()
                        .clone()
                        .into_values()
                        .into_iter()
                        .map(|(name, value)| NamedAnnotation::of(name, value))
                        .collect()
                })
                .collect();
            Ok((values, rows, None::<f64>))
        })
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
            .zip(set.questions())
            .map(|(((name, entry), receipt), named)| {
                let detail = ObservedQuestion::from_annotated(
                    entry,
                    set.profile(),
                    engine.backend(),
                    model,
                    (receipt.usage, receipt.requests_sent, receipt.replayed),
                )?
                .with_receipt(&receipt.trace)
                .with_threshold(named.threshold())
                .with_declarations(named.metadata());
                Ok((name.clone(), detail))
            })
            .collect::<Result<Vec<_>, Error>>()?
    } else {
        Vec::new()
    };
    Ok(Values {
        values: annotation.values,
        observed,
    })
}

/// One record as its groups read it. Only a part group parses the text, once,
/// as the command reads a whole document, so a root-only set sends it as given.
pub(crate) fn record(_set: &core::QuestionSet, text: &str) -> Result<core::BatchRecord, Error> {
    let evidence = evidence(text)?;
    let usage = |error: core::RecordError| Error::usage(error.to_string());
    let reading = core::Reading::new(core::Framing::Document, Vec::new())
        .map_err(|_| Error::defect("a document reading takes no pointer"))?;
    let held = reading.annotation_record(text.as_bytes()).map_err(usage)?;
    let value = reading.batch_record(&held).map_err(usage)?.value;
    Ok(core::BatchRecord { evidence, value })
}
