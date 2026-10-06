//! Eager set rank on one question pipeline and one shared call control.
use std::fmt;
use std::path::Path;
use std::sync::Arc;

use crate::core::{self, pack, ranking, turns};
use crate::engine::pipeline::{Answered, Asker};
use crate::public::asking::{self, Decided, Decisions, Miss, Text};
use crate::public::bulk::selected_set_batch;
use crate::public::engine::{Engine, Evidence, evidence};
use crate::public::error::Error;
use crate::public::options::{CallOptions, Stop};
use crate::public::pull;
use crate::public::question::{Kind, Question};
use crate::public::results::{
    self, Call, ObservedRow, QuestionDetail, RecordObservation, SetRanked,
};

/// An ordered saved set of decide questions for rank. Authored thresholds,
/// pointers and other question kinds are refused before normalization.
/// Debug shows only the member count.
#[derive(Clone, PartialEq)]
pub struct RankSet(pub(crate) core::QuestionSet);

impl fmt::Debug for RankSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RankSet")
            .field("questions", &self.0.questions().len())
            .finish_non_exhaustive()
    }
}

impl RankSet {
    /// Parse the saved version-one grammar, preserving member order.
    ///
    /// # Errors
    /// Returns [`Error::Usage`] when the set breaks rank admission or grammar.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        core::QuestionSet::parse_rank(text)
            .map(Self)
            .map_err(Error::refused)
    }
    /// Load a saved set, capped at 1 MiB.
    ///
    /// # Errors
    /// Returns [`Error::Local`] for file or grammar failures.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = super::question_file::load_text(path.as_ref(), "rank question set")?;
        Self::from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }
}

struct SetDecisions {
    members: Vec<Decisions>,
    questions: Vec<Question>,
}

impl Asker for SetDecisions {
    type Input = Text;
    type Row = Vec<Result<Decided, Miss>>;
    type Error = Miss;

    fn label(&self, text: &Text) -> usize {
        text.at
    }

    fn asks(&self, text: &Text) -> Result<Vec<pack::Ask>, Miss> {
        let mut asks = Vec::new();
        for member in &self.members {
            asks.extend(member.asks(text)?);
        }
        Ok(asks)
    }

    fn row(&self, text: Text, answers: Vec<Answered>) -> Result<Self::Row, Miss> {
        let mut rest = answers.into_iter();
        let mut rows = Vec::with_capacity(self.members.len());
        for (member, question) in self.members.iter().zip(&self.questions) {
            let own = rest
                .by_ref()
                .take(pack::wire_count(&question.core))
                .collect();
            rows.push(member.row(
                Text {
                    at: text.at,
                    input: crate::public::QuestionInput::Text(String::new()),
                },
                own,
            ));
        }
        Ok(rows)
    }
}

impl Engine {
    /// Judge every member over every record, sort each member stably by yes
    /// probability, then visit each depth in saved member order. Duplicates
    /// consume a visit; each original input position appears only once.
    /// Originals need not implement Clone or Send. Facts count input records.
    ///
    /// # Errors
    /// Returns a preflight refusal before sends for blank or excessive input,
    /// or the whole call's error if any member, transport or control fails.
    #[expect(
        clippy::type_complexity,
        reason = "the additive result carries original items and shared facts"
    )]
    pub fn rank_set<I>(
        &self,
        questions: &RankSet,
        records: I,
    ) -> Result<Call<Vec<SetRanked<I::Item>>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.rank_set_with(questions, records, CallOptions::new())
    }

    /// [`Engine::rank_set`] under one shared cancellation, deadline, observer,
    /// context, batching and send budget. Returns the full merged order.
    ///
    /// # Errors
    /// As [`Engine::rank_set`].
    #[expect(
        clippy::type_complexity,
        reason = "the additive result carries original items and shared facts"
    )]
    pub fn rank_set_with<I>(
        &self,
        questions: &RankSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<SetRanked<I::Item>>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        let records = self.within_limit(records)?;
        for record in records.as_slice() {
            evidence(record.evidence())?;
        }
        let set = questions.0.clone();
        let setting = selected_set_batch(&set, &options, self.batch)?;
        let context = options.context_text().map(evidence).transpose()?;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let engine = Arc::clone(&self.inner);
        let questions: Vec<_> = set
            .questions()
            .iter()
            .map(|member| Question {
                core: member.question().clone(),
                threshold: None,
                model: None,
                profile: set.profile().cloned(),
                batch: None,
                kind: Kind::Rank,
            })
            .collect();
        let asker = SetDecisions {
            members: questions
                .iter()
                .map(|question| Decisions::new(&engine, question, context.clone()))
                .collect(),
            questions: questions.clone(),
        };
        let backend = engine.backend().clone();
        let call = pull::Call {
            engine,
            stop,
            packing: pull::packing(setting, context.is_some(), false),
            most: self.most,
        };
        let observed_set = set.clone();
        let mut batch = pull::start(
            call,
            asker,
            records,
            Box::new(move |stop, index, item, row| {
                let rows = row.map_err(asking::failure)?;
                let probabilities =
                    observed(&observed_set, &questions, &backend, stop, index, rows)?;
                let item = item.ok_or_else(|| Error::defect("a rank row lost its original"))?;
                Ok(Some((item, probabilities)))
            }),
        );
        let rows = batch.by_ref().collect::<Result<Vec<_>, _>>()?;
        let facts = batch
            .facts()
            .cloned()
            .ok_or_else(|| Error::defect("a completed set rank has no facts"))?;
        let lists = (0..set.questions().len())
            .map(|member| {
                let odds = rows
                    .iter()
                    .filter_map(|(_, values)| values.get(member).copied())
                    .collect::<Vec<_>>();
                ranking(&odds, None)
            })
            .collect::<Vec<_>>();
        let selected = turns(&lists, None);
        let mut rows: Vec<_> = rows.into_iter().map(Some).collect();
        let mut ranked = Vec::with_capacity(rows.len());
        for (index, member) in selected {
            let (item, probabilities) = rows
                .get_mut(index)
                .and_then(Option::take)
                .ok_or_else(|| Error::defect("a merged rank lost its original"))?;
            let probability = probabilities
                .get(member)
                .copied()
                .ok_or_else(|| Error::defect("a merged rank lost its probability"))?;
            let name = set
                .questions()
                .get(member)
                .ok_or_else(|| Error::defect("a merged rank lost its name"))?
                .name()
                .to_owned();
            ranked.push(SetRanked::new(index, item, probability, name));
        }
        Ok(Call::new(ranked, facts))
    }
}

fn observed(
    set: &core::QuestionSet,
    questions: &[Question],
    backend: &core::Backend,
    stop: &Stop<'_>,
    index: usize,
    rows: Vec<Result<Decided, Miss>>,
) -> Result<Vec<f64>, Error> {
    let mut probabilities = Vec::with_capacity(rows.len());
    let mut failed = false;
    for ((named, question), row) in set.questions().iter().zip(questions).zip(rows) {
        let decided = match row {
            Ok(decided) => decided,
            Err(Miss::Failed(decided)) => {
                failed = true;
                decided
            }
            Err(Miss::Refused(error)) => return Err(error),
        };
        if stop.observing() {
            let observed = decided.observed(question, backend)?;
            stop.observe(RecordObservation::Question {
                index,
                member: Some(named.name()),
                stage: None,
                position: 0,
                detail: QuestionDetail::of(&observed),
            });
            if stop.observer_panicked() {
                return Err(Error::cancelled());
            }
        }
        if let Some(judged) = decided.judgment(question) {
            probabilities.push(
                judged
                    .answer
                    .yes()
                    .ok_or_else(|| Error::defect("a decide answer lost its probability"))?,
            );
        }
    }
    if failed {
        return Err(asking::backend_failed());
    }
    if stop.observing() {
        stop.observe(RecordObservation::Row {
            index,
            value: ObservedRow::Judgment(&results::judgment(&core::Value::YesNo(None))),
        });
        if stop.observer_panicked() {
            return Err(Error::cancelled());
        }
    }
    Ok(probabilities)
}
