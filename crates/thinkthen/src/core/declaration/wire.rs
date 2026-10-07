//! Actual presentation types for the admitted declaration subset and its schema.
use super::{InputDeclaration, InputProperty, InputPropertyType, ObjectDeclaration};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "lowercase")]
pub(super) enum Property {
    String,
    Number,
    Boolean,
    #[serde(rename = "array")]
    StringList {
        items: StringRoot,
    },
}
impl From<InputPropertyType> for Property {
    fn from(kind: InputPropertyType) -> Self {
        match kind {
            InputPropertyType::String => Self::String,
            InputPropertyType::Number => Self::Number,
            InputPropertyType::Boolean => Self::Boolean,
            InputPropertyType::StringList => Self::StringList {
                items: StringRoot::new(),
            },
        }
    }
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub(super) enum StringType {
    String,
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(super) struct StringRoot {
    r#type: StringType,
}
impl StringRoot {
    fn new() -> Self {
        Self {
            r#type: StringType::String,
        }
    }
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub(super) enum ObjectType {
    Object,
}
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(
    test,
    schemars(with = "std::collections::BTreeMap<String, InputPropertyType>")
)]
struct Properties<'a>(&'a [InputProperty]);
impl Serialize for Properties<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(
            self.0
                .iter()
                .map(|property| (&property.name, property.kind)),
        )
    }
}
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(super) struct ObjectRoot<'a> {
    r#type: ObjectType,
    properties: Properties<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<&'a [String]>,
}
impl<'a> From<&'a ObjectDeclaration> for ObjectRoot<'a> {
    fn from(object: &'a ObjectDeclaration) -> Self {
        Self {
            r#type: ObjectType::Object,
            properties: Properties(&object.properties),
            required: (!object.required.is_empty()).then_some(object.required.as_slice()),
        }
    }
}
#[derive(Serialize)]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "inputDeclaration")
)]
#[serde(untagged)]
pub(super) enum Root<'a> {
    String(StringRoot),
    Object(ObjectRoot<'a>),
}
impl<'a> From<&'a InputDeclaration> for Root<'a> {
    fn from(declaration: &'a InputDeclaration) -> Self {
        match declaration {
            InputDeclaration::String => Self::String(StringRoot::new()),
            InputDeclaration::Object(object) => Self::Object(object.into()),
        }
    }
}
