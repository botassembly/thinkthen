//! The JSON a record holds, as the tool's own tree over bytes it did not write.
//!
//! The wire bodies decode into typed structs, and a record cannot, because a
//! user's record has whatever shape the user gave it. This tree is the one
//! dynamic value in the crate. It is built here so that the duplicate name and
//! the number that is not finite are refused where they arrive.

use std::fmt;

use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use serde_json::Number;
use thiserror::Error;

/// The message a duplicate member name is refused with.
const DUPLICATE: &str = "duplicate JSON member at ";
const DUPLICATE_MESSAGE: &str =
    "a JSON record holds each member name once, and one name arrived twice";

/// The message a number that is not finite is refused with.
const NOT_FINITE: &str = "a JSON number is finite, so `NaN` and `Infinity` are refused";

/// Why an input is not JSON this tool will read.
///
/// Display and debug text reveal no input byte, because input may hold private evidence.
#[derive(Clone, Eq, Error, PartialEq)]
pub enum JsonError {
    /// Two members of one object arrived under one name.
    #[error("{DUPLICATE_MESSAGE}")]
    DuplicateName {
        /// Dot-separated path to the repeated name.
        path: String,
    },
    /// A number arrived that no finite JSON number can hold.
    #[error("{NOT_FINITE}")]
    NotFinite,
    /// The bytes are not JSON at all.
    #[error("the record is not valid JSON")]
    Syntax {
        /// The one-based line where the parser stopped.
        line: usize,
        /// The one-based column where the parser stopped, or zero at empty input.
        column: usize,
    },
}

impl fmt::Debug for JsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateName { .. } => formatter.write_str("DuplicateName(<path withheld>)"),
            Self::NotFinite => formatter.write_str("NotFinite"),
            Self::Syntax { line, column } => formatter
                .debug_struct("Syntax")
                .field("line", line)
                .field("column", column)
                .finish(),
        }
    }
}

/// One JSON value, in the order it arrived.
///
/// An object keeps its members in document order, because the record is
/// printed back under `input` and a reader compares it with what they wrote.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Json {
    /// `null`.
    Null,
    /// `true` or `false`.
    Bool(bool),
    /// A finite JSON number.
    Number(Number),
    /// A JSON string.
    String(String),
    /// A JSON array.
    Array(Vec<Json>),
    /// A JSON object, in document order.
    Object(Vec<(String, Json)>),
}

impl Json {
    /// Read one JSON value from text.
    ///
    /// # Errors
    ///
    /// Returns [`JsonError`] when the text is not JSON, when one object holds
    /// two members under one name, or when a number is not finite.
    pub(crate) fn parse(text: &str) -> Result<Self, JsonError> {
        let error = match serde_json::from_str::<Self>(text) {
            Ok(value) => return Ok(value),
            Err(error) => error,
        };
        let message = error.to_string();
        if let Some(path) = marked_path(&message) {
            return Err(JsonError::DuplicateName { path });
        }
        if message.starts_with(NOT_FINITE) || message.contains("number out of range") {
            return Err(JsonError::NotFinite);
        }
        Err(JsonError::Syntax {
            line: error.line(),
            column: error.column(),
        })
    }

    /// The string this value holds, or `None` when it is not a string.
    pub(crate) fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(text) => Some(text.as_str()),
            _ => None,
        }
    }

    /// The member of this object under that name, or `None` when it has none.
    pub(crate) fn member(&self, name: &str) -> Option<&Self> {
        match self {
            Self::Object(members) => members
                .iter()
                .find_map(|(held, value)| (held == name).then_some(value)),
            _ => None,
        }
    }

    /// The element of this array at that place, or `None` when it has none.
    pub(crate) fn element(&self, place: usize) -> Option<&Self> {
        match self {
            Self::Array(elements) => elements.get(place),
            _ => None,
        }
    }
}

impl Serialize for Json {
    /// Write the value back as compact JSON, in the order it arrived.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_unit(),
            Self::Bool(held) => serializer.serialize_bool(*held),
            Self::Number(number) => number.serialize(serializer),
            Self::String(text) => serializer.serialize_str(text),
            Self::Array(elements) => {
                let mut sequence = serializer.serialize_seq(Some(elements.len()))?;
                for element in elements {
                    sequence.serialize_element(element)?;
                }
                sequence.end()
            }
            Self::Object(members) => {
                let mut map = serializer.serialize_map(Some(members.len()))?;
                for (name, value) in members {
                    map.serialize_entry(name, value)?;
                }
                map.end()
            }
        }
    }
}

/// The visitor that reads one JSON value and refuses what the tool will not hold.
struct Reader;

impl<'de> Visitor<'de> for Reader {
    type Value = Json;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("one JSON value")
    }

    fn visit_unit<E>(self) -> Result<Json, E> {
        Ok(Json::Null)
    }

    fn visit_bool<E>(self, held: bool) -> Result<Json, E> {
        Ok(Json::Bool(held))
    }

    fn visit_i64<E>(self, number: i64) -> Result<Json, E> {
        Ok(Json::Number(number.into()))
    }

    fn visit_u64<E>(self, number: u64) -> Result<Json, E> {
        Ok(Json::Number(number.into()))
    }

    fn visit_f64<E: serde::de::Error>(self, number: f64) -> Result<Json, E> {
        Number::from_f64(number)
            .map_or_else(|| Err(E::custom(NOT_FINITE)), |held| Ok(Json::Number(held)))
    }

    fn visit_str<E>(self, text: &str) -> Result<Json, E> {
        Ok(Json::String(text.to_owned()))
    }

    fn visit_string<E>(self, text: String) -> Result<Json, E> {
        Ok(Json::String(text))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Json, A::Error> {
        let mut elements = Vec::new();
        loop {
            match access.next_element::<Json>() {
                Ok(Some(element)) => elements.push(element),
                Ok(None) => break,
                Err(error) => {
                    return Err(prefix_duplicate(error, &format!("[{}]", elements.len())));
                }
            }
        }
        Ok(Json::Array(elements))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Json, A::Error> {
        let mut members: Vec<(String, Json)> = Vec::new();
        while let Some(name) = access.next_key::<String>()? {
            if members.iter().any(|(held, _)| held == &name) {
                return Err(<A::Error as serde::de::Error>::custom(format!(
                    "{DUPLICATE}{name}"
                )));
            }
            let value = access
                .next_value::<Json>()
                .map_err(|error| prefix_duplicate(error, &name))?;
            members.push((name, value));
        }
        Ok(Json::Object(members))
    }
}

fn marked_path(message: &str) -> Option<String> {
    let rest = message.strip_prefix(DUPLICATE)?;
    Some(rest.split(" at line ").next().unwrap_or(rest).to_owned())
}

fn prefix_duplicate<E: serde::de::Error>(error: E, parent: &str) -> E {
    let message = error.to_string();
    let Some(path) = marked_path(&message) else {
        return error;
    };
    let separator = if path.starts_with('[') { "" } else { "." };
    E::custom(format!("{DUPLICATE}{parent}{separator}{path}"))
}

impl<'de> Deserialize<'de> for Json {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(Reader)
    }
}

#[cfg(test)]
mod tests {
    use super::{Json, JsonError};
    use crate::render::json_line;

    #[test]
    fn a_record_is_written_back_in_the_order_it_arrived() {
        let cases = [
            r#"{"id":"T-91","body":"Payouts failed.","open":true,"seen":null}"#,
            r#"{"b":1,"a":2}"#,
            r#"[1,2.5,"three",null,{"a":[]}]"#,
            r#""a bare string""#,
            "12",
            "true",
            "null",
        ];
        for case in cases {
            let value = Json::parse(case).expect("a JSON value");
            assert_eq!(json_line(&value).expect("a value is writable"), case);
        }
    }

    #[test]
    fn a_duplicate_member_name_is_refused_wherever_it_sits() {
        let cases = [
            r#"{"id":1,"id":2}"#,
            r#"{"a":{"id":1,"id":2}}"#,
            r#"[{"id":1,"id":2}]"#,
        ];
        for case in cases {
            assert!(
                matches!(Json::parse(case), Err(JsonError::DuplicateName { .. })),
                "{case}"
            );
        }
        assert_eq!(
            Json::parse(r#"{"a":{"id":1,"id":2}}"#),
            Err(JsonError::DuplicateName {
                path: "a.id".to_owned()
            })
        );
        let hidden = Json::parse(r#"{"private-marker":1,"private-marker":2}"#)
            .expect_err("duplicate refused");
        assert!(!format!("{hidden:?}").contains("private-marker"));
        assert!(!hidden.to_string().contains("private-marker"));
    }

    #[test]
    fn a_number_that_is_not_finite_is_refused_and_a_large_finite_one_is_kept() {
        for case in [r#"{"x":1e999}"#, "-1e400"] {
            assert_eq!(Json::parse(case), Err(JsonError::NotFinite), "{case}");
        }
        for case in [r#"{"x":NaN}"#, r#"{"x":Infinity}"#, r#"{"x":-Infinity}"#] {
            assert!(Json::parse(case).is_err(), "{case}");
        }
        assert!(Json::parse("1e308").is_ok());
    }

    #[test]
    fn bytes_that_are_not_json_are_refused_as_syntax() {
        let cases = [
            ("", 1, 0),
            ("\u{feff}{\"a\":1}", 1, 1),
            ("{\"a\":1,}\n", 1, 8),
            ("{\n\"private-marker\": true,\n}\n", 3, 1),
        ];
        for (case, line, column) in cases {
            let error = Json::parse(case).expect_err("invalid JSON");
            assert_eq!(error, JsonError::Syntax { line, column }, "{case}");
            assert_eq!(
                format!("{error:?}"),
                format!("Syntax {{ line: {line}, column: {column} }}"),
                "{case}"
            );
        }
    }

    #[test]
    fn a_member_and_an_element_are_read_by_name_and_by_place() {
        let value = Json::parse(r#"{"a":["x","y"],"b":"z"}"#).expect("a JSON value");
        assert_eq!(value.member("b").and_then(Json::as_str), Some("z"));
        assert_eq!(value.member("c"), None);
        let array = value.member("a").expect("a member");
        assert_eq!(array.element(1).and_then(Json::as_str), Some("y"));
        assert_eq!(array.element(2), None);
        assert_eq!(array.member("0"), None);
    }
}
