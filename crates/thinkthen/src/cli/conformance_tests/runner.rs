use super::{
    Asked, CASES, Document, STAGED, asked, command, expected, find_asked, finding, outcomes,
    record_requests, validate_operation,
};
use crate::core::{Backend, Evidence, ModelName, Url, Value};
use crate::engine::Cancel;
use crate::engine::error::Error as EngineError;
use crate::engine::facade::{Answered, Completed, Engine, Input, RunOutcome, Settings, Storage};
use crate::engine::request::{Injection, inject};
use crate::failure::{Failure, report};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// An engine that answers only from this replay folder and has no key.
fn replaying(backend: &Backend, replay: PathBuf) -> Engine {
    Engine::new(Settings {
        backend: backend.clone(),
        profile: None,
        timeout: Duration::from_secs(1),
        max_retries: 0,
        retry_wait: Duration::ZERO,
        width: None,
        storage: Storage {
            replay: Some(replay),
            ..Storage::default()
        },
        key: || Err(EngineError::NoKey("THINKTHEN_API_KEY")),
        usage: Arc::default(),
    })
    .expect("a replaying engine")
}

/// Answer one exchange through the facade call its verb uses.
fn facade_answer(
    engine: &Engine,
    case: &super::Case,
    request: &Asked,
    evidence: &str,
) -> Result<Answered, EngineError> {
    let cancel = Cancel::default();
    match case.verb.as_str() {
        "find" => Ok(engine
            .find(&finding(case).expect("find"), &cancel)?
            .answered),
        "annotate" => {
            let mut answers = Vec::new();
            engine.ask_chunks(engine.split(&request.plan)?, &cancel, |answered| {
                answers.push(answered);
                Ok::<(), EngineError>(())
            })?;
            let [answered] = <[Answered; 1]>::try_from(answers)
                .map_err(|_| EngineError::Defect("one annotate group is one request"))?;
            Ok(answered)
        }
        _ => Ok(engine
            .judge(
                &request.plan.questions()[0],
                request.thresholds[0],
                Evidence::new(evidence).expect("evidence"),
                &cancel,
            )?
            .answered),
    }
}

#[test]
#[allow(
    clippy::excessive_nesting,
    clippy::too_many_lines,
    reason = "the command runner checks every field of every shared case in one pass"
)]
fn every_case_crosses_the_private_facade_under_replay() {
    let document: Document = serde_json::from_str(CASES).expect("shared document");
    let backend_url = Url::new(&document.backend_url).expect("canonical URL");
    let backend = Backend::from_parts(backend_url, ModelName::new("jev-latest").expect("model"));
    for case in &document.cases {
        if let Some(expected) = &case.expect.error {
            run_fault(case, &expected.kind);
            continue;
        }
        let success = case.expect.success.as_ref().expect("successful case");
        if STAGED.contains(&case.verb.as_str()) {
            command::staged(case, success);
            continue;
        }
        let requests = case
            .exchanges
            .iter()
            .map(|exchange| {
                crate::core::recording::Exchange::new(backend.url(), exchange.request.as_bytes())
                    .digest()
                    .as_str()
                    .to_owned()
            })
            .collect::<Vec<_>>();
        let (_scratch, replay) = command::replay(case);
        let engine = replaying(&backend, replay);
        let mut values = Vec::<Value>::new();
        let mut odds = Vec::new();
        let mut failed_questions = 0;
        for (place, exchange) in case.exchanges.iter().enumerate() {
            let request = if case.verb == "find" {
                find_asked(case).expect("find plan")
            } else {
                asked(case, place, exchange).expect("case plan")
            };
            let answered = facade_answer(&engine, case, &request, &exchange.evidence)
                .unwrap_or_else(|error| panic!("{}: {error:?}", case.id));
            assert!(answered.replayed, "{}", case.id);
            assert_eq!(answered.requests_sent, 0, "{}", case.id);
            assert_eq!(answered.request.as_str(), requests[place], "{}", case.id);
            assert_eq!(
                answered.reply.outcomes().len(),
                request.names.len(),
                "{}",
                case.id
            );
            for (answer_place, outcome) in answered.reply.outcomes().iter().enumerate() {
                let name = &request.names[answer_place];
                let held = expected(success, place, name).expect("expected named answer");
                match outcomes::check(
                    &case.id,
                    name,
                    outcome,
                    held,
                    request.thresholds[answer_place],
                )
                .expect("expected answer and details")
                {
                    Some((value, probability)) => {
                        values.push(value);
                        odds.push(probability);
                    }
                    None => failed_questions += 1,
                }
                assert_eq!(
                    request.digests[answer_place], held.details.question_sha256,
                    "{}",
                    case.id
                );
                assert_eq!(
                    answered.reply.model().as_str(),
                    held.details.model,
                    "{}",
                    case.id
                );
                let actual_requests = if case.verb == "annotate" {
                    record_requests(case, place, &requests).expect("record requests")
                } else {
                    std::slice::from_ref(&requests[place])
                };
                assert_eq!(held.details.requests, actual_requests, "{}", case.id);
            }
        }
        assert_eq!(failed_questions, success.failed_questions, "{}", case.id);
        assert_eq!(
            success.answers.len(),
            values.len() + failed_questions,
            "{}",
            case.id
        );
        validate_operation(case, success, &odds, &values).expect("expected operation output");
        if let Some(counters) = &success.counters {
            command::counters(case, counters);
        }
    }
}

fn run_fault(case: &super::Case, expected: &str) {
    if case.question_form.is_some() {
        command::form(case);
        return;
    }
    let injection = &case.operation.as_ref().expect("fault injection").injection;
    let backend = Backend::resolve(None, None, "jev-latest").expect("backend");
    let engine = replaying(&backend, std::env::temp_dir());
    let outcome = engine
        .records(
            false,
            &Cancel::default(),
            |requests, events| {
                thread::spawn(move || {
                    requests.recv().expect("one injected input");
                    events
                        .send(Input::Item(()))
                        .expect("scheduler receives input");
                });
            },
            &|()| Err::<Completed<()>, _>(injected(injection)),
            |()| Ok(true),
        )
        .expect("scheduler itself remains sound");
    let RunOutcome::Stopped { cause, .. } = outcome else {
        panic!("{} did not stop", case.id);
    };
    assert_eq!(cause.kind().as_str(), expected, "{}", case.id);
    let failure = Failure::Stopped {
        at: 1,
        finished: 0,
        replayed: 0,
        recording: false,
        held: false,
        cause: Box::new(Failure::from(cause)),
    };
    let mut diagnostic = Vec::new();
    let _code = report(&failure, &mut diagnostic);
    assert!(
        !diagnostic.is_empty(),
        "{} crossed the CLI mapping",
        case.id
    );
}

fn injected(name: &str) -> EngineError {
    match name {
        "invalid_arguments" => EngineError::Usage("injected invalid arguments"),
        "response_refusal" => inject(Injection::Backend),
        "recording_read_failure" => inject(Injection::Local),
        "cancel_token" => inject(Injection::Cancelled),
        "expired_deadline" => inject(Injection::Deadline),
        "internal_invariant_failure" => inject(Injection::Defect),
        other => panic!("unknown injection {other}"),
    }
}
