//! The R surface's Rust half over the `thinkthen` public API (ticket 0108).
//!
//! The R half composes each question as the question file's JSON and hands
//! it here with the caller's texts. This half runs one engine call on a
//! fresh worker thread, waits on R's main thread with R's interrupt check
//! every 100 ms, and builds R values from the plain data the worker returns.
//! It holds no rule, retry, or request of its own.
//!
pub mod ffi;

mod calls;
mod native_results;
mod plan;
mod request;
mod results_generated;

use std::sync::{Mutex, MutexGuard, PoisonError};

use thinkthen::{BatchSetting, Engine, EngineBuilder, Error, ErrorKind};

pub use ffi::get_thinkthen_metadata;

/// An owned error crosses threads without retaining an R value.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeError {
    kind: String,
    retryable: bool,
    message: String,
    complete: Option<String>,
}
impl From<NativeError> for extendr_api::Robj {
    fn from(error: NativeError) -> Self {
        use extendr_api::prelude::*;
        let complete = error
            .complete
            .as_deref()
            .and_then(|text| crate::native_results::failure(text).ok());
        let facts = complete
            .as_ref()
            .and_then(|value| List::try_from(value.clone()).ok())
            .and_then(|value| {
                value
                    .iter()
                    .find(|(name, _)| *name == "facts")
                    .map(|(_, value)| value)
            });
        let mut value = list!(
            message = error.message.replace('\0', "\\u0000"),
            kind = error.kind.clone(),
            retryable = error.retryable,
            complete = complete.unwrap_or_else(|| ().into()),
            facts = facts.unwrap_or_else(|| ().into()),
            call = NULL
        );
        let _ = value.set_class([
            format!("thinkthen_{}", error.kind),
            "thinkthen_error".into(),
            "error".into(),
            "condition".into(),
        ]);
        value.into()
    }
}
fn carry(error: &Error) -> NativeError {
    NativeError {
        kind: error.kind().name().into(),
        retryable: error.retryable(),
        message: error.to_string(),
        complete: serde_json::to_string(&error.complete()).ok(),
    }
}
fn usage(message: &str) -> NativeError {
    carry(&Error::new(ErrorKind::Usage, message))
}
fn defect(message: &str) -> NativeError {
    carry(&Error::new(ErrorKind::Defect, message))
}
fn interrupted() -> NativeError {
    NativeError {
        kind: "interrupt".into(),
        retryable: false,
        message: "the call was interrupted".into(),
        complete: None,
    }
}

/// The engine settings `tt_engine` takes, as the caller gave them.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Settings {
    pub(crate) backend: Option<String>,
    pub(crate) base_url: Option<String>,
    pub(crate) model: Option<String>,
    pub(crate) throttle: Option<u8>,
    pub(crate) max_requests: Option<usize>,
    pub(crate) max_requests_total: Option<u64>,
    pub(crate) max_request_bytes: Option<usize>,
    /// A folder, or `None` inside for `cache = FALSE`.
    pub(crate) cache: Option<Option<String>>,
    pub(crate) timeout: Option<u64>,
    pub(crate) max_retries: Option<u32>,
    pub(crate) record: Option<String>,
    pub(crate) replay: Option<String>,
    pub(crate) profile: Option<String>,
    pub(crate) batch: Option<BatchSetting>,
    pub(crate) refresh_cache: bool,
}

impl Settings {
    /// The settings in force, named without their values' secrets.
    fn named(&self) -> String {
        let cache = self.cache.as_ref().map(|folder| match folder {
            Some(_) => "cache = <folder>".to_owned(),
            None => "cache = FALSE".to_owned(),
        });
        let named: Vec<String> = [
            self.backend.as_ref().map(|it| format!("backend = {it:?}")),
            self.base_url.as_ref().map(|_| "base_url".to_owned()),
            self.model.as_ref().map(|it| format!("model = {it:?}")),
            self.throttle.map(|it| format!("throttle = {it}")),
            self.max_requests.map(|it| format!("max_requests = {it}")),
            self.max_requests_total
                .map(|it| format!("max_requests_total = {it}")),
            self.max_request_bytes
                .map(|it| format!("max_request_bytes = {it}")),
            cache,
            self.timeout.map(|it| format!("timeout = {it}")),
            self.max_retries.map(|it| format!("max_retries = {it}")),
            self.record.as_ref().map(|_| "record = <folder>".to_owned()),
            self.replay.as_ref().map(|_| "replay = <folder>".to_owned()),
            self.profile
                .as_ref()
                .map(|_| "profile = <folder>".to_owned()),
            self.batch.map(|it| format!("batch = {it:?}")),
        ]
        .into_iter()
        .flatten()
        .collect();
        if named.is_empty() {
            "the environment's settings".to_owned()
        } else {
            named.join(", ")
        }
    }

    /// The engine these settings build over the environment's own.
    fn build(&self) -> Result<Engine, Error> {
        let mut builder = EngineBuilder::from_env()?.refresh_cache(self.refresh_cache);
        if let Some(backend) = &self.backend {
            builder = builder.backend(backend)?;
        }
        if let Some(base) = &self.base_url {
            builder = builder.base_url(base)?;
        }
        if let Some(model) = &self.model {
            builder = builder.model(model)?;
        }
        if let Some(throttle) = self.throttle {
            builder = builder.throttle(throttle)?;
        }
        if self.max_requests.is_some() {
            builder = builder.max_requests(self.max_requests)?;
        }
        if self.max_requests_total.is_some() {
            builder = builder.max_requests_total(self.max_requests_total);
        }
        if let Some(bytes) = self.max_request_bytes {
            builder = builder.max_request_bytes(bytes)?;
        }
        builder = match &self.cache {
            Some(Some(folder)) => builder.cache_at(folder)?,
            Some(None) => builder.no_cache(),
            None => builder,
        };
        if let Some(seconds) = self.timeout {
            builder = builder.timeout(std::time::Duration::from_secs(seconds))?;
        }
        if let Some(retries) = self.max_retries {
            builder = builder.max_retries(retries);
        }
        if let Some(folder) = &self.record {
            builder = builder.record(folder)?;
        }
        if let Some(folder) = &self.replay {
            builder = builder.replay(folder)?;
        }
        if let Some(path) = &self.profile {
            builder = builder.profile(path)?;
        }
        if let Some(batch) = self.batch {
            builder = builder.batch(batch);
        }
        builder.build()
    }
}

/// The engine `tt_engine` built, with the settings that built it.
static CHOSEN: Mutex<Option<(Settings, Engine)>> = Mutex::new(None);

/// The slot, recovered from a panic that poisoned it: the value is still whole.
fn chosen() -> MutexGuard<'static, Option<(Settings, Engine)>> {
    CHOSEN.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Build and keep the engine every later call uses. Equal settings a second
/// time do nothing, and any other settings are refused, because the
/// throttle is process-wide (ticket 0077).
fn choose_engine(settings: Settings) -> Result<(), crate::NativeError> {
    let mut slot = chosen();
    if let Some((held, _)) = slot.as_ref() {
        return if *held == settings {
            Ok(())
        } else {
            Err(usage(&format!(
                "the engine is already set with {}; start a new R session to change it",
                held.named()
            )))
        };
    }
    let engine = settings
        .build()
        .map_err(|error| usage(error.detail().message()))?;
    *slot = Some((settings, engine));
    Ok(())
}

/// The engine in use: the chosen one, or the default engine.
fn engine() -> Result<Engine, NativeError> {
    if let Some((_, engine)) = chosen().as_ref() {
        return Ok(engine.clone());
    }
    static DEFAULT: Mutex<Option<Engine>> = Mutex::new(None);
    let mut slot = DEFAULT.lock().unwrap_or_else(PoisonError::into_inner);
    if slot.is_none() {
        *slot = Some(Settings::default().build().map_err(|error| carry(&error))?);
    }
    slot.as_ref()
        .cloned()
        .ok_or_else(|| defect("default engine was not retained"))
}
