//! The SIGINT handler, its install, and the relate bridge's pipe: every
//! `unsafe` line of the signal path.
//!
//! The handler calls only atomics, `getpid`, `write`, `raise`, `signal`, and
//! the host's own handler. `check.sh` reads this file and refuses a lock, an
//! allocation, or a call into `thinkthen` inside the handler (R5-21).
//!
//! The bridge (ticket 0118 decision 7): the handler writes one byte to a pipe
//! this process owns, and a bridge thread reads it and interrupts every busy
//! kept connection. One atomic packs the owner's process id and the pipe's
//! write end, so the handler reads both in one load. A forked child that has
//! not built its own pipe sees another process's id and writes nothing
//! (R6-5).
#![allow(
    unsafe_code,
    reason = "a signal handler and sigaction need the C signal API"
)]

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};

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
    wake_bridge();
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

/// The bridge's owner in the high 32 bits and its write end in the low 32;
/// 0 until a bridge starts.
static BRIDGE: AtomicU64 = AtomicU64::new(0);

/// The read end beside [`BRIDGE`], so a forked child can close the pair it
/// inherited.
static BRIDGE_READ: AtomicI32 = AtomicI32::new(-1);

/// Serializes bridge starts. The handler never takes it.
static BRIDGE_START: Mutex<()> = Mutex::new(());

const fn unpack(packed: u64) -> (u32, libc::c_int) {
    ((packed >> 32) as u32, packed as u32 as libc::c_int)
}

/// The handler's share of the bridge: one byte to this process's own pipe,
/// with the interrupted thread's `errno` put back.
fn wake_bridge() {
    let (owner, end) = unpack(BRIDGE.load(Ordering::Acquire));
    // SAFETY: `getpid` and `write` are async-signal-safe; `errno` is this
    // thread's own slot.
    unsafe {
        if owner == 0 || owner as libc::pid_t != libc::getpid() {
            return;
        }
        let slot = errno();
        let saved = *slot;
        let byte = 1_u8;
        libc::write(end, (&raw const byte).cast(), 1);
        *slot = saved;
    }
}

#[cfg(target_os = "linux")]
unsafe fn errno() -> *mut libc::c_int {
    // SAFETY: the calling thread's own errno slot.
    unsafe { libc::__errno_location() }
}

#[cfg(target_os = "macos")]
unsafe fn errno() -> *mut libc::c_int {
    // SAFETY: the calling thread's own errno slot.
    unsafe { libc::__error() }
}

/// Start this process's bridge once: a thread that runs `on_wake` for each
/// byte the handler writes. A forked child builds its own before its first
/// relate and closes its copies of the parent's pipe.
pub(crate) fn start_bridge(on_wake: fn()) {
    let pid = std::process::id();
    if unpack(BRIDGE.load(Ordering::Acquire)).0 == pid {
        return;
    }
    let _start = BRIDGE_START.lock().unwrap_or_else(PoisonError::into_inner);
    let inherited = BRIDGE.load(Ordering::Acquire);
    if unpack(inherited).0 == pid {
        return;
    }
    if inherited != 0 {
        BRIDGE.store(0, Ordering::Release);
        // SAFETY: closing this process's copies of the inherited pipe.
        unsafe {
            libc::close(unpack(inherited).1);
            libc::close(BRIDGE_READ.swap(-1, Ordering::AcqRel));
        }
    }
    let mut ends: [libc::c_int; 2] = [-1, -1];
    // SAFETY: a valid two-slot array; the write end never blocks, since a
    // full pipe already holds a wake.
    let made = unsafe {
        cloexec_pipe(&mut ends) && libc::fcntl(ends[1], libc::F_SETFL, libc::O_NONBLOCK) == 0
    };
    let [read_end, write_end] = ends;
    if !made {
        return;
    }
    let spawned = std::thread::Builder::new()
        .name("thinkthen-interrupt".to_owned())
        .spawn(move || {
            let mut bytes = [0_u8; 64];
            loop {
                // SAFETY: a valid buffer; the read end stays open for the
                // thread's life, so a late write never raises SIGPIPE.
                let read = unsafe { libc::read(read_end, bytes.as_mut_ptr().cast(), bytes.len()) };
                if read > 0 {
                    on_wake();
                } else if read == 0
                    || std::io::Error::last_os_error().kind() != std::io::ErrorKind::Interrupted
                {
                    return;
                }
            }
        });
    if spawned.is_err() {
        // SAFETY: the two ends this call made.
        unsafe {
            libc::close(read_end);
            libc::close(write_end);
        }
        return;
    }
    BRIDGE_READ.store(read_end, Ordering::Release);
    BRIDGE.store(
        (u64::from(pid) << 32) | u64::from(write_end as u32),
        Ordering::Release,
    );
}

#[cfg(target_os = "linux")]
unsafe fn cloexec_pipe(ends: &mut [libc::c_int; 2]) -> bool {
    // SAFETY: a valid two-slot array.
    unsafe { libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC) == 0 }
}

#[cfg(target_os = "macos")]
unsafe fn cloexec_pipe(ends: &mut [libc::c_int; 2]) -> bool {
    // SAFETY: a valid two-slot array; macOS has no `pipe2`.
    unsafe {
        libc::pipe(ends.as_mut_ptr()) == 0
            && ends
                .iter()
                .all(|end| libc::fcntl(*end, libc::F_SETFD, libc::FD_CLOEXEC) == 0)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    static PARENT: AtomicUsize = AtomicUsize::new(0);
    static CHILD: AtomicUsize = AtomicUsize::new(0);

    fn parent_woke() {
        PARENT.fetch_add(1, Ordering::SeqCst);
    }

    fn child_woke() {
        CHILD.fetch_add(1, Ordering::SeqCst);
    }

    fn waited(count: &AtomicUsize, least: usize) -> usize {
        let until = Instant::now() + Duration::from_secs(2);
        while count.load(Ordering::SeqCst) < least && Instant::now() < until {
            std::thread::sleep(Duration::from_millis(5));
        }
        count.load(Ordering::SeqCst)
    }

    /// R6-5: a forked child's wake, before and after it builds its own
    /// bridge, never reaches the parent's bridge. The child exits 0 only
    /// when its own bridge saw its second wake. Dropping the process-id
    /// check in `wake_bridge` sends the child's first wake to the parent.
    #[test]
    fn a_forked_childs_wake_never_reaches_the_parents_bridge() {
        super::start_bridge(parent_woke);
        // SAFETY: the child calls only the bridge, then `_exit`.
        let child = unsafe { libc::fork() };
        assert!(child >= 0, "fork failed");
        if child == 0 {
            super::wake_bridge();
            super::start_bridge(child_woke);
            super::wake_bridge();
            let code = i32::from(waited(&CHILD, 1) != 1);
            // SAFETY: leave the forked copy of the test harness at once.
            unsafe { libc::_exit(code) };
        }
        let mut status = 0;
        // SAFETY: the child this test forked.
        let reaped = unsafe { libc::waitpid(child, &raw mut status, 0) };
        assert_eq!(reaped, child);
        assert!(
            libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0,
            "the child's bridge missed its wake: {status}"
        );
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(
            PARENT.load(Ordering::SeqCst),
            0,
            "the child's wakes reached the parent"
        );
        super::wake_bridge();
        assert_eq!(waited(&PARENT, 1), 1, "the parent's own wake");
    }
}
