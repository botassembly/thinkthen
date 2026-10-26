//! How many records share one request.

use std::num::NonZeroUsize;
use std::str::FromStr;

use crate::core::{Error, ErrorKind};

/// The variable that holds the batch size.
const BATCH_VARIABLE: &str = "THINKTHEN_BATCH";

/// The most records one request carries. The default is 50.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Batch(NonZeroUsize);

impl Batch {
    /// The batch a caller names: `flag`, then `THINKTHEN_BATCH`, then the default.
    /// `variable` reads one environment variable by name.
    ///
    /// # Errors
    ///
    /// `usage.batch` when the chosen value is not a whole number of at least 1.
    pub fn resolve(
        flag: Option<&str>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, Error> {
        Ok(Self::override_from(flag, variable)?.unwrap_or_default())
    }

    /// The caller's flag or environment override, if either is present.
    ///
    ///
    /// # Errors
    /// Returns `usage.batch` when the selected override is invalid.
    pub fn override_from(
        flag: Option<&str>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Option<Self>, Error> {
        flag.map(str::to_owned)
            .or_else(|| variable(BATCH_VARIABLE))
            .map(|text| text.parse())
            .transpose()
    }

    /// The number of records per request.
    #[must_use]
    pub const fn get(self) -> NonZeroUsize {
        self.0
    }
}

impl Default for Batch {
    fn default() -> Self {
        Self(NonZeroUsize::MIN.saturating_add(49))
    }
}

impl FromStr for Batch {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Error> {
        text.bytes()
            .all(|byte| byte.is_ascii_digit())
            .then(|| text.parse::<NonZeroUsize>().ok())
            .flatten()
            .map(Self)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::Batch,
                    crate::core::diagnostic::expected(
                        "batch",
                        "a whole number of at least 1",
                        &serde_json::Value::String(text.into()),
                    ),
                )
            })
    }
}

impl From<NonZeroUsize> for Batch {
    fn from(size: NonZeroUsize) -> Self {
        Self(size)
    }
}

impl serde::Serialize for Batch {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&self.0, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for Batch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <NonZeroUsize as serde::Deserialize>::deserialize(deserializer).map(Self)
    }
}
impl schemars::JsonSchema for Batch {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Batch".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonZeroUsize as schemars::JsonSchema>::json_schema(generator)
    }
}
impl TryFrom<u64> for Batch {
    type Error = Error;
    fn try_from(value: u64) -> Result<Self, Error> {
        usize::try_from(value)
            .ok()
            .and_then(NonZeroUsize::new)
            .map(Self)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::Batch,
                    crate::core::diagnostic::requirement(
                        "batch",
                        "a positive integer within the platform range",
                        "an out-of-range count",
                    ),
                )
            })
    }
}
