//! A row's outcomes and receipt from its wire answers, by ADR 0111 section 7.
//! Every asker builds its rows here, so a row reports its usage, model,
//! keys and sends by one rule on every surface.

use crate::core::adapters::built_in::DecodeError;
use crate::core::pack;
use crate::core::recording::Digest;
use crate::core::{AnswerOutcome, ModelName, Question, Reply, Usage};
use crate::engine::error::Error;
use crate::engine::facade;

use super::Answered;

/// One question's outcomes from its wire answers, read under the model the
/// first answer names.
pub(crate) fn read(
    question: &Question,
    answers: &[Answered],
) -> Result<Vec<AnswerOutcome>, DecodeError> {
    let stored: Vec<_> = answers
        .iter()
        .map(|answered| answered.answer.as_deref().map_err(DecodeError::cause))
        .collect();
    pack::read(std::slice::from_ref(question), &stored, model(answers))
}

/// What a row reports for `outcomes`: the first answer's model, the summed
/// usage, whether the store gave every answer, the first key and the sends.
pub(crate) fn receipt(
    answers: &[Answered],
    outcomes: Vec<AnswerOutcome>,
) -> Result<facade::Answered, Error> {
    let model =
        ModelName::reported(model(answers)).map_err(|_| Error::Defect("a reply named no model"))?;
    let mut usage = RowUsage::default();
    for answered in answers {
        usage.add(answered.usage);
    }
    Ok(facade::Answered {
        reply: Reply::new(model, outcomes, usage.total()?),
        replayed: answers.iter().all(|answered| answered.cached),
        request: Digest::named(
            answers
                .first()
                .map(|answered| answered.key.hex())
                .unwrap_or_default(),
        ),
        requests_sent: answers.iter().map(|answered| answered.requests_sent).sum(),
    })
}

fn model(answers: &[Answered]) -> &str {
    answers
        .first()
        .map_or("", |answered| &*answered.answered_by)
}

/// A row's usage: the sum of its shares, live or stored. It is absent when
/// any share lacks counts or none was added. Otherwise a sum that does not
/// fit fails the row. A missing share outranks an overflow, so the order
/// the shares arrive in never changes the total.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RowUsage {
    sum: Option<Usage>,
    missing: bool,
    overflowed: bool,
}

impl RowUsage {
    /// Add one share.
    pub(crate) fn add(&mut self, share: Option<Usage>) {
        let Some(share) = share else {
            self.missing = true;
            return;
        };
        match self.sum {
            None => self.sum = Some(share),
            Some(sum) => match sum.checked_plus(share) {
                Some(next) => self.sum = Some(next),
                None => self.overflowed = true,
            },
        }
    }

    /// The row's usage by the rule above.
    pub(crate) fn total(self) -> Result<Option<Usage>, Error> {
        match self {
            Self { missing: true, .. } => Ok(None),
            Self {
                overflowed: true, ..
            } => Err(Error::UsageOverflow),
            Self { sum, .. } => Ok(sum),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One edge row: the shares, then the usage, or `Err` for the overflow.
    type Row = (&'static [Option<Usage>], Result<Option<Usage>, ()>);

    #[test]
    fn a_row_sums_its_shares_by_one_rule() {
        const MOST: Option<Usage> = Some(Usage::new(u64::MAX, 0));
        const ONE: Option<Usage> = Some(Usage::new(1, 1));
        const TWO: Option<Usage> = Some(Usage::new(2, 3));
        let rows: [Row; 7] = [
            (&[], Ok(None)),
            (&[ONE], Ok(ONE)),
            (&[ONE, TWO], Ok(Some(Usage::new(3, 4)))),
            (&[ONE, None], Ok(None)),
            (&[MOST, ONE], Err(())),
            (&[MOST, ONE, None], Ok(None)),
            (&[None, MOST, ONE], Ok(None)),
        ];
        for (shares, expected) in rows {
            let mut usage = RowUsage::default();
            for share in shares {
                usage.add(*share);
            }
            let total = usage.total().map_err(|error| {
                assert!(matches!(error, Error::UsageOverflow), "{shares:?}");
            });
            assert_eq!(total, expected, "{shares:?}");
        }
    }
}
