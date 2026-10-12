//! Native set rank uses ordinary records, one call and the shared turns merge.
use super::records::{Prepared, Records, pipeline_failure, prepare_record};
use crate::core::{self, pack};
use crate::engine::{
    facade,
    pipeline::{Answered, Asker},
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
        let records = self.try_within_admission(records, &options)?;
        self.execute_rank_set_records_complete_with(set, records, options)
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
        let records = self.admit_prepared_stream(records, &options)?;
        let observed_engine = Arc::clone(&engine);
        let observed_questions = questions.clone();
        let mut batch = pull::try_start_prepared(
            pull::Call {
                engine: Arc::clone(&engine),
                stop,
                packing,
                most: self.most,
                aggregate_inputs: options.eager_inputs,
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
    pub(crate) fn rank_set_records_complete_with<I, T>(
        &self,
        set: &RankSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        crate::public::request::native(
            self,
            crate::RequestFunction::Rank,
            set.clone().into(),
            records.into_iter().map(Ok),
            options,
            |engine, definition, records, controls| {
                let q = definition.rank_set()?;
                Ok(crate::public::request::restore(
                    engine.execute_rank_set_records_complete_with(&q, records, controls)?,
                ))
            },
        )
    }

    #[allow(
        clippy::type_complexity,
        reason = "the private Request primitive retains original and complete result types"
    )]
    pub(crate) fn execute_rank_set_records_complete_with<I, T>(
        &self,
        set: &RankSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteSetRank>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        let mut options = options.started()?;
        options.eager_inputs = true;
        self.live_rank_set(set, records.into_iter().map(Ok), options, None, None)
    }
    /// Complete set rank for plain originals under one optional shared text context.
    /// # Errors
    /// As rank_set_records_complete_with; released rank_set convenience calls remain intact.
    #[expect(
        clippy::type_complexity,
        reason = "each original retains complete member and final rank results"
    )]
    pub(crate) fn rank_set_complete_with<I, T>(
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
            records
                .into_iter()
                .map(|original| crate::public::record_input::plain(original)),
            options,
        )
    }
}
fn questions(set: &core::QuestionSet) -> Vec<Question> {
    crate::public::rank_set::member_questions(set)
}
fn prepare_one<T: InputEvidence>(
    questions: &[Question],
    record: RecordInput<T>,
    options: &CallOptions<'_>,
    at: usize,
) -> Result<(Held<T>, Input), Error> {
    options.admission()?;
    record
        .admit_recognition_controls(
            crate::public::request::RequestFunction::from_input(InputFunction::Rank),
            "record examples are admitted only for recognize",
            "record seed spans are admitted only for recognize",
        )
        .map_err(|error| error.at_record(at))?;
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

pub(super) fn preview_asks(
    engine: &Arc<facade::Engine>,
    set: &RankSet,
    record: RecordInput<crate::QuestionInput>,
    options: &CallOptions<'_>,
    at: usize,
) -> Result<(Vec<pack::Ask>, bool), Error> {
    let questions = questions(&set.0);
    let (_, input) = prepare_one(&questions, record, options, at)?;
    let dropped = questions.iter().any(|q| {
        crate::core::adapters::built_in::drops_detail_of(engine.backend().descriptions(), &q.core)
    });
    let asks = SetRecords {
        records: Records(Arc::clone(engine), false),
        questions,
    }
    .asks(&input)
    .map_err(pipeline_failure)?;
    Ok((asks, dropped))
}
