//! The one aggregate request behind the public `find` command.

use std::io::{Read, Write};
use std::process::ExitCode;

use crate::core::{
    Backend, Evidence, Find, Framing, MAX_RECORD_BYTES, Meta, PlanDocument, Pointer, QuestionText,
    Reading, Record, RequestMeta, json_line,
};

use crate::args::{Common, FindArguments};
use crate::asking::{self, Folders};
use crate::edge::{self, Environment};
use crate::engine::facade::{self, Found};
use crate::failure::{Failure, ReplayContext};
use crate::profile;

/// Read one bounded set, ask once, and print its selected original unit.
pub(crate) fn run(
    arguments: &FindArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let common = &arguments.common.as_common();
    let framing = if common.jsonl {
        Framing::Jsonl
    } else {
        Framing::Lines
    };
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
    let backend = Backend::resolve(
        common.url.as_deref(),
        environment.base_url(),
        common
            .model
            .as_deref()
            .or_else(|| environment.model())
            .unwrap_or(crate::core::DEFAULT_MODEL),
    )?;
    environment.check_key(&backend)?;
    let profile = profile::read(common)?;
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
    let units = read_units(
        &reading,
        edge::source(common.input.as_deref(), input)?,
        most,
        arguments.none,
        recording,
    )?;
    if units.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    if !(2..=most).contains(&units.len()) {
        return Err(Failure::FindCount {
            none: arguments.none,
        });
    }
    let evidence: Vec<_> = units
        .iter()
        .map(|(_, _, evidence)| evidence.clone())
        .collect();
    let find = Find::new(question, &evidence, backend.model().clone(), arguments.none)
        .map_err(|_| Failure::Defect("a validated find set could not become a plan"))?;
    if common.dry_run {
        let _prepared = facade::split(&backend, profile.as_ref(), find.plan())?;
        let document = PlanDocument::of(&backend, find.plan())
            .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
        let document = document.reading(&reading);
        edge::write_line(writer, &json_line(&document)?)?;
        return Ok(ExitCode::SUCCESS);
    }
    let engine = engine.ok_or(Failure::Defect("a live find run has no engine"))?;
    let found = engine.find(&find, environment.cancel()).map_err(|error| {
        Failure::from(error).with_replay_context(ReplayContext::FindSet(units.len()))
    })?;
    let (line, resolved) = rendered(common, &find, &backend, &reading, &units, found)?;
    if let Some(line) = line {
        edge::write_line(writer, &line)?;
    }
    environment.usage().record_done();
    Ok(if resolved {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(3)
    })
}

type Unit = (Vec<u8>, Record, Evidence);

fn read_units(
    reading: &Reading,
    source: Box<dyn std::io::BufRead + Send>,
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
        units.push((bytes, record, evidence));
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
) -> Result<(Option<String>, bool), Failure> {
    let Found {
        selection: selected,
        answered,
    } = found;
    let reply = &answered.reply;
    let place = selected.selected();
    let line = if common.details {
        let value = place
            .map(|place| {
                units
                    .get(place)
                    .map(|unit| unit.1.clone())
                    .ok_or(Failure::Defect("a find selection is outside its units"))
            })
            .transpose()?;
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
        place
            .map(|place| {
                let unit = units
                    .get(place)
                    .ok_or(Failure::Defect("a find selection is outside its units"))?;
                reading
                    .as_it_arrived(&unit.0)
                    .map(str::to_owned)
                    .map_err(|error| Failure::record(error, true))
            })
            .transpose()?
    };
    Ok((line, place.is_some()))
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
