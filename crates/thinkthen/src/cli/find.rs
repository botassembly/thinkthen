//! The one aggregate request behind the public `find` command.

use std::io::{Read, Write};
use std::process::ExitCode;

use crate::core::{
    Backend, Evidence, Find, Framing, MAX_RECORD_BYTES, Meta, PlanDocument, PlanSummary, Pointer,
    QuestionText, Reading, Record, RequestMeta, json_line,
};

use crate::args::{Common, FindArguments};
use crate::asking::{self, Folders};
use crate::cli::{
    display::Display,
    intake::{self, Position, Snapshot},
};
use crate::edge::{self, Environment};
use crate::engine::facade::{self, Found};
use crate::failure::{Failure, ReplayContext};
use crate::profile;

/// Read one bounded set, ask once, and print its selected original unit.
pub(crate) fn run(
    arguments: &FindArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let common = &arguments.common.as_common();
    common.check_plan_name()?;
    let mut display = Display::default();
    display.arguments = arguments.display.clone();
    display.validate(common)?;
    let framing = framing(common);
    let fields = common
        .field
        .iter()
        .map(|typed| {
            Pointer::new(typed).map_err(|error| Failure::Pointer("--field", typed.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let reading = Reading::new(framing, fields)?;
    let question = QuestionText::new(&arguments.question)
        .map_err(|_| Failure::Usage("`find` takes a question that is text, not white space"))?;
    let backend = environment.resolve(
        common.backend.as_deref(),
        common.url.as_deref(),
        common.model.as_deref(),
    )?;
    let profile = profile::read(common, environment, &backend)?;
    let folders = Folders::of(common, environment)?;
    if common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    let recording = folders.reported();
    let engine = (!common.dry_run)
        .then(|| {
            asking::engine(
                common,
                environment,
                folders,
                backend.clone(),
                profile.clone(),
                None,
            )
        })
        .transpose()?;
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
    let evidence: Vec<_> = units.iter().map(|unit| unit.evidence.clone()).collect();
    let find = Find::new(question, &evidence, backend.model().clone(), arguments.none)
        .map_err(|_| Failure::Defect("a validated find set could not become a plan"))?;
    if common.dry_run {
        return planned(
            &find,
            (&backend, environment.key_variable()),
            profile.as_ref(),
            &reading,
            units.len(),
            writer,
        );
    }
    let engine = engine.ok_or(Failure::Defect("a live find run has no engine"))?;
    let found = engine.find(&find, environment.cancel()).map_err(|error| {
        Failure::from(error).with_replay_context(ReplayContext::FindSet(units.len()))
    })?;
    let rendered = rendered(common, &find, &backend, &reading, &units, found)?;
    display.emit_row(
        &mut writer,
        rendered.line.as_deref(),
        rendered.position.as_ref(),
        rendered.score,
    )?;
    environment.usage().record_done();
    Ok(if rendered.resolved {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(3)
    })
}

/// Print the plan. `target` is the backend and its first key variable.
fn planned(
    find: &Find,
    (backend, key_env): (&Backend, &str),
    profile: Option<&crate::core::BackendProfile>,
    reading: &Reading,
    records: usize,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let mut asks = facade::Asks::default();
    asks.add(backend, find.plan())?;
    let prepared = asks.requests(backend, profile, facade::Bound::WHOLE)?;
    let mut summary = PlanSummary::new(false);
    summary
        .records_added(records)
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    for request in prepared {
        summary
            .request(&request.body)
            .map_err(|_| Failure::Defect("a plan is too large"))?;
    }
    let document = PlanDocument::of(backend, find.plan())
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?
        .key_env(key_env);
    if summary.first_body() != Some(document.request_body()) {
        return Err(Failure::Defect(
            "the disclosed request changed after preparation",
        ));
    }
    edge::write_line(&mut writer, &json_line(&document.reading(reading))?)?;
    let counts = summary
        .counts()
        .map_err(|_| Failure::Defect("a plan is too large"))?;
    edge::write_line(&mut writer, &json_line(&counts)?)?;
    Ok(ExitCode::SUCCESS)
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

fn rendered(
    common: &Common,
    find: &Find,
    backend: &Backend,
    reading: &Reading,
    units: &[Unit],
    found: Found,
) -> Result<Rendered, Failure> {
    let Found {
        selection: selected,
        answered,
    } = found;
    let reply = &answered.reply;
    let place = selected.selected();
    let unit = place
        .map(|place| {
            units
                .get(place)
                .ok_or(Failure::Defect("a find selection is outside its units"))
        })
        .transpose()?;
    let position = unit.map(|unit| unit.position.clone());
    let score = place
        .map(|place| {
            selected
                .probabilities()
                .get(place)
                .map(|(_, probability)| *probability)
                .ok_or(Failure::Defect("a find selection carries no probability"))
        })
        .transpose()?;
    let mut line = if common.details {
        let value = unit.map(|unit| unit.record.clone());
        let meta = Meta::new(
            env!("CARGO_PKG_VERSION"),
            find.question_sha256()
                .map_err(|_| Failure::Defect("a find question could not be digested"))?,
            backend.url().clone(),
            reply.model().clone(),
            reply.usage(),
            RequestMeta::new(
                answered.replayed,
                answered.requests_sent,
                vec![answered.request.as_str().to_owned()],
            ),
        );
        Some(json_line(&find.result(value, selected, meta))?)
    } else {
        unit.map(|unit| {
            reading
                .as_it_arrived(&unit.bytes)
                .map(str::to_owned)
                .map_err(|error| Failure::record(error, true))
        })
        .transpose()?
    };
    if common.details {
        intake::locate(&mut line, position.as_ref())?;
        intake::source_members(&mut line, position.as_ref())?;
    } else if let (Some(unit), Some(position), Some(line)) =
        (unit, position.as_ref().filter(|p| p.located), &mut line)
    {
        *line = intake::source_value(&unit.record, &json_line(&unit.record)?, position)?;
    }
    Ok(Rendered {
        line,
        resolved: place.is_some(),
        position,
        score,
    })
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
