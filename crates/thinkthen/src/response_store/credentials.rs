//! Inspect JSON strings without retaining a second decoded response tree.
use crate::{Code, Error};
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};

pub(crate) fn contains(json: &[u8], key: &str) -> Result<bool, Error> {
    if json.windows(key.len()).any(|bytes| bytes == key.as_bytes()) {
        return Ok(true);
    }
    let mut input = serde_json::Deserializer::from_slice(json);
    let found = Scan(key)
        .deserialize(&mut input)
        .map_err(|_| super::error(Code::LocalResponseStore))?;
    input
        .end()
        .map_err(|_| super::error(Code::LocalResponseStore))?;
    Ok(found)
}
#[derive(Clone, Copy)]
struct Scan<'a>(&'a str);
impl<'de> DeserializeSeed<'de> for Scan<'_> {
    type Value = bool;
    fn deserialize<D: serde::Deserializer<'de>>(self, input: D) -> Result<bool, D::Error> {
        input.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Scan<'_> {
    type Value = bool;
    fn expecting(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("a JSON value")
    }
    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<bool, E> {
        Ok(value.contains(self.0))
    }
    fn visit_bool<E: serde::de::Error>(self, _: bool) -> Result<bool, E> {
        Ok(false)
    }
    fn visit_i64<E: serde::de::Error>(self, _: i64) -> Result<bool, E> {
        Ok(false)
    }
    fn visit_u64<E: serde::de::Error>(self, _: u64) -> Result<bool, E> {
        Ok(false)
    }
    fn visit_f64<E: serde::de::Error>(self, _: f64) -> Result<bool, E> {
        Ok(false)
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<bool, E> {
        Ok(false)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<bool, A::Error> {
        let mut found = false;
        while let Some(value) = sequence.next_element_seed(self)? {
            found |= value;
        }
        Ok(found)
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<bool, A::Error> {
        let mut found = false;
        while let Some(key) = map.next_key_seed(self)? {
            found |= key;
            found |= map.next_value_seed(self)?;
        }
        Ok(found)
    }
}
