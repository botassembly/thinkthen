//! Annotation keeps CLI framing and presentation around the native request stream.
use super::{Judging, asker::Parser};
use crate::core::{Outcome, Reading, Record, RecordValue, Setting, json_line};
use crate::failure::Failure;
use crate::schedule::{Judged, Output, Placed};
use std::{cell::RefCell, collections::BTreeMap, io::Write, process::ExitCode, sync::Arc};

struct Held {
    record: Record,
    position: Option<crate::cli::intake::Position>,
}
#[derive(Default)]
struct Ended {
    finished: usize,
    replayed: usize,
    skipped: usize,
    partial: bool,
    closed: bool,
    failure: Option<Failure>,
}

#[expect(
    clippy::too_many_lines,
    reason = "one request bridge retains ordered skips, output failures and final facts"
)]
pub(super) fn run(
    judging: &Judging<'_>,
    admitted: crate::AdmittedRequest,
    reading: &Reading,
    inputs: impl Iterator<Item = Result<crate::cli::intake::Item, Placed>> + Send + 'static,
    setting: Option<Setting>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let parser = Parser::of(judging, reading);
    let frames = inputs.enumerate().map(move |(ordinal, frame)| {
        let frame = frame?;
        let at = frame.at;
        let position = frame.position.clone();
        let record = parser
            .record(frame)
            .map_err(|error| Placed::at(error, at))?;
        Ok((ordinal, Held { record, position }))
    });
    let reader = crate::asking::native_reader::Reader::new(frames);
    let ready = || reader.ready();
    let readiness =
        crate::public::cli_reader::CliReader::new(&ready, judging.environment.input_pause());
    reader.wake(readiness.wake());
    let held = RefCell::new(BTreeMap::new());
    let fields = reading
        .fields()
        .iter()
        .map(crate::core::Pointer::as_str)
        .collect::<Vec<_>>();
    let composition = crate::RecordReading::new(&fields, None, None).map_err(Failure::from)?;
    let records = std::iter::from_fn(|| {
        reader.next().map(|frame| {
            let (ordinal, original) =
                frame.map_err(|placed| input_error(placed.cause, placed.at))?;
            let record = compose(judging, &composition, &original.record)
                .map_err(|error| input_error(error, Some(ordinal + 1)))?;
            held.borrow_mut().insert(ordinal, original);
            Ok(record.map_original(crate::QuestionInput::Record))
        })
    });
    let ended = RefCell::new(Ended::default());
    let output = RefCell::new(output);
    let token = crate::CancelToken::new();
    let downstream = crate::edge::Downstream::default();
    let signal = || judging.cancel().fired() || token.is_cancelled() || downstream.gone();
    let mut controls = crate::CallOptions::new()
        .cli_cancel(&token, judging.cancel().deadline())
        .interrupt(&signal)
        .surface(crate::Surface::Cli)
        .attempts(judging.details())
        .cli_reader(&readiness);
    if let Some(setting) = setting {
        controls = controls.batch(match setting {
            Setting::Max => crate::BatchSetting::Max,
            Setting::Records(n) => crate::BatchSetting::Records(n),
        });
    }
    if let Some(context) = judging.context.as_deref() {
        controls = controls.context(context);
    }
    let recover = |ordinal, pointer: &str| {
        held.borrow_mut().remove(&ordinal);
        let mut ended = ended.borrow_mut();
        match super::error_row::missed(ordinal + 1, pointer)
            .and_then(|row| output.borrow_mut().take(row))
        {
            Ok(true) => {
                ended.finished += 1;
                ended.skipped += 1;
                true
            }
            Ok(false) => {
                ended.closed = true;
                token.cancel();
                true
            }
            Err(error) => {
                ended.failure = Some(error);
                token.cancel();
                false
            }
        }
    };
    let sink = |value| {
        let mut ended = ended.borrow_mut();
        if ended.closed || ended.failure.is_some() {
            return;
        }
        match receive(judging, value, &held, &mut output.borrow_mut(), &mut ended) {
            Ok(()) => {}
            Err(error) => {
                ended.failure = Some(error);
                token.cancel();
            }
        }
        if ended.closed {
            token.cancel();
        }
    };
    let release = |ordinal| {
        held.borrow_mut().remove(&ordinal);
    };
    let request = admitted
        .retain_cli_definition(crate::QuestionSet(judging.set.clone()).into())
        .map_err(Failure::from)?
        .with_composed_feed("cli-annotate");
    let engine = crate::Engine::from_cli(
        judging.engine.clone(),
        judging.environment.config().prices(),
    );
    let result = engine.execute_cli_request(
        &request,
        crate::RequestEnvironment {
            controls,
            feed: Some(crate::RequestFeed::from_records("cli-annotate", records)),
        },
        &sink,
        &release,
        judging.continue_missing().then_some(&recover),
    );
    let mut ended = ended.into_inner();
    let failure = match result {
        Ok(crate::RequestOutcome::Complete(call)) => {
            judging.environment.settle_native(call.facts());
            None
        }
        Ok(crate::RequestOutcome::Failed { error, .. }) | Err(error) => {
            let at = error.stopped().at();
            if let Some(facts) = error.facts() {
                judging.environment.settle_native(facts);
            }
            Some((at, host_error(error, reading.streams())))
        }
    };
    if ended.skipped > 0 {
        writeln!(
            std::io::stderr().lock(),
            "thinkthen: {} record{} skipped",
            ended.skipped,
            if ended.skipped == 1 { "" } else { "s" }
        )
        .map_err(Failure::Output)?;
    }
    if ended.closed || downstream.gone() {
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
            recording: judging.engine.recording(),
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

pub(super) fn compose(
    judging: &Judging<'_>,
    composition: &crate::RecordReading,
    original: &Record,
) -> Result<crate::RecordInput<crate::RecordEvidence>, Failure> {
    let mut record = composition
        .compose(crate::RawRecord(Arc::new(original.clone())))
        .map_err(Failure::from)?;
    let schema = judging
        .set
        .questions()
        .first()
        .and_then(|member| member.metadata().context_schema.as_ref());
    record.context =
        crate::asking::context::record(original, judging.context_field.as_deref(), schema)?;
    Ok(record)
}

fn input_error(cause: Failure, at: Option<usize>) -> crate::Error {
    let error = crate::Error::usage("the CLI reader failed").with_diagnostic(
        crate::public::error::diagnostic::Diagnostic::CliInput(Box::new(cause)),
    );
    match at {
        Some(at) => error.at_record(at.saturating_sub(1)),
        None => error,
    }
}

fn host_error(mut error: crate::Error, streams: bool) -> Failure {
    match error.take_diagnostic() {
        Some(crate::public::error::diagnostic::Diagnostic::PartialReply {
            cause: Some(cause),
            ..
        }) if !streams => Failure::Reply(cause),
        Some(crate::public::error::diagnostic::Diagnostic::EngineRange { cause, first, last }) => {
            let cause =
                Failure::from(cause).with_replay_context(crate::failure::ReplayContext::Annotate);
            if matches!(
                cause,
                Failure::Transport(_)
                    | Failure::Status(_)
                    | Failure::TokenLimit
                    | Failure::Reply(_)
            ) && (streams || last > first)
            {
                Failure::BatchFailed {
                    last: last.saturating_add(1),
                    cause: Box::new(cause),
                }
            } else {
                cause
            }
        }
        Some(diagnostic) => Failure::from(error.with_diagnostic(diagnostic)),
        None => Failure::from(error),
    }
}

fn receive(
    judging: &Judging<'_>,
    value: crate::RequestValue,
    held: &RefCell<BTreeMap<usize, Held>>,
    output: &mut Output<'_>,
    ended: &mut Ended,
) -> Result<(), Failure> {
    let crate::RequestValue::Annotations(rows) = value else {
        return Err(Failure::Defect("annotate returned another function"));
    };
    for row in rows {
        let original = held
            .borrow_mut()
            .remove(&row.ordinal())
            .ok_or(Failure::Defect("native annotate lost its original"))?;
        let canonical = &row.result().canonical;
        let partial = canonical
            .members
            .iter()
            .any(|(_, member)| matches!(member.identity, crate::core::MemberIdentity::Failed(_)));
        let replayed = canonical
            .identity
            .question_sources()
            .iter()
            .all(|source| source.origin() == crate::core::Origin::Replay);
        let values = canonical.legacy.value().clone().into_values();
        let mut printed = Some(if judging.details() {
            json_line(canonical)?
        } else if judging.streams() && !original.record.is_object() {
            json_line(&RecordValue::new(
                original.record.clone(),
                crate::core::NamedValues::new(values),
            ))?
        } else {
            json_line(&original.record.clone().annotated(values))?
        });
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
        let judged = Judged {
            model: None,
            printed,
            position: None,
            outcome: Outcome::Yes,
            replayed,
            order_value: None,
            partial_failure: partial,
            profile_mismatch: judging.mismatch.notice(),
        };
        if !output.take(judged)? {
            ended.closed = true;
            break;
        }
        ended.finished += 1;
        ended.replayed += usize::from(replayed);
        ended.partial |= partial;
    }
    Ok(())
}
