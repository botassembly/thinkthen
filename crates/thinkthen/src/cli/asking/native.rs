//! Native atomic execution owns judgments; the CLI retains framing and presentation.
use super::{JudgingInput, judged::{Held, Records}};
use crate::core::{Outcome, Reading, RecordValue, Setting, Value, json_line};
use crate::failure::Failure;
use crate::schedule::{Judged, Output};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::process::ExitCode;
use std::sync::Arc;

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
    let inner = crate::cli::construction::engine(common, environment, configuration.folders, (configuration.backend.clone(), configuration.profile.clone()), common.jobs, false)?;
    let engine = crate::Engine::from_cli(inner, environment.config().prices());
    let fields = reading.fields().iter().map(crate::core::Pointer::as_str).collect::<Vec<_>>();
    let options = admitted.request().call.arguments().options.options_field.as_deref();
    let composition = crate::RecordReading::new(&fields, None, options).map_err(Failure::from)?;
    let held = RefCell::new(VecDeque::new());
    let mut reader = super::native_reader::Reader::new(records);
    let ready = || reader.ready();
    let readiness = crate::public::options::cli_reader::CliReader::new(&ready, environment.input_pause());
    reader.wake(readiness.wake());
    let rows = std::iter::from_fn(|| {
        reader.next().map(|row| {
            let unit = row.map_err(|placed| input_error(placed.cause, placed.at))?;
            let mut record = composition.compose(crate::RawRecord(Arc::new(unit.record.clone())))?;
            record.context = unit.context.clone();
            if let Some(images) = &unit.images {
                record.original = record.original.with_images(images.images().to_vec())?;
            }
            if let Some(position) = unit.position.as_ref().filter(|p| p.located)
                && let Some(file) = &position.file {
                record.original = record.original.with_location(crate::SourceLocation::new(file.clone(), position.first, position.last)?);
            }
            held.borrow_mut().push_back(unit);
            Ok(record.map_original(crate::QuestionInput::Record))
        })
    });
    let token = crate::CancelToken::new();
    let signal = || environment.cancel().fired();
    let context = configuration.context.as_ref().map(super::Context::evidence).map(|c| c.as_text().map(|s| s.into_owned())).transpose()?;
    let mut controls = crate::CallOptions::new().cli_cancel(&token, environment.cancel().deadline()).interrupt(&signal).surface(crate::Surface::Cli).attempts(common.details).cli_reader(&readiness);
    if let Some(setting) = setting {
        controls = controls.batch(match setting {
            Setting::Max => crate::BatchSetting::Max,
            Setting::Records(count) => crate::BatchSetting::Records(count),
        });
    }
    if let Some(context) = &context { controls = controls.context(context); }
    let mut feed = crate::RequestFeed::from_records("cli-atomic", rows);
    if common.images() { feed = feed.with_image_inputs(); }
    if configuration.keeping == crate::judge::Keeping::Passing { feed = feed.with_all_filter_results(); output.guard_models(); }
    let rendering = Renderer {
        view: configuration.view,
        keeping: configuration.keeping,
        streams: configuration.streams,
        documents: configuration.documents,
        text_view: configuration.text_view,
        mismatch: configuration.mismatch,
    };
    let ended = RefCell::new(Ended::default());
    let output = RefCell::new(output);
    let sink = |value| {
        let mut ended = ended.borrow_mut();
        if ended.closed || ended.failure.is_some() { return; }
        let result = take(value, &held, &rendering, reading, &mut output.borrow_mut(), &mut ended);
        if let Err(error) = result { ended.failure = Some(error); }
        if ended.closed || ended.failure.is_some() { token.cancel(); }
    };
    let admitted = admitted.with_composed_feed("cli-atomic");
    let result = engine.execute_cli_request(&admitted, crate::RequestEnvironment { controls, feed: Some(feed) }, &sink);
    let failure = match result {
        Ok(crate::RequestOutcome::Complete(call)) => { environment.settle_native(call.facts()); None }
        Ok(crate::RequestOutcome::Failed { error, .. }) | Err(error) => {
            let at = error.stopped().at();
            if let Some(facts) = error.facts() { environment.settle_native(facts); }
            Some((at, Failure::from(error)))
        }
    };
    let ended = ended.into_inner();
    if ended.closed { return Ok(ExitCode::SUCCESS); }
    let failure = ended.failure.map(|failure| (None, failure)).or(failure);
    if let Some((at, cause)) = failure {
        if !rendering.streams && !rendering.documents && ended.finished > 0 {
            return Ok(super::exit_code(ended.outcome.unwrap_or(Outcome::Unresolved)));
        }
        if !rendering.streams { return Err(cause.with_replay_context(crate::failure::ReplayContext::Document("atomic"))); }
        return Err(Failure::Stopped { at: at.unwrap_or(ended.finished + 1), finished: ended.finished, replayed: ended.replayed, recording, held: false, cause: Box::new(cause) });
    }
    if !rendering.streams && !rendering.documents { return Ok(super::exit_code(ended.outcome.unwrap_or(Outcome::Unresolved))); }
    output.into_inner().ended()?;
    Ok(ExitCode::SUCCESS)
}

fn input_error(cause: Failure, at: Option<usize>) -> crate::Error {
    let error = crate::Error::usage("the CLI reader failed").with_diagnostic(crate::public::error::diagnostic::Diagnostic::CliInput(Box::new(cause)));
    at.map_or(error, |at| error.at_record(at.saturating_sub(1)))
}
#[derive(Default)]
struct Ended { finished: usize, replayed: usize, outcome: Option<Outcome>, closed: bool, failure: Option<Failure> }
struct Renderer { view: crate::judge::View, keeping: crate::judge::Keeping, streams: bool, documents: bool, text_view: bool, mismatch: crate::profile::Mismatch }

fn take(value: crate::RequestValue, held: &RefCell<VecDeque<Held>>, rendering: &Renderer, reading: &Reading, output: &mut Output<'_>, ended: &mut Ended) -> Result<(), Failure> {
    macro_rules! rows { ($rows:expr) => { for row in $rows {
        let unit = held.borrow_mut().pop_front().ok_or(Failure::Defect("native atomic row lost its host occurrence"))?;
        let judged = rendering.row(reading, unit, row.result().canonical.clone())?;
        let replayed = judged.replayed;
        let outcome = judged.outcome;
        if output.take_members(vec![judged], ended.finished)? { ended.finished += 1; ended.replayed += usize::from(replayed); ended.outcome = Some(outcome); } else { ended.closed = true; break; }
    } }; }
    match value { crate::RequestValue::Decisions(v) => rows!(v), crate::RequestValue::Choices(v) => rows!(v), crate::RequestValue::Tags(v) => rows!(v), crate::RequestValue::Scores(v) => rows!(v), crate::RequestValue::Filtered(v) => rows!(v), _ => return Err(Failure::Defect("atomic request returned another function")) }
    Ok(())
}
impl Renderer {
    fn row(&self, reading: &Reading, unit: Held, canonical: crate::core::CompleteAtomic) -> Result<Judged, Failure> {
        let value = canonical.value();
        let outcome = match value { Value::YesNo(Some(true)) => Outcome::Yes, Value::YesNo(Some(false)) => Outcome::No, Value::YesNo(None) | Value::Choice(None) => Outcome::Unresolved, _ => Outcome::Yes };
        let replayed = canonical.identity.question_sources().iter().all(|s| s.origin() == crate::core::Origin::Replay);
        let model = (!replayed).then(|| canonical.identity.question_sources().first().map(|s| s.model().clone())).flatten();
        let passing = self.keeping == crate::judge::Keeping::Passing;
        let mut printed = if passing && outcome != Outcome::Yes { None }
        else if self.view.details { Some(json_line(&Details { canonical: &canonical, original: (self.streams || passing).then_some(&unit.record), index: passing.then_some(unit.ordinal) })?) }
        else if passing { Some(match &unit.arrived { Some(bytes) => reading.as_it_arrived(bytes)?.to_owned(), None => json_line(&unit.record)? }) }
        else if self.view.raw { match value.label() { Some(label) => Some(label.to_owned()), None if self.streams => Some(String::new()), None => None } }
        else if self.view.quiet { None }
        else if self.streams { Some(json_line(&RecordValue::new(unit.record.clone(), value.clone()))?) }
        else { Some(json_line(value)?) };
        if self.view.details { crate::cli::intake::locate(&mut printed, unit.position.as_ref())?; }
        if let Some(images) = &unit.images {
            if self.view.details { if !self.streams { crate::cli::intake::image_input(&mut printed, images)?; } crate::cli::intake::source_members(&mut printed, unit.position.as_ref())?; }
            else if let Some(position) = unit.position.as_ref().filter(|p| p.located) { printed = printed.as_ref().map(|v| crate::cli::intake::source_value(images, v, position)).transpose()?; }
        } else if let Some(position) = unit.position.as_ref().filter(|p| p.located && !self.text_view) {
            if self.view.details || (self.streams && !passing && !self.view.raw) { crate::cli::intake::source_members(&mut printed, Some(position))?; }
            else if let Some(line) = &mut printed { let original = json_line(&unit.record)?; *line = crate::cli::intake::source_value(&unit.record, if passing { &original } else { line }, position)?; }
        }
        if self.documents && !unit.position.as_ref().is_some_and(|p| p.located) { crate::cli::intake::document(&mut printed, unit.position.as_ref(), self.view.details)?; }
        Ok(Judged { rank: None, model, printed, position: unit.position, outcome, replayed, order_value: canonical.answer().yes(), partial_failure: false, profile_mismatch: self.mismatch.notice() })
    }
}
struct Details<'a> { canonical: &'a crate::core::CompleteAtomic, original: Option<&'a crate::core::Record>, index: Option<usize> }
impl serde::Serialize for Details<'_> { fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok,S::Error> { self.canonical.serialize_occurrence(self.original, None, self.index, serializer) } }
