//! Validated opaque correlation identities, separate from question digests.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use thiserror::Error;

pub(crate) mod answer;
pub(crate) mod framing;
pub(crate) mod legacy;

/// Why a correlation identity is invalid; the supplied bytes are withheld.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("an identity is exactly 64 lowercase hexadecimal characters")]
pub struct IdentityError;

macro_rules! identity {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Validate the exact opaque spelling, without trimming.
            ///
            /// # Errors
            /// Returns [`IdentityError`] for any other spelling.
            pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
                let value = value.into();
                if value.len() != 64
                    || !value
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                {
                    return Err(IdentityError);
                }
                Ok(Self(value))
            }

            /// The validated JSON and header spelling.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = IdentityError;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                Self::new(String::deserialize(d)?).map_err(serde::de::Error::custom)
            }
        }
    };
}

identity!(
    CallId,
    "One admitted engine invocation, including a zero-send call."
);
identity!(
    SdkRequestId,
    "One prepared send; status retries retain this identity."
);
identity!(
    ObservationId,
    "One accepted wire answer; cache and replay retain this identity."
);
identity!(
    FailureId,
    "One failed logical occurrence, never a stored answer."
);
identity!(
    AnswerId,
    "A stable logical reading of observations and ordered children."
);

#[cfg(test)]
mod tests;
