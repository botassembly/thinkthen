//! Python engine settings, checked before a worker starts.

use std::num::NonZeroUsize;

use pyo3::prelude::*;
use pyo3::types::{PyBool, PyInt, PyString};
use thinkthen::{BatchSetting, EngineBuilder};

use super::{Arg, MAX_REQUESTS_TOTAL};
use crate::diagnostics::{host, host_error};
use crate::input::whole;
use crate::{raised, usage};

const THROTTLE: &str = "a throttle is a whole number from 1 through 32";
const CACHE: &str = "cache is a folder path, False for no cache, or True for the default folder";
const BATCH: &str = "batch is 'max' or a whole number of 1 or more";
const CONTEXT: &str = "context is nonblank text";

/// How the engine caches: `True` for the default folder, `False` for none,
/// or a folder path.
fn cached(builder: EngineBuilder, cache: &Bound<'_, PyAny>) -> PyResult<EngineBuilder> {
    let py = cache.py();
    if let Ok(on) = cache.cast::<PyBool>() {
        return Ok(if on.is_true() {
            builder.default_cache()
        } else {
            builder.no_cache()
        });
    }
    let folder: std::path::PathBuf = host_error(host(|| cache.extract()), || usage(py, CACHE))?;
    builder.cache_at(folder).map_err(|error| raised(py, &error))
}

/// A whole-number setting in the range its type holds, or its sentence.
pub(super) fn setting<T: TryFrom<i64>>(
    value: Option<&Bound<'_, PyAny>>,
    sentence: &str,
) -> PyResult<Option<T>> {
    value
        .map(|value| T::try_from(whole(value, sentence)?).map_err(|_| usage(value.py(), sentence)))
        .transpose()
}

/// The throttle, checked here so `throttle=300` is a `UsageError`, not an
/// `OverflowError` (amendment change 13).
pub(super) fn checked_throttle(value: Arg<'_, '_>) -> PyResult<Option<u8>> {
    match (value, setting::<u8>(value, THROTTLE)?) {
        (Some(value), Some(read)) if !(1..=32).contains(&read) => Err(usage(value.py(), THROTTLE)),
        (_, read) => Ok(read),
    }
}

/// The shared process cap uses the full unsigned range, including zero.
pub(super) fn total(value: Arg<'_, '_>) -> PyResult<Option<u64>> {
    value
        .map(|value| {
            if value.is_instance_of::<PyBool>() || !value.is_instance_of::<PyInt>() {
                return Err(usage(value.py(), MAX_REQUESTS_TOTAL));
            }
            host_error(host(|| value.extract()), || {
                usage(value.py(), MAX_REQUESTS_TOTAL)
            })
        })
        .transpose()
}

pub(crate) fn batch(value: Arg<'_, '_>) -> PyResult<Option<BatchSetting>> {
    let Some(value) = value else { return Ok(None) };
    if value.cast::<PyBool>().is_ok() {
        return Err(usage(value.py(), BATCH));
    }
    if let Ok(text) = value.cast::<PyString>() {
        return if text.to_str()? == "max" {
            Ok(Some(BatchSetting::Max))
        } else {
            Err(usage(value.py(), BATCH))
        };
    }
    let count = setting::<usize>(Some(value), BATCH)?
        .and_then(NonZeroUsize::new)
        .ok_or_else(|| usage(value.py(), BATCH))?;
    Ok(Some(BatchSetting::Records(count)))
}

pub(crate) fn context(value: Arg<'_, '_>) -> PyResult<Option<String>> {
    value
        .map(|value| {
            let text = value
                .cast::<PyString>()
                .map_err(|_| usage(value.py(), CONTEXT))?;
            let text = text.to_str().map_err(|_| usage(value.py(), CONTEXT))?;
            if text.trim().is_empty() {
                return Err(usage(value.py(), CONTEXT));
            }
            Ok(text.to_owned())
        })
        .transpose()
}

/// Read one optional folder setting with its own public refusal sentence.
pub(super) fn folder_path(
    py: Python<'_>,
    value: Arg<'_, '_>,
    refusal: &'static str,
) -> PyResult<Option<std::path::PathBuf>> {
    value
        .map(|value| host_error(host(|| value.extract()), || usage(py, refusal)))
        .transpose()
}

/// The checked settings of `tt.Engine`, each applied over the environment.
pub(super) struct Settings<'a> {
    pub(super) base_url: Option<&'a str>,
    pub(super) model: Option<&'a str>,
    pub(super) throttle: Option<u8>,
    pub(super) batch: Option<BatchSetting>,
    pub(super) most: Option<usize>,
    pub(super) most_total: Option<u64>,
    pub(super) max_request_bytes: Option<usize>,
    pub(super) timeout: Option<u64>,
    pub(super) retries: Option<u32>,
    pub(super) record: Option<std::path::PathBuf>,
    pub(super) replay: Option<std::path::PathBuf>,
    pub(super) profile: Option<std::path::PathBuf>,
}

impl Settings<'_> {
    pub(super) fn build(self, py: Python<'_>, cache: Arg<'_, '_>) -> PyResult<thinkthen::Engine> {
        let refused = |error: thinkthen::Error| raised(py, &error);
        let mut builder = EngineBuilder::from_env().map_err(refused)?;
        if let Some(address) = self.base_url {
            builder = builder.base_url(address).map_err(refused)?;
        }
        if let Some(name) = self.model {
            builder = builder.model(name).map_err(refused)?;
        }
        if self.most.is_some() {
            builder = builder.max_requests(self.most).map_err(refused)?;
        }
        if self.most_total.is_some() {
            builder = builder.max_requests_total(self.most_total);
        }
        if let Some(size) = self.max_request_bytes {
            builder = builder.max_request_bytes(size).map_err(refused)?;
        }
        if let Some(seconds) = self.timeout {
            builder = builder
                .timeout(std::time::Duration::from_secs(seconds))
                .map_err(refused)?;
        }
        if let Some(retries) = self.retries {
            builder = builder.max_retries(retries);
        }
        if let Some(folder) = self.record {
            builder = builder.record(folder).map_err(refused)?;
        }
        if let Some(folder) = self.replay {
            builder = builder.replay(folder).map_err(refused)?;
        }
        if let Some(path) = self.profile {
            builder = builder.profile(path).map_err(refused)?;
        }
        if let Some(cache) = cache {
            builder = cached(builder, cache)?;
        }
        if let Some(throttle) = self.throttle {
            builder = builder.throttle(throttle).map_err(refused)?;
        }
        if let Some(batch) = self.batch {
            builder = builder.batch(batch);
        }
        builder.build().map_err(refused)
    }
}
