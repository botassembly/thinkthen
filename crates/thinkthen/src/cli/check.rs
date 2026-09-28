//! `thinkthen check`: four fixed requests that show whether a backend works here.
//!
//! Each probe goes through the production encoder, engine, and decoder, so a
//! critical finding is what a real run would meet. `core::check` holds the
//! probes and grades the replies. This file resolves, sends, and prints.

use std::io::Write;
use std::process::ExitCode;
use std::time::Duration;

use crate::cli::args::CheckArguments;
use crate::cli::edge::{self, Environment};
use crate::core::check::{self, Probe, Report};
use crate::core::{Backend, DEFAULT_MODEL, NAME};
use crate::engine::error::Error;
use crate::engine::facade::{Chunk, Engine, Settings, Storage};
use crate::failure::{self, Failure};

/// The check sends only to an address the user named, never the built-in one.
const NO_ADDRESS: &str = "check needs an address you name: give --url, set THINKTHEN_BASE_URL, or set url in the configuration file";

/// The retry count every command defaults to. The check takes no option for it.
const MAX_RETRIES: u32 = 2;

pub(crate) fn run(
    arguments: &CheckArguments,
    environment: &Environment,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let url = arguments.url.as_deref().or_else(|| environment.base_url());
    let url = url.ok_or(Failure::Usage(NO_ADDRESS))?;
    let asked = arguments.model.as_deref().or_else(|| environment.model());
    let backend = Backend::resolve(Some(url), None, asked.unwrap_or(DEFAULT_MODEL))?;
    environment.check_key(&backend)?;
    let probes =
        check::probes(backend.model()).ok_or(Failure::Defect("a check probe no longer parses"))?;
    let engine = Engine::new(Settings {
        backend,
        profile: None,
        timeout: Duration::from_secs(arguments.timeout),
        max_retries: MAX_RETRIES,
        retry_wait: environment.retry_wait(),
        width: None,
        storage: Storage::default(),
        key: environment.key_reader(),
        usage: environment.counters(),
    })?;
    let mut lines = vec![
        format!("url {}", engine.backend().url().as_str()),
        format!("provider {}", check::PROVIDER),
        format!("model asked {}", asked.unwrap_or("unspecified")),
        format!("model sent {}", engine.backend().model().as_str()),
    ];
    let mut report = Report::new(&probes);
    for probe in &probes {
        // The one request a probe makes, from the split every command uses.
        let Ok([chunk]) = <[Chunk; 1]>::try_from(engine.split(&probe.plan)?) else {
            return Err(Failure::Defect("a check probe split into several requests"));
        };
        if arguments.dry_run {
            let body = String::from_utf8_lossy(&chunk.request.body);
            lines.push(format!("request {} {body}", probe.name));
        } else if !send(
            &engine,
            environment,
            probe,
            chunk,
            (&mut report, &mut lines),
        )? {
            break;
        }
    }
    if environment.cancel().fired() {
        return Err(Failure::Cancelled);
    }
    let (graded, critical) = report.lines();
    if !arguments.dry_run {
        lines.extend(graded);
    }
    for line in lines {
        edge::write_line(&mut writer, &line)?;
    }
    Ok(ExitCode::from(if critical { 4 } else { 0 }))
}

/// Send one probe, print its decoded reply, and grade what came back.
/// `false` stops the check.
fn send(
    engine: &Engine,
    environment: &Environment,
    probe: &Probe,
    chunk: Chunk,
    (report, lines): (&mut Report, &mut Vec<String>),
) -> Result<bool, Failure> {
    let mut reply = None;
    let sent = engine.ask_chunks(vec![chunk], environment.cancel(), |answered| {
        reply = Some(answered.reply);
        Ok::<(), Error>(())
    });
    let error = match (sent, reply) {
        (Ok(()), Some(reply)) => {
            let line = check::reply_line(probe, &reply);
            lines.push(line.ok_or(Failure::Defect("a check reply did not render"))?);
            report.replied(probe, &reply);
            return Ok(true);
        }
        (Ok(()), None) => return Err(Failure::Defect("a sent check probe had no reply")),
        (Err(error), _) => error,
    };
    let gate = match &error {
        Error::Transport(_) => Some("connection"),
        Error::Status(401..=403) => Some("key"),
        Error::Status(404) => Some("endpoint"),
        Error::Status(_) | Error::TokenLimit | Error::ReplyTooLarge(_) | Error::Reply(_) => None,
        _ => return Err(error.into()),
    };
    let sentence = sentence(&Failure::from(error));
    match gate {
        Some(gate) => report.stopped(probe, gate, sentence),
        None => report.failed(probe, sentence),
    }
    Ok(gate.is_none())
}

/// The sentence every command prints for this failure, without its prefix.
fn sentence(failure: &Failure) -> String {
    let mut said = Vec::new();
    let _code = failure::report(failure, &mut said);
    let said = String::from_utf8_lossy(&said);
    let said = said.trim_end();
    said.strip_prefix(&format!("{NAME}: "))
        .unwrap_or(said)
        .to_owned()
}
