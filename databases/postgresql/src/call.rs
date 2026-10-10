//! One call: the error table, the setter plan, the engine each backend
//! keeps, and the worker every call runs on (ticket 0111 decisions 3, 7,
//! and 11; ADR 0043 as amended).

use std::any::Any;
use std::panic::AssertUnwindSafe;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

use pgrx::pg_sys::panic::{CaughtError, ErrorReport, ErrorReportWithLevel};
use pgrx::prelude::*;
use thinkthen::{
    BatchSetting, CallOptions, CancelToken, Counters, Engine, EngineBuilder, Error, ErrorKind,
    SendBudgetDenial, Settings,
};

use crate::ffi;

mod settings;
pub(crate) use settings::{Plan, file_directory, read, read_result, register, throttle_refusal};

/// A refusal of the call or its arguments, as `usage`.
pub(crate) fn usage(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Usage, message)
}

/// A fault inside the extension, as `defect`.
pub(crate) fn defect(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Defect, message)
}

/// The message PostgreSQL shows: `thinkthen <kind>: <message> (retryable: yes|no)`.
pub(crate) fn text(error: &Error) -> String {
    let retry = if error.retryable() { "yes" } else { "no" };
    format!(
        "thinkthen {}: {} (retryable: {retry})",
        error.kind().name(),
        error.detail().message()
    )
}

/// A result with its error as the text PostgreSQL shows, for tests.
#[cfg(test)]
pub(crate) fn shown<T>(result: Result<T, Error>) -> Result<T, String> {
    result.map_err(|error| text(&error))
}

/// A failed row carries only its public kind and fixed advice.
pub(crate) fn failed_row(error: &Error) -> Option<serde_json::Value> {
    let message = match error.kind() {
        ErrorKind::Usage => {
            "check the row's question and arguments, or raise the process request total when it is spent"
        }
        ErrorKind::Local => "check the named file and its permissions",
        ErrorKind::Backend => "the backend did not answer; retry if allowed",
        ErrorKind::Cancelled | ErrorKind::Deadline | ErrorKind::Defect => return None,
    };
    Some(
        serde_json::json!({"status":"failed","error":{"kind":error.kind().name(),"message":message,"retryable":error.retryable()}}),
    )
}

/// An engine error as PostgreSQL reports it: a spent process send budget
/// names `thinkthen.max_requests_total`.
fn reported(error: Error) -> Error {
    if matches!(
        error.send_budget_denial(),
        Some(
            SendBudgetDenial::BeforeFirstSend
                | SendBudgetDenial::BeforeAdditionalSend
                | SendBudgetDenial::BeforeRetry { .. }
        )
    ) {
        return spent(settings::current_total());
    }
    error
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
pub(crate) fn raise(error: Error) -> ! {
    ereport!(ERROR, sqlstate(error.kind()), text(&error));
}

/// Run one SQL function's body on the backend thread. pgrx raises every SQL
/// error by panicking with its own payload types, and a PostgreSQL error
/// comes back as a `CaughtError`, so those pass through untouched. Any other
/// panic is forgotten without running its destructor, as
/// `thinkthen::contained` does, and raises the fixed defect. Its payload
/// reaches neither the client nor the server log.
pub(crate) fn guarded<T>(body: impl FnOnce() -> T) -> T {
    match std::panic::catch_unwind(AssertUnwindSafe(body)) {
        Ok(value) => value,
        Err(payload) if pgrx_raised(&*payload) => std::panic::resume_unwind(payload),
        Err(payload) => {
            std::mem::forget(payload);
            raise(defect("the extension panicked"))
        }
    }
}

/// Whether a payload is one pgrx raises and reports itself. A rethrown
/// `RustPanic` carries a panic's payload, so it is not.
fn pgrx_raised(payload: &(dyn Any + Send)) -> bool {
    match payload.downcast_ref::<CaughtError>() {
        Some(caught) => !matches!(caught, CaughtError::RustPanic { .. }),
        None => payload.is::<ErrorReportWithLevel>() || payload.is::<ErrorReport>(),
    }
}

/// Unwrap a value or raise its failure.
pub(crate) trait OrRaise<T> {
    fn or_raise(self) -> T;
}

impl<T> OrRaise<T> for Result<T, Error> {
    fn or_raise(self) -> T {
        self.unwrap_or_else(|error| raise(error))
    }
}

/// One engine per plan this backend used. Each engine counts its own sends,
/// so the usage totals add them all.
static ENGINES: Mutex<Vec<(Plan, Engine)>> = Mutex::new(Vec::new());

fn engines() -> std::sync::MutexGuard<'static, Vec<(Plan, Engine)>> {
    ENGINES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Build an engine from the server's environment and the plan. The engine
/// refuses a throttle that differs from the one this backend selected.
fn build(plan: &Plan) -> Result<Engine, Error> {
    let engine = settings::apply(plan, EngineBuilder::from_env()?.shared_host())?.build()?;
    // Kept before the call runs, so a cancelled call's sends still count.
    // A worker that lost the race uses the recorded engine, so every send counts.
    let mut all = engines();
    if let Some((_, held)) = all.iter().find(|(held, _)| held == plan) {
        return Ok(held.clone());
    }
    all.push((plan.clone(), engine.clone()));
    Ok(engine)
}

/// Flush every engine this backend built. A call can hold the list when
/// `FATAL` reaches `proc_exit`, so a busy list skips the flush rather than
/// waiting on a lock its holder never releases.
pub(crate) fn finish_usage() {
    let Ok(all) = ENGINES.try_lock() else {
        return;
    };
    let kept: Vec<Engine> = all.iter().map(|(_, engine)| engine.clone()).collect();
    drop(all);
    for engine in kept {
        engine.finish_usage();
    }
}

/// This backend's totals, summed over every engine it built.
pub(crate) fn totals() -> Counters {
    engines().iter().map(|(_, engine)| engine.usage()).sum()
}

/// Observe every engine built by this backend, without waiting on its registry.
pub(crate) fn usage_status() -> thinkthen::UsagePersistence {
    match ENGINES.try_lock() {
        Ok(all) => thinkthen::UsagePersistence::aggregate(
            all.iter().map(|(_, engine)| engine.usage_persistence()),
        ),
        Err(_) => thinkthen::UsagePersistence::Pending,
    }
}

/// What a call read on the backend thread before its worker starts.
#[derive(Debug)]
pub(crate) struct Call {
    pub(crate) plan: Plan,
    pub(crate) deadline_ms: i64,
    pub(crate) context: Option<String>,
    pub(crate) batch: Option<BatchSetting>,
    /// `thinkthen.max_requests_total`, which the engine's one process total
    /// enforces for each actual send.
    total: Option<u64>,
}

/// The refusal once `thinkthen.max_requests_total` is spent.
fn spent(total: u64) -> Error {
    usage(format!(
        "thinkthen.max_requests_total allows {total} requests in this backend, and they are spent"
    ))
}

impl Call {
    pub(crate) fn with_model(mut self, model: &str) -> Self {
        self.plan = self.plan.with_model(model);
        self
    }

    pub(crate) fn with_settings(mut self, settings: &Settings) -> Result<Self, Error> {
        if let Some(value) = settings.deadline_ms() {
            self.deadline_ms = value;
        }
        self.context = settings.context().map(str::to_owned);
        self.batch = if settings.batch_max() {
            Some(BatchSetting::Max)
        } else {
            settings
                .batch_records()
                .and_then(std::num::NonZeroUsize::new)
                .map(BatchSetting::Records)
        };
        Ok(self)
    }

    /// Refuse held records over `max_requests` before any send. The total
    /// belongs to transport attempts, so it cannot truncate record input.
    pub(crate) fn within(&self, count: usize) {
        record_limit(count, self.plan.max_requests).or_raise();
    }
}

pub(crate) fn record_limit(count: usize, most: Option<usize>) -> Result<(), Error> {
    if let Some(most) = most.filter(|most| count > *most) {
        return Err(usage(format!(
            "this engine answers at most {most} records in one call"
        )));
    }
    Ok(())
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
    // A panic sends nothing, so the caller reads `lost`, and its payload
    // reaches no server log.
    if let Some(value) = thinkthen::contained(work) {
        let _ignored = answer.send(value);
    }
}

/// A worker that ended without a result.
pub(crate) fn lost() -> Error {
    defect("the call's worker stopped without a result")
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
) -> Result<T, Error> {
    let held = engines()
        .iter()
        .find(|(plan, _)| *plan == call.plan)
        .map(|(_, engine)| engine.clone());
    if held.is_none() {
        ffi::flush_usage_at_exit();
    }
    let millis = call.deadline_ms;
    let due = u64::try_from(millis)
        .ok()
        .map(|budget| Instant::now() + Duration::from_millis(budget));
    let token = CancelToken::new();
    let (answer, answered) = mpsc::channel::<Result<T, Error>>();
    let (plan, worker_token) = (call.plan.clone(), token.clone());
    let total = call.total;
    let (call_batch, call_context) = (call.batch, call.context);
    ffi::spawn_masked(move || {
        deliver(&answer, || {
            let engine = match held {
                Some(engine) => engine,
                None => build(&plan)?,
            };
            let options = CallOptions::new()
                .cancel(&worker_token)
                .max_requests_total(total);
            let options = options.deadline_ms(millis)?;
            let options = call_batch.map_or(options, |batch| options.batch(batch));
            let options = call_context
                .as_deref()
                .map_or(options, |text| options.context(text));
            work(&engine, options)
        });
    })
    .map_err(|_| defect("the call's worker could not start"))
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
        Waited::Done(result) => result.map_err(reported),
        Waited::Lost => raise(lost()),
        Waited::Cancel => {
            token.cancel();
            ffi::raise_pending_interrupt();
            raise(Error::new(ErrorKind::Cancelled, "the call was cancelled"))
        }
        Waited::Deadline => {
            token.cancel();
            raise(Error::new(
                ErrorKind::Deadline,
                format!("the call's deadline of {millis} ms passed"),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                text(&Error::new(kind, "why")),
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

    const CHILD: &str = "THINKTHEN_TEST_POSTGRESQL_PANIC_CHILD";
    const MARKER: &str = "postgresql-worker-payload-marker";
    const HOST: &str = "postgresql-unrelated-host-marker";

    /// A worker panic reaches no output and reads as lost; the next call
    /// answers, and an unrelated panic still reaches the host's hook.
    #[test]
    fn a_worker_panic_stays_out_of_the_server_log() {
        if std::env::var_os(CHILD).is_some() {
            std::panic::set_hook(Box::new(|info| {
                let text = info.payload().downcast_ref::<&str>().copied();
                let _ = std::io::Write::write_all(
                    &mut std::io::stderr(),
                    format!("{}\n", text.unwrap_or("other panic")).as_bytes(),
                );
            }));
            let (answer, answered) = mpsc::channel::<u8>();
            let worker = std::thread::spawn(move || {
                deliver(&answer, || std::panic::panic_any(MARKER));
            });
            assert!(worker.join().is_ok());
            assert_eq!(wait(&answered, TICK, || None), Waited::Lost);
            let (answer, answered) = mpsc::channel();
            std::thread::spawn(move || deliver(&answer, || 7));
            assert_eq!(wait(&answered, TICK, || None), Waited::Done(7));
            let _ = std::thread::spawn(|| std::panic::panic_any(HOST)).join();
            return;
        }
        let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
            .env_clear()
            .args([
                "--exact",
                "call::tests::a_worker_panic_stays_out_of_the_server_log",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .output()
            .expect("isolated PostgreSQL proof");
        assert!(output.status.success());
        for stream in [&output.stdout, &output.stderr] {
            assert!(!String::from_utf8_lossy(stream).contains(MARKER));
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(&format!("{HOST}\n")), "{stderr}");
    }

    /// A channel that closes with no result reads as lost, which raises `defect`.
    #[test]
    fn a_channel_closed_without_a_result_is_a_defect() {
        let (answer, answered) = mpsc::channel::<u8>();
        std::thread::spawn(move || drop(answer));
        assert_eq!(wait(&answered, TICK, || None), Waited::Lost);
        assert_eq!(
            text(&lost()),
            "thinkthen defect: the call's worker stopped without a result (retryable: no)"
        );
    }
}
