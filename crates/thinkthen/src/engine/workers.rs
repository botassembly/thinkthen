//! Scoped request workers shared by the schedulers and the single calls.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, SyncSender, channel, sync_channel};
use std::sync::{Mutex, Once};
use std::thread;

use crate::engine::Cancel;
use crate::engine::error::Error;

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
    let (done, finished) = sync_channel(1);
    thread::scope(|scope| {
        let worker = scope.spawn(move || {
            with_engine_diagnostics(|| {
                enter();
                let sent = send();
                let _caller_waits = done.send(());
                sent
            })
        });
        while let Err(RecvTimeoutError::Timeout) = finished.recv_timeout(Cancel::poll()) {
            cancel.poll_between_sends();
        }
        worker
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
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
    body: impl FnOnce(SyncSender<W>) -> T,
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
    body: impl FnOnce(SyncSender<W>) -> T,
) -> T
where
    W: Send,
    R: Send,
    G: Send,
{
    let (send, receive) = sync_channel(jobs);
    let queue = Mutex::new(receive);
    thread::scope(|scope| {
        for _ in 0..jobs {
            let results = results.clone();
            let queue = &queue;
            scope.spawn(move || observed_worker(queue, &results, work, begin));
        }
        drop(results);
        body(send)
    })
}

fn observed_worker<W, R, G>(
    queue: &Mutex<Receiver<W>>,
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

/// Run `items` on up to `jobs` workers and hand each result on in item order.
///
/// It feeds one item to each free worker, never more. A failed item, a
/// failure of `each`, or a stop feeds nothing further. The items in flight
/// finish, and the first failure in item order returns. The stop runs its
/// host check here, between feeds and while items are in flight, since this
/// may be the calling thread.
pub(crate) fn ordered<W, R, E>(
    jobs: usize,
    items: Vec<W>,
    cancel: &Cancel<'_>,
    work: &(impl Fn(W) -> Result<R, Error> + Sync),
    each: impl FnMut(R) -> Result<(), E>,
) -> Result<(), E>
where
    W: Send,
    R: Send,
    E: From<Error>,
{
    ordered_until(jobs, items, cancel, work, |_| false, each)
}

/// The native recoverable caller can stop feeding on a completed result while
/// still delivering and joining every item already admitted.
pub(crate) fn ordered_until<W, R, E>(
    jobs: usize,
    items: Vec<W>,
    cancel: &Cancel<'_>,
    work: &(impl Fn(W) -> Result<R, Error> + Sync),
    terminal: impl Fn(&R) -> bool,
    mut each: impl FnMut(R) -> Result<(), E>,
) -> Result<(), E>
where
    W: Send,
    R: Send,
    E: From<Error>,
{
    let (results, received) = channel();
    scoped(
        jobs,
        results,
        &|(place, item)| (place, work(item)),
        |feed| {
            let mut items = items.into_iter().enumerate().peekable();
            let (mut held, mut next, mut in_flight) = (BTreeMap::new(), 0, 0);
            let (mut failure, mut halted) = (None, false);
            loop {
                while failure.is_none() {
                    match held.remove(&next) {
                        Some(Ok(result)) => failure = each(result).err(),
                        Some(Err(error)) => failure = Some(E::from(error)),
                        None => break,
                    }
                    next += 1;
                }
                if failure.is_none() && !halted && (in_flight > 0 || items.peek().is_some()) {
                    failure = cancel.stop().map(E::from);
                }
                halted |= failure.is_some();
                let room = if halted { 0 } else { jobs - in_flight };
                for item in items.by_ref().take(room) {
                    feed.send(item)
                        .map_err(|_| Error::Defect("a request worker ended early"))?;
                    in_flight += 1;
                }
                if in_flight == 0 && (halted || items.peek().is_none()) {
                    return failure.map_or(Ok(()), Err);
                }
                match received.recv_timeout(Cancel::poll()) {
                    Ok((place, result)) => {
                        in_flight -= 1;
                        halted |= result.as_ref().map_or(true, &terminal);
                        held.insert(place, result);
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => {
                        return Err(Error::Defect("the request workers ended early").into());
                    }
                }
            }
        },
    )
}

fn worker<W, R>(queue: &Mutex<Receiver<W>>, results: &Sender<R>, work: &(impl Fn(W) -> R + Sync)) {
    while let Ok(Ok(item)) = queue.lock().map(|receiver| receiver.recv()) {
        if results.send(work(item)).is_err() {
            return;
        }
    }
}

/// Block asynchronous host signals on this engine worker for its lifetime.
///
/// A host signal that lands in a timed socket read ends the read with `EINTR`
/// even under `SA_RESTART`, and the transport would fail the call. Signals a
/// thread raises by its own action stay open, so the host's disposition still
/// governs them: `SIGXFSZ` from a file-size limit, `SIGPIPE`, and the faults.
/// The calling thread keeps the host's mask.
#[cfg(unix)]
fn mask_host_signals() {
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
const fn mask_host_signals() {}
