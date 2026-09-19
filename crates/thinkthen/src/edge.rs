//! The process boundary, which every value from outside crosses once.

use std::env;
use std::fmt;
use std::io::{ErrorKind, Read, Write};
use std::time::Duration;

use thinkthen_core::{Evidence, KEY_VAR};

use crate::failure::Failure;

/// The wait before the first retry, which only a test shortens.
const RETRY_WAIT: Duration = Duration::from_secs(1);

/// The environment the command reads, read once.
///
/// `THINKTHEN_BASE_URL` names where the System One interface lives. The other
/// variable shortens the retry wait, and only a test sets it. The key itself is
/// read later, by name, and only when a request is about to go out.
#[derive(Debug, Default)]
pub(crate) struct Environment {
    base_url: Option<String>,
    retry_wait_ms: Option<u64>,
}

impl Environment {
    /// Read the base address and the hidden test wait, which help never shows.
    pub(crate) fn read() -> Self {
        Self {
            base_url: read("THINKTHEN_BASE_URL"),
            retry_wait_ms: read("THINKTHEN_TEST_RETRY_WAIT_MS").and_then(|text| text.parse().ok()),
        }
    }

    /// The base the request is posted under, or `None` when the variable is empty.
    pub(crate) fn base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    /// How long the first retry waits before the wait doubles.
    pub(crate) fn retry_wait(&self) -> Duration {
        self.retry_wait_ms.map_or(RETRY_WAIT, Duration::from_millis)
    }
}

/// Read one variable, or `None` when it holds nothing at all.
///
/// A variable set to the empty string counts as unset, the way most Unix tools
/// read one, and so does a variable holding only white space.
fn read(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

/// Read standard input to its end and take it as the evidence.
///
/// # Errors
///
/// Returns [`Failure`] when the bytes cannot be read, are not valid UTF-8, or
/// hold nothing but white space.
pub(crate) fn evidence(mut reader: impl Read) -> Result<Evidence, Failure> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).map_err(Failure::Input)?;
    let text = String::from_utf8(bytes).map_err(|_| Failure::NotUtf8)?;
    Ok(Evidence::new(text)?)
}

/// The key one request carries, which no diagnostic and no `Debug` line shows.
pub(crate) struct Key(String);

impl Key {
    /// Read the key back for the one header that carries it.
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    /// Take a key value directly, which only a test that needs one does.
    #[cfg(test)]
    pub(crate) fn of(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Debug for Key {
    /// Print a fixed placeholder, so a `{:?}` anywhere can never spill the key.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Key(<withheld>)")
    }
}

/// Read the key from the one variable that holds it.
///
/// # Errors
///
/// Returns [`Failure::NoKey`] when the variable is unset or blank. The message
/// names the variable and never a value.
pub(crate) fn key() -> Result<Key, Failure> {
    let value = read(KEY_VAR).unwrap_or_default();
    if value.trim().is_empty() {
        return Err(Failure::NoKey(KEY_VAR.to_owned()));
    }
    Ok(Key(value))
}

/// Write one line and flush it, letting a closed pipe downstream end it quietly.
///
/// A reader that stops early, as `head` does, is how Unix pipelines end. The
/// command has already done its job, so the answer is the exit code it earned.
///
/// # Errors
///
/// Returns [`Failure::Output`] when the write fails for any other reason.
pub(crate) fn write_line(mut writer: impl Write, line: &str) -> Result<(), Failure> {
    match writeln!(writer, "{line}").and_then(|()| writer.flush()) {
        Err(error) if error.kind() == ErrorKind::BrokenPipe => Ok(()),
        Err(error) => Err(Failure::Output(error)),
        Ok(()) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::{evidence, write_line};
    use crate::failure::Failure;
    use std::io::{Error, ErrorKind, Write};

    /// A writer that fails every write with the kind the case names.
    struct Failing(ErrorKind);

    impl Write for Failing {
        fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
            Err(Error::from(self.0))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Err(Error::from(self.0))
        }
    }

    #[test]
    fn a_closed_pipe_ends_the_write_quietly_and_any_other_failure_does_not() {
        assert!(matches!(
            write_line(Failing(ErrorKind::BrokenPipe), "{}"),
            Ok(())
        ));
        assert!(matches!(
            write_line(Failing(ErrorKind::PermissionDenied), "{}"),
            Err(Failure::Output(_))
        ));
    }

    #[test]
    fn evidence_is_refused_when_it_is_not_text_or_holds_only_white_space() {
        assert!(matches!(evidence(&b"\xff\xfe"[..]), Err(Failure::NotUtf8)));
        assert!(matches!(evidence(&b"  \n"[..]), Err(Failure::Blank(_))));
        assert!(evidence(&b" kept "[..]).is_ok());
    }
}
