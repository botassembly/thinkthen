#![cfg(test)]
#![allow(
    unsafe_code,
    reason = "ABI checks read documented process-lifetime native strings"
)]
//! The native code registry exposes the same families as the typed engine.
use std::ffi::CStr;

#[test]
fn exported_families_preserve_all_codes_and_refuse_unknown_values() {
    for code in thinkthen::Code::ALL {
        for value in [code.abi(), -code.abi()] {
            let pointer = thinkthen_ffi::thinkthen_code_family(value);
            assert!(!pointer.is_null(), "family missing for {value}");
            // SAFETY: the registry export lends a process-lifetime NUL string.
            let family = unsafe { CStr::from_ptr(pointer) }.to_str().unwrap();
            assert_eq!(family, code.family().as_str());
            assert_eq!(pointer, thinkthen_ffi::thinkthen_code_family(code.abi()));
            let pointer = thinkthen_ffi::thinkthen_code_name(value);
            assert!(!pointer.is_null(), "name missing for {value}");
            // SAFETY: the registry export lends a process-lifetime NUL string.
            assert_eq!(
                unsafe { CStr::from_ptr(pointer) }.to_str().unwrap(),
                code.as_str()
            );
        }
    }
    for unknown in [i32::MIN, -1, 0, i32::MAX] {
        assert!(thinkthen_ffi::thinkthen_code_family(unknown).is_null());
        assert!(thinkthen_ffi::thinkthen_code_name(unknown).is_null());
    }
}
