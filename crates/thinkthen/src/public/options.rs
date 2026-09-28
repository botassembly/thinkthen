//! Call options, the cancel token, and the one door every public call passes.

mod budget;

pub(crate) use budget::SendReservation;
pub use budget::{SendBudget, SendBudgetDenial};

use std::any::Any;
use std::fmt;
use std::num::NonZeroUsize;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::engine::{CallFacts, Cancel, Deadline, workers};
use crate::public::error::Error;

/// How many records one model request may contain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchSetting {
    /// Fill each request to its applicable limits.
    Max,
    /// Close a request after this many records at most.
    Records(NonZeroUsize),
}

impl From<BatchSetting> for crate::core::batch::Setting {
    fn from(value: BatchSetting) -> Self {
        match value {
            BatchSetting::Max => Self::Max,
            BatchSetting::Records(count) => Self::Records(count),
        }
    }
}

/// The largest budget a deadline takes: 4,294,967,295 seconds (ADR 0041).
const MOST_SECONDS: u64 = 4_294_967_295;

/// A number plain up to 20 characters, the width of `u64::MAX`, else as `1e300`.
fn shown(value: f64) -> String {
    let plain = value.to_string();
    if plain.len() <= 20 {
        plain
    } else {
        format!("{value:e}")
    }
}

/// A cancel flag a caller may set from any thread.
///
/// Every clone shares one flag. A call that carries it starts no request or
/// retry after the fire, lets sent attempts finish, and returns
/// [`Error::Cancelled`] whatever those attempts answered. A batch ends with
/// that error after the rows it already yielded.
#[derive(Clone, Debug, Default)]
pub struct CancelToken(Arc<AtomicBool>);

impl CancelToken {
    /// A token that has not fired.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Fire the token for every call that carries it.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }

    /// The shared flag, which the engine reads on every thread of a call.
    fn flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.0)
    }

    /// Whether the token has fired.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}

/// When a call's deadline falls, fixed as an instant when the call begins.
#[derive(Clone, Copy, Debug)]
enum Due {
    At(Instant),
    After(Duration),
}

/// The caller controls one call carries: a cancel token, a deadline, and an
/// interrupt check.
///
/// An interrupt check runs only on the calling thread: once before the first
/// send and at each 50 ms poll while the call waits, never during one
/// blocking send. A `true` return cancels this call alone. A panic in the
/// check stops the call, joins its workers, and then resumes on the caller.
#[derive(Clone, Copy, Default)]
pub struct CallOptions<'a> {
    cancel: Option<&'a CancelToken>,
    due: Option<Due>,
    check: Option<&'a (dyn Fn() -> bool + Sync)>,
    send_budget: Option<(&'a SendBudget, Option<u64>)>,
    batch: Option<BatchSetting>,
    context: Option<&'a str>,
}

impl fmt::Debug for CallOptions<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CallOptions")
            .field("cancel", &self.cancel)
            .field("deadline", &self.due)
            .field("interrupt", &self.check.is_some())
            .field("send_budget", &self.send_budget.is_some())
            .field("batch", &self.batch)
            .field("context", &self.context.is_some())
            .finish()
    }
}

impl<'a> CallOptions<'a> {
    /// No token, no deadline, and no check.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cancel: None,
            due: None,
            check: None,
            send_budget: None,
            batch: None,
            context: None,
        }
    }

    /// Stop the call when this token fires.
    #[must_use]
    pub const fn cancel(mut self, value: &'a CancelToken) -> Self {
        self.cancel = Some(value);
        self
    }

    /// Share a process send counter and apply this call's optional total.
    /// A cache or strict replay answer uses no reservation.
    #[must_use]
    pub const fn send_budget(mut self, value: &'a SendBudget, limit: Option<u64>) -> Self {
        self.send_budget = Some((value, limit));
        self
    }

    /// Select the number of records one eligible many-record request holds.
    #[must_use]
    pub const fn batch(mut self, setting: BatchSetting) -> Self {
        self.batch = Some(setting);
        self
    }

    /// Share nonblank text as context for an eligible many-record request.
    #[must_use]
    pub const fn context(mut self, value: &'a str) -> Self {
        self.context = Some(value);
        self
    }

    pub(crate) const fn batch_setting(&self) -> Option<BatchSetting> {
        self.batch
    }

    pub(crate) const fn context_text(&self) -> Option<&'a str> {
        self.context
    }

    /// Stop the call at this instant. A past instant sends nothing.
    #[must_use]
    pub fn deadline_at(mut self, value: Instant) -> Self {
        self.due = Some(Due::At(value));
        self
    }

    /// Stop the call this long after it begins.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a budget above 4,294,967,295 seconds.
    pub fn deadline_after(mut self, value: Duration) -> Result<Self, Error> {
        if value > Duration::from_secs(MOST_SECONDS) {
            return Err(Error::usage(format!(
                "a deadline of {} seconds is above the most, {MOST_SECONDS} seconds",
                value.as_secs()
            )));
        }
        self.due = Some(Due::After(value));
        Ok(self)
    }

    /// Stop the call this many seconds after it begins: `-1` is no deadline
    /// and clears an earlier one, and `0` is spent, so the call sends nothing.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for any other negative, NaN, infinity, or a
    /// budget above 4,294,967,295 seconds.
    pub fn deadline_seconds(self, value: f64) -> Result<Self, Error> {
        if value == -1.0 {
            return Ok(self.cleared());
        }
        let refused = || {
            Error::usage(format!(
                "a deadline of {} seconds is not -1, 0, or a positive budget of at most {MOST_SECONDS} seconds",
                shown(value)
            ))
        };
        if !value.is_finite() || value < 0.0 {
            return Err(refused());
        }
        let budget = Duration::try_from_secs_f64(value).map_err(|_| refused())?;
        self.deadline_after(budget).map_err(|_| refused())
    }

    /// Stop the call this many milliseconds after it begins, under the rules
    /// of [`CallOptions::deadline_seconds`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a negative value other than `-1`, or a
    /// budget above 4,294,967,295 seconds.
    pub fn deadline_millis(self, value: i64) -> Result<Self, Error> {
        if value == -1 {
            return Ok(self.cleared());
        }
        let budget = u64::try_from(value)
            .ok()
            .map(Duration::from_millis)
            .filter(|budget| *budget <= Duration::from_secs(MOST_SECONDS))
            .ok_or_else(|| {
                Error::usage(format!(
                    "a deadline of {value} milliseconds is not -1, 0, or a positive budget of at most {MOST_SECONDS} seconds"
                ))
            })?;
        self.deadline_after(budget)
    }

    /// Run this check on the calling thread while the call waits.
    #[must_use]
    pub const fn interrupt(mut self, check: &'a (dyn Fn() -> bool + Sync)) -> Self {
        self.check = Some(check);
        self
    }

    const fn cleared(mut self) -> Self {
        self.due = None;
        self
    }

    /// The deadline fixed now, as the call begins.
    fn deadline(&self) -> Result<Option<Deadline>, Error> {
        let budget = match self.due {
            None => return Ok(None),
            Some(Due::After(budget)) => budget,
            Some(Due::At(at)) => at.saturating_duration_since(Instant::now()),
        };
        Deadline::after(budget)
            .map(Some)
            .ok_or_else(|| Error::usage("the clock cannot name a deadline this far away"))
    }
}

/// The per-call stop state: the call's own flag and deadline, the caller's
/// token and check, and a check's panic held until the call has joined.
pub(crate) struct Stop<'a> {
    base: Cancel<'static>,
    facts: CallFacts,
    token: Option<&'a CancelToken>,
    check: Option<&'a (dyn Fn() -> bool + Sync)>,
    panic: Mutex<Option<Box<dyn Any + Send>>>,
}

impl<'a> Stop<'a> {
    /// Fix the deadline and refuse a call whose token already fired.
    pub(crate) fn begin(options: CallOptions<'a>) -> Result<Self, Error> {
        let facts = CallFacts::new();
        let stop = Self {
            base: Cancel::default()
                .with_deadline(options.deadline()?)
                .with_token(options.cancel.map(CancelToken::flag))
                .with_send_budget(
                    options
                        .send_budget
                        .map(|(budget, limit)| (budget.clone(), limit)),
                )
                .with_facts(facts.clone()),
            facts,
            token: options.cancel,
            check: options.check,
            panic: Mutex::new(None),
        };
        if stop.token.is_some_and(CancelToken::is_cancelled) {
            return Err(Error::cancelled());
        }
        Ok(stop)
    }

    /// This call's flag, deadline and the caller's token, without the check, for
    /// engine threads.
    pub(crate) fn shared(&self) -> Cancel<'static> {
        self.base.clone()
    }

    /// Fire this call's own flag, which the caller's token never sees.
    pub(crate) fn fire(&self) {
        self.base.flag().store(true, Ordering::Release);
    }

    /// Read the caller's token, then the caller's check. A check that panics
    /// reads as `true`, and its payload waits in [`Stop::finish`].
    pub(crate) fn interrupted(&self) -> bool {
        if self.token.is_some_and(CancelToken::is_cancelled) {
            return true;
        }
        let Some(check) = self.check else {
            return false;
        };
        workers::with_host_diagnostics(|| catch_unwind(AssertUnwindSafe(check))).unwrap_or_else(
            |payload| {
                if let Ok(mut held) = self.panic.lock() {
                    held.get_or_insert(payload);
                }
                true
            },
        )
    }

    /// Run one engine call on this thread with the caller's controls polled
    /// here. An engine panic becomes [`Error::Defect`]; a check's panic
    /// resumes on the caller after the call has joined.
    pub(crate) fn run<T>(
        &self,
        call: impl FnOnce(&Cancel<'_>) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let polled = || self.interrupted();
        let cancel = self.base.with_check(&polled);
        let result = guarded(|| call(&cancel));
        self.finish(result)
    }

    /// Resume a check's panic, once every worker of the call has joined, then
    /// return the call's result, or cancellation when the token has fired.
    pub(crate) fn finish<T>(&self, result: Result<T, Error>) -> Result<T, Error> {
        self.facts.finish();
        let held = self.panic.lock().ok().and_then(|mut held| held.take());
        if let Some(payload) = held {
            resume_unwind(payload);
        }
        if self.token.is_some_and(CancelToken::is_cancelled) {
            return Err(Error::cancelled());
        }
        result
    }
}

/// Run one engine call and turn any panic below the door into a defect.
pub(crate) fn guarded<T>(call: impl FnOnce() -> Result<T, Error>) -> Result<T, Error> {
    catch_unwind(AssertUnwindSafe(|| workers::with_engine_diagnostics(call)))
        .unwrap_or_else(|_| Err(Error::defect("the engine panicked below the public door")))
}

impl fmt::Debug for Stop<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Stop").finish_non_exhaustive()
    }
}

/// The private child exercises diagnostic and worker boundaries that no
/// ordinary input can force. The public API has no fault hook.
#[cfg(test)]
mod tests {
    use std::panic::{AssertUnwindSafe, catch_unwind, panic_any};
    use std::process::Command;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc;

    use super::{CallOptions, Stop, guarded};
    use crate::engine::{Cancel, workers};
    use crate::public::error::{Error, ErrorKind};

    fn wait_for_host_check(held: mpsc::Receiver<()>, joined: Arc<AtomicBool>) {
        held.recv().expect("host check releases worker");
        joined.store(true, Ordering::Release);
    }

    #[test]
    fn diagnostic_boundary_child() {
        if std::env::var_os("THINKTHEN_TEST_DIAGNOSTIC_CHILD").is_none() {
            return;
        }
        std::panic::set_hook(Box::new(|info| {
            let message = info
                .payload()
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
                .unwrap_or("unknown panic");
            let _written = std::io::Write::write_all(
                &mut std::io::stderr(),
                format!("prior hook: {message}\n").as_bytes(),
            );
        }));

        let worker: Result<(), Error> = guarded(|| {
            workers::on_worker(&Cancel::default(), || {
                panic_any("worker key evidence secret")
            });
            Ok(())
        });
        let error = worker.expect_err("worker panic becomes a defect");
        assert_eq!(error.kind(), ErrorKind::Defect);
        assert!(!error.retryable());
        assert_eq!(
            error.to_string(),
            "defect: the engine panicked below the public door"
        );

        let scoped: Result<(), Error> = guarded(|| {
            let (results, _received) = mpsc::channel::<()>();
            workers::scoped_observed(
                1,
                results,
                &|_: ()| (),
                &|| panic_any("scoped key evidence secret"),
                |queue| {
                    let _sent = queue.send(());
                },
            );
            Ok(())
        });
        let error = scoped.expect_err("scoped worker panic becomes a defect");
        assert_eq!(error.kind(), ErrorKind::Defect);
        assert!(!error.retryable());

        let (release, held) = mpsc::channel();
        let joined = Arc::new(AtomicBool::new(false));
        let check = || -> bool {
            let _sent = release.send(());
            panic_any("host stop marker");
        };
        let stop = Stop::begin(CallOptions::new().interrupt(&check)).expect("valid stop");
        let joined_worker = Arc::clone(&joined);
        let resumed = catch_unwind(AssertUnwindSafe(|| {
            let _: Result<(), Error> = stop.run(|cancel| {
                workers::on_worker(cancel, move || wait_for_host_check(held, joined_worker));
                Ok(())
            });
        }))
        .expect_err("host check panic resumes");
        assert_eq!(resumed.downcast_ref::<&str>(), Some(&"host stop marker"));
        assert!(joined.load(Ordering::Acquire));

        let (release, held) = mpsc::channel();
        let joined = Arc::new(AtomicBool::new(false));
        let check = || -> bool {
            let _sent = release.send(());
            panic_any("host cancel marker");
        };
        let cancel = Cancel::default().with_check(&check);
        let joined_worker = Arc::clone(&joined);
        let resumed = catch_unwind(AssertUnwindSafe(|| {
            workers::with_engine_diagnostics(|| {
                workers::on_worker(&cancel, move || wait_for_host_check(held, joined_worker));
            });
        }))
        .expect_err("direct host check panic resumes");
        assert_eq!(resumed.downcast_ref::<&str>(), Some(&"host cancel marker"));
        assert!(joined.load(Ordering::Acquire));

        let _unrelated = std::thread::spawn(|| panic_any("unrelated host marker")).join();
        assert_eq!(guarded(|| Ok(7)).ok(), Some(7));
    }

    #[test]
    fn engine_diagnostics_hide_worker_payloads_and_preserve_host_hook() {
        let output = Command::new(std::env::current_exe().expect("test binary"))
            .args([
                "--exact",
                "public::options::tests::diagnostic_boundary_child",
                "--nocapture",
            ])
            .env("THINKTHEN_TEST_DIAGNOSTIC_CHILD", "1")
            .output()
            .expect("run diagnostic child");
        assert!(
            output.status.success(),
            "diagnostic child failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        for stream in [&*stdout, &*stderr] {
            assert!(!stream.contains("worker key evidence secret"));
            assert!(!stream.contains("scoped key evidence secret"));
        }
        assert!(stderr.contains("prior hook: host stop marker"));
        assert!(stderr.contains("prior hook: host cancel marker"));
        assert!(stderr.contains("prior hook: unrelated host marker"));
    }
}
