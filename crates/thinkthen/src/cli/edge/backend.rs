//! The backend a command uses, chosen once from the typed, environment, and
//! configuration tiers (ADR 0114 section 3).

use super::{Environment, key};
use crate::core::named::{self, Choice};
use crate::core::{Backend, DEFAULT_MODEL};
use crate::failure::Failure;

impl Environment {
    /// Decide which tier names the backend or the address. The typed tier is
    /// `--backend` and `--url`, then `THINKTHEN_BACKEND` and
    /// `THINKTHEN_BASE_URL`, then the configuration's `backend` and `url`.
    pub(crate) fn choose<'a>(
        &'a self,
        backend: Option<&'a str>,
        url: Option<&'a str>,
    ) -> Result<Choice<'a>, Failure> {
        let tiers = [
            (backend, url),
            (self.backend.as_deref(), self.base_url.as_deref()),
            (self.config.backend(), self.config.url()),
        ];
        Ok(named::choose(&tiers, self.config.named())?)
    }

    /// Choose and settle the backend for a command. `asked` is the model the
    /// flag or the question file names.
    pub(crate) fn resolve(
        &self,
        backend: Option<&str>,
        url: Option<&str>,
        asked: Option<&str>,
    ) -> Result<Backend, Failure> {
        let choice = self.choose(backend, url)?;
        self.settle(&choice, asked)
    }

    /// Resolve the address and model, refuse a built-in key at another
    /// built-in's host before any key is read, then read the key once.
    pub(crate) fn settle(
        &self,
        choice: &Choice<'_>,
        asked: Option<&str>,
    ) -> Result<Backend, Failure> {
        let backend = choice.backend(asked, self.model().unwrap_or(DEFAULT_MODEL))?;
        choice.guard(&backend)?;
        self.key.check(
            &backend,
            &choice.keys(),
            choice.named.as_ref().map(named::Named::name),
        )?;
        Ok(backend)
    }

    /// Whether the settled backend is a named one.
    pub(crate) fn named(&self) -> Option<&str> {
        self.key.name()
    }

    /// The first key variable of the settled backend.
    pub(crate) fn key_variable(&self) -> &str {
        self.key.variable()
    }

    /// Report the checked snapshot, without rereading the process environment.
    pub(crate) fn api_key_set(&self) -> bool {
        self.key.is_set()
    }

    /// The already checked snapshot that every request from this command uses.
    pub(crate) fn key_reader(&self) -> key::Reader {
        self.key.reader()
    }
}
