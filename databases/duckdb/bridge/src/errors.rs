//! SQL error labels shared with the existing DuckDB extension.

use thinkthen::{Error, ErrorKind};

#[derive(Debug)]
pub(crate) struct RowError {
    pub(crate) text: String,
}

impl RowError {
    pub(crate) fn usage(message: &str) -> Self {
        Self {
            text: usage(message),
        }
    }

    pub(crate) fn local(message: &str) -> Self {
        Self {
            text: render(ErrorKind::Local, message, false),
        }
    }
}

impl From<Error> for RowError {
    fn from(error: Error) -> Self {
        Self {
            text: render(error.kind(), error.detail().message(), error.retryable()),
        }
    }
}

pub(crate) fn usage(message: &str) -> String {
    render(ErrorKind::Usage, message, false)
}

fn render(kind: ErrorKind, message: &str, retryable: bool) -> String {
    let (first, later) = message.split_at(message.find('\n').unwrap_or(message.len()));
    let retry = if retryable { "yes" } else { "no" };
    format!("{}{first} (retryable: {retry}){later}", prefix(kind))
}

fn prefix(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::Usage => "thinkthen usage: ",
        ErrorKind::Backend => "thinkthen backend: ",
        ErrorKind::Local => "thinkthen local: ",
        ErrorKind::Cancelled => "thinkthen cancelled: ",
        ErrorKind::Deadline => "thinkthen deadline: ",
        ErrorKind::Defect => "thinkthen defect: ",
    }
}
