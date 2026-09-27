//! Relation plans split under the built-in ceiling at the hosted address (ticket 0123).
//!
//! Every run is a dry run: it reads no key and sends nothing. With no `--url`
//! the plan resolves to the built-in address. `http://127.0.0.1:9/v1` stands
//! for any other address.

use std::{fs, path::PathBuf};

use serde_json::Value;

use crate::harness::spawn;

const ELSEWHERE: &str = "http://127.0.0.1:9/v1";
const BEATLES: [&str; 2] = ["sung_by=song:person", "appears_on=song:album"];

/// The committed Beatles set: 184 songs, 13 albums, and 4 people.
fn beatles() -> Vec<u8> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/recognize-239/input.json"
    );
    let fixture: Value = serde_json::from_slice(&fs::read(path).expect("fixture")).expect("json");
    serde_json::to_vec(&fixture["entities"]).expect("entities")
}

fn profile(name: &str, limit: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-ceiling");
    fs::create_dir_all(&folder).expect("profile folder");
    let path = folder.join(format!("{name}.json"));
    let text = format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}",{limit}}}"#);
    fs::write(&path, text).expect("profile");
    path
}

/// The plan of one dry run, after checking it succeeded.
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

#[test]
fn the_beatles_set_splits_at_the_hosted_address_and_nowhere_else() {
    let input = beatles();
    let hosted = plan(&BEATLES, &input);
    assert_eq!(hosted["url"], "https://api.typesafe.ai/v1/systemone");
    assert_eq!(hosted["backend_profile"], Value::Null);
    assert_eq!(counts(&hosted), [1, 2]);
    assert_eq!(bytes(&hosted), [81_943, 95_779, 76_411]);

    let elsewhere = plan(&[BEATLES[0], BEATLES[1], "--url", ELSEWHERE], &input);
    assert_eq!(counts(&elsewhere), [1, 1]);
    assert_eq!(bytes(&elsewhere), [81_943, 161_252]);
    assert_eq!(
        elsewhere["requests"][1]["digest"],
        "5fb2630212ea2666c1eff52b3baec2158c9e983c0e6974117fc59dbfe084d279"
    );
}

#[test]
fn a_trailing_slash_keeps_the_ceiling_and_another_version_drops_it() {
    let input = beatles();
    let cases = [
        ("https://api.typesafe.ai/v1/", [1, 2]),
        ("https://api.typesafe.ai/v2", [1, 1]),
    ];
    for (base, expected) in cases {
        let run = plan(&[BEATLES[0], BEATLES[1], "--url", base], &input);
        assert_eq!(counts(&run), expected, "{base}");
    }
}

#[test]
fn a_profile_byte_limit_replaces_the_ceiling_and_other_limits_join_it() {
    let input = beatles();
    let wide = profile("wide", r#""max_request_bytes":200000"#);
    let replaced = plan(
        &[
            BEATLES[0],
            BEATLES[1],
            "--profile",
            wide.to_str().expect("path"),
        ],
        &input,
    );
    assert_eq!(counts(&replaced), [1, 1]);
    assert_eq!(bytes(&replaced), [81_943, 161_252]);

    let questions = profile("sixty-four", r#""max_questions":64"#);
    let joined = plan(
        &[
            BEATLES[0],
            BEATLES[1],
            "--profile",
            questions.to_str().expect("path"),
        ],
        &input,
    );
    assert_eq!(counts(&joined), [3, 3]);
    assert_eq!(
        bytes(&joined),
        [35_565, 35_631, 32_688, 63_154, 63_220, 56_829]
    );
}

#[test]
fn one_question_over_the_ceiling_goes_alone_and_is_not_refused() {
    let album = "a".repeat(100_000);
    let input = serde_json::json!([
        {"name":"one","kind":"song"},
        {"name":"two","kind":"song"},
        {"name":album,"kind":"album"}
    ]);
    let input = serde_json::to_vec(&input).expect("input");
    let hosted = plan(&["appears_on=song:album"], &input);
    assert_eq!(bytes(&hosted), [200_493, 200_493]);
    let elsewhere = plan(&["appears_on=song:album", "--url", ELSEWHERE], &input);
    assert_eq!(bytes(&elsewhere), [300_710]);
}

fn lines(count: usize) -> Vec<u8> {
    (1..=count)
        .map(|item| format!("entity {item}\n"))
        .collect::<String>()
        .into_bytes()
}

#[test]
fn the_splitter_keeps_the_longest_fitting_prefix_the_old_loop_chose() {
    let limit = profile("twenty-thousand", r#""max_request_bytes":20000"#);
    let options = [
        "linked",
        "--lines",
        "--url",
        ELSEWHERE,
        "--model",
        "local-1",
        "--profile",
        limit.to_str().expect("path"),
    ];
    // Printed by main at 08a8e754, before the splitter changed.
    let expected = [19_938, 19_966, 19_923, 19_923, 19_923, 19_924, 16_660];
    assert_eq!(bytes(&plan(&options, &lines(40))), expected);
}

/// 255 line entities ask 64,770 yes/no questions. The prefix loop encoded every
/// prefix of them, and the harness's child deadline would kill it.
#[test]
fn a_full_line_set_plans_inside_the_child_deadline() {
    let input = lines(255);
    let elsewhere = plan(
        &[
            "linked", "--lines", "--url", ELSEWHERE, "--model", "local-1",
        ],
        &input,
    );
    assert_eq!(bytes(&elsewhere), [5_386_112]);
    let hosted = plan(&["linked", "--lines"], &input);
    let sizes = bytes(&hosted);
    assert_eq!(sizes.len(), 63);
    assert!(sizes.iter().all(|size| *size <= 96_000), "{sizes:?}");
}
