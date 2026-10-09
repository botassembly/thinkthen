//! Private CLI readiness and wake registration for the native pull host.
use std::sync::{Arc, Mutex};
use std::time::Duration;

type Notify = Box<dyn Fn() + Send + Sync>;

#[derive(Clone, Default)]
pub(crate) struct Wake(Arc<Mutex<Option<Notify>>>);
impl Wake {
    pub(crate) fn register(&self, notify: Notify) {
        if let Ok(mut held) = self.0.lock() {
            *held = Some(notify);
        }
    }
    pub(crate) fn notify(&self) {
        if let Ok(held) = self.0.lock()
            && let Some(notify) = held.as_ref()
        {
            notify();
        }
    }
    pub(crate) fn clear(&self) {
        if let Ok(mut held) = self.0.lock() {
            *held = None;
        }
    }
}

pub(crate) struct CliReader<'a> {
    ready: &'a (dyn Fn() -> bool + Sync),
    pub(crate) pause: Option<Duration>,
    wake: Wake,
}
impl<'a> CliReader<'a> {
    pub(crate) fn new(ready: &'a (dyn Fn() -> bool + Sync), pause: Option<Duration>) -> Self {
        Self {
            ready,
            pause,
            wake: Wake::default(),
        }
    }
    pub(crate) fn ready(&self) -> bool {
        (self.ready)()
    }
    pub(crate) fn wake(&self) -> Wake {
        self.wake.clone()
    }
}
