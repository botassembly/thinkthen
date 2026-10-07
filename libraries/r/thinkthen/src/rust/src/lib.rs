//! The R surface's Rust half over the `thinkthen` public API (ticket 0108).
//!
//! The R half composes each question as the question file's JSON and hands
//! it here with the caller's texts. This half runs one engine call on a
//! fresh worker thread, waits on R's main thread with R's interrupt check
//! every 100 ms, and builds R values from the plain data the worker returns.
//! It holds no rule, retry, or request of its own.
//!
//! Errors cross as one string: the kind word, the retry signal, and the
//! message, joined by the unit separator. Every `%` doubles, because
//! extendr raises the string through R's `Rf_error`, whose argument is a
//! printf format. A NUL and the separator are escaped, because an R string
//! cannot hold the first and the second would split the packing.

pub mod ffi;

mod calls;
mod complete;
mod files;
mod plan;
mod relate;

use std::sync::{Mutex, MutexGuard, PoisonError};

use thinkthen::{BatchSetting, Engine, EngineBuilder, Error, ErrorKind};

pub use ffi::get_thinkthen_metadata;

/// The separator that carries an error's kind and retry signal to R.
const SEP: char = '\u{1f}';

/// One failure in the shape the R half parses.
fn packed(kind: &str, retryable: bool, message: &str) -> String {
    let message = message
        .replace('%', "%%")
        .replace('\0', "\\u0000")
        .replace(SEP, "\\u001f");
    format!("{kind}{SEP}{retryable}{SEP}{message}")
}

/// The one error-kind table: the engine's own words (ADR 0047 item 7).
fn carry(error: &Error) -> String {
    let base = packed(error.kind().name(), error.retryable(), &error.to_string());
    complete::stream::failure(error).map_or(base.clone(), |snapshot| {
        format!("{base}{SEP}{}", snapshot.replace('%', "%%"))
    })
}

/// A caller's mistake the shim refuses before any request.
fn usage(message: &str) -> String {
    packed(ErrorKind::Usage.name(), false, message)
}

/// A fault in the shim itself.
fn defect(message: &str) -> String {
    packed(ErrorKind::Defect.name(), false, message)
}

/// The marker the R half raises as R's own interrupt condition.
fn interrupted() -> String {
    packed("interrupt", false, "the call was interrupted")
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
fn choose_engine(settings: Settings) -> Result<(), String> {
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
fn engine() -> Result<Engine, String> {
    if let Some((_, engine)) = chosen().as_ref() {
        return Ok(engine.clone());
    }
    thinkthen::default_engine()
        .cloned()
        .map_err(|error| carry(&error))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parts(carried: &str) -> Vec<&str> {
        carried.split(SEP).collect()
    }

    #[test]
    fn each_of_the_six_kinds_crosses_as_its_own_word() {
        let kinds = [
            (ErrorKind::Usage, "usage"),
            (ErrorKind::Backend, "backend"),
            (ErrorKind::Local, "local"),
            (ErrorKind::Cancelled, "cancelled"),
            (ErrorKind::Deadline, "deadline"),
            (ErrorKind::Defect, "defect"),
        ];
        for (kind, word) in kinds {
            assert_eq!(parts(&packed(kind.name(), true, "m")), [word, "true", "m"]);
        }
    }

    #[test]
    fn the_message_escapes_what_r_cannot_carry() {
        let rows = [
            ("100% sure %s", "100%% sure %%s"),
            ("a\u{0}b", "a\\u0000b"),
            ("a\u{1f}b", "a\\u001fb"),
        ];
        for (message, carried) in rows {
            assert_eq!(parts(&usage(message)), ["usage", "false", carried]);
        }
        assert_eq!(
            parts(&interrupted()),
            ["interrupt", "false", "the call was interrupted"]
        );
    }
}
