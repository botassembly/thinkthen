#![cfg(test)]
//! Shared SQL statement policy is proved without a database's argument registration.
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::time::{Duration, Instant};
use thinkthen::{AnyRow, BackendName, Error, Key, Request, Settings};
use thinkthen_sql::{SqlCall, SqlEvent, SqlRecord};

fn engine(server: &Loopback, backend: BackendName) -> thinkthen::Engine {
    Settings::default()
        .with_backend(backend)
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .batch(1_u64.try_into().unwrap())
        .retry(thinkthen::Retry::new(0, Duration::ZERO, Duration::from_secs(5)).unwrap())
        .engine(|_| None)
        .unwrap()
}
fn request(question: Value) -> Request {
    Request::from_json(&json!({"question":question}).to_string()).unwrap()
}
fn form() -> Value {
    json!({"function":"annotate","questions":{
        "refund":{"function":"decide","question":"Refund?","threshold":{"low":0.2,"high":0.8}},
        "topic":{"function":"choose","question":"Topic?","choices":["billing","shipping","other"]},
        "urgency":{"function":"score","question":"Urgent?","levels":["routine","urgent"]}
    }})
}
fn drain(
    call: &mut SqlCall,
    records: Vec<SqlRecord>,
    postgres: bool,
    mut inspect: impl FnMut(&mut SqlCall, &AnyRow),
) -> Vec<AnyRow> {
    let mut records = Some(records);
    let mut rows = Vec::new();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        assert!(Instant::now() < deadline, "SQL call did not complete");
        let event = if postgres {
            call.poll_postgres(Duration::from_millis(20), || false)
        } else {
            call.poll(Duration::from_millis(20), || false)
        }
        .unwrap();
        match event {
            Some(SqlEvent::NeedsChunk) => match records.take() {
                Some(records) => call.send_chunk(records).unwrap(),
                None => call.end_input(),
            },
            Some(SqlEvent::Row(row)) => {
                inspect(call, &row);
                rows.push(row);
            }
            Some(SqlEvent::Finished) => break,
            None => {}
        }
    }
    rows
}
fn summary(error: &Error, count: usize, total: usize, position: usize) {
    assert!(
        error.to_string().starts_with(&format!(
            "{count} of {total} records failed; the first, at position {position}, failed with "
        )),
        "{error}"
    );
}

#[test]
fn failed_rows_remain_rows_even_when_every_service_request_fails() {
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        for raise in [false, true] {
            let server = Loopback::guide_scenario("failed").unwrap();
            let client = engine(&server, backend);
            let mut call = SqlCall::open(
                &client,
                &request(json!({"function":"decide","question":"Ready?"})),
            )
            .unwrap();
            call.set_raise_if_failed(raise);
            let rows = drain(
                &mut call,
                vec![SqlRecord::Text("one".into()), SqlRecord::Text("two".into())],
                false,
                |_, _| {},
            );
            assert_eq!(rows.len(), 2);
            assert!(
                rows.iter()
                    .all(|row| row.error().unwrap().code().as_str() == "backend.status")
            );
            assert_eq!(server.count(), 2);
            if raise {
                summary(call.final_error().unwrap(), 2, 2, 1);
            } else {
                assert!(call.final_error().is_none());
            }
        }
    }
}

#[test]
fn each_failed_form_entry_counts_the_record_once_and_unsure_never_raises() {
    for backend in [BackendName::SystemOne, BackendName::OpenAi] {
        for scenario in [
            "refund-failed",
            "topic-failed",
            "urgency-failed",
            "failed",
            "refund-unsure",
            "topic-unsure",
            "yes",
        ] {
            let server = Loopback::guide_scenario(scenario).unwrap();
            let client = engine(&server, backend);
            let mut call = SqlCall::open(&client, &request(form())).unwrap();
            call.set_raise_if_failed(true);
            let rows = drain(
                &mut call,
                vec![SqlRecord::Text("message".into())],
                false,
                |_, _| {},
            );
            assert_eq!(rows.len(), 1);
            let row = serde_json::to_value(&rows[0]).unwrap();
            assert_eq!(row["status"], "answered");
            if scenario.ends_with("failed") {
                let name = if scenario == "failed" {
                    "refund"
                } else {
                    scenario.strip_suffix("-failed").unwrap()
                };
                assert_eq!(row["answer"][name]["status"], "failed");
                assert!(row["answer"][name].get("answer").is_none());
                let first = rows[0].failure().unwrap();
                assert_eq!(call.final_error().unwrap().code(), first.code());
                summary(call.final_error().unwrap(), 1, 1, 1);
            } else {
                assert!(rows[0].failure().is_none());
                assert!(call.final_error().is_none());
            }
        }
    }
}

#[test]
fn filter_always_reports_failures_and_a_failed_outer_form_has_its_own_error() {
    let server = Loopback::guide_scenario("yes").unwrap();
    let client = engine(&server, BackendName::SystemOne);
    for question in [json!({"function":"filter","question":"Ready?"}), form()] {
        let is_form = question["function"] == "annotate";
        let mut call = SqlCall::open(&client, &request(question)).unwrap();
        call.set_raise_if_failed(is_form);
        let rows = drain(
            &mut call,
            vec![
                SqlRecord::Text("one".into()),
                SqlRecord::Missing,
                SqlRecord::Text("three".into()),
            ],
            false,
            |_, _| {},
        );
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1].error().unwrap().code().as_str(), "record.missing");
        let error = call.final_error().unwrap();
        assert_eq!(error.code().as_str(), "record.missing");
        summary(error, 1, 3, 2);
    }
}

#[test]
fn rank_top_counts_all_inputs_and_first_failure_uses_input_order() {
    let server = Loopback::guide_scenario("yes").unwrap();
    let client = engine(&server, BackendName::SystemOne);
    let mut call = SqlCall::open(
        &client,
        &request(json!({"function":"rank","question":"Urgent?","levels":["low","high"],"top":1})),
    )
    .unwrap();
    call.set_raise_if_failed(true);
    let representation = thinkthen::native::sql_postgres_id_error("NaN")
        .unwrap()
        .unwrap();
    let rows = drain(
        &mut call,
        vec![
            SqlRecord::Missing,
            SqlRecord::Text("two".into()),
            SqlRecord::Text("three".into()),
        ],
        false,
        |call, row| {
            if row.error().is_none() {
                assert_eq!(
                    call.observe_row_failure(99, &representation)
                        .unwrap_err()
                        .code()
                        .as_str(),
                    "usage.abi"
                );
                assert_eq!(
                    call.observe_row_failure(row.position(), &thinkthen::native::abi_error())
                        .unwrap_err()
                        .code()
                        .as_str(),
                    "usage.abi"
                );
                call.observe_row_failure(row.position(), &representation)
                    .unwrap();
            }
        },
    );
    assert_eq!(rows.len(), 2, "top one plus failed row");
    assert_eq!(rows[0].position(), 2);
    assert_eq!(
        call.observe_row_failure(1, &representation)
            .unwrap_err()
            .code()
            .as_str(),
        "usage.abi"
    );
    assert_eq!(
        call.final_error().unwrap().code().as_str(),
        "record.missing"
    );
    summary(call.final_error().unwrap(), 2, 3, 1);
}

#[test]
fn postgres_representation_failures_are_seen_before_statement_summary() {
    let server = Loopback::guide_scenario("yes").unwrap();
    let client = engine(&server, BackendName::SystemOne);
    let mut call = SqlCall::open(
        &client,
        &request(json!({"function":"find","question":"Which line?"})),
    )
    .unwrap();
    call.set_raise_if_failed(true);
    let rows = drain(
        &mut call,
        vec![SqlRecord::Text("first\nsecond\u{0}line".into())],
        true,
        |call, row| {
            // A selected id failure on an already failed row still counts that input once.
            let failure = thinkthen::native::sql_postgres_id_error("NaN")
                .unwrap()
                .unwrap();
            call.observe_row_failure(row.position(), &failure).unwrap();
        },
    );
    assert_eq!(rows[0].error().unwrap().code().as_str(), "record.json");
    let (_, original) = rows[0].original_record().unwrap().original();
    assert_eq!(original, "first\nsecond\u{0}line".as_bytes());
    summary(call.final_error().unwrap(), 1, 1, 1);
}

#[test]
fn annotation_column_clashes_are_rejected_before_any_request() {
    let server = Loopback::guide_scenario("yes").unwrap();
    let client = engine(&server, BackendName::SystemOne);
    for names in [vec!["status"], vec!["probability"]] {
        let questions: serde_json::Map<String, Value> = names
            .iter()
            .map(|name| {
                (
                    (*name).into(),
                    json!({"function":"decide","question":"Ready?"}),
                )
            })
            .collect();
        let error = SqlCall::open(
            &client,
            &request(json!({"function":"annotate","questions":questions})),
        )
        .unwrap_err();
        assert_eq!(error.code().as_str(), "usage.column");
        assert!(error.to_string().contains(names.last().unwrap()));
    }
    let allowed = request(json!({"function":"annotate","questions":{
        "refund":{"function":"decide","question":"Refund?"},
        "refund_status":{"function":"decide","question":"Ready?"}
    }}));
    drop(SqlCall::open(&client, &allowed).unwrap());
    assert_eq!(server.count(), 0);
}
