//! Native set rank uses ordinary records, one call and the shared turns merge.
use super::records::{Prepared, Records, pipeline_failure, prepare_record};
use crate::core::{self, pack};
use crate::engine::{
    facade,
    pipeline::{self, Answered, Asker, Flow},
};
use crate::public::{
    Call, CallOptions, CompleteRank, CompleteRankMember, CompleteRecord, CompleteSetRank, Engine,
    Error, InputEvidence, InputFunction, Question, RankSet, RecordInput,
    asking::{Decided, Miss},
    options::Stop,
    pull,
};
use std::{num::NonZeroUsize, sync::Arc};
mod render;

struct Input {
    at: usize,
    members: Vec<Prepared>,
}
struct SetRecords {
    records: Records,
    questions: Vec<Question>,
}
impl Asker for SetRecords {
    type Input = Input;
    type Row = Vec<Result<Decided, Miss>>;
    type Error = Miss;
    fn label(&self, input: &Input) -> usize {
        input.at
    }
    fn asks(&self, input: &Input) -> Result<Vec<pack::Ask>, Miss> {
        let mut asks = Vec::new();
        for member in &input.members {
            asks.extend(self.records.asks(member)?);
        }
        Ok(asks)
    }
    fn row(&self, input: Input, answers: Vec<Answered>) -> Result<Self::Row, Miss> {
        let mut answers = answers.into_iter();
        let mut rows = Vec::with_capacity(input.members.len());
        for (member, question) in input.members.into_iter().zip(&self.questions) {
            let own = answers
                .by_ref()
                .take(pack::wire_count(&question.core))
                .collect();
            rows.push(self.records.row(member, own));
        }
        if answers.next().is_some() {
            return Err(Miss::Refused(Error::defect("a set rank has extra answers")));
        }
        Ok(rows)
    }
}
struct Held<T> {
    original: T,
    context_sha256: Option<String>,
    source: Option<core::CompletePhysicalSource>,
}
type Admission<T> = (Vec<Held<T>>, Vec<Input>);

impl Engine {
    /// Complete saved decide-set rank, retaining all member observations and originals.
    /// Per-item context uses the existing record composition; images/options are refused.
    /// # Errors
    /// All materialized inputs are admitted before lookup/send. Started failures retain facts.
    #[expect(
        clippy::type_complexity,
        reason = "each original retains complete member and final rank results"
    )]
    pub fn rank_set_records_complete_with<I, T>(
        &self,
        set: &RankSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        let options = options.started()?;
        options.admission()?;
        let questions = questions(&set.0);
        let setting = crate::public::bulk::selected_set_batch(&set.0, &options, self.batch)?;
        let (held, inputs) = prepare(
            &questions,
            self.within_admission(records, &options)?,
            &options,
        )?;
        let engine = Arc::clone(&self.inner);
        let asker = SetRecords {
            records: Records(Arc::clone(&engine), false),
            questions: questions.clone(),
        };
        for input in &inputs {
            asker.asks(input).map_err(pipeline_failure)?;
        }
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let captured = stop.facts().attempts().is_some();
        let mut packing = pull::packing(setting, false, false);
        packing.detailed = captured;
        stop.run_call(0, |cancel| {
            let mut rows = Rows {
                set: &set.0,
                questions: &questions,
                backend: engine.backend(),
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
                return Err(Error::defect("a set rank lost an original"));
            }
            Ok(rows.values)
        })?
        .try_map(|rows| {
            render::ranked(
                self, &engine, &set.0, &questions, setting, held, rows, captured,
            )
        })
    }
    /// Complete set rank for plain originals under one optional shared text context.
    /// # Errors
    /// As rank_set_records_complete_with; released rank_set convenience calls remain intact.
    #[expect(
        clippy::type_complexity,
        reason = "each original retains complete member and final rank results"
    )]
    pub fn rank_set_complete_with<I, T>(
        &self,
        set: &RankSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error>
    where
        I: IntoIterator<Item = T>,
        T: InputEvidence,
    {
        self.rank_set_records_complete_with(
            set,
            records.into_iter().map(|original| RecordInput {
                examples: None,
                original,
                context: None,
                options: None,
            }),
            options,
        )
    }
    /// Admit a fallible whole original set before complete set ranking.
    /// # Errors
    /// Reader failures refuse before sending; otherwise as rank_set_records_complete_with.
    #[expect(
        clippy::type_complexity,
        reason = "each original retains complete member and final rank results"
    )]
    pub fn try_rank_set_records_complete_with<I, T>(
        &self,
        set: &RankSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>>,
        T: InputEvidence,
    {
        let options = options.started()?;
        let records = self.try_within_admission(
            records
                .into_iter()
                .enumerate()
                .map(|(at, record)| record.map_err(|error| error.at_record(at))),
            &options,
        )?;
        self.rank_set_records_complete_with(set, records, options)
    }
}
fn questions(set: &core::QuestionSet) -> Vec<Question> {
    crate::public::rank_set::member_questions(set)
}
fn prepare<T: InputEvidence>(
    questions: &[Question],
    records: impl Iterator<Item = RecordInput<T>>,
    options: &CallOptions<'_>,
) -> Result<Admission<T>, Error> {
    let mut held = Vec::new();
    let mut inputs = Vec::new();
    for (at, record) in records.enumerate() {
        options.admission()?;
        if record.examples.is_some() {
            return Err(
                Error::usage("record examples are admitted only for recognize").at_record(at),
            );
        }
        if record.options.is_some() {
            return Err(Error::usage("rank accepts no per-item options").at_record(at));
        }
        let input = record.original.question_input();
        options.admission()?;
        let mut members = Vec::with_capacity(questions.len());
        let mut context_sha256 = None;
        for question in questions {
            let (item, prepared) = prepare_record(
                InputFunction::Rank,
                question,
                RecordInput {
                    examples: None,
                    original: input.clone(),
                    context: record.context.clone(),
                    options: None,
                },
                options.context_text(),
                at,
            )?;
            context_sha256 = item.context_sha256;
            members.push(prepared);
        }
        held.push(Held {
            source: super::physical_source(&input),
            original: record.original,
            context_sha256,
        });
        inputs.push(Input { at, members });
    }
    Ok((held, inputs))
}

struct Rows<'a, 'o> {
    set: &'a core::QuestionSet,
    questions: &'a [Question],
    backend: &'a core::Backend,
    stop: &'a Stop<'o>,
    values: Vec<Vec<crate::public::engine::Keyed>>,
    failure: Option<Error>,
}
impl Rows<'_, '_> {
    fn take(&mut self, row: pull::Row<SetRecords>) -> Flow {
        let at = self.values.len();
        let result = row
            .map_err(crate::public::asking::failure)
            .and_then(|rows| {
                crate::public::rank_set::observed_judgments(
                    self.set,
                    self.questions,
                    self.backend,
                    self.stop,
                    at,
                    rows,
                )
            });
        match result {
            Ok(row) => {
                self.values.push(row);
                Flow::Continue
            }
            Err(error) => {
                self.failure = Some(error.at_record(at));
                Flow::Stop
            }
        }
    }
}
