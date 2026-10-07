use super::{
    Asked, CASES, Document, STAGED, asked, command, expected, find_asked, finding, outcomes,
    record_requests, validate_operation,
};
use crate::cli::schedule::ordered::{self, Input, Outcome};
use crate::core::{Backend, DEFAULT_MODEL, Evidence, ModelName, Url, Value};
use crate::engine::Cancel;
use crate::engine::error::Error as EngineError;
use crate::engine::facade::{Answered, Asks, Bound, Engine, Settings, Storage};
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
        per_minute: None,
        storage: Storage {
            replay: Some(replay),
            ..Storage::default()
        },
        key: std::sync::Arc::new(|| Err(EngineError::NoKey("THINKTHEN_API_KEY".to_owned()))),
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
            let mut asks = Asks::default();
            asks.add(engine.backend(), &request.plan)?;
            let mut answers = Vec::new();
            engine.ask_each(&asks, Bound::WHOLE, &cancel, |_, answered| {
                answers.push(answered);
                Ok(())
            })?;
            let first = answers
                .first()
                .ok_or(EngineError::Defect("an annotate group had no answer"))?;
            Ok(Answered {
                attempts: Vec::new(),
                sources: answers
                    .iter()
                    .flat_map(|answered| answered.sources.clone())
                    .collect(),
                observations: answers
                    .iter()
                    .flat_map(|answered| answered.observations.clone())
                    .collect(),
                reply: crate::core::Reply::new(
                    first.reply.model().clone(),
                    answers
                        .iter()
                        .flat_map(|answered| answered.reply.outcomes().iter().cloned())
                        .collect(),
                    None,
                ),
                replayed: answers.iter().all(|answered| answered.replayed),
                request: first.request.clone(),
                requests_sent: answers.iter().map(|answered| answered.requests_sent).sum(),
            })
        }
        _ => Ok(engine
            .judge(
                &request.questions[0],
                request.thresholds[0],
                Evidence::new(evidence).expect("evidence"),
                &cancel,
            )?
            .answered),
    }
}

/// One exchange's replayed answer, or `None` for the miss a failed question
/// makes. The store keeps no failed answer, by ADR 0111 section 6, so replay
/// misses a question the backend failed. The miss is accepted only when this
/// exchange expects a failure; the decoding check reads that failure from the
/// recorded body.
fn replayed(
    engine: &Engine,
    case: &super::Case,
    request: &Asked,
    place: usize,
    exchange: &super::Exchange,
    success: &super::Success,
) -> Option<Answered> {
    match facade_answer(engine, case, request, &exchange.evidence) {
        Ok(answered) => Some(answered),
        Err(EngineError::QuestionMiss(_))
            if success
                .answers
                .iter()
                .any(|answer| answer.exchange == place && answer.details.failure.is_some()) =>
        {
            None
        }
        Err(error) => panic!("{}: {error:?}", case.id),
    }
}

/// The failed and answered counts an exchange expects, for one the replay
/// cannot read whole.
fn unread(success: &super::Success, place: usize) -> (usize, usize) {
    let expected = success
        .answers
        .iter()
        .filter(|answer| answer.exchange == place);
    let failed = expected
        .clone()
        .filter(|answer| answer.details.failure.is_some())
        .count();
    (failed, expected.count() - failed)
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
    let backend = Backend::from_parts(backend_url, ModelName::new(DEFAULT_MODEL).expect("model"));
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
        let metadata_keys = command::metadata_keys(case, backend.url());
        let (_scratch, replay) = command::replay(case);
        let engine = replaying(&backend, replay);
        let mut values = Vec::<Value>::new();
        let mut odds = Vec::new();
        let mut failed_questions = 0;
        let mut unread_answers = 0;
        for (place, exchange) in case.exchanges.iter().enumerate() {
            let request = if case.verb == "find" {
                find_asked(case).expect("find plan")
            } else {
                asked(case, place, exchange).expect("case plan")
            };
            let Some(answered) = replayed(&engine, case, &request, place, exchange, success) else {
                let (failed, answered) = unread(success, place);
                failed_questions += failed;
                unread_answers += answered;
                continue;
            };
            assert!(answered.replayed, "{}", case.id);
            assert_eq!(answered.requests_sent, 0, "{}", case.id);
            // A reply names its first question's key, by ADR 0111 section 2.
            let request_name = metadata_keys[&requests[place]]
                .first()
                .expect("one question");
            assert_eq!(answered.request.as_str(), request_name, "{}", case.id);
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
            values.len() + failed_questions + unread_answers,
            "{}",
            case.id
        );
        // The operation reads every good answer, so it runs only when the
        // replay read them all.
        if unread_answers == 0 {
            validate_operation(case, success, &odds, &values).expect("expected operation output");
        }
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
    let outcome = ordered::run(
        1,
        &Cancel::default(),
        |requests, events| {
            thread::spawn(move || {
                requests.recv().expect("one injected input");
                events
                    .send(Input::Item(()))
                    .expect("the runner receives input");
            });
        },
        &|()| Err::<ordered::Row<()>, _>(injected(injection)),
        |()| Ok(true),
        &|error| error,
        || EngineError::Defect("the reader ended"),
    )
    .expect("the runner itself remains sound");
    let Outcome::Stopped { cause, .. } = outcome else {
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
        "response_refusal" => EngineError::Status(422),
        "recording_read_failure" => EngineError::RecordingStorage,
        "cancel_token" => EngineError::Cancelled,
        "expired_deadline" => EngineError::Deadline(crate::engine::error::Budget(Duration::ZERO)),
        "internal_invariant_failure" => EngineError::Defect("injected invariant failure"),
        other => panic!("unknown injection {other}"),
    }
}
