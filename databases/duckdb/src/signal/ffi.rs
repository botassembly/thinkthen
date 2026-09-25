//! The SIGINT handler and its install: every `unsafe` line of the signal path.
//!
//! The handler calls only atomics, `raise`, `signal`, and the host's own
//! handler. `check.sh` reads this file and refuses a lock, an allocation, or
//! a call into `thinkthen` inside the handler (R5-21).
#![allow(
    unsafe_code,
    reason = "a signal handler and sigaction need the C signal API"
)]

use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

/// The host's own SIGINT action, recorded before ours goes in. Each record
/// is leaked, because a handler may be reading the one it replaces.
static HOST: AtomicPtr<libc::sigaction> = AtomicPtr::new(std::ptr::null_mut());

/// Whether ours is installed, so a second LOAD never chains to itself.
static INSTALLED: AtomicBool = AtomicBool::new(false);

/// The handler: count the signal, then run the host's own action with the
/// signature its flags name. A default action is restored and re-raised, so
/// it still ends the process.
extern "C" fn on_interrupt(
    signal: libc::c_int,
    info: *mut libc::siginfo_t,
    context: *mut libc::c_void,
) {
    super::on_signal();
    let recorded = HOST.load(Ordering::SeqCst);
    if recorded.is_null() {
        return;
    }
    // SAFETY: a record is stored once and never freed.
    let host = unsafe { *recorded };
    if host.sa_sigaction == libc::SIG_DFL {
        // SAFETY: plain async-signal-safe calls.
        unsafe {
            libc::signal(libc::SIGINT, libc::SIG_DFL);
            libc::raise(signal);
        }
    } else if host.sa_flags & libc::SA_SIGINFO != 0 {
        // SAFETY: the host installed this address with SA_SIGINFO.
        let chained = unsafe {
            std::mem::transmute::<
                usize,
                extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void),
            >(host.sa_sigaction)
        };
        chained(signal, info, context);
    } else if host.sa_sigaction != libc::SIG_IGN {
        // SAFETY: the host installed this address as a one-argument handler.
        let chained =
            unsafe { std::mem::transmute::<usize, extern "C" fn(libc::c_int)>(host.sa_sigaction) };
        chained(signal);
    }
}

/// Take SIGINT, keeping the host's action for the chain. A host that ignores
/// SIGINT keeps ignoring it, and ours never goes in (R3-13).
pub(crate) fn install() {
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    super::start_clock();
    // SAFETY: sigaction with valid pointers to zeroed structs.
    let mut previous: libc::sigaction = unsafe { std::mem::zeroed() };
    if unsafe { libc::sigaction(libc::SIGINT, std::ptr::null(), &raw mut previous) } != 0
        || previous.sa_sigaction == libc::SIG_IGN
    {
        INSTALLED.store(false, Ordering::SeqCst);
        return;
    }
    // The record comes before the install, so a SIGINT right after the
    // install finds the host's action (R7-7).
    HOST.store(Box::into_raw(Box::new(previous)), Ordering::SeqCst);
    #[cfg(feature = "test-hooks")]
    if std::env::var_os("thinkthen_test_hook_raise_in_install").is_some() {
        // SAFETY: raising a signal is always allowed.
        unsafe { libc::raise(libc::SIGINT) };
    }
    // SAFETY: as above.
    let mut ours: libc::sigaction = unsafe { std::mem::zeroed() };
    ours.sa_sigaction = on_interrupt
        as extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void)
        as usize;
    ours.sa_flags = libc::SA_SIGINFO | libc::SA_RESTART;
    let mut replaced: libc::sigaction = unsafe { std::mem::zeroed() };
    // SAFETY: valid pointers to initialized structs.
    let installed = unsafe {
        libc::sigemptyset(&raw mut ours.sa_mask);
        libc::sigaction(libc::SIGINT, &raw const ours, &raw mut replaced)
    };
    if installed != 0 {
        INSTALLED.store(false, Ordering::SeqCst);
    } else if replaced.sa_sigaction == libc::SIG_IGN {
        // Another thread chose to ignore SIGINT first: its choice outranks ours.
        // SAFETY: as above.
        unsafe { libc::sigaction(libc::SIGINT, &raw const replaced, std::ptr::null_mut()) };
        INSTALLED.store(false, Ordering::SeqCst);
    } else if replaced.sa_sigaction != previous.sa_sigaction {
        // Another thread changed SIGINT between the read and the install.
        HOST.store(Box::into_raw(Box::new(replaced)), Ordering::SeqCst);
    }
}
