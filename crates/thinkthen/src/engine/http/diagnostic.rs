//! Carry only provider error identifiers from known fields, never provider prose.

use super::{Exchange, Key};
use crate::core::{Json, adapters::ApiType};
use crate::engine::error::{Error, ProviderDiagnostic};
use base64::{Engine as _, engine::general_purpose};

const BASE64_ENCODINGS: [general_purpose::GeneralPurpose; 4] = [
    general_purpose::STANDARD,
    general_purpose::STANDARD_NO_PAD,
    general_purpose::URL_SAFE,
    general_purpose::URL_SAFE_NO_PAD,
];

pub(super) fn decorate(
    error: Error,
    body: Option<&[u8]>,
    exchange: &Exchange<'_>,
    api: ApiType,
) -> Error {
    let parsed = body
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .and_then(|text| Json::parse(text).ok());
    let quota = matches!(error, Error::Status(402))
        || matches!(error, Error::Status(429))
            && parsed.as_ref().is_some_and(|body| {
                let details = body
                    .member("error")
                    .or_else(|| body.member("detail"))
                    .unwrap_or(body);
                ["type", "code", "error_type"].iter().any(|field| {
                    matches!(
                        details.member(field).and_then(Json::as_str),
                        Some(
                            "insufficient_quota"
                                | "insufficient_credits"
                                | "insufficient_balance"
                                | "credits_exhausted"
                                | "billing_hard_limit_reached"
                        )
                    )
                })
            });
    let ollama = matches!(error, Error::Status(400) | Error::TokenLimit)
        && super::send::ollama_address(exchange.url);
    let details = parsed.as_ref().map(|body| {
        body.member("error")
            .or_else(|| body.member("detail"))
            .unwrap_or(body)
    });
    let field = |names: &[&str]| {
        names.iter().find_map(|name| {
            details
                .and_then(|body| body.member(name))
                .and_then(Json::as_str)
                // Error identifiers are single bounded machine fields, not display prose.
                .filter(|text| (1..=128).contains(&text.len()))
                .filter(|text| {
                    text.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.')
                    })
                })
                .filter(|text| !key_echo(text, exchange.key))
                .filter(|text| !request_echo(text, exchange.body))
                .map(str::to_owned)
        })
    };
    let error_type = field(&["type", "error_type"]);
    let code = field(&["code"]);
    if !quota && !ollama && error_type.is_none() && code.is_none() {
        return error;
    }
    Error::Provider {
        cause: Box::new(error),
        diagnostic: ProviderDiagnostic {
            backend: api.provider_name(),
            quota,
            ollama,
            error_type,
            code,
        },
    }
}

/// Apply the submitted-body collision boundary before carrying untrusted prose.
pub(super) fn request_echo(text: &str, body: &[u8]) -> bool {
    !text.is_empty()
        && (body.windows(text.len()).any(|part| part == text.as_bytes())
            || std::str::from_utf8(body)
                .ok()
                .and_then(|body| Json::parse(body).ok())
                .is_some_and(|body| {
                    let literal = serde_json::to_string(text).unwrap_or_default();
                    submitted_text(&body, text, &literal)
                }))
}

fn submitted_text(value: &Json, text: &str, literal: &str) -> bool {
    match value {
        Json::String(value) => {
            value.contains(text)
                || !literal.is_empty() && value.contains(literal)
                || reflected_literal(value, text)
        }
        Json::Array(values) => values
            .iter()
            .any(|value| submitted_text(value, text, literal)),
        Json::Object(values) => values
            .iter()
            .any(|(_, value)| submitted_text(value, text, literal)),
        _ => false,
    }
}

// Submitted literals, including decoded strings inside framed JSON, stay private.
fn reflected_literal(value: &str, text: &str) -> bool {
    !value.is_empty() && text.contains(value)
        || value.char_indices().any(|(offset, character)| {
            character == '"'
                && serde_json::Deserializer::from_str(value.get(offset..).unwrap_or_default())
                    .into_iter::<String>()
                    .next()
                    .is_some_and(|value| value.is_ok_and(|value| reflected_literal(&value, text)))
        })
}

pub(super) fn key_echo(text: &str, key: &Key) -> bool {
    !key.as_str().is_empty()
        && key.as_str().len() <= text.len()
        && (text.contains(key.as_str())
            || BASE64_ENCODINGS
                .iter()
                .any(|decoder| text.contains(&decoder.encode(key.as_str())))
            || text.to_ascii_lowercase().contains(
                &key.as_str()
                    .bytes()
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<String>(),
            )
            || text
                .split(|ch: char| !ch.is_ascii_alphanumeric() && !"_-/+=%".contains(ch))
                .any(|word| {
                    encoded_key(word, key)
                        || word.split(['/', '=']).any(|part| encoded_key(part, key))
                }))
}

fn encoded_key(word: &str, key: &Key) -> bool {
    let bytes = key.as_str().as_bytes();
    // A malformed escape makes the token ambiguous; keep the entire prose private.
    if percent_decoded(word)
        .is_none_or(|decoded| decoded.windows(bytes.len()).any(|part| part == bytes))
    {
        return true;
    }
    for decoder in BASE64_ENCODINGS {
        if decoder
            .decode(word)
            .is_ok_and(|decoded| decoded.windows(bytes.len()).any(|part| part == bytes))
        {
            return true;
        }
    }
    let hex = word
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let pair = std::str::from_utf8(pair).ok()?;
            u8::from_str_radix(pair, 16).ok()
        })
        .collect::<Option<Vec<_>>>();
    hex.is_some_and(|decoded| decoded.windows(bytes.len()).any(|part| part == bytes))
}

/// Decode recognizable percent escapes without deriving or storing any key value.
fn percent_decoded(word: &str) -> Option<Vec<u8>> {
    let mut input = word.bytes();
    let mut output = Vec::new();
    while let Some(byte) = input.next() {
        if byte == b'%' {
            let high = char::from(input.next()?).to_digit(16)?;
            let low = char::from(input.next()?).to_digit(16)?;
            output.push(u8::try_from(high * 16 + low).ok()?);
        } else {
            output.push(byte);
        }
    }
    Some(output)
}
