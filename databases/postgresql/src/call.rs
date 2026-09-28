//! One call: the error table, the setter plan, the engine each backend
//! keeps, and the worker every call runs on (ticket 0111 decisions 3, 7,
//! and 11; ADR 0043 as amended).

use std::ffi::CString;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

use pgrx::prelude::*;
use pgrx::{GucContext, GucFlags, GucRegistry, GucSetting};
use thinkthen::{CallOptions, CancelToken, Engine, EngineBuilder, Error, ErrorKind, SendBudget, SendBudgetDenial};

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

    /// A failed row carries only its public kind and fixed advice.
    pub(crate) fn value(&self) -> Option<serde_json::Value> {
        let message = match self.kind {
            ErrorKind::Usage => {
                "check the row's question and arguments, or raise the process request total when it is spent"
            }
            ErrorKind::Local => "check the named file and its permissions",
            ErrorKind::Backend => "the backend did not answer; retry if allowed",
            ErrorKind::Cancelled | ErrorKind::Deadline | ErrorKind::Defect => return None,
        };
        Some(
            serde_json::json!({"status":"failed","error":{"kind":self.kind.name(),"message":message,"retryable":self.retryable}}),
        )
    }
}

impl From<Error> for Refusal {
    fn from(error: Error) -> Self {
        if matches!(error.send_budget_denial(), Some(SendBudgetDenial::BeforeFirstSend | SendBudgetDenial::BeforeRetry { .. })) {
            return spent(u64::try_from(MAX_REQUESTS_TOTAL.get()).unwrap_or_default());
        }
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

/// The throttle's refusal where it is set, in the engine's own sentence, or
/// `None` for -1 (unset) and 1 through 32 (Ian's range).
pub(crate) fn throttle_refusal(value: i32) -> Option<String> {
    (value != UNSET && !(1..=32).contains(&value))
        .then(|| Refusal::usage("a throttle is a whole number from 1 through 32").text())
}

/// The setter calls the four engine settings ask for. An unset setting
/// calls nothing, so the value `EngineBuilder::from_env` seeded stands.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Plan {
    throttle: Option<u8>,
    max_requests: Option<usize>,
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
    cache: Option<&'a str>,
    model: Option<&'a str>,
    timeout: i32,
    max_retries: i32,
    profile: Option<&'a str>,
    record: Option<&'a str>,
    replay: Option<&'a str>,
}

fn folder(value: Option<&str>) -> Result<Option<PathBuf>, Refusal> {
    let Some(value) = value.filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    let path = PathBuf::from(value);
    if !path.is_absolute() {
        return Err(Refusal::usage("a SQL folder must be absolute"));
    }
    Ok(Some(path))
}

impl Plan {
    /// Read the four raw values. The throttle's check already holds it to
    /// -1 or 1..=32, and PostgreSQL's range checks hold the others to -1 or more.
    fn of(raw: Raw<'_>) -> Result<Self, Refusal> {
        let cache = if raw.cache == Some("off") { Some(None) } else { folder(raw.cache)?.map(Some) };
        Ok(Self {
            throttle: (raw.throttle != UNSET).then(|| u8::try_from(raw.throttle).unwrap_or(u8::MAX)),
            max_requests: usize::try_from(raw.max_requests).ok(),
            cache,
            model: raw.model.filter(|value| !value.is_empty()).map(str::to_owned),
            timeout: u64::try_from(raw.timeout).ok().map(Duration::from_secs),
            max_retries: u32::try_from(raw.max_retries).ok(),
            profile: raw.profile.filter(|value| !value.is_empty()).map(str::to_owned),
            record: folder(raw.record)?,
            replay: folder(raw.replay)?,
        })
    }
}

/// Apply a plan to a seeded builder. A requested throttle always reaches
/// the public setter, which accepts the active width and refuses a change.
fn apply(plan: &Plan, mut builder: EngineBuilder) -> Result<EngineBuilder, Error> {
    if let Some(value) = plan.throttle {
        // The public engine accepts an equal width and refuses a changed one.
        // Apply even when a width is already active.
        builder = builder.throttle(value)?;
    }
    if let Some(value) = plan.max_requests {
        builder = builder.max_requests(Some(value))?;
    }
    match &plan.cache {
        Some(Some(folder)) => builder = builder.cache_at(folder)?,
        Some(None) => builder = builder.no_cache(),
        None => {}
    }
    if let Some(model) = &plan.model { builder = builder.model(model)?; }
    if let Some(timeout) = plan.timeout { builder = builder.timeout(timeout)?; }
    if let Some(retries) = plan.max_retries { builder = builder.max_retries(retries); }
    if let Some(profile) = &plan.profile { builder = builder.profile_json(profile)?; }
    if let Some(record) = &plan.record { builder = builder.record(record)?; }
    if let Some(replay) = &plan.replay { builder = builder.replay(replay)?; }
    Ok(builder)
}

/// The explicit throttle this backend registered, 0 for none.
static ACTIVE_THROTTLE: AtomicU8 = AtomicU8::new(0);

/// One engine per plan this backend used. Each engine counts its own sends,
/// so the usage totals add them all.
static ENGINES: Mutex<Vec<(Plan, Engine)>> = Mutex::new(Vec::new());
static SEND_BUDGET: OnceLock<SendBudget> = OnceLock::new();

fn engines() -> std::sync::MutexGuard<'static, Vec<(Plan, Engine)>> {
    ENGINES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Build an engine from the server's environment and the plan.
fn build(plan: &Plan) -> Result<Engine, Error> {
    let active = std::num::NonZeroU8::new(ACTIVE_THROTTLE.load(Ordering::Acquire)).map(u8::from);
    let engine = apply(plan, EngineBuilder::from_env()?)?.build()?;
    if let (None, Some(value)) = (active, plan.throttle) {
        ACTIVE_THROTTLE.store(value, Ordering::Release);
    }
    // Kept before the call runs, so a cancelled call's sends still count.
    // A worker that lost the race uses the recorded engine, so every send counts.
    let mut all = engines();
    if let Some((_, held)) = all.iter().find(|(held, _)| held == plan) {
        return Ok(held.clone());
    }
    all.push((plan.clone(), engine.clone()));
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
    /// `thinkthen.max_requests_total`, and the requests it still leaves.
    total: Option<u64>,
    left: Option<u64>,
}

/// The refusal once `thinkthen.max_requests_total` is spent.
fn spent(total: u64) -> Refusal {
    Refusal::usage(format!(
        "thinkthen.max_requests_total allows {total} requests in this backend, and they are spent"
    ))
}

impl Call {
    /// Refuse held records over `max_requests` before any send. Return how
    /// many of them the total leaves, when that is fewer than all.
    pub(crate) fn within(&self, count: usize) -> Option<usize> {
        if let Some(most) = self.plan.max_requests.filter(|most| count > *most) {
            raise(Refusal::usage(format!(
                "this engine answers at most {most} records in one call"
            )));
        }
        let left = usize::try_from(self.left?).unwrap_or(usize::MAX);
        (count > left).then_some(left)
    }

    /// The refusal after a call that sent only what the total left.
    pub(crate) fn spent(&self) -> Refusal {
        spent(self.total.unwrap_or_default())
    }
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

/// Run one call on a detachable masked worker, waiting in 50 ms ticks. A
/// cancel, terminate, or statement timeout cancels the token, detaches the
/// worker, and raises PostgreSQL's error; a passed deadline raises `deadline`.
pub(crate) fn run<T: Send + 'static>(
    call: Call,
    work: impl FnOnce(&Engine, CallOptions<'_>) -> Result<T, Error> + Send + 'static,
) -> T {
    run_result(call, work).or_raise()
}

/// Return row failures as typed data. Host cancel and deadline still raise.
pub(crate) fn run_result<T: Send + 'static>(
    call: Call,
    work: impl FnOnce(&Engine, CallOptions<'_>) -> Result<T, Error> + Send + 'static,
) -> Result<T, Refusal> {
    let held = engines()
        .iter()
        .find(|(plan, _)| *plan == call.plan)
        .map(|(_, engine)| engine.clone());
    let millis = call.deadline_ms;
    let due = u64::try_from(millis)
        .ok()
        .map(|budget| Instant::now() + Duration::from_millis(budget));
    let token = CancelToken::new();
    let (answer, answered) = mpsc::channel::<Result<T, Error>>();
    let (plan, worker_token) = (call.plan.clone(), token.clone());
    let total = call.total;
    let send_budget = SEND_BUDGET.get_or_init(SendBudget::new);
    ffi::spawn_masked(move || {
        deliver(&answer, || {
            let engine = match held {
                Some(engine) => engine,
                None => build(&plan)?,
            };
            CallOptions::new()
                .cancel(&worker_token)
                .send_budget(send_budget, total)
                .deadline_millis(i64::from(millis))
                .and_then(|options| work(&engine, options))
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
        Waited::Done(result) => result.map_err(Into::into),
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
/// A set key refuses every call (decision 12), and names no value.
pub(crate) fn read() -> Call {
    read_result().or_raise()
}

/// Read settings without raising recoverable row failures.
pub(crate) fn read_result() -> Result<Call, Refusal> {
    if text_of(&API_KEY).is_some_and(|key| !key.trim().is_empty()) {
        return Err(Refusal::usage(
            "thinkthen.api_key is not read; unset it and set THINKTHEN_API_KEY in the server's environment",
        ));
    }
    let cache = text_of(&CACHE);
    let model = text_of(&MODEL);
    let profile = text_of(&PROFILE);
    let record = text_of(&RECORD);
    let replay = text_of(&REPLAY);
    let plan = Plan::of(Raw {
        throttle: THROTTLE.get(),
        max_requests: MAX_REQUESTS.get(),
        cache: cache.as_deref(),
        model: model.as_deref(),
        timeout: TIMEOUT.get(),
        max_retries: MAX_RETRIES.get(),
        profile: profile.as_deref(),
        record: record.as_deref(),
        replay: replay.as_deref(),
    })?;
    let active = ACTIVE_THROTTLE.load(Ordering::Acquire);
    if plan
        .throttle
        .is_some_and(|requested| active != 0 && requested != active)
    {
        return Err(Refusal::usage(format!(
            "throttle {active} is already active for this process; use throttle {active} or drop the throttle argument"
        )));
    }
    // Ian's ruling of 2026-09-25: the backend's total, computed once per call.
    let total = u64::try_from(MAX_REQUESTS_TOTAL.get()).ok();
    let left = total.map(|total| total.saturating_sub(totals()[0]));
    if left == Some(0) {
        return Err(spent(total.unwrap_or_default()));
    }
    Ok(Call {
        plan,
        deadline_ms: DEADLINE_MS.get(),
        total,
        left,
    })
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
    int(c"thinkthen.timeout", c"backend timeout in seconds; -1 keeps the environment default", &TIMEOUT, i32::MAX, GucContext::Userset, GucFlags::UNIT_S);
    int(c"thinkthen.max_retries", c"backend status retries; -1 keeps the environment default", &MAX_RETRIES, i32::MAX, GucContext::Userset, GucFlags::default());
    GucRegistry::define_string_guc(c"thinkthen.model", c"backend model; empty keeps the environment default", c"", &MODEL, GucContext::Userset, GucFlags::default());
    GucRegistry::define_string_guc(c"thinkthen.profile", c"backend limits profile as JSON", c"", &PROFILE, GucContext::Userset, GucFlags::default());
    GucRegistry::define_string_guc(c"thinkthen.record", c"recording folder", c"", &RECORD, GucContext::Suset, GucFlags::default());
    GucRegistry::define_string_guc(c"thinkthen.replay", c"strict replay folder", c"", &REPLAY, GucContext::Suset, GucFlags::default());
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

    /// Decision 3: the registered defaults plan no setter, and each set
    /// value reaches the plan. PostgreSQL's `'1MB'` arrives as bytes.
    #[test]
    fn the_registered_defaults_plan_nothing_and_set_values_carry() {
        let default = Raw { throttle: UNSET, max_requests: UNSET, cache: None, model: None, timeout: UNSET, max_retries: UNSET, profile: None, record: None, replay: None };
        assert_eq!(Plan::of(default), Ok(Plan::default()));
        assert_eq!(Plan::of(Raw { cache: Some(""), ..default }), Ok(Plan::default()));
        let set = Plan {
            throttle: Some(8),
            max_requests: Some(3),
            cache: Some(Some(PathBuf::from("/srv/cache"))),
            ..Plan::default()
        };
        assert_eq!(Plan::of(Raw { throttle: 8, max_requests: 3, cache: Some("/srv/cache"), ..default }), Ok(set));
    }

    /// R1-31 and R2-31: the one table maps every kind to its SQLSTATE.
    #[test]
    fn each_kind_has_its_sqlstate() {
        let table = [
            (
                ErrorKind::Usage,
                "usage",
                PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE,
            ),
            (
                ErrorKind::Backend,
                "backend",
                PgSqlErrorCode::ERRCODE_EXTERNAL_ROUTINE_EXCEPTION,
            ),
            (ErrorKind::Local, "local", PgSqlErrorCode::ERRCODE_IO_ERROR),
            (
                ErrorKind::Cancelled,
                "cancelled",
                PgSqlErrorCode::ERRCODE_QUERY_CANCELED,
            ),
            (
                ErrorKind::Deadline,
                "deadline",
                PgSqlErrorCode::ERRCODE_QUERY_CANCELED,
            ),
            (
                ErrorKind::Defect,
                "defect",
                PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
            ),
        ];
        for (kind, word, code) in table {
            assert_eq!(sqlstate(kind), code, "{kind:?}");
            assert_eq!(
                Refusal::of(kind, "why").text(),
                format!("thinkthen {word}: why (retryable: no)")
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
