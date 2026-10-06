//! Restore native Ctrl-C delivery before the command installs its CRT signals.
#![cfg(windows)]
#![allow(
    unsafe_code,
    reason = "restoring the inherited console Ctrl-C disposition requires the native console API"
)]
#![deny(unsafe_op_in_unsafe_fn)]

pub(crate) fn enable_interrupts() -> std::io::Result<()> {
    // SAFETY: no callback or Rust pointer is supplied. This changes only the
    // calling process's inherited ignore flag, before signal registration.
    // A process without a console still uses CRT signals and needs no reset.
    unsafe {
        if windows_sys::Win32::System::Console::GetConsoleCP() == 0 {
            return Ok(());
        }
        if windows_sys::Win32::System::Console::SetConsoleCtrlHandler(None, 0) == 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}
