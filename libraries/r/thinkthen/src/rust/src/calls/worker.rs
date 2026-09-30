//! The R caller's interrupt-safe worker and one owned completion path.

use std::sync::Arc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::thread;
use std::time::{Duration, Instant};

use thinkthen::{CallOptions, CancelToken, Engine, Error};

use super::account::{Account, Completed};
use super::receipt::Receipt;
use crate::{carry, defect, engine, interrupted, usage};

/// How long the main thread waits between R's interrupt checks.
const TICK: Duration = Duration::from_millis(100);

/// R's guarded interrupt check, run on the main thread.
pub(crate) type Pending<'a> = &'a dyn Fn() -> bool;

/// A shim result: R's value, or the packed failure.
pub(crate) type Crossed<T> = Result<T, String>;

/// Cancels the call's token on every way out of the wait, so a detached
/// worker starts no new request after its caller has left.
pub(super) struct Stop(CancelToken);

impl Drop for Stop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

fn call_with_receipt<T: Send + 'static>(
    deadline: Option<f64>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
    work: impl FnOnce(&Engine, CallOptions<'_>) -> Result<T, Error> + Send + 'static,
) -> Crossed<T> {
    if pending() {
        return Err(interrupted());
    }
    let due = due(deadline)?;
    let engine = engine()?;
    let token = CancelToken::new();
    let held = token.clone();
    on_worker_with_receipt(token, pending, receipt, move || {
        let options = CallOptions::new().cancel(&held);
        let options = due.map_or(options, |at| options.deadline_at(at));
        work(&engine, options).map_err(|error| carry(&error))
    })
}

/// One worker result keeps its complete account even when the Rust call failed.
/// The receipt owns no R object and can outlive the caller's prompt interrupt.
pub(crate) fn call_owned<T: Send + 'static>(
    deadline: Option<f64>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
    positions: Option<Vec<usize>>,
    work: impl FnOnce(&Engine, CallOptions<'_>, &Account) -> Result<T, String> + Send + 'static,
) -> Crossed<Completed<T>> {
    call_with_receipt(
        deadline,
        pending,
        receipt.clone(),
        move |engine, options| {
            let account = Account::new(positions);
            let observer = |event: thinkthen::RecordObservation<'_>| account.observe(event, None);
            let result = work(engine, options.observe(&observer), &account);
            let snapshot = account.finish();
            if let Some(held) = &receipt {
                let kind = result.as_ref().map_or_else(
                    |error| error.split('\u{1f}').next().unwrap_or("defect").to_owned(),
                    |_| "success".to_owned(),
                );
                held.settle(kind, snapshot.clone());
            }
            Ok(Completed { result, snapshot })
        },
    )
}

/// The deadline as one instant, fixed before the call starts, so every
/// engine call a verb makes shares it. The host boundary uses whole
/// milliseconds, with `-1` for none.
fn due(deadline: Option<f64>) -> Crossed<Option<Instant>> {
    let Some(milliseconds) = deadline else {
        return Ok(None);
    };
    if milliseconds == -1.0 {
        return Ok(None);
    }
    if !milliseconds.is_finite()
        || milliseconds.fract() != 0.0
        || !(0.0..=4_294_967_295_000.0).contains(&milliseconds)
    {
        return Err(usage(
            "deadline_ms is -1, 0, or at most 4294967295000 milliseconds",
        ));
    }
    let late = || {
        usage(&format!(
            "a deadline of {milliseconds} milliseconds does not fit this clock"
        ))
    };
    let budget = Duration::try_from_secs_f64(milliseconds / 1000.0).map_err(|_| late())?;
    Instant::now()
        .checked_add(budget)
        .map(Some)
        .ok_or_else(late)
}

/// Run `body` on a new thread and wait for it in ticks.
#[cfg(test)]
pub(crate) fn on_worker<T: Send + 'static>(
    token: CancelToken,
    pending: Pending<'_>,
    body: impl FnOnce() -> Crossed<T> + Send + 'static,
) -> Crossed<T> {
    on_worker_with_receipt(token, pending, None, body)
}

fn on_worker_with_receipt<T: Send + 'static>(
    token: CancelToken,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
    body: impl FnOnce() -> Crossed<T> + Send + 'static,
) -> Crossed<T> {
    let (sender, receiver) = channel();
    let worker_receipt = receipt.clone();
    thread::Builder::new()
        .name("thinkthen-r".to_owned())
        .spawn(move || {
            let answer = thinkthen::contained(body).unwrap_or_else(|| {
                if let Some(held) = &worker_receipt {
                    held.settle("defect".to_owned(), super::account::Snapshot::empty());
                }
                Err(defect("the call panicked"))
            });
            // The caller left after an interrupt when this send fails.
            let _ignored = sender.send(answer);
        })
        .map_err(|error| {
            if let Some(held) = &receipt {
                held.settle_early("defect".to_owned());
            }
            defect(&format!("the call's worker did not start: {error}"))
        })?;
    if let Some(held) = &receipt {
        held.running();
    }
    wait(&receiver, &Stop(token), pending)
}

/// The main thread's wait. The stop guard cancels on every return.
pub(super) fn wait<T>(
    receiver: &Receiver<Crossed<T>>,
    _stop: &Stop,
    pending: Pending<'_>,
) -> Crossed<T> {
    loop {
        match receiver.recv_timeout(TICK) {
            Ok(answer) => return answer,
            Err(RecvTimeoutError::Timeout) if pending() => return Err(interrupted()),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                return Err(defect("the call's worker ended without an answer"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    static PANICS: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn each_worker_edge_answers_without_a_stray_panic() {
        std::panic::set_hook(Box::new(|_| {
            PANICS.fetch_add(1, Ordering::SeqCst);
        }));
        let ticks = AtomicUsize::new(0);
        let left = on_worker(
            CancelToken::new(),
            &|| ticks.fetch_add(1, Ordering::SeqCst) > 0,
            || {
                thread::sleep(Duration::from_millis(400));
                Ok(1)
            },
        );
        assert_eq!(left, Err(interrupted()));
        thread::sleep(Duration::from_millis(500));
        assert_eq!(
            PANICS.load(Ordering::SeqCst),
            0,
            "the worker's failed send panicked"
        );

        let closed = channel::<Crossed<i32>>();
        drop(closed.0);
        let answer = wait(&closed.1, &Stop(CancelToken::new()), &|| false);
        assert_eq!(
            answer,
            Err(defect("the call's worker ended without an answer"))
        );

        let boom: Crossed<i32> = on_worker(CancelToken::new(), &|| false, || {
            std::panic::panic_any("boom")
        });
        assert_eq!(boom, Err(defect("the call panicked")));
        assert_eq!(on_worker(CancelToken::new(), &|| false, || Ok(7)), Ok(7));
        let _ = std::panic::take_hook();
    }

    #[test]
    fn the_stop_guard_cancels_the_token_on_an_interrupt() {
        let token = CancelToken::new();
        let seen = token.clone();
        let left = on_worker(token, &|| true, || {
            thread::sleep(Duration::from_millis(300));
            Ok(())
        });
        assert_eq!(left, Err(interrupted()));
        assert!(seen.is_cancelled());
    }
}
