//! Console events run in an owned injector process, never in the test runner.
#![cfg(all(windows, test))]
#![allow(
    unsafe_code,
    reason = "ticket 0380 confines native console injection to this test leaf"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use std::io;
use std::os::windows::io::{AsRawHandle as _, FromRawHandle as _, OwnedHandle};
use std::path::Path;
use windows_sys::Win32::System::Console::{
    AttachConsole, CTRL_C_EVENT, FreeConsole, GenerateConsoleCtrlEvent, SetConsoleCtrlHandler,
};

#[allow(
    dead_code,
    reason = "identity-only native unit inclusion uses the independent file query"
)]
pub(crate) fn inject(process: u32, acknowledgment: Option<&Path>) -> io::Result<()> {
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_SYNCHRONIZE};
    // SAFETY: the parent supplies only its owned child's PID. The opened
    // synchronization handle transfers once and closes on every return path.
    let handle = unsafe { OpenProcess(PROCESS_SYNCHRONIZE, 0, process) };
    if handle.is_null() {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: successful OpenProcess returned this uniquely owned handle.
    let target = unsafe { OwnedHandle::from_raw_handle(handle) };
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
        // Generation is asynchronous. Keep the injector attached until the
        // target acknowledges the event or exits, including packed commands
        // that keep their normal 30-second admitted request timeout.
        let delivered = delivered(&target, acknowledgment);
        if FreeConsole() == 0 {
            return Err(io::Error::last_os_error());
        }
        delivered?;
    }
    Ok(())
}

fn delivered(target: &OwnedHandle, acknowledgment: Option<&Path>) -> io::Result<()> {
    use windows_sys::Win32::Foundation::{WAIT_FAILED, WAIT_OBJECT_0};
    use windows_sys::Win32::System::Threading::WaitForSingleObject;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(45);
    loop {
        if acknowledgment
            .is_some_and(|path| matches!(std::fs::read(path), Ok(byte) if byte == b"1"))
        {
            return Ok(());
        }
        // SAFETY: target owns the synchronization handle throughout this
        // bounded wait. No Rust memory is borrowed by the native call.
        match unsafe { WaitForSingleObject(target.as_raw_handle(), 10) } {
            WAIT_OBJECT_0 => {
                if acknowledgment
                    .is_none_or(|path| matches!(std::fs::read(path), Ok(byte) if byte == b"1"))
                {
                    return Ok(());
                }
                return Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "target exited without acknowledging the console event",
                ));
            }
            WAIT_FAILED => return Err(io::Error::last_os_error()),
            _ => {}
        }
        if std::time::Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "console event acknowledgment",
            ));
        }
    }
}

#[allow(
    dead_code,
    reason = "console-only unit inclusion uses only the injector"
)]
pub(crate) fn identity(file: &std::fs::File) -> io::Result<(u64, [u8; 16])> {
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
