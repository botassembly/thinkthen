//! Keep error codes and bounded prose, never raw bodies or header dumps.

use super::Key;
use crate::core::{Json, adapters::ApiType};
use crate::engine::error::{Error, ProviderDiagnostic};
use base64::{Engine as _, engine::general_purpose};

const MAX_MESSAGE_CHARS: usize = 320;

pub(super) fn decorate(
    error: Error,
    body: Option<&[u8]>,
    key: &Key,
    api: ApiType,
    plain: bool,
    request: &[u8],
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
    let message = parsed
        .as_ref()
        .and_then(|body| {
            let details = body.member("error").or_else(|| body.member("detail"));
            details
                .and_then(|value| {
                    value
                        .member("message")
                        .and_then(Json::as_str)
                        .or_else(|| value.as_str())
                })
                .or_else(|| body.member("message").and_then(Json::as_str))
        })
        .or_else(|| {
            plain
                .then_some(body)
                .flatten()
                .and_then(|bytes| std::str::from_utf8(bytes).ok())
        })
        .filter(|text| !request_echo(text, request))
        .and_then(|text| safe_message(text, key));
    if !quota && message.is_none() {
        return error;
    }
    Error::Provider {
        cause: Box::new(error),
        diagnostic: ProviderDiagnostic {
            backend: api.provider_name(),
            quota,
            message,
        },
    }
}

fn safe_message(text: &str, key: &Key) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let forbidden = [
        "bearer ",
        "authorization=",
        "api_key=",
        "api-key=",
        "apikey=",
        "password=",
        "credential=",
        "secret=",
        "key hash",
        "key fingerprint",
        "sha256:",
    ];
    if text.trim().is_empty()
        || forbidden.iter().any(|word| lower.contains(word))
        || header_lines(text)
        || text
            .chars()
            .any(|character| character.is_control() && !character.is_whitespace())
        || text.contains(['{', '}', '[', ']', '<', '>'])
        || text
            .split_whitespace()
            .any(|word| opaque_token(word) || word.split([':', '=', '.']).any(opaque_token))
        || key_echo(text, key)
    {
        return None;
    }
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut chars = normalized.chars();
    let mut message: String = chars.by_ref().take(MAX_MESSAGE_CHARS).collect();
    if chars.next().is_some() {
        message.push('…');
    }
    Some(message)
}

fn opaque_token(word: &str) -> bool {
    let word =
        word.trim_matches(|ch: char| matches!(ch, '.' | ',' | ':' | ';' | '\'' | '"' | '(' | ')'));
    word.len() >= 32
        && word
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_-/+=".contains(&byte))
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

// Short generic input words collide with ordinary provider prose. Longer exact
// literals, including decoded strings inside framed JSON, must stay private.
fn reflected_literal(value: &str, text: &str) -> bool {
    value.chars().count() >= 8 && text.contains(value)
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
        && (text.contains(key.as_str())
            || text
                .split(|ch: char| !ch.is_ascii_alphanumeric() && !"_-/+=%".contains(ch))
                .any(|word| {
                    encoded_key(word, key)
                        || word.split(['/', '=']).any(|part| encoded_key(part, key))
                }))
}

fn encoded_key(word: &str, key: &Key) -> bool {
    let bytes = key.as_str().as_bytes();
    if percent_decoded(word)
        .is_some_and(|decoded| decoded.windows(bytes.len()).any(|part| part == bytes))
    {
        return true;
    }
    for decoder in [
        general_purpose::STANDARD,
        general_purpose::STANDARD_NO_PAD,
        general_purpose::URL_SAFE,
        general_purpose::URL_SAFE_NO_PAD,
    ] {
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

/// A single HTTP field name before a colon is ambiguous untrusted prose.
fn header_lines(text: &str) -> bool {
    text.lines().any(|line| {
        let Some((name, _)) = line.split_once(':') else {
            return false;
        };
        let name = name.trim();
        !name.is_empty()
            && name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte))
    })
}
