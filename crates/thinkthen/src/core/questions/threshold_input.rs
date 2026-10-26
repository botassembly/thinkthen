//! Retain invalid numeric input until the shared typed admission step.
use super::*;

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub(super) enum ThresholdInput {
    Number(f64),
    Band(BandInput),
    Invalid(serde::de::IgnoredAny),
    #[serde(skip)]
    Admitted(Threshold),
}
#[derive(Debug, serde::Serialize)]
pub(super) struct BandInput {
    low: f64,
    high: f64,
}
impl<'de> serde::Deserialize<'de> for BandInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BandInput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an object with low and high")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                map: M,
            ) -> Result<BandInput, M::Error> {
                #[derive(serde::Deserialize)]
                #[serde(deny_unknown_fields)]
                struct Fields {
                    low: f64,
                    high: f64,
                }
                let value = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(map),
                )?;
                Ok(BandInput {
                    low: value.low,
                    high: value.high,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
impl Default for ThresholdInput {
    fn default() -> Self {
        Self::Admitted(Threshold::default())
    }
}
impl ThresholdInput {
    pub(super) fn admit(self) -> Result<Threshold, Error> {
        match self {
            Self::Number(value) => Cut::new(value).map(Into::into),
            Self::Band(value) => Threshold::band(value.low, value.high),
            Self::Admitted(value) => Ok(value),
            Self::Invalid(_) => Err(crate::core::threshold::invalid()),
        }
    }
}
impl From<Threshold> for ThresholdInput {
    fn from(value: Threshold) -> Self {
        Self::Admitted(value)
    }
}
impl serde::Serialize for ThresholdInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Number(value) => value.serialize(serializer),
            Self::Band(value) => value.serialize(serializer),
            Self::Admitted(value) => value.serialize(serializer),
            Self::Invalid(_) => serializer.serialize_unit(),
        }
    }
}
impl schemars::JsonSchema for ThresholdInput {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Threshold".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        Threshold::json_schema(generator)
    }
}
