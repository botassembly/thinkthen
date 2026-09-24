//! Scoped request workers shared by the schedulers and the single calls.

use std::cell::Cell;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender, SyncSender, sync_channel};
use std::thread;

thread_local! {
    /// Whether this thread is an engine worker with host signals masked.
    static ENGINE_WORKER: Cell<bool> = const { Cell::new(false) };
}

/// Which thread asked and which thread sent, for each live attempt a test makes.
#[cfg(test)]
pub(crate) static SENDS: Mutex<Vec<(thread::ThreadId, thread::ThreadId)>> = Mutex::new(Vec::new());

/// Run one live attempt on an engine worker: this thread when it is one, or
/// else one scoped worker joined before return.
pub(crate) fn on_worker<T: Send>(send: impl FnOnce() -> T + Send) -> T {
    #[cfg(test)]
    let caller = thread::current().id();
    let sent = move || {
        #[cfg(test)]
        if let Ok(mut sends) = SENDS.lock() {
            sends.push((caller, thread::current().id()));
        }
        send()
    };
    if ENGINE_WORKER.get() {
        return sent();
    }
    thread::scope(|scope| {
        scope
            .spawn(|| {
                enter();
                sent()
            })
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
            scope.spawn(move || {
                enter();
                let _lifetime = begin();
                worker(queue, &results, work);
            });
        }
        drop(results);
        body(send)
    })
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
