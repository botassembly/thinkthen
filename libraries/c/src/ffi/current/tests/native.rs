//! Keep native value/storage regressions while the complete ABI waits for result/2.
#[path = "checks.rs"]
mod checks;
use super::{content, door, files, list, question, record, scratch, source, string, text};
use crate::ffi::carriers::{
    ChoiceV1, ChoicesV1, MemberSpecV1, MemberSpecsV1, OptionalContentV1, QuestionSpecV1,
    RelationV1, RelationsV1, RuleV1,
};
use crate::{
    Door,
    current::{self, QuestionHandle, ResultHandle, Row},
    failures,
};
use conformance_backend::{Backend, Canned, Listener};
use thinkthen::CallOptions;

fn make(door: &Door, kind: u32) -> Box<QuestionHandle> {
    let choices = [
        ChoiceV1 {
            name: string("z-first"),
            ..Default::default()
        },
        ChoiceV1 {
            name: string("a-second"),
            ..Default::default()
        },
    ];
    let mut spec = QuestionSpecV1 {
        kind,
        ..Default::default()
    };
    if kind < 8 {
        spec.text = content("Does this need attention?");
    }
    if (2..=4).contains(&kind) {
        spec.choices = ChoicesV1 {
            data: choices.as_ptr(),
            len: 2,
        };
    }
    let first;
    let second;
    let members;
    if kind == 8 {
        first = make(door, 1);
        second = make(door, 2);
        members = [
            MemberSpecV1 {
                name: string("z_first"),
                question: &*first,
            },
            MemberSpecV1 {
                name: string("a_second"),
                question: &*second,
            },
        ];
        spec.members = MemberSpecsV1 {
            data: members.as_ptr(),
            len: 2,
        };
    }
    let relation = RelationV1 {
        name: string("supports"),
        source: string("*"),
        target: string("*"),
        ..Default::default()
    };
    if kind == 10 {
        spec.relations = RelationsV1 {
            data: &relation,
            len: 1,
        };
    }
    question(door, spec)
}
fn call(door: &Door, kind: u32, q: &QuestionHandle, s: &current::SourceHandle) -> ResultHandle {
    current::ask(&door.0.engine, kind, q, s, CallOptions::default(), true).expect("native result")
}
#[test]
fn ten_private_native_projections_keep_values_probabilities_order_and_owned_storage() {
    for file_mode in [false, true] {
        let backend = Backend::start().expect("loopback");
        let door = door(&format!("{}/generic/v1", backend.origin()));
        let source = if file_mode {
            files(
                &door,
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../specification/fixtures/files/documents"
                ),
                3,
            )
        } else {
            source(
                &door,
                &[
                    record("Maria Chen joined Northwind Freight."),
                    record("A different document."),
                ],
            )
        };
        let results = (1..=10)
            .map(|kind| call(&door, kind, &make(&door, kind), &source))
            .collect::<Vec<_>>();
        drop(door);
        drop(source);
        for (index, result) in results.iter().enumerate() {
            assert_eq!(result.summary.function, index as u32 + 1);
            assert_eq!(result.summary.attempts_present, 1);
            assert!(!result.attempts.is_empty());
            assert_eq!(
                result.summary.facts.requests_sent,
                if index == 8 { 4 } else { 1 }
            );
            assert_eq!(result.attempts[0].ordinal, 1);
            assert_eq!(result.attempts[0].outcome, 1);
            assert!(!result.questions.is_empty());
            assert_eq!(result.questions[0].state, 1);
            assert!(result.questions[0].meta.requests.len > 0);
            checks::row(&result.rows[0], index, file_mode);
        }
        assert_eq!(backend.count(), 13);
    }
}
#[test]
fn native_null_authored_meanings_and_empty_attempts_remain_distinct() {
    for (probability, authored, kind, expected) in [
        (0.5, "", 1, 0),
        (0.9, "granted", 1, 2),
        (0.9, "{\"result\":true}", 2, 2),
    ] {
        let reply = format!(
            r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":{probability}}}}}}}"#
        );
        let backend = Listener::answering(move |_| Canned::ok(&reply)).expect("backend");
        let door = door(backend.base());
        let q = question(
            &door,
            QuestionSpecV1 {
                kind: 1,
                text: content("Is it relevant?"),
                yes: OptionalContentV1 {
                    present: i32::from(!authored.is_empty()),
                    value: crate::ffi::carriers::ContentV1 {
                        kind,
                        data: string(authored),
                    },
                },
                threshold: if authored.is_empty() {
                    RuleV1 {
                        kind: 3,
                        low: 0.2,
                        high: 0.8,
                    }
                } else {
                    RuleV1::default()
                },
                ..Default::default()
            },
        );
        let result = call(&door, 1, &q, &source(&door, &[record("evidence")]));
        let Row::Atomic(row) = &result.rows[0] else {
            panic!("atomic");
        };
        assert_eq!(row.value.decide_kind, expected);
        assert_eq!(row.value.authored.present, i32::from(expected == 2));
        if expected == 2 {
            assert_eq!(row.value.authored.value.kind, kind);
            assert_eq!(text(row.value.authored.value.data), authored);
        }
        let empty = call(&door, 1, &q, &source(&door, &[]));
        assert_eq!(
            (
                empty.summary.count,
                empty.summary.question_count,
                empty.summary.facts.records,
                empty.summary.facts.requests_sent
            ),
            (0, 0, 0, 0)
        );
        assert_eq!(
            (
                empty.summary.attempts_present,
                empty.summary.attempt_count,
                empty.summary.facts.model.present
            ),
            (1, 0, 0)
        );
        assert_eq!(backend.count(), 1);
    }
}
#[test]
fn native_wrong_kind_deadline_and_cancel_refuse_without_sending() {
    let backend = Backend::start().expect("loopback");
    let door = door(&format!("{}/generic/v1", backend.origin()));
    let q = make(&door, 1);
    let source = source(&door, &[record("evidence")]);
    assert_eq!(
        door.0.fail(
            current::ask(
                &door.0.engine,
                2,
                &q,
                &source,
                CallOptions::default(),
                false
            )
            .expect_err("wrong kind")
        ),
        1
    );
    let options = CallOptions::default().deadline_ms(0).expect("deadline");
    assert_eq!(
        door.0.fail(
            current::ask(&door.0.engine, 1, &q, &source, options, false).expect_err("deadline")
        ),
        3
    );
    let token = thinkthen::CancelToken::new();
    token.cancel();
    assert_eq!(
        door.0.fail(
            current::ask(
                &door.0.engine,
                1,
                &q,
                &source,
                CallOptions::default().cancel(&token),
                false
            )
            .expect_err("cancel")
        ),
        5
    );
    assert_eq!(backend.count(), 0);
}
#[test]
fn native_started_failure_retains_actual_final_facts() {
    let backend = Backend::start().expect("loopback");
    let door = door(&format!("{}/arm/status/401/v1", backend.origin()));
    let q = make(&door, 1);
    let s = source(&door, &[record("evidence")]);
    assert_eq!(
        door.0.fail(
            current::ask(&door.0.engine, 1, &q, &s, CallOptions::default(), false)
                .expect_err("401")
        ),
        2
    );
    let legacy: serde_json::Value =
        serde_json::from_str(&super::saved_facts(&door)).expect("legacy facts");
    assert_eq!(legacy["requests_sent"], 1);
    assert_eq!(legacy["records"], 0);
    let saved = failures::snapshot(Some(&door.0)).expect("saved failure");
    assert_eq!(saved.code, 2);
    assert_eq!(
        saved
            .facts
            .as_ref()
            .expect("native final facts")
            .requests_sent(),
        1
    );
    assert_eq!(
        saved.facts.as_ref().expect("native final facts").records(),
        0
    );
    door.0.fail(failures::Failure::usage("later refusal"));
    drop(door);
    assert_eq!(
        saved
            .facts
            .as_ref()
            .expect("owned final facts")
            .requests_sent(),
        1
    );
    assert_eq!(backend.count(), 1);
}
#[test]
fn native_loaded_question_set_and_member_failures_keep_order_and_authored_values() {
    let backend = Backend::start().expect("loopback");
    let door = door(&format!("{}/generic/v1", backend.origin()));
    let q = current::load(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specification/fixtures/files/questions.json"
    ))
    .expect("loaded");
    let result = call(&door, 8, &q, &source(&door, &[record("evidence")]));
    let Row::Annotation(row) = &result.rows[0] else {
        panic!("annotation");
    };
    let members = list(row.members.data, row.members.len);
    assert_eq!(
        members.iter().map(|m| text(m.name)).collect::<Vec<_>>(),
        ["contract", "urgent"]
    );
    assert_eq!(backend.count(), 1);
    let door = super::door(&format!(
        "{}/arm/malformed/missing_probability/v1",
        backend.origin()
    ));
    let q = current::parse(8, r#"{"version":1,"questions":{"first":{"decide":"Q?","true":"granted"},"second":{"choose":"Which?","options":["yes","no"]}}}"#.into()).expect("set");
    let result = call(&door, 8, &q, &source(&door, &[record("evidence")]));
    let Row::Annotation(row) = &result.rows[0] else {
        panic!("annotation");
    };
    let members = list(row.members.data, row.members.len);
    assert_eq!(members[0].state, 1);
    assert_eq!(text(members[0].value.authored.value.data), "granted");
    assert_eq!(
        (members[1].state, members[1].failure, members[1].value.kind),
        (2, 3, 0)
    );
    assert_eq!(
        (
            result.questions[1].state,
            result.questions[1].failure,
            result.questions[1].failed_questions
        ),
        (2, 3, 1)
    );
    assert_eq!(backend.count(), 2);
}
#[test]
fn native_recording_replay_preserves_values_keys_and_zero_send_attempts() {
    let backend = Backend::start().expect("loopback");
    let base = format!("{}/generic/v1", backend.origin());
    let folder = scratch("recording").join("recording");
    let builder = || {
        thinkthen::Engine::builder()
            .base_url(&base)
            .expect("url")
            .api_key("sk-c-door-loopback")
            .expect("fake key")
            .no_cache()
    };
    let live = Door(failures::Held::new(
        builder()
            .record(&folder)
            .expect("record")
            .build()
            .expect("engine"),
    ));
    let q = make(&live, 1);
    let source = source(&live, &[record("evidence")]);
    let first = call(&live, 1, &q, &source);
    drop(live);
    let replay = Door(failures::Held::new(
        builder()
            .replay(&folder)
            .expect("replay")
            .build()
            .expect("engine"),
    ));
    let second = call(&replay, 1, &q, &source);
    drop(replay);
    drop(source);
    drop(q);
    assert_eq!(first.summary.facts.requests_sent, 1);
    assert_eq!(second.summary.facts.requests_sent, 0);
    assert_eq!(
        (
            second.summary.attempts_present,
            second.summary.attempt_count
        ),
        (1, 0)
    );
    let Row::Atomic(first) = &first.rows[0] else {
        panic!("atomic");
    };
    let Row::Atomic(second) = &second.rows[0] else {
        panic!("atomic");
    };
    assert_eq!((first.meta.cached, second.meta.cached), (0, 1));
    assert_eq!(
        text(list(first.meta.requests.data, first.meta.requests.len)[0]),
        text(list(second.meta.requests.data, second.meta.requests.len)[0])
    );
    assert_eq!(backend.count(), 1);
}
#[test]
fn native_line_reader_keeps_unicode_crlf_and_physical_lines_without_sending_paths() {
    let backend = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#)).expect("loopback");
    let door = door(backend.base());
    let path = scratch("lines").join("named-input.txt");
    std::fs::write(&path, "é\r\n\r\n😀 tail\n").expect("source");
    let source = files(&door, path.to_str().expect("path"), 1);
    let result = call(&door, 1, &make(&door, 1), &source);
    for (row, input, line) in [(&result.rows[0], "é", 1), (&result.rows[1], "😀 tail", 3)] {
        let Row::Atomic(row) = row else {
            panic!("atomic");
        };
        assert_eq!(text(row.common.input.value.data), input);
        assert_eq!(row.common.position.value.first_line.value, line);
        assert_eq!(row.common.position.value.last_line.value, line);
    }
    assert_eq!(backend.count(), 1);
    let request = String::from_utf8(backend.requests()[0].body.clone()).expect("request");
    assert!(!request.contains("named-input.txt"));
    assert!(request.contains('é'));
    assert!(request.contains("😀 tail"));
}
#[test]
fn immutable_shared_native_inputs_produce_independent_results_on_two_threads() {
    let backend = Backend::start().expect("loopback");
    let door = door(&format!("{}/generic/v1", backend.origin()));
    let q = make(&door, 1);
    let source = source(&door, &[record("evidence")]);
    std::thread::scope(|scope| {
        let one = scope.spawn(|| {
            let result = call(&door, 1, &q, &source);
            assert_eq!(result.summary.count, 1);
        });
        let two = scope.spawn(|| {
            let result = call(&door, 1, &q, &source);
            assert_eq!(result.summary.count, 1);
        });
        one.join().expect("first");
        two.join().expect("second");
    });
    assert_eq!(backend.count(), 2);
}
