//! The compiled recognize command against a counted loopback backend.

use crate::harness::{Canned, Listener, spawn};
use serde_json::Value;
use std::{fs, path::PathBuf, process::Output, sync::Arc, sync::Barrier};

const ANSWERED: &str = include_str!("../fixtures/recognize-answered.json");

const RELATED: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"choice","choice":"i2","probabilities":{"i2":0.9,"none":0.1}}},"usage":{"input_tokens":20,"output_tokens":4}}"#;

fn run(listener: &Listener, options: &[&str], input: &[u8]) -> Output {
    let mut arguments = vec!["recognize", "--url", listener.base(), "--model", "local-1", "--no-cache"];
    arguments.extend_from_slice(options);
    spawn(&arguments, &[("THINKTHEN_API_KEY", "secret-value")], input).expect("command")
}

fn automatic(body: &[u8]) -> Canned { automatic_answers(body, false, 0.9) }

fn automatic_answers(body: &[u8], every_relation_option: bool, pair_probability: f64) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let mut answers = serde_json::Map::new();
    for (name, question) in request["questions"].as_object().expect("questions") {
        let answer = if question["type"] == "noul" {
            serde_json::json!({"type":"noul","noul":pair_probability})
        } else {
            let criteria = question["criteria"].as_object().expect("criteria");
            let words = question["instructions"].as_str().expect("instructions");
            let pick = match (criteria.contains_key("IN"), criteria.contains_key("person")) {
                (true, _) => ["[[met]]", "[[and]]", "[[x]]", "[[.]]"].iter().any(|word| words.contains(word)).then_some("OUT").unwrap_or("IN"),
                (_, true) => {
                    if words.contains("[[Town") { "place" }
                    else if words.contains("[[Acme]]") || words.contains("[[Corp]]") || words.contains("[[O") { "organization" }
                    else { "person" }
                },
                _ => criteria.keys().find(|label| label.starts_with('i')).expect("relation option"),
            };
            let relation_options = criteria.keys().filter(|label| label.starts_with('i')).count();
            let probabilities = criteria.keys().map(|label| {
                let probability = if every_relation_option && relation_options > 0 && label.starts_with('i') { 1.0 / relation_options as f64 } else { f64::from(label == pick) };
                (label.clone(), Value::from(probability))
            }).collect::<serde_json::Map<_, _>>();
            serde_json::json!({"type":"choice","choice":pick,"probabilities":probabilities})
        };
        answers.insert(name.clone(), answer);
    }
    Canned::ok(&serde_json::json!({"model":"local-1","answers":answers}).to_string())
}

#[test]
fn recognize_prints_the_bare_object_with_scalar_offsets_and_strengths() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("listener");
    let output = run(&listener, &[], b"Ada met Acme.");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), r#"{"entities":[{"name":"Ada","kind":"person","start":0,"end":3,"strength":0.81},{"name":"Acme","kind":"organization","start":8,"end":12,"strength":0.72}]}"#.to_owned() + "\n");
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn one_kind_recognizes_without_an_invalid_one_option_choice() {
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(&listener, &["person"], b"Ada");
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(serde_json::from_slice::<Value>(&output.stdout).unwrap()["entities"][0]["kind"], "person");
}

#[test]
fn malformed_and_dry_runs_open_no_connection() {
    let listener = Listener::answering(|_| Canned::ok(ANSWERED)).expect("listener");
    let malformed = spawn(&["recognize", "person", "--relation", "works_for=person", "--url", listener.base(), "--no-cache"], &[("THINKTHEN_API_KEY", "secret-value")], b"Ada met Acme.").expect("command");
    assert_eq!(malformed.status.code(), Some(2));

    let dry = spawn(&["recognize", "--relation", "first=person:person", "--relation", "second=person:person", "--relation", "third=person:person", "person", "organization", "--dry-run", "--url", listener.base(), "--no-cache"], &[], b"Ada met Acme.").expect("command");
    assert_eq!(dry.status.code(), Some(0));
    let report: Value = serde_json::from_slice(&dry.stdout).expect("dry-run JSON");
    assert_eq!(report["kind_questions"], 4);
    assert_eq!(report["relation_pairs_upper_bound"], 36);
    assert_eq!(report["relation_requests_upper_bound"], 36);
    let one = spawn(&["recognize", "person", "--dry-run", "--url", listener.base(), "--no-cache"], &[], b"Ada met Acme.").expect("one-kind dry run");
    let one: Value = serde_json::from_slice(&one.stdout).expect("one-kind JSON");
    let counts = ["words", "detection_questions", "kind_questions", "request_count"].map(|key| one[key].clone());
    assert_eq!(counts, [4, 4, 0, 1].map(Value::from));
    assert_eq!(listener.connections(), 0);
}

/// A dry run prints the digests and bodies a live run of the same text sends.
#[test]
fn the_dry_run_prints_the_requests_a_live_run_sends() {
    let listener = Listener::answering(automatic).expect("listener");
    let dry = run(&listener, &["person", "organization", "--dry-run"], b"Ada met Acme.");
    let plan: Value = serde_json::from_slice(&dry.stdout).expect("dry-run JSON");
    let head = ["schema", "url", "model", "key_env", "words", "request_count"].map(|key| plan[key].clone());
    let url = format!("{}/systemone", listener.base());
    assert_eq!(head, serde_json::json!(["thinkthen.recognize-plan/1", url, "local-1", "THINKTHEN_API_KEY", 4, 1]).as_array().unwrap().as_slice());
    assert_eq!(listener.connections(), 0);
    let live = run(&listener, &["person", "organization", "--details"], b"Ada met Acme.");
    assert_eq!(live.status.code(), Some(0), "{}", String::from_utf8_lossy(&live.stderr));
    let result: Value = serde_json::from_slice(&live.stdout).expect("result JSON");
    let planned = plan["requests"].as_array().expect("a request list");
    let digests: Vec<Value> = planned.iter().map(|request| request["digest"].clone()).collect();
    assert_eq!(Value::from(digests), result["meta"]["requests"]);
    let sent: Vec<Value> = listener.requests().iter().map(|request| Value::from(String::from_utf8_lossy(&request.body))).collect();
    let bodies: Vec<Value> = planned.iter().map(|request| request["body_utf8"].clone()).collect();
    assert_eq!(bodies, sent);
    assert_eq!(planned[0]["bytes"], Value::from(sent[0].as_str().map_or(0, str::len)));
}

#[test]
fn question_file_dry_run_attributes_source_and_sums_every_rule_bound() {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-question.json");
    fs::write(&path, r#"{"version":1,"recognize":{"kinds":{"person":"A person.","organization":"An organization."},"relations":[{"name":"directed","source":"person","target":"person"},{"name":"either","source":"person","target":"person","either":true},{"name":"cross","source":"person","target":"organization"}]}}"#).expect("question file");
    let listener = Listener::serving(Vec::new()).expect("listener");
    let output = spawn(&["recognize", &format!("@{}", path.display()), "--dry-run", "--url", listener.base()], &[], b"Ada met Acme.").expect("dry run");
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let report: Value = serde_json::from_slice(&output.stdout).expect("dry-run JSON");
    assert_eq!(report["from"], serde_json::json!({"question":"file"}));
    assert_eq!(report["relation_pairs_upper_bound"], 30);
    assert_eq!(report["relation_requests_upper_bound"], 30);
    assert_eq!(listener.connections(), 0);
}

#[test]
fn local_validation_matrix_never_sends() {
    for options in [vec!["person", "person"], vec!["person", "organization", "--relation", "x=person:place"], vec!["person", "organization", "--threshold", "0"], vec!["person", "organization", "--kind", "place=A place."]] {
        let listener = Listener::answering(automatic).expect("listener");
        let output = run(&listener, &options, b"Ada met Acme.");
        assert_eq!(output.status.code(), Some(2), "{options:?}");
        assert_eq!(listener.connections(), 0, "{options:?}");
    }
}

#[test]
fn relations_are_self_contained_and_absent_without_a_rule() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED), Canned::ok(RELATED), Canned::ok(ANSWERED)]).expect("listener");
    let output = run(&listener, &["--relation", "works_for=person:organization"], b"Ada met Acme.");
    assert_eq!(output.status.code(), Some(0));
    let value: Value = serde_json::from_slice(&output.stdout).expect("relation output");
    assert_eq!(value["relations"].as_array().expect("relations").len(), 1);
    assert_eq!(value["relations"][0]["relation"], "works_for");
    assert_eq!(value["relations"][0]["source"], value["entities"][0]);
    assert_eq!(value["relations"][0]["target"], value["entities"][1]);
    assert_eq!(value["relations"][0]["probability"], 0.9);

    let empty = run(&listener, &["--relation", "visits=place:person"], b"Ada met Acme.");
    let value: Value = serde_json::from_slice(&empty.stdout).expect("empty relation output");
    assert_eq!(value["relations"], serde_json::json!([]));
}

#[test]
fn command_planner_maps_every_above_cut_choice_option_to_exact_entities() {
    let listener = Listener::answering(|body| automatic_answers(body, true, 0.9)).expect("listener");
    let output = run(&listener, &["--relation", "works_for=person:organization"], b"Ada x Bob x Acme x Corp");
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    let edges = value["relations"].as_array().unwrap().iter().map(|edge| (edge["source"]["name"].as_str().unwrap(), edge["target"]["name"].as_str().unwrap(), edge["probability"].as_f64().unwrap())).collect::<Vec<_>>();
    assert_eq!(edges, [("Ada", "Acme", 0.5), ("Ada", "Corp", 0.5), ("Bob", "Acme", 0.5), ("Bob", "Corp", 0.5)]);
}

#[test]
fn split_recognition_and_relation_batches_assemble_normalized_choice_and_pair_edges() {
    let profile = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-one-question.json");
    fs::write(&profile, r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#).expect("profile");
    let listener = Listener::answering(automatic).expect("listener");
    let text = b"Ada Lovelace met Acme and Corp.";
    let output = run(&listener, &["--profile", &profile.to_string_lossy(), "--relation", "works_for=person:organization", "--relation", "partners=organization:organization"], text);
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let value: Value = serde_json::from_slice(&output.stdout).expect("result");
    assert_eq!(value["entities"][0]["name"], "Ada Lovelace");
    assert_eq!(value["relations"].as_array().expect("relations").len(), 4);
    let edges = value["relations"].as_array().unwrap().iter().map(|edge| (edge["relation"].as_str().unwrap(), edge["source"]["name"].as_str().unwrap(), edge["target"]["name"].as_str().unwrap())).collect::<Vec<_>>();
    assert_eq!(edges, [("works_for", "Ada Lovelace", "Acme"), ("works_for", "Ada Lovelace", "Corp"), ("partners", "Acme", "Corp"), ("partners", "Corp", "Acme")]);
    let requests = listener.requests();
    assert!(requests.len() > text.split(|byte| byte.is_ascii_whitespace()).count() * 2);
    let states = requests.iter().map(|request| serde_json::from_slice::<Value>(&request.body).unwrap()["state"].clone()).collect::<Vec<_>>();
    assert!(states.iter().any(|state| state == "Ada Lovelace met Acme and Corp."));
    let relation_states = states.iter().filter(|state| state.is_object()).collect::<Vec<_>>();
    assert!(relation_states.iter().all(|state| state["evidence"] == "Ada Lovelace met Acme and Corp."));
    assert!(relation_states.iter().all(|state| state["entities"].as_array().unwrap().len() == 3));
}

#[test]
fn split_recognition_mixes_cache_and_live_then_replays_without_a_key() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-mixed-cache");
    let _removed = fs::remove_dir_all(&root);
    let profile = root.with_extension("profile.json");
    fs::write(&profile, r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#).expect("profile");
    let listener = Listener::answering(automatic).expect("listener");
    let arguments = ["recognize", "--url", listener.base(), "--model", "local-1", "--profile", &profile.to_string_lossy(), "--cache", &root.to_string_lossy()];
    let first = spawn(&arguments, &[("THINKTHEN_API_KEY", "key")], b"Ada met Acme.").expect("fill cache");
    assert_eq!(first.status.code(), Some(0));
    let _filled = listener.requests();
    let missing = fs::read_dir(&root).unwrap().filter_map(Result::ok).find(|entry| entry.path().extension().is_some_and(|ext| ext == "json") && !entry.file_name().to_string_lossy().starts_with('.')).expect("entry");
    fs::remove_file(missing.path()).expect("remove one answer");
    let mixed = spawn(&arguments, &[("THINKTHEN_API_KEY", "key")], b"Ada met Acme.").expect("mixed run");
    assert_eq!(mixed.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 1);
    let replay = spawn(&arguments, &[], b"Ada met Acme.").expect("replay");
    assert_eq!(replay.status.code(), Some(0));
    assert_eq!(mixed.stdout, replay.stdout);
    assert!(listener.requests().is_empty());
}

#[test]
fn relation_identity_drives_recording_replay_and_cache_without_changing_recognition_bytes() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-relation-identity");
    let _removed = fs::remove_dir_all(&root);
    let baseline = Listener::answering(automatic).expect("baseline");
    let output = run(&baseline, &[], b"Ada met Acme.");
    assert_eq!(output.status.code(), Some(0));
    let recognition_body = baseline.requests().pop().expect("recognition request").body;

    let recording = root.join("recording");
    let listener = Listener::answering(automatic).expect("recording listener");
    let recorded = spawn(&["recognize", "--url", listener.base(), "--model", "local-1", "--record", &recording.to_string_lossy(), "--no-cache", "--relation", "works_for=person:organization"], &[("THINKTHEN_API_KEY", "key")], b"Ada met Acme.").expect("record");
    assert_eq!(recorded.status.code(), Some(0));
    let requests = listener.requests();
    assert_eq!(requests.first().expect("recognition request").body, recognition_body);
    let relation = requests.iter().find(|request| serde_json::from_slice::<Value>(&request.body).unwrap()["state"].is_object()).expect("relation request");
    let digest = crate::support::digest(listener.url(), &relation.body);
    let mut old: Value = serde_json::from_slice(&relation.body).unwrap();
    old["state"] = Value::String("Ada met Acme.".to_owned());
    let old_digest = crate::support::digest(listener.url(), &serde_json::to_vec(&old).unwrap());
    assert_ne!(digest, old_digest);
    assert!(recording.join(format!("{digest}.json")).is_file());

    let replayed = spawn(&["recognize", "--url", listener.base(), "--model", "local-1", "--replay", &recording.to_string_lossy(), "--no-cache", "--relation", "works_for=person:organization"], &[], b"Ada met Acme.").expect("replay");
    assert_eq!(replayed.status.code(), Some(0));
    assert_eq!(replayed.stdout, recorded.stdout);
    assert!(listener.requests().is_empty());

    let cache = root.join("cache");
    let arguments = ["recognize", "--url", listener.base(), "--model", "local-1", "--cache", &cache.to_string_lossy(), "--relation", "works_for=person:organization"];
    let filled = spawn(&arguments, &[("THINKTHEN_API_KEY", "key")], b"Ada met Acme.").expect("fill cache");
    assert_eq!(filled.status.code(), Some(0));
    let _sent = listener.requests();
    let cached = spawn(&arguments, &[], b"Ada met Acme.").expect("cached");
    assert_eq!(cached.status.code(), Some(0));
    assert_eq!(cached.stdout, filled.stdout);
    assert!(listener.requests().is_empty());
    assert!(cache.join(format!("{digest}.json")).is_file());
}

#[test]
fn exact_profile_keeps_under_budget_bytes_and_one_byte_less_falls_back_to_h() {
    let long = "z".repeat(300);
    let text = format!("P1{long} x P2{long} x P3{long} x P4{long} x O1{long} x O2{long} x O3{long} x O4{long} x O5{long}");
    let baseline = Listener::answering(automatic).expect("baseline");
    let output = run(&baseline, &["--relation", "works=person:organization"], text.as_bytes());
    assert_eq!(output.status.code(), Some(0));
    let bodies = baseline.requests().into_iter().map(|request| request.body).collect::<Vec<_>>();
    let largest = bodies.iter().map(Vec::len).max().unwrap();
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let exact = root.join("recognize-exact-bytes.json");
    fs::write(&exact, format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"exact","max_request_bytes":{largest}}}"#)).unwrap();
    let listener = Listener::answering(automatic).expect("exact");
    let output = run(&listener, &["--profile", &exact.to_string_lossy(), "--relation", "works=person:organization"], text.as_bytes());
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(listener.requests().into_iter().map(|request| request.body).collect::<Vec<_>>(), bodies);

    let one = root.join("recognize-one-choice.json");
    fs::write(&one, r#"{"schema":"thinkthen.backend-profile/1","name":"one-choice","max_questions":1}"#).unwrap();
    let listener = Listener::answering(automatic).expect("one choice");
    let output = run(&listener, &["--profile", &one.to_string_lossy(), "--relation", "works=person:organization"], text.as_bytes());
    assert_eq!(output.status.code(), Some(0));
    let choice = listener.requests().into_iter().find(|request| String::from_utf8_lossy(&request.body).contains("fills the blank")).expect("choice request").body;
    let fallback = root.join("recognize-choice-fallback.json");
    fs::write(&fallback, format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"fallback","max_request_bytes":{}}}"#, choice.len() - 1)).unwrap();
    let listener = Listener::answering(automatic).expect("fallback");
    let output = run(&listener, &["--profile", &fallback.to_string_lossy(), "--relation", "works=person:organization"], text.as_bytes());
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let last: Value = serde_json::from_slice(&listener.requests().last().unwrap().body).unwrap();
    assert!(last["questions"].as_object().unwrap().values().all(|question| question["type"] == "noul"));
}

#[test]
fn two_hundred_fifty_five_choice_options_stay_on_the_choice_path() {
    let mut names = (0..255).map(|place| format!("P{place}")).collect::<Vec<_>>();
    names.extend((0..254).map(|place| format!("O{place}")));
    let text = names.join(" x ");
    let profile = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-255.json");
    fs::write(&profile, r#"{"schema":"thinkthen.backend-profile/1","name":"fifty","max_questions":50}"#).unwrap();
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(&listener, &["--profile", &profile.to_string_lossy(), "--relation", "works=person:organization"], text.as_bytes());
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let relations = listener.requests().into_iter().filter(|request| String::from_utf8_lossy(&request.body).contains("fills the blank")).collect::<Vec<_>>();
    assert_eq!(relations.len(), 6);
    let relation = relations.first().unwrap();
    let request: Value = serde_json::from_slice(&relation.body).unwrap();
    let questions = request["questions"].as_object().unwrap();
    assert_eq!(relations.iter().map(|request| serde_json::from_slice::<Value>(&request.body).unwrap()["questions"].as_object().unwrap().len()).sum::<usize>(), 255);
    assert!(questions.values().all(|question| question["criteria"].as_object().unwrap().len() == 255));
}

#[test]
fn runtime_option_limit_falls_back_per_expanded_concrete_relation() {
    let profile = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-three-options.json");
    fs::write(&profile, r#"{"schema":"thinkthen.backend-profile/1","name":"three","max_options":3}"#).unwrap();
    let listener = Listener::answering(automatic).expect("listener");
    let output = run(&listener, &["--profile", &profile.to_string_lossy(), "--relation", "links=person:*"], b"P1 x P2 x P3 x P4 x O1 x O2 x Town1 x Town2 x Town3");
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let relations = listener.requests().into_iter().filter_map(|request| {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        body["state"]["relation"].is_object().then_some(body)
    }).collect::<Vec<_>>();
    let methods = relations.iter().map(|body| {
        let relation = &body["state"]["relation"];
        let method = if body["questions"].as_object().unwrap().values().all(|question| question["type"] == "choice") { "choice" } else { "yes_no" };
        (relation["source"].as_str().unwrap(), relation["target"].as_str().unwrap(), method)
    }).collect::<Vec<_>>();
    assert_eq!(methods, [("person", "person", "yes_no"), ("person", "organization", "choice"), ("person", "place", "yes_no")]);
}

#[test]
fn unsplittable_choice_bytes_fall_back_per_expanded_concrete_relation() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let one = root.join("recognize-wildcard-one-question.json");
    fs::write(&one, r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#).unwrap();
    let long = "z".repeat(120);
    let text = format!("P1 x P2 x P3 x P4 x O1 x O2 x Town1{long} x Town2{long} x Town3{long}");
    let baseline = Listener::answering(automatic).expect("baseline");
    let output = run(&baseline, &["--profile", &one.to_string_lossy(), "--relation", "links=person:*"], text.as_bytes());
    assert_eq!(output.status.code(), Some(0));
    let baseline_requests = baseline.requests();
    let bodies = baseline_requests.iter().filter_map(|request| {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        (body["state"]["relation"]["target"] == "organization" && body["questions"].as_object().unwrap().values().all(|question| question["type"] == "choice")).then_some(request.body.len())
    }).collect::<Vec<_>>();
    let place_choices = baseline_requests.iter().filter_map(|request| {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        (body["state"]["relation"]["target"] == "place" && body["questions"].as_object().unwrap().values().all(|question| question["type"] == "choice")).then_some(request.body.len())
    }).collect::<Vec<_>>();
    let option_fallback = root.join("recognize-wildcard-option-fallback.json");
    fs::write(&option_fallback, r#"{"schema":"thinkthen.backend-profile/1","name":"option-fallback","max_questions":1,"max_options":3}"#).unwrap();
    let h_listener = Listener::answering(automatic).expect("H baseline");
    let output = run(&h_listener, &["--profile", &option_fallback.to_string_lossy(), "--relation", "links=person:*"], text.as_bytes());
    assert_eq!(output.status.code(), Some(0));
    let place_h = h_listener.requests().into_iter().filter_map(|request| {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        (body["state"]["relation"]["target"] == "place" && body["questions"].as_object().unwrap().values().all(|question| question["type"] == "noul")).then_some(request.body.len())
    }).collect::<Vec<_>>();
    let limit = bodies.iter().chain(&place_h).copied().max().expect("fitting request");
    assert!(place_choices.iter().all(|bytes| *bytes > limit));
    let profile = root.join("recognize-wildcard-byte-fallback.json");
    fs::write(&profile, format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"bytes","max_questions":1,"max_request_bytes":{limit}}}"#)).unwrap();
    let listener = Listener::answering(automatic).expect("fallback");
    let output = run(&listener, &["--profile", &profile.to_string_lossy(), "--relation", "links=person:*"], text.as_bytes());
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let methods = listener.requests().into_iter().filter_map(|request| {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        let target = body["state"]["relation"]["target"].as_str()?;
        let method = if body["questions"].as_object().unwrap().values().all(|question| question["type"] == "choice") { "choice" } else { "yes_no" };
        Some((target.to_owned(), method))
    }).collect::<Vec<_>>();
    assert!(methods.contains(&("organization".to_owned(), "choice")));
    assert!(methods.contains(&("place".to_owned(), "yes_no")));
}

#[test]
fn details_carry_strength_inputs_and_request_metadata() {
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("listener");
    let output = run(&listener, &["--details"], b"Ada met Acme.");
    let request = listener.requests().pop().expect("request");
    let expected = include_str!("../fixtures/recognize-detailed.json").replace("$URL", listener.url()).replace("$REQUEST", &crate::support::digest(listener.url(), &request.body));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
}

#[test]
fn failed_relation_question_prints_no_partial_entity_or_edge_object() {
    let wrong = r#"{"model":"local-1","marker":"PRIVATE-RESPONSE","answers":{"q1":{"type":"noul","noul":0.5}}}"#;
    let listener = Listener::serving(vec![Canned::ok(ANSWERED), Canned::ok(wrong)]).expect("listener");
    let output = run(&listener, &["--relation", "works_for=person:organization"], b"Ada met Acme.");
    assert_eq!(output.status.code(), Some(4));
    assert!(output.stdout.is_empty());
    assert_eq!(String::from_utf8_lossy(&output.stderr), "thinkthen: the reply was refused: the answer to question `q1` is not the shape the question asked for\n");
    assert!(!String::from_utf8_lossy(&output.stderr).contains("PRIVATE-RESPONSE"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-value"));
}

#[test]
fn concurrent_record_workers_print_in_input_order() {
    let release = Arc::new(Barrier::new(2));
    let listener = Listener::answering(move |_| Canned::ok(ANSWERED).after_release(release.clone())).expect("listener");
    let output = run(&listener, &["--lines", "--jobs", "2"], b"Ada met Acme.\nBob met Corp.\n");
    assert_eq!(output.status.code(), Some(0));
    let rows = String::from_utf8(output.stdout).expect("output text");
    assert_eq!(rows, concat!(r#"{"input":"Ada met Acme.","value":{"entities":[{"name":"Ada","kind":"person","start":0,"end":3,"strength":0.81},{"name":"Acme","kind":"organization","start":8,"end":12,"strength":0.72}]}}"#, "\n", r#"{"input":"Bob met Corp.","value":{"entities":[{"name":"Bob","kind":"person","start":0,"end":3,"strength":0.81},{"name":"Corp","kind":"organization","start":8,"end":12,"strength":0.72}]}}"#, "\n"));
    assert_eq!(listener.peak(), 2);
}

#[test]
fn an_impossible_profile_sends_nothing() {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-tiny-profile.json");
    fs::write(&path, r#"{"schema":"thinkthen.backend-profile/1","name":"tiny-recognize","max_evidence_bytes":2}"#).expect("profile");
    let listener = Listener::answering(|_| Canned::ok(ANSWERED)).expect("listener");
    let output = spawn(&["recognize", "--profile", &path.to_string_lossy(), "--url", listener.base(), "--no-cache"], &[], b"Ada met Acme.").expect("command");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
}

#[test]
fn all_forty_harvest_cases_replay_without_a_key_or_network() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/recognize-225");
    let cases: Vec<Value> = serde_json::from_slice(&fs::read(root.join("cases.json")).expect("read mechanically adapted cases")).expect("parse mechanically adapted cases");
    assert_eq!(cases.len(), 40);
    assert_eq!(cases.iter().filter(|case| !case["divergence"].is_null()).count(), 10);
    for case in cases {
        let id = case["id"].as_str().expect("case id");
        let replay = root.join(id);
        let output = spawn(&["recognize", "--url", "https://api.typesafe.ai/v1", "--replay", replay.to_str().expect("fixture path"), "--kind", "PER=Part of a person's name.", "--kind", "ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.", "--kind", "LOC=Part of the name of a place: a country, region, city, or geographic feature.", "--kind", "MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work."], &[], case["text"].as_str().expect("case text").as_bytes()).expect("replay command");
        assert_eq!(output.status.code(), Some(0), "{}: {}", id, String::from_utf8_lossy(&output.stderr));
        let actual: Value = serde_json::from_slice(&output.stdout).expect("recognize output");
        let actual = actual["entities"].as_array().expect("actual entities");
        let expected = case["expected"]["entities"].as_array().expect("expected entities");
        assert_eq!(actual.len(), expected.len(), "{id}");
        for (actual, expected) in actual.iter().zip(expected) {
            for field in ["name", "kind", "start", "end"] {
                assert_eq!(actual[field], expected[field], "{id} {field}");
            }
            assert_eq!(actual["strength"].as_f64(), expected["strength"].as_f64(), "{id}");
        }
        assert!(fs::read_dir(replay).unwrap().filter_map(Result::ok).all(|entry| !fs::read_to_string(entry.path()).unwrap().contains("secret-value")));
    }
}

/// Ticket 0132: 64,000 bytes of text, whose one loopback reply passes 1 MiB, keep it.
#[test]
fn text_of_64000_bytes_keeps_its_reply() {
    let backend = conformance_backend::Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let text = format!("{}.", &"word ".repeat(12_800)[..63_999]);
    let arguments = ["recognize", "--kind", "P=a", "--kind", "O=b", "--url", &base, "--no-cache"];
    let output = spawn(&arguments, &[("THINKTHEN_API_KEY", "secret-value")], text.as_bytes())
        .expect("command");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert_eq!(output.status.code(), Some(0));
    let expected = format!(
        r#"{{"entities":[{{"name":"{text}","kind":"P","start":0,"end":64000,"strength":0.81}}]}}"#
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected + "\n");
    assert_eq!(backend.count(), 1);
}
