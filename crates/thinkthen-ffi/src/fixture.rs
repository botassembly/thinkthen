//! Private test builds own loopback routes without adding product configuration.
#![allow(
    unsafe_code,
    reason = "private fixture exports borrow counted inputs and own their route guard"
)]
use crate::{
    HostError,
    exports::{OK, operation},
    memory::text,
};
/// Process-owned test routes. Existing clients retain their admitted snapshots.
#[derive(Debug)]
pub struct FixtureHandle {
    _guard: thinkthen::test_fixture::RouteGuard,
}
/// Install both loopback routes only in an explicit debug test artifact.
/// # Safety
/// URL bytes are readable; out/error are distinct empty writable slots.
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
            let owner = thinkthen::test_fixture::RouteGuard::install(
                text(systemone, systemone_len)?,
                text(openai, openai_len)?,
            )?;
            out.write(Box::into_raw(Box::new(FixtureHandle { _guard: owner })));
            Ok(OK)
        })
    }
}
/// Clear the owned test routes; opened clients keep their copied destinations.
/// # Safety
/// NULL or the unique live guard owner after all readers finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_fixture_routes_free(owner: *mut FixtureHandle) {
    if !owner.is_null() {
        /* SAFETY: the caller transfers its unique owned guard allocation. */
        drop(unsafe { Box::from_raw(owner) });
    }
}
