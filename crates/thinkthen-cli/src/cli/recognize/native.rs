//! Recognition framing and display around the public ordered request session.
use super::{Running, source};
use crate::cli::intake::{Data, Item};
use crate::cli::reading::Reading;
use crate::failure::{Failure, ReplayContext};
use crate::render::json_line;
use crate::schedule::Outcome;
use crate::schedule::{Judged, Output, Placed};
use std::{cell::RefCell, collections::BTreeMap, process::ExitCode};

struct Held {
    record: crate::RawRecord,
    position: Option<crate::cli::intake::Position>,
    located: Option<String>,
}
#[derive(serde::Serialize)]
struct RecordValue<'a, T> {
    input: &'a crate::RawRecord,
    value: T,
}
#[derive(Default)]
struct Ended {
    finished: usize,
    replayed: usize,
    closed: bool,
    failure: Option<Failure>,
    input_at: Option<usize>,
}

pub(super) fn request(admitted: crate::AdmittedRequest, context: Option<&str>) -> crate::Request {
    let mut arguments = admitted.request().call.arguments().clone();
    arguments.input = crate::RequestInput::Feed {
        name: "cli-recognize".to_owned(),
        framing: crate::RequestFraming::Document,
        reading: crate::ReaderOptions::default(),
        images: Vec::new(),
    };
    arguments.options.field = None;
    arguments.options.context_field = None;
    arguments.options.examples_field = None;
    arguments.options.seed_spans_field = None;
    arguments.options.context = context.map(str::to_owned);
    crate::Request::for_function(crate::RequestFunction::Recognize, arguments)
}

#[expect(
    clippy::too_many_lines,
    reason = "one request retains original rows, ordered output and joined settlement"
)]
pub(super) fn run(
    running: &Running<'_>,
    admitted: crate::AdmittedRequest,
    reading: &Reading,
    question: &crate::Recognize,
    inputs: impl Iterator<Item = Result<Item, Placed>> + Send + 'static,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let eager = running.context_field.is_some()
        || running.examples_field.is_some()
        || running.seed_spans_field.is_some();
    let projection = composition(
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
        let mut native = projection.compose(record.clone())?;
        native.context = crate::asking::context::record(
            &record,
            running.context_field.as_deref(),
            question.reading().context_schema(),
        )?;
        let original = match record.literal() {
            Some(text) => crate::RequestOriginal::Text {
                text: text.to_owned(),
            },
            None => crate::RequestOriginal::Json {
                value: record.clone(),
            },
        };
        let descriptor = crate::RequestSessionDescriptor {
            item: crate::RequestItem {
                original: Some(original),
                images: Vec::new(),
                context: native.context,
                options: None,
                examples: native.examples,
                seed_spans: native.seed_spans,
            },
            location: None,
        };
        held.borrow_mut().insert(
            ordinal,
            Held {
                record,
                position,
                located,
            },
        );
        Ok(descriptor)
    };
    // Auxiliary selectors retain whole-input admission. Ordinary feeds remain lazy.
    let (reader, mut finite) = if eager {
        let descriptors = inputs
            .enumerate()
            .map(|(ordinal, frame)| {
                let frame = frame.map_err(|placed| placed.cause)?;
                compose(ordinal, frame).map_err(|error| super::examples::at_record(error, ordinal))
            })
            .collect::<Result<Vec<_>, Failure>>()?;
        (None, Some(descriptors.into_iter()))
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
    let tally = running.environment.tally().start();
    let session = running.engine.request_session_with_feed_options(
        request(admitted, running.context.as_deref()),
        crate::Surface::Cli,
        crate::RequestSessionFeedOptions {
            interrupt: Some(running.environment.cancel().interrupt()),
            retain_success_on_cancel: true,
            incremental_recognition: true,
            eager,
            input_pause: running.environment.input_pause(),
            record_reading: Some(composition(reading, None, None)?),
            ..crate::RequestSessionFeedOptions::default()
        },
    )?;
    let downstream = crate::edge::Downstream::default();
    let mut ended = Ended::default();
    let mut input_done = false;
    let mut cancelled = false;
    let mut terminal = None;
    let mut tally = Some(tally);
    loop {
        if !cancelled
            && (running.environment.cancel().fired()
                || downstream.gone()
                || ended.closed
                || ended.failure.is_some())
        {
            session.cancel();
            cancelled = true;
            input_done = true;
        }
        let packet = session.try_read();
        let received = matches!(&packet, crate::RequestSessionRead::Result(_));
        match packet {
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Aggregate(value))
                if !ended.closed && ended.failure.is_none() =>
            {
                if let Err(error) = receive(running, reading, value, &held, output, &mut ended) {
                    ended.failure = Some(error);
                }
            }
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Terminal(value)) => {
                running.environment.settle_session(&value, &mut tally)?;
                terminal = Some(value.error.map_or(Ok(()), Err));
            }
            crate::RequestSessionRead::Result(_) => {}

            crate::RequestSessionRead::End => break,
            crate::RequestSessionRead::Pending => {}
        }
        if received {
            continue;
        }
        let mut pending_reader = false;
        let next_descriptor = || {
            if let Some(finite) = &mut finite {
                return finite.next().ok_or(FeedRead::End);
            }
            match reader
                .as_ref()
                .map(crate::asking::native_reader::Reader::try_next)
            {
                Some(crate::asking::native_reader::Read::Pending) => Err(FeedRead::Pending),
                Some(crate::asking::native_reader::Read::Item(Err(placed))) => {
                    ended.input_at = placed.at;
                    Err(FeedRead::Failed(placed.cause.into_error()))
                }
                Some(crate::asking::native_reader::Read::Item(Ok((ordinal, frame)))) => {
                    compose(ordinal, frame).map_err(|error| placed_failure(error, ordinal))
                }
                _ => Err(FeedRead::End),
            }
        };
        if !input_done {
            let pushed = session.try_push_with(next_descriptor);
            match pushed {
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
        if pending_reader {
            if let Some(reader) = &reader {
                reader.wait(std::time::Duration::from_millis(50));
            }
            continue;
        }
        if crate::asking::native_reader::session_ended(&session, cancelled, input_done) {
            break;
        }
    }
    let result = terminal.ok_or(Failure::Defect("native session ended without settlement"))?;
    let failure = result.err().map(|error| {
        let at = error.stopped().at().or(ended.input_at);
        let cause = Failure::from(error)
            .with_replay_context(ReplayContext::Recognize)
            .with_size_advice(crate::failure::SizeAdvice::Batches);
        let cause = match at {
            Some(at) => super::examples::at_record(cause, at.saturating_sub(1)),
            None => cause,
        };
        (at, cause)
    });
    if ended.closed || downstream.latched() {
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
            recording: running.recording,
            held: false,
            cause: Box::new(cause),
        });
    }
    Ok(ExitCode::SUCCESS)
}

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
        let canonical = row.result();
        let replayed = canonical
            .identity()
            .question_sources()
            .iter()
            .all(|source| source.origin() != crate::Origin::Live);
        let location = frame.position.as_ref().zip(frame.located.as_deref());
        let line = if running.common.details {
            json_line(&source::Complete {
                value: location
                    .map(|(position, text)| source::located(canonical.value(), position, text))
                    .transpose()?,
                canonical,
                input: reading.streams().then_some(&frame.record),
            })?
        } else {
            match location {
                Some((position, text)) => located_line(
                    canonical.value(),
                    &frame.record,
                    position,
                    text,
                    reading.streams(),
                )?,
                None if reading.streams() => json_line(&RecordValue {
                    input: &frame.record,
                    value: crate::saved_json::Json::parse(&canonical.value().to_json())?,
                })?,
                None => canonical.value().to_json(),
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
    value: &crate::Recognized,
    record: &crate::RawRecord,
    position: &crate::cli::intake::Position,
    text: &str,
    streams: bool,
) -> Result<String, Failure> {
    let value = source::located(value, position, text)?;
    if streams {
        json_line(&RecordValue {
            input: record,
            value,
        })
        .map_err(Failure::from)
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
        .map(String::as_str)
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

fn placed_failure(error: Failure, ordinal: usize) -> FeedRead {
    FeedRead::Failed(super::examples::at_record(error, ordinal).into_error())
}
