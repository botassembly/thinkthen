//! Only command-owned failures cross the public caller-source boundary.
use super::Failure;
use std::{fmt, sync::Mutex};

#[derive(Debug)]
struct CommandSource(Mutex<Option<Failure>>);

impl fmt::Display for CommandSource {
    fn fmt(&self, writer: &mut fmt::Formatter<'_>) -> fmt::Result {
        writer.write_str("command input failure")
    }
}
impl std::error::Error for CommandSource {}

impl Failure {
    /// Retain a command refusal through a public reader or result callback.
    pub(crate) fn into_error(self) -> thinkthen::Error {
        let (code, message) = super::sentence(&self);
        let kind = match code {
            2 => thinkthen::ErrorKind::Usage,
            4 => thinkthen::ErrorKind::Backend,
            5 => thinkthen::ErrorKind::Local,
            130 => thinkthen::ErrorKind::Cancelled,
            _ => thinkthen::ErrorKind::Defect,
        };
        thinkthen::Error::new(kind, message).with_source(CommandSource(Mutex::new(Some(self))))
    }
}

pub(super) fn take(error: &thinkthen::Error) -> Option<Failure> {
    let source = std::error::Error::source(error)?.downcast_ref::<CommandSource>()?;
    source.0.lock().ok()?.take()
}
