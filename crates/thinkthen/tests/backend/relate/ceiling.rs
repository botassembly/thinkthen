//! Shared pair plans obey 400 questions and the request-size limit at every address.

use std::{fs, path::PathBuf};

use serde_json::Value;

use crate::harness::spawn;

const ELSEWHERE: &str = "http://127.0.0.1:9/v1";
const BEATLES: [&str; 2] = ["sung_by=song:person", "appears_on=song:album"];

fn beatles() -> Vec<u8> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/recognize-239/input.json"
    );
    let fixture: Value = serde_json::from_slice(&fs::read(path).expect("fixture")).expect("json");
    serde_json::to_vec(&fixture["entities"]).expect("entities")
}

fn entities(groups: &[(&str, usize)]) -> Vec<u8> {
    let values = groups
        .iter()
        .flat_map(|(kind, count)| {
            (0..*count).map(move |n| serde_json::json!({"name":format!("{kind} {n}"),"kind":kind}))
        })
        .collect::<Vec<_>>();
    serde_json::to_vec(&values).expect("entities")
}

fn lines(count: usize) -> Vec<u8> {
    (1..=count)
        .map(|n| format!("entity {n}\n"))
        .collect::<String>()
        .into_bytes()
}

fn profile(name: &str, limit: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-ceiling");
    fs::create_dir_all(&folder).expect("profile folder");
    let path = folder.join(format!("{name}.json"));
    fs::write(
        &path,
        format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}",{limit}}}"#),
    )
    .expect("profile");
    path
}

fn plan(options: &[&str], input: &[u8]) -> Value {
    let mut arguments = vec!["relate", "--dry-run", "--no-cache"];
    arguments.extend_from_slice(options);
    let output = spawn(&arguments, &[], input).expect("command");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("plan")
}

fn counts(plan: &Value) -> Vec<u64> {
    plan["relations"]
        .as_array()
        .expect("relations")
        .iter()
        .map(|item| item["request_count"].as_u64().expect("count"))
        .collect()
}

fn bytes(plan: &Value) -> Vec<u64> {
    plan["requests"]
        .as_array()
        .expect("requests")
        .iter()
        .map(|item| item["bytes"].as_u64().expect("bytes"))
        .collect()
}

fn questions(plan: &Value) -> Vec<usize> {
    plan["requests"]
        .as_array()
        .expect("requests")
        .iter()
        .map(|item| {
            serde_json::from_str::<Value>(item["body_utf8"].as_str().expect("body"))
                .expect("request")["questions"]
                .as_object()
                .expect("questions")
                .len()
        })
        .collect()
}

#[test]
fn the_beatles_set_uses_one_state_and_the_same_bound_at_every_address() {
    let input = beatles();
    let hosted = plan(&BEATLES, &input);
    let elsewhere = plan(&[BEATLES[0], BEATLES[1], "--url", ELSEWHERE], &input);
    assert_eq!(counts(&hosted), [2, 7]);
    assert_eq!(hosted["request_count"], 8);
    assert_eq!(questions(&hosted), [400, 400, 400, 400, 400, 400, 400, 328]);
    assert_eq!(bytes(&hosted), bytes(&elsewhere));
    assert!(bytes(&hosted).iter().all(|size| *size <= 96_000));
    let first: Value =
        serde_json::from_str(hosted["requests"][0]["body_utf8"].as_str().expect("body"))
            .expect("body");
    assert!(first["state"].get("relation").is_none());
}

#[test]
fn four_hundred_and_profile_limits_split_shared_rule_requests() {
    let input = entities(&[("person", 5), ("song", 180)]);
    let one = plan(&["wrote=person:song", "--url", ELSEWHERE], &input);
    assert_eq!(one["logical_questions"], 900);
    assert_eq!(questions(&one), [400, 400, 100]);
    assert_eq!(counts(&one), [3]);
    let two = plan(
        &["sang=person:song", "wrote=person:song", "--url", ELSEWHERE],
        &input,
    );
    assert_eq!(two["logical_questions"], 1_800);
    assert_eq!(questions(&two), [400, 400, 400, 400, 200]);
    assert_eq!(counts(&two), [3, 3]);
    assert_eq!(two["request_count"], 5);
    let low = profile("hundred", r#""max_questions":100"#);
    let low = plan(
        &[
            "wrote=person:song",
            "--profile",
            low.to_str().expect("path"),
        ],
        &input,
    );
    assert_eq!(questions(&low), [100; 9]);
    let high = profile("thousand", r#""max_questions":1000"#);
    let high = plan(
        &[
            "wrote=person:song",
            "--profile",
            high.to_str().expect("path"),
        ],
        &input,
    );
    assert_eq!(questions(&high), [400, 400, 100]);
}

#[test]
fn the_largest_cross_kind_rule_keeps_every_request_bounded() {
    let input = entities(&[("person", 127), ("organization", 128)]);
    let run = plan(
        &["works_for=person:organization", "--url", ELSEWHERE],
        &input,
    );
    assert_eq!(run["logical_questions"], 16_256);
    assert_eq!(run["request_count"], 41);
    assert_eq!(counts(&run), [41]);
    assert_eq!(questions(&run).last(), Some(&256));
    assert!(questions(&run).iter().all(|count| *count <= 400));
    assert!(bytes(&run).iter().all(|size| *size <= 96_000));
}

#[test]
fn small_cross_kind_rule_admits_254_and_255_complete_entities() {
    for (other, expected) in [(252, 254), (253, 255)] {
        let input = entities(&[("person", 1), ("place", 1), ("other", other)]);
        let run = plan(&["linked=person:place", "--url", ELSEWHERE], &input);
        assert_eq!(run["entity_count"], expected);
        assert_eq!(run["logical_questions"], 1);
        assert_eq!(run["request_count"], 1);
        assert_eq!(questions(&run), [1]);
    }
}

#[test]
fn oversized_sets_name_the_complete_count_and_hypothetical_pairs_exactly() {
    for (other, expected) in [
        (
            254,
            "thinkthen: relate takes at most 255 entities; this set has 256. If all 256 were distinct, an unordered all-kind rule would have 32640 candidate pairs; split the set or narrow by kind\n",
        ),
        (
            255,
            "thinkthen: relate takes at most 255 entities; this set has 257. If all 257 were distinct, an unordered all-kind rule would have 32896 candidate pairs; split the set or narrow by kind\n",
        ),
    ] {
        let input = entities(&[("person", 1), ("place", 1), ("other", other)]);
        let output = spawn(
            &["relate", "linked=person:place", "--url", ELSEWHERE],
            &[],
            &input,
        )
        .expect("command");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(String::from_utf8_lossy(&output.stderr), expected);
    }
}

#[test]
fn a_profile_lowers_the_byte_size_and_one_question_still_goes_alone() {
    let limit = profile("twenty-thousand", r#""max_request_bytes":20000"#);
    let small = plan(
        &[
            "linked",
            "--lines",
            "--url",
            ELSEWHERE,
            "--profile",
            limit.to_str().expect("path"),
        ],
        &lines(40),
    );
    assert!(bytes(&small).len() > 1);
    assert!(bytes(&small).iter().all(|size| *size <= 20_000));
    assert_eq!(questions(&small).iter().sum::<usize>(), 40 * 39);

    let long = "a".repeat(100_000);
    let input = serde_json::to_vec(&serde_json::json!([
        {"name":"one","kind":"song"}, {"name":"two","kind":"song"},
        {"name":long,"kind":"album"}
    ]))
    .expect("input");
    for options in [
        &["appears_on=song:album"][..],
        &["appears_on=song:album", "--url", ELSEWHERE][..],
    ] {
        let run = plan(options, &input);
        assert_eq!(questions(&run), [1, 1]);
        assert!(bytes(&run).iter().all(|size| *size > 96_000));
    }
}

#[test]
fn a_full_line_set_plans_inside_the_child_deadline() {
    let run = plan(
        &[
            "linked",
            "--lines",
            "--url",
            ELSEWHERE,
            "--max-request-bytes",
            "6000000",
        ],
        &lines(255),
    );
    assert_eq!(run["logical_questions"], 64_770);
    assert_eq!(run["request_count"], 162);
    assert_eq!(questions(&run).last(), Some(&370));
}
