//! The rule that turns a probability into yes, no or not sure.

use std::str::FromStr;

use crate::core::error::{Error, ErrorKind};
use crate::core::probability::Probability;

/// What a threshold makes of a probability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Verdict {
    /// The probability reaches the yes side.
    Yes,
    /// The probability falls on the no side.
    No,
    /// The probability sits inside a band.
    Unsure,
}

/// A finite decision cut strictly above zero and at most one.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub struct Cut(Probability);

impl Cut {
    /// Validate a numeric decision cut.
    ///
    /// # Errors
    /// Returns `usage.threshold` for a nonfinite value or a value outside (0, 1].
    pub fn new(value: f64) -> Result<Self, Error> {
        Probability::new(value)
            .filter(|value| value.get() > 0.0)
            .map(Self)
            .ok_or_else(invalid)
    }

    /// The cut's original full-precision value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0.get()
    }
}

impl<'de> serde::Deserialize<'de> for Cut {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <f64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl schemars::JsonSchema for Cut {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Cut".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = Probability::json_schema(generator);
        let fields = schema.ensure_object();
        if let Some(minimum) = fields.remove("minimum") {
            fields.insert("exclusiveMinimum".into(), minimum);
        }
        schema
    }
}

pub(crate) fn invalid() -> Error {
    Error::new(
        ErrorKind::Threshold,
        crate::core::diagnostic::requirement(
            "threshold",
            "a cut above 0 and at most 1, or a band with 0 < low < high <= 1",
            "an invalid threshold",
        ),
    )
}

/// A single cut, or a band with a not sure middle.
///
/// A cut `T` says yes at `p >= T`. A band `LOW:HIGH` says yes at `p >= HIGH`,
/// no below `LOW`, and not sure from `LOW` up to `HIGH`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Threshold(Rule);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rule {
    Cut(Probability),
    Band(Probability, Probability),
}

impl Threshold {
    #[cfg(feature = "native")]
    pub(crate) fn as_cut(self) -> Result<Cut, Error> {
        match self.0 {
            Rule::Cut(value) => Ok(Cut(value)),
            Rule::Band(..) => Err(invalid()),
        }
    }
    /// Construct a band with ordered, validated numeric bounds.
    ///
    /// # Errors
    /// Returns `usage.threshold` unless 0 < low < high <= 1.
    pub fn band(low: f64, high: f64) -> Result<Self, Error> {
        let (low, high) = (Cut::new(low)?, Cut::new(high)?);
        if low.get() >= high.get() {
            return Err(invalid());
        }
        Ok(Self(Rule::Band(low.0, high.0)))
    }

    /// Require a single cut for a question that cannot return not sure.
    ///
    /// # Errors
    ///
    /// `usage.threshold` when this threshold is a band.
    pub fn require_cut(self) -> Result<Self, Error> {
        match self.0 {
            Rule::Cut(_) => Ok(self),
            Rule::Band(..) => Err(Error::new(
                ErrorKind::Threshold,
                crate::core::diagnostic::requirement(
                    "threshold",
                    "a single numeric cut",
                    "a low/high band",
                ),
            )),
        }
    }

    /// What this threshold makes of `p`.
    #[must_use]
    pub(crate) fn verdict(self, p: Probability) -> Verdict {
        match self.0 {
            Rule::Cut(cut) if p >= cut => Verdict::Yes,
            Rule::Cut(_) => Verdict::No,
            Rule::Band(_, high) if p >= high => Verdict::Yes,
            Rule::Band(low, _) if p < low => Verdict::No,
            Rule::Band(..) => Verdict::Unsure,
        }
    }
}

impl Default for Threshold {
    fn default() -> Self {
        Self(Rule::Cut(Probability::HALF))
    }
}

// The command and current request parser retain their text adapter until their
// own contract slice. New serde admission never accepts this string grammar.
impl FromStr for Threshold {
    type Err = Error;
    fn from_str(text: &str) -> Result<Self, Error> {
        let number = |part: &str| part.parse::<f64>().map_err(|_| invalid());
        match text.split_once(':') {
            Some((low, high)) => Self::band(number(low)?, number(high)?),
            None => Cut::new(number(text)?).map(Into::into),
        }
    }
}

impl From<Cut> for Threshold {
    fn from(cut: Cut) -> Self {
        Self(Rule::Cut(cut.0))
    }
}

impl serde::Serialize for Threshold {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Rule::Cut(cut) => serializer.serialize_f64(cut.get()),
            Rule::Band(low, high) => Band {
                low: Cut(low),
                high: Cut(high),
            }
            .serialize(serializer),
        }
    }
}

#[allow(
    clippy::disallowed_types,
    reason = "schema derivation builds metadata; bounds remain typed"
)]
mod wire {
    use super::Cut;
    #[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub(super) struct Band {
        pub(super) low: Cut,
        pub(super) high: Cut,
    }
}
use wire::Band;

impl<'de> serde::Deserialize<'de> for Threshold {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Threshold;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a numeric cut or an object with low and high")
            }
            fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Threshold, E> {
                Cut::new(value).map(Into::into).map_err(E::custom)
            }
            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Threshold, E> {
                self.visit_f64(value as f64)
            }
            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Threshold, E> {
                self.visit_f64(value as f64)
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                map: M,
            ) -> Result<Threshold, M::Error> {
                let band = <Band as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(map),
                )?;
                Threshold::band(band.low.get(), band.high.get()).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

impl schemars::JsonSchema for Threshold {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Threshold".into()
    }
    #[allow(
        clippy::disallowed_types,
        reason = "schema composition builds metadata only"
    )]
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"oneOf": [generator.subschema_for::<Cut>(), generator.subschema_for::<Band>()]})
    }
}

impl std::fmt::Display for Threshold {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Rule::Cut(cut) => write!(formatter, "{}", cut.get()),
            Rule::Band(low, high) => write!(formatter, "{}:{}", low.get(), high.get()),
        }
    }
}
