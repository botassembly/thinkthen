//! Unconfirmed proposals reach classification without changing boundary answers.
use super::*;
use serde_json::json;

fn judge(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let mut answers = serde_json::Map::new();
    for (id, q) in request["questions"].as_object().unwrap() {
        let labels = q["criteria"].as_object().unwrap();
        let pick = if labels.contains_key("BEGIN") {
            "OUT"
        } else if labels.contains_key("ENTITY") {
            "ENTITY"
        } else if labels.contains_key("person") {
            "person"
        } else {
            labels.keys().next().unwrap()
        };
        let probabilities: serde_json::Map<_, _> = labels
            .keys()
            .map(|label| (label.clone(), json!(if label == pick { 1.0 } else { 0.0 })))
            .collect();
        answers.insert(
            id.clone(),
            json!({"type":"choice","choice":pick,"probabilities":probabilities}),
        );
    }
    Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
}
const FLAGS: &[&str] = &[
    "--jsonl",
    "--field",
    "/body",
    "--seed-spans-field",
    "/seeds",
];

#[test]
fn missed_single_piece_is_classified_but_retains_the_boundary_strength_cut() {
    let listener = Listener::answering(judge).unwrap();
    let input = format!("{}\n", json!({"body":"Ada","seeds":[{"start":0,"end":3}]}));
    let output = run(
        &listener,
        &[FLAGS, &["--details"]].concat(),
        input.as_bytes(),
    );
    let detail: Value = serde_json::from_str(&stdout(&output)).unwrap();
    assert_eq!(listener.count(), 2);
    assert_eq!(detail["answer"]["names"][0]["kinds"]["ENTITY"], 1.0);
    assert_eq!(detail["value"]["entities"], json!([]));
    let baseline = plan_json(&run(&listener, &["--plan"], b"Ada"));
    let seeded = plan_json(&run(
        &listener,
        &[FLAGS, &["--plan"]].concat(),
        input.as_bytes(),
    ));
    assert_eq!(baseline["requests"], seeded["requests"]);
    assert_eq!(listener.count(), 2);
}

#[test]
fn invalid_later_seed_admission_sends_nothing_and_withholds_input() {
    let listener = Listener::answering(judge).unwrap();
    for seeds in [
        Value::Null,
        json!([{"start":1,"end":3}]),
        json!([{"start":0,"end":9}]),
        json!([{"start":0,"end":3,"kind":null}]),
        json!([{"start":0,"end":3,"kind":"secret-kind"}]),
        json!([{"start":-1,"end":3}]),
    ] {
        let input = format!(
            "{}\n{}\n",
            json!({"body":"Ada","seeds":[]}),
            json!({"body":"Bob","seeds":seeds})
        );
        let output = run(&listener, FLAGS, input.as_bytes());
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-kind"));
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn overlapping_untyped_seeds_have_two_question_bounds_each() {
    let listener = Listener::answering(judge).unwrap();
    let input = format!(
        "{}\n",
        json!({"body":"Ada Bob Cara","seeds":[{"start":0,"end":12},{"start":0,"end":3},{"start":4,"end":12},{"start":4,"end":7}]})
    );
    let plan = plan_json(&run(
        &listener,
        &[FLAGS, &["--plan"]].concat(),
        input.as_bytes(),
    ));
    assert_eq!(plan["name_requests_upper_bound"], 11);
    assert_eq!(listener.count(), 0);
}

#[test]
fn scalar_seed_offsets_deduplicate_and_match_native_request_bodies() {
    use thinkthen::{Engine, Request, RequestEnvironment, RequestOutcome};
    for (text, seeds, spans) in [
        (
            "😀 Ada",
            json!([{"start":0,"end":1},{"start":2,"end":5},{"start":2,"end":5}]),
            vec![(0, 1), (2, 5)],
        ),
        ("A\u{301}da", json!([{"start":0,"end":4}]), vec![(0, 4)]),
        (
            "\u{feff}\u{200b}Ada",
            json!([{"start":2,"end":5}]),
            vec![(2, 5)],
        ),
        (
            "Ada Bob Cara",
            json!([{"start":0,"end":12},{"start":0,"end":3},{"start":4,"end":7}]),
            vec![(0, 3), (0, 12), (4, 7)],
        ),
    ] {
        let listener = Listener::answering(judge).unwrap();
        let original = json!({"body":text,"seeds":seeds});
        let input = format!("{original}\n");
        let cli = json(&run(
            &listener,
            &[FLAGS, &["--details"]].concat(),
            input.as_bytes(),
        ));
        let actual: Vec<_> = cli["answer"]["names"]
            .as_array()
            .unwrap()
            .iter()
            .map(|name| {
                (
                    name["start"].as_u64().unwrap(),
                    name["end"].as_u64().unwrap(),
                )
            })
            .collect();
        assert_eq!(actual, spans);
        let sent = listener.requests();
        let request = Request::from_json(
            &json!({
                "schema":"thinkthen.request/1", "call":{
                    "function":"recognize", "question":{"kind":"definition","value":{"version":1,"recognize":{"kinds":{}}}},
                    "input":{"kind":"json","value":original},
                    "options":{"field":["/body"],"seed_spans_field":"/seeds"}
                }
            })
            .to_string(),
        )
        .unwrap()
        .admit()
        .unwrap();
        let engine = Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("local-1")
            .unwrap()
            .api_key("secret-value")
            .unwrap()
            .no_cache()
            .max_retries(0)
            .build()
            .unwrap();
        let RequestOutcome::Complete(_) = engine
            .execute_request(&request, RequestEnvironment::default())
            .unwrap()
        else {
            panic!("native recognition completes")
        };
        let native = listener.requests();
        assert_eq!(sent.len(), native.len());
        for (left, right) in sent.iter().zip(&native) {
            assert_eq!(left.body, right.body);
        }
    }
}

#[test]
fn hinted_kind_can_be_corrected_or_declined_and_nested_evidence_reaches_its_last_piece() {
    let text = "Ada Bob Cara Dan Eve Finn Gail Hugo Iris Jack Ken Lia Mona Ned Ora";
    let end = text.chars().count();
    let listener = Listener::answering(judge).unwrap();
    let input = format!(
        "{}\n",
        json!({"body":text,"seeds":[
            {"start":0,"end":end,"kind":"organization"},{"start":0,"end":3,"kind":"person"},
            {"start":0,"end":3,"kind":"organization"}
        ]})
    );
    let detail = json(&run(
        &listener,
        &[FLAGS, &KINDS, &["--details"]].concat(),
        input.as_bytes(),
    ));
    assert_eq!(detail["answer"]["names"].as_array().unwrap().len(), 2);
    assert_eq!(detail["answer"]["names"][1]["kinds"]["person"], 1.0);
    let sent = listener.requests();
    let stage = questions(&sent[1].body);
    assert!(
        stage
            .iter()
            .filter(|q| q["criteria"].get("person").is_some())
            .all(|q| q["criteria"].get("organization").is_some())
    );
    assert!(String::from_utf8_lossy(&sent[1].body).contains("Ora"));
    let refusing = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).unwrap();
        if request["questions"].as_object().unwrap().values().any(|q| q["criteria"].get("person").is_some()) {
            Canned::ok(&json!({"model":"local-1","answers":{"q1":{"type":"choice","choice":"none of these","probabilities":{"person":0.0,"organization":0.0,"none of these":1.0}}}}).to_string())
        } else { judge(body) }
    }).unwrap();
    let input = format!(
        "{}\n",
        json!({"body":"Ada","seeds":[{"start":0,"end":3,"kind":"person"}]})
    );
    let detail = json(&run(
        &refusing,
        &[FLAGS, &KINDS, &["--details"]].concat(),
        input.as_bytes(),
    ));
    assert_eq!(detail["answer"]["names"][0]["kinds"]["none of these"], 1.0);
    assert_eq!(detail["value"]["entities"], json!([]));
}

#[test]
fn native_seed_fallback_clearing_and_refusals_preserve_zero_send_admission() {
    use thinkthen::{Engine, Request, RequestEnvironment};
    let listener = Listener::answering(judge).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("local-1")
        .unwrap()
        .api_key("secret-value")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap();
    let base = json!({"schema":"thinkthen.request/1","call":{
        "function":"recognize","question":{"kind":"definition","value":{"version":1,"recognize":{"kinds":{}}}},
        "input":{"kind":"json","value":{"body":"Ada"}},
        "options":{"field":["/body"],"seed_spans_field":"/seeds","seed_spans":[{"start":0,"end":3}]}
    }});
    for (seeds, expected) in [(None, 2), (Some(json!([])), 1)] {
        let mut value = base.clone();
        if let Some(seeds) = seeds {
            value["call"]["input"]["value"]["seeds"] = seeds;
        }
        let before = listener.count();
        let request = Request::from_json(&value.to_string())
            .unwrap()
            .admit()
            .unwrap();
        engine
            .execute_request(&request, RequestEnvironment::default())
            .unwrap();
        assert_eq!(listener.count() - before, expected);
    }
    let before = listener.count();
    let mut cases = Vec::new();
    for seeds in [
        Value::Null,
        json!([{"start":0,"end":3,"kind":null}]),
        json!([{"start":0,"end":2}]),
        json!([{"start":0,"end":3,"kind":"private-marker"}]),
        json!([{"start":18446744073709551615u64,"end":3}]),
    ] {
        let mut value = base.clone();
        value["call"]["input"]["value"]["seeds"] = seeds;
        cases.push(value);
    }
    let mut invalid_pointer = base.clone();
    invalid_pointer["call"]["options"]["seed_spans_field"] = json!("seeds");
    cases.push(invalid_pointer);
    let mut null_fallback = base.clone();
    null_fallback["call"]["options"]["seed_spans"] = Value::Null;
    cases.push(null_fallback);
    let mut conflict = base.clone();
    conflict["call"]["input"] = json!({"kind":"records","items":[{"original":{"kind":"json","value":{"body":"Ada"}},"seed_spans":[]}]});
    cases.push(conflict);
    for value in cases {
        let outcome = Request::from_json(&value.to_string())
            .and_then(Request::admit)
            .and_then(|request| engine.execute_request(&request, RequestEnvironment::default()));
        let error = outcome.unwrap_err();
        assert_eq!(error.kind(), thinkthen::ErrorKind::Usage);
        assert!(!format!("{error:?}").contains("private-marker"));
    }
    assert_eq!(listener.count(), before);
}

#[test]
fn seeds_preserve_boundary_bytes_and_replay_their_classification_identity() {
    let listener = Listener::answering(judge).unwrap();
    let baseline = run(&listener, &["--details"], b"Ada");
    let boundary = listener.requests()[0].body.clone();
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("recognition-seeds-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let recording = root.join("recording").to_string_lossy().into_owned();
    let cache = root.join("cache").to_string_lossy().into_owned();
    let seeded = format!("{}\n", json!({"body":"Ada","seeds":[{"start":0,"end":3}]}));
    let recorded = local(
        &listener,
        &[FLAGS, &["--details", "--record", &recording, "--no-cache"]].concat(),
        Some("secret-value"),
        seeded.as_bytes(),
    );
    let answer = json(&recorded);
    let sent = listener.requests();
    assert_eq!(sent.len(), 2);
    assert_eq!(sent[0].body, boundary);
    assert_eq!(
        json(&baseline)["answer"]["pieces"],
        answer["answer"]["pieces"]
    );
    let cached = local(
        &listener,
        &[FLAGS, &["--details", "--cache", &cache]].concat(),
        Some("secret-value"),
        seeded.as_bytes(),
    );
    assert_eq!(json(&cached)["answer"], answer["answer"]);
    assert_eq!(listener.count(), 5);
    let before = listener.count();
    for mode in ["--cache", "--replay"] {
        let path = if mode == "--cache" {
            &cache
        } else {
            &recording
        };
        let replayed = local(
            &listener,
            &[FLAGS, &["--details", mode, path]].concat(),
            None,
            seeded.as_bytes(),
        );
        let replayed = json(&replayed);
        assert_eq!(replayed["answer"], answer["answer"]);
        assert_eq!(replayed["meta"]["requests"], answer["meta"]["requests"]);
        assert_eq!(
            replayed["meta"]["question_sha256"],
            answer["meta"]["question_sha256"]
        );
    }
    assert_eq!(listener.count(), before);
}
