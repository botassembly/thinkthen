//! Pair refusals after cached recognition answers.

use crate::harness::Listener;
use crate::recognize::{automatic, local, questions, stdout};
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Output,
};

fn dense_names(count: usize) -> String {
    (0..count)
        .map(|place| format!("P{place:03}"))
        .collect::<Vec<_>>()
        .join(" x ")
}

#[test]
fn relation_plan_limits_refuse_after_the_real_recognition_sends() {
    for (names, expected) in [
        (
            64,
            "thinkthen: recognize: 64 distinct relation-eligible names would ask 4032 relation questions, over the limit of 4000; reduce names or relation rules, or split the input\n",
        ),
        (
            256,
            "thinkthen: recognize: 256 distinct relation-eligible names exceed the limit of 255; reduce names or split the input\n",
        ),
    ] {
        let listener = Listener::answering(automatic).expect("listener");
        let input = dense_names(names);
        let output = local(
            &listener,
            &["person", "--relation", "near=person:person", "--no-cache"],
            Some("key"),
            input.as_bytes(),
        );
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8(output.stderr).expect("diagnostic"),
            expected
        );
        let sent = listener.requests();
        assert_eq!(sent.len(), if names == 64 { 8 } else { 26 });
        assert!(sent.iter().all(|request| {
            questions(&request.body)
                .iter()
                .all(|question| question["type"] != "noul")
        }));
    }
}

#[test]
fn many_names_without_a_possible_pair_keep_empty_relations() {
    let listener = Listener::answering(automatic).expect("listener");
    let input = dense_names(256);
    let output = local(
        &listener,
        &[
            "person",
            "place",
            "--relation",
            "near=person:place",
            "--no-cache",
        ],
        Some("key"),
        input.as_bytes(),
    );
    let value: serde_json::Value = serde_json::from_str(&stdout(&output)).expect("result");
    assert_eq!(value["entities"].as_array().map(Vec::len), Some(256));
    assert_eq!(value["relations"], serde_json::json!([]));
    assert!(listener.requests().iter().all(|request| {
        questions(&request.body)
            .iter()
            .all(|question| question["type"] != "noul")
    }));

    let no_rule = local(
        &listener,
        &["person", "--no-cache"],
        Some("key"),
        b"Ada x Grace",
    );
    let no_rule: serde_json::Value = serde_json::from_str(&stdout(&no_rule)).expect("result");
    assert!(no_rule.get("relations").is_none());
}

#[test]
fn public_relation_refusal_keeps_the_spent_recognition_facts() {
    let listener = Listener::answering(automatic).expect("listener");
    let engine = thinkthen::Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("key")
        .expect("key")
        .no_cache()
        .build()
        .expect("engine");
    let asked = thinkthen::Recognize::builder()
        .kind(thinkthen::Kind::new("person", None).expect("kind"))
        .expect("kind")
        .relation(thinkthen::RelationRule::one_way("near", "person", "person").expect("rule"))
        .expect("relation")
        .build()
        .expect("recognize");
    let error = engine
        .recognize(&asked, &dense_names(64))
        .expect_err("limit");
    assert_eq!(error.kind(), thinkthen::ErrorKind::Usage);
    assert_eq!(
        error.detail().message(),
        "64 distinct relation-eligible names would ask 4032 relation questions, over the limit of 4000; reduce names or relation rules, or split the input"
    );
    let facts = error.facts().expect("completed stage facts");
    assert_eq!((facts.records(), facts.requests_sent()), (0, 8));
    assert_eq!(
        (facts.input_tokens(), facts.output_tokens()),
        (Some(80), Some(16))
    );
    assert_eq!(listener.requests().len(), 8);
}

#[test]
fn impossible_relation_state_uses_cached_recognition_and_sends_nothing() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-relation-refusal-cache");
    let _removed = fs::remove_dir_all(&root);
    let listener = Listener::answering(automatic).expect("listener");
    let cache = root.to_string_lossy();
    let filled = local(
        &listener,
        &["person", "--cache", &cache],
        Some("key"),
        b"Ada x Grace",
    );
    stdout(&filled);
    assert_eq!(listener.requests().len(), 2);

    let profile = root.with_extension("profile.json");
    fs::write(&profile, r#"{"schema":"thinkthen.backend-profile/1","name":"source-only","max_evidence_bytes":11,"max_options":2}"#).expect("profile");
    let profile = profile.to_string_lossy();
    let options = [
        "person",
        "--relation",
        "knows=person:person",
        "--profile",
        &profile,
        "--cache",
        &cache,
    ];
    let refused = local(&listener, &options, None, b"Ada x Grace");
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    assert_eq!(listener.requests().len(), 0);
    assert!(!String::from_utf8_lossy(&refused.stderr).contains("key"));
}

struct Fixture {
    root: PathBuf,
    cache: String,
    input: String,
    listener: Listener,
}

impl Fixture {
    fn new() -> io::Result<Self> {
        let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-pair-byte-refusal");
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        fs::create_dir_all(&root)?;
        let long = "z".repeat(1_200);
        Ok(Self {
            cache: root.join("cache").to_string_lossy().into_owned(),
            root,
            input: ["P1", "P2", "O1", "O2"]
                .map(|name| format!("{name}{long}"))
                .join(" a b c d e f g "),
            listener: Listener::answering(automatic)?,
        })
    }

    fn profile(&self, name: &str, contents: &str) -> io::Result<PathBuf> {
        let path = self.root.join(name);
        fs::write(&path, contents)?;
        Ok(path)
    }

    fn run(&self, profile: &Path, relation: bool, keyed: bool) -> io::Result<Output> {
        let profile = profile.to_string_lossy();
        let mut options = vec![
            "person",
            "organization",
            "--profile",
            &profile,
            "--cache",
            &self.cache,
        ];
        if relation {
            options.extend(["--relation", "works=person:organization"]);
        }
        let key = keyed.then_some("PRIVATE-KEY");
        let output = local(&self.listener, &options, key, self.input.as_bytes());
        if keyed && !output.status.success() {
            return Err(io::Error::other(
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ));
        }
        Ok(output)
    }

    fn largest_sent(&self) -> io::Result<usize> {
        self.listener
            .requests()
            .iter()
            .map(|request| request.body.len())
            .max()
            .ok_or_else(|| io::Error::other("nothing was sent"))
    }
}

/// The pair request passes the byte limit every cached recognition request
/// meets, so the run refuses it by name and sends nothing.
#[test]
fn a_pair_request_over_the_byte_limit_refuses_after_cached_recognition_without_a_send()
-> io::Result<()> {
    let fixture = Fixture::new()?;
    let one = fixture.profile(
        "one-question.json",
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    )?;
    fixture.run(&one, false, true)?;
    let limit = fixture.largest_sent()?;
    fixture.run(&one, true, true)?;
    let pair_bytes = fixture.largest_sent()?;
    assert!(pair_bytes > limit);

    let bytes = fixture.profile(
        "bytes.json",
        &format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"final-byte","max_questions":1,"max_request_bytes":{limit}}}"#),
    )?;
    let refused = fixture.run(&bytes, true, false)?;
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    assert_eq!(
        String::from_utf8(refused.stderr).map_err(io::Error::other)?,
        format!(
            "thinkthen: profile final-byte allows at most {limit} request bytes; this request has {pair_bytes}\n"
        )
    );
    assert!(fixture.listener.requests().is_empty());
    Ok(())
}
