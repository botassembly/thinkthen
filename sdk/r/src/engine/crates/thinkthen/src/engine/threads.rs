//! The threads a client keeps between calls, so a later call starts none.
//!
//! A host with many long-lived threads leaves glibc no free malloc arena, so each
//! new thread is given the next arena in turn and a thread per call walks the
//! work across all of them.
use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::{self, JoinHandle, ThreadId};
use std::time::Duration;

/// A kept thread given no work for this long ends.
const IDLE: Duration = Duration::from_secs(60);

/// What the work returns is dropped once its thread is free for other work.
type Work = Box<dyn FnOnce() -> Box<dyn Send> + Send>;

struct State {
    /// The way to hand work to each thread that has none, most recently freed last.
    free: Vec<(ThreadId, Sender<Work>)>,
    kept: HashMap<ThreadId, JoinHandle<()>>,
    released: bool,
    /// The process the threads run in.
    process: u32,
}

impl State {
    /// A forked child inherits this record and none of the threads, so it starts with
    /// no thread. The parent's are neither handed work nor joined, and their channels
    /// are not closed, since a parent thread may have held a channel's lock at the fork.
    /// A fork is safe only while no call is running and no idle thread is ending: a
    /// child that inherits this lock held still hangs, which only unsafe code could fix.
    fn own(&mut self) {
        let process = std::process::id();
        if self.process != process {
            self.process = process;
            std::mem::forget(std::mem::take(&mut self.free));
            std::mem::forget(std::mem::take(&mut self.kept));
        }
    }
}

struct Shared {
    state: Mutex<State>,
    idle: Duration,
}

/// A client and its clones share one set, and releasing the last of them ends its threads.
pub(crate) struct Threads(Arc<Shared>);

impl Default for Threads {
    fn default() -> Self {
        Self::ending_after(IDLE)
    }
}

impl Threads {
    pub(super) fn ending_after(idle: Duration) -> Self {
        Self(Arc::new(Shared {
            state: Mutex::new(State {
                free: Vec::new(),
                kept: HashMap::new(),
                released: false,
                process: std::process::id(),
            }),
            idle,
        }))
    }

    /// Run `work` on the most recently freed kept thread, or on a new one when none is
    /// free, so no call waits for another call's thread and surplus threads stay idle
    /// long enough to end.
    ///
    /// The thread counts as free before the value `work` returns is dropped: a caller
    /// that waits for that drop can hand the same thread its next work.
    /// # Errors
    /// Returns the operating system's refusal to start a thread.
    pub(crate) fn run<T: Send + 'static>(
        &self,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> std::io::Result<()> {
        let work: Work = Box::new(move || -> Box<dyn Send> { Box::new(work()) });
        let mut state = self.0.lock();
        state.own();
        if let Some((_, hand)) = state.free.pop() {
            // Sent under the lock: a thread leaves the list before it stops listening.
            let _ = hand.send(work);
            return Ok(());
        }
        let shared = self.0.clone();
        // The lock is held, so the new thread cannot end before it is listed.
        let thread = thread::Builder::new().spawn(move || shared.keep(work))?;
        state.kept.insert(thread.thread().id(), thread);
        Ok(())
    }
}

impl Drop for Threads {
    fn drop(&mut self) {
        let mut state = self.0.lock();
        state.own();
        state.released = true;
        // Dropping the hands wakes the free threads.
        state.free.clear();
        let kept = std::mem::take(&mut state.kept);
        drop(state);
        // The last client can be released by work on one of these threads, which then
        // ends on its own.
        let own = thread::current().id();
        for (_, thread) in kept.into_iter().filter(|(id, _)| *id != own) {
            let _ = thread.join();
        }
    }
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn keep(&self, mut work: Work) {
        let own = thread::current().id();
        loop {
            // Work reports its own panics. One that escapes must not end the thread,
            // nor may a failing drop end it once it is listed as free.
            let finished = catch_unwind(AssertUnwindSafe(work));
            let (hand, handed) = channel();
            {
                let mut state = self.lock();
                if state.released {
                    return;
                }
                state.free.push((own, hand));
            }
            let _ = catch_unwind(AssertUnwindSafe(move || drop(finished)));
            work = match handed.recv_timeout(self.idle) {
                Ok(work) => work,
                // The client was released or the idle period passed.
                Err(_) => {
                    let mut state = self.lock();
                    state.free.retain(|(id, _)| *id != own);
                    // Work handed before this thread left the list has arrived.
                    let Ok(work) = handed.try_recv() else {
                        state.kept.remove(&own);
                        return;
                    };
                    work
                }
            };
        }
    }
}
