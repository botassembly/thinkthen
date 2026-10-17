//! Identical recognition records wait for the existing cache writer, not another send.
use super::records::Prepared;
use crate::engine::Cancel;
use crate::public::{Error, Recognize};
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

#[derive(Clone, PartialEq)]
struct Identity {
    text: String,
    context: Option<crate::core::Evidence>,
    ask: Recognize,
}

#[derive(Default)]
pub(super) struct InFlight {
    active: Mutex<Vec<Identity>>,
    changed: Condvar,
}

impl InFlight {
    fn lock(&self) -> MutexGuard<'_, Vec<Identity>> {
        self.active.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(super) fn enter<'a>(
        &'a self,
        prepared: &Prepared,
        cancel: &Cancel<'_>,
    ) -> Result<Entered<'a>, Error> {
        let identity = Identity {
            text: prepared.text.clone(),
            context: prepared.resolved_context.clone(),
            ask: prepared.ask.clone(),
        };
        let mut active = self.lock();
        loop {
            cancel.stop_or_remaining().map_err(Error::from)?;
            if !active.contains(&identity) {
                active.push(identity.clone());
                return Ok(Entered {
                    owner: self,
                    identity,
                });
            }
            active = self
                .changed
                .wait_timeout(active, Cancel::poll())
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
    }
}

pub(super) struct Entered<'a> {
    owner: &'a InFlight,
    identity: Identity,
}

impl Drop for Entered<'_> {
    fn drop(&mut self) {
        self.owner
            .lock()
            .retain(|identity| *identity != self.identity);
        self.owner.changed.notify_all();
    }
}
