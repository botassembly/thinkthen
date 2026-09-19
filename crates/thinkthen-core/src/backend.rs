//! The backend a request goes to.

use std::str::FromStr;

use thiserror::Error;

use crate::adapter::{Adapter, UnknownAdapterError};
use crate::text::{BlankTextError, KeyVar, ModelName, ProfileName, Url};

/// A named profile, as one row of data.
#[derive(Clone, Copy, Debug)]
struct Profile {
    name: &'static str,
    base: &'static str,
    adapter: Adapter,
    model: &'static str,
    key_env: &'static str,
}

/// The profile a user gets when nothing else is named.
const JEV: Profile = Profile {
    name: "jev",
    base: "https://api.typesafe.ai/v1",
    adapter: Adapter::SystemOne,
    model: "jev-latest",
    key_env: "THINKTHEN_API_KEY",
};

/// The profiles version one is born with.
const PROFILES: [Profile; 1] = [JEV];

/// What the flags offer a backend, each value absent when it was not given.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BackendValues<'a> {
    profile: Option<&'a str>,
    url: Option<&'a str>,
    adapter: Option<&'a str>,
    model: Option<&'a str>,
    key_env: Option<&'a str>,
    base: Option<&'a str>,
}

impl<'a> BackendValues<'a> {
    /// Gather the five flag values, in the order `specification/backends.md` lists them.
    #[must_use]
    pub const fn new(
        profile: Option<&'a str>,
        url: Option<&'a str>,
        adapter: Option<&'a str>,
        model: Option<&'a str>,
        key_env: Option<&'a str>,
    ) -> Self {
        Self {
            profile,
            url,
            adapter,
            model,
            key_env,
            base: None,
        }
    }

    /// Take the base address the environment offers, which `--url` outranks.
    ///
    /// The binary reads `THINKTHEN_BASE_URL` at its edge and hands the value
    /// here, so the core still reads no environment of its own.
    #[must_use]
    pub const fn with_base(self, base: Option<&'a str>) -> Self {
        Self { base, ..self }
    }
}

/// Where one request goes, in what language, to which model, under which key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Backend {
    profile: Option<ProfileName>,
    url: Url,
    adapter: Adapter,
    model: ModelName,
    key_env: Option<KeyVar>,
}

impl Backend {
    /// Read the profile name back, or `None` for an ad-hoc backend.
    #[must_use]
    pub const fn profile(&self) -> Option<&ProfileName> {
        self.profile.as_ref()
    }

    /// Read the URL the request is posted to.
    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// Read the wire format the backend speaks.
    #[must_use]
    pub const fn adapter(&self) -> Adapter {
        self.adapter
    }

    /// Read the model the request names.
    #[must_use]
    pub const fn model(&self) -> &ModelName {
        &self.model
    }

    /// Read the key variable, or `None` when the request carries no key.
    #[must_use]
    pub const fn key_env(&self) -> Option<&KeyVar> {
        self.key_env.as_ref()
    }
}

/// Why the given backend values name no backend.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum BackendError {
    /// One of the five values arrived blank.
    #[error(transparent)]
    Blank(#[from] BlankTextError),
    /// The adapter name belongs to no adapter.
    #[error(transparent)]
    UnknownAdapter(#[from] UnknownAdapterError),
    /// The profile name belongs to no profile.
    #[error("no backend profile is named `{0}`")]
    UnknownProfile(String),
    /// A URL arrived without both the adapter and the model that go with it.
    #[error("an ad-hoc backend needs a url, an adapter, and a model together")]
    IncompleteAdHoc,
    /// An adapter arrived with no URL to speak to.
    #[error("an adapter names the language a url speaks, so it needs a url")]
    AdapterWithoutUrl,
    /// A profile name arrived beside a URL, and an ad-hoc backend has no name.
    #[error("an ad-hoc backend has no name, so a url cannot join a named profile")]
    NameWithUrl,
    /// The base names no scheme the tool speaks, or it carries user information.
    ///
    /// The message shows no address, because the refused base is the one that
    /// may carry a secret.
    #[error("a base address begins with `http://` or `https://` and carries no user information")]
    NotAnAddress,
}

/// Resolve the backend from the flags, the environment, and the built-in profile.
///
/// Every source names a base, and the request is posted to `BASE/systemone`. A
/// URL, an adapter, and a model given together make an ad-hoc backend, which
/// has no name and takes no key variable from any profile. A model alone
/// replaces the profile's model, which is how a run is pinned to one version.
///
/// # Errors
///
/// Returns [`BackendError`] when a value is blank, when a base names no `http`
/// or `https` address, when a name belongs to no profile or adapter, or when
/// the values do not make one backend.
pub fn resolve_backend(flags: BackendValues<'_>) -> Result<Backend, BackendError> {
    let name = given(flags.profile, |text: &str| ProfileName::new(text))?;
    let adapter = given(flags.adapter, Adapter::from_str)?;
    let model = given(flags.model, |text: &str| ModelName::new(text))?;
    let key_env = given(flags.key_env, |text: &str| KeyVar::new(text))?;

    let Some(base) = flags.url else {
        if adapter.is_some() {
            return Err(BackendError::AdapterWithoutUrl);
        }
        return from_profile(name, flags.base, model, key_env);
    };
    if name.is_some() {
        return Err(BackendError::NameWithUrl);
    }
    let (Some(adapter), Some(model)) = (adapter, model) else {
        return Err(BackendError::IncompleteAdHoc);
    };
    Ok(Backend {
        profile: None,
        url: address(base, adapter)?,
        adapter,
        model,
        key_env,
    })
}

/// The address one request is posted to: the base, then the adapter's path.
///
/// Space around the base is not part of it, and neither are the slashes it
/// ends in, so both are dropped before the path is added. A scheme is matched
/// without regard to case, as every reader of an address matches one.
fn address(base: &str, adapter: Adapter) -> Result<Url, BackendError> {
    let base = base.trim();
    if base.is_empty() {
        return Err(BackendError::Blank(BlankTextError::Url));
    }
    let base = base.trim_end_matches('/');
    let rest = after_scheme(base).ok_or(BackendError::NotAnAddress)?;
    // The address is printed in a plan and kept in a recording, so a base that
    // carries user information would write a password into both.
    let authority = rest.split('/').next().unwrap_or(rest);
    if authority.contains('@') {
        return Err(BackendError::NotAnAddress);
    }
    Ok(Url::new(format!("{base}/{}", adapter.as_str()))?)
}

/// What follows a scheme the tool speaks, or `None` when it speaks none of them.
fn after_scheme(base: &str) -> Option<&str> {
    for scheme in ["http://", "https://"] {
        if let Some((found, rest)) = base.split_at_checked(scheme.len())
            && found.eq_ignore_ascii_case(scheme)
        {
            return Some(rest);
        }
    }
    None
}

/// Read one flag value into the type that holds it.
fn given<T, E>(flag: Option<&str>, read: impl Fn(&str) -> Result<T, E>) -> Result<Option<T>, E> {
    flag.map(read).transpose()
}

/// Fill a named profile in, letting the environment, a model, and a key
/// variable replace its own.
fn from_profile(
    name: Option<ProfileName>,
    base: Option<&str>,
    model: Option<ModelName>,
    key_env: Option<KeyVar>,
) -> Result<Backend, BackendError> {
    let wanted = name.as_ref().map_or(JEV.name, ProfileName::as_str);
    let profile = PROFILES
        .iter()
        .find(|profile| profile.name == wanted)
        .ok_or_else(|| BackendError::UnknownProfile(wanted.to_owned()))?;
    Ok(Backend {
        profile: Some(ProfileName::new(profile.name)?),
        url: address(base.unwrap_or(profile.base), profile.adapter)?,
        adapter: profile.adapter,
        model: match model {
            Some(model) => model,
            None => ModelName::new(profile.model)?,
        },
        key_env: match key_env {
            Some(key_env) => Some(key_env),
            None => Some(KeyVar::new(profile.key_env)?),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::{Backend, BackendError, BackendValues, resolve_backend as resolve};

    /// The address the built-in profile posts to when no base replaces its own.
    const BUILT_IN: &str = "https://api.typesafe.ai/v1/systemone";
    use crate::adapter::Adapter;
    use crate::text::{BlankTextError, KeyVar, ProfileName};

    /// The values a resolved backend is compared against, as plain text.
    fn flat(backend: &Backend) -> (Option<&str>, &str, Adapter, &str, Option<&str>) {
        (
            backend.profile().map(ProfileName::as_str),
            backend.url().as_str(),
            backend.adapter(),
            backend.model().as_str(),
            backend.key_env().map(KeyVar::as_str),
        )
    }

    /// Only a model, which is the one profile value a flag may replace alone.
    fn model_only(model: &str) -> BackendValues<'_> {
        BackendValues::new(None, None, None, Some(model), None)
    }

    #[test]
    fn nothing_named_resolves_the_built_in_profile() {
        let backend = resolve(BackendValues::default()).expect("the built-in profile resolves");

        assert_eq!(
            flat(&backend),
            (
                Some("jev"),
                BUILT_IN,
                Adapter::SystemOne,
                "jev-latest",
                Some("THINKTHEN_API_KEY"),
            )
        );
    }

    #[test]
    fn a_flag_replaces_the_profile_value_it_names() {
        let backend =
            resolve(model_only("jev-1.13.0")).expect("a model alone replaces the profile model");
        assert_eq!(backend.model().as_str(), "jev-1.13.0");

        let backend = resolve(BackendValues::new(
            Some("jev"),
            None,
            None,
            None,
            Some("OTHER_KEY"),
        ))
        .expect("a named profile resolves");
        assert_eq!(
            flat(&backend),
            (
                Some("jev"),
                BUILT_IN,
                Adapter::SystemOne,
                "jev-latest",
                Some("OTHER_KEY"),
            )
        );
    }

    #[test]
    fn an_ad_hoc_backend_has_no_name_and_no_borrowed_key_variable() {
        let values = BackendValues::new(
            None,
            Some("http://127.0.0.1:8080/v1"),
            Some("systemone"),
            Some("local-1"),
            None,
        );
        let backend = resolve(values).expect("an ad-hoc backend resolves");

        assert_eq!(
            flat(&backend),
            (
                None,
                "http://127.0.0.1:8080/v1/systemone",
                Adapter::SystemOne,
                "local-1",
                None,
            )
        );
    }

    #[test]
    fn an_ad_hoc_backend_takes_the_key_variable_the_user_names_for_it() {
        let values = BackendValues::new(
            None,
            Some("http://127.0.0.1:8080/v1"),
            Some("systemone"),
            Some("local-1"),
            Some("LOCAL_KEY"),
        );
        let backend = resolve(values).expect("an ad-hoc backend resolves");

        assert_eq!(backend.key_env().map(KeyVar::as_str), Some("LOCAL_KEY"));
        assert_eq!(backend.profile(), None);
    }

    #[test]
    fn values_that_make_no_backend_each_name_their_own_cause() {
        let url = "http://127.0.0.1:8080/v1";
        let cases: [(BackendValues<'_>, BackendError); 8] = [
            (
                BackendValues::new(None, Some(url), None, None, None),
                BackendError::IncompleteAdHoc,
            ),
            (
                BackendValues::new(None, Some(url), Some("systemone"), None, None),
                BackendError::IncompleteAdHoc,
            ),
            (
                BackendValues::new(None, Some(url), None, Some("local-1"), None),
                BackendError::IncompleteAdHoc,
            ),
            (
                BackendValues::new(None, None, Some("systemone"), None, None),
                BackendError::AdapterWithoutUrl,
            ),
            (
                BackendValues::new(Some("jev"), Some(url), Some("systemone"), Some("m"), None),
                BackendError::NameWithUrl,
            ),
            (
                BackendValues::new(Some("other"), None, None, None, None),
                BackendError::UnknownProfile("other".to_owned()),
            ),
            (
                BackendValues::new(None, Some(url), Some("chat-logprobs"), Some("m"), None),
                BackendError::UnknownAdapter(
                    "chat-logprobs"
                        .parse::<Adapter>()
                        .expect_err("no adapter is named chat-logprobs"),
                ),
            ),
            (
                BackendValues::new(None, Some(" "), Some("systemone"), Some("m"), None),
                BackendError::Blank(BlankTextError::Url),
            ),
        ];

        for (values, expected) in cases {
            assert_eq!(
                resolve(values),
                Err(expected.clone()),
                "{values:?} names {expected}"
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
            ("HTTPS://host/v1", "HTTPS://host/v1/systemone"),
            ("http://host", "http://host/systemone"),
        ];
        for (base, expected) in taken {
            let values = BackendValues::default().with_base(Some(base));
            let backend = resolve(values).expect("a base names an address");
            assert_eq!(backend.url().as_str(), expected, "{base:?}");
        }

        let refused = [
            "ftp://host/v1",
            "host/v1",
            "http:/host/v1",
            "https://someone:sk-in-the-address@host/v1",
            "http://someone@host/v1",
        ];
        for base in refused {
            let error = resolve(BackendValues::default().with_base(Some(base)))
                .expect_err("a base that names no address is refused");
            assert_eq!(error, BackendError::NotAnAddress, "{base}");
            assert!(!error.to_string().contains("host"), "{error}");
        }
    }

    #[test]
    fn a_blank_flag_value_is_refused() {
        let cases: [(BackendValues<'_>, BlankTextError); 4] = [
            (
                BackendValues::new(Some(""), None, None, None, None),
                BlankTextError::ProfileName,
            ),
            (
                BackendValues::new(None, None, None, Some("\t"), None),
                BlankTextError::ModelName,
            ),
            (
                BackendValues::new(None, None, None, None, Some(" ")),
                BlankTextError::KeyVar,
            ),
            (
                BackendValues::new(None, Some("\n"), Some("systemone"), Some("m"), None),
                BlankTextError::Url,
            ),
        ];

        for (values, expected) in cases {
            assert_eq!(
                resolve(values),
                Err(BackendError::Blank(expected)),
                "a flag offering {values:?}"
            );
        }
    }
}
