//! The stable bootstrap used before any other C operation.
use std::ffi::c_char;

/// Return the loaded library's NUL-terminated package version and write its interface
/// number. The returned immutable text lives until the library is unloaded; never free
/// it. This bootstrap signature stays stable even when the interface number changes.
/// It allocates nothing, sends nothing, and changes no engine or thread error state.
///
/// Check once when loading the library, before creating an engine or calling any other
/// operation. Compare the written number exactly with THINKTHEN_INTERFACE_NUMBER.
/// On mismatch, report "ThinkThen C interface mismatch: header requires %u, loaded
/// library %s provides %u" with the header number, returned version and loaded number,
/// then refuse all other operations. A dynamic loader must also refuse a library
/// missing this symbol. Package version differences alone do not imply incompatibility.
///
/// # Safety
/// interface_number is nonnull, aligned writable storage for one uint32_t, retained
/// throughout this call. The library remains loaded while reading the returned text.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_version(interface_number: *mut u32) -> *const c_char {
    // SAFETY: the host supplies one aligned writable uint32_t under this contract.
    unsafe { *interface_number = super::values::THINKTHEN_INTERFACE_NUMBER };
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast()
}
