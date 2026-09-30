//! Named backends on the engine builder (ADR 0114).
//!
//! `from_env` captures the environment and configuration tiers and every
//! nonblank key variable a built-in or a configuration entry names. The setter
//! and `build` read no environment; they choose from what was captured.

use std::collections::BTreeMap;
use std::fmt;

use super::{EngineBuilder, Secret};
use crate::core::named::{self, Named};
use crate::core::{Backend, DEFAULT_MODEL, ModelName};
use crate::public::error::Error;

/// What `from_env` captured below the engine settings: the environment tier,
/// the configuration tier, and the key variables. A bare builder holds none.
#[derive(Default)]
pub(super) struct Captured {
    pub(super) base_url: Option<String>,
    pub(super) backend: Option<String>,
    pub(super) config_url: Option<String>,
    pub(super) config_backend: Option<String>,
    pub(super) config_model: Option<ModelName>,
    pub(super) configured: Vec<Named>,
    /// Each named variable that was set: its key, or `None` when it is not UTF-8.
    pub(super) keys: BTreeMap<String, Option<Secret>>,
}

impl fmt::Debug for Captured {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let withheld = |value: &Option<String>| value.as_ref().map(|_| "<withheld>");
        formatter
            .debug_struct("Captured")
            .field("base_url", &withheld(&self.base_url))
            .field("backend", &withheld(&self.backend))
            .field("config_url", &withheld(&self.config_url))
            .field("config_backend", &self.config_backend)
            .field("config_model", &self.config_model)
            .field("configured", &self.configured)
            .field("keys", &self.keys.len())
            .finish()
    }
}

/// The resolved backend, the key it sends, and the variable a missing key names.
pub(super) struct Selected {
    pub(super) backend: Backend,
    pub(super) key: Option<Secret>,
    pub(super) variable: String,
}

impl EngineBuilder {
    /// Use this named backend: its base, its key variables, and its model.
    ///
    /// It outranks a captured `THINKTHEN_BACKEND` and `THINKTHEN_BASE_URL` and
    /// the configuration file. An explicit [`EngineBuilder::base_url`] sends
    /// this backend's key to that address instead of its base. A builder from
    /// [`EngineBuilder::from_env`] knows the built-ins and the configuration's
    /// backends; `Engine::builder()` knows the built-ins alone and reads no
    /// key, so pair it with [`EngineBuilder::api_key`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for an invalid or unknown name.
    pub fn backend(mut self, name: &str) -> Result<Self, Error> {
        named::find(name, &self.captured.configured).map_err(Error::refused)?;
        self.backend = Some(name.to_owned());
        Ok(self)
    }

    /// Choose the backend from the three tiers and select its key. An explicit
    /// key outranks every variable and skips the host rule, because the caller
    /// chose both the key and the backend.
    pub(super) fn selected(&self) -> Result<Selected, Error> {
        let captured = &self.captured;
        let tiers = [
            (self.backend.as_deref(), self.base_url.as_deref()),
            (captured.backend.as_deref(), captured.base_url.as_deref()),
            (
                captured.config_backend.as_deref(),
                captured.config_url.as_deref(),
            ),
        ];
        let choice = named::choose(&tiers, &captured.configured).map_err(Error::refused)?;
        let unnamed_model = captured
            .config_model
            .as_ref()
            .map_or(DEFAULT_MODEL, ModelName::as_str);
        let backend = choice
            .backend(self.model.as_ref().map(ModelName::as_str), unnamed_model)
            .map_err(Error::refused)?;
        let key = match &self.key {
            Some(key) => Some(key.clone()),
            None => {
                choice.guard(&backend).map_err(Error::refused)?;
                first_key(&captured.keys, &choice.keys())?
            }
        };
        Ok(Selected {
            backend,
            key,
            variable: choice.key_variable().to_owned(),
        })
    }
}

/// The first set variable of `names`, in order. A selected variable that is
/// not UTF-8 is refused by name.
fn first_key(
    keys: &BTreeMap<String, Option<Secret>>,
    names: &[&str],
) -> Result<Option<Secret>, Error> {
    for name in names {
        match keys.get(*name) {
            Some(Some(key)) => return Ok(Some(key.clone())),
            Some(None) => return Err(Error::usage(format!("{name} is not valid UTF-8"))),
            None => {}
        }
    }
    Ok(None)
}
