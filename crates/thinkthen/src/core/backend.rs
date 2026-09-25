//! The backend a request goes to: one address and one model.

use thiserror::Error;

use crate::core::adapters::built_in;
use crate::core::text::{BlankTextError, ModelName, Url};

/// The one environment variable that holds the key.
///
/// The key goes to the address the user named, because naming the address is
/// the user's own act. `specification/backends.md` states the rule.
pub(crate) const KEY_VAR: &str = "THINKTHEN_API_KEY";

/// Where one request goes and which model it names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Backend {
    url: Url,
    model: ModelName,
}

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
}

/// The three host spellings `http://` may carry.
///
/// Each one names this machine and nothing else, so the key never leaves it.
/// Every other spelling is refused, including one that would resolve to
/// loopback, because this rule reads text and resolves no name. `localhost.`,
/// `127.1`, `0.0.0.0`, and `[::ffff:127.0.0.1]` are therefore all refused.
const LOOPBACK: [&str; 3] = ["localhost", "127.0.0.1", "[::1]"];

impl Backend {
    #[cfg(test)]
    pub(crate) const fn from_parts(url: Url, model: ModelName) -> Self {
        Self { url, model }
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
        Ok(Self {
            url: address(url.or(base).unwrap_or(built_in::DEFAULT_BASE))?,
            model: ModelName::new(model)?,
        })
    }

    /// Read the URL the request is posted to.
    #[must_use]
    pub(crate) const fn url(&self) -> &Url {
        &self.url
    }

    /// Read the model the request names.
    #[must_use]
    pub(crate) const fn model(&self) -> &ModelName {
        &self.model
    }

    /// The request-byte ceiling a relation plan splits under, at the built-in address alone.
    ///
    /// The hosted backend refuses a request over 65,536 input tokens. Relate's
    /// JSON measured 0.516 input tokens a byte, so 96,000 bytes comes to about
    /// 49,500 tokens (ticket 0123). Every other address has no ceiling.
    #[must_use]
    pub(crate) fn relation_ceiling(&self) -> Option<usize> {
        let base = self
            .url
            .as_str()
            .strip_suffix(built_in::ENDPOINT_PATH)
            .and_then(|rest| rest.strip_suffix('/'));
        (base == Some(built_in::DEFAULT_BASE)).then_some(RELATION_CEILING)
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

/// The built-in address's relation request-byte ceiling. See `Backend::relation_ceiling`.
const RELATION_CEILING: usize = 96_000;

/// The address one request is posted to: the base, then the endpoint path.
///
/// Space around the base is not part of it, and neither are the slashes it
/// ends in, so both are dropped before the path is added. A scheme is read
/// without regard to case and written back in lower case, so one exchange
/// keeps one recording digest whatever case the caller typed.
fn address(base: &str) -> Result<Url, BackendError> {
    let base = base.trim();
    if base.is_empty() {
        return Err(BackendError::Blank(BlankTextError::Url));
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
    Ok(Url::new(format!(
        "{scheme}{rest}/{}",
        built_in::ENDPOINT_PATH
    ))?)
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
mod tests {
    use super::{Backend, BackendError};
    use crate::core::adapters::built_in::DEFAULT_MODEL;
    use crate::core::text::BlankTextError;

    /// The address the tool posts to when no base replaces the default one.
    const BUILT_IN: &str = "https://api.typesafe.ai/v1/systemone";

    /// Resolve from the two sources a case names, with the default model.
    fn resolve(url: Option<&str>, base: Option<&str>) -> Result<Backend, BackendError> {
        Backend::resolve(url, base, DEFAULT_MODEL)
    }

    #[test]
    fn nothing_named_resolves_the_default_base_and_the_default_model() {
        let backend = resolve(None, None).expect("the default base resolves");

        assert_eq!(backend.url().as_str(), BUILT_IN);
        assert_eq!(backend.model().as_str(), "jev-latest");
    }

    #[test]
    fn the_option_outranks_the_variable_and_the_variable_outranks_the_default() {
        let option = "http://127.0.0.1:8080/v1";
        let variable = "http://127.0.0.1:9090/v1";
        let cases = [
            (
                Some(option),
                Some(variable),
                "http://127.0.0.1:8080/v1/systemone",
            ),
            (Some(option), None, "http://127.0.0.1:8080/v1/systemone"),
            (None, Some(variable), "http://127.0.0.1:9090/v1/systemone"),
            (None, None, BUILT_IN),
        ];

        for (url, base, expected) in cases {
            let backend = resolve(url, base).expect("a base names an address");
            assert_eq!(backend.url().as_str(), expected, "{url:?} over {base:?}");
        }
    }

    #[test]
    fn a_model_replaces_the_default_and_a_blank_one_is_refused() {
        let backend =
            Backend::resolve(None, None, "jev-1.13.0").expect("a model names the version");
        assert_eq!(backend.model().as_str(), "jev-1.13.0");

        for blank in ["", " ", "\t\n"] {
            assert_eq!(
                Backend::resolve(None, None, blank),
                Err(BackendError::Blank(BlankTextError::ModelName)),
                "{blank:?}"
            );
        }
    }

    #[test]
    fn a_base_is_read_to_one_address_or_refused_without_showing_itself() {
        let taken = [
            ("http://localhost/v1", "http://localhost/v1/systemone"),
            ("http://localhost/v1/", "http://localhost/v1/systemone"),
            ("http://localhost/v1///", "http://localhost/v1/systemone"),
            ("  http://localhost/v1\n", "http://localhost/v1/systemone"),
            ("HTTPS://host/v1", "https://host/v1/systemone"),
            ("HtTpS://host/v1", "https://host/v1/systemone"),
            ("HTTP://127.0.0.1/v1", "http://127.0.0.1/v1/systemone"),
            ("http://localhost", "http://localhost/systemone"),
        ];
        for (base, expected) in taken {
            let backend = resolve(None, Some(base)).expect("a base names an address");
            assert_eq!(backend.url().as_str(), expected, "{base:?}");
        }

        let refused = [
            ("ftp://host/v1", BackendError::InvalidScheme),
            ("host/v1", BackendError::InvalidScheme),
            ("http:/host/v1", BackendError::InvalidScheme),
            (
                "https://someone:sk-in-the-address@host/v1",
                BackendError::UserInformation,
            ),
            ("http://someone@host/v1", BackendError::UserInformation),
            (
                "http://host/v1?key=sk-in-the-address",
                BackendError::QueryOrFragment,
            ),
            (
                "http://host/v1#sk-in-the-address",
                BackendError::QueryOrFragment,
            ),
            // A port is a number a socket can carry, and none of these is one.
            ("http://localhost:x/v1", BackendError::InvalidPort),
            ("https://host:8080a/v1", BackendError::InvalidPort),
            ("http://localhost:/v1", BackendError::InvalidPort),
            ("http://localhost:99999999999/v1", BackendError::InvalidPort),
            ("https://host:65536/v1", BackendError::InvalidPort),
            ("http://[::1]:+80/v1", BackendError::InvalidPort),
            ("https:///v1", BackendError::NotAnAddress),
        ];
        for (base, expected) in refused {
            let error =
                resolve(None, Some(base)).expect_err("a base that names no address is refused");
            assert_eq!(error, expected, "{base}");
            assert!(!error.to_string().contains(base), "{error}");
            if expected == BackendError::NotAnAddress {
                assert_eq!(error.to_string(), "a base address has a host");
            }
        }
    }

    #[test]
    fn only_literal_ascii_letters_in_an_unbracketed_host_become_lowercase() {
        let cases = [
            (
                "HTTPS://XN--BCHER-KVA.ExAmPlE:00443/MiXeD/%2F",
                "https://xn--bcher-kva.example:00443/MiXeD/%2F/systemone",
            ),
            (
                "https://[2001:DB8::A]:00443/MiXeD/%2F",
                "https://[2001:DB8::A]:00443/MiXeD/%2F/systemone",
            ),
            (
                "https://MiXeD%2EHoSt/MiXeD/%2F",
                "https://mixed%2Ehost/MiXeD/%2F/systemone",
            ),
            (
                "https://BÜCHER.ExAmPlE/MiXeD/%2F",
                "https://bÜcher.example/MiXeD/%2F/systemone",
            ),
        ];

        for (base, expected) in cases {
            let backend = resolve(None, Some(base)).expect("a base names an address");
            assert_eq!(backend.url().as_str(), expected, "{base:?}");
        }
    }

    #[test]
    fn plain_http_reaches_loopback_alone_and_every_other_host_is_refused() {
        let loopback = [
            "http://localhost/v1",
            "http://LocalHost:8080/v1",
            "http://127.0.0.1/v1",
            "http://127.0.0.1:8721/v1",
            "http://[::1]/v1",
            "http://[::1]:9/v1",
        ];
        for base in loopback {
            resolve(None, Some(base)).expect("a loopback base is taken");
        }

        let refused = [
            "http://host/v1",
            "http://example.com/v1",
            "http://localhost.example.com/v1",
            "http://localhost./v1",
            "http://127.0.0.1./v1",
            "http://127.1/v1",
            "http://0.0.0.0/v1",
            "http://10.0.0.5/v1",
            "http://192.168.1.4:8080/v1",
            "http://[::ffff:127.0.0.1]/v1",
            "http://[::ffff:7f00:1]/v1",
            "http://[0:0:0:0:0:0:0:1]/v1",
            "HTTP://host/v1",
            "http://2130706433/v1",
        ];
        for base in refused {
            let error = resolve(None, Some(base)).expect_err("a plain http base is refused");
            assert_eq!(error, BackendError::KeyInClear, "{base}");
            assert_eq!(
                error.to_string(),
                "`http://` sends the key across the network in clear text, \
                 so it reaches localhost, 127.0.0.1, and [::1] alone"
            );
        }

        for base in [
            "https://host/v1",
            "HTTPS://example.com/v1",
            "https://[::1]/v1",
        ] {
            resolve(None, Some(base)).expect("every https base is untouched");
        }
    }

    #[test]
    fn an_address_is_secure_under_https_and_never_under_plain_http() {
        for base in ["https://host/v1", "HTTPS://host/v1", "https://[::1]/v1"] {
            let backend = resolve(None, Some(base)).expect("a base names an address");
            assert!(backend.is_secure(), "{base}");
        }

        for base in ["http://localhost/v1", "HTTP://127.0.0.1/v1", "http://[::1]"] {
            let backend = resolve(None, Some(base)).expect("a base names an address");
            assert!(!backend.is_secure(), "{base}");
        }
    }

    #[test]
    fn a_base_that_holds_only_white_space_is_refused_as_a_blank_address() {
        for blank in ["", " ", "\t", "\n", "  \t\r\n "] {
            for (url, base) in [(Some(blank), None), (None, Some(blank))] {
                assert_eq!(
                    resolve(url, base),
                    Err(BackendError::Blank(BlankTextError::Url)),
                    "{blank:?}"
                );
            }
        }
    }
}
