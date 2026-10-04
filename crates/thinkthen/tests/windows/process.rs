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
        command.creation_flags(windows_sys::Win32::System::Threading::CREATE_NEW_CONSOLE);
        command.spawn().map(|child| Self(Some(child)))
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
        let deadline = Instant::now() + limit;
        loop {
            if self.0.as_mut().expect("owned child").try_wait()?.is_some() {
                return self.0.take().expect("owned child").wait_with_output();
            }
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "owned Windows child",
                ));
            }
            std::thread::sleep(Duration::from_millis(10));
        }
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
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "Windows child did not acknowledge readiness"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(std::fs::read(path).expect("marker"), b"1");
}
pub(crate) fn inject(process: u32, helper: &str) {
    let mut command = Command::new(std::env::current_exe().expect("test executable"));
    command
        .clear_environment()
        .args(["--exact", helper, "--ignored", "--nocapture"])
        .env("THINKTHEN_CONSOLE_INJECT_PID", process.to_string());
    assert!(
        Owned::spawn(&mut command)
            .expect("injector")
            .finish()
            .expect("bounded injector")
            .status
            .success()
    );
}
