//! The wire format a backend speaks.

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The name given for an adapter that does not exist.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("no adapter is named `{0}`")]
pub struct UnknownAdapterError(String);

/// The wire formats an adapter speaks. Version one speaks one of them.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Adapter {
    /// The `systemone` request and response format.
    SystemOne,
}

impl Adapter {
    /// Read the lowercase name this adapter is known by.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemOne => "systemone",
        }
    }
}

impl FromStr for Adapter {
    type Err = UnknownAdapterError;

    /// Read an adapter from the lowercase name a profile or a flag gives.
    ///
    /// # Errors
    ///
    /// Returns [`UnknownAdapterError`] for any other name.
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "systemone" => Ok(Self::SystemOne),
            other => Err(UnknownAdapterError(other.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Adapter, UnknownAdapterError};

    #[test]
    fn an_adapter_parses_from_its_lowercase_name() {
        assert_eq!("systemone".parse(), Ok(Adapter::SystemOne));
        for case in ["", "SystemOne", "systemone ", "chat-logprobs", "system one"] {
            assert_eq!(
                case.parse::<Adapter>(),
                Err(UnknownAdapterError(case.to_owned())),
                "{case:?}"
            );
        }
    }
}
