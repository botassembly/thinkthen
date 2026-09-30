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
use crate::core::{NAME, PlanSummary, json_line};
use crate::engine::error::Error;
use crate::engine::facade::{Chunk, Engine, Settings, Storage};
use crate::failure::{self, Failure};

/// The check sends only to an address the user named, never the built-in one.
const NO_ADDRESS: &str = "check needs an address you name: give --url or --backend, set THINKTHEN_BASE_URL or THINKTHEN_BACKEND, or set url or backend in the configuration file";

/// The retry count every command defaults to. The check takes no option for it.
const MAX_RETRIES: u32 = 2;

pub(crate) fn run(
    arguments: &CheckArguments,
    environment: &Environment,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    if arguments.retired_dry_run {
        return Err(Failure::Usage("--dry-run was renamed --plan"));
    }
    let choice = environment.choose(arguments.backend.as_deref(), arguments.url.as_deref())?;
    if choice.tier.is_none() {
        return Err(Failure::Usage(NO_ADDRESS));
    }
    // The configuration's model applies only on the unnamed path.
    let unnamed = choice.named.is_none();
    let asked = arguments
        .model
        .as_deref()
        .or_else(|| environment.model().filter(|_| unnamed));
    let backend = environment.settle(&choice, asked)?;
    let roots = environment.roots()?;
    let probes = check::probes(backend.model(), backend.descriptions())
        .ok_or(Failure::Defect("a check probe no longer parses"))?;
    if arguments.dry_run {
        say_dropped_detail(probes.iter().any(|probe| probe.drops_detail))?;
    }
    let engine = Engine::with_roots(
        Settings {
            backend,
            profile: None,
            timeout: Duration::from_secs(arguments.timeout),
            max_retries: MAX_RETRIES,
            retry_wait: environment.retry_wait(),
            per_minute: environment.per_minute,
            width: None,
            storage: Storage::default(),
            key: environment.key_reader(),
            usage: environment.counters(),
        },
        roots,
    )?;
    let mut lines = vec![
        format!("url {}", engine.backend().url().as_str()),
        format!("provider {}", check::PROVIDER),
        format!("model asked {}", asked.unwrap_or("unspecified")),
        format!("model sent {}", engine.backend().model().as_str()),
    ];
    let mut report = Report::new(&probes);
    let mut summary = PlanSummary::new(false);
    for probe in &probes {
        // The one request a probe makes, from the split every command uses.
        let Ok([chunk]) = <[Chunk; 1]>::try_from(engine.split(&probe.plan)?) else {
            return Err(Failure::Defect("a check probe split into several requests"));
        };
        if arguments.dry_run {
            summary
                .record()
                .map_err(|_| Failure::Defect("a plan is too large"))?;
            summary
                .request(&chunk.request.body)
                .map_err(|_| Failure::Defect("a plan is too large"))?;
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
    } else {
        let counts = summary
            .counts()
            .map_err(|_| Failure::Defect("a plan is too large"))?;
        lines.push(json_line(&counts)?);
    }
    for line in lines {
        edge::write_line(&mut writer, &line)?;
    }
    Ok(ExitCode::from(if critical { 4 } else { 0 }))
}

/// Say once, under `--plan`, that the Ollama workaround turned a description
/// object into text (ADR 0115 section 4). The workaround is debt, owned by
/// `sdlc/issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md`.
pub(crate) fn say_dropped_detail(dropped: bool) -> Result<(), Failure> {
    if dropped {
        writeln!(
            std::io::stderr().lock(),
            "thinkthen: {}",
            check::DROPPED_DETAIL
        )
        .map_err(Failure::Output)?;
    }
    Ok(())
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
