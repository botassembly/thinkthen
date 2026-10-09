//! The one aggregate request behind the public `find` command.

use std::io::{Read, Write};
use std::process::ExitCode;

use crate::core::{
    Backend, Evidence, Find, Framing, MAX_RECORD_BYTES, PlanDocument, PlanSummary, Pointer,
    Reading, Record, json_line,
};

use crate::args::{Common, FindArguments};
use crate::asking::{self, Folders};
use crate::cli::{
    display::Display,
    intake::{self, Position, Snapshot},
};
use crate::edge::{self, Environment};
use crate::engine::facade;
use crate::failure::{Failure, ReplayContext};
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
        metadata,
        common,
        question,
        profile: saved_profile,
        sources,
    } = question::Prepared::new(arguments, &mut admitted)?;
    let common = &common;
    let context = asking::context::shared(arguments.common.context.as_deref())?;
    let mut display = display(arguments, common)?;
    let reading = Reading::new(framing(common), fields(common)?)?
        .with_item_schema(metadata.item_schema.clone());
    let backend = environment
        .resolve(
            common.backend.as_deref(),
            common.url.as_deref(),
            common.model.as_deref(),
        )?
        .with_request_size(
            environment.request_size(arguments.common.max_request_bytes.as_deref())?,
        );
    let profile = profile::read(common, environment, &backend)?;
    let folders = Folders::of(common, environment)?;
    if common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    let recording = folders.reported();
    let engine = live_engine(
        common,
        environment,
        folders,
        backend.clone(),
        profile.clone(),
    )?;
    let most = if arguments.none { 254 } else { 255 };
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
    if !(2..=most).contains(&units.len()) {
        return Err(Failure::FindCount {
            none: arguments.none,
        });
    }
    if common.dry_run {
        let evidence: Vec<_> = units.iter().map(|unit| unit.evidence.clone()).collect();
        let find = Find::new(question, &evidence, backend.model().clone(), arguments.none)
            .map_err(|_| Failure::Defect("a validated find set could not become a plan"))?
            .with_profile(saved_profile);
        return planned(
            &find,
            (&backend, environment.key_variable()),
            (profile.as_ref(), context.as_deref()),
            (&reading, sources),
            units.len(),
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
    let rendered = result::rendered(common, &reading, &units, &found, &metadata)?;
    display.emit_row(
        &mut writer,
        rendered.line.as_deref(),
        rendered.position.as_ref(),
        rendered.score,
    )?;
    environment.usage().record_done();
    Ok(ExitCode::from(if rendered.resolved { 0 } else { 3 }))
}

fn execute(
    admitted: crate::AdmittedRequest,
    engine: facade::Engine,
    common: &Common,
    environment: &Environment,
    units: &[Unit],
    context: Option<&str>,
) -> Result<crate::CompleteFound<crate::QuestionInput>, Failure> {
    let engine = crate::Engine::from_cli(engine, environment.config().prices());
    let fields = common.field.iter().map(String::as_str).collect::<Vec<_>>();
    let composition = crate::RecordReading::new(&fields, None, None).map_err(Failure::from)?;
    let rows = units.iter().map(|unit| {
        let mut record =
            composition.compose(crate::RawRecord(std::sync::Arc::new(unit.record.clone())))?;
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
    });
    let token = crate::CancelToken::from_flag(environment.cancel().flag());
    let mut controls = crate::CallOptions::new()
        .cli_cancel(&token, environment.cancel().deadline())
        .surface(crate::Surface::Cli)
        .attempts(common.details);
    if let Some(context) = context {
        controls = controls.context(context);
    }
    let admitted = admitted.with_composed_feed("cli-find");
    let outcome = engine
        .execute_request(
            &admitted,
            crate::RequestEnvironment {
                controls,
                feed: Some(crate::RequestFeed::from_records("cli-find", rows)),
            },
        )
        .map_err(|error| {
            if let Some(facts) = error.facts() {
                environment.settle_native(facts);
            }
            Failure::from(error).with_replay_context(ReplayContext::FindSet(units.len()))
        })?;
    let crate::RequestOutcome::Complete(call) = outcome else {
        return Err(Failure::Defect("whole-set find returned a stream failure"));
    };
    environment.settle_native(call.facts());
    let crate::RequestValue::Found(found) = call.into_value() else {
        return Err(Failure::Defect("find returned another native result"));
    };
    Ok(found)
}

fn live_engine(
    common: &Common,
    environment: &Environment,
    folders: Folders,
    backend: Backend,
    profile: Option<crate::core::BackendProfile>,
) -> Result<Option<facade::Engine>, Failure> {
    (!common.dry_run)
        .then(|| {
            crate::cli::construction::engine(
                common,
                environment,
                folders,
                (backend, profile),
                None,
                false,
            )
        })
        .transpose()
}

fn display(arguments: &FindArguments, common: &Common) -> Result<Display, Failure> {
    let mut display = Display::default();
    display.arguments = arguments.display.clone();
    display.validate(common)?;
    Ok(display)
}

/// Print the plan. `target` is the backend and its first key variable.
fn planned(
    find: &Find,
    (backend, key_env): (&Backend, &str),
    (profile, context): (Option<&crate::core::BackendProfile>, Option<&str>),
    (reading, sources): (&Reading, Option<crate::core::Sources>),
    records: usize,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let mut asks = facade::Asks::default();
    asks.add(backend, find.plan())?;
    if let Some(context) = context {
        asks = asks.with_context(backend, context)?;
    }
    let prepared = asks.requests(backend, profile, facade::Bound::WHOLE)?;
    let mut summary = PlanSummary::new(false).with_accounting(backend.accounting());
    summary
        .records_added(records)
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    for request in prepared {
        summary
            .request(&request.body)
            .map_err(|_| Failure::Defect("a plan is too large"))?;
    }
    let mut document = match context {
        Some(_) => PlanDocument::of_body(
            backend,
            summary
                .first_body()
                .ok_or(Failure::Defect("a find preview has no request"))?
                .to_vec(),
        ),
        None => PlanDocument::of(backend, find.plan()),
    }
    .map_err(|_| Failure::Defect("a request could not be written as JSON"))?
    .key_env(key_env);
    if summary.first_body() != Some(document.request_body()) {
        return Err(Failure::Defect(
            "the disclosed request changed after preparation",
        ));
    }
    if let Some(sources) = sources {
        document = document.from(sources);
    }
    edge::write_line(&mut writer, &json_line(&document.reading(reading))?)?;
    let counts = summary
        .counts()
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    edge::write_line(&mut writer, &json_line(&counts)?)?;
    Ok(ExitCode::SUCCESS)
}

fn fields(common: &Common) -> Result<Vec<Pointer>, Failure> {
    common
        .field
        .iter()
        .map(|typed| {
            Pointer::new(typed).map_err(|error| Failure::Pointer("--field", typed.clone(), error))
        })
        .collect()
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
        if original > MAX_RECORD_BYTES {
            return Err(Failure::FindTooLarge);
        }
        let record = reading
            .record(&bytes)
            .map_err(|error| stopped(units.len(), recording, Failure::record(error, true)))?;
        let evidence = reading
            .evidence(&record)
            .map_err(|error| stopped(units.len(), recording, Failure::record(error, true)))?;
        let mut position = item
            .position
            .ok_or(Failure::Defect("find source has no physical position"))?;
        position.located = true;
        units.push(Unit {
            bytes,
            record,
            evidence,
            position,
        });
    }
    Ok(units)
}

struct Unit {
    bytes: Vec<u8>,
    record: Record,
    evidence: Evidence,
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
    let mut chunks = edge::Chunks::new(source, true);
    let mut original = 0usize;
    let mut units = Vec::new();
    while let Some(bytes) = chunks.next().transpose()? {
        if units.len() == most {
            return Err(Failure::FindCount { none });
        }
        original = original
            .checked_add(bytes.len())
            .ok_or(Failure::FindTooLarge)?;
        if original > MAX_RECORD_BYTES {
            return Err(Failure::FindTooLarge);
        }
        let record = reading
            .record(&bytes)
            .map_err(|error| stopped(units.len(), recording, Failure::record(error, true)))?;
        let evidence = reading
            .evidence(&record)
            .map_err(|error| stopped(units.len(), recording, Failure::record(error, true)))?;
        let line = units.len() + 1;
        units.push(Unit {
            bytes,
            record,
            evidence,
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
