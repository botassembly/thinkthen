//! Native context declarations use canonical validated constructors.
use super::field;
use crate::call;
use pgrx::prelude::*;
use thinkthen::{Error, InputDeclaration, InputProperty, InputPropertyType, ObjectDeclaration};

pub(super) fn read(
    reading: Option<&pgrx::composite_type!("thinkthen_reading")>,
) -> Result<Option<InputDeclaration>, Error> {
    let Some(reading) = reading else {
        return Ok(None);
    };
    let Some(schema) =
        field::<pgrx::composite_type!("thinkthen_context_schema")>(reading, "context_schema")?
    else {
        return Ok(None);
    };
    let kind = field::<String>(&schema, "kind")?
        .ok_or_else(|| call::usage("context schema kind is NULL"))?;
    let properties = field::<Array<'_, pgrx::composite_type!("thinkthen_context_property")>>(
        &schema,
        "properties",
    )?;
    let required = field::<Array<'_, &str>>(&schema, "required")?;
    if kind == "string" && properties.is_none() && required.is_none() {
        return Ok(Some(InputDeclaration::String));
    }
    if kind != "object" {
        return Err(call::usage("context schema is string or object"));
    }
    let properties = properties
        .ok_or_else(|| call::usage("object context requires properties"))?
        .iter()
        .map(|property| {
            let property =
                property.ok_or_else(|| call::usage("context properties contain NULL"))?;
            let name = field::<String>(&property, "name")?
                .ok_or_else(|| call::usage("context property name is NULL"))?;
            let kind = match field::<String>(&property, "kind")?.as_deref() {
                Some("string") => InputPropertyType::String,
                Some("number") => InputPropertyType::Number,
                Some("boolean") => InputPropertyType::Boolean,
                Some("string_list") => InputPropertyType::StringList,
                _ => {
                    return Err(call::usage(
                        "context property is string, number, boolean or string_list",
                    ));
                }
            };
            InputProperty::new(&name, kind)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let required = required
        .map(|values| {
            values
                .iter()
                .map(|value| {
                    value
                        .map(str::to_owned)
                        .ok_or_else(|| call::usage("required context names contain NULL"))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();
    Ok(Some(InputDeclaration::Object(ObjectDeclaration::new(
        properties, required,
    )?)))
}
