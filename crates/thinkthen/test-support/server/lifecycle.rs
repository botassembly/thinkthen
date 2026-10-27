//! Own accepted sockets and join workers before releasing the fixture.
use super::{Rule, State};
use std::collections::BTreeMap;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug, Default)]
pub(super) struct Stop(Mutex<bool>, Condvar);
impl Stop {
    pub(super) fn wait(&self, delay: Duration) -> bool {
        let guard = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if *guard || delay.is_zero() {
            return *guard;
        }
        *self
            .1
            .wait_timeout_while(guard, delay, |stopped| !*stopped)
            .unwrap_or_else(PoisonError::into_inner)
            .0
    }
    fn set(&self) {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = true;
        self.1.notify_all();
    }
}
#[derive(Debug)]
pub(crate) struct Owner {
    address: SocketAddr,
    stop: Arc<Stop>,
    sockets: Arc<Mutex<BTreeMap<usize, TcpStream>>>,
    worker: Option<JoinHandle<()>>,
}
impl Owner {
    pub(super) fn start(
        listener: TcpListener,
        state: Arc<Mutex<State>>,
        rule: Arc<Rule>,
    ) -> std::io::Result<Self> {
        let address = listener.local_addr()?;
        let stop = Arc::new(Stop::default());
        let sockets = Arc::new(Mutex::new(BTreeMap::new()));
        let stopping = Arc::clone(&stop);
        let active = Arc::clone(&sockets);
        let worker = thread::spawn(move || {
            let mut workers: Vec<JoinHandle<()>> = Vec::new();
            let mut next = 0;
            for stream in listener.incoming().flatten() {
                if stopping.wait(Duration::ZERO) {
                    break;
                }
                let mut pending = Vec::new();
                for worker in workers.drain(..) {
                    if worker.is_finished() {
                        let _ = worker.join();
                    } else {
                        pending.push(worker);
                    }
                }
                workers = pending;
                let Ok(registered) = stream.try_clone() else {
                    continue;
                };
                let id = next;
                next += 1;
                active
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .insert(id, registered);
                let active = Arc::clone(&active);
                let state = Arc::clone(&state);
                let rule = Arc::clone(&rule);
                let stopping = Arc::clone(&stopping);
                workers.push(thread::spawn(move || {
                    super::serve(stream, rule.as_ref(), &state, &stopping);
                    active
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .remove(&id);
                }));
            }
            for socket in active
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .values()
            {
                let _ = socket.shutdown(Shutdown::Both);
            }
            for worker in workers {
                let _ = worker.join();
            }
        });
        Ok(Self {
            address,
            stop,
            sockets,
            worker: Some(worker),
        })
    }
    pub(crate) fn active(&self) -> usize {
        self.sockets
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .len()
    }
}
impl Drop for Owner {
    fn drop(&mut self) {
        self.stop.set();
        let _ = TcpStream::connect(self.address);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
