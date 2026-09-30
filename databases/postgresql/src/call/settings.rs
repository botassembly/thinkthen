//! PostgreSQL GUC registration and the one validated engine-setting snapshot.

use std::ffi::CString;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::time::Duration;

use pgrx::{GucContext, GucFlags, GucRegistry, GucSetting};
use thinkthen::{BatchSetting, EngineBuilder, Error};

use super::{Call, OrRaise};
use crate::ffi;

/// The registered value that leaves a numeric engine setting unset.
pub(crate) const UNSET: i32 = -1;

/// The throttle's refusal where it is set, in the engine's own sentence, or
/// `None` for -1 (unset) and 1 through 32 (Ian's range).
pub(crate) fn throttle_refusal(value: i32) -> Option<String> {
    (value != UNSET && !(1..=32).contains(&value)).then(|| {
        super::text(&super::usage(
            "a throttle is a whole number from 1 through 32",
        ))
    })
}

/// The setter calls the four engine settings ask for. An unset setting
/// calls nothing, so the value `EngineBuilder::from_env` seeded stands.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Plan {
    pub(super) throttle: Option<u8>,
    pub(super) max_requests: Option<usize>,
    max_request_bytes: Option<usize>,
    batch: Option<BatchSetting>,
    cache: Option<Option<PathBuf>>,
    model: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    profile: Option<String>,
    record: Option<PathBuf>,
    replay: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug)]
struct Raw<'a> {
    throttle: i32,
    max_requests: i32,
    max_request_bytes: i32,
    batch: Option<&'a str>,
    cache: Option<&'a str>,
    model: Option<&'a str>,
    timeout: i32,
    max_retries: i32,
    profile: Option<&'a str>,
    record: Option<&'a str>,
    replay: Option<&'a str>,
}

fn folder(value: Option<&str>) -> Result<Option<PathBuf>, Error> {
    let Some(value) = value.filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    let path = PathBuf::from(value);
    if !path.is_absolute() {
        return Err(super::usage("a SQL folder must be absolute"));
    }
    Ok(Some(path))
}

fn batch(value: Option<&str>) -> Result<Option<BatchSetting>, Error> {
    let Some(value) = value.filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    if value == "max" {
        return Ok(Some(BatchSetting::Max));
    }
    if !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(super::usage(
            "thinkthen.batch is max or a whole number of 1 or more",
        ));
    }
    let count = value
        .parse::<usize>()
        .ok()
        .and_then(NonZeroUsize::new)
        .ok_or_else(|| super::usage("thinkthen.batch is max or a whole number of 1 or more"))?;
    Ok(Some(BatchSetting::Records(count)))
}

impl Plan {
    pub(super) fn with_model(mut self, model: &str) -> Self {
        self.model = Some(model.to_owned());
        self
    }

    /// Read the four raw values. The throttle's check already holds it to
    /// -1 or 1..=32, and PostgreSQL's range checks hold the others to -1 or more.
    fn of(raw: Raw<'_>) -> Result<Self, Error> {
        let cache = if raw.cache == Some("off") {
            Some(None)
        } else {
            folder(raw.cache)?.map(Some)
        };
        Ok(Self {
            throttle: (raw.throttle != UNSET)
                .then(|| u8::try_from(raw.throttle).unwrap_or(u8::MAX)),
            max_requests: usize::try_from(raw.max_requests).ok(),
            max_request_bytes: usize::try_from(raw.max_request_bytes).ok(),
            batch: batch(raw.batch)?,
            cache,
            model: raw
                .model
                .filter(|value| !value.is_empty())
                .map(str::to_owned),
            timeout: u64::try_from(raw.timeout).ok().map(Duration::from_secs),
            max_retries: u32::try_from(raw.max_retries).ok(),
            profile: raw
                .profile
                .filter(|value| !value.is_empty())
                .map(str::to_owned),
            record: folder(raw.record)?,
            replay: folder(raw.replay)?,
        })
    }
}

/// Apply a plan to a seeded builder. A requested throttle always reaches
/// the public setter, which accepts the active width and refuses a change.
pub(super) fn apply(plan: &Plan, mut builder: EngineBuilder) -> Result<EngineBuilder, Error> {
    if let Some(value) = plan.throttle {
        // The public engine accepts an equal width and refuses a changed one.
        // Apply even when a width is already active.
        builder = builder.throttle(value)?;
    }
    if let Some(value) = plan.max_requests {
        builder = builder.max_requests(Some(value))?;
    }
    if let Some(value) = plan.max_request_bytes {
        builder = builder.max_request_bytes(value)?;
    }
    if let Some(value) = plan.batch {
        builder = builder.batch(value);
    }
    match &plan.cache {
        Some(Some(folder)) => builder = builder.cache_at(folder)?,
        Some(None) => builder = builder.no_cache(),
        None => {}
    }
    if let Some(model) = &plan.model {
        builder = builder.model(model)?;
    }
    if let Some(timeout) = plan.timeout {
        builder = builder.timeout(timeout)?;
    }
    if let Some(retries) = plan.max_retries {
        builder = builder.max_retries(retries);
    }
    if let Some(profile) = &plan.profile {
        builder = builder.profile_json(profile)?;
    }
    if let Some(record) = &plan.record {
        builder = builder.record(record)?;
    }
    if let Some(replay) = &plan.replay {
        builder = builder.replay(replay)?;
    }
    Ok(builder)
}

static DEADLINE_MS: GucSetting<i32> = GucSetting::<i32>::new(-1);
static API_KEY: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static FILE_DIRECTORY: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static THROTTLE: GucSetting<i32> = GucSetting::<i32>::new(UNSET);
static MAX_REQUESTS: GucSetting<i32> = GucSetting::<i32>::new(UNSET);
static MAX_REQUEST_BYTES: GucSetting<i32> = GucSetting::<i32>::new(UNSET);
static BATCH: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static MAX_REQUESTS_TOTAL: GucSetting<i32> = GucSetting::<i32>::new(UNSET);
static CACHE: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static MODEL: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static TIMEOUT: GucSetting<i32> = GucSetting::<i32>::new(UNSET);
static MAX_RETRIES: GucSetting<i32> = GucSetting::<i32>::new(UNSET);
static PROFILE: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static RECORD: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static REPLAY: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);

fn text_of(setting: &GucSetting<Option<CString>>) -> Option<String> {
    setting
        .get()
        .map(|held| held.to_string_lossy().into_owned())
}

/// Everything a call reads before its worker starts. `GucSetting::get`
/// panics off the backend thread, so this runs first in every function.
/// A set key is ignored by the engine, which reads only its server environment.
pub(crate) fn read() -> Call {
    read_result().or_raise()
}

/// Read settings without raising recoverable row failures.
pub(crate) fn read_result() -> Result<Call, Error> {
    if text_of(&API_KEY).is_some_and(|value| !value.is_empty()) {
        return Err(super::usage(
            "thinkthen.api_key is not read; unset it and set THINKTHEN_API_KEY in the server's environment",
        ));
    }
    let cache = text_of(&CACHE);
    let batch = text_of(&BATCH);
    let model = text_of(&MODEL);
    let profile = text_of(&PROFILE);
    let record = text_of(&RECORD);
    let replay = text_of(&REPLAY);
    let plan = Plan::of(Raw {
        throttle: THROTTLE.get(),
        max_requests: MAX_REQUESTS.get(),
        max_request_bytes: MAX_REQUEST_BYTES.get(),
        batch: batch.as_deref(),
        cache: cache.as_deref(),
        model: model.as_deref(),
        timeout: TIMEOUT.get(),
        max_retries: MAX_RETRIES.get(),
        profile: profile.as_deref(),
        record: record.as_deref(),
        replay: replay.as_deref(),
    })?;
    // Ian's ruling of 2026-09-25: the backend's total, computed once per call.
    let total = u64::try_from(MAX_REQUESTS_TOTAL.get()).ok();
    Ok(Call {
        plan,
        deadline_ms: i64::from(DEADLINE_MS.get()),
        context: None,
        batch: None,
        total,
    })
}

/// The one directory an unprivileged named-file read may touch.
pub(crate) fn file_directory() -> Option<String> {
    text_of(&FILE_DIRECTORY)
}

fn register_new_engine_settings() {
    let int = |name, about, setting, flags| {
        GucRegistry::define_int_guc(
            name,
            about,
            c"",
            setting,
            -1,
            i32::MAX,
            GucContext::Userset,
            flags,
        );
    };
    int(
        c"thinkthen.timeout",
        c"backend timeout in seconds; -1 keeps the environment default",
        &TIMEOUT,
        GucFlags::UNIT_S,
    );
    int(
        c"thinkthen.max_retries",
        c"backend status retries; -1 keeps the environment default",
        &MAX_RETRIES,
        GucFlags::default(),
    );
    GucRegistry::define_string_guc(
        c"thinkthen.batch",
        c"record member cap: max or a decimal whole number of 1 or more; empty keeps the environment default",
        c"",
        &BATCH,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_string_guc(
        c"thinkthen.model",
        c"backend model; empty keeps the environment default",
        c"",
        &MODEL,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_string_guc(
        c"thinkthen.profile",
        c"backend limits profile as JSON",
        c"",
        &PROFILE,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_string_guc(
        c"thinkthen.record",
        c"recording folder",
        c"",
        &RECORD,
        GucContext::Suset,
        GucFlags::default(),
    );
    GucRegistry::define_string_guc(
        c"thinkthen.replay",
        c"strict replay folder",
        c"",
        &REPLAY,
        GucContext::Suset,
        GucFlags::default(),
    );
}

/// Register the settings with PostgreSQL. `_PG_init` calls this alone.
pub(crate) fn register() {
    let int = |name, about, setting, most, context, flags| {
        GucRegistry::define_int_guc(name, about, c"", setting, -1, most, context, flags);
    };
    int(
        c"thinkthen.deadline_ms",
        c"per-call deadline in milliseconds: -1 none, 0 spent",
        &DEADLINE_MS,
        i32::MAX,
        GucContext::Userset,
        GucFlags::default(),
    );
    ffi::define_throttle(&THROTTLE);
    int(
        c"thinkthen.max_requests_total",
        c"most requests one backend sends; -1 means no total",
        &MAX_REQUESTS_TOTAL,
        i32::MAX,
        GucContext::Suset,
        GucFlags::default(),
    );
    int(
        c"thinkthen.max_requests",
        c"most records one call answers; -1 means no limit",
        &MAX_REQUESTS,
        i32::MAX,
        GucContext::Suset,
        GucFlags::default(),
    );
    int(
        c"thinkthen.max_request_bytes",
        c"positive request-byte ceiling; -1 keeps the environment default",
        &MAX_REQUEST_BYTES,
        i32::MAX,
        GucContext::Userset,
        GucFlags::default(),
    );
    register_new_engine_settings();
    GucRegistry::define_string_guc(
        c"thinkthen.cache",
        c"answer cache folder; empty leaves the environment's",
        c"",
        &CACHE,
        GucContext::Suset,
        GucFlags::default(),
    );
    GucRegistry::define_string_guc(
        c"thinkthen.file_directory",
        c"the only directory an unprivileged named-file read may touch",
        c"",
        &FILE_DIRECTORY,
        GucContext::Suset,
        GucFlags::NO_SHOW_ALL,
    );
    ffi::define_ignored_api_key(&API_KEY);
}

/// The active SQL total, read on PostgreSQL's backend thread.
pub(super) fn current_total() -> u64 {
    u64::try_from(MAX_REQUESTS_TOTAL.get()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::shown;

    /// Decision 3: the registered defaults plan no setter, and each set
    /// value reaches the plan. PostgreSQL's `'1MB'` arrives as bytes.
    #[test]
    fn the_registered_defaults_plan_nothing_and_set_values_carry() {
        let default = Raw {
            throttle: UNSET,
            max_requests: UNSET,
            max_request_bytes: UNSET,
            batch: None,
            cache: None,
            model: None,
            timeout: UNSET,
            max_retries: UNSET,
            profile: None,
            record: None,
            replay: None,
        };
        assert_eq!(shown(Plan::of(default)), Ok(Plan::default()));
        assert_eq!(
            shown(Plan::of(Raw {
                cache: Some(""),
                ..default
            })),
            Ok(Plan::default())
        );
        let set = Plan {
            throttle: Some(8),
            max_requests: Some(3),
            max_request_bytes: Some(20_000),
            batch: Some(BatchSetting::Records(NonZeroUsize::new(2).unwrap())),
            cache: Some(Some(PathBuf::from("/srv/cache"))),
            ..Plan::default()
        };
        assert_eq!(
            shown(Plan::of(Raw {
                throttle: 8,
                max_requests: 3,
                max_request_bytes: 20_000,
                batch: Some("2"),
                cache: Some("/srv/cache"),
                ..default
            })),
            Ok(set)
        );
        for invalid in [
            "0",
            "-1",
            "+1",
            "1.5",
            "MAX",
            "no",
            "999999999999999999999999999999999999",
        ] {
            assert!(batch(Some(invalid)).is_err());
        }
        assert_eq!(shown(batch(None)), Ok(None));
        assert_eq!(shown(batch(Some(""))), Ok(None));
        assert_eq!(shown(batch(Some("max"))), Ok(Some(BatchSetting::Max)));
    }
}
