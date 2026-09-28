//! One eligible refusal may produce two requests; the caller decides before right.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::{self, Batch, BatchRecord, Evidence, Question};
use crate::engine::Cancel;
use crate::engine::error::{Error, Kind};
use crate::engine::prepared_request::Answered;

use super::Engine;

pub(crate) enum SplitDecision {
    Stop,
    SendRight,
}

pub(crate) struct SplitParent {
    pub(crate) digest: String,
    pub(crate) sent: u64,
    pub(crate) total: usize,
    pub(crate) closed: core::batch::Closed,
}

type SplitHalf = Box<(Batch, Result<Answered, Error>)>;

pub(crate) enum OneSplit {
    Single(Result<Answered, Error>),
    Halved {
        parent: SplitParent,
        left: SplitHalf,
        right: Option<SplitHalf>,
    },
}

fn fatal(error: &Error) -> bool {
    matches!(
        error.kind(),
        Kind::Cancelled | Kind::Deadline | Kind::Defect
    )
}

pub(super) fn ask(
    engine: &Engine,
    batch: &Batch,
    records: impl FnOnce() -> Result<Vec<(BatchRecord, Question)>, Error>,
    context: Option<&Evidence>,
    cancel: &Cancel,
    after_left: impl FnOnce(&Batch, &Result<Answered, Error>) -> SplitDecision,
) -> Result<OneSplit, Error> {
    let attempted = AtomicU64::new(0);
    let first = engine.ask_batch_with_attempts(batch, cancel, Some(&attempted));
    if !first.as_ref().err().is_some_and(Error::too_large) || batch.outcomes.len() < 2 {
        return Ok(OneSplit::Single(first));
    }
    let [left, right] =
        core::batch::halves_with_questions(engine.backend(), engine.profile(), context, records()?)
            .map_err(|_| Error::Defect("a refused batch could not be halved"))?;
    let parent = SplitParent {
        digest: batch.digest.as_str().to_owned(),
        sent: attempted.load(Ordering::Relaxed),
        total: batch.outcomes.len(),
        closed: batch.closed,
    };
    let answered_left = match engine.ask_batch(&left, cancel) {
        Err(error) if fatal(&error) => return Err(error),
        result => result,
    };
    if matches!(after_left(&left, &answered_left), SplitDecision::Stop) {
        return Ok(OneSplit::Halved {
            parent,
            left: Box::new((left, answered_left)),
            right: None,
        });
    }
    if let Some(stop) = cancel.stop() {
        return Err(stop);
    }
    let answered_right = match engine.ask_batch(&right, cancel) {
        Err(error) if fatal(&error) => return Err(error),
        result => result,
    };
    Ok(OneSplit::Halved {
        parent,
        left: Box::new((left, answered_left)),
        right: Some(Box::new((right, answered_right))),
    })
}
