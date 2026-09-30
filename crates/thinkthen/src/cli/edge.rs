//! The process boundary, which every value from outside crosses once.

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, IsTerminal as _, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::core::{Backend, Reading};

mod backend;
mod key;
mod roots;
use key::KeySnapshot;

/// The command's shared, latched observation of a closed output pipe.
#[derive(Clone, Default)]
pub(crate) struct Downstream(Arc<AtomicBool>);

impl Downstream {
    pub(crate) fn gone(&self) -> bool {
        if self.latched() {
            return true;
        }
        #[cfg(unix)]
        {
            use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
            use std::os::fd::AsFd;
            let stdout = io::stdout();
            let mut fds = [PollFd::new(stdout.as_fd(), PollFlags::POLLOUT)];
            if poll(&mut fds, PollTimeout::ZERO).is_ok()
                && fds[0]
                    .revents()
                    .is_some_and(|flags| flags.intersects(PollFlags::POLLERR | PollFlags::POLLHUP))
            {
                self.0.store(true, Ordering::SeqCst);
            }
        }
        self.latched()
    }

    pub(crate) fn latched(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

use crate::config::{self, Config};
use crate::engine::{backoff::per_minute, estimated_total, usage::Counters};
use crate::failure::Failure;

/// The wait before the first retry, which only a test shortens.
const RETRY_WAIT: Duration = Duration::from_secs(1);

/// The most bytes one record is read from the input.
///
/// A record at the limit is taken, and one byte past it is refused, so reading
/// one byte past the limit settles it. A stream may end a record with `\r\n`,
/// and neither byte is part of the record, so the bound allows both.
const BOUND: u64 = crate::core::MAX_RECORD_BYTES as u64 + 2;

/// The environment the command reads, read once. Only a debug build reads the
/// `THINKTHEN_TEST_` variables. The key is captured once after the final
/// address resolves, and that value governs every later check and request.
#[derive(Default)]
pub(crate) struct Environment {
    base_url: Option<String>,
    backend: Option<String>,
    batch: Option<String>,
    max_request_bytes: Option<String>,
    named_cache: bool,
    cache: Option<PathBuf>,
    cache_is_platform_default: bool,
    config: Config,
    config_path: Option<PathBuf>,
    retry_wait_ms: Option<u64>,
    pub(super) sigint_ack: Option<PathBuf>,
    pub(super) cancel: crate::engine::Cancel<'static>,
    usage: std::sync::Arc<Counters>,
    usage_path: Option<PathBuf>,
    key: KeySnapshot,
    ca_bundle: Option<PathBuf>,
    pub(crate) per_minute: Option<std::num::NonZeroU32>,
    pub(crate) estimated_total: Option<u64>,
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Environment")
            .field("base_url", &self.base_url.as_ref().map(|_| "<withheld>"))
            .field("backend", &self.backend.as_ref().map(|_| "<withheld>"))
            .field("config", &self.config)
            .field("key", &self.key)
            .finish_non_exhaustive()
    }
}

impl Environment {
    /// Read the base address and the hidden test wait, which help never shows.
    pub(crate) fn read() -> Result<Self, Failure> {
        let named_cache = read("THINKTHEN_CACHE");
        let config_path = config::path();
        let config = Config::read(config_path.as_deref())?;
        let usage_path = config::usage_path();
        let usage = std::sync::Arc::new(Counters::new(usage_path.clone()));
        usage.set_prices(config.prices());
        Ok(Self {
            base_url: read("THINKTHEN_BASE_URL"),
            backend: read("THINKTHEN_BACKEND"),
            batch: read("THINKTHEN_BATCH"),
            max_request_bytes: env::var("THINKTHEN_MAX_REQUEST_BYTES").ok(),
            cache: named_cache
                .as_ref()
                .map(PathBuf::from)
                .or_else(config::cache_path),
            cache_is_platform_default: named_cache.is_none(),
            named_cache: named_cache.is_some(),
            config,
            config_path,
            retry_wait_ms: test_only("THINKTHEN_TEST_RETRY_WAIT_MS")
                .and_then(|text| text.parse().ok()),
            sigint_ack: test_only("THINKTHEN_TEST_SIGINT_ACK").map(PathBuf::from),
            cancel: crate::engine::Cancel::default(),
            usage,
            usage_path,
            key: KeySnapshot::default(),
            ca_bundle: read("THINKTHEN_CA_BUNDLE").map(PathBuf::from),
            per_minute: per_minute(read("THINKTHEN_REQUESTS_PER_MINUTE").as_deref())
                .map_err(Failure::Usage)?,
            estimated_total: estimated_total(read("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL"))
                .map_err(Failure::Usage)?,
        })
    }

    /// The answer-cache folder selected by the environment or platform.
    pub(crate) fn cache(&self) -> Option<&Path> {
        self.cache.as_deref()
    }

    pub(crate) const fn cache_is_platform_default(&self) -> bool {
        self.cache_is_platform_default
    }

    pub(crate) fn model(&self) -> Option<&str> {
        self.config.model()
    }

    pub(crate) fn default_cache_enabled(&self) -> bool {
        self.config.cache_enabled()
    }

    pub(crate) fn cache_bytes(&self) -> u64 {
        self.config.cache_bytes()
    }

    pub(crate) const fn config(&self) -> &Config {
        &self.config
    }
    pub(crate) fn config_path(&self) -> Option<&Path> {
        self.config_path.as_deref()
    }
    pub(crate) const fn named_cache(&self) -> bool {
        self.named_cache
    }
    pub(crate) fn usage_path(&self) -> Option<&Path> {
        self.usage_path.as_deref()
    }
    pub(crate) fn usage(&self) -> &Counters {
        &self.usage
    }
    /// The process counters, shared with the engine a command builds.
    pub(crate) fn counters(&self) -> std::sync::Arc<Counters> {
        std::sync::Arc::clone(&self.usage)
    }
    /// `THINKTHEN_BATCH`, read by record-batching judging commands.
    pub(crate) fn batch(&self) -> Option<&str> {
        self.batch.as_deref()
    }

    /// Resolve the byte limit only for a command where the setting acts.
    pub(crate) fn request_size(&self, flag: Option<&str>) -> Result<usize, Failure> {
        let chosen = flag.or(self.max_request_bytes.as_deref());
        let Some(value) = chosen else {
            return Ok(Backend::DEFAULT_REQUEST_SIZE);
        };
        let number = value
            .parse::<usize>()
            .ok()
            .filter(|&number| number > 0 && value.bytes().all(|byte| byte.is_ascii_digit()));
        number.ok_or(Failure::Usage(if flag.is_some() {
            "--max-request-bytes takes a whole number of at least 1"
        } else {
            "THINKTHEN_MAX_REQUEST_BYTES takes a whole number of at least 1"
        }))
    }

    /// Warn before planning or sending a request above the built-in default.
    pub(crate) fn warn_request_size(&self, backend: &Backend) -> Result<(), Failure> {
        let size = backend.ceiling();
        if backend.is_built_in() && size > Backend::DEFAULT_REQUEST_SIZE {
            writeln!(
                io::stderr().lock(),
                "thinkthen: warning: max_request_bytes {size} is above the default of 96000; the built-in backend refuses a request over 65536 input tokens"
            )
            .map_err(Failure::Output)?;
        }
        Ok(())
    }

    /// How long the first retry waits before the wait doubles.
    pub(crate) fn retry_wait(&self) -> Duration {
        self.retry_wait_ms.map_or(RETRY_WAIT, Duration::from_millis)
    }

    pub(crate) const fn cancel(&self) -> &crate::engine::Cancel<'static> {
        &self.cancel
    }
}

/// Take `--model` on the commands that build their own specification.
pub(crate) fn model_flag(text: &str) -> Result<crate::core::ModelName, Failure> {
    crate::core::ModelName::new(text).map_err(|error| {
        Failure::Usage(match error {
            crate::core::BlankTextError::ModelControl => {
                "--model holds no control character or white space but a plain space"
            }
            _ => "--model is text, not white space",
        })
    })
}

/// Read a `THINKTHEN_TEST_` variable in a build with debug assertions, which
/// is what the test suites spawn. A release binary reads `None`.
fn test_only(name: &str) -> Option<String> {
    if cfg!(debug_assertions) {
        read(name)
    } else {
        None
    }
}

/// Read one variable, or `None` when it holds nothing at all.
///
/// A variable set to the empty string counts as unset, the way most Unix tools
/// read one, and so does a variable holding only white space.
fn read(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

/// Open the file the user named, or take the reader the process was given.
///
/// # Errors
///
/// Returns [`Failure::OpenInput`] when the path names no file this user reads.
pub(crate) fn source<'a>(
    path: Option<&Path>,
    reader: impl Read + Send + 'a,
) -> Result<Box<dyn BufRead + Send + 'a>, Failure> {
    match path {
        Some(path) => opened(File::open(path).map_err(Failure::OpenInput)?),
        None => Ok(Box::new(BufReader::new(reader))),
    }
}

/// Classify and buffer the handle that `source` opened.
fn opened<'a>(file: File) -> Result<Box<dyn BufRead + Send + 'a>, Failure> {
    if file.metadata().map_err(Failure::OpenInput)?.is_dir() {
        Err(Failure::InputDirectory)
    } else {
        Ok(Box::new(BufReader::new(file)))
    }
}

/// The bytes of one record at a time, read no further than the caller asks.
///
/// A stream yields one line at a time, with the line feed that ended it, so a
/// plan over the first record reads that record alone. One document yields
/// every byte once and then nothing.
#[derive(Debug)]
pub(crate) struct Chunks<R> {
    reader: R,
    streams: bool,
    spent: bool,
}

impl<R: BufRead> Chunks<R> {
    /// Read records of this shape from this reader.
    pub(crate) const fn new(reader: R, streams: bool) -> Self {
        Self {
            reader,
            streams,
            spent: false,
        }
    }
}

impl<R: BufRead> Iterator for Chunks<R> {
    type Item = Result<Vec<u8>, Failure>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.spent {
            return None;
        }
        let mut bytes = Vec::new();
        // The core refuses a record past the limit, so nothing beyond the two
        // bytes that may end one is worth reading. A stream with no line feed
        // in it would otherwise be read into memory whole.
        let mut reader = (&mut self.reader).take(BOUND);
        let read = if self.streams {
            let read = reader.read_until(b'\n', &mut bytes);
            // A read that stopped at the bound found no line feed, so the
            // record runs past the cut. What follows the cut is the middle of
            // that record and not a record of its own, and framing it as one
            // would send part of a refused record to the backend. The stream
            // ends here, and the record the cut holds is refused for its size.
            if !bytes.ends_with(b"\n") {
                self.spent = true;
            }
            read
        } else {
            self.spent = true;
            reader.read_to_end(&mut bytes)
        };
        match read {
            Err(error) => {
                self.spent = true;
                Some(Err(Failure::Input(error)))
            }
            Ok(0) if self.streams => None,
            Ok(_) => Some(Ok(bytes)),
        }
    }
}

/// Keep the input line number while dropping only blank lines in line framing.
pub(crate) fn numbered<R: BufRead>(
    chunks: Chunks<R>,
    reading: &Reading,
) -> impl Iterator<Item = (usize, Result<Vec<u8>, Failure>)> + use<R> {
    let reading = reading.clone();
    chunks.enumerate().filter_map(move |(place, row)| {
        if row.as_ref().is_ok_and(|bytes| reading.skips(bytes)) {
            None
        } else {
            Some((place + 1, row))
        }
    })
}

/// What a user sitting at a terminal is told the command is waiting for.
///
/// A command reading from a terminal looks hung, because it waits for evidence
/// nobody typed yet. The line names what is read and how to end it.
const WAITING: &str = concat!(
    "thinkthen: reading evidence from the terminal; ",
    "end it with Ctrl-D on a line of its own\n"
);

/// Say what the command waits for, when a person is the one it waits on.
///
/// The line goes to standard error, so it never joins the answer, and it is
/// written only when standard input is a terminal. A pipe, a file under
/// `--input`, and a redirection all leave it unwritten, so the bytes a script
/// reads never change. A line that cannot be written changes nothing.
fn waiting_on_terminal(input: Option<&Path>, terminal: bool, mut writer: impl Write) {
    if input.is_some() || !terminal {
        return;
    }
    let _unwritten = write!(writer, "{WAITING}").and_then(|()| writer.flush());
}

/// Say what the command waits for, reading the terminal from this process.
pub(crate) fn waiting(input: Option<&Path>, writer: impl Write) {
    waiting_on_terminal(input, io::stdin().is_terminal(), writer);
}

/// Write one line and flush it, saying whether the pipe downstream is still open.
///
/// A reader that stops early, as `head` does, is how Unix pipelines end. The
/// command has already done its job, so the answer is the exit code it earned.
/// A record run reads the `false` and stops reading and scheduling.
///
/// # Errors
///
/// Returns [`Failure::Output`] when the write fails for any other reason.
pub(crate) fn write_line(mut writer: impl Write, line: &str) -> Result<bool, Failure> {
    match writeln!(writer, "{line}").and_then(|()| writer.flush()) {
        Err(error) if error.kind() == ErrorKind::BrokenPipe => Ok(false),
        Err(error) => Err(Failure::Output(error)),
        Ok(()) => Ok(true),
    }
}

#[cfg(test)]
mod deadline_tests;

#[cfg(test)]
mod tests {
    use super::{Chunks, Environment, opened, write_line};
    use crate::failure::Failure;
    use std::fs::{self, File};
    use std::io::{Error, ErrorKind, Read as _, Write};

    #[test]
    fn raw_environment_debug_withholds_the_address() {
        let environment = Environment {
            base_url: Some("http://localhost/environment-marker-0210".to_owned()),
            backend: Some("sk-backend-marker-0334".to_owned()),
            ..Environment::default()
        };
        let shown = format!("{environment:?}");
        assert!(shown.contains("base_url: Some(\"<withheld>\")"));
        assert!(!shown.contains("environment-marker-0210"));
        assert!(!shown.contains("sk-backend-marker-0334"));
    }

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
    fn only_a_terminal_with_no_input_file_is_told_what_the_command_waits_for() {
        let file = std::path::Path::new("evidence.txt");
        let cases = [
            (None, true, super::WAITING),
            (None, false, ""),
            (Some(file), true, ""),
            (Some(file), false, ""),
        ];

        for (input, terminal, expected) in cases {
            let mut written = Vec::new();
            super::waiting_on_terminal(input, terminal, &mut written);
            let said = String::from_utf8(written).expect("a diagnostic is text");

            assert_eq!(said, expected, "{input:?} {terminal}");
        }
        assert_eq!(
            super::WAITING,
            "thinkthen: reading evidence from the terminal; \
             end it with Ctrl-D on a line of its own\n"
        );
    }

    #[test]
    fn a_closed_pipe_ends_the_write_quietly_and_any_other_failure_does_not() {
        assert!(matches!(
            write_line(Failing(ErrorKind::BrokenPipe), "{}"),
            Ok(false)
        ));
        assert!(matches!(
            write_line(Failing(ErrorKind::PermissionDenied), "{}"),
            Err(Failure::Output(_))
        ));
    }

    #[test]
    fn input_kind_follows_the_opened_handle_when_the_path_changes() {
        let path = std::env::temp_dir().join(format!(
            "thinkthen-opened-input-kind-{}",
            std::process::id()
        ));
        let _absent = fs::remove_file(&path);
        let _absent = fs::remove_dir(&path);

        fs::write(&path, "opened file").expect("a file");
        let file = File::open(&path).expect("the file opens");
        fs::remove_file(&path).expect("the old name leaves");
        fs::create_dir(&path).expect("a directory takes the name");
        let mut reader = opened(file).expect("the opened file stays a file");
        let mut text = String::new();
        reader.read_to_string(&mut text).expect("the file reads");
        assert_eq!(text, "opened file");

        fs::remove_dir(&path).expect("the replacement leaves");
        fs::create_dir(&path).expect("a directory");
        let directory = File::open(&path).expect("the directory opens");
        fs::remove_dir(&path).expect("the old name leaves");
        fs::write(&path, "replacement file").expect("a file takes the name");
        assert!(matches!(opened(directory), Err(Failure::InputDirectory)));
        fs::remove_file(path).expect("the fixture leaves");
    }

    /// A record with no end in sight is read no further than the refusal needs.
    ///
    /// The core refuses anything past the limit, so two bytes past it are
    /// enough: a record of exactly the limit may still arrive with `\r\n` after
    /// it. Without the bound a stream with no line feed would be read into
    /// memory whole, however long it ran.
    #[test]
    fn a_record_is_read_no_further_than_two_bytes_past_the_limit() {
        let endless = crate::core::MAX_RECORD_BYTES * 4;
        for streams in [true, false] {
            let reader = std::io::BufReader::new(std::io::repeat(b'x').take(endless as u64));
            let mut chunks = Chunks::new(reader, streams);
            let first = chunks.next().expect("one record").expect("bytes");

            assert_eq!(first.len(), crate::core::MAX_RECORD_BYTES + 2);
        }
    }

    /// A record of exactly the limit still arrives whole, however it was ended.
    #[test]
    fn a_record_of_exactly_the_limit_arrives_whole_with_its_ending() {
        let limit = crate::core::MAX_RECORD_BYTES;
        for (ending, expected) in [("", limit), ("\n", limit + 1), ("\r\n", limit + 2)] {
            let mut bytes = vec![b'x'; limit];
            bytes.extend_from_slice(ending.as_bytes());
            let mut chunks = Chunks::new(bytes.as_slice(), true);
            let first = chunks.next().expect("one record").expect("bytes");

            assert_eq!(first.len(), expected, "{ending:?}");
        }
    }

    #[test]
    fn a_stream_yields_one_line_at_a_time_and_a_document_yields_every_byte_once() {
        let read = |bytes: &[u8], streams: bool| {
            Chunks::new(bytes, streams)
                .map(|chunk| String::from_utf8_lossy(&chunk.expect("bytes")).into_owned())
                .collect::<Vec<_>>()
        };
        assert_eq!(read(b"a\nb\n", true), ["a\n", "b\n"]);
        assert_eq!(read(b"a\nb", true), ["a\n", "b"]);
        assert_eq!(read(b"", true), [""; 0]);
        assert_eq!(read(b"a\nb\n", false), ["a\nb\n"]);
        assert_eq!(read(b"", false), [""]);
    }
}
