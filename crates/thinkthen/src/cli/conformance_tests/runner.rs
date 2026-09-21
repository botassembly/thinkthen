use super::{CASES, Document, asked, expected, find_asked, outcomes, validate_operation};
use crate::core::{Backend, ModelName, Url, Value};
use crate::engine::error::Error as EngineError;
use crate::engine::http::Key;
use crate::engine::recorder::Recorder;
use crate::engine::request::{Injection, ask_with, inject};
use crate::engine::schedule::{self, Completed, Input, Outcome};
use crate::failure::{Failure, report};
use std::thread;

#[test]
#[allow(
    clippy::excessive_nesting,
    clippy::too_many_lines,
    reason = "the command runner checks every field of every shared case in one pass"
)]
fn command_runner_crosses_the_private_engine_for_every_case() {
    let document: Document = serde_json::from_str(CASES).expect("shared document");
    let backend_url = Url::new(&document.backend_url).expect("canonical URL");
    let backend = Backend::from_parts(backend_url, ModelName::new("jev-latest").expect("model"));
    let recorder = Recorder::of(None, None).expect("no folders");
    for case in &document.cases {
        if let Some(expected) = &case.expect.error {
            run_fault(case, &expected.kind);
            continue;
        }
        let success = case.expect.success.as_ref().expect("successful case");
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
        let mut values = Vec::<Value>::new();
        let mut odds = Vec::new();
        let mut failed_questions = 0;
        for (place, exchange) in case.exchanges.iter().enumerate() {
            let request = if case.verb == "find" {
                find_asked(case).expect("find plan")
            } else {
                asked(case, place, exchange).expect("case plan")
            };
            let answered = ask_with::<EngineError>(
                &backend,
                &request.plan,
                &recorder,
                || Ok(Key::of("offline")),
                |prepared, _| {
                    assert_eq!(prepared.body, exchange.request.as_bytes(), "{}", case.id);
                    Ok(exchange.response.get().as_bytes().to_vec())
                },
            )
            .expect("embedded exchange");
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
                    requests.as_slice()
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
    }
}

fn run_fault(case: &super::Case, expected: &str) {
    let injection = &case.operation.as_ref().expect("fault injection").injection;
    let outcome = schedule::run(
        1,
        false,
        |requests, events| {
            thread::spawn(move || {
                requests.recv().expect("one injected input");
                events
                    .send(Input::Item(()))
                    .expect("scheduler receives input");
            });
        },
        &|()| Err::<Completed<()>, _>(injected(injection)),
        |_| Ok(true),
        EngineError::Defect,
    )
    .expect("scheduler itself remains sound");
    let Outcome::Stopped { cause, .. } = outcome else {
        panic!("{} did not stop", case.id);
    };
    assert_eq!(cause.kind().as_str(), expected, "{}", case.id);
    let failure = Failure::Stopped {
        at: 1,
        finished: 0,
        replayed: 0,
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
