//! A JSON Pointer, as RFC 6901 writes one, and nothing else.
//!
//! The tool never guesses a pointer language. `$.body` is JSONPath, `#/id` is
//! the URI fragment form, a wildcard belongs to neither, and a negative index
//! belongs to no JSON Pointer. Each one is refused by name, so a user who
//! typed the wrong language is told which one this is.

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::json::Json;

/// Why a string is not a JSON Pointer.
///
/// Every message names RFC 6901, because the point of the refusal is to say
/// which pointer language the tool reads.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PointerError {
    /// The string does not begin with `/`, as `$.body` does not.
    #[error("a pointer is RFC 6901, so it is empty or begins with `/`, and `$.body` is not one")]
    NotAPointer,
    /// The string begins with `#`, which is the URI fragment form.
    #[error("a pointer is RFC 6901, and `#/id` is the URI fragment form of one")]
    Fragment,
    /// A part is a wildcard, which RFC 6901 has none of.
    #[error("a pointer is RFC 6901, which has no wildcard")]
    Wildcard,
    /// A part is a negative index, which RFC 6901 has none of.
    #[error("a pointer is RFC 6901, which has no negative index")]
    NegativeIndex,
    /// A `~` is not followed by `0` or `1`.
    #[error("a pointer is RFC 6901, so `~` is followed by `0` or by `1`")]
    Escape,
}

/// One JSON Pointer, kept as the user wrote it and as the parts it names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pointer {
    text: String,
    parts: Vec<String>,
}

impl Pointer {
    /// Read one pointer, or say which pointer language was typed instead.
    ///
    /// # Errors
    ///
    /// Returns [`PointerError`] for every string RFC 6901 does not spell.
    pub fn new(text: impl Into<String>) -> Result<Self, PointerError> {
        let text = text.into();
        if text.starts_with('#') {
            return Err(PointerError::Fragment);
        }
        if !text.is_empty() && !text.starts_with('/') {
            return Err(PointerError::NotAPointer);
        }
        let mut parts = Vec::new();
        for raw in text.split('/').skip(1) {
            parts.push(part(raw)?);
        }
        Ok(Self { text, parts })
    }

    /// The last part, which keys this pointer's member of an evidence object.
    ///
    /// The whole record has no last part, so its key is the empty name, and
    /// two pointers that would share one key are refused where they are taken.
    #[must_use]
    pub fn key(&self) -> &str {
        self.parts.last().map_or("", String::as_str)
    }

    /// The pointer as the user wrote it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// The value this pointer names, or `None` when the record holds none.
    pub(crate) fn resolve<'a>(&self, value: &'a Json) -> Option<&'a Json> {
        let mut found = value;
        for name in &self.parts {
            found = match index(name) {
                Some(place) => found.element(place).or_else(|| found.member(name))?,
                None => found.member(name)?,
            };
        }
        Some(found)
    }
}

/// Read one reference token, refusing every language RFC 6901 is not.
fn part(raw: &str) -> Result<String, PointerError> {
    if !raw.is_empty() && raw.bytes().all(|byte| byte == b'*') {
        return Err(PointerError::Wildcard);
    }
    if let Some(rest) = raw.strip_prefix('-')
        && !rest.is_empty()
        && rest.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(PointerError::NegativeIndex);
    }
    let mut part = String::with_capacity(raw.len());
    let mut characters = raw.chars();
    while let Some(character) = characters.next() {
        if character != '~' {
            part.push(character);
            continue;
        }
        match characters.next() {
            Some('0') => part.push('~'),
            Some('1') => part.push('/'),
            _ => return Err(PointerError::Escape),
        }
    }
    Ok(part)
}

/// The array place this token names, or `None` when it names no place.
///
/// RFC 6901 spells an index as `0` or as digits with no leading zero, so `01`
/// names a member called `01` and never the second element.
fn index(name: &str) -> Option<usize> {
    if name != "0" && name.starts_with('0') {
        return None;
    }
    name.parse().ok()
}

impl Serialize for Pointer {
    /// Write the pointer as the user wrote it, so a plan shows what was typed.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::{Pointer, PointerError};
    use crate::json::Json;
    use proptest::collection::vec;
    use proptest::prelude::{Strategy, any};
    use proptest::{prop_assert_eq, proptest};

    fn record() -> Json {
        Json::parse(r#"{"id":"T-91","a":{"text":"inner","~x":1,"a/b":2},"list":["zero","one"]}"#)
            .expect("a JSON record")
    }

    #[test]
    fn a_pointer_names_the_part_rfc_6901_says_it_names() {
        let record = record();
        let cases = [
            ("/id", Some(r#""T-91""#)),
            ("/a/text", Some(r#""inner""#)),
            ("/a/~0x", Some("1")),
            ("/a/a~1b", Some("2")),
            ("/list/0", Some(r#""zero""#)),
            ("/list/1", Some(r#""one""#)),
            ("/list/2", None),
            ("/list/-", None),
            ("/list/01", None),
            ("/missing", None),
            ("/id/deeper", None),
        ];
        for (text, expected) in cases {
            let pointer = Pointer::new(text).expect("an RFC 6901 pointer");
            let found = pointer
                .resolve(&record)
                .map(|value| crate::render::json_line(value).expect("a value is writable"));
            assert_eq!(found.as_deref(), expected, "{text}");
        }
        let whole = Pointer::new("").expect("the whole record");
        assert_eq!(whole.resolve(&record), Some(&record));
    }

    #[test]
    fn every_other_pointer_language_is_refused_by_name() {
        let cases = [
            ("$.body", PointerError::NotAPointer),
            ("body", PointerError::NotAPointer),
            ("#/id", PointerError::Fragment),
            ("#", PointerError::Fragment),
            ("/*", PointerError::Wildcard),
            ("/a/*", PointerError::Wildcard),
            ("/**/text", PointerError::Wildcard),
            ("/list/-1", PointerError::NegativeIndex),
            ("/-12", PointerError::NegativeIndex),
            ("/a~2b", PointerError::Escape),
            ("/a~", PointerError::Escape),
        ];
        for (text, expected) in cases {
            assert_eq!(Pointer::new(text), Err(expected), "{text}");
        }
        for (text, expected) in cases {
            let said = Pointer::new(text).expect_err("refused").to_string();
            assert!(said.contains("RFC 6901"), "{expected} says {said}");
        }
    }

    #[test]
    fn the_key_of_a_pointer_is_its_last_part_decoded() {
        let cases = [("/body", "body"), ("/a/text", "text"), ("/a~1b", "a/b")];
        for (text, key) in cases {
            assert_eq!(Pointer::new(text).expect("a pointer").key(), key);
        }
        assert_eq!(Pointer::new("").expect("a pointer").key(), "");
    }

    /// Member names that spell no other pointer language and no array place.
    fn names() -> impl Strategy<Value = String> {
        vec(any::<char>(), 1..8)
            .prop_map(|characters| characters.into_iter().collect::<String>())
            .prop_filter(
                "a name that is no index, no wildcard, and no sign",
                |name| {
                    name.parse::<usize>().is_err()
                        && name.bytes().any(|byte| byte != b'*')
                        && !name.starts_with('-')
                },
            )
    }

    proptest! {
        /// Any nesting of member names round trips through the pointer that spells it.
        #[test]
        fn a_pointer_over_written_names_finds_the_value_they_nest(names in vec(names(), 1..5)) {
            let mut value = Json::String("found".to_owned());
            for name in names.iter().rev() {
                value = Json::Object(vec![(name.clone(), value)]);
            }
            let text: String = names
                .iter()
                .map(|name| format!("/{}", name.replace('~', "~0").replace('/', "~1")))
                .collect();
            let pointer = Pointer::new(text).expect("a written pointer is RFC 6901");
            prop_assert_eq!(pointer.key(), names.last().expect("one name").as_str());
            prop_assert_eq!(pointer.resolve(&value), Some(&Json::String("found".to_owned())));
        }
    }
}
