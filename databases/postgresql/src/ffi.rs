//! Every `unsafe` line of the extension: PostgreSQL's interrupt flags, the
//! signal mask a worker starts under, the file privilege check, the
//! descriptor opens behind the named-file gate, the throttle's check, and
//! the exit hook that flushes the usage totals.

use std::ffi::c_int;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::thread::JoinHandle;

use pgrx::callconv::{Arg, ArgAbi};
use pgrx::datum::JsonString;
use pgrx::nullable::Nullable;
use pgrx::pg_sys;
use pgrx::pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use pgrx::{FromDatum, pg_sys::Datum};
use pgrx::{
    GucContext, GucFlags, GucRegistry, GucSetting, PgSqlErrorCode, check_for_interrupts, pg_guard,
};

/// PostgreSQL `json` as original bytes, retaining duplicate member names.
#[derive(Debug)]
pub(crate) struct RawJson(pub(crate) String);

impl FromDatum for RawJson {
    unsafe fn from_polymorphic_datum(
        datum: Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self> {
        // SAFETY: the SQL declaration below supplies a json datum and pgrx owns its copy.
        unsafe { JsonString::from_polymorphic_datum(datum, is_null, typoid) }
            .map(|value| Self(value.0))
    }
}

// SAFETY: FromDatum reads the same PostgreSQL json datum this declaration names.
unsafe impl SqlTranslatable for RawJson {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("json"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("json")))
    }
}

// SAFETY: the SQL mapping guarantees a PostgreSQL json datum for the delegated reader.
unsafe impl<'fcx> ArgAbi<'fcx> for RawJson {
    unsafe fn unbox_arg_unchecked(arg: Arg<'_, 'fcx>) -> Self {
        // SAFETY: pgrx passes this non-null json argument through FromDatum.
        unsafe { arg.unbox_arg_using_from_datum() }.unwrap_or_else(|| pgrx::error!("json was null"))
    }
    unsafe fn unbox_nullable_arg(arg: Arg<'_, 'fcx>) -> Nullable<Self> {
        // SAFETY: FromDatum returns None for SQL NULL.
        unsafe { arg.unbox_arg_using_from_datum() }.into()
    }
}

// PostgreSQL's interrupt flags, read only. PostgreSQL raises the error; the
// wait reads these to tell a cancel from any other pending interrupt.
unsafe extern "C" {
    #[link_name = "QueryCancelPending"]
    static QUERY_CANCEL_PENDING: c_int;
    #[link_name = "ProcDiePending"]
    static PROC_DIE_PENDING: c_int;
}

/// Whether the backend holds a cancel: SIGINT (`pg_cancel_backend`, a
/// statement timeout) sets `QueryCancelPending`, and SIGTERM
/// (`pg_terminate_backend`) sets `ProcDiePending`. A memory-contexts request
/// or a barrier sets neither and waits until the call ends.
pub(crate) fn cancel_pending() -> bool {
    // SAFETY: two plain ints PostgreSQL's signal handlers write on this
    // thread; a volatile read sees the handler's store.
    let (query, die) = unsafe {
        (
            std::ptr::read_volatile(&raw const QUERY_CANCEL_PENDING),
            std::ptr::read_volatile(&raw const PROC_DIE_PENDING),
        )
    };
    query != 0 || die != 0
}

/// Let PostgreSQL raise its own error for a pending cancel, terminate, or
/// statement timeout. Called on the backend thread after the worker detaches.
pub(crate) fn raise_pending_interrupt() {
    check_for_interrupts!();
}

/// Start `work` on a thread that blocks every signal. The backend blocks all
/// signals, spawns, and restores its own mask, so the worker and every
/// thread the engine starts from it inherit the full mask, and PostgreSQL's
/// handlers run only on the backend thread.
pub(crate) fn spawn_masked(
    work: impl FnOnce() + Send + 'static,
) -> std::io::Result<JoinHandle<()>> {
    // SAFETY: `sigset_t` is plain data that `sigfillset` and
    // `pthread_sigmask` initialize before any read.
    let mut all: libc::sigset_t = unsafe { std::mem::zeroed() };
    let mut before: libc::sigset_t = unsafe { std::mem::zeroed() };
    // SAFETY: both pointers name live `sigset_t` values on this stack.
    unsafe {
        libc::sigfillset(&raw mut all);
        libc::pthread_sigmask(libc::SIG_SETMASK, &raw const all, &raw mut before);
    }
    let spawned = std::thread::Builder::new()
        .name("thinkthen-call".to_owned())
        .spawn(work);
    // SAFETY: `before` holds the mask `pthread_sigmask` just read.
    unsafe {
        libc::pthread_sigmask(libc::SIG_SETMASK, &raw const before, std::ptr::null_mut());
    }
    spawned
}

/// Flush this backend's usage totals when it exits (ADR 0113). Registered
/// once, on the backend thread, before the backend builds its first engine.
pub(crate) fn flush_usage_at_exit() {
    static REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if REGISTERED.swap(true, std::sync::atomic::Ordering::AcqRel) {
        return;
    }
    // SAFETY: called on the backend thread; the hook lives for the process.
    unsafe { pg_sys::on_proc_exit(Some(flush_usage), Datum::from(0)) };
}

/// The exit hook. A panic stays inside `thinkthen::contained`.
unsafe extern "C-unwind" fn flush_usage(_code: c_int, _arg: Datum) {
    let _flushed = thinkthen::contained(crate::call::finish_usage);
}

/// Register `thinkthen.throttle` with a check, so PostgreSQL refuses a bad
/// value where it is set: `SET`, `ALTER ROLE`, or a configuration file.
/// The check alone judges the range, so every bad value reads one sentence.
pub(crate) fn define_throttle(setting: &'static GucSetting<i32>) {
    // SAFETY: the check is guarded, and the setting lives for the process.
    unsafe {
        GucRegistry::define_int_guc_with_hooks(
            c"thinkthen.throttle",
            c"requests in flight at once, 1 through 32; -1 leaves the engine default",
            c"",
            setting,
            i32::MIN,
            i32::MAX,
            GucContext::Suset,
            GucFlags::default(),
            Some(check_throttle),
            None,
            None,
        );
    }
}

/// Keep SET successful; only an interactive nonempty assignment gets advice.
pub(crate) fn define_ignored_api_key(setting: &'static GucSetting<Option<std::ffi::CString>>) {
    // SAFETY: the guarded hook only reads PostgreSQL's live proposed value.
    unsafe {
        GucRegistry::define_string_guc_with_hooks(
            c"thinkthen.api_key",
            c"ignored; use THINKTHEN_API_KEY in the server environment",
            c"",
            setting,
            GucContext::Userset,
            GucFlags::NO_SHOW_ALL | GucFlags::SUPERUSER_ONLY | GucFlags::DISALLOW_IN_AUTO_FILE,
            Some(check_ignored_api_key),
            None,
            None,
        );
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn check_ignored_api_key(
    value: *mut *mut std::ffi::c_char,
    _extra: *mut *mut std::ffi::c_void,
    source: pg_sys::GucSource::Type,
) -> bool {
    if matches!(
        source,
        pg_sys::GucSource::PGC_S_INTERACTIVE | pg_sys::GucSource::PGC_S_SESSION
    ) {
        // SAFETY: PostgreSQL passes a live pointer to the proposed NUL-terminated value.
        let proposed = unsafe { *value };
        if !proposed.is_null() && !unsafe { std::ffi::CStr::from_ptr(proposed) }.is_empty() {
            pgrx::ereport!(
                WARNING,
                PgSqlErrorCode::ERRCODE_WARNING,
                "thinkthen.api_key is never read; unset it and set THINKTHEN_API_KEY in the server's environment"
            );
        }
    }
    true
}

#[pg_guard]
unsafe extern "C-unwind" fn check_throttle(
    value: *mut c_int,
    _extra: *mut *mut std::ffi::c_void,
    _source: pg_sys::GucSource::Type,
) -> bool {
    // SAFETY: PostgreSQL passes the proposed value, live for this call.
    let Some(refusal) = crate::call::throttle_refusal(unsafe { *value }) else {
        return true;
    };
    let text = std::ffi::CString::new(refusal).unwrap_or_default();
    // SAFETY: PostgreSQL clears the message before each check and reads it
    // right after; `pstrdup` copies it into the current memory context.
    unsafe {
        pg_sys::GUC_check_errmsg_string = pg_sys::pstrdup(text.as_ptr());
    }
    false
}

/// Whether the current role may read server files by PostgreSQL's own rule:
/// a superuser, or a role with the privileges of `pg_read_server_files`.
pub(crate) fn may_read_files() -> bool {
    // SAFETY: catalog reads on the backend thread inside a transaction.
    unsafe {
        if pg_sys::superuser() {
            return true;
        }
        let role = pg_sys::get_role_oid(c"pg_read_server_files".as_ptr(), false);
        pg_sys::has_privs_of_role(pg_sys::GetUserId(), role)
    }
}

/// Open one file for a read that never blocks and never follows its final
/// symlink.
pub(crate) fn open_plain(path: &Path) -> std::io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW)
        .open(path)
}

/// `openat2` on `dir` with the flags and resolve rules given.
fn openat2(dir: c_int, name: &std::ffi::CStr, flags: c_int, resolve: u64) -> c_int {
    // SAFETY: `open_how` is plain integers, and zero is its documented default.
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(flags).unwrap_or_default();
    how.resolve = resolve;
    // SAFETY: a live or special directory descriptor, a NUL-terminated name,
    // and a pointer and size that describe `how`.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            dir,
            name.as_ptr(),
            &raw const how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    c_int::try_from(fd).unwrap_or(-1)
}

/// Open `rel` beneath `base`: `RESOLVE_BENEATH` refuses a `..` or symlink
/// step out. Only a process without `openat2` falls back to a plain open,
/// and the caller checks the spelling before and the descriptor after.
pub(crate) fn open_beneath(base: &Path, rel: &Path) -> std::io::Result<File> {
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::OpenOptionsExt;
    let dir = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_PATH | libc::O_DIRECTORY)
        .open(base)?;
    let name = std::ffi::CString::new(rel.as_os_str().as_bytes())?;
    let flags = libc::O_RDONLY | libc::O_NONBLOCK | libc::O_NOFOLLOW | libc::O_CLOEXEC;
    let fd = openat2(
        dir.as_raw_fd(),
        &name,
        flags,
        libc::RESOLVE_BENEATH | libc::RESOLVE_NO_MAGICLINKS,
    );
    if fd >= 0 {
        // SAFETY: the kernel just returned this descriptor to us alone.
        return Ok(unsafe { File::from_raw_fd(fd) });
    }
    let error = std::io::Error::last_os_error();
    if openat2_missing() {
        return open_plain(&base.join(rel));
    }
    Err(error)
}

/// Whether this process cannot call `openat2` at all, probed once on `/`.
fn openat2_missing() -> bool {
    static MISSING: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *MISSING.get_or_init(|| {
        let fd = openat2(libc::AT_FDCWD, c"/", libc::O_PATH | libc::O_CLOEXEC, 0);
        if fd >= 0 {
            // SAFETY: the probe's own descriptor, closed once.
            unsafe { libc::close(fd) };
            return false;
        }
        matches!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ENOSYS | libc::EPERM)
        )
    })
}

/// The path the kernel holds for an open descriptor.
pub(crate) fn path_of(file: &File) -> Option<PathBuf> {
    use std::os::fd::AsRawFd;
    std::fs::read_link(format!("/proc/self/fd/{}", file.as_raw_fd())).ok()
}
