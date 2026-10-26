//! A pooled connection belongs to the call currently performing its socket I/O.
use crate::Cancel;
use crate::retry::CancelWake;
use std::cell::RefCell;
use std::io;
use std::marker::PhantomData;
use std::net::{Shutdown, TcpStream};
use std::rc::Rc;
use std::sync::{Arc, Mutex, PoisonError, Weak};

thread_local! {
    static ACTIVE: RefCell<Option<Cancel>> = const { RefCell::new(None) };
}

/// ureq performs its synchronous exchange on this worker, including TLS I/O.
pub(super) fn enter(cancel: &Cancel) -> Exchange {
    Exchange {
        previous: ACTIVE.replace(Some(cancel.clone())),
        thread: PhantomData,
    }
}

/// Restore the previous exchange on normal return and during unwinding.
pub(super) struct Exchange {
    previous: Option<Cancel>,
    thread: PhantomData<Rc<()>>,
}

impl Drop for Exchange {
    fn drop(&mut self) {
        ACTIVE.set(self.previous.take());
    }
}

pub(super) fn cancelled() -> bool {
    ACTIVE.with_borrow(|active| active.as_ref().is_some_and(Cancel::is_cancelled))
}

#[derive(Debug)]
struct SocketWake(Mutex<Option<Weak<TcpStream>>>);
impl CancelWake for SocketWake {
    fn wake_cancel(&self) {
        let socket = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(socket) = socket.as_ref().and_then(Weak::upgrade) {
            let _ = socket.shutdown(Shutdown::Both);
        }
    }
}

#[derive(Debug)]
pub(super) struct Registration(Arc<SocketWake>);
impl Drop for Registration {
    fn drop(&mut self) {
        // A cancelling thread may already own a strong listener reference.
        // Revoke that callback and finish any shutdown before pool return.
        self.0
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
    }
}

/// Keep the wake listener alive only during this blocking socket operation.
/// It expires before ureq can put the connection back in its shared pool.
pub(super) fn register(socket: &Arc<TcpStream>) -> io::Result<Option<Registration>> {
    let cancel = ACTIVE.with_borrow(Clone::clone);
    let Some(cancel) = cancel else {
        return Ok(None);
    };
    let registration = Registration(Arc::new(SocketWake(Mutex::new(Some(Arc::downgrade(
        socket,
    ))))));
    cancel.listen(registration.0.clone());
    if cancel.is_cancelled() {
        return Err(io::ErrorKind::Interrupted.into());
    }
    Ok(Some(registration))
}
