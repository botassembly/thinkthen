//! Isolate process-global native state in a bounded child test process.
use crate::test_deadline::child::ChildEnvironment as _;
use std::{
    env, fs,
    process::{Command, Stdio},
};
const CHILD: &str = "THINKTHEN_TEST_WIDTH_CHILD";

/// Run the ignored test at `path` alone in a fresh copy of the binary. A
/// child still running at the test deadline is killed and fails the test.
pub(crate) fn in_child_at(path: &str) {
    let name = path.replace("::", "-");
    let home = env::temp_dir().join(format!("thinkthen-width-{name}-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).expect("child home");
    let mut command = Command::new(env::current_exe().expect("test binary"));
    command
        .args([
            "--exact",
            path,
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .clear_environment()
        .env(CHILD, "1")
        .home(&home)
        .env("XDG_CACHE_HOME", home.join("cache"))
        .env("XDG_CONFIG_HOME", home.join("config"))
        .env("THINKTHEN_API_KEY", "sk-test-value")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let child = command.spawn().expect("child");
    let output = crate::test_deadline::finish(child, path).expect("child");
    let _removed = fs::remove_dir_all(&home);
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("1 passed"),
        "the child ran its test"
    );
}

/// Only a child started by [`in_child_at`] runs the body of an ignored test.
pub(crate) fn child() -> bool {
    env::var_os(CHILD).is_some()
}
