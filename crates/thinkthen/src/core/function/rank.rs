//! Stable ordering by a yes probability or a scale score.

use crate::core::{Answer, Ask, Decide, Error, ErrorKind, Function, Outcome, Row, Score};

/// The question whose answers order a collection of records.
#[derive(Debug, Clone, serde::Serialize)]
pub enum Rank {
    /// Order by the probability of yes, independent of the threshold.
    Decide(Decide),
    /// Order by the probability-weighted scale score.
    Score(Score),
}

impl Rank {
    pub(crate) fn definition(&self) -> super::identity::Definition<'_> {
        super::identity::Definition::Rank {
            question: Box::new(match self {
                Self::Decide(question) => ("decide", question.definition()),
                Self::Score(question) => ("score", question.definition()),
            }),
        }
    }

    /// Replace a score question's levels.
    ///
    /// # Errors
    /// Returns `usage.question` for a decide question or invalid levels.
    pub fn levels(self, levels: Vec<String>) -> Result<Self, Error> {
        match self {
            Self::Score(question) => Ok(Self::Score(question.levels(levels)?)),
            Self::Decide(_) => Err(Error::new(
                ErrorKind::Question,
                "levels require a score question",
            )),
        }
    }
}

/// A positive bound on the number of retained ranked records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RankLimit(std::num::NonZeroUsize);

impl RankLimit {
    /// Admit the caller's top limit.
    ///
    /// # Errors
    /// Returns `usage.question` when the limit is zero.
    pub fn new(value: usize) -> Result<Self, Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self)
            .ok_or_else(Self::invalid)
    }

    /// The number of records to retain.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }

    fn invalid() -> Error {
        Error::new(
            ErrorKind::Question,
            "the top limit must be a whole number of at least 1",
        )
    }
}

impl std::str::FromStr for RankLimit {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Error> {
        text.parse::<usize>()
            .map_err(|_| Self::invalid())
            .and_then(Self::new)
    }
}

type OrderKey = (std::cmp::Reverse<Option<u64>>, usize);

/// Evaluated records retained for a stable ranking.
#[derive(Debug)]
pub struct Ranking<T> {
    rows: std::collections::BTreeMap<OrderKey, (T, Row)>,
    top: Option<RankLimit>,
    next: usize,
}

impl<T> Ranking<T> {
    /// Retain all records, or only the best `top` records.
    #[must_use]
    pub fn new(top: Option<RankLimit>) -> Self {
        Self {
            rows: std::collections::BTreeMap::new(),
            top,
            next: 0,
        }
    }

    /// Consider one evaluated record in input order.
    pub fn push(&mut self, original: T, row: Row) {
        // Validated metrics are nonnegative finite values. Their IEEE bits
        // have numeric order once the sign bit on negative zero is cleared.
        let metric = metric(&row).map(|value| value.to_bits() & !(1_u64 << 63));
        let key = (std::cmp::Reverse(metric), self.next);
        self.next += 1;
        self.rows.insert(key, (original, row));
        if self.top.is_some_and(|top| self.rows.len() > top.get()) {
            self.rows.pop_last();
        }
    }

    /// Return retained records best first, with positions starting at one.
    #[must_use]
    pub fn finish(self) -> Vec<(T, Row)> {
        self.rows
            .into_values()
            .enumerate()
            .map(|(index, (original, mut row))| {
                row.position(index + 1);
                (original, row)
            })
            .collect()
    }
}

impl Function for Rank {
    const NAME: &'static str = "rank";
    fn identity_as(&self, function: &str) -> String {
        super::identity(function, &self.definition())
    }
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        match self {
            Self::Decide(q) => q.try_asks_record(record, limits),
            Self::Score(q) => q.try_asks_record(record, limits),
        }
    }

    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Outcome {
        let question = context.question();
        match question {
            Self::Decide(q) => Function::read_record(context.for_function(q), limits),
            Self::Score(q) => Function::read_record(context.for_function(q), limits),
        }
    }
}

fn metric(row: &Row) -> Option<f64> {
    match row.outcome() {
        Outcome::Answered { p: Some(p), .. } | Outcome::Unsure { p, .. } => Some(p.get()),
        Outcome::Answered {
            answer: Answer::Number(score),
            ..
        } => Some(*score),
        _ => None,
    }
}
