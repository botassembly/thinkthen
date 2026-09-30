//! Call options, the cancel token, and the one door every public call passes.

pub(crate) mod budget;
mod observer;

pub use budget::{EstimatedInputDenial, SendBudget, SendBudgetDenial};
pub(crate) use budget::{EstimatedReservation, SendReservation};

use std::any::Any;
use std::fmt;
use std::num::NonZeroUsize;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, sync_channel};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::core::Prices;
use crate::engine::{AttemptSink, CallFacts, Cancel, Deadline, workers};
use crate::public::error::Error;
use crate::public::results::{AttemptObservation, Call, Facts, RecordObservation};

type Observer<'a> = &'a (dyn for<'r> Fn(RecordObservation<'r>) + Send + Sync);
type AttemptObserver<'a> = &'a (dyn Fn(AttemptObservation) + Send + Sync);

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

fn shown(value: f64) -> String {
    let plain = value.to_string();
    if plain.len() <= 20 {
        return plain;
    }
    format!("{value:e}")
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
    observer: Option<Observer<'a>>,
    attempt_observer: Option<AttemptObserver<'a>>,
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
            .field("observer", &self.observer.is_some())
            .field("attempt_observer", &self.attempt_observer.is_some())
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
            observer: None,
            attempt_observer: None,
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

    /// Observe each completed logical question and row on this caller thread.
    /// Borrowed detail remains valid only during the callback.
    #[must_use]
    pub const fn observe(
        mut self,
        observer: &'a (dyn for<'r> Fn(RecordObservation<'r>) + Send + Sync),
    ) -> Self {
        self.observer = Some(observer);
        self
    }

    /// Observe each live HTTP attempt on this calling thread with owned data.
    /// Cache and replay produce no current attempt observation.
    #[must_use]
    pub const fn observe_attempt(
        mut self,
        observer: &'a (dyn Fn(AttemptObservation) + Send + Sync),
    ) -> Self {
        self.attempt_observer = Some(observer);
        self
    }

    pub(crate) const fn batch_setting(&self) -> Option<BatchSetting> {
        self.batch
    }

    pub(crate) const fn context_text(&self) -> Option<&'a str> {
        self.context
    }

    pub(crate) fn without_context(&self, call: &str) -> Result<(), Error> {
        self.context.map_or(Ok(()), |_| {
            Err(Error::usage(format!(
                "{call} does not take a shared context"
            )))
        })
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

    /// Seconds since call start; `-1` clears and `0` sends nothing.
    /// Retained for host source migration to `deadline_ms`.
    ///
    /// # Errors
    /// Refuses another negative value, NaN, infinity, or an excessive budget.
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

    /// Milliseconds since call start; `-1` clears and `0` sends nothing.
    ///
    /// # Errors
    /// Refuses another negative value or more than 4,294,967,295 seconds.
    pub fn deadline_ms(self, value: i64) -> Result<Self, Error> {
        if value == -1 {
            return Ok(self.cleared());
        }
        let budget = u64::try_from(value)
            .ok()
            .map(Duration::from_millis)
            .filter(|_| crate::core::settings::valid_deadline_ms(value))
            .ok_or_else(|| {
                Error::usage(format!(
                    "a deadline of {value} milliseconds is not -1, 0, or a positive budget of at most {MOST_SECONDS} seconds"
                ))
            })?;
        self.deadline_after(budget)
    }

    /// Milliseconds retained for host source migration to `deadline_ms`.
    ///
    /// # Errors
    /// As [`CallOptions::deadline_ms`].
    pub fn deadline_millis(self, value: i64) -> Result<Self, Error> {
        self.deadline_ms(value)
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
    prices: Option<Prices>,
    token: Option<&'a CancelToken>,
    check: Option<&'a (dyn Fn() -> bool + Sync)>,
    observer: Option<Observer<'a>>,
    attempt_observer: Option<AttemptObserver<'a>>,
    attempts: Option<Mutex<Receiver<AttemptObservation>>>,
    panic: Mutex<Option<Box<dyn Any + Send>>>,
}

impl<'a> Stop<'a> {
    /// Fix the deadline and refuse a call whose token already fired.
    pub(crate) fn begin(options: CallOptions<'a>) -> Result<Self, Error> {
        let facts = CallFacts::new();
        let (sender, attempts) = if options.attempt_observer.is_some() {
            let (sender, receiver) = sync_channel(32);
            (Some(sender), Some(Mutex::new(receiver)))
        } else {
            (None, None)
        };
        let mut base = Cancel::default()
            .with_deadline(options.deadline()?)
            .with_token(options.cancel.map(CancelToken::flag))
            .with_send_budget(
                options
                    .send_budget
                    .map(|(budget, limit)| (budget.clone(), limit)),
            )
            .with_facts(facts.clone());
        if let Some(sender) = sender {
            base = base.with_attempt_sink(AttemptSink::new(move |event| {
                let _sent = sender.send(event);
            }));
        }
        let stop = Self {
            base,
            facts,
            prices: None,
            token: options.cancel,
            check: options.check,
            observer: options.observer,
            attempt_observer: options.attempt_observer,
            attempts,
            panic: Mutex::new(None),
        };
        if stop.token.is_some_and(CancelToken::is_cancelled) {
            return Err(Error::cancelled());
        }
        Ok(stop)
    }

    pub(crate) fn with_prices(mut self, prices: Option<Prices>) -> Self {
        self.prices = prices;
        self
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
        self.drain_attempts();
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

    /// Run an eager call, then retain its final receipts on success or failure.
    pub(crate) fn run_call<T>(
        &self,
        records: usize,
        call: impl FnOnce(&Cancel<'_>) -> Result<T, Error>,
    ) -> Result<Call<T>, Error> {
        let result = self.run(call);
        if result.is_ok() {
            self.facts.finished_records(records);
        }
        let facts = Facts::of(self.facts.snapshot(), self.prices);
        result
            .map(|value| Call::new(value, facts.clone()))
            .map_err(|error| error.with_facts(facts))
    }

    /// Resume a check's panic, once every worker of the call has joined, then
    /// return the call's result, or cancellation when the token has fired.
    pub(crate) fn finish<T>(&self, result: Result<T, Error>) -> Result<T, Error> {
        self.drain_attempts();
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

    pub(crate) fn facts(&self) -> Facts {
        Facts::of(self.facts.snapshot(), self.prices)
    }
}

/// Run one engine call and turn any panic below the door into a defect.
pub(crate) fn guarded<T>(call: impl FnOnce() -> Result<T, Error>) -> Result<T, Error> {
    contained(call)
        .unwrap_or_else(|| Err(Error::defect("the engine panicked below the public door")))
}

/// Run `body` and return `None` if it panics, with no trace of the panic.
///
/// This is the one panic guard every binding boundary uses. A panic inside
/// `body` on this thread skips the host's panic hook, so its message reaches
/// no output. The payload is forgotten inside the guard without running its
/// destructor, so a payload whose drop panics again stays silent too. A panic
/// on any other thread still reaches the host's hook. The first call installs
/// one process hook that keeps the previous hook for everything else.
pub fn contained<T>(body: impl FnOnce() -> T) -> Option<T> {
    catch_unwind(AssertUnwindSafe(|| {
        workers::with_engine_diagnostics(|| match catch_unwind(AssertUnwindSafe(body)) {
            Ok(value) => Some(value),
            Err(payload) => {
                std::mem::forget(payload);
                None
            }
        })
    }))
    .unwrap_or_else(|payload| {
        std::mem::forget(payload);
        None
    })
}

/// Run host code inside [`contained`] work under the host's own panic hook.
///
/// Use it for a host callback, or to drop a host-owned value, so the host's
/// diagnostics still report the host's own panics.
pub fn uncontained<T>(body: impl FnOnce() -> T) -> T {
    workers::with_host_diagnostics(body)
}

impl fmt::Debug for Stop<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Stop").finish_non_exhaustive()
    }
}

/// The private child exercises diagnostic and worker boundaries that no
/// ordinary input can force. The public API has no fault hook.
#[cfg(test)]
mod tests;
