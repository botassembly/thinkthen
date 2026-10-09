//! The backend a request goes to: one address and one model.

use std::num::NonZeroU32;

use thiserror::Error;

use crate::core::adapters::built_in;
use crate::core::plan::Descriptions;
use crate::core::text::{BlankTextError, ModelName, Url};

pub(crate) mod named;

/// The one environment variable that holds the key.
///
/// The key goes to the address the user named, because naming the address is
/// the user's own act. `specification/backends.md` states the rule.
pub(crate) const KEY_VAR: &str = "THINKTHEN_API_KEY";

/// The same refusal at the CLI and public builder boundaries.
pub(crate) const KEY_IN_ADDRESS: &str =
    "the backend address contains the API key; keep the key out of the address";

/// The highest requests-a-minute rate the variable or the configuration file takes.
pub(crate) const MAX_PER_MINUTE: u32 = 60_000;

/// Where one request goes, which model it names, how its descriptions travel,
/// and the rate its named backend's configuration entry sets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Backend {
    api_type: crate::core::adapters::ApiType,
    accounting: InputAccounting,
    url: Url,
    model: ModelName,
    descriptions: Descriptions,
    request_size: usize,
    explicit_request_size: bool,
    image_route: built_in::images::ImageRoute,
    per_minute: Option<NonZeroU32>,
}

/// How the selected route estimates text shared across wire questions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum InputAccounting {
    #[default]
    EncodedBody,
    RepeatedState,
}

/// The default maximum request size at every address, in bytes.
const DEFAULT_REQUEST_SIZE: usize = 96_000;

/// Why the given address and model name no backend.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum BackendError {
    /// The base or the model arrived blank.
    #[error(transparent)]
    Blank(#[from] BlankTextError),
    /// The base does not begin with a scheme the tool speaks.
    ///
    /// The message shows no address, because the refused base is the one that
    /// may carry a secret.
    #[error("a base address begins with `http://` or `https://`")]
    InvalidScheme,
    /// The base carries credentials in its authority.
    #[error("a base address carries no user information")]
    UserInformation,
    /// The base carries a port that is not a decimal `u16`.
    #[error("a port is digits naming a number from 0 to 65535")]
    InvalidPort,
    /// The base carries a query or fragment.
    #[error("a base address carries no query or fragment")]
    QueryOrFragment,
    /// The scheme is valid, but the authority has no host.
    #[error("a base address has a host")]
    NotAnAddress,
    /// The base names a host under `http://` that is not proven to be loopback.
    ///
    /// The message shows no address, for the reason [`Self::NotAnAddress`]
    /// gives. No option overrides the rule: a backend on another machine is
    /// reached over `https://`, or over a tunnel that ends on loopback.
    #[error(
        "`http://` sends the key across the network in clear text, \
         so it reaches localhost, 127.0.0.1, and [::1] alone"
    )]
    KeyInClear,
    #[error("a backend name uses 1 to 32 lowercase letters, digits, and hyphens")]
    InvalidName,
    #[error("unknown backend `{0}`; the built-in backends are {list}, and the configuration file may name more", list = named::built_in_list())]
    Unknown(String),
    /// A built-in's key variable at another built-in's host (ADR 0114 section 5).
    #[error(
        "backend `{name}` reads `{variable}`, the key of backend `{owner}`, which never goes to the address of backend `{other}`"
    )]
    KeyElsewhere {
        name: String,
        variable: String,
        owner: &'static str,
        other: &'static str,
    },
}

/// The three host spellings `http://` may carry.
///
/// Each one names this machine and nothing else, so the key never leaves it.
/// Every other spelling is refused, including one that would resolve to
/// loopback, because this rule reads text and resolves no name. `localhost.`,
/// `127.1`, `0.0.0.0`, and `[::ffff:127.0.0.1]` are therefore all refused.
const LOOPBACK: [&str; 3] = ["localhost", "127.0.0.1", "[::1]"];

impl Backend {
    pub(crate) const DEFAULT_REQUEST_SIZE: usize = DEFAULT_REQUEST_SIZE;

    #[cfg(test)]
    pub(crate) const fn from_parts(url: Url, model: ModelName) -> Self {
        Self {
            api_type: crate::core::adapters::ApiType::Primary,
            accounting: InputAccounting::EncodedBody,
            url,
            model,
            descriptions: Descriptions::Authored,
            request_size: DEFAULT_REQUEST_SIZE,
            explicit_request_size: false,
            image_route: built_in::images::ImageRoute::Unsupported,
            per_minute: None,
        }
    }

    /// Resolve the backend from the option, the environment, and the default.
    ///
    /// Every source names a base, and the request is posted to the base with
    /// the adapter's endpoint path after it. `--url` outranks
    /// `THINKTHEN_BASE_URL`, which outranks the adapter's default base.
    /// The binary reads the variable at its edge and hands the value here, so
    /// the core still reads no environment of its own.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when the base or the model is blank, when the
    /// base names no `http` or `https` address, and when a plain `http://` base
    /// names a host this rule cannot prove is loopback.
    pub(crate) fn resolve(
        url: Option<&str>,
        base: Option<&str>,
        model: &str,
    ) -> Result<Self, BackendError> {
        Self::resolve_path(url, base, model, built_in::ENDPOINT_PATH)
    }

    /// Resolve a named backend at its validated relative posting path.
    pub(crate) fn resolve_path(
        url: Option<&str>,
        base: Option<&str>,
        model: &str,
        path: &str,
    ) -> Result<Self, BackendError> {
        Ok(Self {
            api_type: crate::core::adapters::ApiType::Primary,
            accounting: InputAccounting::EncodedBody,
            url: address(url.or(base).unwrap_or(built_in::DEFAULT_BASE), path)?,
            model: ModelName::new(model)?,
            descriptions: Descriptions::Authored,
            request_size: DEFAULT_REQUEST_SIZE,
            explicit_request_size: false,
            image_route: built_in::images::ImageRoute::Unsupported,
            per_minute: None,
        })
    }

    pub(crate) const fn with_api_type(mut self, api: crate::core::adapters::ApiType) -> Self {
        self.api_type = api;
        self
    }
    pub(crate) const fn api_type(&self) -> crate::core::adapters::ApiType {
        self.api_type
    }

    pub(crate) const fn with_accounting(mut self, accounting: InputAccounting) -> Self {
        self.accounting = accounting;
        self
    }

    pub(crate) const fn accounting(&self) -> InputAccounting {
        self.accounting
    }

    /// Send descriptions in this form. Only a named backend sets one other
    /// than [`Descriptions::Authored`] (ADR 0115).
    pub(crate) const fn with_descriptions(mut self, descriptions: Descriptions) -> Self {
        self.descriptions = descriptions;
        self
    }

    pub(crate) const fn with_image_route(mut self, route: built_in::images::ImageRoute) -> Self {
        self.image_route = route;
        self
    }

    pub(crate) const fn image_route(&self) -> built_in::images::ImageRoute {
        self.image_route
    }

    pub(crate) fn with_model(mut self, model: ModelName) -> Self {
        self.model = model;
        self
    }

    /// Pace at the rate the selected backend's configuration entry sets.
    /// `THINKTHEN_REQUESTS_PER_MINUTE` outranks it where the engine is built.
    pub(crate) const fn with_per_minute(mut self, rate: Option<NonZeroU32>) -> Self {
        self.per_minute = rate;
        self
    }

    /// The configuration file's rate for this backend, if it set one.
    pub(crate) const fn per_minute(&self) -> Option<NonZeroU32> {
        self.per_minute
    }

    /// Carry the command's resolved request size into both planners.
    pub(crate) fn with_request_size(mut self, size: usize) -> Self {
        self.request_size = size;
        self.explicit_request_size = true;
        self
    }

    /// Whether the address names `localhost`, `127.0.0.1`, or `[::1]`, the
    /// hosts the clear-text rule proves are this machine.
    #[must_use]
    pub(crate) fn is_loopback(&self) -> bool {
        self.host()
            .is_some_and(|host| LOOPBACK.iter().any(|kind| host.eq_ignore_ascii_case(kind)))
    }

    /// The host of the posting URL, lower case and without its port.
    #[must_use]
    pub(crate) fn host(&self) -> Option<&str> {
        after_scheme(self.url.as_str())
            .and_then(|(_, rest)| host_of(rest.split('/').next().unwrap_or(rest)).ok())
    }

    /// Read the URL the request is posted to.
    #[must_use]
    pub(crate) const fn url(&self) -> &Url {
        &self.url
    }

    /// Compare the final posting URL with the effective nonblank key exactly.
    #[must_use]
    pub(crate) fn address_contains_key(&self, key: Option<&str>) -> bool {
        key.filter(|value| !value.trim().is_empty())
            .is_some_and(|value| self.url.as_str().contains(value))
    }

    /// Read the model the request names.
    #[must_use]
    pub(crate) const fn model(&self) -> &ModelName {
        &self.model
    }

    /// How this backend's descriptions travel.
    #[must_use]
    pub(crate) const fn descriptions(&self) -> Descriptions {
        self.descriptions
    }

    /// The model and description form every plan for this backend carries.
    #[must_use]
    pub(crate) fn asked(&self) -> (ModelName, Descriptions) {
        (self.model.clone(), self.descriptions)
    }

    /// The request-byte ceiling a relation plan splits under and a batch closes at.
    #[must_use]
    pub(crate) fn image_ceiling(&self) -> Option<usize> {
        self.explicit_request_size.then_some(self.request_size)
    }
    pub(crate) fn ceiling(&self) -> usize {
        self.request_size
    }

    /// Whether the posting URL names the built-in backend, for its warning.
    #[must_use]
    pub(crate) fn is_built_in(&self) -> bool {
        let base = self
            .url
            .as_str()
            .strip_suffix(built_in::ENDPOINT_PATH)
            .and_then(|rest| rest.strip_suffix('/'));
        base == Some(built_in::DEFAULT_BASE)
    }

    /// Say whether the request travels under TLS.
    ///
    /// A `false` here means the address is a loopback one, because the address
    /// rule refuses every other host under `http://`. The binary reads this to
    /// cancel any proxy for such a request: a proxy would carry it off this
    /// machine in clear text and undo the rule. Under `https://` a proxy sees
    /// the host alone, so it is left in place.
    #[must_use]
    pub(crate) fn is_secure(&self) -> bool {
        self.url.as_str().starts_with("https://")
    }
}

/// The address one request is posted to: the base, then the endpoint path.
///
/// Space around the base is not part of it, and neither are the slashes it
/// ends in, so both are dropped before the path is added. A scheme is read
/// without regard to case and written back in lower case, so one exchange
/// keeps one recording digest whatever case the caller typed.
fn address(base: &str, path: &str) -> Result<Url, BackendError> {
    let base = base.trim();
    if base.is_empty() {
        return Err(BackendError::Blank(BlankTextError::Url));
    }
    if base
        .chars()
        .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(BackendError::NotAnAddress);
    }
    let base = base.trim_end_matches('/');
    let (scheme, rest) = after_scheme(base).ok_or(BackendError::InvalidScheme)?;
    // The address is printed in a plan and kept in a recording, so a base that
    // carries user information or a query would write a secret into both. A
    // path added after either one would also land in the wrong place.
    let authority = rest.split('/').next().unwrap_or(rest);
    if authority.contains('@') {
        return Err(BackendError::UserInformation);
    }
    if rest.contains(['?', '#']) {
        return Err(BackendError::QueryOrFragment);
    }
    let host = host_of(authority)?;
    if scheme == "http://" && !LOOPBACK.iter().any(|kind| host.eq_ignore_ascii_case(kind)) {
        return Err(BackendError::KeyInClear);
    }
    let rest = canonical_rest(rest, host);
    Ok(Url::new(format!("{scheme}{rest}/{path}"))?)
}

/// Normalize an already resolved posting URL without appending an endpoint again.
pub(crate) fn posting_address(value: &str) -> Result<Url, BackendError> {
    let url = address(value, "")?;
    Url::new(url.as_str().trim_end_matches('/')).map_err(BackendError::Blank)
}

/// Lowercase literal ASCII letters in an unbracketed host and keep every other byte.
fn canonical_rest(rest: &str, host: &str) -> String {
    if host.starts_with('[') {
        return rest.to_owned();
    }
    let mut canonical = String::with_capacity(rest.len());
    let mut offset = 0;
    while let Some(remaining) = host.get(offset..)
        && !remaining.is_empty()
    {
        let bytes = remaining.as_bytes();
        if bytes.first() == Some(&b'%')
            && bytes.get(1).is_some_and(u8::is_ascii_hexdigit)
            && bytes.get(2).is_some_and(u8::is_ascii_hexdigit)
        {
            canonical.push_str(remaining.get(..3).unwrap_or_default());
            offset += 3;
        } else {
            let character = remaining.chars().next().unwrap_or_default();
            canonical.push(character.to_ascii_lowercase());
            offset += character.len_utf8();
        }
    }
    canonical.push_str(rest.get(host.len()..).unwrap_or_default());
    canonical
}

/// The host inside an authority, with the port dropped, or `None` when it holds none.
///
/// An address in brackets is an IPv6 literal, and its colons belong to the
/// address rather than to a port, so the brackets are kept and only what
/// follows them may be a port.
fn host_of(authority: &str) -> Result<&str, BackendError> {
    let end = match authority.strip_prefix('[') {
        Some(inside) => inside
            .find(']')
            .map_or(Err(BackendError::NotAnAddress), |index| Ok(index + 2))?,
        None => authority.find(':').unwrap_or(authority.len()),
    };
    let (host, port) = authority
        .split_at_checked(end)
        .ok_or(BackendError::NotAnAddress)?;
    if host.is_empty() {
        return Err(BackendError::NotAnAddress);
    }
    match port.strip_prefix(':') {
        // A port is digits that a socket can carry, or there is no colon at
        // all. An empty port, a sign, a number past 65535, and a second colon
        // each name no port, so none of them is an address this rule reads. A
        // second colon means the authority holds an address the brackets
        // should have held.
        Some(number) if !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit()) => {
            number
                .parse::<u16>()
                .map_or(Err(BackendError::InvalidPort), |_| Ok(host))
        }
        Some(_) => Err(BackendError::InvalidPort),
        None if port.is_empty() => Ok(host),
        None => Err(BackendError::NotAnAddress),
    }
}

/// The scheme in lower case and what follows it, or `None` when it is neither.
fn after_scheme(base: &str) -> Option<(&'static str, &str)> {
    for scheme in ["http://", "https://"] {
        if let Some((found, rest)) = base.split_at_checked(scheme.len())
            && found.eq_ignore_ascii_case(scheme)
        {
            return Some((scheme, rest));
        }
    }
    None
}

#[cfg(test)]
mod tests;
