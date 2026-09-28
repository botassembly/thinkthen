//! Put one member's value into JSON text the parser accepted, and keep every other byte.

use crate::core::json::Json;

/// One object member: its decoded key and the byte places around it.
struct Member {
    key: String,
    key_start: usize,
    key_end: usize,
    value_start: usize,
    value_end: usize,
    comma: Option<usize>,
}

/// Set or append the member at `path` and return the new text and the old value.
/// Returns `None` when a name on the way is missing or holds no object.
pub(crate) fn splice(text: &str, path: &[&str], value: &str) -> Option<(String, Option<String>)> {
    let bytes = text.as_bytes();
    let (last, way) = path.split_last()?;
    let mut at = space(bytes, 0);
    for name in way {
        let (members, _) = object(bytes, at)?;
        at = members.into_iter().find(|m| m.key == *name)?.value_start;
    }
    let (members, close) = object(bytes, at)?;
    if let Some(found) = members.iter().find(|m| m.key == *last) {
        let (head, tail) = (text.get(..found.value_start)?, text.get(found.value_end..)?);
        let old = text.get(found.value_start..found.value_end)?.to_owned();
        return Some((format!("{head}{value}{tail}"), Some(old)));
    }
    let (at, separator, colon) = match members.last() {
        None => (close, "", ": "),
        Some(tail) => (
            tail.value_end,
            tail.comma
                .map_or(Some(", "), |comma| text.get(comma..tail.key_start))?,
            text.get(tail.key_end..tail.value_start)?,
        ),
    };
    let (head, tail) = (text.get(..at)?, text.get(at..)?);
    Some((
        format!("{head}{separator}\"{last}\"{colon}{value}{tail}"),
        None,
    ))
}

/// Remove one object member while preserving every other member's bytes.
/// Returns the new text and the removed value, or `None` when the path is absent.
pub(crate) fn remove(text: &str, path: &[&str]) -> Option<(String, String)> {
    let bytes = text.as_bytes();
    let (last, way) = path.split_last()?;
    let mut at = space(bytes, 0);
    for name in way {
        let (members, _) = object(bytes, at)?;
        at = members.into_iter().find(|m| m.key == *name)?.value_start;
    }
    let (members, _) = object(bytes, at)?;
    let position = members.iter().position(|member| member.key == *last)?;
    let found = members.get(position)?;
    let old = text.get(found.value_start..found.value_end)?.to_owned();
    let (start, end) = if let Some(comma) = found.comma {
        (comma, found.value_end)
    } else if let Some(next) = members.get(position + 1) {
        (found.key_start, next.key_start)
    } else {
        (found.key_start, found.value_end)
    };
    Some((format!("{}{}", text.get(..start)?, text.get(end..)?), old))
}

fn space(bytes: &[u8], mut at: usize) -> usize {
    while bytes.get(at).is_some_and(u8::is_ascii_whitespace) {
        at += 1;
    }
    at
}

/// The members of the object at `at`, and the place of its closing brace.
fn object(bytes: &[u8], at: usize) -> Option<(Vec<Member>, usize)> {
    if bytes.get(at) != Some(&b'{') {
        return None;
    }
    let mut members = Vec::new();
    let mut at = space(bytes, at + 1);
    let mut comma = None;
    while bytes.get(at) == Some(&b'"') {
        let (key, key_end) = string(bytes, at)?;
        let value_start = space(bytes, space(bytes, key_end) + 1);
        let value_end = value(bytes, value_start)?;
        let key_start = at;
        members.push(Member {
            key,
            key_start,
            key_end,
            value_start,
            value_end,
            comma,
        });
        let next = space(bytes, value_end);
        if bytes.get(next) != Some(&b',') {
            return Some((members, next));
        }
        comma = Some(next);
        at = space(bytes, next + 1);
    }
    Some((members, at))
}

/// Just past the value that starts at `at`.
fn value(bytes: &[u8], at: usize) -> Option<usize> {
    match bytes.get(at)? {
        b'"' => string(bytes, at).map(|(_, end)| end),
        open @ (b'{' | b'[') => {
            let close = if *open == b'{' { b'}' } else { b']' };
            let mut at = space(bytes, at + 1);
            while bytes.get(at) != Some(&close) {
                at = space(bytes, value(bytes, at)?);
                if bytes.get(at) == Some(&b':') {
                    at = space(bytes, value(bytes, space(bytes, at + 1))?);
                }
                if bytes.get(at) == Some(&b',') {
                    at = space(bytes, at + 1);
                }
            }
            Some(at + 1)
        }
        _ => {
            let mut end = at;
            while bytes.get(end).is_some_and(|b| !b",}] \t\r\n".contains(b)) {
                end += 1;
            }
            Some(end)
        }
    }
}

/// The decoded string that starts at `at`, and the place just past its closing quote.
fn string(bytes: &[u8], at: usize) -> Option<(String, usize)> {
    let mut end = at + 1;
    while *bytes.get(end)? != b'"' {
        end += if bytes.get(end) == Some(&b'\\') { 2 } else { 1 };
    }
    let raw = std::str::from_utf8(bytes.get(at..=end)?).ok()?;
    Some((Json::parse(raw).ok()?.as_str()?.to_owned(), end + 1))
}

#[cfg(test)]
#[path = "splice_tests.rs"]
mod tests;
