//! Named backends: each pairs a base with its own key variables and a model.
//!
//! ADR 0114 states the rules. The command, the configuration file, and the
//! Rust builder hand typed values here in tier order, and this module decides
//! which backend and address apply and which key variables are read. It reads
//! no environment.

use std::fmt;
use std::num::NonZeroU32;

use crate::core::adapters::{ApiType, built_ins};
use crate::core::backend::{Backend, BackendError, KEY_VAR};
use crate::core::plan::Descriptions;
use crate::core::{BackendProfile, Prices};

/// One named backend: a built-in or a configuration entry.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Named {
    api_type: ApiType,
    name: String,
    base: String,
    path: String,
    keys: Vec<String>,
    model: String,
    descriptions: Descriptions,
    /// The configuration file's rate for this backend; a built-in has none of its own.
    per_minute: Option<NonZeroU32>,
    prices: Option<Prices>,
    profile: Option<BackendProfile>,
}

impl fmt::Debug for Named {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Named")
            .field("name", &self.name)
            .field("base", &"<withheld>")
            .field("keys", &self.keys)
            .field("model", &self.model)
            .field("descriptions", &self.descriptions)
            .field("per_minute", &self.per_minute)
            .finish()
    }
}

impl Named {
    /// A configuration entry, whose fields the configuration reader checked.
    /// Its descriptions start as authored; the validated setup may select BothSides.
    pub(crate) fn new(name: &str, base: &str, key: &str, model: &str) -> Self {
        Self {
            api_type: ApiType::Primary,
            name: name.to_owned(),
            base: base.to_owned(),
            path: crate::core::adapters::built_in::ENDPOINT_PATH.to_owned(),
            keys: vec![key.to_owned()],
            model: model.to_owned(),
            descriptions: Descriptions::Authored,
            per_minute: None,
            prices: None,
            profile: None,
        }
    }

    /// Use a relative path already validated by the configuration reader.
    pub(crate) fn with_path(mut self, path: &str) -> Self {
        self.path = path.to_owned();
        self
    }

    /// Pace this backend at the rate its configuration entry sets.
    #[must_use]
    pub(crate) const fn with_per_minute(mut self, rate: Option<NonZeroU32>) -> Self {
        self.per_minute = rate;
        self
    }

    /// Apply validated setup settings without changing transport identity.
    pub(crate) fn with_setup(
        mut self,
        prices: Option<Prices>,
        profile: Option<BackendProfile>,
        both_sides: bool,
    ) -> Self {
        self.prices = prices;
        self.profile = profile;
        if both_sides {
            self.descriptions = Descriptions::BothSides;
        }
        self
    }

    /// The built-in backend of this name, if one exists.
    pub(crate) fn built_in(name: &str) -> Option<Self> {
        built_ins()
            .find(|(built_in, _)| built_in.name == name)
            .map(|(built_in, api_type)| Self {
                api_type,
                name: built_in.name.to_owned(),
                base: built_in.base.to_owned(),
                path: built_in.path.to_owned(),
                keys: built_in.keys.iter().map(|&key| key.to_owned()).collect(),
                model: built_in.model.to_owned(),
                descriptions: built_in.descriptions,
                per_minute: None,
                prices: None,
                profile: None,
            })
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// The key variables, in the order they are read.
    pub(crate) fn keys(&self) -> &[String] {
        &self.keys
    }
}

/// Whether a name uses 1 to 32 lowercase ASCII letters, digits, and hyphens.
pub(crate) fn valid_name(name: &str) -> bool {
    (1..=32).contains(&name.len())
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

/// Every key variable a built-in backend reads.
pub(crate) fn built_in_keys() -> impl Iterator<Item = &'static str> {
    built_ins().flat_map(|(built_in, _)| built_in.keys.iter().copied())
}

/// The built-in names as the unknown-name sentence lists them.
pub(crate) fn built_in_list() -> String {
    let mut names: Vec<String> = built_ins()
        .map(|(built_in, _)| format!("`{}`", built_in.name))
        .collect();
    names.sort();
    match names.split_last() {
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
        None => String::new(),
    }
}

/// Find a backend by name among the configured entries and the built-ins.
///
/// # Errors
///
/// Returns [`BackendError::InvalidName`] or [`BackendError::Unknown`].
pub(crate) fn find(name: &str, configured: &[Named]) -> Result<Named, BackendError> {
    if !valid_name(name) {
        return Err(BackendError::InvalidName);
    }
    configured
        .iter()
        .find(|entry| entry.name == name)
        .cloned()
        .or_else(|| Named::built_in(name))
        .ok_or_else(|| BackendError::Unknown(name.to_owned()))
}

/// One tier's backend name and address, either of which may be absent.
pub(crate) type Tier<'a> = (Option<&'a str>, Option<&'a str>);

/// What the deciding tier chose: a named backend, an address, both, or neither.
#[derive(Clone, Debug)]
pub(crate) struct Choice<'a> {
    pub(crate) named: Option<Named>,
    pub(crate) url: Option<&'a str>,
    /// The index of the tier that decided, or `None` when no tier named anything.
    pub(crate) tier: Option<usize>,
    /// Configured built-ins whose settings may enrich an exact unnamed posting URL.
    setups: Vec<Named>,
}

/// Walk the tiers from the top; the first tier that names a backend or an
/// address decides, and every lower tier is ignored.
///
/// # Errors
///
/// Returns [`BackendError`] when the deciding tier names an invalid or unknown backend.
pub(crate) fn choose<'a>(
    tiers: &[Tier<'a>],
    configured: &[Named],
) -> Result<Choice<'a>, BackendError> {
    let setups: Vec<Named> = configured
        .iter()
        .filter(|entry| Named::built_in(&entry.name).is_some())
        .cloned()
        .collect();
    for (index, &(name, url)) in tiers.iter().enumerate() {
        if name.is_none() && url.is_none() {
            continue;
        }
        let named = name.map(|name| find(name, configured)).transpose()?;
        return Ok(Choice {
            named,
            url,
            tier: Some(index),
            setups,
        });
    }
    Ok(Choice {
        named: None,
        url: None,
        tier: None,
        setups,
    })
}

impl Choice<'_> {
    /// Resolve the address and the model. A named backend supplies its base
    /// when its tier named no address, and its model when nothing was asked.
    /// The unnamed path takes `unnamed_model` as today, and the rate of a
    /// built-in whose base posts to the same URL, so a rate on the built-in
    /// whose base is the default address paces a run that names nothing.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] for an address or model the backend rule refuses.
    pub(crate) fn backend(
        &self,
        asked: Option<&str>,
        unnamed_model: &str,
    ) -> Result<Backend, BackendError> {
        match &self.named {
            None => {
                Backend::resolve(self.url, None, asked.unwrap_or(unnamed_model)).map(|backend| {
                    let rate = self
                        .unnamed_setup(&backend)
                        .and_then(|entry| entry.per_minute);
                    backend.with_per_minute(rate)
                })
            }
            Some(named) => Backend::resolve_path(
                Some(self.url.unwrap_or(&named.base)),
                None,
                asked.unwrap_or(&named.model),
                &named.path,
            )
            .map(|backend| {
                backend
                    .with_api_type(named.api_type)
                    .with_descriptions(named.descriptions)
                    .with_image_route(crate::core::adapters::built_in::images::named(
                        &named.name,
                        &named.path,
                    ))
                    .with_per_minute(named.per_minute)
            }),
        }
    }

    /// The configured built-in whose own canonical posting URL matches.
    fn unnamed_setup(&self, backend: &Backend) -> Option<&Named> {
        let posts_here = |entry: &&Named| {
            Backend::resolve_path(
                Some(&entry.base),
                None,
                backend.model().as_str(),
                &entry.path,
            )
            .is_ok_and(|base| base.url() == backend.url())
        };
        self.setups.iter().find(posts_here)
    }

    /// Settings from the named setup or an exact canonical built-in posting URL.
    pub(crate) fn setup(&self, backend: &Backend) -> (Option<Prices>, Option<BackendProfile>) {
        let entry = self.named.as_ref().or_else(|| self.unnamed_setup(backend));
        entry.map_or((None, None), |entry| (entry.prices, entry.profile.clone()))
    }

    /// The key variables this choice reads, in order.
    pub(crate) fn keys(&self) -> Vec<&str> {
        self.named.as_ref().map_or_else(
            || vec![KEY_VAR],
            |named| named.keys.iter().map(String::as_str).collect(),
        )
    }

    /// The first key variable, which the missing-key sentence names.
    pub(crate) fn key_variable(&self) -> &str {
        self.named
            .as_ref()
            .and_then(|named| named.keys.first())
            .map_or(KEY_VAR, String::as_str)
    }

    /// Refuse a built-in's key variable at another built-in's host (ADR 0114
    /// section 5). A built-in whose base is loopback is no other built-in's
    /// host, because any program on this machine may listen there and the
    /// user named that address (ADR 0115 section 2).
    ///
    /// # Errors
    ///
    /// Returns [`BackendError::KeyElsewhere`], which names no address and no key.
    pub(crate) fn guard(&self, backend: &Backend) -> Result<(), BackendError> {
        let (Some(named), Some(host)) = (&self.named, backend.host().map(comparable)) else {
            return Ok(());
        };
        for variable in &named.keys {
            let Some((owner, _)) =
                built_ins().find(|(built_in, _)| built_in.keys.contains(&variable.as_str()))
            else {
                continue;
            };
            let other = built_ins().find(|(built_in, _)| {
                built_in.name != owner.name
                    && Backend::resolve(Some(built_in.base), None, built_in.model)
                        .ok()
                        .filter(|base| !base.is_loopback())
                        .is_some_and(|base| base.host().map(comparable).as_ref() == Some(&host))
            });
            if let Some((other, _)) = other {
                return Err(BackendError::KeyElsewhere {
                    name: named.name.clone(),
                    variable: variable.clone(),
                    owner: owner.name,
                    other: other.name,
                });
            }
        }
        Ok(())
    }
}

/// A host as DNS reads it: ASCII percent escapes decoded, lower case, and
/// trailing dots dropped, so `api.liquid.ai.` and `api%2Eliquid.ai` compare
/// equal to `api.liquid.ai`.
fn comparable(host: &str) -> String {
    let bytes = host.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        let escaped = (byte == b'%')
            .then(|| bytes.get(index + 1..index + 3))
            .flatten()
            .and_then(|pair| std::str::from_utf8(pair).ok())
            .and_then(|pair| u8::from_str_radix(pair, 16).ok());
        if let Some(value) = escaped {
            decoded.push(value);
            index += 3;
        } else {
            decoded.push(byte);
            index += 1;
        }
    }
    String::from_utf8_lossy(&decoded)
        .to_ascii_lowercase()
        .trim_end_matches('.')
        .to_owned()
}

#[cfg(test)]
mod tests;

/// Whether a suffix uses the configuration's relative segment grammar.
pub(crate) fn valid_path(path: &str) -> bool {
    (1..=128).contains(&path.len())
        && path.split('/').all(|segment| {
            !segment.is_empty()
                && !matches!(segment, "." | "..")
                && segment.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'@')
                })
        })
}

#[cfg(test)]
mod posting_path_tests;
