//! The compiled recognize command against a counted loopback backend.

use crate::harness::{Canned, Listener, spawn};
use serde_json::Value;
use std::{fs, path::PathBuf, process::Output, sync::Arc, sync::Barrier};

mod rules;

const ADA: &[u8] = b"Ada met Acme.";

const ADA_AND_ACME: &str = r#"{"entities":[{"text":"Ada","start":0,"end":3,"length":3,"kind":"person","strength":0.9},{"text":"Acme","start":8,"end":12,"length":4,"kind":"organization","strength":0.9}]}"#;

pub(super) fn run(listener: &Listener, options: &[&str], input: &[u8]) -> Output {
    let mut arguments = vec![
        "recognize",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--no-cache",
    ];
    arguments.extend_from_slice(options);
    spawn(&arguments, &[("THINKTHEN_API_KEY", "secret-value")], input).expect("command")
}

fn capital(word: &str) -> bool {
    word.chars().next().is_some_and(char::is_uppercase)
}

/// The text before, inside and after the `[[ ]]` stretch a question shows.
fn marked(words: &str) -> (&str, &str, &str) {
    let (_, shown) = words
        .split_once("\n\nSnippet: ")
        .or_else(|| words.split_once("\n\nText: "))
        .expect("a shown text");
    let (before, rest) = shown.split_once("[[").expect("an opening mark");
    let (inside, after) = rest.split_once("]]").expect("a closing mark");
    (before, inside, after)
}

/// A step-1 tag from capitals: a capitalized piece is part of a name, and
/// capitalized neighbours across a space join it.
fn tag(words: &str) -> &'static str {
    let (before, piece, after) = marked(words);
    let before = before
        .ends_with(' ')
        .then(|| before.split_whitespace().last())
        .flatten();
    let after = after
        .starts_with(' ')
        .then(|| after.split_whitespace().next())
        .flatten();
    let joined = |word: Option<&str>| word.is_some_and(capital);
    match (capital(piece), joined(before), joined(after)) {
        (false, ..) => "OUT",
        (true, false, false) => "SINGLE",
        (true, false, true) => "BEGIN",
        (true, true, true) => "INSIDE",
        (true, true, false) => "END",
    }
}

/// A kind by spelling: `Nobody` is declined, `Town…` is a place, `Acme`,
/// `Corp` and `O…` are organizations, and any other name is a person.
fn kind(words: &str) -> &'static str {
    match marked(words).1 {
        "Nobody" => "none of these",
        name if name.starts_with("Town") => "place",
        "Acme" | "Corp" => "organization",
        name if name.starts_with('O') => "organization",
        _ => "person",
    }
}

/// Answer every question: step-1 tags from capitals at 1.0, a kind by
/// spelling at 0.9, the edge as found, and each pair at 0.9.
pub(super) fn automatic(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let mut answers = serde_json::Map::new();
    for (name, question) in request["questions"].as_object().expect("questions") {
        if question["type"] == "noul" {
            answers.insert(name.clone(), serde_json::json!({"type":"noul","noul":0.9}));
            continue;
        }
        let labels = question["criteria"].as_object().expect("criteria");
        let words = question["instructions"].as_str().expect("instructions");
        let (pick, (share, other)) = if labels.contains_key("BEGIN") {
            (tag(words), (1.0, 0.0))
        } else if labels.contains_key("none of these") {
            let held = kind(words);
            (
                if labels.contains_key(held) {
                    held
                } else {
                    "none of these"
                },
                (0.9, 0.1),
            )
        } else {
            (marked(words).1, (1.0, 0.0))
        };
        let rest = labels
            .keys()
            .find(|label| *label != pick)
            .expect("a second option");
        let probabilities = labels
            .keys()
            .map(|label| {
                let value = if label == pick {
                    share
                } else if label == rest {
                    other
                } else {
                    0.0
                };
                (label.clone(), Value::from(value))
            })
            .collect::<serde_json::Map<_, _>>();
        answers.insert(
            name.clone(),
            serde_json::json!({"type":"choice","choice":pick,"probabilities":probabilities}),
        );
    }
    Canned::ok(&serde_json::json!({"model":"local-1","answers":answers,"usage":{"input_tokens":10,"output_tokens":2}}).to_string())
}

pub(super) fn questions(body: &[u8]) -> Vec<Value> {
    let request: Value = serde_json::from_slice(body).expect("request");
    request["questions"]
        .as_object()
        .expect("questions")
        .values()
        .cloned()
        .collect()
}

fn options(question: &Value) -> Vec<String> {
    question["criteria"]
        .as_object()
        .expect("criteria")
        .keys()
        .cloned()
        .collect()
}

fn stdout(output: &Output) -> String {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout.clone()).expect("output text")
}

/// Ticket 0147, test 6: one step-1 request, then one step-2 request holding
/// both kind questions and `Acme`'s edge question.
#[test]
fn ada_met_acme_sends_one_step_one_request_then_one_step_two_request() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(&listener, &["person", "organization"], ADA);
    assert_eq!(stdout(&output), format!("{ADA_AND_ACME}\n"));
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let first = questions(&requests[0].body);
    assert_eq!(first.len(), 4);
    assert!(
        first
            .iter()
            .all(|question| options(question) == ["BEGIN", "END", "INSIDE", "OUT", "SINGLE"])
    );
    let second = questions(&requests[1].body);
    let shapes: Vec<Vec<String>> = second.iter().map(options).collect();
    assert_eq!(
        shapes,
        [
            vec!["none of these", "organization", "person"],
            vec!["none of these", "organization", "person"],
            vec!["Acme", "Acme."],
        ]
    );
}

/// Ticket 0147, test 6: no kinds asks only edge questions, a step-2 request
/// with no questions is not sent, and a text with no names sends one request.
#[test]
fn no_kinds_asks_only_edges_and_sends_no_empty_step_two_request() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(&listener, &[], ADA);
    assert_eq!(
        stdout(&output),
        concat!(
            r#"{"entities":[{"text":"Ada","start":0,"end":3,"length":3,"kind":"ENTITY","strength":1.0},"#,
            r#"{"text":"Acme","start":8,"end":12,"length":4,"kind":"ENTITY","strength":1.0}]}"#,
            "\n"
        )
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let second: Vec<Vec<String>> = questions(&requests[1].body).iter().map(options).collect();
    assert_eq!(second, [vec!["Acme", "Acme."]]);

    let output = run(&listener, &[], b"Ada met Bob");
    assert_eq!(
        stdout(&output),
        concat!(
            r#"{"entities":[{"text":"Ada","start":0,"end":3,"length":3,"kind":"ENTITY","strength":1.0},"#,
            r#"{"text":"Bob","start":8,"end":11,"length":3,"kind":"ENTITY","strength":1.0}]}"#,
            "\n"
        )
    );
    assert_eq!(listener.requests().len(), 1);

    let output = run(&listener, &["person"], b"the cat sat");
    assert_eq!(stdout(&output), "{\"entities\":[]}\n");
    assert_eq!(listener.requests().len(), 1);
}

/// Ticket 0147, test 6: a kind's description reaches its step-2 option and
/// never the step-1 request.
#[test]
fn descriptions_reach_the_step_two_option_and_never_step_one() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(
        &listener,
        &[
            "--kind",
            "person=DESC-PERSON",
            "--kind",
            "organization=DESC-ORG",
        ],
        ADA,
    );
    assert_eq!(stdout(&output), format!("{ADA_AND_ACME}\n"));
    let requests = listener.requests();
    assert!(!String::from_utf8_lossy(&requests[0].body).contains("DESC-"));
    let second = questions(&requests[1].body);
    assert_eq!(second[0]["criteria"]["person"], "DESC-PERSON");
    assert_eq!(second[0]["criteria"]["organization"], "DESC-ORG");
}

/// Ticket 0147, test 6: a failed step-2 request fails the text at exit 4
/// with no partial names.
#[test]
fn a_failed_step_two_request_fails_the_text() {
    let wrong = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.5}}}"#;
    let listener = Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains("none of these") {
            Canned::ok(wrong)
        } else {
            automatic(body)
        }
    })
    .expect("listener");
    let output = run(&listener, &["person", "organization"], ADA);
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the reply was refused: the answer to question `q1` is not the shape the question asked for\n"
    );
    assert_eq!(listener.requests().len(), 2);
}

/// Ticket 0147, test 6: the guard plans 600,000 bytes, refuses 600,001 before
/// any send, and moves with `--max-text-bytes`.
#[test]
fn the_guard_plans_600000_bytes_and_refuses_600001_before_any_send() {
    let listener = Listener::answering(automatic).expect("listener");
    let word = format!("{} ", "w".repeat(9_999));
    let at_limit = word.repeat(60);
    assert_eq!(at_limit.len(), 600_000);
    let planned = run(&listener, &["person", "--dry-run"], at_limit.as_bytes());
    let plan: Value = serde_json::from_str(&stdout(&planned)).expect("plan");
    assert_eq!(plan["pieces"], 60);
    assert_eq!(plan["request_count"], 2);

    let over = format!("{at_limit}w");
    let refused = run(&listener, &["person"], over.as_bytes());
    assert_eq!(refused.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: recognize: the text is 600001 bytes, over the limit of 600000; raise it with --max-text-bytes\n"
    );
    let raised = run(
        &listener,
        &["person", "--dry-run", "--max-text-bytes", "700000"],
        over.as_bytes(),
    );
    let plan: Value = serde_json::from_str(&stdout(&raised)).expect("plan");
    assert_eq!(plan["pieces"], 61);
    let zero = run(&listener, &["person", "--max-text-bytes", "0"], ADA);
    assert_eq!(zero.status.code(), Some(2));
    assert_eq!(listener.connections(), 0);
}

/// A dry run prints the digests and bodies of the step-1 requests a live run sends first.
#[test]
fn the_dry_run_prints_the_step_one_requests_a_live_run_sends() {
    let listener = Listener::answering(automatic).expect("listener");
    let dry = run(&listener, &["person", "organization", "--dry-run"], ADA);
    let plan: Value = serde_json::from_str(&stdout(&dry)).expect("dry-run JSON");
    let head = [
        "schema",
        "url",
        "model",
        "key_env",
        "pieces",
        "request_count",
        "name_requests_upper_bound",
    ]
    .map(|key| plan[key].clone());
    let url = format!("{}/systemone", listener.base());
    assert_eq!(
        head,
        serde_json::json!([
            "thinkthen.recognize-plan/2",
            url,
            "local-1",
            "THINKTHEN_API_KEY",
            4,
            1,
            1
        ])
        .as_array()
        .unwrap()
        .as_slice()
    );
    assert_eq!(listener.connections(), 0);
    let live = run(&listener, &["person", "organization", "--details"], ADA);
    let result: Value = serde_json::from_str(&stdout(&live)).expect("result JSON");
    let planned = plan["requests"].as_array().expect("a request list");
    assert_eq!(planned[0]["digest"], result["meta"]["requests"][0]);
    let sent = listener.requests();
    assert_eq!(
        planned[0]["body_utf8"],
        Value::from(String::from_utf8_lossy(&sent[0].body))
    );
    assert_eq!(planned[0]["bytes"], sent[0].body.len());
}

#[test]
fn question_file_dry_run_attributes_source_and_sums_every_rule_bound() {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-question.json");
    fs::write(&path, r#"{"version":1,"recognize":{"kinds":{"person":"A person.","organization":"An organization."},"relations":[{"name":"directed","source":"person","target":"person"},{"name":"either","source":"person","target":"person","either":true},{"name":"cross","source":"person","target":"organization"}]}}"#).expect("question file");
    let listener = Listener::serving(Vec::new()).expect("listener");
    let output = spawn(
        &[
            "recognize",
            &format!("@{}", path.display()),
            "--dry-run",
            "--url",
            listener.base(),
        ],
        &[],
        ADA,
    )
    .expect("dry run");
    let report: Value = serde_json::from_str(&stdout(&output)).expect("dry-run JSON");
    assert_eq!(report["from"], serde_json::json!({"question":"file"}));
    assert_eq!(report["relation_pairs_upper_bound"], 30);
    assert_eq!(report["relation_requests_upper_bound"], 30);
    assert_eq!(listener.connections(), 0);
}

#[test]
fn local_validation_matrix_never_sends() {
    for options in [
        vec!["person", "person"],
        vec!["person", "organization", "--relation", "x=person:place"],
        vec!["person", "organization", "--threshold", "0"],
        vec!["person", "organization", "--kind", "place=A place."],
        vec!["person", "--relation", "works_for=person"],
    ] {
        let listener = Listener::answering(automatic).expect("listener");
        let output = run(&listener, &options, ADA);
        assert_eq!(output.status.code(), Some(2), "{options:?}");
        assert_eq!(listener.connections(), 0, "{options:?}");
    }
}

#[test]
fn a_kind_without_a_sign_names_the_sign() {
    for options in [
        vec!["--kind", "PER"],
        vec!["--kind", "PER", "--kind", "ORG"],
    ] {
        let listener = Listener::answering(automatic).expect("listener");
        let output = run(&listener, &options, ADA);
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "thinkthen: --kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind\n",
            "{options:?}"
        );
        assert_eq!(output.status.code(), Some(2), "{options:?}");
        assert_eq!(listener.connections(), 0, "{options:?}");
    }
}

#[test]
fn relations_are_self_contained_and_absent_without_a_rule() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(
        &listener,
        &[
            "person",
            "organization",
            "--relation",
            "works_for=person:organization",
        ],
        ADA,
    );
    let value: Value = serde_json::from_str(&stdout(&output)).expect("relation output");
    assert_eq!(value["relations"].as_array().expect("relations").len(), 1);
    assert_eq!(value["relations"][0]["relation"], "works_for");
    assert_eq!(value["relations"][0]["source"], value["entities"][0]);
    assert_eq!(value["relations"][0]["target"], value["entities"][1]);
    assert_eq!(value["relations"][0]["probability"], 0.9);
    let pairs = listener.requests().pop().expect("pair request");
    let state: Value = serde_json::from_slice::<Value>(&pairs.body).unwrap()["state"].clone();
    assert_eq!(
        state,
        serde_json::json!({"evidence":"Ada met Acme.","entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Acme","kind":"organization"}]})
    );

    let empty = run(
        &listener,
        &[
            "person",
            "organization",
            "--relation",
            "visits=organization:organization",
        ],
        ADA,
    );
    let value: Value = serde_json::from_str(&stdout(&empty)).expect("empty relation output");
    assert_eq!(value["relations"], serde_json::json!([]));
    assert_eq!(listener.requests().len(), 2);
}

/// A one-question profile splits every request and prints the same names and edges.
#[test]
fn a_one_question_profile_prints_what_the_whole_requests_print() {
    let profile = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-one-question.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    )
    .expect("profile");
    let text = b"Ada Lovelace met Acme and Corp.";
    let options = [
        "person",
        "organization",
        "--relation",
        "works_for=person:organization",
        "--relation",
        "partners=organization:organization",
    ];
    let listener = Listener::answering(automatic).expect("listener");
    let whole = stdout(&run(&listener, &options, text));
    let whole_requests = listener.requests().len();
    let mut split_options = options.to_vec();
    split_options.extend(["--profile", profile.to_str().unwrap()]);
    let split = stdout(&run(&listener, &split_options, text));
    assert_eq!(split, whole);
    let value: Value = serde_json::from_str(&split).unwrap();
    assert_eq!(value["entities"][0]["text"], "Ada Lovelace");
    let edges = value["relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|edge| {
            (
                edge["relation"].as_str().unwrap(),
                edge["source"]["text"].as_str().unwrap(),
                edge["target"]["text"].as_str().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        edges,
        [
            ("works_for", "Ada Lovelace", "Acme"),
            ("works_for", "Ada Lovelace", "Corp"),
            ("partners", "Acme", "Corp"),
            ("partners", "Corp", "Acme")
        ]
    );
    let requests = listener.requests();
    assert!(requests.len() > whole_requests);
    assert!(
        requests
            .iter()
            .all(|request| questions(&request.body).len() == 1)
    );
}

#[test]
fn split_recognition_mixes_cache_and_live_then_replays_without_a_key() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-mixed-cache");
    let _removed = fs::remove_dir_all(&root);
    let profile = root.with_extension("profile.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    )
    .expect("profile");
    let listener = Listener::answering(automatic).expect("listener");
    let arguments = [
        "recognize",
        "person",
        "organization",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--profile",
        &profile.to_string_lossy(),
        "--cache",
        &root.to_string_lossy(),
    ];
    let first = spawn(&arguments, &[("THINKTHEN_API_KEY", "key")], ADA).expect("fill cache");
    assert_eq!(first.status.code(), Some(0));
    let _filled = listener.requests();
    let missing = fs::read_dir(&root)
        .unwrap()
        .filter_map(Result::ok)
        .find(|entry| {
            entry.path().extension().is_some_and(|ext| ext == "json")
                && !entry.file_name().to_string_lossy().starts_with('.')
        })
        .expect("entry");
    fs::remove_file(missing.path()).expect("remove one answer");
    let mixed = spawn(&arguments, &[("THINKTHEN_API_KEY", "key")], ADA).expect("mixed run");
    assert_eq!(mixed.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 1);
    let replay = spawn(&arguments, &[], ADA).expect("replay");
    assert_eq!(replay.status.code(), Some(0));
    assert_eq!(mixed.stdout, replay.stdout);
    assert_eq!(first.stdout, replay.stdout);
    assert!(listener.requests().is_empty());
}

#[test]
fn relation_identity_drives_recording_replay_and_cache_without_changing_recognition_bytes() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-relation-identity");
    let _removed = fs::remove_dir_all(&root);
    let kinds = ["person", "organization"];
    let baseline = Listener::answering(automatic).expect("baseline");
    let output = run(&baseline, &kinds, ADA);
    assert_eq!(output.status.code(), Some(0));
    let recognition: Vec<Vec<u8>> = baseline
        .requests()
        .into_iter()
        .map(|request| request.body)
        .collect();

    let recording = root.join("recording");
    let listener = Listener::answering(automatic).expect("recording listener");
    let recorded = spawn(
        &[
            "recognize",
            "person",
            "organization",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--record",
            &recording.to_string_lossy(),
            "--no-cache",
            "--relation",
            "works_for=person:organization",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        ADA,
    )
    .expect("record");
    assert_eq!(recorded.status.code(), Some(0));
    let requests = listener.requests();
    let bodies: Vec<Vec<u8>> = requests
        .iter()
        .take(2)
        .map(|request| request.body.clone())
        .collect();
    assert_eq!(bodies, recognition);
    let relation = requests.last().expect("relation request");
    let digest = crate::support::digest(listener.url(), &relation.body);
    assert!(recording.join(format!("{digest}.json")).is_file());

    let replayed = spawn(
        &[
            "recognize",
            "person",
            "organization",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--replay",
            &recording.to_string_lossy(),
            "--no-cache",
            "--relation",
            "works_for=person:organization",
        ],
        &[],
        ADA,
    )
    .expect("replay");
    assert_eq!(replayed.status.code(), Some(0));
    assert_eq!(replayed.stdout, recorded.stdout);
    assert!(listener.requests().is_empty());

    let cache = root.join("cache");
    let arguments = [
        "recognize",
        "person",
        "organization",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &cache.to_string_lossy(),
        "--relation",
        "works_for=person:organization",
    ];
    let filled = spawn(&arguments, &[("THINKTHEN_API_KEY", "key")], ADA).expect("fill cache");
    assert_eq!(filled.status.code(), Some(0));
    let _sent = listener.requests();
    let cached = spawn(&arguments, &[], ADA).expect("cached");
    assert_eq!(cached.status.code(), Some(0));
    assert_eq!(cached.stdout, filled.stdout);
    assert!(listener.requests().is_empty());
    assert!(cache.join(format!("{digest}.json")).is_file());
}

#[test]
fn details_carry_every_probability_and_request_metadata() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(
        &listener,
        &[
            "person",
            "organization",
            "--relation",
            "works_for=person:organization",
            "--details",
        ],
        ADA,
    );
    let requests = listener.requests();
    let mut expected =
        include_str!("../fixtures/recognize-detailed.json").replace("$URL", listener.url());
    for (place, request) in requests.iter().enumerate() {
        expected = expected.replace(
            &format!("$REQUEST{}", place + 1),
            &crate::support::digest(listener.url(), &request.body),
        );
    }
    assert_eq!(stdout(&output), expected);
}

#[test]
fn failed_relation_question_prints_no_partial_entity_or_edge_object() {
    let wrong = r#"{"model":"local-1","marker":"PRIVATE-RESPONSE","answers":{"q1":{"type":"choice","choice":"x","probabilities":{"x":1.0}}}}"#;
    let listener = Listener::answering(move |body| {
        if String::from_utf8_lossy(body).contains("Does the text itself state") {
            Canned::ok(wrong)
        } else {
            automatic(body)
        }
    })
    .expect("listener");
    let output = run(
        &listener,
        &[
            "person",
            "organization",
            "--relation",
            "works_for=person:organization",
        ],
        ADA,
    );
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: the reply was refused: the answer to question `q1` is not the shape the question asked for\n"
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("PRIVATE-RESPONSE"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-value"));
}

#[test]
fn concurrent_record_workers_print_in_input_order() {
    let release = Arc::new(Barrier::new(2));
    let listener = Listener::answering(move |body| automatic(body).after_release(release.clone()))
        .expect("listener");
    let output = run(
        &listener,
        &["person", "organization", "--lines", "--jobs", "2"],
        b"Ada met Acme.\nBob met Corp.\n",
    );
    let rows = stdout(&output);
    assert_eq!(
        rows,
        concat!(
            r#"{"input":"Ada met Acme.","value":{"entities":[{"text":"Ada","start":0,"end":3,"length":3,"kind":"person","strength":0.9},{"text":"Acme","start":8,"end":12,"length":4,"kind":"organization","strength":0.9}]}}"#,
            "\n",
            r#"{"input":"Bob met Corp.","value":{"entities":[{"text":"Bob","start":0,"end":3,"length":3,"kind":"person","strength":0.9},{"text":"Corp","start":8,"end":12,"length":4,"kind":"organization","strength":0.9}]}}"#,
            "\n"
        )
    );
    assert_eq!(listener.peak(), 2);
}

#[test]
fn an_impossible_profile_sends_nothing() {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-tiny-profile.json");
    fs::write(&path, r#"{"schema":"thinkthen.backend-profile/1","name":"tiny-recognize","max_evidence_bytes":2}"#).expect("profile");
    let listener = Listener::answering(automatic).expect("listener");
    let output = spawn(
        &[
            "recognize",
            "person",
            "--profile",
            &path.to_string_lossy(),
            "--url",
            listener.base(),
            "--no-cache",
        ],
        &[],
        ADA,
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
}
