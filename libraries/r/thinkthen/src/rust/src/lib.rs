//! The R surface's Rust half over the `thinkthen` public API (ticket 0108).
//!
//! R supplies authored values; the shared request session owns execution and cancellation.
//! Native conversion runs on the R main thread.
pub mod ffi;

mod calls;
mod native_results;
mod plan;
mod request;
mod results_generated;

use std::sync::{Mutex, MutexGuard, PoisonError};

use thinkthen::{Engine, EngineBuilder, Error, ErrorKind};

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

/// Settings retain caller equality and safe diagnostic spelling; Rust admits them.
#[derive(Clone, Debug, PartialEq)]
struct Settings(serde_json::Map<String, serde_json::Value>);
impl Settings {
    fn build(&self) -> Result<Engine, Error> {
        EngineBuilder::from_settings_json(&serde_json::Value::Object(self.0.clone()).to_string())?
            .build()
    }
    fn named(&self) -> String {
        let names = self
            .0
            .iter()
            .filter(|(name, _)| name.as_str() != "refresh_cache")
            .map(|(name, value)| match name.as_str() {
                "base_url" => name.clone(),
                "cache" if value == &serde_json::Value::Bool(false) => "cache = FALSE".into(),
                "cache" | "record" | "replay" | "profile" => format!("{name} = <folder>"),
                _ => format!("{name} = {value}"),
            })
            .collect::<Vec<_>>();
        if names.is_empty() {
            "the environment's settings".into()
        } else {
            names.join(", ")
        }
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
        *slot = Some(
            EngineBuilder::from_env()
                .and_then(EngineBuilder::build)
                .map_err(|error| carry(&error))?,
        );
    }
    slot.as_ref()
        .cloned()
        .ok_or_else(|| defect("default engine was not retained"))
}
