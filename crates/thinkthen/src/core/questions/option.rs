//! Shared named answers and their optional meanings.

/// One named answer with an optional description.
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(try_from = "OptionWire", into = "OptionWire")]
#[schemars(with = "OptionWire")]
pub struct AnswerOption {
    label: String,
    description: Option<String>,
}

impl AnswerOption {
    /// An option with a nonblank label.
    ///
    /// # Errors
    /// Returns the shared name or question error for an invalid label.
    pub fn new(label: impl Into<String>) -> Result<Self, crate::core::Error> {
        let label = label.into();
        crate::core::ask::admit_name(&label)?;
        if label.trim().is_empty() {
            return Err(crate::core::Error::new(
                crate::core::ErrorKind::Question,
                crate::core::diagnostic::requirement("label", "nonblank text", "blank text"),
            ));
        }
        Ok(Self {
            label,
            description: None,
        })
    }

    /// Describe when this option fits.
    #[must_use]
    pub fn description(self, description: impl Into<String>) -> Self {
        Self {
            description: Some(description.into()),
            ..self
        }
    }

    /// The option's label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The optional description.
    #[must_use]
    pub fn description_text(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
#[serde(untagged)]
pub(super) enum OptionWire {
    Name(String),
    Described(DescribedOption),
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct DescribedOption {
    name: String,
    description: String,
}
impl TryFrom<OptionWire> for AnswerOption {
    type Error = crate::core::Error;
    fn try_from(value: OptionWire) -> Result<Self, Self::Error> {
        match value {
            OptionWire::Name(name) => Self::new(name),
            OptionWire::Described(value) => {
                Ok(Self::new(value.name)?.description(value.description))
            }
        }
    }
}
impl From<AnswerOption> for OptionWire {
    fn from(value: AnswerOption) -> Self {
        match value.description {
            Some(description) => Self::Described(DescribedOption {
                name: value.label,
                description,
            }),
            None => Self::Name(value.label),
        }
    }
}

impl<'de> serde::Deserialize<'de> for OptionWire {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OptionWire;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a string or an object with name and description")
            }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(OptionWire::Name(value.into()))
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                map: A,
            ) -> Result<Self::Value, A::Error> {
                <DescribedOption as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(map),
                )
                .map(OptionWire::Described)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}
