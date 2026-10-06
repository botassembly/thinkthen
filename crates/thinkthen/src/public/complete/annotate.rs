//! Native original occurrences annotate on the existing ordered group pipeline.
use crate::core::{self, pack::Ask};
use crate::engine::{
    facade,
    pipeline::{self, Asker, Flow},
};
use crate::public::{
    Call, CallOptions, CompleteAnnotated, CompleteRecord, Engine, Error, InputEvidence,
    InputFunction, QuestionInput, QuestionSet, RecordInput,
};
use crate::public::{asking::Text, bulk::annotation::Annotating, options::Stop, pull};
use std::sync::Arc;
mod render;

struct Prepared {
    text: Text,
    context: Option<String>,
}
struct Held<T> {
    original: T,
    input: core::Record,
    context: Option<String>,
}
struct Annotations {
    engine: Arc<facade::Engine>,
    set: core::QuestionSet,
}
impl Asker for Annotations {
    type Input = Prepared;
    type Row = facade::Annotation;
    type Error = Error;
    fn label(&self, input: &Prepared) -> usize {
        input.text.at
    }
    fn asks(&self, input: &Prepared) -> Result<Vec<Ask>, Error> {
        Annotating::new(&self.engine, self.set.clone())
            .with_context(input.context.as_deref())?
            .asks(&input.text)
    }
    fn row(
        &self,
        input: Prepared,
        answers: Vec<pipeline::Answered>,
    ) -> Result<facade::Annotation, Error> {
        Annotating::new(&self.engine, self.set.clone()).row(input.text, answers)
    }
}
impl Engine {
    /// Annotate every original, retaining successful null and failed member identities.
    /// # Errors
    /// Eager invalid records refuse before sending; all-failed rows and stopped calls retain final facts.
    #[allow(
        clippy::type_complexity,
        reason = "Every original retains its concrete complete annotation"
    )]
    pub fn annotate_records_complete_with<I, T>(
        &self,
        questions: &QuestionSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        let setting = crate::public::bulk::selected_set_batch(&questions.0, &options, self.batch)?;
        let (held, inputs) = prepare(self.within_limit(records)?, options.context_text())?;
        let engine = Arc::clone(&self.inner);
        let asker = Annotations {
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        for input in &inputs {
            asker.asks(input)?;
        }
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let mut packing = pull::packing(setting, false, false);
        packing.detailed = stop.facts().attempts().is_some();
        stop.run_call(0, |cancel| {
            let mut rows = Rows {
                held: &held,
                engine: &engine,
                set: &questions.0,
                stop: &stop,
                values: Vec::new(),
                failure: None,
            };
            let host = pipeline::eager(inputs, |row| {
                let flow = rows.take(row);
                cancel.finished_records(usize::from(flow == Flow::Continue));
                flow
            });
            engine
                .ask_all(&asker, packing, host, cancel)
                .map_err(Error::from)?;
            if let Some(error) = rows.failure {
                return Err(error);
            }
            if rows.values.len() != held.len() {
                return Err(Error::defect("an annotate call lost an original"));
            }
            Ok(rows
                .values
                .into_iter()
                .zip(held)
                .enumerate()
                .map(|(at, (result, held))| CompleteRecord {
                    original: held.original,
                    ordinal: at,
                    result,
                })
                .collect())
        })
    }
    /// Annotate plain originals using one optional shared context.
    /// # Errors
    /// As annotate_records_complete_with.
    #[allow(
        clippy::type_complexity,
        reason = "Every original retains its concrete complete annotation"
    )]
    pub fn annotate_complete_with<I, T>(
        &self,
        questions: &QuestionSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error>
    where
        I: IntoIterator<Item = T>,
        T: InputEvidence,
    {
        self.annotate_records_complete_with(
            questions,
            records.into_iter().map(|original| RecordInput {
                original,
                context: None,
                options: None,
            }),
            options,
        )
    }
}

type Admission<T> = (Vec<Held<T>>, Vec<Prepared>);
fn prepare<T: InputEvidence>(
    records: impl Iterator<Item = RecordInput<T>>,
    fallback: Option<&str>,
) -> Result<Admission<T>, Error> {
    let reading =
        core::Reading::new(core::Framing::Document, Vec::new()).map_err(Error::refused)?;
    let mut held = Vec::new();
    let mut inputs = Vec::new();
    for (at, record) in records.enumerate() {
        if record.options.is_some() {
            return Err(Error::usage("record options are admitted only for choose"));
        }
        let input = record.original.question_input();
        crate::public::images::guard(InputFunction::Annotate, &input)?;
        let QuestionInput::Text(text) = input else {
            return Err(super::wrong());
        };
        crate::public::engine::evidence(&text)?;
        let input = reading
            .annotation_record(text.as_bytes())
            .map_err(Error::refused)?;
        let context = record
            .context
            .as_deref()
            .or(fallback)
            .filter(|text| !text.is_empty())
            .map(|text| {
                crate::public::engine::evidence(text)?;
                Ok::<_, Error>(text.to_owned())
            })
            .transpose()?;
        inputs.push(Prepared {
            text: Text {
                at,
                input: QuestionInput::Text(text),
            },
            context: context.clone(),
        });
        held.push(Held {
            original: record.original,
            input,
            context,
        });
    }
    Ok((held, inputs))
}
fn failed(row: pipeline::Failed<Error>) -> Error {
    match row {
        pipeline::Failed::Asker(error) => error,
        pipeline::Failed::Pack { error, .. } => crate::public::asking::packed(error),
        pipeline::Failed::Engine { error, .. } | pipeline::Failed::Stopped(error) => {
            Error::from(error)
        }
    }
}

struct Rows<'a, 'o, T> {
    held: &'a [Held<T>],
    engine: &'a facade::Engine,
    set: &'a core::QuestionSet,
    stop: &'a Stop<'o>,
    values: Vec<CompleteAnnotated>,
    failure: Option<Error>,
}
impl<T> Rows<'_, '_, T> {
    fn take(&mut self, row: pull::Row<Annotations>) -> Flow {
        match self.completed(row) {
            Ok(value) => {
                self.values.push(value);
                Flow::Continue
            }
            Err(error) => {
                self.failure = Some(error);
                Flow::Stop
            }
        }
    }
    fn completed(&self, row: pull::Row<Annotations>) -> Result<CompleteAnnotated, Error> {
        let annotation = row.map_err(failed)?;
        let at = self.values.len();
        let held = self.held.get(at).ok_or_else(super::wrong)?;
        let observed = crate::public::bulk::annotation::rendered(
            self.set,
            self.engine,
            annotation.clone(),
            self.stop.observing(),
        )?;
        crate::public::bulk::observe_annotated(&observed, at, self.stop)?;
        if annotation.failed_questions == annotation.details.len() && !annotation.details.is_empty()
        {
            return Err(crate::public::asking::backend_failed());
        }
        render::complete(
            self.engine,
            self.set,
            annotation,
            held,
            at,
            self.stop.facts().attempts().is_some(),
        )
    }
}
