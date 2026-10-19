use super::child::ChildEnvironment as _;
use std::io;
use std::os::windows::process::CommandExt as _;
use std::path::Path;
use std::process::{Child, Command, Output};
use std::time::{Duration, Instant};

/// The guard kills and reaps only the child it created, including assertion failures.
pub(crate) struct Owned(pub(crate) Option<Child>);
impl Owned {
    pub(crate) fn spawn(command: &mut Command) -> io::Result<Self> {
        command.spawn().map(|child| Self(Some(child)))
    }
    pub(crate) fn console(command: &mut Command) -> io::Result<Self> {
        command.creation_flags(windows_sys::Win32::System::Threading::CREATE_NEW_CONSOLE);
        Self::spawn(command)
    }
    pub(crate) fn id(&self) -> u32 {
        self.0.as_ref().expect("owned child").id()
    }
    pub(crate) fn alive(&mut self) -> bool {
        self.0
            .as_mut()
            .expect("owned child")
            .try_wait()
            .expect("child state")
            .is_none()
    }
    pub(crate) fn finish(self) -> io::Result<Output> {
        self.finish_after(Duration::from_secs(30))
    }
    pub(crate) fn finish_after(mut self, limit: Duration) -> io::Result<Output> {
        let output = super::child::wait::finish_after(
            self.0.as_mut().expect("owned child"),
            "owned Windows child",
            limit,
        )?;
        let _finished = self.0.take();
        Ok(output)
    }
}
impl Drop for Owned {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _killed = child.kill();
            let _reaped = child.wait();
        }
    }
}
pub(crate) fn wait(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        if matches!(std::fs::read(path), Ok(byte) if byte == b"1") {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    panic!("Windows child did not acknowledge readiness with the complete marker byte");
}
pub(crate) fn inject(process: u32, helper: &str, acknowledgment: Option<&Path>) {
    let mut command = Command::new(std::env::current_exe().expect("test executable"));
    command
        .clear_environment()
        .args(["--exact", helper, "--ignored", "--nocapture"])
        .env("THINKTHEN_CONSOLE_INJECT_PID", process.to_string());
    if let Some(path) = acknowledgment {
        command.env("THINKTHEN_CONSOLE_INJECT_ACK", path);
    }
    // The injector holds its console through the target's existing 45-second
    // packed-command boundary; the common test-child deadline also reaps it.
    let output = Owned::spawn(&mut command)
        .expect("injector")
        .finish_after(Duration::from_secs(60))
        .expect("bounded injector");
    assert!(output.status.success(), "console injector: {output:?}");
}
