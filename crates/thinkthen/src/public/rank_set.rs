//! Eager set rank on one question pipeline and one shared call control.
use std::fmt;
use std::path::Path;

use crate::core;
use crate::public::asking::{self, Decided, Miss};
use crate::public::engine::{Engine, Evidence};
use crate::public::error::Error;
use crate::public::options::{CallOptions, Stop};
use crate::public::question::{Kind, Question};
use crate::public::results::{self, ObservedRow, QuestionDetail, RecordObservation};

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

impl Engine {
    /// Judge every member over every record, sort each member stably by yes
    /// probability, then visit each depth in saved member order. Duplicates
    /// consume a visit; each original input position appears only once.
    /// Originals need not implement Clone or Send. Facts count input records.
    ///
    /// # Errors
    /// Returns a preflight refusal before sends for blank or excessive input,
    /// or the whole call's error if any member, transport or control fails.
    pub fn rank_set<I>(
        &self,
        questions: &RankSet,
        records: I,
    ) -> Result<crate::SetRankCall<I::Item>, Error>
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
    pub fn rank_set_with<I>(
        &self,
        questions: &RankSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<crate::SetRankCall<I::Item>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.rank_set_complete_with(questions, records, options)?
            .project(|rows| {
                let values = rows.iter().map(|row| row.ordinal).collect();
                let probabilities = rows.iter().map(|row| row.result.probability()).collect();
                Ok((values, rows, probabilities))
            })
    }
}

pub(crate) fn member_questions(set: &core::QuestionSet) -> Vec<Question> {
    set.questions()
        .iter()
        .map(|member| Question {
            metadata: member.metadata().clone(),
            core: member.question().clone(),
            threshold: None,
            authored_threshold: false,
            model: None,
            profile: set.profile().cloned(),
            batch: None,
            kind: Kind::Rank,
        })
        .collect()
}

pub(crate) fn observed_judgments(
    set: &core::QuestionSet,
    questions: &[Question],
    backend: &core::Backend,
    stop: &Stop<'_>,
    index: usize,
    rows: Vec<Result<Decided, Miss>>,
) -> Result<Vec<crate::public::engine::Keyed>, Error> {
    let mut judgments = Vec::with_capacity(rows.len());
    let mut failed = false;
    for ((named, question), row) in set.questions().iter().zip(questions).zip(rows) {
        let decided = match row {
            Ok(decided) => decided,
            Err(Miss::Failed(decided)) => {
                failed = true;
                *decided
            }
            Err(Miss::Refused(error)) => return Err(error),
        };
        if stop.observing() {
            let observed = decided.observed(question, backend)?.qualified(
                crate::public::InputFunction::Rank,
                index,
                Some(named.name()),
                None,
                0,
            )?;
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
            judgments.push((judged, decided.keys));
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
    Ok(judgments)
}
