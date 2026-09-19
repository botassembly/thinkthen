//! The backend a request goes to: one address and one model.

use thiserror::Error;

use crate::systemone;
use crate::text::{BlankTextError, ModelName, Url};

/// The base the tool posts under when nothing else names one.
const DEFAULT_BASE: &str = "https://api.typesafe.ai/v1";

/// The model a request names when `--model` names none.
pub const DEFAULT_MODEL: &str = "jev-latest";

/// The one environment variable that holds the key.
///
/// The key goes to the address the user named, because naming the address is
/// the user's own act. `specification/backends.md` states the rule.
pub const KEY_VAR: &str = "THINKTHEN_API_KEY";

/// Where one request goes and which model it names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Backend {
    url: Url,
    model: ModelName,
}

/// Why the given address and model name no backend.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum BackendError {
    /// The base or the model arrived blank.
    #[error(transparent)]
    Blank(#[from] BlankTextError),
    /// The base names no scheme the tool speaks, or it carries user information.
    ///
    /// The message shows no address, because the refused base is the one that
    /// may carry a secret.
    #[error("a base address begins with `http://` or `https://` and carries no user information")]
    NotAnAddress,
}

impl Backend {
    /// Resolve the backend from the option, the environment, and the default.
    ///
    /// Every source names a base, and the request is posted to `BASE/systemone`.
    /// `--url` outranks `THINKTHEN_BASE_URL`, which outranks the default base
    /// `https://api.typesafe.ai/v1`.
    /// The binary reads the variable at its edge and hands the value here, so
    /// the core still reads no environment of its own.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when the base or the model is blank, or when
    /// the base names no `http` or `https` address.
    pub fn resolve(
        url: Option<&str>,
        base: Option<&str>,
        model: &str,
    ) -> Result<Self, BackendError> {
        Ok(Self {
            url: address(url.or(base).unwrap_or(DEFAULT_BASE))?,
            model: ModelName::new(model)?,
        })
    }

    /// Read the URL the request is posted to.
    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// Read the model the request names.
    #[must_use]
    pub const fn model(&self) -> &ModelName {
        &self.model
    }
}

/// The address one request is posted to: the base, then the wire shape's name.
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
    let (scheme, rest) = after_scheme(base).ok_or(BackendError::NotAnAddress)?;
    // The address is printed in a plan and kept in a recording, so a base that
    // carries user information or a query would write a secret into both. A
    // path added after either one would also land in the wrong place.
    let authority = rest.split('/').next().unwrap_or(rest);
    if authority.contains('@') || rest.contains(['?', '#']) {
        return Err(BackendError::NotAnAddress);
    }
    Ok(Url::new(format!("{scheme}{rest}/{}", systemone::NAME))?)
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
    use super::{Backend, BackendError, DEFAULT_MODEL};
    use crate::text::BlankTextError;

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
            ("http://host/v1", "http://host/v1/systemone"),
            ("http://host/v1/", "http://host/v1/systemone"),
            ("http://host/v1///", "http://host/v1/systemone"),
            ("  http://host/v1\n", "http://host/v1/systemone"),
            ("HTTPS://host/v1", "https://host/v1/systemone"),
            ("HtTpS://host/v1", "https://host/v1/systemone"),
            ("http://host", "http://host/systemone"),
        ];
        for (base, expected) in taken {
            let backend = resolve(None, Some(base)).expect("a base names an address");
            assert_eq!(backend.url().as_str(), expected, "{base:?}");
        }

        let refused = [
            "ftp://host/v1",
            "host/v1",
            "http:/host/v1",
            "https://someone:sk-in-the-address@host/v1",
            "http://someone@host/v1",
            "http://host/v1?key=sk-in-the-address",
            "http://host/v1#sk-in-the-address",
        ];
        for base in refused {
            let error =
                resolve(None, Some(base)).expect_err("a base that names no address is refused");
            assert_eq!(error, BackendError::NotAnAddress, "{base}");
            assert!(!error.to_string().contains("host"), "{error}");
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
