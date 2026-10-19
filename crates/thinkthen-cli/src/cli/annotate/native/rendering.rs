//! Ordered command rendering and final native annotation diagnostics.
use super::super::Judging;
use crate::cli::reading::Reading;
use crate::failure::Failure;
use crate::schedule::{Judged, Outcome, Output};
use std::{collections::BTreeMap, io::Write, process::ExitCode};

pub(super) struct Held {
    pub(super) record: crate::RawRecord,
    pub(super) position: Option<crate::cli::intake::Position>,
}
#[derive(Default)]
pub(super) struct Ended {
    pub(super) input_at: Option<usize>,
    pub(super) finished: usize,
    replayed: usize,
    pub(super) skipped: usize,
    partial: bool,
    pub(super) closed: bool,
    pub(super) failure: Option<Failure>,
}

pub(super) fn receive(
    judging: &Judging<'_>,
    row: crate::CompleteRecord<crate::QuestionInput, crate::CompleteAnnotated>,
    held: &mut BTreeMap<usize, Held>,
    output: &mut Output<'_>,
    ended: &mut Ended,
) {
    if ended.closed || ended.failure.is_some() {
        return;
    }
    if let Err(error) = take(judging, row, held, output, ended) {
        ended.failure = Some(error);
    }
}
fn take(
    judging: &Judging<'_>,
    row: crate::CompleteRecord<crate::QuestionInput, crate::CompleteAnnotated>,
    held: &mut BTreeMap<usize, Held>,
    output: &mut Output<'_>,
    ended: &mut Ended,
) -> Result<(), Failure> {
    let original = held
        .remove(&row.ordinal())
        .ok_or(Failure::Defect("native annotate lost its original"))?;
    let result = row.result();
    let partial = result.members().any(|member| member.failure_id().is_some());
    let replayed = result
        .identity()
        .question_sources()
        .iter()
        .all(|source| source.origin() != crate::Origin::Live);
    let mut printed = Some(super::super::presentation::line(
        result,
        &original.record,
        judging.streams(),
        judging.details(),
    )?);
    if let Some(position) = original.position.as_ref().filter(|p| p.located)
        && !judging.details()
        && let Some(line) = &mut printed
    {
        *line = crate::cli::intake::source_value(&original.record, line, position)?;
    }
    if judging.details() {
        crate::cli::intake::source_members(&mut printed, original.position.as_ref())?;
        crate::cli::intake::locate(&mut printed, original.position.as_ref())?;
    }
    if !output.take(Judged {
        model: None,
        printed,
        position: None,
        outcome: Outcome::Yes,
        replayed,
        order_value: None,
        partial_failure: partial,
        profile_mismatch: judging.mismatch.notice(),
    })? {
        ended.closed = true;
        return Ok(());
    }
    ended.finished += 1;
    ended.replayed += usize::from(replayed);
    ended.partial |= partial;
    Ok(())
}

pub(super) fn finish(
    judging: &Judging<'_>,
    reading: &Reading,
    mut ended: Ended,
    result: Result<(), crate::Error>,
    downstream_closed: bool,
) -> Result<ExitCode, Failure> {
    let failure = result.err().map(|error| {
        let at = error.stopped().at().or(ended.input_at);
        (at, host_error(error, reading.streams()))
    });
    if ended.skipped > 0 {
        writeln!(
            std::io::stderr().lock(),
            "thinkthen: {} record{} skipped",
            ended.skipped,
            if ended.skipped == 1 { "" } else { "s" }
        )
        .map_err(Failure::Output)?;
    }
    if ended.closed || downstream_closed {
        return Ok(ExitCode::SUCCESS);
    }
    if let Some((at, cause)) = ended.failure.take().map(|cause| (None, cause)).or(failure) {
        if !reading.streams()
            && judging.common.input.len() <= 1
            && !judging.common.located()
            && ended.finished > 0
        {
            return Ok(ExitCode::SUCCESS);
        }
        if !reading.streams() {
            return Err(cause);
        }
        return Err(Failure::Stopped {
            at: at.unwrap_or(ended.finished + 1),
            finished: ended.finished,
            replayed: ended.replayed,
            recording: judging.recording,
            held: false,
            cause: Box::new(cause),
        });
    }
    Ok(ExitCode::from(if ended.skipped > 0 {
        7
    } else if ended.partial {
        6
    } else {
        0
    }))
}
fn host_error(error: crate::Error, streams: bool) -> Failure {
    if !streams
        && let Some(crate::ErrorDiagnostic::PartialReply {
            cause: Some(cause), ..
        }) = error.diagnostic()
    {
        return Failure::Reply(cause);
    }
    let single_batch_end = match error.diagnostic() {
        Some(crate::ErrorDiagnostic::Range { first, last, .. }) if streams && first == last => {
            Some(last)
        }
        _ => None,
    };
    let cause = Failure::from(error).with_replay_context(crate::failure::ReplayContext::Annotate);
    let failure = match single_batch_end {
        Some(last)
            if matches!(
                cause,
                Failure::Transport(_)
                    | Failure::Status(..)
                    | Failure::Provider { .. }
                    | Failure::TokenLimit
                    | Failure::Reply(_)
            ) =>
        {
            Failure::BatchFailed {
                last: last.saturating_add(1),
                cause: Box::new(cause),
            }
        }
        _ => cause,
    };
    failure.with_size_advice(if streams {
        crate::failure::SizeAdvice::Batches
    } else {
        crate::failure::SizeAdvice::Text
    })
}
