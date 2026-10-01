//! Scoped request workers shared by the schedulers and the single calls.

use std::cell::Cell;
use std::sync::Once;
use std::thread::{self, ScopedJoinHandle};

use crate::engine::Cancel;
use crate::engine::fork_safe::{Receiver, RecvTimeoutError, Sender, channel};

thread_local! {
    /// Whether this thread is an engine worker with host signals masked.
    static ENGINE_WORKER: Cell<bool> = const { Cell::new(false) };
    /// Suppress panic diagnostics only while ThinkThen owns this thread.
    static DIAGNOSTIC_DEPTH: Cell<usize> = const { Cell::new(0) };
}

static DIAGNOSTIC_HOOK: Once = Once::new();

/// Keep the host's hook for unrelated threads, including after this call.
fn install_diagnostic_hook() {
    DIAGNOSTIC_HOOK.call_once(|| {
        let prior = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let owned = DIAGNOSTIC_DEPTH
                .try_with(|depth| depth.get() != 0)
                .unwrap_or(false);
            if !owned {
                prior(info);
            }
        }));
    });
}

/// Restore the prior depth even if a nested call unwinds.
struct DiagnosticDepth(usize);

impl Drop for DiagnosticDepth {
    fn drop(&mut self) {
        let _ = DIAGNOSTIC_DEPTH.try_with(|depth| depth.set(self.0));
    }
}

/// Mark work owned by the engine on this thread, including worker threads.
/// A thread past its local-storage teardown runs `work` unmarked.
pub(crate) fn with_engine_diagnostics<T>(work: impl FnOnce() -> T) -> T {
    install_diagnostic_hook();
    let prior = DIAGNOSTIC_DEPTH.try_with(|depth| {
        let prior = depth.get();
        depth.set(prior.saturating_add(1));
        prior
    });
    let _restore = prior.ok().map(DiagnosticDepth);
    work()
}

/// Let a host interrupt callback use the previous hook on direct engine calls.
pub(crate) fn with_host_diagnostics<T>(check: impl FnOnce() -> T) -> T {
    let prior = DIAGNOSTIC_DEPTH.try_with(|depth| depth.replace(0));
    let _restore = prior.ok().map(DiagnosticDepth);
    check()
}

/// Run one live attempt on an engine worker: this thread when it is one, or
/// else one scoped worker joined before return.
///
/// The calling thread polls the call's stop meanwhile, so the host check runs
/// there during a width gate or retry wait and never during a blocking send.
pub(crate) fn on_worker<T: Send>(cancel: &Cancel<'_>, send: impl FnOnce() -> T + Send) -> T {
    if ENGINE_WORKER.get() {
        return with_engine_diagnostics(send);
    }
    let (done, finished) = channel();
    thread::scope(|scope| {
        let worker = scope.spawn(move || {
            with_engine_diagnostics(|| {
                enter();
                let sent = send();
                let _caller_waits = done.send(());
                sent
            })
        });
        // A host check that panics still joins the worker first, so the scope
        // never parks the calling thread (ticket 0365).
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            while let Err(RecvTimeoutError::Timeout) = finished.recv_timeout(Cancel::poll()) {
                let _stop = cancel.stop_between_sends();
            }
        }));
        let sent = worker.join();
        if let Err(panic) = polled {
            std::panic::resume_unwind(panic);
        }
        sent.unwrap_or_else(|panic| std::panic::resume_unwind(panic))
    })
}

/// Mark this thread an engine worker and mask host signals for its lifetime.
fn enter() {
    mask_host_signals();
    ENGINE_WORKER.set(true);
}

/// Run a bounded group of workers around one scheduler body.
///
/// Every worker is joined before this function returns, including when the
/// scheduler body stops early after a failure or a closed output pipe.
pub(crate) fn scoped<W, R, T>(
    jobs: usize,
    results: Sender<R>,
    work: &(impl Fn(W) -> R + Sync),
    body: impl FnOnce(Sender<W>) -> T,
) -> T
where
    W: Send,
    R: Send,
{
    scoped_observed(jobs, results, work, &|| (), body)
}

pub(crate) fn scoped_observed<W, R, T, G>(
    jobs: usize,
    results: Sender<R>,
    work: &(impl Fn(W) -> R + Sync),
    begin: &(impl Fn() -> G + Sync),
    body: impl FnOnce(Sender<W>) -> T,
) -> T
where
    W: Send,
    R: Send,
    G: Send,
{
    let (send, queue) = channel();
    thread::scope(|scope| {
        let workers = Joined(
            (0..jobs)
                .map(|_| {
                    let results = results.clone();
                    let queue = &queue;
                    scope.spawn(move || observed_worker(queue, &results, work, begin))
                })
                .collect(),
        );
        drop(results);
        let value = body(send);
        workers.join();
        value
    })
}

/// Scoped workers joined before their scope closes, also while the body
/// unwinds. A scope that still has running threads parks the calling thread,
/// which a forked child on macOS cannot do (ticket 0365).
struct Joined<'scope>(Vec<ScopedJoinHandle<'scope, ()>>);

impl Joined<'_> {
    /// Join every worker, then resume the first worker's panic.
    fn join(mut self) {
        let mut panicked = None;
        for worker in self.0.drain(..) {
            if let Err(panic) = worker.join() {
                panicked.get_or_insert(panic);
            }
        }
        if let Some(panic) = panicked {
            std::panic::resume_unwind(panic);
        }
    }
}

impl Drop for Joined<'_> {
    fn drop(&mut self) {
        for worker in self.0.drain(..) {
            let _joined = worker.join();
        }
    }
}

fn observed_worker<W, R, G>(
    queue: &Receiver<W>,
    results: &Sender<R>,
    work: &(impl Fn(W) -> R + Sync),
    begin: &(impl Fn() -> G + Sync),
) {
    with_engine_diagnostics(|| {
        enter();
        let _lifetime = begin();
        worker(queue, results, work);
    });
}

fn worker<W, R>(queue: &Receiver<W>, results: &Sender<R>, work: &(impl Fn(W) -> R + Sync)) {
    while let Ok(item) = queue.recv() {
        if results.send(work(item)).is_err() {
            return;
        }
    }
}

/// Block asynchronous host signals on this engine thread for its lifetime.
///
/// A host signal that lands in a timed socket read ends the read with `EINTR`
/// even under `SA_RESTART`, and the transport would fail the call. Signals a
/// thread raises by its own action stay open, so the host's disposition still
/// governs them: `SIGXFSZ` from a file-size limit, `SIGPIPE`, and the faults.
/// The calling thread keeps the host's mask.
#[cfg(unix)]
pub(crate) fn mask_host_signals() {
    use nix::sys::signal::{SigSet, Signal};

    let mut mask = SigSet::all();
    for own in [
        Signal::SIGXFSZ,
        Signal::SIGPIPE,
        Signal::SIGSEGV,
        Signal::SIGBUS,
        Signal::SIGFPE,
        Signal::SIGILL,
        Signal::SIGTRAP,
        Signal::SIGSYS,
    ] {
        mask.remove(own);
    }
    // `pthread_sigmask` fails only for an invalid `how`, which this is not.
    let _masked = mask.thread_block();
}

#[cfg(not(unix))]
pub(crate) const fn mask_host_signals() {}
