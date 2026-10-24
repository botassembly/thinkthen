//! Where requests go, which model answers, and the key that pays.

use std::fmt;

use crate::core::{Adapter, Error, ErrorKind};

pub(crate) const DEFAULT_BACKEND: &str = "systemone";

/// An API key. Its `Debug` form shows nothing of it.
#[derive(Clone)]
pub struct Key(String);

impl Key {
    /// The key `text` without surrounding whitespace, or `None` when nothing is left.
    #[must_use]
    pub fn new(text: &str) -> Option<Self> {
        let text = text.trim();
        (!text.is_empty()).then(|| Self(text.to_owned()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Key(..)")
    }
}

/// One named remote provider's routing and model bounds.
#[derive(Debug, Clone)]
pub struct BackendEntry {
    name: String,
    adapter: Option<Adapter>,
    address: String,
    address_variable: Option<String>,
    key_variable: String,
    default_model: String,
    limits: crate::Limits,
    models: std::collections::BTreeMap<String, crate::Limits>,
}

impl BackendEntry {
    /// Describe a named backend without resolving credentials.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        adapter: Adapter,
        address: impl Into<String>,
        key_variable: impl Into<String>,
        default_model: impl Into<String>,
        limits: crate::Limits,
    ) -> Self {
        Self {
            name: name.into(),
            adapter: Some(adapter),
            address: address.into(),
            address_variable: None,
            key_variable: key_variable.into(),
            default_model: default_model.into(),
            limits,
            models: std::collections::BTreeMap::new(),
        }
    }

    /// Set the bounds for a particular model instead of the entry's default bounds.
    #[must_use]
    pub fn model_limits(mut self, model: impl Into<String>, limits: crate::Limits) -> Self {
        self.models.insert(model.into(), limits);
        self
    }
}

/// Named backend definitions resolved once when a client is created.
#[derive(Debug, Clone)]
pub struct BackendTable {
    entries: Vec<BackendEntry>,
}

impl Default for BackendTable {
    #[expect(clippy::expect_used, reason = "built-in bounds are nonzero constants")]
    fn default() -> Self {
        use std::num::NonZeroUsize;
        let limits = crate::Limits {
            request_bytes: NonZeroUsize::new(96_000).expect("nonzero"),
            questions: None,
            options: NonZeroUsize::new(255),
        };
        let mut systemone = BackendEntry::new(
            DEFAULT_BACKEND,
            Adapter::SystemOne,
            "https://api.typesafe.ai/v1",
            "THINKTHEN_API_KEY",
            "jev-1.13.0",
            limits,
        );
        systemone.address_variable = Some("THINKTHEN_BASE_URL".to_owned());
        let openai = BackendEntry::new(
            "openai",
            Adapter::OpenAi,
            "https://api.openai.com/v1",
            "OPENAI_API_KEY",
            "gpt-6-luna",
            crate::Limits {
                questions: NonZeroUsize::new(400),
                ..limits
            },
        );
        let stub = BackendEntry {
            name: "stub".to_owned(),
            adapter: None,
            address: "stub".to_owned(),
            address_variable: None,
            key_variable: String::new(),
            default_model: "stub".to_owned(),
            limits,
            models: std::collections::BTreeMap::new(),
        };
        Self {
            entries: vec![systemone, openai, stub],
        }
    }
}

impl BackendTable {
    /// Add an entry, replacing the definition with the same name if present.
    pub fn insert(&mut self, entry: BackendEntry) {
        self.entries.retain(|held| held.name != entry.name);
        self.entries.push(entry);
    }

    /// Resolve a selected entry and its model bounds. Only this entry's variables are read.
    ///
    /// # Errors
    /// Returns a usage error for an unknown name or unsafe URL, or a missing-key error.
    pub fn resolve(
        &self,
        name: &str,
        url: Option<&str>,
        model: Option<&str>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Backend, Error> {
        self.resolve_key(name, url, model, None, variable)
    }

    pub(crate) fn resolve_key(
        &self,
        name: &str,
        url: Option<&str>,
        model: Option<&str>,
        key_source: Option<&str>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Backend, Error> {
        let (entry, model) = self.selected(name, model)?;
        if key_source.is_some_and(|source| entry.adapter.is_none() || source != entry.key_variable)
        {
            return Err(Error::new(
                ErrorKind::KeySource,
                "key_source must name the selected backend's own key variable",
            ));
        }
        let Some(adapter) = entry.adapter else {
            return Ok(Backend {
                name: entry.name.clone(),
                remote: None,
                model: entry.default_model.clone(),
                limits: entry.limits,
            });
        };
        let base = url
            .map(str::to_owned)
            .or_else(|| entry.address_variable.as_deref().and_then(&variable))
            .unwrap_or_else(|| entry.address.clone());
        validate_url(&base)?;
        let key = variable(&entry.key_variable)
            .as_deref()
            .and_then(Key::new)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::KeyMissing,
                    "the selected backend's key variable is unset or blank, so no key is sent",
                )
            })?;
        let model = model.to_owned();
        let limits = entry.models.get(&model).copied().unwrap_or(entry.limits);
        Ok(Backend {
            name: entry.name.clone(),
            remote: Some(Remote { base, key, adapter }),
            model,
            limits,
        })
    }

    /// Select a backend and effective model without resolving credentials or addresses.
    ///
    /// # Errors
    /// Returns a usage error if the backend name is unknown.
    pub fn selection<'a>(
        &'a self,
        name: &str,
        model: Option<&'a str>,
    ) -> Result<(&'a str, &'a str), Error> {
        let (entry, model) = self.selected(name, model)?;
        Ok((&entry.name, model))
    }

    fn selected<'a>(
        &'a self,
        name: &str,
        model: Option<&'a str>,
    ) -> Result<(&'a BackendEntry, &'a str), Error> {
        let entry = self
            .entries
            .iter()
            .find(|entry| entry.name == name)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::Backend,
                    "the backend name is not in the backend table",
                )
            })?;
        let model = if entry.adapter.is_none() {
            &entry.default_model
        } else {
            model.unwrap_or(&entry.default_model)
        };
        Ok((entry, model))
    }

    #[expect(
        clippy::expect_used,
        reason = "the built-in table defines each wire adapter"
    )]
    fn for_adapter(&self, adapter: Adapter) -> &BackendEntry {
        self.entries
            .iter()
            .find(|entry| entry.adapter == Some(adapter))
            .expect("built-in adapter")
    }
}

/// One admitted backend and the selected model's limits.
#[derive(Debug, Clone)]
pub struct Backend {
    name: String,
    remote: Option<Remote>,
    model: String,
    limits: crate::Limits,
}

#[derive(Debug, Clone)]
pub(crate) struct Remote {
    base: String,
    pub(crate) key: Key,
    pub(crate) adapter: Adapter,
}

impl Backend {
    /// A System One backend at `base` that asks `model` and pays with `key`.
    #[must_use]
    pub fn new(base: impl Into<String>, key: Key, model: impl Into<String>) -> Self {
        let table = BackendTable::default();
        let entry = table.for_adapter(Adapter::SystemOne);
        Self {
            name: entry.name.clone(),
            remote: Some(Remote {
                base: base.into(),
                key,
                adapter: Adapter::SystemOne,
            }),
            model: model.into(),
            limits: entry.limits,
        }
    }

    /// A deterministic backend that reads no variables and opens no connection.
    #[must_use]
    #[expect(
        clippy::expect_used,
        reason = "the built-in stub has no admission failure"
    )]
    pub fn stub() -> Self {
        Self::resolve_named("stub", None, None, |_| None).expect("built-in stub")
    }

    /// Resolve the default backend through the built-in table.
    ///
    /// # Errors
    /// Returns the selected backend's admission error.
    pub fn resolve(
        url: Option<&str>,
        model: Option<&str>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, Error> {
        Self::resolve_named(DEFAULT_BACKEND, url, model, variable)
    }

    /// Resolve a plain backend name through the built-in table.
    ///
    /// # Errors
    /// Returns the selected backend's admission error.
    pub fn resolve_named(
        name: &str,
        url: Option<&str>,
        model: Option<&str>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, Error> {
        BackendTable::default().resolve(name, url, model, variable)
    }

    /// Resolve the built-in entry for a wire adapter.
    ///
    /// # Errors
    /// Returns the selected backend's admission error.
    pub fn resolve_adapter(
        adapter: Adapter,
        url: Option<&str>,
        model: Option<&str>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Self, Error> {
        let table = BackendTable::default();
        table.resolve(&table.for_adapter(adapter).name, url, model, variable)
    }

    /// Select the built-in entry for a wire adapter, retaining explicit connection settings.
    #[must_use]
    pub fn adapter(mut self, adapter: Adapter) -> Self {
        if let Some(remote) = &mut self.remote {
            let table = BackendTable::default();
            let entry = table.for_adapter(adapter);
            remote.adapter = adapter;
            self.name.clone_from(&entry.name);
            self.limits = entry
                .models
                .get(&self.model)
                .copied()
                .unwrap_or(entry.limits);
        }
        self
    }

    pub(crate) fn url(&self) -> String {
        self.remote.as_ref().map_or_else(
            || self.name.clone(),
            |remote| {
                format!(
                    "{}/{}",
                    remote.base.trim_end_matches('/'),
                    remote.adapter.path()
                )
            },
        )
    }

    pub(crate) fn remote(&self) -> Option<&Remote> {
        self.remote.as_ref()
    }

    /// The model this backend asks.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn limits(&self) -> crate::Limits {
        self.limits
    }
    pub(crate) fn wire_adapter(&self) -> Adapter {
        self.remote
            .as_ref()
            .map_or(Adapter::SystemOne, |remote| remote.adapter)
    }
}

/// Refuse unencrypted remote connections before credentials can be sent.
pub(crate) fn validate_url(url: &str) -> Result<ureq::http::Uri, Error> {
    let uri = url
        .parse::<ureq::http::Uri>()
        .map_err(|_| Error::new(ErrorKind::Url, "the backend address is not a valid URL"))?;
    match (uri.scheme_str(), uri.host()) {
        (Some("https"), Some(_)) | (Some("http"), Some("localhost" | "127.0.0.1" | "[::1]")) => {
            Ok(uri)
        }
        _ => Err(Error::new(
            ErrorKind::Url,
            "the backend address must use HTTPS, or HTTP to localhost, 127.0.0.1 or [::1]",
        )),
    }
}
