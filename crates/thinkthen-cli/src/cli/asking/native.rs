//! Native atomic execution owns judgments; the CLI retains framing and presentation.
use super::{
    JudgingInput,
    judged::{Held, Records},
};
use crate::BatchSetting as Setting;
use crate::cli::reading::Reading;
use crate::schedule::Outcome;
mod presentation;
use crate::failure::Failure;
use crate::render::json_line;
use crate::schedule::{Judged, Output};
use presentation::Atomic;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::process::ExitCode;

#[expect(
    clippy::too_many_lines,
    reason = "one bridge owns input framing, presentation and joined native facts"
)]
pub(super) fn run(
    configuration: JudgingInput<'_>,
    admitted: crate::AdmittedRequest,
    reading: &Reading,
    records: Records,
    setting: Option<Setting>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let environment = configuration.environment;
    let common = configuration.common;
    let recording = configuration.folders.reported();
    let engine =
        crate::cli::construction::engine(common, environment, configuration.folders, common.jobs)?;
    environment.observe_engine(&engine);
    let composition = composition(&admitted, reading)?;
    let function = admitted.request().call.function();
    let definition = admitted.resolve_question()?;
    let mut arguments = admitted.request().call.arguments().clone();
    arguments.question = crate::RequestQuestion::Definition { value: definition };
    arguments.input = crate::RequestInput::Feed {
        name: "cli-atomic".to_owned(),
        framing: crate::RequestFraming::Document,
        reading: crate::ReaderOptions::default(),
        images: Vec::new(),
    };
    arguments.options.field = None;
    arguments.options.context_field = None;
    arguments.options.options_field = None;
    arguments.options.seed_spans_field = None;
    arguments.options.examples_field = None;
    arguments.options.attempts = common.details;
    arguments.options.context = configuration.context.as_ref().map(super::Context::evidence);
    arguments.options.batch = if !configuration.streams {
        Some(crate::RequestBatch::Count(1))
    } else {
        setting.map(|setting| match setting {
            Setting::Max => crate::RequestBatch::Named("max".to_owned()),
            Setting::Records(count) => crate::RequestBatch::Count(count.get()),
        })
    };
    let request = crate::Request::for_function(function, arguments);
    let tally = environment.tally().start();
    let session = engine.request_session_with_feed_options(
        request,
        crate::Surface::Cli,
        crate::RequestSessionFeedOptions {
            on_missing_annotation_pointer: None,
            incremental_recognition: false,
            incremental_ranking: function == crate::RequestFunction::Rank,
            input_pause: environment.input_pause(),
            retain_success_on_cancel: true,
            eager: false,
            image_inputs: common.images(),
            all_filter_results: configuration.keeping == crate::judge::Keeping::Passing,
            record_reading: Some(composition),
        },
    )?;
    let held = RefCell::new(BTreeMap::new());
    let reader = super::native_reader::Reader::new(records);
    let downstream = crate::edge::Downstream::default();
    if configuration.keeping != crate::judge::Keeping::Answers {
        output.guard_models();
    }
    let rendering = Renderer {
        view: configuration.view,
        keeping: configuration.keeping,
        streams: configuration.streams,
        size_advice: if common.images() {
            crate::failure::SizeAdvice::Backend
        } else if configuration.streams {
            crate::failure::SizeAdvice::Batches
        } else {
            crate::failure::SizeAdvice::Text
        },
        documents: configuration.documents,
        text_view: configuration.text_view,
        mismatch: configuration
            .mismatch
            .running_profile(configuration.profile.as_deref(), engine.profile_name()),
    };
    let mut progress = RankProgress::default();
    let mut ended = Ended::default();
    let mut input_done = false;
    let mut cancelled = false;
    let mut tally = Some(tally);
    let mut terminal = None;
    loop {
        if !cancelled
            && (environment.cancel().fired()
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
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Row(row)) => {
                match row_value(row) {
                    Ok(value) => rendering.receive(value, &held, reading, output, &mut ended),
                    Err(error) => {
                        ended.failure.get_or_insert(error);
                    }
                }
            }
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Aggregate(value)) => {
                rendering.receive(value, &held, reading, output, &mut ended);
            }
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Observation {
                value,
                ..
            }) if configuration.keeping == crate::judge::Keeping::Ordered => {
                progress.observe(value);
                if progress.failure.is_some() {
                    session.cancel();
                    cancelled = true;
                    input_done = true;
                }
            }
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Observation {
                ..
            }) => {}
            crate::RequestSessionRead::Result(crate::RequestSessionResult::Terminal(value)) => {
                environment.settle_session(&value, &mut tally)?;
                terminal = Some(value.error.map_or(Ok(()), Err));
            }

            crate::RequestSessionRead::End => break,
            crate::RequestSessionRead::Pending => {}
        }
        if received {
            continue;
        }
        let mut pending_reader = false;
        if !input_done {
            let pushed = session.try_push_with(|| match reader.try_next() {
                super::native_reader::Read::Pending => Err(FeedRead::Pending),
                super::native_reader::Read::End => Err(FeedRead::End),
                super::native_reader::Read::Item(Err(placed)) => {
                    ended.input_at = placed.at;
                    Err(FeedRead::Failed(placed.cause.into_error()))
                }
                super::native_reader::Read::Item(Ok(unit)) => {
                    let descriptor = descriptor(&unit)?;
                    held.borrow_mut().insert(unit.ordinal, unit);
                    Ok(descriptor)
                }
            });
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
        // A caller-owned input read may block. Poll the signal while the native
        // session owns all provider waits and final settlement.
        if pending_reader {
            reader.wait(std::time::Duration::from_millis(50));
            continue;
        }
        if crate::asking::native_reader::session_ended(&session, cancelled, input_done) {
            break;
        }
    }
    if configuration.keeping == crate::judge::Keeping::Ordered {
        ended.finished = progress.finished;
        ended.replayed = progress.replayed;
        if let Some(failure) = progress.failure {
            ended.failure.get_or_insert(failure);
        }
    }
    rendering.finish(
        terminal.ok_or(Failure::Defect("native session ended without settlement"))?,
        ended,
        (recording, configuration.asks.verb(), downstream.latched()),
    )
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

fn descriptor(unit: &Held) -> Result<crate::RequestSessionDescriptor, crate::Error> {
    let original = if unit
        .images
        .as_ref()
        .is_some_and(|images| images.text().is_none())
    {
        None
    } else {
        Some(match unit.record.literal() {
            Some(text) => crate::RequestOriginal::Text {
                text: text.to_owned(),
            },
            None => crate::RequestOriginal::Json {
                value: unit.record.clone(),
            },
        })
    };
    let images = unit.images.as_ref().map_or_else(Vec::new, |images| {
        images
            .images()
            .iter()
            .map(|image| crate::RequestImage::Bytes {
                media: image.media(),
                bytes: image.bytes().to_vec(),
            })
            .collect()
    });
    let location = unit
        .position
        .as_ref()
        .filter(|position| position.located)
        .and_then(|position| position.file.as_ref().map(|file| (file, position)))
        .map(|(file, position)| {
            crate::SourceLocation::new(file.clone(), position.first, position.last)
        })
        .transpose()?;
    Ok(crate::RequestSessionDescriptor {
        item: crate::RequestItem {
            original,
            images,
            context: unit.context.clone(),
            options: None,
            seed_spans: None,
            examples: None,
        },
        location,
    })
}

fn row_value(row: crate::RequestSessionRow) -> Result<crate::RequestValue, Failure> {
    Ok(match row {
        crate::RequestSessionRow::Decision(row) => crate::RequestValue::Decisions(vec![row]),
        crate::RequestSessionRow::Choice(row) => crate::RequestValue::Choices(vec![row]),
        crate::RequestSessionRow::Tags(row) => crate::RequestValue::Tags(vec![row]),
        crate::RequestSessionRow::Score(row) => crate::RequestValue::Scores(vec![row]),
        crate::RequestSessionRow::Filter(row) => crate::RequestValue::Filtered(vec![row]),
        crate::RequestSessionRow::Annotation(_) => {
            return Err(Failure::Defect("atomic session returned annotation"));
        }
    })
}

pub(super) fn composition(
    admitted: &crate::AdmittedRequest,
    reading: &Reading,
) -> Result<crate::RecordReading, Failure> {
    let fields = reading
        .fields()
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let options = admitted
        .request()
        .call
        .arguments()
        .options
        .options_field
        .as_deref();
    let composition = crate::RecordReading::new(&fields, None, options).map_err(Failure::from)?;
    Ok(composition)
}

pub(super) fn compose(
    composition: &crate::RecordReading,
    unit: &Held,
) -> Result<crate::RecordInput<crate::QuestionInput>, crate::Error> {
    if unit.record.literal().is_some()
        && let Some(images) = &unit.images
    {
        let mut record = composition.compose_images(images.clone())?;
        record.context = unit.context.clone();
        return Ok(record);
    }
    let mut record = composition.compose(unit.record.clone())?;
    record.context = unit.context.clone();
    if let Some(images) = &unit.images {
        record.original = record.original.with_images(images.images().to_vec())?;
    }
    if let Some(position) = unit.position.as_ref().filter(|p| p.located)
        && let Some(file) = &position.file
    {
        record.original = record.original.with_location(crate::SourceLocation::new(
            file.clone(),
            position.first,
            position.last,
        )?);
    }
    Ok(record.map_original(crate::QuestionInput::Record))
}

impl Renderer {
    fn receive(
        &self,
        value: crate::RequestValue,
        held: &RefCell<BTreeMap<usize, Held>>,
        reading: &Reading,
        output: &mut Output<'_>,
        ended: &mut Ended,
    ) {
        if ended.closed || ended.failure.is_some() {
            return;
        }
        let result = take(value, held, self, reading, output, ended);
        if let Err(error) = result {
            ended.failure = Some(error);
        }
    }
}

impl Renderer {
    fn finish(
        &self,
        result: Result<(), crate::Error>,
        ended: Ended,
        completion: (bool, &'static str, bool),
    ) -> Result<ExitCode, Failure> {
        let (recording, verb, closed) = completion;
        let failure = result.err().map(|error| {
            let at = error.stopped().at().or(ended.input_at);
            (
                at,
                host_error(error, self.streams).with_size_advice(self.size_advice),
            )
        });

        if ended.closed || closed {
            return Ok(ExitCode::SUCCESS);
        }
        let failure = ended.failure.map(|failure| (None, failure)).or(failure);
        if let Some((at, cause)) = failure {
            if matches!(
                &cause,
                Failure::Context(crate::failure::context::Error::OverLimit { initial: true, .. })
            ) {
                return Err(cause);
            }
            if !self.streams && !self.documents && ended.finished > 0 {
                return Ok(super::exit_code(
                    ended.outcome.unwrap_or(Outcome::Unresolved),
                ));
            }
            if !self.streams {
                return Err(
                    cause.with_replay_context(crate::failure::ReplayContext::Document(verb))
                );
            }
            return Err(Failure::Stopped {
                at: at.unwrap_or(ended.finished + 1),
                finished: ended.finished,
                replayed: ended.replayed,
                recording,
                held: self.keeping == crate::judge::Keeping::Ordered,
                cause: Box::new(cause),
            });
        }
        if !self.streams && !self.documents {
            return Ok(super::exit_code(
                ended.outcome.unwrap_or(Outcome::Unresolved),
            ));
        }
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Default)]
struct RankProgress {
    models: crate::schedule::ModelGuard,
    failure: Option<Failure>,
    finished: usize,
    replayed: usize,
    live: bool,
}
impl RankProgress {
    fn observe(&mut self, event: crate::OwnedRecordObservation) {
        match event {
            crate::OwnedRecordObservation::Question { detail, .. } => {
                self.live |= detail
                    .detail()
                    .question_sources()
                    .iter()
                    .any(|source| source.origin() == crate::Origin::Live);
                if self.failure.is_none() {
                    self.failure = self
                        .models
                        .check_sources(detail.detail().question_sources().iter())
                        .err();
                }
            }
            crate::OwnedRecordObservation::Row { .. } => {
                self.finished += 1;
                self.replayed += usize::from(!self.live);
                self.live = false;
            }
        }
    }
}
#[derive(Default)]
struct Ended {
    input_at: Option<usize>,
    finished: usize,
    replayed: usize,
    outcome: Option<Outcome>,
    closed: bool,
    failure: Option<Failure>,
}
struct Renderer {
    view: crate::judge::View,
    keeping: crate::judge::Keeping,
    streams: bool,
    size_advice: crate::failure::SizeAdvice,
    documents: bool,
    text_view: bool,
    mismatch: crate::profile::Mismatch,
}

fn take(
    value: crate::RequestValue,
    held: &RefCell<BTreeMap<usize, Held>>,
    rendering: &Renderer,
    reading: &Reading,
    output: &mut Output<'_>,
    ended: &mut Ended,
) -> Result<(), Failure> {
    macro_rules! rows {
        ($rows:expr, $variant:ident) => {
            for row in $rows {
                let unit = held
                    .borrow_mut()
                    .remove(&row.ordinal())
                    .ok_or(Failure::Defect(
                        "native atomic row lost its host occurrence",
                    ))?;
                let judged = rendering.row(reading, unit, Atomic::$variant(row.result()))?;
                let replayed = judged.replayed;
                let outcome = judged.outcome;
                if output.take(judged)? {
                    ended.finished += 1;
                    ended.replayed += usize::from(replayed);
                    ended.outcome = Some(outcome);
                } else {
                    ended.closed = true;
                    break;
                }
            }
        };
    }
    match value {
        crate::RequestValue::Decisions(v) => rows!(v, Decision),
        crate::RequestValue::Choices(v) => rows!(v, Choice),
        crate::RequestValue::Tags(v) => rows!(v, Tags),
        crate::RequestValue::Scores(v) => rows!(v, Score),
        crate::RequestValue::Filtered(v) => rows!(v, Filter),
        crate::RequestValue::Ranked(v) => {
            for row in v {
                let unit = held.borrow_mut().remove(&row.ordinal());
                let judged = if rendering.view.details {
                    rendering.rank_details(&row, unit.and_then(|unit| unit.position))?
                } else {
                    rendering.row(
                        reading,
                        unit.ok_or(Failure::Defect("native rank lost its host occurrence"))?,
                        Atomic::Rank(row.result()),
                    )?
                };
                if !output.print(judged)? {
                    ended.closed = true;
                    break;
                }
            }
        }
        crate::RequestValue::SetRanked(v) => {
            for row in v {
                let unit = held.borrow_mut().remove(&row.ordinal());
                let judged = if rendering.view.details {
                    rendering.rank_details(&row, unit.and_then(|unit| unit.position))?
                } else {
                    rendering.row(
                        reading,
                        unit.ok_or(Failure::Defect("native set rank lost its host occurrence"))?,
                        Atomic::Rank(row.result().result()),
                    )?
                };
                if !output.print(judged)? {
                    ended.closed = true;
                    break;
                }
            }
        }
        _ => return Err(Failure::Defect("atomic request returned another function")),
    }
    Ok(())
}
impl Renderer {
    fn rank_details(
        &self,
        row: &impl serde::Serialize,
        position: Option<crate::cli::intake::Position>,
    ) -> Result<Judged, Failure> {
        let mut printed = Some(if self.streams {
            json_line(row)?
        } else {
            presentation::without_batch(row)?
        });
        crate::cli::intake::locate(&mut printed, position.as_ref())?;
        crate::cli::intake::source_members(&mut printed, position.as_ref())?;
        Ok(Judged {
            model: None,
            printed,
            position,
            outcome: Outcome::Yes,
            replayed: false,
            order_value: None,
            partial_failure: false,
            profile_mismatch: self.mismatch.notice(),
        })
    }
}
impl Renderer {
    fn row(&self, reading: &Reading, unit: Held, value: Atomic<'_>) -> Result<Judged, Failure> {
        let outcome = value.outcome();
        let metadata = value.meta();
        let replayed = metadata
            .identity()
            .question_sources()
            .iter()
            .all(|source| source.origin() != crate::Origin::Live);
        let model = metadata
            .identity()
            .question_sources()
            .iter()
            .find(|source| source.origin() == crate::Origin::Live)
            .map(|source| source.answered_by().to_owned());
        let passing = self.keeping == crate::judge::Keeping::Passing;
        let mut printed = if passing && outcome != Outcome::Yes {
            None
        } else if self.view.details {
            Some(
                value.details(
                    (self.streams || passing || self.keeping == crate::judge::Keeping::Ordered)
                        .then_some(&unit.record),
                    (passing || self.keeping == crate::judge::Keeping::Ordered)
                        .then_some(unit.ordinal),
                    self.streams,
                )?,
            )
        } else if self.keeping.streams_only() {
            Some(match &unit.arrived {
                Some(bytes) => reading.as_it_arrived(bytes)?.to_owned(),
                None => json_line(&unit.record)?,
            })
        } else if self.view.raw {
            match value.label() {
                Some(label) => Some(label.to_owned()),
                None if self.streams => Some(String::new()),
                None => None,
            }
        } else if self.view.quiet {
            None
        } else if self.streams {
            #[derive(serde::Serialize)]
            struct RecordValue<'a> {
                input: &'a crate::RawRecord,
                value: crate::saved_json::Json,
            }
            Some(json_line(&RecordValue {
                input: &unit.record,
                value: value.value()?,
            })?)
        } else {
            Some(json_line(&value.value()?)?)
        };
        if self.view.details {
            crate::cli::intake::locate(&mut printed, unit.position.as_ref())?;
        }
        self.locate(&unit, &mut printed)?;
        Ok(Judged {
            model,
            printed,
            position: unit.position,
            outcome,
            replayed,
            order_value: value.order_value(),
            partial_failure: false,
            profile_mismatch: self.mismatch.notice(),
        })
    }
    fn locate(&self, unit: &Held, printed: &mut Option<String>) -> Result<(), Failure> {
        if let Some(images) = &unit.images {
            if self.view.details && !self.streams {
                crate::cli::intake::image_input(printed, images)?;
            }
            if self.view.details {
                crate::cli::intake::source_members(printed, unit.position.as_ref())?;
            } else if let Some(position) = unit.position.as_ref().filter(|p| p.located) {
                *printed = printed
                    .as_ref()
                    .map(|v| crate::cli::intake::source_value(images, v, position))
                    .transpose()?;
            }
            return Ok(());
        }
        self.source(unit, printed)?;
        if self.documents && !unit.position.as_ref().is_some_and(|p| p.located) {
            crate::cli::intake::document(printed, unit.position.as_ref(), self.view.details)?;
        }
        Ok(())
    }
    fn source(&self, unit: &Held, printed: &mut Option<String>) -> Result<(), Failure> {
        let passing = self.keeping == crate::judge::Keeping::Passing;
        let Some(position) = unit
            .position
            .as_ref()
            .filter(|p| p.located && !self.text_view)
        else {
            return Ok(());
        };
        if self.view.details || (self.streams && !passing && !self.view.raw) {
            return crate::cli::intake::source_members(printed, Some(position));
        }
        let Some(line) = printed else {
            return Ok(());
        };
        let original = json_line(&unit.record)?;
        let shown = if self.keeping.streams_only() {
            &original
        } else {
            &*line
        };
        *line = crate::cli::intake::source_value(&unit.record, shown, position)?;
        Ok(())
    }
}

fn host_error(error: crate::Error, streams: bool) -> Failure {
    if !streams
        && let Some(crate::ErrorDiagnostic::PartialReply {
            cause: Some(cause), ..
        }) = error.diagnostic()
    {
        return Failure::Reply(cause);
    }
    Failure::from(error)
}
