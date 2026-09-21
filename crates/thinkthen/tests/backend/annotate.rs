//! The compiled `annotate` command against a loopback backend.

use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::harness::{Canned, Listener, spawn};

const ANSWERED: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.92},"#,
    r#""q2":{"type":"choice","choice":"bug","probabilities":{"bug":0.8,"other":0.2}},"#,
    r#""q3":{"type":"score","score":0.8,"probabilities":{"0":0.2,"1":0.8}}},"#,
    r#""usage":{"input_tokens":30,"output_tokens":6}}"#,
);

fn set(name: &str, text: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate");
    let _created = fs::create_dir_all(&folder);
    let path = folder.join(format!("{name}.json"));
    let _written = fs::write(&path, text);
    path
}

fn questions() -> PathBuf {
    static QUESTIONS: OnceLock<PathBuf> = OnceLock::new();
    QUESTIONS
        .get_or_init(|| {
            set(
                "mixed",
                concat!(
                    r#"{"version":1,"questions":{"risky":{"decide":"Is this risky?"},"#,
                    r#""kind":{"choose":"What kind?","options":["bug","other"]},"#,
                    r#""severity":{"score":"How severe?","levels":["low","high"]}}}"#,
                ),
            )
        })
        .clone()
}

mod request_identity;
mod scheduling;

#[test]
fn same_evidence_packs_mixed_questions_and_appends_answers() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a listener");
    let file = questions();
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"id":7,"body":"The service failed."}"#,
    )
    .expect("the command runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"id\":7,\"body\":\"The service failed.\",\"risky\":true,\"kind\":\"bug\",\"severity\":0.8}\n"
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body = String::from_utf8_lossy(&requests[0].body);
    assert!(body.contains(r#""q1":{"type":"noul"#), "{body}");
    assert!(body.contains(r#""q2":{"type":"choice"#), "{body}");
    assert!(body.contains(r#""q3":{"type":"score"#), "{body}");
}

#[test]
fn tag_aggregates_beside_another_question_in_one_request() {
    let answer = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.92},"#,
        r#""q2":{"type":"noul","noul":0.81},"q3":{"type":"noul","noul":0.12}}}"#,
    );
    let listener = Listener::serving(vec![Canned::ok(answer)]).expect("a listener");
    let file = set(
        "tag-mixed",
        concat!(
            r#"{"version":1,"questions":{"risky":{"decide":"Is this risky?"},"#,
            r#""topics":{"tag":"Which topics?","labels":["billing","urgent"]}}}"#,
        ),
    );
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"body":"The invoice failed."}"#,
    )
    .expect("the command runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"body\":\"The invoice failed.\",\"risky\":true,\"topics\":[\"billing\"]}\n"
    );
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn absent_on_and_explicit_root_share_one_request_and_whole_evidence() {
    let answer = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"noul","noul":0.1}}}"#,
    );
    let listener = Listener::serving(vec![Canned::ok(answer)]).expect("a listener");
    let file = set(
        "root-group",
        r#"{"version":1,"questions":{"first":{"decide":"first?"},"second":{"decide":"second?","on":""}}}"#,
    );
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"whole plain-text evidence",
    )
    .expect("the command runs");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"first\":true,\"second\":false}\n"
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert!(String::from_utf8_lossy(&requests[0].body).contains("whole plain-text evidence"));
}

#[test]
fn a_collision_and_dry_run_send_no_request() {
    let listener = Listener::serving(Vec::new()).expect("a listener");
    let file = questions();
    let common = [
        "annotate",
        &file.to_string_lossy(),
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--jsonl",
    ];
    let collided = spawn(
        &common,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"risky":false}"#,
    )
    .expect("the command runs");
    assert_eq!(collided.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&collided.stderr),
        concat!(
            "thinkthen: the record already holds `risky`, so that question cannot be appended\n",
            "thinkthen: stopped at record 1; 0 records finished, 0 records from a recording\n",
        )
    );

    let mut dry = common.to_vec();
    dry.push("--dry-run");
    let planned = spawn(&dry, &[], br#"{"body":"The service failed."}"#).expect("the command runs");
    assert_eq!(planned.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&planned.stdout);
    assert!(
        stdout.contains(r#""on":{"risky":[""],"kind":[""],"severity":[""]}"#),
        "{stdout}"
    );
    assert!(listener.requests().is_empty());
}

#[test]
fn likely_path_mistakes_name_the_input_option_and_the_swap() {
    let questions = questions();
    let evidence = set("plain-evidence", r#"{"body":"hello"}"#);
    let second_path = spawn(
        &[
            "annotate",
            &questions.to_string_lossy(),
            &evidence.to_string_lossy(),
            "--dry-run",
        ],
        &[],
        b"",
    )
    .expect("the command runs");
    assert_eq!(second_path.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&second_path.stderr),
        "thinkthen: the second path is input; write it as `--input FILE`\n"
    );

    let swapped = spawn(
        &[
            "annotate",
            &evidence.to_string_lossy(),
            "--input",
            &questions.to_string_lossy(),
            "--dry-run",
        ],
        &[],
        b"",
    )
    .expect("the command runs");
    assert_eq!(swapped.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&swapped.stderr),
        "thinkthen: the question set and input appear to be swapped; put the question set after `annotate` and the evidence after `--input`\n"
    );
}

#[test]
fn a_detailed_recording_replays_without_a_key_or_second_request() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a listener");
    let file = questions();
    let recording = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-recording");
    let _absent = fs::remove_dir_all(&recording);
    let recording_text = recording.to_string_lossy();
    let base = [
        "annotate",
        &file.to_string_lossy(),
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--details",
    ];
    let mut recorded = base.to_vec();
    recorded.extend(["--record", &recording_text]);
    let first = spawn(
        &recorded,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"The service failed.",
    )
    .expect("the recording run");
    assert_eq!(
        first.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let row = String::from_utf8_lossy(&first.stdout);
    assert!(row.contains(r#""questions_sha256":"#), "{row}");
    assert!(row.contains(r#""replayed":false"#), "{row}");
    assert!(row.contains(r#""answers":{"risky":{"value":true"#), "{row}");

    let mut replayed = base.to_vec();
    replayed.extend(["--replay", &recording_text]);
    let second = spawn(&replayed, &[], b"The service failed.").expect("the replay run");
    assert_eq!(
        second.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    let row = String::from_utf8_lossy(&second.stdout);
    assert!(row.contains(r#""replayed":true"#), "{row}");
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn different_safe_model_versions_fail_one_record_and_name_both() {
    let first = r#"{"model":"jev-1.2","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let second = r#"{"model":"jev-1.3","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let listener =
        Listener::serving(vec![Canned::ok(first), Canned::ok(second)]).expect("a listener");
    let file = set(
        "two-groups",
        r#"{"version":1,"questions":{"left_answer":{"decide":"left?","on":"/left"},"right_answer":{"decide":"right?","on":"/right"}}}"#,
    );
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "jev-latest",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"left":"yes","right":"yes"}"#,
    )
    .expect("the command runs");
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the backend returned model versions `jev-1.2` and `jev-1.3` for one record; pin --model and rerun with --record or --cache\n"
    );
    assert!(output.stdout.is_empty());
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn the_first_group_failure_wins_when_the_second_finishes_first() {
    let listener = Listener::answering(|body| {
        let body = String::from_utf8_lossy(body);
        if body.contains(r#""state":"slow""#) {
            Canned::status(422, "{}").after(40)
        } else {
            Canned::status(401, "{}")
        }
    })
    .expect("a listener");
    let file = set(
        "failure-order",
        r#"{"version":1,"questions":{"first_answer":{"decide":"first?","on":"/first"},"second_answer":{"decide":"second?","on":"/second"}}}"#,
    );
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--jobs",
            "4",
            "--max-retries",
            "0",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"first":"slow","second":"fast"}"#,
    )
    .expect("the command runs");
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the backend answered with status 422: the backend refused the request as malformed or too large\n"
    );
    assert_eq!(listener.requests().len(), 2);
}

#[test]
fn hostile_model_names_never_reach_the_diagnostic() {
    let file = set(
        "hostile-models",
        r#"{"version":1,"questions":{"left_answer":{"decide":"left?","on":"/left"},"right_answer":{"decide":"right?","on":"/right"}}}"#,
    );
    let oversized = "x".repeat(65);
    let oversized_version = format!("jev-{}", "1".repeat(65));
    let cases = [
        ("jev-latest".to_owned(), "jev-1\\u001b".to_owned()),
        ("jev-latest".to_owned(), oversized),
        ("jev-latest".to_owned(), oversized_version),
        (
            "key-marker\u{1b}evidence-marker".to_owned(),
            "key-marker\\u001bevidence-marker".to_owned(),
        ),
    ];
    for (place, (requested, returned)) in cases.into_iter().enumerate() {
        let first =
            format!(r#"{{"model":"{returned}","answers":{{"q1":{{"type":"noul","noul":0.9}}}}}}"#);
        let second = r#"{"model":"jev-1.2","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
        let listener =
            Listener::serving(vec![Canned::ok(&first), Canned::ok(second)]).expect("a listener");
        let output = spawn(
            &[
                "annotate",
                &file.to_string_lossy(),
                "--url",
                listener.base(),
                "--model",
                &requested,
            ],
            &[("THINKTHEN_API_KEY", "key-marker")],
            br#"{"left":"evidence-marker","right":"plain"}"#,
        )
        .expect("the command runs");
        assert_eq!(output.status.code(), Some(4), "case {place}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "thinkthen: the backend returned different model versions for one record; pin --model and rerun with --record or --cache\n",
            "case {place}"
        );
        assert!(output.stdout.is_empty(), "case {place}");
        assert_eq!(listener.requests().len(), 2, "case {place}");
    }
}
