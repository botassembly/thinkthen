//! The engine builder, and the environment read it captures once.

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{self, Config};
use crate::core::{Backend, BackendProfile, DEFAULT_MODEL, KEY_IN_ADDRESS, KEY_VAR, ModelName};
use crate::engine::Width;
use crate::engine::error::Error as EngineError;
use crate::engine::facade::{Key, Settings, Storage};
use crate::engine::facade::{Roots, RootsError};
use crate::engine::usage::Counters;
use crate::public::BatchSetting;
use crate::public::error::Error;

const NO_DEFAULT_CACHE: &str =
    "no default cache folder is available; set THINKTHEN_CACHE or use no_cache";

/// Which answer cache an engine reads and writes.
#[derive(Clone, Debug)]
enum Cache {
    Default,
    At(PathBuf),
    Off,
}

/// One profile source; each later setter replaces the previous one.
#[derive(Debug)]
enum Profile {
    File(PathBuf),
    Inline(BackendProfile),
}

/// The default cache folder, resolved once when a builder is seeded.
#[derive(Clone, Debug)]
struct Seeded {
    folder: Option<PathBuf>,
    platform: bool,
    enabled: bool,
}

/// A key held for the engine. `Debug` never shows it.
#[derive(Clone)]
struct Secret(Arc<str>);

impl fmt::Debug for Secret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<withheld>")
    }
}

/// Engine settings, checked at each step and applied at [`EngineBuilder::build`].
///
/// A builder holds settings alone. It opens no file, counts nothing, and
/// registers no throttle until `build`.
pub struct EngineBuilder {
    base_url: Option<String>,
    key: Option<Secret>,
    model: Option<ModelName>,
    width: Option<Width>,
    max_requests: Option<usize>,
    max_requests_total: Option<u64>,
    max_request_bytes: usize,
    batch: Option<crate::core::Setting>,
    env_batch: Option<String>,
    cache: Cache,
    seeded: Option<Seeded>,
    timeout: Duration,
    max_retries: u32,
    profile: Option<Profile>,
    record: Option<PathBuf>,
    replay: Option<PathBuf>,
    ca_bundle: Option<PathBuf>,
}

impl fmt::Debug for EngineBuilder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EngineBuilder")
            .field("base_url", &self.base_url.as_ref().map(|_| "<withheld>"))
            .field("key", &self.key)
            .field("model", &self.model)
            .field("width", &self.width)
            .field("max_requests", &self.max_requests)
            .field("max_requests_total", &self.max_requests_total)
            .field("max_request_bytes", &self.max_request_bytes)
            .field("batch", &self.batch)
            .field("env_batch", &self.env_batch.is_some())
            .field("cache", &self.cache)
            .field("seeded", &self.seeded)
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .field("profile", &self.profile)
            .field("record", &self.record)
            .field("replay", &self.replay)
            .field("ca_bundle", &self.ca_bundle.as_ref().map(|_| "<withheld>"))
            .finish()
    }
}

impl EngineBuilder {
    /// Validate the closed C/host engine-settings object without reading the
    /// environment. The total-request cap is active before any send.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for duplicate, unknown, or invalid settings.
    pub fn validate_settings_json(text: &str) -> Result<(), Error> {
        crate::core::engine_settings(text).map_err(Error::usage)
    }

    pub(crate) fn new() -> Self {
        Self {
            base_url: None,
            key: None,
            model: None,
            width: None,
            max_requests: None,
            max_requests_total: None,
            max_request_bytes: Backend::DEFAULT_REQUEST_SIZE,
            batch: None,
            env_batch: None,
            cache: Cache::Default,
            seeded: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            profile: None,
            record: None,
            replay: None,
            ca_bundle: None,
        }
    }

    /// Capture what the command reads: `THINKTHEN_BASE_URL`,
    /// `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, `THINKTHEN_CA_BUNDLE`,
    /// `THINKTHEN_BATCH`, `THINKTHEN_MAX_REQUEST_BYTES`, the XDG cache home, and the
    /// XDG configuration file. The setters and `build` read no environment.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] naming a malformed variable or configuration
    /// field, and [`Error::Local`] when the configuration file exists but
    /// cannot be read.
    pub fn from_env() -> Result<Self, Error> {
        let config = Config::read(config::path().as_deref()).map_err(|refused| {
            if refused.unreadable {
                Error::local(refused.message)
            } else {
                Error::usage(refused.message)
            }
        })?;
        let named = variable("THINKTHEN_CACHE")?.map(PathBuf::from);
        let mut builder = Self::new();
        builder.seeded = Some(Seeded {
            platform: named.is_none(),
            folder: named.or_else(config::cache_path),
            enabled: config.cache_enabled(),
        });
        if let Some(base) =
            variable("THINKTHEN_BASE_URL")?.or_else(|| config.url().map(str::to_owned))
        {
            builder = builder
                .base_url(&base)
                .map_err(|error| Error::usage(format!("THINKTHEN_BASE_URL: {error}")))?;
        }
        if let Some(key) = variable(KEY_VAR)? {
            builder.key = Some(Secret(key.into()));
        }
        // A later explicit setter outranks this path, including an invalid one.
        // Validate only the path selected when the engine is built.
        builder.ca_bundle = variable("THINKTHEN_CA_BUNDLE")?.map(PathBuf::from);
        builder.env_batch = variable("THINKTHEN_BATCH")?;
        if let Some(model) = config.model() {
            builder = builder.model(model)?;
        }
        if let Some(size) = variable("THINKTHEN_MAX_REQUEST_BYTES")? {
            let value = size
                .parse::<usize>()
                .ok()
                .filter(|value| *value > 0 && size.bytes().all(|byte| byte.is_ascii_digit()))
                .ok_or_else(|| {
                    Error::usage("THINKTHEN_MAX_REQUEST_BYTES takes a whole number of at least 1")
                })?;
            builder = builder.max_request_bytes(value)?;
        }
        Ok(builder)
    }

    /// Post requests under this base address.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for an address the backend rule refuses.
    pub fn base_url(mut self, value: &str) -> Result<Self, Error> {
        Backend::resolve(Some(value), None, DEFAULT_MODEL).map_err(Error::refused)?;
        self.base_url = Some(value.to_owned());
        Ok(self)
    }

    /// Send this key with each live attempt, and only to the base address.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a blank key.
    pub fn api_key(mut self, value: &str) -> Result<Self, Error> {
        if value.trim().is_empty() {
            return Err(Error::usage("a key is text, not white space"));
        }
        self.key = Some(Secret(value.into()));
        Ok(self)
    }

    /// Replace Mozilla roots with certificates from this absolute PEM file at build.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a relative path. `build` reports unreadable
    /// or invalid contents before any transport key use or network request.
    pub fn ca_bundle(mut self, value: impl AsRef<Path>) -> Result<Self, Error> {
        let path = value.as_ref();
        if !path.is_absolute() {
            return Err(Error::usage(
                "THINKTHEN_CA_BUNDLE must name an absolute local file",
            ));
        }
        self.ca_bundle = Some(path.to_owned());
        Ok(self)
    }

    /// Name this model when a question names none.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a blank model.
    pub fn model(mut self, value: &str) -> Result<Self, Error> {
        self.model = Some(ModelName::new(value).map_err(Error::refused)?);
        Ok(self)
    }

    /// Hold at most this many live attempts in flight in this process, 1
    /// through 32. Without it the engine follows the process throttle.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for 0 or anything above 32.
    pub fn throttle(mut self, value: u8) -> Result<Self, Error> {
        let width = Width::new(u64::from(value))
            .map_err(|_| Error::usage("a throttle is a whole number from 1 through 32"))?;
        self.width = Some(width);
        Ok(self)
    }

    /// Refuse a call over more than this many records. `rank` and `find`
    /// hold their input first and refuse before any request. The
    /// streaming calls send the first records and end with [`Error::Usage`]
    /// when the record past the limit arrives. `None` is no limit.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for `Some(0)`.
    pub fn max_requests(mut self, value: Option<usize>) -> Result<Self, Error> {
        if value == Some(0) {
            return Err(Error::usage(
                "a request limit is a whole number of 1 or more",
            ));
        }
        self.max_requests = value;
        Ok(self)
    }

    /// Limit this engine's live attempts against the shared process count.
    /// `None` leaves this engine unbounded while its sends still count for
    /// other engines. Zero refuses every live attempt.
    #[must_use]
    pub fn max_requests_total(mut self, value: Option<u64>) -> Self {
        self.max_requests_total = value;
        self
    }

    /// Set the request-byte ceiling for split plans. A lone question still goes alone.
    /// A smaller backend-profile ceiling takes precedence when a plan is prepared.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for zero.
    pub fn max_request_bytes(mut self, value: usize) -> Result<Self, Error> {
        if value == 0 {
            return Err(Error::usage(
                "max_request_bytes is a whole number of at least 1",
            ));
        }
        self.max_request_bytes = value;
        Ok(self)
    }

    /// Select the default number of records in one eligible request.
    #[must_use]
    pub fn batch(mut self, setting: BatchSetting) -> Self {
        self.batch = Some(setting.into());
        self.env_batch = None;
        self
    }

    /// Cache answers in the default folder: `THINKTHEN_CACHE` or the XDG
    /// cache home, as captured by [`EngineBuilder::from_env`].
    #[must_use]
    pub fn default_cache(mut self) -> Self {
        self.cache = Cache::Default;
        self
    }

    /// Cache answers in this folder.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for an empty path.
    pub fn cache_at(mut self, value: impl AsRef<Path>) -> Result<Self, Error> {
        self.cache = Cache::At(folder(
            value.as_ref(),
            "a cache folder is a path, not empty",
        )?);
        Ok(self)
    }

    /// Read and write no answer cache.
    #[must_use]
    pub fn no_cache(mut self) -> Self {
        self.cache = Cache::Off;
        self
    }

    /// End each live attempt after this duration.
    /// # Errors
    /// Returns [`Error::Usage`] for zero.
    pub fn timeout(mut self, value: Duration) -> Result<Self, Error> {
        if value.is_zero() {
            return Err(Error::usage("a timeout is a time above zero"));
        }
        self.timeout = value;
        Ok(self)
    }

    /// Retry a failed live attempt at most this many times.
    #[must_use]
    pub fn max_retries(mut self, value: u32) -> Self {
        self.max_retries = value;
        self
    }

    /// Read this backend profile at build time.
    /// # Errors
    /// Returns [`Error::Usage`] for an empty path.
    pub fn profile(mut self, value: impl AsRef<Path>) -> Result<Self, Error> {
        self.profile = Some(Profile::File(folder(
            value.as_ref(),
            "a profile file is a path, not empty",
        )?));
        Ok(self)
    }

    /// Parse one closed version-one backend profile without reading a file.
    /// # Errors
    /// Returns [`Error::Usage`] for an invalid profile object.
    pub fn profile_json(mut self, value: &str) -> Result<Self, Error> {
        self.profile = Some(Profile::Inline(
            BackendProfile::parse(value)
                .map_err(|error| Error::usage(format!("the profile JSON {error}")))?,
        ));
        Ok(self)
    }

    /// Write every exchange to this folder.
    /// # Errors
    /// Returns [`Error::Usage`] for an empty path.
    pub fn record(mut self, value: impl AsRef<Path>) -> Result<Self, Error> {
        self.record = Some(folder(
            value.as_ref(),
            "a recording folder is a path, not empty",
        )?);
        Ok(self)
    }

    /// Read only saved exchanges from this folder.
    /// # Errors
    /// Returns [`Error::Usage`] for an empty path.
    pub fn replay(mut self, value: impl AsRef<Path>) -> Result<Self, Error> {
        self.replay = Some(folder(
            value.as_ref(),
            "a recording folder is a path, not empty",
        )?);
        Ok(self)
    }

    /// Build the engine. An explicit throttle registers here, and nothing is sent.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when the default cache is selected and no
    /// folder is available, or when a different throttle is already active.
    pub fn build(self) -> Result<super::Engine, Error> {
        let batch = match (self.batch, self.env_batch.as_deref()) {
            (Some(setting), _) => Some(setting),
            (None, Some(value)) => Some(crate::core::Setting::parse(value).ok_or_else(|| {
                Error::usage("THINKTHEN_BATCH takes max or a whole number of at least 1")
            })?),
            (None, None) => None,
        };
        let model = self.model.as_ref().map_or(DEFAULT_MODEL, ModelName::as_str);
        let backend = Backend::resolve(self.base_url.as_deref(), None, model)
            .map_err(Error::refused)?
            .with_request_size(self.max_request_bytes);
        if backend.address_contains_key(self.key.as_ref().map(|Secret(value)| value.as_ref())) {
            return Err(Error::usage(KEY_IN_ADDRESS));
        }
        let roots = self
            .ca_bundle
            .as_deref()
            .map(Roots::load)
            .transpose()
            .map_err(|error| match error {
                RootsError::Usage(message) => Error::usage(message),
                RootsError::Local(message) => Error::local(message),
            })?;
        let profile = self
            .profile
            .as_ref()
            .map(|source| match source {
                Profile::File(path) => {
                    let text = std::fs::read_to_string(path)
                        .map_err(|_| Error::local("the profile file could not be read"))?;
                    BackendProfile::parse(&text)
                        .map_err(|error| Error::local(format!("the profile file {error}")))
                }
                Profile::Inline(profile) => Ok(profile.clone()),
            })
            .transpose()?;
        let key = self.key.clone();
        let settings = Settings {
            backend,
            profile: profile.clone(),
            timeout: self.timeout,
            max_retries: self.max_retries,
            retry_wait: Duration::from_secs(1),
            width: self.width,
            storage: self.storage()?,
            key: Arc::new(move || {
                key.as_ref()
                    .map(|Secret(value)| Key::new(value.as_ref().to_owned()))
                    .ok_or(EngineError::NoKey(KEY_VAR))
            }),
            usage: Arc::new(Counters::new(None)),
        };
        super::Engine::from_settings(
            settings,
            self.max_requests,
            self.max_requests_total,
            profile,
            roots,
            batch,
        )
    }

    fn storage(&self) -> Result<Storage, Error> {
        if matches!((&self.record, &self.replay), (Some(record), Some(replay)) if record != replay)
        {
            return Err(Error::usage(
                "record and replay name two different folders, and one engine keeps one",
            ));
        }
        if self.record.is_some() || self.replay.is_some() {
            if matches!(self.cache, Cache::At(_)) {
                return Err(Error::usage(
                    "a cache folder is record and replay on one folder, so it stands beside neither",
                ));
            }
            return Ok(Storage {
                record: self.record.clone(),
                replay: self.replay.clone(),
                private_default: false,
                cache_answers: false,
                refresh_cache: false,
            });
        }
        let (folder, private_default) = match &self.cache {
            Cache::Off => return Ok(Storage::default()),
            Cache::At(folder) => (folder.clone(), false),
            Cache::Default => match &self.seeded {
                Some(seeded) if !seeded.enabled && seeded.platform => return Ok(Storage::default()),
                Some(seeded) => (
                    seeded
                        .folder
                        .clone()
                        .ok_or_else(|| Error::usage(NO_DEFAULT_CACHE))?,
                    seeded.platform,
                ),
                None => (
                    config::cache_path().ok_or_else(|| Error::usage(NO_DEFAULT_CACHE))?,
                    true,
                ),
            },
        };
        Ok(Storage {
            record: Some(folder.clone()),
            replay: Some(folder),
            private_default,
            cache_answers: true,
            refresh_cache: false,
        })
    }
}

fn folder(path: &Path, sentence: &str) -> Result<PathBuf, Error> {
    if path.as_os_str().is_empty() {
        Err(Error::usage(sentence))
    } else {
        Ok(path.to_owned())
    }
}

/// One variable, or `None` when it holds nothing but white space.
fn variable(name: &str) -> Result<Option<String>, Error> {
    match std::env::var(name) {
        Ok(value) if value.trim().is_empty() => Ok(None),
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => {
            Err(Error::usage(format!("{name} is not valid UTF-8")))
        }
    }
}
