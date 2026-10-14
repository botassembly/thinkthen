//! Typed question-member settings retain authored names and descriptions.
use super::field;
use crate::{call, ffi::RawJson};
use pgrx::prelude::*;
use serde::ser::{SerializeMap as _, Serializer};
use thinkthen::{Error, Settings};

struct Members(Vec<(String, Option<Box<serde_json::value::RawValue>>)>);
impl serde::Serialize for Members {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0.iter().all(|(_, description)| description.is_none()) {
            return serde::Serialize::serialize(&self.0.iter().map(|(name, _)| name).collect::<Vec<_>>(), serializer);
        }
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (name, description) in &self.0 {
            map.serialize_entry(name, description)?;
        }
        map.end()
    }
}

pub(super) fn read(
    tuple: Option<&pgrx::composite_type!("thinkthen_reading")>,
) -> Result<Settings, Error> {
    let mut members = Vec::new();
    if let Some(tuple) = tuple {
        for key in ["options", "levels", "labels"] {
            let Some(values) = field::<Array<'_, pgrx::composite_type!("thinkthen_option")>>(tuple, key)? else { continue };
            let values = values.iter().map(|value| {
                let value = value.ok_or_else(|| call::usage("question members contain NULL"))?;
                let name = field::<String>(&value, "name")?.ok_or_else(|| call::usage("question member name is NULL"))?;
                let description = field::<RawJson>(&value, "description")?
                    .map(|value| serde_json::value::RawValue::from_string(value.0)
                        .map_err(|_| call::usage("question member description is invalid JSON")))
                    .transpose()?;
                Ok((name, description))
            }).collect::<Result<Vec<_>, Error>>()?;
            members.push((key, Members(values)));
        }
    }
    struct Fields<'a>(&'a [(&'a str, Members)]);
    impl serde::Serialize for Fields<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut map = serializer.serialize_map(Some(self.0.len()))?;
            for (key, value) in self.0 { map.serialize_entry(key, value)?; }
            map.end()
        }
    }
    let source = serde_json::to_string(&Fields(&members))
        .map_err(|_| call::defect("question settings could not be serialized"))?;
    Settings::parse(&source).map_err(|error| call::usage(error.to_string()))
}
