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
mod retention;

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
    #[expect(
        clippy::type_complexity,
        reason = "set rank retains original and complete members"
    )]
    pub(crate) fn request_rank_set_records_complete_with<'a, I, T>(
        &'a self,
        set: &'a RankSet,
        records: I,
        options: CallOptions<'a>,
        top: Option<usize>,
        release: Option<&dyn Fn(usize)>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        if release.is_some() {
            return self.live_rank_set(set, records, options, top, release);
        }
        self.try_rank_set_records_complete_with(set, records, options)
    }
    #[expect(
        clippy::type_complexity,
        reason = "set rank retains original and complete members"
    )]
    fn live_rank_set<'a, I, T>(
        &'a self,
        set: &'a RankSet,
        records: I,
        options: CallOptions<'a>,
        top: Option<usize>,
        release: Option<&dyn Fn(usize)>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        let questions = questions(&set.0);
        let setting = crate::public::bulk::selected_set_batch(&set.0, &options, self.batch)?;
        let engine = Arc::clone(&self.inner);
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let captured = stop.facts().attempts().is_some();
        let mut packing = pull::packing(setting, false, false);
        packing.detailed = captured;
        let prepared = SetRecords {
            records: Records(Arc::clone(&engine), false),
            questions: questions.clone(),
        };
        let own_questions = questions.clone();
        let records = records.into_iter().enumerate().map(move |(at, row)| {
            let (held, input) = prepare_one(&own_questions, row?, &options, at)?;
            prepared.asks(&input).map_err(pipeline_failure)?;
            Ok((held, input))
        });
        let observed_engine = Arc::clone(&engine);
        let observed_questions = questions.clone();
        let mut batch = pull::try_start_prepared(
            pull::Call {
                engine: Arc::clone(&engine),
                stop,
                packing,
                most: self.most,
            },
            SetRecords {
                records: Records(Arc::clone(&engine), false),
                questions: questions.clone(),
            },
            records,
            Box::new(|_, original: &(Held<T>, Input)| {
                Ok(Input {
                    at: original.1.at,
                    members: original.1.members.iter().map(Prepared::duplicate).collect(),
                })
            }),
            Box::new(move |stop, at, original, row| {
                let row = row.map_err(crate::public::asking::failure)?;
                let values = crate::public::rank_set::observed_judgments(
                    &set.0,
                    &observed_questions,
                    observed_engine.backend(),
                    stop,
                    at,
                    row,
                )?;
                let (held, _) =
                    original.ok_or_else(|| Error::defect("set rank lost its original"))?;
                Ok(Some((at, held, values)))
            }),
        );
        let mut retained = retention::Retained::new(top, questions.len());
        for row in batch.by_ref() {
            let (at, held, values) = row?;
            let weights = values
                .iter()
                .map(|(judged, _)| judged.answer.yes().ok_or_else(super::wrong))
                .collect::<Result<Vec<_>, Error>>()?;
            retained.take(at, (held, values), &weights, release)?;
        }
        let facts = batch
            .facts()
            .cloned()
            .ok_or_else(|| Error::defect("completed set rank has no facts"))?;
        Call::new(retained, facts).try_map(|retained| {
            let (rows, selected, positions) = retained.finish()?;
            render::selected_rows(
                self, &engine, &set.0, &questions, setting, rows, selected, positions, captured,
            )
        })
    }

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
                seed_spans: None,
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
        let (original, prepared) = prepare_one(questions, record, options, at)?;
        held.push(original);
        inputs.push(prepared);
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

fn prepare_one<T: InputEvidence>(
    questions: &[Question],
    record: RecordInput<T>,
    options: &CallOptions<'_>,
    at: usize,
) -> Result<(Held<T>, Input), Error> {
    options.admission()?;
    if record.examples.is_some() {
        return Err(Error::usage("record examples are admitted only for recognize").at_record(at));
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
                seed_spans: None,
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
    let held = Held {
        source: super::physical_source(&input),
        original: record.original,
        context_sha256,
    };
    Ok((held, Input { at, members }))
}
