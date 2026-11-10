//! One cancellation-aware deadline covers mutex admission and SQLite contention.
use crate::{Cancel, Code, Error};
use std::cell::{Cell, RefCell};
use std::time::{Duration, Instant};

thread_local! { static ACTIVE: RefCell<Option<Context>> = const { RefCell::new(None) }; }
#[derive(Debug)]
struct Context {
    cancel: Cancel,
    deadline: Instant,
    sqlite_reported: Cell<bool>,
    mutex_reported: Cell<bool>,
}
#[derive(Debug)]
pub(super) struct Wait {
    previous: Option<Context>,
}
impl Wait {
    pub(super) fn enter(cancel: &Cancel, milliseconds: u64) -> Self {
        Self {
            previous: ACTIVE.with(|held| {
                held.replace(Some(Context {
                    cancel: cancel.clone(),
                    deadline: Instant::now() + Duration::from_millis(milliseconds),
                    sqlite_reported: Cell::new(false),
                    mutex_reported: Cell::new(false),
                }))
            }),
        }
    }
    pub(super) fn pause(&self) -> Result<(), Error> {
        pause(false)
    }
}
impl Drop for Wait {
    fn drop(&mut self) {
        ACTIVE.with(|held| {
            held.replace(self.previous.take());
        });
    }
}
fn pause(sqlite: bool) -> Result<(), Error> {
    // Release the thread-local borrow before invoking a caller-owned logger.
    let (cancel, deadline, first) = ACTIVE
        .with(|held| {
            held.borrow().as_ref().map(|context| {
                let reported = if sqlite {
                    &context.sqlite_reported
                } else {
                    &context.mutex_reported
                };
                (
                    context.cancel.clone(),
                    context.deadline,
                    !reported.replace(true),
                )
            })
        })
        .ok_or_else(|| super::error(Code::LocalResponseStore))?;
    if first {
        if sqlite {
            log::debug!(target: "thinkthen::response_store", "waiting for response-store SQLite access");
        } else {
            log::debug!(target: "thinkthen::response_store", "waiting for response-store client connection");
        }
    }
    if cancel.is_cancelled() {
        return Err(crate::retry::cancelled());
    }
    let remaining = deadline
        .checked_duration_since(Instant::now())
        .filter(|left| !left.is_zero())
        .ok_or_else(|| super::error(Code::LocalResponseStore))?;
    if cancel.wait(remaining.min(Duration::from_millis(10))) {
        return Err(crate::retry::cancelled());
    }
    Ok(())
}
pub(super) fn busy(_: i32) -> bool {
    pause(true).is_ok()
}
pub(super) fn cancelled() -> bool {
    ACTIVE.with(|held| {
        held.borrow()
            .as_ref()
            .is_some_and(|context| context.cancel.is_cancelled())
    })
}
