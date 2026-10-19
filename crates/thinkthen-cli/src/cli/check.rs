//! Backend probes use the same public native preparation and calls as consumers.
use crate::cli::args::{CheckArguments, DEFAULT_MAX_RETRIES};
use crate::cli::edge::{self, Environment};
use crate::failure::{self, Failure};
use crate::probes::{self, Probe, Report};
use crate::render::json_line;
use crate::{Engine, PlanEstimate};
use std::{io::Write, process::ExitCode, time::Duration};
mod functions;

const NO_ADDRESS: &str = "check needs an address you name: give --url or --backend, set THINKTHEN_BASE_URL or THINKTHEN_BACKEND, or set url or backend in the configuration file";

pub(crate) fn run(
    arguments: &CheckArguments,
    environment: &Environment,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    if arguments.retired_dry_run {
        return Err(Failure::Usage("--dry-run was renamed --plan"));
    }
    let route = environment.resolve_for_check(
        arguments.backend.as_deref(),
        arguments.url.as_deref(),
        arguments.model.as_deref(),
        NO_ADDRESS,
    )?;
    let asked = arguments
        .model
        .as_deref()
        .or_else(|| (route.model_source == "configuration").then_some(route.model.as_str()));
    environment.roots()?;
    let engine = environment
        .take_builder()?
        .no_cache()
        .timeout(Duration::from_secs(arguments.timeout))?
        .max_retries(DEFAULT_MAX_RETRIES)
        .retry_wait(environment.retry_wait())
        .build()?;
    environment.observe_engine(&engine);
    // Native shared-state preparation checks every rich probe's unsplit profile
    // ceiling before the minimal function plans or any output/send.
    let probes = probes::probes(&engine, environment.named())?;
    let (function_plans, function_requests) = functions::prepare(&engine)?;
    if arguments.dry_run {
        say_dropped_detail(
            probes.iter().any(|probe| probe.drops_detail),
            environment.named(),
        )?;
    }
    let mut lines = vec![
        format!("url {}", route.url),
        format!("provider {}", engine.provider_name()),
        format!("model asked {}", asked.unwrap_or("unspecified")),
        format!("model sent {}", route.model),
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
        let mut counts = crate::cli::plan_counts::Counts::from(&summary);
        // These four fixed probes each send exactly one shared-state request.
        // The native count is a general occurrence bound, not this closed count.
        counts.records = probes.len();
        counts.requests = probes.len();
        counts.upper_bound = false;
        lines.push(format!("rich-probes {}", json_line(&counts)?));
        lines.extend(function_plans);
        lines.push(summary.token_estimate_note().to_owned());
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

fn inspect(
    engine: &Engine,
    environment: &Environment,
    probes: &[Probe],
    dry_run: bool,
    report: &mut Report,
    lines: &mut Vec<String>,
) -> Result<(bool, PlanEstimate), Failure> {
    let mut stopped = false;
    let mut summary: Option<PlanEstimate> = None;
    for probe in probes {
        summary = Some(match summary {
            Some(previous) => previous.combine(&probe.plan)?,
            None => probe.plan.clone(),
        });
        if dry_run {
            let body = probe
                .plan
                .first_body()
                .ok_or(Failure::Defect("a check probe has no prepared request"))?;
            lines.push(format!(
                "request {} {}",
                probe.name,
                String::from_utf8_lossy(body)
            ));
        } else if !send(engine, environment, probe, report, lines)? {
            stopped = true;
            break;
        }
    }
    Ok((
        stopped,
        summary.ok_or(Failure::Defect("a backend check has no probes"))?,
    ))
}

/// Emit the existing adapter detail-loss warning once at command preview time.
pub(crate) fn say_dropped_detail(dropped: bool, name: Option<&str>) -> Result<(), Failure> {
    if dropped {
        writeln!(
            std::io::stderr().lock(),
            "thinkthen: {}",
            probes::detail_warning(name)
        )
        .map_err(Failure::Output)?;
    }
    Ok(())
}

fn send(
    engine: &Engine,
    environment: &Environment,
    probe: &Probe,
    report: &mut Report,
    lines: &mut Vec<String>,
) -> Result<bool, Failure> {
    let interrupted = || environment.cancel().fired();
    let request = crate::Request::named("annotate", probe.arguments.clone())?.admit()?;
    let tally = environment.tally().start();
    let result = engine.execute_request(
        &request,
        crate::RequestEnvironment {
            controls: crate::CallOptions::new()
                .surface(crate::Surface::Cli)
                .interrupt(&interrupted)
                .shared_state_groups()
                .batch(crate::BatchSetting::Max),
            feed: None,
        },
    );
    let error = match result {
        Ok(crate::RequestOutcome::Complete(call)) => {
            environment.settle_native(call.facts());
            tally.finish(call.facts())?;
            let crate::RequestValue::Annotations(rows) = call.value() else {
                return Err(Failure::Defect("a check returned another function"));
            };
            let [row] = rows.as_slice() else {
                return Err(Failure::Defect("a check returned another record count"));
            };
            lines.push(
                probes::reply_line(probe, row.result())
                    .ok_or(Failure::Defect("a check reply did not render"))?,
            );
            report.replied(probe, row.result());
            return Ok(true);
        }
        Ok(crate::RequestOutcome::Failed { error, .. }) | Err(error) => error,
    };
    if let Some(facts) = error.facts() {
        environment.settle_native(facts);
        tally.finish(facts)?;
    }
    let cause = match error.diagnostic() {
        Some(crate::ErrorDiagnostic::PartialReply {
            cause: Some(sentence),
            ..
        }) => Failure::Reply(sentence),
        Some(crate::ErrorDiagnostic::PartialReply { .. }) => {
            Failure::Reply(error.detail().message().to_owned())
        }
        _ => Failure::from(error),
    };
    let Some((gate, can_continue)) = classify(&cause) else {
        return Err(cause);
    };
    let (_, sentence) = failure::sentence(&cause.with_size_advice(failure::SizeAdvice::Backend));
    if let Some(gate) = gate {
        report.stopped(probe, gate, sentence);
    } else {
        report.failed(probe, sentence);
    }
    Ok(gate.is_none() && can_continue)
}
fn classify(cause: &Failure) -> Option<(Option<&'static str>, bool)> {
    Some(match cause {
        Failure::Provider { cause, diagnostic } => {
            let (gate, next) = classify(cause)?;
            (gate, next && !diagnostic.quota)
        }
        Failure::Transport(_) => (Some("connection"), true),
        Failure::Status(401..=403, _) => (Some("key"), true),
        Failure::Status(404, _) => (Some("endpoint"), true),
        Failure::Status(..)
        | Failure::TokenLimit
        | Failure::ReplyTooLarge(_)
        | Failure::Reply(_) => (None, true),
        _ => return None,
    })
}
