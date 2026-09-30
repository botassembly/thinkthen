//! Recognize over profiles, the cache, recordings, details and record workers.

use super::*;
use conformance_backend::Rendezvous;

/// A one-question profile splits every request and prints the same names and edges.
#[test]
fn a_one_question_profile_prints_what_the_whole_requests_print() {
    let one = profile("one-question", r#""max_questions":1"#);
    let text = b"Ada Lovelace met Acme and Corp.";
    let options = [
        &WORKS[..],
        &["--relation", "partners=organization:organization"],
    ]
    .concat();
    let listener = Listener::answering(automatic).expect("listener");
    let whole = stdout(&run(&listener, &options, text));
    let whole_requests = listener.requests().len();
    let split = stdout(&run(
        &listener,
        &[&options[..], &["--profile", one.to_str().unwrap()]].concat(),
        text,
    ));
    assert_eq!(split, whole);
    let value: Value = serde_json::from_str(&split).unwrap();
    assert_eq!(value["entities"][0]["text"], "Ada Lovelace");
    let edges: Vec<[&str; 3]> = value["relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|edge| {
            ["relation", "source", "target"].map(|at| {
                edge[at]
                    .as_str()
                    .or_else(|| edge[at]["text"].as_str())
                    .unwrap()
            })
        })
        .collect();
    assert_eq!(
        edges,
        [
            ["works_for", "Ada Lovelace", "Acme"],
            ["works_for", "Ada Lovelace", "Corp"],
            ["partners", "Acme", "Corp"],
            ["partners", "Corp", "Acme"]
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
    let one = profile("one-question", r#""max_questions":1"#);
    let listener = Listener::answering(automatic).expect("listener");
    let (one, cache) = (one.to_string_lossy(), root.to_string_lossy());
    let options = [&KINDS[..], &["--profile", &one, "--cache", &cache]].concat();
    let first = local(&listener, &options, Some("key"), ADA);
    assert_eq!(first.status.code(), Some(0));
    let filled = listener.requests().len();
    let store = rusqlite::Connection::open(root.join("thinkthen.sqlite")).expect("the store");
    let forgotten = store
        .execute(
            "DELETE FROM answers WHERE id = (SELECT MIN(id) FROM answers)",
            [],
        )
        .expect("remove one answer");
    assert_eq!(forgotten, 1);
    drop(store);
    let mixed = local(&listener, &options, Some("key"), ADA);
    let asked = listener.requests();
    assert!(filled > 1, "{filled} requests filled the cache");
    assert_eq!(asked.len(), 1);
    assert_eq!(questions(&asked[0].body).len(), 1);
    let replay = local(&listener, &options, None, ADA);
    assert_eq!(
        (stdout(&mixed), stdout(&replay)),
        (stdout(&first), stdout(&first))
    );
    assert!(listener.requests().is_empty());
}

#[test]
fn relation_identity_drives_recording_replay_and_cache_without_changing_recognition_bytes() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-relation-identity");
    let _removed = fs::remove_dir_all(&root);
    let baseline = Listener::answering(automatic).expect("baseline");
    stdout(&run(&baseline, &KINDS, ADA));
    let recognition: Vec<Vec<u8>> = baseline
        .requests()
        .into_iter()
        .map(|request| request.body)
        .collect();

    let listener = Listener::answering(automatic).expect("recording listener");
    let (recording, cache) = (
        root.join("recording").to_string_lossy().into_owned(),
        root.join("cache"),
    );
    let recorded = local(
        &listener,
        &[&WORKS[..], &["--record", &recording, "--no-cache"]].concat(),
        Some("key"),
        ADA,
    );
    let requests = listener.requests();
    let bodies: Vec<Vec<u8>> = requests
        .iter()
        .take(2)
        .map(|request| request.body.clone())
        .collect();
    assert_eq!(bodies, recognition);
    let relation = crate::support::keys(
        listener.url(),
        &requests.last().expect("relation request").body,
    );
    let holds = |folder: &std::path::Path| {
        let stored: Vec<String> = crate::support::stored(folder)
            .expect("stored answers")
            .iter()
            .filter_map(|line| line["key"].as_str().map(str::to_owned))
            .collect();
        relation.iter().all(|key| stored.contains(key))
    };
    assert!(holds(&root.join("recording")));
    let replayed = local(
        &listener,
        &[&WORKS[..], &["--replay", &recording, "--no-cache"]].concat(),
        None,
        ADA,
    );
    assert_eq!(stdout(&replayed), stdout(&recorded));
    assert!(listener.requests().is_empty());

    let held = cache.to_string_lossy();
    let options = [&WORKS[..], &["--cache", &held]].concat();
    let filled = local(&listener, &options, Some("key"), ADA);
    let _sent = listener.requests();
    assert_eq!(
        stdout(&local(&listener, &options, None, ADA)),
        stdout(&filled)
    );
    assert!(listener.requests().is_empty());
    assert!(holds(&cache));
}

#[test]
fn details_carry_every_probability_and_request_metadata() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(&listener, &[&WORKS[..], &["--details"]].concat(), ADA);
    let keys: Vec<String> = listener
        .requests()
        .iter()
        .flat_map(|request| crate::support::keys(listener.url(), &request.body))
        .collect();
    let expected = include_str!("../../fixtures/recognize-detailed.json")
        .replace("$URL", listener.url())
        .replace("$REQUESTS", &Value::from(keys).to_string());
    assert_eq!(stdout(&output), expected);
}

#[test]
fn failed_relation_question_prints_no_partial_entity_or_edge_object() {
    let wrong = r#"{"model":"local-1","marker":"PRIVATE-RESPONSE","answers":{"q1":{"type":"choice","choice":"x","probabilities":{"x":1.0}}}}"#;
    let output = run(
        &failing_on("Does the text itself state", wrong.to_owned()),
        &WORKS,
        ADA,
    );
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(String::from_utf8_lossy(&output.stderr), REFUSED);
}

#[test]
fn concurrent_record_workers_print_in_input_order() {
    let release = Arc::new(Rendezvous::new(2));
    let listener = Listener::answering(move |body| automatic(body).after_release(release.clone()))
        .expect("listener");
    let output = run(
        &listener,
        &[&KINDS[..], &["--lines", "--jobs", "2"]].concat(),
        b"Ada met Acme.\nBob met Corp.\n",
    );
    let bob = ADA_AND_ACME.replace("Ada", "Bob").replace("Acme", "Corp");
    assert_eq!(
        stdout(&output),
        format!(
            "{{\"input\":\"Ada met Acme.\",\"value\":{ADA_AND_ACME}}}\n{{\"input\":\"Bob met Corp.\",\"value\":{bob}}}\n"
        )
    );
    assert_eq!(listener.peak(), 2);
}

#[test]
fn an_impossible_profile_sends_nothing() {
    let tiny = profile("tiny", r#""max_evidence_bytes":2"#);
    let listener = Listener::answering(automatic).expect("listener");
    let output = local(
        &listener,
        &["person", "--profile", &tiny.to_string_lossy(), "--no-cache"],
        None,
        ADA,
    );
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
}
