//! PostgreSQL GUC registration and raw engine settings captured before the worker.
use super::{Call, OrRaise};
use crate::ffi;
use pgrx::{GucContext, GucFlags, GucRegistry, GucSetting};
use std::ffi::{CStr, CString};
use thinkthen::{EngineBuilder, Error};
pub(crate) const UNSET: i32 = -1;
pub(crate) type SettingPairs = Vec<(String, String)>;

pub(super) fn apply(
    pairs: &SettingPairs,
    mut builder: EngineBuilder,
) -> Result<EngineBuilder, Error> {
    for (name, raw) in pairs {
        builder = builder.setting(name, raw)?;
    }
    Ok(builder)
}

enum Control {
    Number(&'static GucSetting<i32>, GucFlags),
    Text(&'static GucSetting<Option<CString>>),
    Flag(&'static GucSetting<bool>),
}
type Registration = (&'static CStr, &'static CStr, GucContext, Control);
macro_rules! controls {
    ($( $field:ident: $ty:ty = $default:expr => $name:literal, $about:literal, $context:ident, $kind:ident $(($flags:expr))?; )*) => {
        $(static $field: GucSetting<$ty> = GucSetting::<$ty>::new($default);)*
        fn controls() -> Vec<Registration> {
            vec![$(($name, $about, GucContext::$context, Control::$kind(&$field $(, $flags)?)),)*]
        }
    };
}
controls! {
    THROTTLE: i32 = UNSET => c"thinkthen.throttle", c"maximum concurrent requests; -1 keeps the environment default", Userset, Number(GucFlags::default());
    MAX_REQUESTS: i32 = UNSET => c"thinkthen.max_requests", c"most records one call answers; -1 means no limit", Suset, Number(GucFlags::default());
    MAX_REQUEST_BYTES: i32 = UNSET => c"thinkthen.max_request_bytes", c"positive request-byte ceiling; -1 keeps the environment default", Userset, Number(GucFlags::default());
    TIMEOUT: i32 = UNSET => c"thinkthen.timeout", c"backend timeout in seconds; -1 keeps the environment default", Userset, Number(GucFlags::UNIT_S);
    MAX_RETRIES: i32 = UNSET => c"thinkthen.max_retries", c"backend status retries; -1 keeps the environment default", Userset, Number(GucFlags::default());
    BATCH: Option<CString> = None => c"thinkthen.batch", c"record member cap: max or a decimal whole number of 1 or more; empty keeps the environment default", Userset, Text;
    CACHE: Option<CString> = None => c"thinkthen.cache", c"answer cache folder; empty leaves the environment's", Suset, Text;
    BACKEND: Option<CString> = None => c"thinkthen.backend", c"named backend; empty keeps captured selection", Suset, Text;
    MODEL: Option<CString> = None => c"thinkthen.model", c"backend model; empty keeps the environment default", Userset, Text;
    PROFILE: Option<CString> = None => c"thinkthen.profile", c"backend limits profile as JSON", Userset, Text;
    RECORD: Option<CString> = None => c"thinkthen.record", c"recording folder", Suset, Text;
    REPLAY: Option<CString> = None => c"thinkthen.replay", c"strict replay folder", Suset, Text;
    BASE: Option<CString> = None => c"thinkthen.base_url", c"Explicit backend address", Suset, Text;
    INPUT: Option<CString> = None => c"thinkthen.usd_per_million_input", c"Caller input price per million tokens", Userset, Text;
    OUTPUT: Option<CString> = None => c"thinkthen.usd_per_million_output", c"Caller output price per million tokens", Userset, Text;
    REFRESH: bool = false => c"thinkthen.refresh_cache", c"Refresh native cached answers", Suset, Flag;
}
static DEADLINE_MS: GucSetting<i32> = GucSetting::<i32>::new(-1);
static TOTAL: GucSetting<i32> = GucSetting::<i32>::new(-1);
static API_KEY: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static FILE_DIRECTORY: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);

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
    let mut plan = Vec::new();
    for (name, _, _, control) in controls() {
        let key = name
            .to_str()
            .map_err(|_| super::defect("invalid setting name"))?
            .strip_prefix("thinkthen.")
            .ok_or_else(|| super::defect("invalid setting name"))?;
        let raw = match control {
            Control::Number(setting, _) => {
                let value = setting.get();
                if value == UNSET {
                    continue;
                }
                value.to_string()
            }
            Control::Text(setting) => {
                let Some(value) = text_of(setting).filter(|value| !value.is_empty()) else {
                    continue;
                };
                if matches!(key, "cache" | "record" | "replay")
                    && value != "off"
                    && !std::path::Path::new(&value).is_absolute()
                {
                    return Err(super::usage("a SQL folder must be absolute"));
                }
                if key == "batch" && value != "max" {
                    value
                } else if key == "cache" && value == "off" {
                    "false".to_owned()
                } else {
                    serde_json::Value::String(value).to_string()
                }
            }
            Control::Flag(setting) => setting.get().to_string(),
        };
        plan.push((key.to_owned(), raw));
    }
    // Ian's ruling of 2026-09-25: the backend's total, computed once per call.
    let total = u64::try_from(TOTAL.get()).ok();
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

/// Register the settings with PostgreSQL. `_PG_init` calls this alone.
pub(crate) fn register() {
    for (name, about, context, control) in controls() {
        match control {
            Control::Number(setting, flags) if name == c"thinkthen.throttle" => {
                let _ = flags;
                ffi::define_throttle(setting);
            }
            Control::Number(setting, flags) => {
                GucRegistry::define_int_guc(name, about, c"", setting, -1, i32::MAX, context, flags)
            }
            Control::Text(setting) => GucRegistry::define_string_guc(
                name,
                about,
                c"",
                setting,
                context,
                GucFlags::default(),
            ),
            Control::Flag(setting) => GucRegistry::define_bool_guc(
                name,
                about,
                c"",
                setting,
                context,
                GucFlags::default(),
            ),
        }
    }
    GucRegistry::define_int_guc(
        c"thinkthen.deadline_ms",
        c"per-call deadline in milliseconds: -1 none, 0 spent",
        c"",
        &DEADLINE_MS,
        -1,
        i32::MAX,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_int_guc(
        c"thinkthen.max_requests_total",
        c"most requests one backend sends; -1 means no total",
        c"",
        &TOTAL,
        -1,
        i32::MAX,
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
    u64::try_from(TOTAL.get()).unwrap_or_default()
}
