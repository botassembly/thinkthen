//! The one aggregate request behind the public `find` command.

use crate::plan_document::PlanDocument;
use std::io::{Read, Write};
use std::process::ExitCode;

use crate::{RawRecord as Record, RequestFraming as Framing, ResolvedSettings};
use crate::cli::reading::Reading;
use crate::render::json_line;

use crate::args::{Common, FindArguments};
use crate::asking::{self, Folders};
use crate::cli::{
    display::Display,
    intake::{self, Position, Snapshot},
};
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::profile;
mod question;
mod result;

/// Read one bounded set, ask once, and print its selected original unit.
pub(crate) fn run(
    arguments: &FindArguments,
    environment: &Environment,
    mut admitted: crate::AdmittedRequest,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let question::Prepared {
        authored_model,
        common,
        sources,
    } = question::Prepared::new(arguments, &mut admitted)?;
    let common = &common;
    let context = asking::context::shared(arguments.common.context.as_deref())?;
    let mut display = display(arguments, common)?;
    let reading = Reading::new(framing(common), fields(common)?)?;
    let backend = environment.resolve(common.backend.as_deref(), common.url.as_deref(), common.model.as_deref())?;
    environment.set_request_size(environment.request_size(arguments.common.max_request_bytes.as_deref())?)?;
    profile::read(common, environment)?;
    let folders = Folders::of(common, environment)?;
    if common.dry_run && folders.named() { return Err(Failure::DryRunWithRecording); }
    let recording = folders.reported();
    let engine = if common.dry_run { None } else {
        Some(crate::cli::construction::engine(common, environment, folders, None)?)
    };
    let range = crate::find_candidate_count_range(arguments.none);
    let most = *range.end();
    let units = input_units(
        common,
        input,
        &mut display,
        &reading,
        (most, arguments.none, recording),
    )?;
    if units.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    // Native whole-set admission validates each typed item before its minimum count.
    if common.dry_run {
        return planned(
            admitted,
            (common, environment),
            (&backend, context.as_deref()),
            (&reading, sources),
            &units,
            writer,
        );
    }
    let engine = engine.ok_or(Failure::Defect("a live find run has no engine"))?;
    let found = execute(
        admitted,
        engine,
        common,
        environment,
        &units,
        context.as_deref(),
    )?;
    let rendered = result::rendered(common, &reading, &units, &found, authored_model.as_deref())?;
    display.emit_row(
        &mut writer,
        rendered.line.as_deref(),
        rendered.position.as_ref(),
        rendered.score,
    )?;
    Ok(ExitCode::from(if rendered.resolved { 0 } else { 3 }))
}

mod execution;
use execution::execute;

fn composition(common: &Common) -> Result<crate::RecordReading, Failure> {
    let fields = common.field.iter().map(String::as_str).collect::<Vec<_>>();
    crate::RecordReading::new(&fields, None, None).map_err(Failure::from)
}

fn composed<'a>(
    composition: &'a crate::RecordReading,
    units: &'a [Unit],
) -> impl Iterator<Item = Result<crate::RecordInput<crate::QuestionInput>, crate::Error>> + 'a {
    units.iter().map(|unit| {
        let mut record =
            composition.compose(unit.record.clone())?;
        if unit.position.located
            && let Some(file) = &unit.position.file
        {
            record.original = record.original.with_location(crate::SourceLocation::new(
                file.clone(),
                unit.position.first,
                unit.position.last,
            )?);
        }
        Ok(record.map_original(crate::QuestionInput::Record))
    })
}

fn display(arguments: &FindArguments, common: &Common) -> Result<Display, Failure> {
    let mut display = Display::default();
    display.arguments = arguments.display.clone();
    display.validate(common)?;
    Ok(display)
}

/// Print exactly the native admitted whole-set request and aggregate counts.
fn planned(
    admitted: crate::AdmittedRequest,
    (common, environment): (&Common, &Environment),
    (backend, context): (&ResolvedSettings, Option<&str>),
    (reading, sources): (&Reading, Option<crate::plan_document::Sources>),
    units: &[Unit], mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let native = crate::cli::construction::engine(common, environment, Folders::of(common, environment)?, None)?;
    let composition = composition(common)?;
    let rows = composed(&composition, units);
    let admitted = request(&admitted, "cli-find-plan", context)?.admit()?;
    let summary = native.plan_request(&admitted, crate::RequestEnvironment {
        controls: crate::CallOptions::new().surface(crate::Surface::Cli),
        feed: Some(crate::RequestFeed::from_records("cli-find-plan", rows)),
    })?;
    let mut document = PlanDocument::of_body(backend, summary.first_body()
        .ok_or(Failure::Defect("a find preview has no request"))?.to_vec())?
        .key_env(&backend.key_variable);
    if let Some(sources) = sources { document = document.from(sources); }
    edge::write_line(&mut writer, &json_line(&document.reading(reading.framing(), reading.fields().to_vec(), reading.is_default()))?)?;
    edge::write_line(&mut writer, &json_line(&crate::cli::plan_counts::Counts::from(&summary))?)?;
    Ok(ExitCode::SUCCESS)
}

fn request(admitted: &crate::AdmittedRequest, feed: &str, context: Option<&str>) -> Result<crate::Request, Failure> {
    let mut arguments = admitted.request().call.arguments().clone();
    arguments.question = crate::RequestQuestion::Definition { value: admitted.resolve_question()? };
    arguments.input = crate::RequestInput::Feed { name: feed.to_owned(), framing: Framing::Document,
        reading: crate::ReaderOptions::default(), images: Vec::new() };
    arguments.options.field = None;
    arguments.options.context = context.map(str::to_owned);
    Ok(crate::Request::for_function(crate::RequestFunction::Find, arguments))
}

fn fields(common: &Common) -> Result<Vec<String>, Failure> {
    common.field.iter().map(|typed| {
        crate::validate_pointer(typed).map_err(|error| Failure::Pointer("--field", typed.clone(), error.to_string()))?;
        Ok(typed.clone())
    }).collect()
}

fn input_units(
    common: &Common,
    input: impl Read + Send + 'static,
    display: &mut Display,
    reading: &Reading,
    (most, none, recording): (usize, bool, bool),
) -> Result<Vec<Unit>, Failure> {
    if common.located() || common.input.len() > 1 {
        return located_units(common, input, display, reading, (most, none, recording));
    }
    let path = common.input.first();
    let mut source = edge::source(path.map(std::path::PathBuf::as_path), input)?;
    if display.around.is_some() {
        let (opened, snapshot) = Snapshot::prepare(vec![(path.cloned(), source)])?;
        source = opened
            .into_iter()
            .next()
            .ok_or(Failure::Defect("a find snapshot has no source"))?
            .1;
        display.snapshot = snapshot;
    }
    read_units(
        reading,
        source,
        path.map(|path| path.to_string_lossy().into_owned()),
        most,
        none,
        recording,
    )
}

fn located_units(
    common: &Common,
    input: impl Read + Send + 'static,
    display: &mut Display,
    reading: &Reading,
    (most, none, recording): (usize, bool, bool),
) -> Result<Vec<Unit>, Failure> {
    let (intake, snapshot) =
        intake::Intake::prepare(common, reading, input, false, display.around.is_some())?;
    display.snapshot = snapshot;
    let mut units = Vec::new();
    let mut original = 0usize;
    for item in intake {
        let item = item.map_err(|placed| placed.cause)?;
        if units.len() == most {
            return Err(Failure::FindCount { none });
        }
        let intake::Data::Bytes(bytes) = item.data else {
            return Err(Failure::Defect("find reader returned a table"));
        };
        original = original
            .checked_add(bytes.len())
            .ok_or(Failure::FindTooLarge)?;
        if original > Record::max_bytes() {
            return Err(Failure::FindTooLarge);
        }
        let record = reading
            .record(&bytes)
            .map_err(|error| stopped(units.len(), recording, Failure::record(error, true)))?;
        reading
            .evidence(&record)
            .map_err(|error| stopped(units.len(), recording, Failure::record(error, true)))?;
        let mut position = item
            .position
            .ok_or(Failure::Defect("find source has no physical position"))?;
        position.located = true;
        units.push(Unit {
            bytes,
            record,
            position,
        });
    }
    Ok(units)
}

struct Unit {
    bytes: Vec<u8>,
    record: Record,
    position: Position,
}

struct Rendered {
    line: Option<String>,
    resolved: bool,
    position: Option<Position>,
    score: Option<f64>,
}

fn read_units(
    reading: &Reading,
    source: Box<dyn std::io::BufRead + Send>,
    file: Option<String>,
    most: usize,
    none: bool,
    recording: bool,
) -> Result<Vec<Unit>, Failure> {
    let mut chunks = crate::FileReader::new(file.clone().unwrap_or_default(), source, crate::ReaderOptions { unit: crate::SourceUnit::Line, window: None })?;
    let mut original = 0usize;
    let mut units = Vec::new();
    while let Some(unit) = chunks.next_bytes().transpose().map_err(intake::input_failure)? {
        let bytes = unit.record;
        if units.len() == most {
            return Err(Failure::FindCount { none });
        }
        original = original
            .checked_add(bytes.len())
            .ok_or(Failure::FindTooLarge)?;
        if original > Record::max_bytes() {
            return Err(Failure::FindTooLarge);
        }
        let record = reading
            .record(&bytes)
            .map_err(|error| stopped(units.len(), recording, Failure::record(error, true)))?;
        reading
            .evidence(&record)
            .map_err(|error| stopped(units.len(), recording, Failure::record(error, true)))?;
        let line = units.len() + 1;
        units.push(Unit {
            bytes,
            record,
            position: Position {
                file: file.clone(),
                first: Some(line),
                last: Some(line),
                images: None,
                source: 0,
                located: false,
            },
        });
    }
    Ok(units)
}

fn stopped(place: usize, recording: bool, cause: Failure) -> Failure {
    Failure::Stopped {
        at: place + 1,
        finished: 0,
        replayed: 0,
        recording,
        held: false,
        cause: Box::new(cause),
    }
}

fn framing(common: &Common) -> Framing {
    if common.unit.as_deref() == Some("file") {
        Framing::Document
    } else if common.jsonl {
        Framing::Jsonl
    } else {
        Framing::Lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::{self, BufRead}, sync::{Arc, atomic::{AtomicUsize, Ordering}}};
    struct Counted { bytes: Vec<u8>, at: usize, consumed: Arc<AtomicUsize> }
    impl Read for Counted {
        fn read(&mut self, target: &mut [u8]) -> io::Result<usize> {
            if target.is_empty() { return Ok(0); }
            let source = self.fill_buf()?;
            let Some(byte) = source.first() else { return Ok(0); };
            target[0] = *byte;
            self.consume(1);
            Ok(1)
        }
    }
    impl BufRead for Counted {
        fn fill_buf(&mut self) -> io::Result<&[u8]> { Ok(&self.bytes[self.at..(self.at + 1).min(self.bytes.len())]) }
        fn consume(&mut self, count: usize) { self.at += count; self.consumed.fetch_add(count, Ordering::Relaxed); }
    }
    #[test]
    fn candidate_overflow_stops_before_the_following_invalid_record() {
        for none in [false, true] {
            let most = *crate::find_candidate_count_range(none).end();
            let mut bytes = b"candidate\n".repeat(most + 1);
            let cutoff = bytes.len();
            bytes.extend_from_slice(b"\xffmust remain unread\n");
            let consumed = Arc::new(AtomicUsize::new(0));
            let source = Counted { bytes, at: 0, consumed: consumed.clone() };
            let reading = Reading::new(Framing::Lines, Vec::new()).unwrap();
            let failure = read_units(&reading, Box::new(source), None, most, none, false).err().unwrap();
            assert!(matches!(failure, Failure::FindCount { none: actual } if actual == none));
            assert_eq!(consumed.load(Ordering::Relaxed), cutoff);
        }
    }
    #[test]
    fn empty_find_input_remains_an_empty_host_set() {
        let reading = Reading::new(Framing::Lines, Vec::new()).unwrap();
        assert!(read_units(&reading, Box::new(io::Cursor::new(Vec::<u8>::new())), None,
            *crate::find_candidate_count_range(false).end(), false, false).unwrap().is_empty());
    }
}
