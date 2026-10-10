//! Native atomic execution owns judgments; the CLI retains framing and presentation.
use super::{
    JudgingInput,
    judged::{Held, Records},
};
use crate::core::{Outcome, Reading, RecordValue, Setting, Value, json_line};
use crate::failure::Failure;
use crate::schedule::{Judged, Output};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::process::ExitCode;
use std::sync::{Arc, Mutex, PoisonError};

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
    let inner = crate::cli::construction::engine(
        common,
        environment,
        configuration.folders,
        (configuration.backend.clone(), configuration.profile.clone()),
        common.jobs,
        false,
    )?;
    let engine = crate::Engine::from_cli(inner, environment.config().prices());
    let composition = composition(&admitted, reading)?;
    let held = RefCell::new(BTreeMap::new());
    let reader = super::native_reader::Reader::new(records);
    let ready = || reader.ready();
    let readiness = crate::public::cli_reader::CliReader::new(&ready, environment.input_pause());
    reader.wake(readiness.wake());
    let rows = std::iter::from_fn(|| {
        reader.next().map(|row| {
            let unit = row.map_err(|placed| input_error(placed.cause, placed.at))?;
            let record = compose(&composition, &unit)?;
            held.borrow_mut().insert(unit.ordinal, unit);
            Ok(record)
        })
    });
    let token = crate::CancelToken::new();
    let downstream = crate::edge::Downstream::default();
    let signal = || {
        if downstream.gone() {
            token.cancel();
        }
        environment.cancel().fired() || token.is_cancelled()
    };
    let context = configuration
        .context
        .as_ref()
        .map(super::Context::evidence)
        .map(|c| c.as_text().map(|s| s.into_owned()))
        .transpose()?;
    let guard = Mutex::new((crate::schedule::ModelGuard::default(), None));
    let observe = |event: crate::RecordObservation<'_>| {
        if let crate::RecordObservation::Question { detail, .. } = event {
            let mut guard = guard.lock().unwrap_or_else(PoisonError::into_inner);
            if guard.1.is_none() {
                guard.1 = guard
                    .0
                    .check_sources(detail.question_sources().iter())
                    .err();
            }
            if guard.1.is_some() {
                token.cancel();
            }
        }
    };
    let mut controls = controls(
        (environment, common, configuration.streams),
        &token,
        &signal,
        &readiness,
        setting,
        context.as_deref(),
    );
    if configuration.keeping == crate::judge::Keeping::Ordered {
        controls = controls.observe(&observe);
    }
    let mut feed = crate::RequestFeed::from_records("cli-atomic", rows);
    if common.images() {
        feed = feed.with_image_inputs();
    }
    if configuration.keeping == crate::judge::Keeping::Passing {
        feed = feed.with_all_filter_results();
    }
    if configuration.keeping != crate::judge::Keeping::Answers {
        output.guard_models();
    }
    let rendering = Renderer {
        view: configuration.view,
        keeping: configuration.keeping,
        streams: configuration.streams,
        documents: configuration.documents,
        text_view: configuration.text_view,
        mismatch: configuration.mismatch,
        declarations: configuration.declarations,
    };
    let ended = RefCell::new(Ended::default());
    let output = RefCell::new(output);
    let sink = |value| {
        let mut ended = ended.borrow_mut();
        rendering.receive(value, &held, reading, &mut output.borrow_mut(), &mut ended);
        if ended.closed || ended.failure.is_some() {
            token.cancel();
        }
    };
    let admitted = admitted.with_composed_feed("cli-atomic");
    let release = |ordinal| {
        held.borrow_mut().remove(&ordinal);
    };
    let result = engine.execute_cli_request(
        &admitted,
        crate::RequestEnvironment {
            controls,
            feed: Some(feed),
        },
        &sink,
        &release,
        None,
    );
    if let Some(failure) = guard.into_inner().unwrap_or_else(PoisonError::into_inner).1 {
        ended.borrow_mut().failure.get_or_insert(failure);
    }
    let result = if configuration.keeping == crate::judge::Keeping::Ordered {
        result.map(|outcome| match outcome {
            crate::RequestOutcome::Complete(call) => {
                crate::RequestOutcome::Complete(call.map(|value| {
                    sink(value);
                    crate::RequestValue::Ranked(Vec::new())
                }))
            }
            stopped => stopped,
        })
    } else {
        result
    };
    rendering.finish(
        environment,
        result,
        ended.into_inner(),
        (recording, configuration.asks.verb(), downstream.latched()),
    )
}

pub(super) fn composition(
    admitted: &crate::AdmittedRequest,
    reading: &Reading,
) -> Result<crate::RecordReading, Failure> {
    let fields = reading
        .fields()
        .iter()
        .map(crate::core::Pointer::as_str)
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

fn controls<'a>(
    configuration: (&crate::edge::Environment, &crate::args::Common, bool),
    token: &'a crate::CancelToken,
    signal: &'a (dyn Fn() -> bool + Sync),
    readiness: &'a crate::public::cli_reader::CliReader<'a>,
    setting: Option<Setting>,
    context: Option<&'a str>,
) -> crate::CallOptions<'a> {
    let mut controls = crate::CallOptions::new()
        .cli_cancel(token, configuration.0.cancel().deadline())
        .interrupt(signal)
        .surface(crate::Surface::Cli)
        .attempts(configuration.1.details)
        .cli_reader(readiness);
    let setting = if !configuration.2 {
        Some(Setting::Records(std::num::NonZeroUsize::MIN))
    } else {
        setting
    };
    if let Some(setting) = setting {
        controls = controls.batch(match setting {
            Setting::Max => crate::BatchSetting::Max,
            Setting::Records(count) => crate::BatchSetting::Records(count),
        });
    }
    if let Some(context) = context {
        controls = controls.context(context);
    }
    controls
}

pub(super) fn compose(
    composition: &crate::RecordReading,
    unit: &Held,
) -> Result<crate::RecordInput<crate::QuestionInput>, crate::Error> {
    let mut record = composition.compose(crate::RawRecord(Arc::new(unit.record.clone())))?;
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
        environment: &crate::edge::Environment,
        result: Result<crate::RequestOutcome, crate::Error>,
        ended: Ended,
        completion: (bool, &'static str, bool),
    ) -> Result<ExitCode, Failure> {
        let (recording, verb, closed) = completion;
        let failure = match result {
            Ok(crate::RequestOutcome::Complete(call)) => {
                environment.settle_native(call.facts());
                None
            }
            Ok(crate::RequestOutcome::Failed { error, .. }) | Err(error) => {
                let at = error.stopped().at();
                if let Some(facts) = error.facts() {
                    environment.settle_native(facts);
                }
                Some((at, host_error(error, self.streams)))
            }
        };

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
                held: false,
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

fn input_error(cause: Failure, at: Option<usize>) -> crate::Error {
    let error = crate::Error::usage("the CLI reader failed").with_diagnostic(
        crate::public::error::diagnostic::Diagnostic::CliInput(Box::new(cause)),
    );
    match at {
        Some(at) => error.at_record(at.saturating_sub(1)),
        None => error,
    }
}
#[derive(Default)]
struct Ended {
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
    documents: bool,
    text_view: bool,
    mismatch: crate::profile::Mismatch,
    declarations: crate::core::declaration::QuestionMetadata,
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
        ($rows:expr) => {
            for row in $rows {
                let unit = held
                    .borrow_mut()
                    .remove(&row.ordinal())
                    .ok_or(Failure::Defect(
                        "native atomic row lost its host occurrence",
                    ))?;
                let judged = rendering.row(reading, unit, row.result().canonical.clone())?;
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
        crate::RequestValue::Decisions(v) => rows!(v),
        crate::RequestValue::Choices(v) => rows!(v),
        crate::RequestValue::Tags(v) => rows!(v),
        crate::RequestValue::Scores(v) => rows!(v),
        crate::RequestValue::Filtered(v) => rows!(v),
        crate::RequestValue::Ranked(v) => {
            for row in v {
                let unit = held
                    .borrow_mut()
                    .remove(&row.ordinal())
                    .ok_or(Failure::Defect("native rank lost its host occurrence"))?;
                let judged = rendering.row(reading, unit, row.result().canonical.clone())?;
                if output.take(judged)? {
                    ended.finished += 1;
                } else {
                    ended.closed = true;
                    break;
                }
            }
        }
        crate::RequestValue::SetRanked(v) => {
            for row in v {
                let unit = held
                    .borrow_mut()
                    .remove(&row.ordinal())
                    .ok_or(Failure::Defect("native set rank lost its host occurrence"))?;
                let mut judged =
                    rendering.row(reading, unit, row.result().result().canonical.clone())?;
                if rendering.view.details {
                    let original = row.original();
                    judged.printed = Some(json_line(&RankMembers {
                        canonical: &row.result().result().canonical,
                        original,
                        index: row.ordinal(),
                        name: row.result().question_name(),
                        members: row.result().members(),
                    })?);
                    crate::cli::intake::locate(&mut judged.printed, judged.position.as_ref())?;
                    crate::cli::intake::source_members(
                        &mut judged.printed,
                        judged.position.as_ref(),
                    )?;
                }
                if output.take(judged)? {
                    ended.finished += 1;
                } else {
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
    fn row(
        &self,
        reading: &Reading,
        unit: Held,
        mut canonical: crate::core::CompleteAtomic,
    ) -> Result<Judged, Failure> {
        canonical.declarations = self.declarations.clone();
        let value = canonical.value();
        let outcome = match value {
            Value::YesNo(Some(true)) => Outcome::Yes,
            Value::YesNo(Some(false)) => Outcome::No,
            Value::YesNo(None) | Value::Choice(None) => Outcome::Unresolved,
            _ => Outcome::Yes,
        };
        let replayed = canonical
            .identity
            .question_sources()
            .iter()
            .all(|s| s.origin() != crate::core::Origin::Live);
        let model = canonical
            .identity
            .question_sources()
            .iter()
            .find(|source| source.origin() == crate::core::Origin::Live)
            .map(|source| source.model().clone());
        let passing = self.keeping == crate::judge::Keeping::Passing;
        let mut printed = if passing && outcome != Outcome::Yes {
            None
        } else if self.view.details {
            Some(json_line(&Details {
                canonical: &canonical,
                original: (self.streams
                    || passing
                    || self.keeping == crate::judge::Keeping::Ordered)
                    .then_some(&unit.record),
                index: (passing || self.keeping == crate::judge::Keeping::Ordered)
                    .then_some(unit.ordinal),
            })?)
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
            Some(json_line(&RecordValue::new(
                unit.record.clone(),
                value.clone(),
            ))?)
        } else {
            Some(json_line(value)?)
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
            order_value: match canonical.value() {
                Value::Score(value) => Some(*value),
                _ => canonical.answer().yes(),
            },
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
struct Details<'a> {
    canonical: &'a crate::core::CompleteAtomic,
    original: Option<&'a crate::core::Record>,
    index: Option<usize>,
}
struct RankMembers<'a> {
    canonical: &'a crate::core::CompleteAtomic,
    original: &'a crate::QuestionInput,
    index: usize,
    name: &'a str,
    members: &'a [crate::CompleteRankMember],
}
impl serde::Serialize for RankMembers<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical.serialize_occurrence_members(
            Some(self.original),
            Some(self.name),
            Some(self.index),
            Some(
                self.members
                    .iter()
                    .map(|member| crate::core::RankMemberDocument {
                        name: member.name(),
                        result: &member.result().canonical,
                    })
                    .collect(),
            ),
            serializer,
        )
    }
}
impl serde::Serialize for Details<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical
            .serialize_occurrence(self.original, None, self.index, serializer)
    }
}

fn host_error(mut error: crate::Error, streams: bool) -> Failure {
    match error.take_diagnostic() {
        Some(crate::public::error::diagnostic::Diagnostic::PartialReply {
            cause: Some(cause),
            ..
        }) if !streams => Failure::Reply(cause),
        Some(diagnostic) => Failure::from(error.with_diagnostic(diagnostic)),
        None => Failure::from(error),
    }
}
