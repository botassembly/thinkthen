//! One bounded queue serves blocking and runtime-independent consumers.
use crate::retry::{CancelWake, cancelled};
use crate::{Cancel, Error};
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use std::task::Waker;
#[cfg(feature = "stream")]
use std::task::{Context, Poll};
#[derive(Debug)]
struct State<T> {
    items: VecDeque<T>,
    closed: bool,
    cancelled: bool,
    waker: Option<Waker>,
    #[cfg(any(feature = "native", feature = "command"))]
    reserved: usize,
}
#[derive(Debug)]
pub(super) struct Queue<T> {
    state: Mutex<State<T>>,
    ready: Condvar,
    space: Condvar,
    capacity: usize,
    #[cfg(any(feature = "native", feature = "command"))]
    signal: Mutex<Option<Arc<super::signal::Signal>>>,
}
#[derive(Debug)]
pub(super) enum Received<T> {
    Item(T),
    Closed,
    Cancelled,
}
impl<T: Send + std::fmt::Debug + 'static> Queue<T> {
    pub(super) fn new(capacity: usize, cancel: &Cancel) -> Arc<Self> {
        let queue = Arc::new(Self {
            state: Mutex::new(State {
                items: VecDeque::new(),
                closed: false,
                cancelled: false,
                waker: None,
                #[cfg(any(feature = "native", feature = "command"))]
                reserved: 0,
            }),
            ready: Condvar::new(),
            space: Condvar::new(),
            capacity,
            #[cfg(any(feature = "native", feature = "command"))]
            signal: Mutex::new(None),
        });
        cancel.listen(queue.clone());
        queue
    }
    pub(super) fn send(&self, item: T) -> Result<(), Error> {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let mut state = self
            .space
            .wait_while(state, |s| {
                let occupied = s.items.len();
                #[cfg(any(feature = "native", feature = "command"))]
                let occupied = occupied + s.reserved;
                occupied >= self.capacity && !s.closed && !s.cancelled
            })
            .unwrap_or_else(PoisonError::into_inner);
        if state.cancelled || state.closed {
            return Err(cancelled());
        }
        state.items.push_back(item);
        let waker = state.waker.take();
        drop(state);
        self.ready.notify_all();
        self.changed();
        if let Some(w) = waker {
            w.wake();
        }
        Ok(())
    }
    pub(super) fn try_receive(&self) -> Option<Received<T>> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.items.is_empty() && !state.closed && !state.cancelled {
            return None;
        }
        let result = Self::take(&mut state);
        drop(state);
        self.space.notify_all();
        self.changed();
        Some(result)
    }
    pub(super) fn receive(&self) -> Received<T> {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let mut state = self
            .ready
            .wait_while(state, |s| s.items.is_empty() && !s.closed && !s.cancelled)
            .unwrap_or_else(PoisonError::into_inner);
        let result = Self::take(&mut state);
        drop(state);
        self.space.notify_all();
        self.changed();
        result
    }
    #[cfg(feature = "stream")]
    pub(super) fn poll(&self, cx: &mut Context<'_>) -> Poll<Received<T>> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.items.is_empty() && !state.closed && !state.cancelled {
            state.waker = Some(cx.waker().clone());
            return Poll::Pending;
        }
        let result = Self::take(&mut state);
        drop(state);
        self.space.notify_all();
        self.changed();
        Poll::Ready(result)
    }
    fn changed(&self) {
        #[cfg(any(feature = "native", feature = "command"))]
        if let Some(signal) = &*self.signal.lock().unwrap_or_else(PoisonError::into_inner) {
            signal.notify();
        }
    }
    #[cfg(any(feature = "native", feature = "command"))]
    pub(super) fn observe(&self, signal: Arc<super::signal::Signal>) {
        *self.signal.lock().unwrap_or_else(PoisonError::into_inner) = Some(signal);
    }
    #[cfg(any(feature = "native", feature = "command"))]
    pub(super) fn reserve(&self) -> bool {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed || state.cancelled || state.items.len() + state.reserved >= self.capacity {
            return false;
        }
        state.reserved += 1;
        true
    }
    #[cfg(any(feature = "native", feature = "command"))]
    pub(super) fn send_reserved(&self, item: T) -> Result<(), Error> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.closed || state.cancelled {
            return Err(cancelled());
        }
        if state.reserved == 0 {
            return Err(super::abi_error());
        }
        state.reserved -= 1;
        state.items.push_back(item);
        let waker = state.waker.take();
        drop(state);
        self.ready.notify_all();
        self.changed();
        if let Some(waker) = waker {
            waker.wake();
        }
        Ok(())
    }
    fn take(state: &mut State<T>) -> Received<T> {
        if let Some(item) = state.items.pop_front() {
            Received::Item(item)
        } else if state.cancelled {
            Received::Cancelled
        } else {
            Received::Closed
        }
    }
    pub(super) fn close(&self) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.closed = true;
        let waker = state.waker.take();
        drop(state);
        self.ready.notify_all();
        self.changed();
        self.space.notify_all();
        self.changed();
        if let Some(w) = waker {
            w.wake();
        }
    }
}
impl<T: Send + std::fmt::Debug + 'static> CancelWake for Queue<T> {
    fn wake_cancel(&self) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.cancelled = true;
        let waker = state.waker.take();
        drop(state);
        self.ready.notify_all();
        self.changed();
        self.space.notify_all();
        self.changed();
        if let Some(w) = waker {
            w.wake();
        }
    }
}
