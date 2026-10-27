//! Where requests go, which model answers, and the key that pays.

use std::fmt;

use crate::core::{Adapter, Error, ErrorKind};

pub(crate) const DEFAULT_BACKEND: &str = crate::BackendName::SystemOne.as_str();

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

impl std::str::FromStr for Key {
    type Err = Error;
    fn from_str(text: &str) -> Result<Self, Error> {
        Self::new(text).ok_or_else(|| {
            Error::new(
                ErrorKind::KeyMissing,
                crate::core::diagnostic::requirement(
                    "api_key",
                    "a nonblank credential",
                    "a blank credential (contents redacted)",
                ),
            )
        })
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Key(..)")
    }
}

/// One named remote provider's routing and model bounds.
#[derive(Debug, Clone)]
pub(crate) struct BackendEntry {
    name: String,
    adapter: Option<Adapter>,
    address: String,
    address_variable: Option<String>,
    key_variable: String,
    default_model: String,
    limits: crate::Limits,
}

impl BackendEntry {
    /// Describe a named backend without resolving credentials.
    #[must_use]
    pub(crate) fn new(
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
        }
    }
}

/// Named backend definitions resolved once when a client is created.
#[derive(Debug, Clone)]
pub(crate) struct BackendTable {
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
            crate::BackendName::OpenAi.as_str(),
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
            name: crate::BackendName::Stub.as_str().to_owned(),
            adapter: None,
            address: "stub".to_owned(),
            address_variable: None,
            key_variable: String::new(),
            default_model: "stub".to_owned(),
            limits,
        };
        Self {
            entries: vec![systemone, openai, stub],
        }
    }
}

impl crate::BackendName {
    /// The built-in model without reading credentials or environment variables.
    #[must_use]
    pub fn default_model(self) -> String {
        BackendTable::default()
            .entries
            .into_iter()
            .find(|entry| entry.name == self.as_str())
            .map_or_else(String::new, |entry| entry.default_model)
    }
}

impl BackendTable {
    #[cfg(feature = "command")]
    pub(crate) fn key_variables(&self) -> impl Iterator<Item = &str> {
        self.entries
            .iter()
            .map(|entry| entry.key_variable.as_str())
            .filter(|name| !name.is_empty())
    }

    pub(crate) fn resolve_direct(
        &self,
        name: &str,
        url: Option<&str>,
        model: Option<&str>,
        api_key: Option<&Key>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Backend, Error> {
        self.resolve_options(name, url, model, api_key, variable)
    }

    fn resolve_options(
        &self,
        name: &str,
        url: Option<&str>,
        model: Option<&str>,
        api_key: Option<&Key>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<Backend, Error> {
        let (entry, model) = self.selected(name, model)?;
        let Some(adapter) = entry.adapter else {
            return Ok(Backend {
                name: entry.name.clone(),
                backend_name: name.parse()?,
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
        let key = api_key
            .cloned()
            .or_else(|| variable(&entry.key_variable).as_deref().and_then(Key::new))
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::KeyMissing,
                    crate::core::diagnostic::requirement(
                        "api_key",
                        "a credential in the selected backend key variable",
                        "an unset or blank credential (contents redacted)",
                    ),
                )
            })?;
        let model = model.to_owned();
        let limits = entry.limits;
        Ok(Backend {
            name: entry.name.clone(),
            backend_name: name.parse()?,
            remote: Some(Remote { base, key, adapter }),
            model,
            limits,
        })
    }

    /// Select a backend and effective model without resolving credentials or addresses.
    ///
    /// # Errors
    /// Returns a usage error if the backend name is unknown.
    #[cfg(feature = "native")]
    pub(crate) fn selection<'a>(
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
                    crate::core::diagnostic::requirement(
                        "backend",
                        "a name in the backend table",
                        "an unknown name (contents redacted)",
                    ),
                )
            })?;
        let model = if entry.adapter.is_none() {
            &entry.default_model
        } else {
            model.unwrap_or(&entry.default_model)
        };
        Ok((entry, model))
    }
}

/// One admitted backend and the selected model's limits.
#[derive(Debug, Clone)]
pub(crate) struct Backend {
    name: String,
    backend_name: crate::BackendName,
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
    pub(crate) fn backend_name(&self) -> crate::BackendName {
        self.backend_name
    }
    pub(crate) fn base_url(&self) -> Option<&str> {
        self.remote.as_ref().map(|remote| remote.base.as_str())
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
    pub(crate) fn model(&self) -> &str {
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
    let uri = url.parse::<ureq::http::Uri>().map_err(|_| {
        Error::new(
            ErrorKind::Url,
            crate::core::diagnostic::requirement(
                "url",
                "a valid backend URL",
                "an invalid URL (contents redacted)",
            ),
        )
    })?;
    match (uri.scheme_str(), uri.host()) {
        (Some("https"), Some(_)) | (Some("http"), Some("localhost" | "127.0.0.1" | "[::1]")) => {
            Ok(uri)
        }
        _ => Err(Error::new(
            ErrorKind::Url,
            crate::core::diagnostic::requirement(
                "url",
                "HTTPS, or HTTP to localhost, 127.0.0.1 or [::1]",
                "an insecure nonlocal URL (contents redacted)",
            ),
        )),
    }
}
