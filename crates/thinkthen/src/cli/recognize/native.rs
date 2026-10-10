//! Recognition framing and display around the native ordered stage executor.
use super::{Running, source};
use crate::cli::intake::{Data, Item};
use crate::core::{Outcome, Reading, RecognizeSpec, Record, RecordValue, json_line};
use crate::failure::{Failure, ReplayContext};
use crate::schedule::{Judged, Output, Placed};
use std::{cell::RefCell, collections::BTreeMap, process::ExitCode, sync::Arc};

struct Held {
    record: Record,
    position: Option<crate::cli::intake::Position>,
    located: Option<String>,
}
#[derive(Default)]
struct Ended {
    finished: usize,
    replayed: usize,
    closed: bool,
    failure: Option<Failure>,
}

#[expect(
    clippy::too_many_lines,
    reason = "one request retains original rows, ordered output and final failures"
)]
pub(super) fn run(
    running: &Running<'_>,
    admitted: crate::AdmittedRequest,
    reading: &Reading,
    spec: &RecognizeSpec,
    inputs: impl Iterator<Item = Result<Item, Placed>> + Send + 'static,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let eager = running.context_field.is_some()
        || running.examples_field.is_some()
        || running.seed_spans_field.is_some();
    let composition = composition(
        reading,
        running.examples_field.as_deref(),
        running.seed_spans_field.as_deref(),
    )?;
    let held = RefCell::new(BTreeMap::new());
    let compose = |ordinal, frame: Item| {
        let record = match &frame.data {
            Data::Bytes(bytes) => reading
                .record(bytes)
                .map_err(|error| Failure::record(error, reading.streams()))?,
            Data::Record(record) => record.clone(),
            Data::Images(_) => {
                return Err(Failure::Usage(
                    "recognize accepts text only; images are unsupported",
                ));
            }
        };
        let position = frame
            .position
            .filter(|p| p.located || running.common.input.len() > 1)
            .map(|mut position| {
                position.located = true;
                position
            });
        let located = position
            .as_ref()
            .map(|_| {
                let Data::Bytes(bytes) = &frame.data else {
                    return Err(Failure::Usage("located recognize needs text units"));
                };
                Ok(reading.as_it_arrived(bytes)?.to_owned())
            })
            .transpose()?;
        let mut native = composition
            .compose(crate::RawRecord(Arc::new(record.clone())))
            .map_err(Failure::from)?;
        native.context = crate::asking::context::record(
            &record,
            running.context_field.as_deref(),
            spec.metadata.context_schema.as_ref(),
        )?;
        held.borrow_mut().insert(
            ordinal,
            Held {
                record,
                position,
                located,
            },
        );
        Ok(native.map_original(crate::QuestionInput::Record))
    };
    // Auxiliary selectors retain whole-input admission. Ordinary feeds remain lazy.
    let (reader, finite) = if eager {
        let records = inputs
            .enumerate()
            .map(|(ordinal, frame)| {
                let frame = frame.map_err(|placed| placed.cause)?;
                compose(ordinal, frame).map_err(|error| super::examples::at_record(error, ordinal))
            })
            .collect::<Result<Vec<_>, Failure>>()?;
        (None, Some(records))
    } else {
        (
            Some(crate::asking::native_reader::Reader::new(
                inputs
                    .enumerate()
                    .map(|(ordinal, frame)| frame.map(|frame| (ordinal, frame))),
            )),
            None,
        )
    };
    let ready = || {
        reader
            .as_ref()
            .is_none_or(crate::asking::native_reader::Reader::ready)
    };
    let readiness =
        crate::public::cli_reader::CliReader::new(&ready, running.environment.input_pause());
    if let Some(reader) = &reader {
        reader.wake(readiness.wake());
    }
    let records: Box<
        dyn Iterator<Item = Result<crate::RecordInput<crate::QuestionInput>, crate::Error>> + '_,
    > = match finite {
        Some(records) => Box::new(records.into_iter().map(Ok)),
        None => Box::new(std::iter::from_fn(|| {
            reader
                .as_ref()
                .and_then(|reader| reader.next(running.environment.cancel()))
                .map(|frame| {
                    let (ordinal, frame) =
                        frame.map_err(|placed| input_error(placed.cause, placed.at))?;
                    compose(ordinal, frame).map_err(|error| input_error(error, Some(ordinal + 1)))
                })
        })),
    };
    let ended = RefCell::new(Ended::default());
    let output = RefCell::new(output);
    let token = crate::CancelToken::new();
    let downstream = crate::edge::Downstream::default();
    let signal = || running.cancel.fired() || token.is_cancelled() || downstream.gone();
    let mut controls = crate::CallOptions::new()
        .cli_cancel(&token, running.cancel.deadline())
        .interrupt(&signal)
        .surface(crate::Surface::Cli)
        .attempts(running.common.details)
        .cli_reader(&readiness)
        .cli_text_limit(running.max_text_bytes);
    if let Some(context) = running.context.as_deref() {
        controls = controls.context(context);
    }
    let sink = |value| {
        let mut ended = ended.borrow_mut();
        if ended.closed || ended.failure.is_some() {
            return;
        }
        let result = receive(
            running,
            reading,
            value,
            &held,
            &mut output.borrow_mut(),
            &mut ended,
        );
        if let Err(error) = result {
            ended.failure = Some(error);
        }
        if ended.closed || ended.failure.is_some() {
            token.cancel();
        }
    };
    let release = |ordinal| {
        held.borrow_mut().remove(&ordinal);
    };
    let request = admitted.with_composed_feed("cli-recognize");
    let engine = crate::Engine::from_cli(
        running.engine.clone(),
        running.environment.config().prices(),
    );
    let feed = crate::RequestFeed::from_records("cli-recognize", records);
    let feed = if eager { feed.eager() } else { feed };
    let result = engine.execute_cli_request(
        &request,
        crate::RequestEnvironment {
            controls,
            feed: Some(feed),
        },
        &sink,
        &release,
        None,
    );
    let mut ended = ended.into_inner();
    let failure = match result {
        Ok(crate::RequestOutcome::Complete(call)) => {
            running.environment.settle_native(call.facts());
            None
        }
        Ok(crate::RequestOutcome::Failed { error, .. }) | Err(error) => {
            let at = error.stopped().at();
            if let Some(facts) = error.facts() {
                running.environment.settle_native(facts);
            }
            let cause = Failure::from(error).with_replay_context(ReplayContext::Recognize);
            let cause = match at {
                Some(at) => super::examples::at_record(cause, at.saturating_sub(1)),
                None => cause,
            };
            Some((at, cause))
        }
    };
    if ended.closed || downstream.gone() {
        return Ok(ExitCode::SUCCESS);
    }
    if let Some((at, cause)) = ended.failure.take().map(|cause| (None, cause)).or(failure) {
        if !reading.streams() && !running.common.located() && running.common.input.len() <= 1 {
            return Err(cause);
        }
        return Err(Failure::Stopped {
            at: at.unwrap_or(ended.finished + 1),
            finished: ended.finished,
            replayed: ended.replayed,
            recording: running.engine.recording(),
            held: false,
            cause: Box::new(cause),
        });
    }
    Ok(ExitCode::SUCCESS)
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
fn receive(
    running: &Running<'_>,
    reading: &Reading,
    value: crate::RequestValue,
    held: &RefCell<BTreeMap<usize, Held>>,
    output: &mut Output<'_>,
    ended: &mut Ended,
) -> Result<(), Failure> {
    let crate::RequestValue::Recognized(rows) = value else {
        return Err(Failure::Defect("recognize returned a different function"));
    };
    for row in rows {
        let frame = held
            .borrow_mut()
            .remove(&row.ordinal())
            .ok_or(Failure::Defect("recognize lost an original row"))?;
        let mut canonical = row.result.canonical;
        let replayed = canonical
            .identity
            .question_sources()
            .iter()
            .all(|source| source.origin() != crate::core::Origin::Live);
        canonical.input = reading.streams().then_some(frame.record.clone());
        let location = frame.position.as_ref().zip(frame.located.as_deref());
        let line = if running.common.details {
            match location {
                Some((position, text)) => json_line(&source::Complete {
                    value: source::located(&canonical.value, position, text)?,
                    canonical: &canonical,
                })?,
                None => json_line(&canonical)?,
            }
        } else {
            match location {
                Some((position, text)) => located_line(
                    &canonical.value,
                    &frame.record,
                    position,
                    text,
                    reading.streams(),
                )?,
                None if reading.streams() => {
                    json_line(&RecordValue::new(frame.record, canonical.value))?
                }
                None => json_line(&canonical.value)?,
            }
        };
        let mut printed = Some(line);
        if location.is_some() && (reading.streams() || running.common.details) {
            crate::cli::intake::source_members(&mut printed, frame.position.as_ref())?;
        }
        if !output.take(Judged {
            model: None,
            printed,
            position: frame.position,
            outcome: Outcome::Yes,
            replayed,
            order_value: None,
            partial_failure: false,
            profile_mismatch: running.mismatch.notice(),
        })? {
            ended.closed = true;
            break;
        }
        ended.finished += 1;
        ended.replayed += usize::from(replayed);
    }
    Ok(())
}

fn located_line(
    value: &crate::core::RecognizedValue,
    record: &Record,
    position: &crate::cli::intake::Position,
    text: &str,
    streams: bool,
) -> Result<String, Failure> {
    let value = source::located(value, position, text)?;
    if streams {
        json_line(&RecordValue::new(record.clone(), value)).map_err(Failure::from)
    } else {
        crate::cli::intake::source_value(&text, &json_line(&value)?, position)
    }
}

pub(super) fn composition(
    reading: &Reading,
    examples_field: Option<&str>,
    seed_spans_field: Option<&str>,
) -> Result<crate::RecordReading, Failure> {
    let fields = reading
        .fields()
        .iter()
        .map(crate::core::Pointer::as_str)
        .collect::<Vec<_>>();
    let mut composition = crate::RecordReading::new(&fields, None, None).map_err(Failure::from)?;
    if let Some(pointer) = examples_field {
        composition = composition
            .with_examples_field(pointer)
            .map_err(Failure::from)?;
    }
    if let Some(pointer) = seed_spans_field {
        composition = composition
            .with_seed_spans_field(pointer)
            .map_err(Failure::from)?;
    }
    Ok(composition)
}
