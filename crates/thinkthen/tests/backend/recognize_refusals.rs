//! Relation refusals after cached recognition answers.

use crate::harness::{Canned, Listener, spawn};
use serde_json::Value;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Output,
};

fn automatic(body: &[u8]) -> Canned {
    automatic_answer(body).unwrap_or_else(|| Canned::status(400, "{}"))
}

fn automatic_answer(body: &[u8]) -> Option<Canned> {
    let request: Value = serde_json::from_slice(body).ok()?;
    let answers = request
        .get("questions")?
        .as_object()?
        .iter()
        .map(|(name, question)| question_answer(question).map(|answer| (name.clone(), answer)))
        .collect::<Option<serde_json::Map<_, _>>>()?;
    Some(Canned::ok(
        &serde_json::json!({"model":"local-1","answers":answers}).to_string(),
    ))
}

fn question_answer(question: &Value) -> Option<Value> {
    if question.get("type")?.as_str()? == "noul" {
        return Some(serde_json::json!({"type":"noul","noul":0.9}));
    }
    let criteria = question.get("criteria")?.as_object()?;
    let instructions = question.get("instructions")?.as_str()?;
    let pick = answer_pick(criteria, instructions)?;
    let probabilities = criteria
        .keys()
        .map(|label| {
            (
                label.clone(),
                Value::from(f64::from(label.as_str() == pick)),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    Some(serde_json::json!({"type":"choice","choice":pick,"probabilities":probabilities}))
}

fn answer_pick<'a>(
    criteria: &'a serde_json::Map<String, Value>,
    instructions: &str,
) -> Option<&'a str> {
    if criteria.contains_key("IN") {
        return Some(detection_pick(instructions));
    }
    if criteria.contains_key("person") {
        return Some(kind_pick(instructions));
    }
    criteria
        .keys()
        .find(|label| label.starts_with('i'))
        .map_or(Some("none"), |label| Some(label.as_str()))
}

fn detection_pick(instructions: &str) -> &'static str {
    if instructions.contains("[[x]]") {
        "OUT"
    } else {
        "IN"
    }
}

fn kind_pick(instructions: &str) -> &'static str {
    if instructions.contains("[[O") {
        "organization"
    } else {
        "person"
    }
}

#[test]
fn impossible_relation_state_uses_cached_recognition_and_sends_nothing() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-relation-refusal-cache");
    let _removed = fs::remove_dir_all(&root);
    let listener = Listener::answering(automatic).expect("listener");
    let cache = root.to_string_lossy();
    let common = [
        "recognize",
        "person",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &cache,
    ];
    let filled =
        spawn(&common, &[("THINKTHEN_API_KEY", "key")], b"Ada x Grace").expect("fill cache");
    assert_eq!(
        filled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&filled.stderr)
    );
    assert_eq!(listener.requests().len(), 1);

    let profile = root.with_extension("profile.json");
    fs::write(&profile, r#"{"schema":"thinkthen.backend-profile/1","name":"source-only","max_evidence_bytes":11,"max_options":2}"#).expect("profile");
    let refused = spawn(
        &[
            "recognize",
            "person",
            "--relation",
            "knows=person:person",
            "--profile",
            &profile.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--cache",
            &cache,
        ],
        &[],
        b"Ada x Grace",
    )
    .expect("refusal");
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    assert_eq!(listener.requests().len(), 0);
    assert!(!String::from_utf8_lossy(&refused.stderr).contains("key"));
}

struct ByteFallbackFixture {
    root: PathBuf,
    cache: String,
    input: String,
    listener: Listener,
}

impl ByteFallbackFixture {
    fn new() -> io::Result<Self> {
        let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("recognize-byte-h-refusal");
        let cache = root.join("cache").to_string_lossy().into_owned();
        let long = "z".repeat(1_200);
        let input = format!("P1{long} x P2{long} x O1{long} x O2{long}");
        let listener = Listener::answering(automatic)?;
        Ok(Self {
            root,
            cache,
            input,
            listener,
        })
    }

    fn reset(&self) -> io::Result<()> {
        match fs::remove_dir_all(&self.root) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        fs::create_dir_all(&self.root)
    }

    fn profile(&self, name: &str, contents: &str) -> io::Result<PathBuf> {
        let path = self.root.join(name);
        fs::write(&path, contents)?;
        Ok(path)
    }

    fn run(&self, profile: &Path, relation: bool, keyed: bool) -> io::Result<Output> {
        let mut arguments = vec![
            "recognize".to_owned(),
            "person".to_owned(),
            "organization".to_owned(),
            "--profile".to_owned(),
            profile.to_string_lossy().into_owned(),
            "--url".to_owned(),
            self.listener.base().to_owned(),
            "--model".to_owned(),
            "local-1".to_owned(),
            "--cache".to_owned(),
            self.cache.clone(),
        ];
        if relation {
            arguments.extend([
                "--relation".to_owned(),
                "works=person:organization".to_owned(),
            ]);
        }
        let arguments = arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let key = [("THINKTHEN_API_KEY", "PRIVATE-KEY")];
        let environment = if keyed { &key[..] } else { &[] };
        spawn(&arguments, environment, self.input.as_bytes())
    }

    fn recognition_limit(&self, profile: &Path) -> io::Result<usize> {
        let output = self.run(profile, false, true)?;
        require_success(&output, "recognition cache fill")?;
        self.listener
            .requests()
            .iter()
            .map(|request| request.body.len())
            .max()
            .ok_or_else(|| io::Error::other("recognition sent no requests"))
    }

    fn relation_bytes(&self, profile: &Path, question_type: &str) -> io::Result<usize> {
        let output = self.run(profile, true, true)?;
        require_success(&output, "relation probe")?;
        let requests = self.listener.requests();
        if !requests
            .iter()
            .all(|request| questions_have_type(&request.body, question_type))
        {
            return Err(io::Error::other(format!(
                "relation probe did not use only {question_type} questions"
            )));
        }
        requests
            .first()
            .map(|request| request.body.len())
            .ok_or_else(|| io::Error::other("relation probe sent no requests"))
    }
}

fn require_success(output: &Output, phase: &str) -> io::Result<()> {
    if output.status.success() {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "{phase} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    )))
}

fn questions_have_type(body: &[u8], expected: &str) -> bool {
    serde_json::from_slice::<Value>(body)
        .ok()
        .and_then(|request| request.get("questions")?.as_object().cloned())
        .is_some_and(|questions| {
            questions
                .values()
                .all(|question| question.get("type").and_then(Value::as_str) == Some(expected))
        })
}

#[test]
fn byte_fallback_refuses_oversized_h_after_cached_recognition_without_a_send() -> io::Result<()> {
    let fixture = ByteFallbackFixture::new()?;
    fixture.reset()?;
    let one = fixture.profile(
        "one-question.json",
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    )?;
    let limit = fixture.recognition_limit(&one)?;
    let choice_bytes = fixture.relation_bytes(&one, "choice")?;
    let h_profile = fixture.profile(
        "h.json",
        r#"{"schema":"thinkthen.backend-profile/1","name":"h","max_questions":1,"max_options":2}"#,
    )?;
    let h_bytes = fixture.relation_bytes(&h_profile, "noul")?;
    assert!(choice_bytes > limit);
    assert!(h_bytes > limit);
    assert_ne!(choice_bytes, h_bytes);

    let final_profile = fixture.profile(
        "final.json",
        &format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"final-byte","max_questions":1,"max_request_bytes":{limit}}}"#),
    )?;
    let refused = fixture.run(&final_profile, true, false)?;
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    assert_eq!(
        String::from_utf8(refused.stderr).map_err(io::Error::other)?,
        format!(
            "thinkthen: profile final-byte allows at most {limit} request bytes; this request has {h_bytes}\n"
        )
    );
    assert!(fixture.listener.requests().is_empty());
    Ok(())
}
