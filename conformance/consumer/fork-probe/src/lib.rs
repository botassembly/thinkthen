//! The one fork call behind the real-fork proofs of ticket 0096.
//!
//! A proof hands [`in_child`] the work a forked child does. The parent waits
//! for the child with a bound and reads whether the work said yes.

use std::panic::{self, AssertUnwindSafe};
use std::thread;
use std::time::{Duration, Instant};

use nix::sys::signal::{Signal, kill};
use nix::sys::wait::{WaitPidFlag, WaitStatus, waitpid};

/// How long a parent waits for its child before it kills it.
const BOUND: Duration = Duration::from_secs(20);

/// Fork, run `work` in the child, and report the child's answer.
///
/// The child leaves by `_exit` with 0 for yes, 1 for no, and 2 for a panic,
/// so no parent handler or buffer runs twice.
///
/// # Errors
///
/// Returns a sentence when the fork fails, the child says no, panics, or
/// outlives the bound.
pub fn in_child(work: impl FnOnce() -> bool) -> Result<(), String> {
    let child = match fork::fork() {
        Ok(Some(child)) => child,
        Ok(None) => {
            let code = match panic::catch_unwind(AssertUnwindSafe(work)) {
                Ok(true) => 0,
                Ok(false) => 1,
                Err(_) => 2,
            };
            fork::leave(code)
        }
        Err(error) => return Err(format!("fork failed: {error}")),
    };
    let started = Instant::now();
    loop {
        match waitpid(child, Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::Exited(_, 0)) => return Ok(()),
            Ok(WaitStatus::Exited(_, code)) => return Err(format!("the child exited {code}")),
            Ok(WaitStatus::StillAlive) if started.elapsed() < BOUND => {
                thread::sleep(Duration::from_millis(10));
            }
            Ok(WaitStatus::StillAlive) => {
                let _killed = kill(child, Signal::SIGKILL);
                let _reaped = waitpid(child, None);
                return Err(format!("the child outlived {BOUND:?}"));
            }
            Ok(other) => return Err(format!("the child ended as {other:?}")),
            Err(error) => return Err(format!("waitpid failed: {error}")),
        }
    }
}

#[allow(
    unsafe_code,
    reason = "a real fork is the boundary ticket 0096 recovers from; nothing else here is unsafe"
)]
mod fork {
    use nix::unistd::{ForkResult, Pid};

    /// Fork once: `Some(child)` in the parent and `None` in the child.
    pub(super) fn fork() -> nix::Result<Option<Pid>> {
        // SAFETY: the child runs only the proof's work and then `_exit`s.
        match unsafe { nix::unistd::fork() }? {
            ForkResult::Parent { child } => Ok(Some(child)),
            ForkResult::Child => Ok(None),
        }
    }

    /// Leave the child without running the parent's exit handlers.
    pub(super) fn leave(code: i32) -> ! {
        // SAFETY: `_exit` takes any status and never returns.
        unsafe { libc::_exit(code) }
    }
}
