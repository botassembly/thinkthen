//! Console events run in an owned injector process, never in the test runner.
#![cfg(all(windows, test))]
#![allow(
    unsafe_code,
    reason = "ticket 0380 confines native console injection to this test leaf"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use std::io;
use windows_sys::Win32::System::Console::{
    AttachConsole, CTRL_C_EVENT, FreeConsole, GenerateConsoleCtrlEvent, SetConsoleCtrlHandler,
};

pub(crate) fn inject(process: u32) -> io::Result<()> {
    // SAFETY: These console calls hold no Rust pointers or borrowed state. This
    // subprocess owns its console attachment; FreeConsole also permits a child
    // born detached. Ignoring Ctrl-C changes only this injector's disposition.
    unsafe {
        let _detached = FreeConsole();
        if AttachConsole(process) == 0 {
            return Err(io::Error::last_os_error());
        }
        if SetConsoleCtrlHandler(None, 1) == 0 {
            let error = io::Error::last_os_error();
            let _detached = FreeConsole();
            return Err(error);
        }
        if GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0) == 0 {
            let error = io::Error::last_os_error();
            let _detached = FreeConsole();
            return Err(error);
        }
        if FreeConsole() == 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

#[allow(
    dead_code,
    reason = "console-only unit inclusion uses only the injector"
)]
pub(crate) fn identity(file: &std::fs::File) -> io::Result<(u64, [u8; 16])> {
    use std::os::windows::io::AsRawHandle as _;
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_ID_INFO, FileIdInfo, GetFileInformationByHandleEx,
    };
    let mut information = FILE_ID_INFO::default();
    let size = u32::try_from(std::mem::size_of::<FILE_ID_INFO>()).map_err(io::Error::other)?;
    // SAFETY: the independently opened File owns this borrowed handle. Output
    // is aligned, writable, exactly size bytes, and lives through the call.
    if unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle(),
            FileIdInfo,
            (&mut information as *mut FILE_ID_INFO).cast(),
            size,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok((
        information.VolumeSerialNumber,
        information.FileId.Identifier,
    ))
}
