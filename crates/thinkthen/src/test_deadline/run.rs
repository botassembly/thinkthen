//! A bounded `Command::output` for a test child.
//!
//! A test binary that includes this file includes `wait.rs` beside it as
//! `wait`, the way the library's `test_deadline` module does.
use std::io;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use super::wait::finish;

/// Run `command` as `Command::output` does, and kill it at the child deadline.
///
/// The child reads an empty input and pipes both outputs. The deadline error
/// names the program and its arguments, and never its environment.
pub(crate) fn output(command: &mut Command) -> io::Result<Output> {
    let mut what = Path::new(command.get_program())
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    for argument in command.get_args() {
        what.push(' ');
        what.push_str(&argument.to_string_lossy());
    }
    let child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    finish(child, &what)
}
