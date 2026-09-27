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
}

impl From<Error> for RowError {
    fn from(error: Error) -> Self {
        let retry = if error.retryable() {
            " (a second try could help)"
        } else {
            ""
        };
        Self {
            text: format!(
                "{}{retry}{}",
                prefix(error.kind()),
                error.detail().message()
            ),
        }
    }
}

pub(crate) fn usage(message: &str) -> String {
    format!("thinkthen usage: {message}")
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
