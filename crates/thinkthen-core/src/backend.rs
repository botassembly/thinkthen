//! The backend a request goes to, resolved from flags, the environment, and the built-in profile.

use std::str::FromStr;

use thiserror::Error;

use crate::adapter::{Adapter, UnknownAdapterError};
use crate::text::{BackendName, BlankTextError, KeyVar, ModelName, Url};

/// A named profile, as one row of data.
#[derive(Clone, Copy, Debug)]
struct Profile {
    name: &'static str,
    url: &'static str,
    adapter: Adapter,
    model: &'static str,
    key_env: &'static str,
}

/// The profile a user gets when nothing else is named.
const JEV: Profile = Profile {
    name: "jev",
    url: "https://api.typesafe.ai/v1/systemone",
    adapter: Adapter::SystemOne,
    model: "jev-latest",
    key_env: "TYPESAFE_API_KEY",
};

/// The profiles version one is born with.
const PROFILES: [Profile; 1] = [JEV];

/// The five backend values one source offers, each absent when it was not given.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BackendValues<'a> {
    backend: Option<&'a str>,
    url: Option<&'a str>,
    adapter: Option<&'a str>,
    model: Option<&'a str>,
    key_env: Option<&'a str>,
}

impl<'a> BackendValues<'a> {
    /// Gather what one source offers, in the order `specification/backends.md` lists it.
    #[must_use]
    pub const fn new(
        backend: Option<&'a str>,
        url: Option<&'a str>,
        adapter: Option<&'a str>,
        model: Option<&'a str>,
        key_env: Option<&'a str>,
    ) -> Self {
        Self {
            backend,
            url,
            adapter,
            model,
            key_env,
        }
    }
}

/// Where one request goes, in what language, to which model, under which key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Backend {
    name: Option<BackendName>,
    url: Url,
    adapter: Adapter,
    model: ModelName,
    key_env: Option<KeyVar>,
}

impl Backend {
    /// Read the profile name back, or `None` for an ad-hoc backend.
    #[must_use]
    pub const fn name(&self) -> Option<&BackendName> {
        self.name.as_ref()
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
    /// The backend name belongs to no profile.
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
}

/// Resolve the backend from the flags, the environment, and the built-in profile.
///
/// A flag beats an environment variable, and an environment variable beats the
/// profile. A URL, an adapter, and a model given together make an ad-hoc
/// backend, which has no name and takes no key variable from any profile.
///
/// # Errors
///
/// Returns [`BackendError`] when a value is blank, when a name belongs to no
/// profile or adapter, or when the values do not make one backend.
pub fn resolve_backend(
    flags: BackendValues<'_>,
    environment: BackendValues<'_>,
) -> Result<Backend, BackendError> {
    let name = given(flags.backend, environment.backend, |text: &str| {
        BackendName::new(text)
    })?;
    let url = given(flags.url, environment.url, |text: &str| Url::new(text))?;
    let adapter = given(flags.adapter, environment.adapter, Adapter::from_str)?;
    let model = given(flags.model, environment.model, |text: &str| {
        ModelName::new(text)
    })?;
    let key_env = given(flags.key_env, environment.key_env, |text: &str| {
        KeyVar::new(text)
    })?;

    let Some(url) = url else {
        if adapter.is_some() {
            return Err(BackendError::AdapterWithoutUrl);
        }
        return from_profile(name, model, key_env);
    };
    if name.is_some() {
        return Err(BackendError::NameWithUrl);
    }
    let (Some(adapter), Some(model)) = (adapter, model) else {
        return Err(BackendError::IncompleteAdHoc);
    };
    Ok(Backend {
        name: None,
        url,
        adapter,
        model,
        key_env,
    })
}

/// Take the flag when it is there, the environment variable otherwise, and read it.
fn given<T, E>(
    flag: Option<&str>,
    environment: Option<&str>,
    read: impl Fn(&str) -> Result<T, E>,
) -> Result<Option<T>, BackendError>
where
    BackendError: From<E>,
{
    flag.or(environment)
        .map(read)
        .transpose()
        .map_err(Into::into)
}

/// Fill a named profile in, letting a model and a key variable replace its own.
fn from_profile(
    name: Option<BackendName>,
    model: Option<ModelName>,
    key_env: Option<KeyVar>,
) -> Result<Backend, BackendError> {
    let wanted = name.as_ref().map_or(JEV.name, BackendName::as_str);
    let profile = PROFILES
        .iter()
        .find(|profile| profile.name == wanted)
        .ok_or_else(|| BackendError::UnknownProfile(wanted.to_owned()))?;
    Ok(Backend {
        name: Some(BackendName::new(profile.name)?),
        url: Url::new(profile.url)?,
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
    use super::{Backend, BackendError, BackendValues, JEV, resolve_backend as resolve};
    use crate::adapter::Adapter;
    use crate::text::{BackendName, BlankTextError, KeyVar};

    /// The values a resolved backend is compared against, as plain text.
    fn flat(backend: &Backend) -> (Option<&str>, &str, Adapter, &str, Option<&str>) {
        (
            backend.name().map(BackendName::as_str),
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
        let backend = resolve(BackendValues::default(), BackendValues::default())
            .expect("the built-in profile resolves");

        assert_eq!(
            flat(&backend),
            (
                Some("jev"),
                JEV.url,
                Adapter::SystemOne,
                "jev-latest",
                Some("TYPESAFE_API_KEY"),
            )
        );
    }

    #[test]
    fn a_flag_beats_an_environment_variable_and_both_beat_the_profile() {
        let backend = resolve(model_only("jev-1.13.0"), model_only("jev-1.12.0"))
            .expect("a model alone replaces the profile model");
        assert_eq!(backend.model().as_str(), "jev-1.13.0");

        let backend = resolve(BackendValues::default(), model_only("jev-1.12.0"))
            .expect("a model alone replaces the profile model");
        assert_eq!(backend.model().as_str(), "jev-1.12.0");

        let backend = resolve(
            BackendValues::new(None, None, None, None, Some("OTHER_KEY")),
            BackendValues::new(Some("jev"), None, None, None, Some("ENV_KEY")),
        )
        .expect("a named profile resolves");
        assert_eq!(
            flat(&backend),
            (
                Some("jev"),
                JEV.url,
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
        let backend =
            resolve(values, BackendValues::default()).expect("an ad-hoc backend resolves");

        assert_eq!(
            flat(&backend),
            (
                None,
                "http://127.0.0.1:8080/v1",
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
        let backend =
            resolve(values, BackendValues::default()).expect("an ad-hoc backend resolves");

        assert_eq!(backend.key_env().map(KeyVar::as_str), Some("LOCAL_KEY"));
        assert_eq!(backend.name(), None);
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
                resolve(values, BackendValues::default()),
                Err(expected.clone()),
                "{values:?} names {expected}"
            );
        }
    }

    #[test]
    fn a_blank_value_is_refused_whichever_source_offered_it() {
        let cases: [(BackendValues<'_>, BlankTextError); 4] = [
            (
                BackendValues::new(Some(""), None, None, None, None),
                BlankTextError::BackendName,
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
                BackendValues::new(None, Some("\n"), None, None, None),
                BlankTextError::Url,
            ),
        ];

        for (values, expected) in cases {
            assert_eq!(
                resolve(values, BackendValues::default()),
                Err(BackendError::Blank(expected)),
                "a flag offering {values:?}"
            );
            assert_eq!(
                resolve(BackendValues::default(), values),
                Err(BackendError::Blank(expected)),
                "an environment variable offering {values:?}"
            );
        }
    }
}
