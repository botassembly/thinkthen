//! Decode each set member with its own receipt through the ordinary rank view.
use super::{
    Asks,
    judged::{Held, JudgeAsker},
};
use crate::core::pack;
use crate::engine::pipeline::Answered;
use crate::failure::Failure;
use crate::schedule::Judged;

pub(super) fn rows(
    asker: &JudgeAsker<'_>,
    held: Held,
    answers: &[Answered],
) -> Result<Vec<Judged>, Failure> {
    let questions = asker.judging.asks.questions(&held.record)?;
    let mut rest = answers;
    let mut rows = Vec::with_capacity(questions.len());
    for question in questions {
        let (own, after) = rest
            .split_at_checked(pack::wire_count(&question))
            .ok_or(Failure::Defect("a rank member lost its answers"))?;
        rest = after;
        let row = asker.one(&held, question, own)?;
        rows.push(row);
    }
    if asker.judging.view.details
        && let Asks::Set(set) = &asker.judging.asks
    {
        crate::schedule::rank::RankRow::bind_set(
            &mut rows,
            set,
            asker.judging.engine.backend().model(),
        )?;
    }
    Ok(rows)
}
