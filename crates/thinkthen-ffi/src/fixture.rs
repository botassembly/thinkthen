//! Private test builds own loopback routes without adding product configuration.
#![allow(
    unsafe_code,
    reason = "private fixture exports borrow counted inputs and own their route guard"
)]
use crate::{
    HostError,
    exports::{THINKTHEN_OK, operation},
    memory::text,
};
/// Test routes to local servers, in a debug test build. Free with thinkthen_fixture_routes_free.
#[derive(Debug)]
pub struct FixtureHandle {
    _guard: crate::fixture_routes::RouteGuard,
}
/// Send backend requests to two local test servers, in a debug test build only.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
/// for a bad URL or a build without test routes, or -THINKTHEN_CODE_USAGE_ABI when the text is
/// not UTF-8. Free *out with thinkthen_fixture_routes_free.
/// # Safety
/// Any thread may call this. systemone points to systemone_len readable bytes, or to a NUL-
/// terminated string when systemone_len is THINKTHEN_NUL_TERMINATED; it is copied before
/// return. openai points to openai_len readable bytes, or to a NUL-terminated string when
/// openai_len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error
/// is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_fixture_routes_install(
    systemone: *const u8,
    systemone_len: usize,
    openai: *const u8,
    openai_len: usize,
    out: *mut *mut FixtureHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: all input extents and exclusive output slots remain live until return. */
    unsafe {
        operation(error, || {
            out.write(std::ptr::null_mut());
            let owner = crate::fixture_routes::RouteGuard::install(
                text(systemone, systemone_len)?,
                text(openai, openai_len)?,
            )?;
            out.write(Box::into_raw(Box::new(FixtureHandle { _guard: owner })));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Remove the test routes. Clients already open keep theirs.
///
/// NULL is ignored.
/// # Safety
/// Call this once, after every other use of owner has returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_fixture_routes_free(owner: *mut FixtureHandle) {
    if !owner.is_null() {
        /* SAFETY: the caller transfers its unique owned guard allocation. */
        drop(unsafe { Box::from_raw(owner) });
    }
}
