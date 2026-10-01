//! Ownership and cancellation for loopback fixture sockets and workers.

use std::collections::HashMap;
use std::io;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, Weak};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// A gate that holds answers until the next round, or for good once released.
#[derive(Debug, Default)]
pub(crate) struct Gate {
    pub(crate) open: AtomicBool,
    pub(crate) round: AtomicU64,
}

impl Gate {
    /// Let every held answer go, now and from here on.
    pub(crate) fn release(&self) {
        self.open.store(true, Ordering::SeqCst);
    }

    /// Let go every answer held now. A later answer holds again.
    pub(crate) fn next_round(&self) {
        self.round.fetch_add(1, Ordering::SeqCst);
    }

    /// Whether an answer taken in this round still waits.
    pub(crate) fn holds(&self, round: u64) -> bool {
        !self.open.load(Ordering::SeqCst) && self.round.load(Ordering::SeqCst) <= round
    }
}

/// What the listener saw, for the assertions that count connections.
#[derive(Debug, Default)]
pub(crate) struct Counts {
    pub(crate) connections: AtomicUsize,
    pub(crate) requests: AtomicUsize,
    pub(crate) questions: AtomicUsize,
    pub(crate) in_flight: AtomicUsize,
    pub(crate) peak: AtomicUsize,
}

const POLL: Duration = Duration::from_millis(5);

#[derive(Debug, Default)]
struct State {
    stopped: bool,
    next: usize,
    sockets: HashMap<usize, TcpStream>,
    workers: Vec<JoinHandle<()>>,
    gates: Vec<Weak<Rendezvous>>,
}

/// The accept thread owns the listening socket; the final owner joins it.
#[derive(Debug)]
pub(crate) struct Lifetime {
    state: Mutex<State>,
    wake: Condvar,
    stopped: AtomicBool,
    accept: Mutex<Option<JoinHandle<()>>>,
}

impl Lifetime {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(State::default()),
            wake: Condvar::new(),
            stopped: AtomicBool::new(false),
            accept: Mutex::new(None),
        })
    }

    pub(crate) fn start(
        self: &Arc<Self>,
        listener: TcpListener,
        serve: impl FnOnce(TcpListener, Arc<Self>) + Send + 'static,
    ) -> io::Result<()> {
        listener.set_nonblocking(true)?;
        let owner = Arc::clone(self);
        let accept = thread::spawn(move || serve(listener, owner));
        *self
            .accept
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = Some(accept);
        Ok(())
    }

    pub(crate) fn accept(&self, listener: &TcpListener) -> Option<TcpStream> {
        loop {
            if self.stopped() {
                return None;
            }
            match listener.accept() {
                Ok((stream, _)) => {
                    // Windows and macOS hand the listener's nonblocking mode
                    // to each accepted socket, and Linux does not. Every
                    // connection reads blocking (ticket 0373).
                    let _blocking = stream.set_nonblocking(false);
                    return Some(stream);
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
                    if state.stopped {
                        return None;
                    }
                    drop(
                        self.wake
                            .wait_timeout(state, POLL)
                            .unwrap_or_else(|error| error.into_inner()),
                    );
                }
                Err(_) => return None,
            }
        }
    }

    pub(crate) fn stopped(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }

    /// Register under the same lock retirement uses to stop acceptance.
    pub(crate) fn register(&self, stream: &TcpStream) -> Option<usize> {
        let copy = stream.try_clone().ok()?;
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if state.stopped {
            return None;
        }
        let id = state.next;
        state.next += 1;
        state.sockets.insert(id, copy);
        Some(id)
    }

    pub(crate) fn finish(&self, id: usize) {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .sockets
            .remove(&id);
    }

    pub(crate) fn spawn(
        self: &Arc<Self>,
        stream: TcpStream,
        work: impl FnOnce(TcpStream, Arc<Self>) + Send + 'static,
    ) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if state.stopped {
            return;
        }
        let Ok(copy) = stream.try_clone() else { return };
        let id = state.next;
        state.next += 1;
        state.sockets.insert(id, copy);
        let owner = Arc::clone(self);
        state.workers.push(thread::spawn(move || {
            work(stream, Arc::clone(&owner));
            owner.finish(id);
        }));
    }

    pub(crate) fn pause(&self, duration: Duration) -> bool {
        let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if state.stopped {
            return false;
        }
        let (state, _) = self
            .wake
            .wait_timeout_while(state, duration, |state| !state.stopped)
            .unwrap_or_else(|error| error.into_inner());
        !state.stopped
    }

    pub(crate) fn watch(&self, gate: Option<&Arc<Rendezvous>>) {
        let Some(gate) = gate else { return };
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if state.stopped {
            gate.cancel();
        } else if !state
            .gates
            .iter()
            .any(|held| held.ptr_eq(&Arc::downgrade(gate)))
        {
            state.gates.push(Arc::downgrade(gate));
        }
    }

    pub(crate) fn retire(&self) {
        {
            let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
            if !state.stopped {
                state.stopped = true;
                self.stopped.store(true, Ordering::SeqCst);
                for socket in state.sockets.values() {
                    let _ = socket.shutdown(Shutdown::Both);
                }
                for gate in &state.gates {
                    if let Some(gate) = gate.upgrade() {
                        gate.cancel();
                    }
                }
                self.wake.notify_all();
            }
        }
        let accept = self
            .accept
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .take();
        if let Some(accept) = accept {
            let _ = accept.join();
        }
        let workers = std::mem::take(
            &mut self
                .state
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .workers,
        );
        for worker in workers {
            let _ = worker.join();
        }
    }
}

/// A rendezvous that lets fixture retirement interrupt a waiting response.
#[derive(Debug)]
pub struct Rendezvous {
    parties: usize,
    state: Mutex<(usize, usize, bool)>,
    wake: Condvar,
}

impl Rendezvous {
    /// Number of participants in each round.
    pub fn new(parties: usize) -> Self {
        Self {
            parties,
            state: Mutex::new((0, 0, false)),
            wake: Condvar::new(),
        }
    }

    /// Wait for the other participants outside the fixture.
    pub fn wait(&self) -> bool {
        self.wait_inner(None)
    }

    pub(crate) fn wait_owned(&self, lifetime: &Lifetime) -> bool {
        self.wait_inner(Some(lifetime))
    }

    fn cancel(&self) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state.2 = true;
        self.wake.notify_all();
    }

    fn wait_inner(&self, lifetime: Option<&Lifetime>) -> bool {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if state.2 {
            return false;
        }
        let round = state.1;
        state.0 += 1;
        if state.0 == self.parties {
            state.0 = 0;
            state.1 += 1;
            self.wake.notify_all();
            return true;
        }
        while state.1 == round {
            if state.2 || lifetime.is_some_and(Lifetime::stopped) {
                return false;
            }
            state = self
                .wake
                .wait(state)
                .unwrap_or_else(|error| error.into_inner());
        }
        !state.2
    }
}
