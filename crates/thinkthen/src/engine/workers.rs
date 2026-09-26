//! Scoped request workers shared by the schedulers and the single calls.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, SyncSender, channel, sync_channel};
use std::thread;

use crate::engine::Cancel;
use crate::engine::error::Error;

thread_local! {
    /// Whether this thread is an engine worker with host signals masked.
    static ENGINE_WORKER: Cell<bool> = const { Cell::new(false) };
}

/// Run one live attempt on an engine worker: this thread when it is one, or
/// else one scoped worker joined before return.
///
/// The calling thread polls the call's stop meanwhile, so the host check runs
/// there during a width gate or retry wait and never during a blocking send.
pub(crate) fn on_worker<T: Send>(cancel: &Cancel<'_>, send: impl FnOnce() -> T + Send) -> T {
    if ENGINE_WORKER.get() {
        return send();
    }
    let (done, finished) = sync_channel(1);
    thread::scope(|scope| {
        let worker = scope.spawn(move || {
            enter();
            let sent = send();
            let _caller_waits = done.send(());
            sent
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

/// Run `items` on up to `jobs` workers and hand each result on in item order.
///
/// It feeds one item to each free worker, never more. A failed item, a
/// failure of `each`, or a stop feeds nothing further. The items in flight
/// finish, and the first failure in item order returns. The stop runs its
/// host check here, between feeds, since this may be the calling thread.
pub(crate) fn ordered<W, R, E>(
    jobs: usize,
    items: Vec<W>,
    cancel: &Cancel<'_>,
    work: &(impl Fn(W) -> Result<R, Error> + Sync),
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
                if failure.is_none() && items.peek().is_some() {
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
                        halted |= result.is_err();
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
