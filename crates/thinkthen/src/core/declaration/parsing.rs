//! The one ordered declaration parser, used by every question-file role.
use super::{
    DeclarationError, InputDeclaration, InputProperty, InputPropertyType, ObjectDeclaration,
    QuestionMetadata, QuestionName, WordingVersion,
};
use crate::core::Json;

pub(crate) const KEYS: [&str; 4] = ["name", "wording_version", "item_schema", "context_schema"];

impl QuestionMetadata {
    pub(crate) fn parse(value: &Json) -> Result<Self, DeclarationError> {
        let name = value
            .member("name")
            .map(|v| {
                v.as_str()
                    .ok_or(DeclarationError)
                    .and_then(QuestionName::validated)
            })
            .transpose()?;
        let wording_version = value
            .member("wording_version")
            .map(|v| {
                let Json::Number(number) = v else {
                    return Err(DeclarationError);
                };
                // serde preserves integer versus floating/exponent number categories.
                number
                    .as_u64()
                    .and_then(|n| u32::try_from(n).ok())
                    .ok_or(DeclarationError)
                    .and_then(WordingVersion::validated)
            })
            .transpose()?;
        Ok(Self {
            name,
            wording_version,
            item_schema: value
                .member("item_schema")
                .map(InputDeclaration::parse)
                .transpose()?,
            context_schema: value
                .member("context_schema")
                .map(InputDeclaration::parse)
                .transpose()?,
        })
    }
    pub(crate) fn is_key(key: &str) -> bool {
        KEYS.contains(&key)
    }
}

fn closed(value: &Json, allowed: &[&str]) -> Result<(), DeclarationError> {
    let Json::Object(members) = value else {
        return Err(DeclarationError);
    };
    if members
        .iter()
        .any(|(key, _)| !allowed.contains(&key.as_str()))
    {
        return Err(DeclarationError);
    }
    Ok(())
}

impl InputDeclaration {
    pub(crate) fn parse(value: &Json) -> Result<Self, DeclarationError> {
        match value.member("type").and_then(Json::as_str) {
            Some("string") => {
                closed(value, &["type"])?;
                Ok(Self::String)
            }
            Some("object") => {
                closed(value, &["type", "properties", "required"])?;
                let Some(Json::Object(properties)) = value.member("properties") else {
                    return Err(DeclarationError);
                };
                let properties = properties
                    .iter()
                    .map(|(name, schema)| InputProperty::validated(name, property_type(schema)?))
                    .collect::<Result<Vec<_>, _>>()?;
                let required = match value.member("required") {
                    None => Vec::new(),
                    Some(Json::Array(names)) => names
                        .iter()
                        .map(|v| v.as_str().map(str::to_owned).ok_or(DeclarationError))
                        .collect::<Result<Vec<_>, _>>()?,
                    _ => return Err(DeclarationError),
                };
                ObjectDeclaration::validated(properties, required).map(Self::Object)
            }
            _ => Err(DeclarationError),
        }
    }
}

fn property_type(value: &Json) -> Result<InputPropertyType, DeclarationError> {
    if value.member("type").and_then(Json::as_str) == Some("array") {
        closed(value, &["type", "items"])?;
        if InputDeclaration::parse(value.member("items").ok_or(DeclarationError)?)?
            != InputDeclaration::String
        {
            return Err(DeclarationError);
        }
        return Ok(InputPropertyType::StringList);
    }
    closed(value, &["type"])?;
    match value.member("type").and_then(Json::as_str) {
        Some("string") => Ok(InputPropertyType::String),
        Some("number") => Ok(InputPropertyType::Number),
        Some("boolean") => Ok(InputPropertyType::Boolean),
        _ => Err(DeclarationError),
    }
}
