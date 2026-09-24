//! The engine builder, and the environment read it captures once.

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{self, Config};
use crate::core::{Backend, DEFAULT_MODEL, KEY_VAR, ModelName};
use crate::engine::Width;
use crate::engine::error::Error as EngineError;
use crate::engine::facade::{Key, Settings, Storage};
use crate::engine::usage::Counters;
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
#[derive(Debug)]
pub struct EngineBuilder {
    base_url: Option<String>,
    key: Option<Secret>,
    model: Option<ModelName>,
    width: Option<Width>,
    max_requests: Option<usize>,
    cache: Cache,
    seeded: Option<Seeded>,
}

impl EngineBuilder {
    pub(crate) fn new() -> Self {
        Self {
            base_url: None,
            key: None,
            model: None,
            width: None,
            max_requests: None,
            cache: Cache::Default,
            seeded: None,
        }
    }

    /// Capture what the command reads: `THINKTHEN_BASE_URL`,
    /// `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, the XDG cache home, and the
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
        if let Some(model) = config.model() {
            builder = builder.model(model)?;
        }
        Ok(builder)
    }

    /// Post requests under this base address.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for an address the backend rule refuses.
    pub fn base_url(mut self, value: &str) -> Result<Self, Error> {
        Backend::resolve(Some(value), None, DEFAULT_MODEL)
            .map_err(Error::refused)?;
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
        let folder = value.as_ref();
        if folder.as_os_str().is_empty() {
            return Err(Error::usage("a cache folder is a path, not empty"));
        }
        self.cache = Cache::At(folder.to_owned());
        Ok(self)
    }

    /// Read and write no answer cache.
    #[must_use]
    pub fn no_cache(mut self) -> Self {
        self.cache = Cache::Off;
        self
    }

    /// Check a cache cap. In 0.1 the library keeps no cap and prunes nothing,
    /// so a valid value has no effect; `thinkthen cache prune` reads its own.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for 0.
    pub fn cache_bytes(self, value: u64) -> Result<Self, Error> {
        if value == 0 {
            return Err(Error::usage(
                "a cache cap is a whole number of bytes above zero",
            ));
        }
        Ok(self)
    }

    /// Build the engine. An explicit throttle registers here, and nothing is sent.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when the default cache is selected and no
    /// folder is available, or when a different throttle is already active.
    pub fn build(self) -> Result<super::Engine, Error> {
        let model = self.model.as_ref().map_or(DEFAULT_MODEL, ModelName::as_str);
        let backend = Backend::resolve(self.base_url.as_deref(), None, model)
            .map_err(Error::refused)?;
        let key = self.key.clone();
        let settings = Settings {
            backend,
            profile: None,
            timeout: Duration::from_secs(30),
            max_retries: 2,
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
        super::Engine::from_settings(settings, self.max_requests)
    }

    fn storage(&self) -> Result<Storage, Error> {
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
        })
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
