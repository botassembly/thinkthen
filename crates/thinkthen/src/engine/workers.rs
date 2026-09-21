//! Scoped request workers shared by the command schedulers.

use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender, SyncSender, sync_channel};
use std::thread;

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
