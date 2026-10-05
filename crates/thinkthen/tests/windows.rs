//! Disposable native Windows CLI boundaries. Source presence is not native proof.
#![cfg(all(windows, feature = "cli"))]
#![allow(clippy::expect_used, reason = "failed native fixtures stop the test")]
#![allow(clippy::panic, reason = "failed native fixtures stop the test")]
#[path = "../src/test_deadline/child.rs"]
mod child;
#[path = "windows/ffi.rs"]
mod ffi;
#[path = "windows/interrupt.rs"]
mod interrupt;
#[path = "windows/privacy.rs"]
mod privacy;
#[path = "windows/process.rs"]
mod process;
#[path = "windows/remaining.rs"]
mod remaining;
#[path = "windows/support.rs"]
mod support;

#[test]
#[ignore = "subprocess-only native console injector"]
fn console_injector() {
    let process = std::env::var("THINKTHEN_CONSOLE_INJECT_PID")
        .expect("owned PID")
        .parse()
        .expect("PID");
    let acknowledgment =
        std::env::var_os("THINKTHEN_CONSOLE_INJECT_ACK").map(std::path::PathBuf::from);
    ffi::inject(process, acknowledgment.as_deref()).expect("native console injection");
}

#[test]
#[ignore = "subprocess-only public library environment capture"]
fn library_child() {
    use std::io::Write as _;
    let captured = thinkthen::EngineBuilder::from_env();
    let mut output = std::io::stdout().lock();
    match captured {
        Ok(builder) => writeln!(output, "accepted {builder:?}").expect("library marker"),
        Err(error) => writeln!(output, "refused {error:?}").expect("library refusal"),
    }
    output.flush().expect("library flush");
}
