//! One call: the error table, the setter plan, the engine each backend
//! keeps, and the worker every call runs on (ticket 0111 decisions 3, 7,
//! and 11; ADR 0043 as amended).

use std::ffi::CString;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

use pgrx::prelude::*;
use pgrx::{GucContext, GucFlags, GucRegistry, GucSetting};
use thinkthen::{CallOptions, CancelToken, Engine, EngineBuilder, Error, ErrorKind};

use crate::ffi;

/// A failure on its way to PostgreSQL: the kind, a safe message, and the
/// retry signal. `thinkthen::Error` has no public constructor, so the
/// binding's own refusals take this shape too.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Refusal {
    kind: ErrorKind,
    message: String,
    retryable: bool,
}

impl Refusal {
    pub(crate) fn of(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            retryable: false,
        }
    }

    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self::of(ErrorKind::Usage, message)
    }

    /// The message PostgreSQL shows: `thinkthen <kind>: <message> (retryable: yes|no)`.
    pub(crate) fn text(&self) -> String {
        let retry = if self.retryable { "yes" } else { "no" };
        format!(
            "thinkthen {}: {} (retryable: {retry})",
            self.kind.name(),
            self.message
        )
    }
}

impl From<Error> for Refusal {
    fn from(error: Error) -> Self {
        Self {
            kind: error.kind(),
            message: error.detail().message().to_owned(),
            retryable: error.retryable(),
        }
    }
}

/// The one error table: each kind's SQLSTATE.
pub(crate) const fn sqlstate(kind: ErrorKind) -> PgSqlErrorCode {
    match kind {
        ErrorKind::Usage => PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE,
        ErrorKind::Backend => PgSqlErrorCode::ERRCODE_EXTERNAL_ROUTINE_EXCEPTION,
        ErrorKind::Local => PgSqlErrorCode::ERRCODE_IO_ERROR,
        ErrorKind::Cancelled | ErrorKind::Deadline => PgSqlErrorCode::ERRCODE_QUERY_CANCELED,
        ErrorKind::Defect => PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
    }
}

/// Raise a failure as PostgreSQL's own error. A failure never reads as NULL.
pub(crate) fn raise(refusal: Refusal) -> ! {
    ereport!(ERROR, sqlstate(refusal.kind), refusal.text());
}

/// Unwrap a value or raise its failure.
pub(crate) trait OrRaise<T> {
    fn or_raise(self) -> T;
}

impl<T, E: Into<Refusal>> OrRaise<T> for Result<T, E> {
    fn or_raise(self) -> T {
        self.unwrap_or_else(|error| raise(error.into()))
    }
}

/// The registered value that leaves a numeric engine setting unset.
pub(crate) const UNSET: i32 = -1;

/// The refusal for a zero cache cap (decision 3).
pub(crate) const CACHE_BYTES_ZERO: &str = "thinkthen.cache_bytes must be -1 or at least 1";

/// The setter calls the four engine settings ask for. An unset setting
/// calls nothing, so the value `EngineBuilder::from_env` seeded stands.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Plan {
    throttle: Option<u8>,
    max_requests: Option<usize>,
    cache: Option<PathBuf>,
    cache_bytes: Option<u64>,
}

impl Plan {
    /// Read the four raw values. PostgreSQL's range checks already hold
    /// throttle to -1..=32 and the others to -1 or more.
    pub(crate) fn of(
        throttle: i32,
        max_requests: i32,
        cache: Option<&str>,
        cache_bytes: i32,
    ) -> Result<Self, Refusal> {
        if cache_bytes == 0 {
            return Err(Refusal::usage(CACHE_BYTES_ZERO));
        }
        Ok(Self {
            throttle: (throttle != UNSET).then(|| u8::try_from(throttle).unwrap_or(u8::MAX)),
            max_requests: usize::try_from(max_requests).ok(),
            cache: cache.filter(|folder| !folder.is_empty()).map(PathBuf::from),
            cache_bytes: u64::try_from(cache_bytes).ok(),
        })
    }

    /// The request limit, which a call over held records checks before any send.
    pub(crate) const fn most(&self) -> Option<usize> {
        self.max_requests
    }
}

/// Apply a plan to a seeded builder. The throttle passes only while this
/// backend has no explicit throttle active, because 0077 refuses a second,
/// different one in one process.
fn apply(
    plan: &Plan,
    active_throttle: Option<u8>,
    mut builder: EngineBuilder,
) -> Result<EngineBuilder, Error> {
    if let (Some(value), None) = (plan.throttle, active_throttle) {
        builder = builder.throttle(value)?;
    }
    if let Some(value) = plan.max_requests {
        builder = builder.max_requests(Some(value))?;
    }
    if let Some(folder) = &plan.cache {
        builder = builder.cache_at(folder)?;
    }
    if let Some(value) = plan.cache_bytes {
        builder = builder.cache_bytes(value)?;
    }
    Ok(builder)
}

/// The explicit throttle this backend registered, 0 for none.
static ACTIVE_THROTTLE: AtomicU8 = AtomicU8::new(0);

/// Every engine this backend built, the current one last, beside the plan
/// it was built from. Each engine counts its own sends, so the usage
/// totals add them all.
static ENGINES: Mutex<Vec<(Plan, Engine)>> = Mutex::new(Vec::new());

fn engines() -> std::sync::MutexGuard<'static, Vec<(Plan, Engine)>> {
    ENGINES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Build an engine from the server's environment and the plan.
fn build(plan: &Plan) -> Result<Engine, Error> {
    let active = std::num::NonZeroU8::new(ACTIVE_THROTTLE.load(Ordering::Acquire)).map(u8::from);
    let engine = apply(plan, active, EngineBuilder::from_env()?)?.build()?;
    if let (None, Some(value)) = (active, plan.throttle) {
        ACTIVE_THROTTLE.store(value, Ordering::Release);
    }
    Ok(engine)
}

/// This backend's totals: requests sent, cache answers, input and output tokens.
pub(crate) fn totals() -> [u64; 4] {
    engines().iter().fold([0; 4], |sum, (_, engine)| {
        let counts = engine.usage();
        let [sent, cached, input, output] = sum;
        [
            sent.saturating_add(counts.requests_sent()),
            cached.saturating_add(counts.cache_answers()),
            input.saturating_add(counts.input_tokens()),
            output.saturating_add(counts.output_tokens()),
        ]
    })
}

/// What a call read on the backend thread before its worker starts.
#[derive(Debug)]
pub(crate) struct Call {
    pub(crate) plan: Plan,
    pub(crate) deadline_ms: i32,
}

/// How often the backend thread looks at its interrupt flags and deadline.
const TICK: Duration = Duration::from_millis(50);

/// How a wait ended: an answer, a worker lost, a cancel, or a passed deadline.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Waited<T> {
    Done(T),
    Lost,
    Cancel,
    Deadline,
}

/// Wait on the worker in ticks. A finished call returns at once, and each
/// tick runs `poll`.
pub(crate) fn wait<T>(
    answer: &Receiver<T>,
    tick: Duration,
    mut poll: impl FnMut() -> Option<Waited<T>>,
) -> Waited<T> {
    loop {
        match answer.recv_timeout(tick) {
            Ok(value) => return Waited::Done(value),
            Err(RecvTimeoutError::Disconnected) => return Waited::Lost,
            Err(RecvTimeoutError::Timeout) => {
                if let Some(stop) = poll() {
                    return stop;
                }
            }
        }
    }
}

/// Run the work and send its result. A detached caller closed the channel,
/// and the result is dropped.
pub(crate) fn deliver<T>(answer: &Sender<T>, work: impl FnOnce() -> T) {
    let _ignored = answer.send(work());
}

/// A worker that ended without a result.
pub(crate) fn lost() -> Refusal {
    Refusal::of(
        ErrorKind::Defect,
        "the call's worker stopped without a result",
    )
}

type Built<T> = (Option<Engine>, Result<T, Error>);

/// Run one call on a detachable masked worker, waiting in 50 ms ticks. A
/// cancel, terminate, or statement timeout cancels the token, detaches the
/// worker, and raises PostgreSQL's error; a passed deadline raises `deadline`.
pub(crate) fn run<T: Send + 'static>(
    call: Call,
    work: impl FnOnce(&Engine, CallOptions<'_>) -> Result<T, Error> + Send + 'static,
) -> T {
    let held = engines()
        .last()
        .filter(|(plan, _)| *plan == call.plan)
        .map(|(_, engine)| engine.clone());
    let millis = call.deadline_ms;
    let due = u64::try_from(millis)
        .ok()
        .map(|budget| Instant::now() + Duration::from_millis(budget));
    let token = CancelToken::new();
    let (answer, answered) = mpsc::channel::<Built<T>>();
    let (plan, worker_token) = (call.plan.clone(), token.clone());
    ffi::spawn_masked(move || {
        deliver(&answer, || {
            let (engine, built) = match held {
                Some(engine) => (engine, false),
                None => match build(&plan) {
                    Ok(engine) => (engine, true),
                    Err(error) => return (None, Err(error)),
                },
            };
            let result = CallOptions::new()
                .cancel(&worker_token)
                .deadline_millis(i64::from(millis))
                .and_then(|options| work(&engine, options));
            (built.then_some(engine), result)
        });
    })
    .map_err(|_| Refusal::of(ErrorKind::Defect, "the call's worker could not start"))
    .or_raise();
    let waited = wait(&answered, TICK, || {
        if ffi::cancel_pending() {
            Some(Waited::Cancel)
        } else if due.is_some_and(|due| Instant::now() >= due) {
            Some(Waited::Deadline)
        } else {
            None
        }
    });
    match waited {
        Waited::Done((built, result)) => {
            if let Some(engine) = built {
                engines().push((call.plan, engine));
            }
            result.or_raise()
        }
        Waited::Lost => raise(lost()),
        Waited::Cancel => {
            token.cancel();
            ffi::raise_pending_interrupt();
            raise(Refusal::of(ErrorKind::Cancelled, "the call was cancelled"))
        }
        Waited::Deadline => {
            token.cancel();
            raise(Refusal::of(
                ErrorKind::Deadline,
                format!("the call's deadline of {millis} ms passed"),
            ))
        }
    }
}

static DEADLINE_MS: GucSetting<i32> = GucSetting::<i32>::new(-1);
static API_KEY: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static FILE_DIRECTORY: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static THROTTLE: GucSetting<i32> = GucSetting::<i32>::new(UNSET);
static MAX_REQUESTS: GucSetting<i32> = GucSetting::<i32>::new(UNSET);
static CACHE: GucSetting<Option<CString>> = GucSetting::<Option<CString>>::new(None);
static CACHE_BYTES: GucSetting<i32> = GucSetting::<i32>::new(UNSET);

fn text_of(setting: &GucSetting<Option<CString>>) -> Option<String> {
    setting
        .get()
        .map(|held| held.to_string_lossy().into_owned())
}

/// Everything a call reads before its worker starts. `GucSetting::get`
/// panics off the backend thread, so this runs first in every function.
/// A set key refuses every call (decision 12), and names no value.
pub(crate) fn read() -> Call {
    if text_of(&API_KEY).is_some_and(|key| !key.trim().is_empty()) {
        raise(Refusal::usage(
            "thinkthen.api_key is not read; unset it and set THINKTHEN_API_KEY in the server's environment",
        ));
    }
    let cache = text_of(&CACHE);
    let plan = Plan::of(
        THROTTLE.get(),
        MAX_REQUESTS.get(),
        cache.as_deref(),
        CACHE_BYTES.get(),
    )
    .or_raise();
    Call {
        plan,
        deadline_ms: DEADLINE_MS.get(),
    }
}

/// The one directory an unprivileged named-file read may touch.
pub(crate) fn file_directory() -> Option<String> {
    text_of(&FILE_DIRECTORY)
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
    int(
        c"thinkthen.throttle",
        c"requests in flight at once; -1 leaves the engine default",
        &THROTTLE,
        32,
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
        c"thinkthen.cache_bytes",
        c"cache cap in bytes; -1 leaves the configured value",
        &CACHE_BYTES,
        i32::MAX,
        GucContext::Suset,
        GucFlags::UNIT_BYTE,
    );
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
    // `Userset`, so a `SET` never fails and never logs its statement; the
    // next call refuses instead (amendment item 2).
    GucRegistry::define_string_guc(
        c"thinkthen.api_key",
        c"not read; the key comes from THINKTHEN_API_KEY in the server's environment",
        c"",
        &API_KEY,
        GucContext::Userset,
        GucFlags::NO_SHOW_ALL | GucFlags::SUPERUSER_ONLY | GucFlags::DISALLOW_IN_AUTO_FILE,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A zero cache cap refuses with the pinned sentence before any setter.
    #[test]
    fn a_zero_cache_cap_refuses_before_any_setter() {
        assert_eq!(
            Plan::of(UNSET, UNSET, None, 0),
            Err(Refusal::usage(CACHE_BYTES_ZERO))
        );
        assert_eq!(
            Refusal::usage(CACHE_BYTES_ZERO).text(),
            "thinkthen usage: thinkthen.cache_bytes must be -1 or at least 1 (retryable: no)"
        );
    }

    /// R1-31 and R2-31: the one table maps every kind to its SQLSTATE.
    #[test]
    fn each_kind_has_its_sqlstate() {
        let table = [
            (
                ErrorKind::Usage,
                PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE,
            ),
            (
                ErrorKind::Backend,
                PgSqlErrorCode::ERRCODE_EXTERNAL_ROUTINE_EXCEPTION,
            ),
            (ErrorKind::Local, PgSqlErrorCode::ERRCODE_IO_ERROR),
            (ErrorKind::Cancelled, PgSqlErrorCode::ERRCODE_QUERY_CANCELED),
            (ErrorKind::Deadline, PgSqlErrorCode::ERRCODE_QUERY_CANCELED),
            (ErrorKind::Defect, PgSqlErrorCode::ERRCODE_INTERNAL_ERROR),
        ];
        for (kind, code) in table {
            assert_eq!(sqlstate(kind), code, "{kind:?}");
            assert_eq!(
                Refusal::of(kind, "why").text(),
                format!("thinkthen {}: why (retryable: no)", kind.name())
            );
        }
    }

    /// A worker whose caller detached finishes without a panic.
    #[test]
    fn a_worker_survives_a_closed_channel() {
        let (answer, answered) = mpsc::channel();
        drop(answered);
        let worker = std::thread::spawn(move || deliver(&answer, || 7));
        assert!(worker.join().is_ok());
    }

    /// A channel that closes with no result reads as lost, which raises `defect`.
    #[test]
    fn a_channel_closed_without_a_result_is_a_defect() {
        let (answer, answered) = mpsc::channel::<u8>();
        std::thread::spawn(move || drop(answer));
        assert_eq!(wait(&answered, TICK, || None), Waited::Lost);
        assert_eq!(
            lost().text(),
            "thinkthen defect: the call's worker stopped without a result (retryable: no)"
        );
    }
}
