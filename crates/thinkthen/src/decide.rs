//! The `decide if` flow, from what was asked to what is printed.

use std::io::{Read, Write};
use std::process::ExitCode;
use std::time::Duration;

use thinkthen_core::recording::Exchange as Recorded;
use thinkthen_core::systemone;
use thinkthen_core::{
    Adapter, Assessment, AssessmentStatus, Backend, Condition, DecisionResult, Meta, PassMark,
    Plan, PlanDocument, Policy, Question, Reply, assess, json_line, resolve_backend,
};

use crate::args::IfArguments;
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::{self, Exchange};
use crate::recorder::Recorder;

/// Judge whether the condition holds for the evidence, and say so on one line.
///
/// # Errors
///
/// Returns [`Failure`] for every outcome `specification/channels.md` gives an
/// exit code other than 0, 1, and 3.
pub(crate) fn decide_if(
    arguments: &IfArguments,
    environment: &Environment,
    input: impl Read,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let policy = policy_of(arguments.min_prob)?;
    let backend = resolve_backend(arguments.backend_values(), environment.backend_values())?;
    let condition = Condition::new(arguments.condition.as_str())?;
    let plan = Plan::new(
        edge::evidence(input)?,
        backend.model().clone(),
        vec![Question::new_if(condition.clone())],
    )
    .map_err(|_| Failure::Defect("a plan of one question asks nothing"))?;

    if arguments.plan {
        if arguments.record.is_some() || arguments.replay.is_some() {
            return Err(Failure::PlanWithRecording);
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
    let assessment = assess(answer, policy);
    let result = DecisionResult::new(
        Question::new_if(condition),
        answer,
        assessment,
        Meta::new(
            backend.name().cloned(),
            backend.url().clone(),
            backend.adapter(),
            reply.model().clone(),
            reply.usage(),
            replayed,
        ),
    );
    edge::write_line(writer, &json_line(&result)?)?;
    Ok(exit_code(arguments.status, assessment))
}

/// Take the pass mark the user named, or accept nothing.
fn policy_of(min_prob: Option<f64>) -> Result<Policy, Failure> {
    match min_prob {
        Some(value) => Ok(Policy::Symmetric(PassMark::new(value)?)),
        None => Ok(Policy::Unassessed),
    }
}

/// Answer the plan from the recording folder, or from the backend itself.
///
/// The recording is read before a key is, so a replay opens no connection and
/// needs no key. Only an exchange the adapter read is recorded.
fn ask(
    backend: &Backend,
    plan: &Plan,
    arguments: &IfArguments,
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

/// Turn the assessment into the exit code `--status` asks for.
fn exit_code(status: bool, assessment: Assessment) -> ExitCode {
    if !status {
        return ExitCode::SUCCESS;
    }
    match (assessment.status(), assessment.value()) {
        (AssessmentStatus::Accepted, Some(true)) => ExitCode::from(0),
        (AssessmentStatus::Accepted, Some(false)) => ExitCode::from(1),
        _ => ExitCode::from(3),
    }
}
