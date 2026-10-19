//! Public sessions own annotation execution; the command owns ordered presentation.
use super::{Judging, asker::Parser, continuation::Continuation};
use crate::cli::reading::Reading;
use crate::failure::Failure;
use crate::schedule::{Output, Placed};
use crate::{BatchSetting, RawRecord, RequestSessionRead, RequestSessionResult};
use std::{collections::BTreeMap, process::ExitCode};
mod rendering;
use rendering::{Ended, Held};

enum FeedRead {
    Pending,
    End,
    Failed(crate::Error),
}
impl From<crate::Error> for FeedRead {
    fn from(error: crate::Error) -> Self {
        Self::Failed(error)
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one ordered pump joins input, continuation acknowledgements and native settlement"
)]
pub(super) fn run(
    judging: &Judging<'_>,
    admitted: crate::AdmittedRequest,
    reading: &Reading,
    inputs: impl Iterator<Item = Result<crate::cli::intake::Item, Placed>> + Send + 'static,
    setting: Option<BatchSetting>,
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
    let fields = reading
        .fields()
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let composition = crate::RecordReading::new(&fields, None, None)?;
    let mut arguments = admitted.request().call.arguments().clone();
    arguments.question = crate::RequestQuestion::Definition {
        value: judging.set.clone().into(),
    };
    arguments.input = crate::RequestInput::Feed {
        name: "cli-annotate".to_owned(),
        framing: crate::RequestFraming::Document,
        reading: crate::ReaderOptions::default(),
        images: Vec::new(),
    };
    arguments.options.field = None;
    arguments.options.context_field = None;
    arguments.options.context = judging.context.clone();
    arguments.options.attempts = judging.details();
    arguments.options.batch = setting.map(|setting| match setting {
        BatchSetting::Max => crate::RequestBatch::Named("max".to_owned()),
        BatchSetting::Records(count) => crate::RequestBatch::Count(count.get()),
    });
    let (continuation, callback) = Continuation::new(judging.cancel().clone());
    let tally = judging.environment.tally().start();
    let session = judging.engine.request_session_with_feed_options(
        crate::Request::for_function(crate::RequestFunction::Annotate, arguments),
        crate::Surface::Cli,
        crate::RequestSessionFeedOptions {
            on_missing_annotation_pointer: judging.continue_missing().then_some(callback),
            input_pause: judging.environment.input_pause(),
            retain_success_on_cancel: true,
            record_reading: Some(composition),
            ..Default::default()
        },
    )?;
    let mut tally = Some(tally);
    let mut held = BTreeMap::new();
    let mut ended = Ended::default();
    let downstream = crate::edge::Downstream::default();
    let mut input_done = false;
    let mut cancelled = false;
    let mut terminal = None;
    loop {
        if !cancelled
            && (judging.cancel().fired()
                || downstream.gone()
                || ended.closed
                || ended.failure.is_some())
        {
            continuation.close();
            session.cancel();
            cancelled = true;
            input_done = true;
        }
        // Drain preceding native rows before acknowledging a missing-pointer row.
        // The callback blocks later rows until the command has written this row.
        let packet = session.try_read();
        let received = matches!(&packet, crate::RequestSessionRead::Result(_));
        match packet {
            crate::RequestSessionRead::Result(RequestSessionResult::Row(crate::RequestSessionRow::Annotation(row))) => {
                        rendering::receive(judging, row, &mut held, output, &mut ended);
                    }
            crate::RequestSessionRead::Result(RequestSessionResult::Row(_)) => {
                        ended
                            .failure
                            .get_or_insert(Failure::Defect("annotate returned another function"));
                    }
            crate::RequestSessionRead::Result(RequestSessionResult::Aggregate(crate::RequestValue::Annotations(rows))) => {
                        for row in rows {
                            rendering::receive(judging, row, &mut held, output, &mut ended);
                        }
                    }
            crate::RequestSessionRead::Result(RequestSessionResult::Aggregate(_)) => {
                        ended
                            .failure
                            .get_or_insert(Failure::Defect("annotate returned another function"));
                    }
            crate::RequestSessionRead::Result(RequestSessionResult::Observation { .. }) => {}
            crate::RequestSessionRead::Result(RequestSessionResult::Terminal(value)) => {
                        judging.environment.settle_session(&value, &mut tally)?;
                        terminal = Some(value.error.map_or(Ok(()), Err));
                    }

            crate::RequestSessionRead::End => break,
            crate::RequestSessionRead::Pending => {}
        }
        if received {
            continue;
        }
        if let Some(missing) = continuation.try_next() {
            held.remove(&missing.ordinal);
            acknowledge_missing(missing, cancelled, output, &mut ended);
            continue;
        }
        let mut pending_reader = false;
        let next_descriptor = || match reader.try_next() {
                crate::asking::native_reader::Read::Pending => Err(FeedRead::Pending),
                crate::asking::native_reader::Read::End => Err(FeedRead::End),
                crate::asking::native_reader::Read::Item(Err(placed)) => {
                    ended.input_at = placed.at;
                    Err(FeedRead::Failed(placed.cause.into_error()))
                }
                crate::asking::native_reader::Read::Item(Ok((ordinal, original))) => {
                    let descriptor = descriptor(judging, &original).map_err(|error| {
                        ended.input_at = Some(ordinal + 1);
                        FeedRead::Failed(error)
                    })?;
                    held.insert(ordinal, original);
                    Ok(descriptor)
                }
            };
        if !input_done {
            match session.try_push_with(next_descriptor) {
                Ok(crate::RequestSessionPushStatus::Accepted) => continue,
                Ok(crate::RequestSessionPushStatus::Full) => {}
                Ok(crate::RequestSessionPushStatus::Closed) => input_done = true,
                Err(FeedRead::Pending) => pending_reader = true,
                Err(FeedRead::End) => {
                    session.finish(None)?;
                    input_done = true;
                }
                Err(FeedRead::Failed(error)) => {
                    session.finish_native_reader_error(error)?;
                    input_done = true;
                }
            }
        }
        // The private callback channel cannot wake the native result queue;
        // keep polling it and cancellation even while native input is finished.
        if pending_reader {
            reader.wait(std::time::Duration::from_millis(50));
            continue;
        }
        let interest = if cancelled {
            crate::RequestSessionWaitInterest::Settlement
        } else if input_done {
            crate::RequestSessionWaitInterest::Result
        } else {
            crate::RequestSessionWaitInterest::Input
        };
        if session.wait(interest, std::time::Duration::from_millis(50))
            == crate::RequestSessionReady::End
        {
            break;
        }
    }
    continuation.close();
    rendering::finish(
        judging,
        reading,
        ended,
        terminal.ok_or(Failure::Defect("native session ended without settlement"))?,
        downstream.latched(),
    )
}

fn descriptor(
    judging: &Judging<'_>,
    original: &Held,
) -> Result<crate::RequestSessionDescriptor, crate::Error> {
    let context = crate::asking::context::record(
        &original.record,
        judging.context_field.as_deref(),
        judging.settings.context_schema.as_ref(),
    )
    .map_err(Failure::into_error)?;
    let location = original
        .position
        .as_ref()
        .filter(|position| position.located)
        .and_then(|position| position.file.as_ref().map(|file| (file, position)))
        .map(|(file, position)| {
            crate::SourceLocation::new(file.clone(), position.first, position.last)
        })
        .transpose()?;
    let original = Some(match original.record.literal() {
        Some(text) => crate::RequestOriginal::Text {
            text: text.to_owned(),
        },
        None => crate::RequestOriginal::Json {
            value: original.record.clone(),
        },
    });
    Ok(crate::RequestSessionDescriptor {
        item: crate::RequestItem {
            original,
            images: Vec::new(),
            context,
            options: None,
            seed_spans: None,
            examples: None,
        },
        location,
    })
}

pub(super) fn compose(
    judging: &Judging<'_>,
    composition: &crate::RecordReading,
    original: &RawRecord,
) -> Result<crate::RecordInput<crate::RecordEvidence>, Failure> {
    let mut record = composition.compose(original.clone())?;
    record.context = crate::asking::context::record(
        original,
        judging.context_field.as_deref(),
        judging.settings.context_schema.as_ref(),
    )?;
    Ok(record)
}

fn acknowledge_missing(
    missing: super::continuation::Missing,
    cancelled: bool,
    output: &mut crate::schedule::Output<'_>,
    ended: &mut rendering::Ended,
) {
if cancelled || ended.closed || ended.failure.is_some() {
    missing.acknowledge(false);
} else {
    match super::error_row::missed(missing.ordinal + 1, &missing.pointer)
        .and_then(|row| output.take(row))
    {
        Ok(true) => {
            ended.finished += 1;
            ended.skipped += 1;
            missing.acknowledge(true);
        }
        // Preserve the old callback's accepted closed-pipe row. Its
        // native accounting still settles before command success.
        Ok(false) => {
            ended.closed = true;
            missing.acknowledge(true);
        }
        Err(error) => {
            ended.failure = Some(error);
            missing.acknowledge(false);
        }
    }
}
}
