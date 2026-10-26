//! Bounded, credential-safe diagnostics from untrusted backend bodies.

use std::io::Read;

const CAPTURE_BYTES: u64 = 16 * 1024;
const EXCERPT_CHARACTERS: usize = 200;
const MASK: &str = "[redacted]";

pub(super) fn capture(body: &mut ureq::Body) -> Vec<u8> {
    let mut bytes = Vec::new();
    // Keep a readable prefix even if a peer closes early. Body errors must not
    // replace the HTTP status, and take() never waits for the rest of a large body.
    let _read = body.as_reader().take(CAPTURE_BYTES).read_to_end(&mut bytes);
    bytes
}

pub(super) fn clean(body: &[u8], key: &str) -> String {
    let text: String = String::from_utf8_lossy(body)
        .chars()
        .filter(|ch| !ch.is_control() && !matches!(ch, '\u{200b}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2060}'..='\u{206f}' | '\u{feff}'))
        .collect();
    let key: String = key.chars().filter(|ch| !ch.is_control()).collect();
    let mut text = text.replace(&key, MASK);
    // The byte capture may end inside the known credential. Remove that suffix
    // before the display limit can expose a credential fragment.
    if body.len() as u64 == CAPTURE_BYTES
        && let Some(start) = text
            .char_indices()
            .map(|(start, _)| start)
            .find(|start| key.starts_with(text.get(*start..).unwrap_or_default()))
    {
        text.truncate(start);
        text.push_str(MASK);
    }
    let mut cleaned = String::new();
    let mut token = String::new();
    let mut secret = false;
    for (index, ch) in text.char_indices() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '/' | '+' | '.') {
            token.push(ch);
        } else {
            let assignment = !token.is_empty()
                && text
                    .get(index..)
                    .unwrap_or_default()
                    .trim_start_matches(|ch: char| ch.is_whitespace() || matches!(ch, '\'' | '"'))
                    .starts_with([':', '=']);
            redact(&mut cleaned, &mut token, &mut secret, assignment);
            cleaned.push(ch);
        }
    }
    redact(&mut cleaned, &mut token, &mut secret, false);
    cleaned.trim().chars().take(EXCERPT_CHARACTERS).collect()
}

fn redact(output: &mut String, token: &mut String, secret: &mut bool, assignment: bool) {
    if token.is_empty() {
        return;
    }
    let label = (assignment && token.eq_ignore_ascii_case("authorization"))
        || matches!(
            token.to_ascii_lowercase().as_str(),
            "bearer"
                | "api_key"
                | "api-key"
                | "apikey"
                | "key"
                | "token"
                | "access_token"
                | "secret"
                | "password"
        );
    if (*secret && !label)
        || token.len() >= 20
        || token.starts_with("sk-")
        || token.starts_with("ghp_")
        || token.starts_with("AKIA")
        || token.starts_with("AIza")
    {
        output.push_str(MASK);
    } else {
        output.push_str(token);
    }
    *secret = label;
    token.clear();
}
