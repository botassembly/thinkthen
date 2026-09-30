#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup or a missing field should stop the boundary test"
)]

use serde::Serialize;
use serde_json::value::RawValue;
use sha2::{Digest as _, Sha256};
use std::fs;
use std::io;
use std::path::Path;
use std::process::Output;

use crate::harness::spawn;

pub(crate) const DEFAULT_BASE: &str = "https://api.typesafe.ai/v1";
pub(crate) const DEFAULT_MODEL: &str = "jev-1.13.0";
pub(crate) const ENDPOINT_PATH: &str = "systemone";
/// The state of every quoted request without a context, by ADR 0111.
pub(crate) const QUOTED: &str = "Each question quotes the text it asks about.";
pub(crate) const MAX_RECORD_BYTES: usize = 16 * 1024 * 1024;

#[derive(Serialize)]
struct Request<'a> {
    state: &'a str,
    model: &'a str,
    questions: Questions<'a>,
}

#[derive(Serialize)]
struct Questions<'a> {
    q1: Decide<'a>,
}

#[derive(Serialize)]
struct Decide<'a> {
    #[serde(rename = "type")]
    kind: &'static str,
    instructions: &'a str,
}

#[derive(Serialize)]
struct Recording<'a> {
    schema: &'static str,
    adapter: &'static str,
    url: &'a str,
    request: Box<RawValue>,
    response: Box<RawValue>,
}

#[allow(
    clippy::expect_used,
    reason = "an infallible test fixture encoder should stop the test if its schema changes"
)]
/// The request `decide` sends for one text, in ADR 0111's quoted form: the
/// fixed sentence as the state, and the text quoted as a JSON string at the
/// head of the question.
pub(crate) fn encoded_decide(evidence: &str, model: &str, question: &str) -> Vec<u8> {
    let quoted = serde_json::to_string(evidence).expect("a text is JSON");
    serde_json::to_vec(&Request {
        state: QUOTED,
        model,
        questions: Questions {
            q1: Decide {
                kind: "noul",
                instructions: &format!("The text is {quoted}. {question}"),
            },
        },
    })
    .expect("the test request is JSON")
}

pub(crate) fn digest(url: &str, request: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"systemone\n");
    hasher.update(url.as_bytes());
    hasher.update(b"\n");
    hasher.update(request);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Every question key of one request body, in wire order, by ADR 0111
/// section 2: the SHA-256 of the adapter, the URL, the model, the state and
/// one question as the body carries them, joined by line feeds.
pub(crate) fn keys(url: &str, body: &[u8]) -> Vec<String> {
    #[derive(serde::Deserialize)]
    struct Parts<'a> {
        #[serde(borrow)]
        state: &'a RawValue,
        #[serde(borrow)]
        model: &'a RawValue,
        #[serde(borrow)]
        questions: std::collections::BTreeMap<String, &'a RawValue>,
    }
    let parts: Parts<'_> = serde_json::from_slice(body).expect("a request body");
    let mut questions: Vec<_> = parts
        .questions
        .into_iter()
        .map(|(name, question)| (name[1..].parse::<usize>().expect("a qN name"), question))
        .collect();
    questions.sort_by_key(|(place, _)| *place);
    questions
        .into_iter()
        .map(|(_, question)| {
            let joined = [
                "systemone",
                url,
                parts.model.get(),
                parts.state.get(),
                question.get(),
            ]
            .join("\n");
            Sha256::digest(joined.as_bytes())
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect()
        })
        .collect()
}

/// Every answer a live store in `folder` holds, as its fixture lines read,
/// sorted by key. `cache convert` reads a copy, so the folder stays as it was.
pub(crate) fn stored(folder: &Path) -> io::Result<Vec<serde_json::Value>> {
    static COPIES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let copy = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "stored-{}-{}",
        std::process::id(),
        COPIES.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ));
    let _removed = fs::remove_dir_all(&copy);
    fs::create_dir_all(&copy)?;
    fs::copy(
        folder.join("thinkthen.sqlite"),
        copy.join("thinkthen.sqlite"),
    )?;
    let converted = spawn(&["cache", "convert", &copy.to_string_lossy()], &[], b"")?;
    if converted.status.code() != Some(0) {
        return Err(io::Error::other(
            String::from_utf8_lossy(&converted.stderr).into_owned(),
        ));
    }
    let lines = fs::read_to_string(copy.join("thinkthen.jsonl"))?;
    fs::remove_dir_all(&copy)?;
    lines
        .lines()
        .map(serde_json::from_str::<serde_json::Value>)
        .filter(|line| line.as_ref().map_or(true, |line| line.get("key").is_some()))
        .collect::<Result<_, _>>()
        .map_err(io::Error::other)
}

/// Write the fixture `thinkthen.jsonl` a replay of this one request body
/// reads: its state, then each question's wire answer in `answers`, with the
/// reply's usage split evenly, the remainder to the earliest question.
pub(crate) fn plant_fixture(
    folder: &Path,
    url: &str,
    body: &[u8],
    answers: &[&str],
    usage: Option<(u64, u64)>,
) -> io::Result<Vec<String>> {
    #[derive(serde::Deserialize)]
    struct Parts<'a> {
        #[serde(borrow)]
        state: &'a RawValue,
        model: String,
        #[serde(borrow)]
        questions: std::collections::BTreeMap<String, &'a RawValue>,
    }
    let parts: Parts<'_> = serde_json::from_slice(body).map_err(io::Error::other)?;
    let mut questions: Vec<_> = parts
        .questions
        .iter()
        .map(|(name, question)| (name[1..].parse::<usize>().unwrap_or(0), question.get()))
        .collect();
    questions.sort_by_key(|(place, _)| *place);
    let keys = keys(url, body);
    let state = parts.state.get();
    let state_sha256: String = Sha256::digest(state.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let count = u64::try_from(keys.len()).unwrap_or(1).max(1);
    let share = |total: u64, place: u64| total / count + u64::from(place < total % count);
    let mut lines: Vec<(String, String)> = keys
        .iter()
        .zip(&questions)
        .zip(answers)
        .enumerate()
        .map(|(place, ((key, (_, question)), answer))| {
            let place = u64::try_from(place).unwrap_or(0);
            let line = serde_json::json!({
                "key": key,
                "url": url,
                "model": parts.model,
                "state": state_sha256,
                "question": question,
                "answer": answer,
                "answered_by": parts.model,
                "input_tokens": usage.map(|(input, _)| share(input, place)),
                "output_tokens": usage.map(|(_, output)| share(output, place)),
                "taken_at": 0,
                "origin": "live",
            });
            (key.clone(), line.to_string())
        })
        .collect();
    lines.sort();
    let mut text = serde_json::json!({"sha256": state_sha256, "state": state}).to_string();
    text.push('\n');
    for (_, line) in lines {
        text.push_str(&line);
        text.push('\n');
    }
    fs::create_dir_all(folder)?;
    fs::write(folder.join("thinkthen.jsonl"), text)?;
    Ok(keys)
}

pub(crate) fn plant_recording(
    folder: &Path,
    url: &str,
    request: &[u8],
    response: &str,
) -> Option<String> {
    let name = format!("{}.json", digest(url, request));
    let entry = Recording {
        schema: "thinkthen.recording/1",
        adapter: "systemone",
        url,
        request: RawValue::from_string(String::from_utf8(request.to_vec()).ok()?).ok()?,
        response: RawValue::from_string(response.to_owned()).ok()?,
    };
    let mut written = serde_json::to_string_pretty(&entry).ok()?;
    written.push('\n');
    fs::create_dir_all(folder).ok()?;
    fs::write(folder.join(&name), written).ok()?;
    Some(name)
}

/// Run `decide` against one URL, with no environment but what the case names.
///
/// `key` is the value `THINKTHEN_API_KEY` holds, or `None` for a run with the
/// variable unset.
pub(crate) fn decide(
    base: &str,
    arguments: &[&str],
    key: Option<&str>,
    evidence: &str,
) -> io::Result<Output> {
    let asked = [
        "decide",
        "asks for a refund",
        "--url",
        base,
        "--model",
        "local-1",
    ];
    spawn(
        &[&asked[..], arguments].concat(),
        &key.map_or_else(Vec::new, |value| vec![("THINKTHEN_API_KEY", value)]),
        evidence.as_bytes(),
    )
}
