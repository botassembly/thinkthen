//! Typed question-member settings reuse canonical ordered label composition.
use super::field;
use crate::{call, ffi::RawJson};
use pgrx::prelude::*;
use serde::ser::{SerializeMap as _, Serializer};
use thinkthen::{Error, RequestQuestion, Settings};

type MemberFields<'a> = Vec<(&'a str, Box<serde_json::value::RawValue>)>;
struct Fields<'a>(&'a MemberFields<'a>);
impl serde::Serialize for Fields<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (key, value) in self.0 {
            map.serialize_entry(key, value)?;
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
            let Some(values) =
                field::<Array<'_, pgrx::composite_type!("thinkthen_option")>>(tuple, key)?
            else {
                continue;
            };
            let value = labels(values, key == "levels")?;
            members.push((key, value));
        }
    }
    let source = serde_json::to_string(&Fields(&members))
        .map_err(|_| call::defect("question settings could not be serialized"))?;
    Settings::parse(&source).map_err(|error| call::usage(error.to_string()))
}

fn labels(
    values: Array<'_, pgrx::composite_type!("thinkthen_option")>,
    bare_levels: bool,
) -> Result<Box<serde_json::value::RawValue>, Error> {
    let mut names = Vec::new();
    let mut descriptions = Vec::new();
    for value in values.iter() {
        let value = value.ok_or_else(|| call::usage("question members contain NULL"))?;
        let name = field::<String>(&value, "name")?
            .ok_or_else(|| call::usage("question member name is NULL"))?;
        if let Some(description) = field::<RawJson>(&value, "description")? {
            descriptions.push((
                name.clone(),
                thinkthen::Description::from_json(&description.0)?,
            ));
        }
        names.push(name);
    }
    let value = RequestQuestion::labels(names, descriptions, bare_levels)?;
    serde_json::value::RawValue::from_string(value.as_json().to_owned())
        .map_err(|_| call::defect("native question members are invalid"))
}
