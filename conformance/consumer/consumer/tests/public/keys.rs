//! Independent specification/cache.md v2 oracle, shared by native and C consumers.
use serde_json::value::RawValue;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
/// Hash captured compact wire bytes and an explicitly supplied saved-reply model.
pub(crate) fn keys(url: &str, body: &[u8], answered_model: &str) -> Result<Vec<String>, String> {
    #[derive(serde::Deserialize)]
    struct Parts<'a> {
        #[serde(borrow)]
        state: &'a RawValue,
        #[serde(borrow)]
        model: &'a RawValue,
        #[serde(borrow)]
        questions: BTreeMap<String, &'a RawValue>,
    }
    let parts: Parts<'_> = serde_json::from_slice(body).map_err(|e| e.to_string())?;
    let reported = serde_json::to_string(answered_model).map_err(|e| e.to_string())?;
    let mut questions = Vec::new();
    for (name, question) in parts.questions {
        let ordinal = name
            .strip_prefix('q')
            .ok_or("no qN name")?
            .parse::<usize>()
            .map_err(|_| "no qN ordinal")?;
        questions.push((ordinal, question));
    }
    questions.sort_by_key(|(ordinal, _)| *ordinal);
    Ok(questions
        .into_iter()
        .map(|(_, question)| {
            let mut hash = Sha256::new();
            hash.update(b"thinkthen.question-key/2\0");
            for part in [
                "systemone",
                url,
                parts.model.get(),
                reported.as_str(),
                parts.state.get(),
                question.get(),
            ] {
                hash.update((part.len() as u64).to_be_bytes());
                hash.update(part.as_bytes());
            }
            hash.finalize()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect()
        })
        .collect())
}

/// The literal reported model of an independently saved fixture reply.
pub(crate) fn reported_model(response: &serde_json::Value) -> Result<&str, String> {
    response
        .get("model")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "no saved reported model".to_owned())
}

/// Original request-digest placeholders identify exchanges in the legacy corpus.
pub(crate) fn digest(url: &str, request: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(b"systemone\n");
    hash.update(url.as_bytes());
    hash.update(b"\n");
    hash.update(request);
    hash.finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
/// Map independent original fixture expectations to their owned endpoint's v2 keys.
pub(crate) fn fixture_keys(
    canonical: &str,
    served: &str,
    exchanges: &[serde_json::Value],
) -> Result<BTreeMap<String, serde_json::Value>, String> {
    exchanges
        .iter()
        .map(|exchange| {
            let request = exchange
                .get("request")
                .and_then(serde_json::Value::as_str)
                .ok_or("no saved request")?
                .as_bytes();
            let response = exchange.get("response").ok_or("no saved response")?;
            Ok((
                digest(canonical, request),
                serde_json::json!(keys(served, request, reported_model(response)?)?),
            ))
        })
        .collect()
}
