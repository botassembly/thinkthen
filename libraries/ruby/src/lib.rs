//! Ruby ownership and naming over native request sessions.
mod ffi;
#[derive(Debug)]
struct Fault {
    kind: thinkthen::ErrorKind,
    message: String,
    retryable: bool,
    facts: Option<Box<thinkthen::Facts>>,
}
impl Fault {
    fn of(kind: thinkthen::ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            retryable: false,
            facts: None,
        }
    }
    fn usage(message: impl Into<String>) -> Self {
        Self::of(thinkthen::ErrorKind::Usage, message)
    }
}
impl From<thinkthen::Error> for Fault {
    fn from(error: thinkthen::Error) -> Self {
        Self {
            kind: error.kind(),
            message: error.detail().message().to_owned(),
            retryable: error.retryable(),
            facts: error.facts().cloned().map(Box::new),
        }
    }
}
