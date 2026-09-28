//! SQL error labels shared with the existing DuckDB extension.

use thinkthen::{Error, ErrorKind};

#[derive(Debug)]
pub(crate) struct RowError {
    kind: ErrorKind,
    retryable: bool,
    pub(crate) text: String,
}

impl RowError {
    pub(crate) fn usage(message: &str) -> Self {
        Self {
            kind: ErrorKind::Usage,
            retryable: false,
            text: usage(message),
        }
    }

    pub(crate) fn local(message: &str) -> Self {
        Self {
            kind: ErrorKind::Local,
            retryable: false,
            text: format!("{}{}", prefix(ErrorKind::Local), message),
        }
    }

    pub(crate) fn value(&self) -> Option<String> {
        let message = match self.kind {
            ErrorKind::Usage => {
                "check the row's question and arguments, or raise the process request total when it is spent"
            }
            ErrorKind::Local => "check the named file and its permissions",
            ErrorKind::Backend => "the backend did not answer; retry if allowed",
            ErrorKind::Cancelled | ErrorKind::Deadline | ErrorKind::Defect => return None,
        };
        Some(format!(
            "{{\"status\":\"failed\",\"error\":{{\"kind\":\"{}\",\"message\":\"{message}\",\"retryable\":{}}}}}",
            self.kind.name(),
            self.retryable
        ))
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
            kind: error.kind(),
            retryable: error.retryable(),
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
