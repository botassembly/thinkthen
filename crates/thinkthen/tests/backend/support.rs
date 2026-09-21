use serde::Serialize;
use serde_json::value::RawValue;
use sha2::{Digest as _, Sha256};
use std::fs;
use std::path::Path;

pub(crate) const DEFAULT_BASE: &str = "https://api.typesafe.ai/v1";
pub(crate) const DEFAULT_MODEL: &str = "jev-latest";
pub(crate) const ENDPOINT_PATH: &str = "systemone";
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
pub(crate) fn encoded_decide(evidence: &str, model: &str, question: &str) -> Vec<u8> {
    serde_json::to_vec(&Request {
        state: evidence,
        model,
        questions: Questions {
            q1: Decide {
                kind: "noul",
                instructions: question,
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
