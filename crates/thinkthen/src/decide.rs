//! The `decide` flow, from what was asked to what is printed.

use std::io::{Read, Write};
use std::process::ExitCode;
use std::time::Duration;

use thinkthen_core::recording::Exchange as Recorded;
use thinkthen_core::systemone;
use thinkthen_core::{
    Adapter, Backend, DecisionResult, Meta, Outcome, Plan, PlanDocument, Question, QuestionText,
    Reply, Threshold, json_line, resolve_backend,
};

use crate::args::DecideArguments;
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::{self, Exchange};
use crate::recorder::Recorder;

/// Answer the question about the evidence, and set the exit code from the answer.
///
/// # Errors
///
/// Returns [`Failure`] for every outcome `specification/channels.md` gives an
/// exit code other than 0, 1, and 3.
pub(crate) fn decide(
    arguments: &DecideArguments,
    environment: &Environment,
    input: impl Read,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    if arguments.quiet && arguments.details {
        return Err(Failure::QuietWithDetails);
    }
    let threshold = threshold_of(arguments.threshold.as_deref())?;
    let backend = resolve_backend(arguments.backend_values().with_base(environment.base_url()))?;
    let text = QuestionText::new(arguments.question.as_str())?;
    let plan = Plan::new(
        edge::evidence(input)?,
        backend.model().clone(),
        vec![Question::new_decide(text.clone())],
    )
    .map_err(|_| Failure::Defect("a plan of one question asks nothing"))?;

    if arguments.dry_run {
        if arguments.record.is_some() || arguments.replay.is_some() {
            return Err(Failure::DryRunWithRecording);
        }
        let document = PlanDocument::of(&backend, &plan)
            .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
        edge::write_line(writer, &json_line(&document)?)?;
        return Ok(ExitCode::SUCCESS);
    }

    let recorder = Recorder::of(arguments.record.as_deref(), arguments.replay.as_deref())?;
    let (reply, replayed) = ask(&backend, &plan, arguments, environment, &recorder)?;
    let answer = *reply
        .answers()
        .first()
        .ok_or(Failure::Defect("the adapter answered no question"))?;
    let outcome = threshold.judge(answer);
    if arguments.details {
        let result = DecisionResult::new(
            outcome,
            Question::new_decide(text),
            answer,
            threshold,
            Meta::new(
                backend.profile().cloned(),
                backend.url().clone(),
                backend.adapter(),
                reply.model().clone(),
                reply.usage(),
                replayed,
            ),
        );
        edge::write_line(&mut writer, &json_line(&result)?)?;
    } else if !arguments.quiet {
        edge::write_line(&mut writer, &json_line(&outcome.value())?)?;
    }
    Ok(exit_code(outcome))
}

/// Take the rule the user named, or the cut of one half that stands for none.
fn threshold_of(given: Option<&str>) -> Result<Threshold, Failure> {
    match given {
        Some(text) => Ok(text.parse()?),
        None => Ok(Threshold::default()),
    }
}

/// Answer the plan from the recording folder, or from the backend itself.
///
/// The recording is read before a key is, so a replay opens no connection and
/// needs no key. Only an exchange the adapter read is recorded.
fn ask(
    backend: &Backend,
    plan: &Plan,
    arguments: &DecideArguments,
    environment: &Environment,
    recorder: &Recorder,
) -> Result<(Reply, bool), Failure> {
    let body = match backend.adapter() {
        Adapter::SystemOne => systemone::encode(plan),
    }
    .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
    let recorded = Recorded::new(backend.adapter(), backend.url(), &body);
    if let Some(response) = recorder.replayed(&recorded)? {
        return Ok((read(backend, plan, &response)?, true));
    }
    let key = backend.key_env().map(edge::key).transpose()?;
    let answered = http::post(&Exchange {
        url: backend.url().as_str(),
        body: &body,
        key: key.as_ref(),
        timeout: Duration::from_secs(arguments.timeout),
        max_retries: arguments.max_retries,
        retry_wait: environment.retry_wait(),
    })?;
    let reply = read(backend, plan, &answered)?;
    recorder.record(&recorded, &answered)?;
    Ok((reply, false))
}

/// Read one response body in the language the backend speaks.
fn read(backend: &Backend, plan: &Plan, body: &[u8]) -> Result<Reply, Failure> {
    match backend.adapter() {
        Adapter::SystemOne => Ok(systemone::decode(plan, body)?),
    }
}

/// Turn the outcome into the exit code `specification/channels.md` fixes.
fn exit_code(outcome: Outcome) -> ExitCode {
    match outcome {
        Outcome::Yes => ExitCode::from(0),
        Outcome::No => ExitCode::from(1),
        Outcome::Unresolved => ExitCode::from(3),
    }
}
