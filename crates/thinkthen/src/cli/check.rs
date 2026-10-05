//! `thinkthen check`: rich probes and minimal production function calls.
//!
//! Each probe goes through the production encoder, engine, and decoder, so a
//! critical finding is what a real run would meet. `core::check` holds the
//! probes and grades the replies. This file resolves, sends, and prints.

use std::io::Write;
use std::process::ExitCode;
use std::time::Duration;

use crate::cli::args::{CheckArguments, DEFAULT_MAX_RETRIES};
use crate::cli::edge::{self, Environment};
use crate::core::adapters::built_in;
use crate::core::check::{self, Probe, Report};
use crate::core::{NAME, PlanSummary, json_line};
use crate::engine::error::Error;
use crate::engine::facade::{Engine, Settings, Storage};
use crate::failure::{self, Failure};

mod functions;

/// The check sends only to an address the user named, never the built-in one.
const NO_ADDRESS: &str = "check needs an address you name: give --url or --backend, set THINKTHEN_BASE_URL or THINKTHEN_BACKEND, or set url or backend in the configuration file";

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
    let probes = check::probes(backend.model(), backend.descriptions(), environment.named())
        .ok_or(Failure::Defect("a check probe no longer parses"))?;
    let profile = choice.setup(&backend).1;
    let engine = Engine::with_roots(
        Settings {
            backend,
            profile,
            timeout: Duration::from_secs(arguments.timeout),
            max_retries: DEFAULT_MAX_RETRIES,
            retry_wait: environment.retry_wait(),
            per_minute: environment.per_minute,
            width: None,
            storage: Storage::default(),
            key: environment.key_reader(),
            usage: environment.counters(),
        },
        roots,
    )?
    // The estimated input cap binds the probes as it binds every live request;
    // `check` has no flag for it, so only the variable sets it.
    .with_process_budget(None, environment.estimated_total);
    for probe in &probes {
        engine.check_plan(&probe.plan)?;
    }
    let (function_plans, function_requests) = functions::prepare(&engine)?;
    if arguments.dry_run {
        say_dropped_detail(
            probes.iter().any(|probe| probe.drops_detail),
            environment.named(),
        )?;
    }
    let mut lines = vec![
        format!("url {}", engine.backend().url().as_str()),
        format!("provider {}", check::PROVIDER),
        format!("model asked {}", asked.unwrap_or("unspecified")),
        format!("model sent {}", engine.backend().model().as_str()),
    ];
    let function_rows = functions::ROWS.map(|(_, row)| row);
    let mut report = Report::new(&probes, &function_rows);
    let (stopped, summary) = inspect(
        &engine,
        environment,
        &probes,
        arguments.dry_run,
        &mut report,
        &mut lines,
    )?;
    if !arguments.dry_run && !stopped {
        functions::check(&engine, environment, &mut report)?;
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
        lines.push(format!("rich-probes {}", json_line(&counts)?));
        lines.extend(function_plans);
        lines.push(format!(
            "prepared-requests upper-bound {} before retries and refusal splits",
            probes.len() + function_requests
        ));
    }
    for line in lines {
        edge::write_line(&mut writer, &line)?;
    }
    Ok(ExitCode::from(if critical { 4 } else { 0 }))
}

/// Prepare rich probe bodies or send and grade their replies.
fn inspect(
    engine: &Engine,
    environment: &Environment,
    probes: &[Probe],
    dry_run: bool,
    report: &mut Report,
    lines: &mut Vec<String>,
) -> Result<(bool, PlanSummary), Failure> {
    let mut stopped = false;
    let mut summary = PlanSummary::new(false);
    for probe in probes {
        if dry_run {
            // The one request a probe makes, from the encoder every command uses.
            let body = built_in::encode(&probe.plan)
                .map_err(|_| Failure::Defect("a check probe could not be written as JSON"))?;
            summary
                .record()
                .map_err(|_| Failure::Defect("a plan is too large"))?;
            summary
                .request(&body)
                .map_err(|_| Failure::Defect("a plan is too large"))?;
            let body = String::from_utf8_lossy(&body);
            lines.push(format!("request {} {body}", probe.name));
        } else if !send(engine, environment, probe, (report, lines))? {
            stopped = true;
            break;
        }
    }
    Ok((stopped, summary))
}

/// Say once, under `--plan`, that a runtime workaround turned a description
/// object into text (ADR 0115 section 4). The workaround is debt; the
/// adapter's backend table names its issue.
pub(crate) fn say_dropped_detail(dropped: bool, name: Option<&str>) -> Result<(), Failure> {
    if dropped {
        writeln!(
            std::io::stderr().lock(),
            "thinkthen: {}",
            built_in::backends::dropped_detail(name)
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
    (report, lines): (&mut Report, &mut Vec<String>),
) -> Result<bool, Failure> {
    let error = match engine.send_plan(&probe.plan, environment.cancel()) {
        Ok(reply) => {
            let line = check::reply_line(probe, &reply);
            lines.push(line.ok_or(Failure::Defect("a check reply did not render"))?);
            report.replied(probe, &reply);
            return Ok(true);
        }
        Err(error) => error,
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
