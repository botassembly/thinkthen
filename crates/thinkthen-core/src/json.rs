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
const DUPLICATE: &str = "a JSON record holds each member name once, and one name arrived twice";

/// The message a number that is not finite is refused with.
const NOT_FINITE: &str = "a JSON number is finite, so `NaN` and `Infinity` are refused";

/// Why a record is not JSON this tool will read.
///
/// No variant carries any part of the record, because a record is evidence.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum JsonError {
    /// Two members of one object arrived under one name.
    #[error("{DUPLICATE}")]
    DuplicateName,
    /// A number arrived that no finite JSON number can hold.
    #[error("{NOT_FINITE}")]
    NotFinite,
    /// The bytes are not JSON at all.
    #[error("the record is not valid JSON")]
    Syntax,
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
        let error = match serde_json::from_str(text) {
            Ok(value) => return Ok(value),
            Err(error) => error.to_string(),
        };
        if error.starts_with(DUPLICATE) {
            return Err(JsonError::DuplicateName);
        }
        if error.starts_with(NOT_FINITE) || error.contains("number out of range") {
            return Err(JsonError::NotFinite);
        }
        Err(JsonError::Syntax)
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
        while let Some(element) = access.next_element()? {
            elements.push(element);
        }
        Ok(Json::Array(elements))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Json, A::Error> {
        let mut members: Vec<(String, Json)> = Vec::new();
        while let Some((name, value)) = access.next_entry::<String, Json>()? {
            if members.iter().any(|(held, _)| held == &name) {
                return Err(<A::Error as serde::de::Error>::custom(DUPLICATE));
            }
            members.push((name, value));
        }
        Ok(Json::Object(members))
    }
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
            assert_eq!(Json::parse(case), Err(JsonError::DuplicateName), "{case}");
        }
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
        for case in ["", "{", "not json", "{'a':1}", "{\"a\":1,}"] {
            assert_eq!(Json::parse(case), Err(JsonError::Syntax), "{case}");
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
