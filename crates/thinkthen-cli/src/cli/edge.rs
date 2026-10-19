//! The process boundary, which every value from outside crosses once.

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, IsTerminal as _, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;


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

use crate::{CacheSettings, Engine, EngineBuilder, Facts, ResolvedSettings, Tally};
use crate::failure::Failure;
use std::sync::{Mutex, OnceLock};

/// The process observation and the one captured builder transferred to its engine.
pub(crate) struct Environment {
    base_url: Option<String>,
    backend: Option<String>,
    batch: Option<String>,
    max_request_bytes: Option<String>,
    cache: CacheSettings,
    builder: Mutex<Option<EngineBuilder>>,
    resolved: OnceLock<ResolvedSettings>,
    engine: Mutex<Option<Engine>>,
    tally: Tally,
    native_facts: Mutex<Option<Facts>>,
    key: KeySnapshot,
    ca_bundle: Option<PathBuf>,
    retry_wait_ms: Option<u64>,
    input_pause_ms: Option<u64>,
    pub(super) sigint_ack: Option<PathBuf>,
    pub(super) cancel: crate::cli::interrupt::Signal,
}

impl Default for Environment {
    fn default() -> Self {
        let builder = Engine::builder();
        Self {
            base_url: None, backend: None, batch: None, max_request_bytes: None,
            cache: builder.cache_settings(), builder: Mutex::new(Some(builder)),
            resolved: OnceLock::new(), engine: Mutex::new(None), tally: Tally::new(),
            native_facts: Mutex::new(None), key: KeySnapshot::default(),
            ca_bundle: None, retry_wait_ms: None, input_pause_ms: None, sigint_ack: None,
            cancel: crate::cli::interrupt::Signal::default(),
        }
    }
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("Environment")
            .field("base_url", &self.base_url.as_ref().map(|_| "<withheld>"))
            .field("backend", &self.backend.as_ref().map(|_| "<withheld>"))
            .field("key", &self.key).finish_non_exhaustive()
    }
}

impl Environment {
    pub(crate) fn settle_native(&self, facts: &Facts) {
        if let Ok(mut settled) = self.native_facts.lock() { *settled = Some(facts.clone()); }
    }
    pub(crate) fn native_call_id(&self) -> Option<crate::CallId> {
        self.native_facts.lock().ok()
            .and_then(|facts| facts.as_ref().and_then(|facts| facts.call_id().cloned()))
    }
    pub(crate) fn read() -> Result<Self, Failure> {
        let mut builder = EngineBuilder::from_config()?;
        let mut cache = builder.cache_settings();
        if let Some(path) = read("THINKTHEN_CACHE") {
            builder = builder.cache_at(&path)?;
            cache = builder.cache_settings();
            cache.cache_enabled_source = "environment";
            cache.cache_path_source = "environment";
        }
        if let Some(rate) = read("THINKTHEN_REQUESTS_PER_MINUTE") {
            let invalid = || Failure::Usage("THINKTHEN_REQUESTS_PER_MINUTE takes a whole number from 1 to 60000");
            let value = rate.parse::<u32>().ok()
                .filter(|_| rate.bytes().all(|byte| byte.is_ascii_digit())).ok_or_else(invalid)?;
            builder = builder.requests_per_minute(value).map_err(|_| invalid())?;
        }
        if let Some(total) = read("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL") {
            let value = total.parse::<u64>().ok()
                .filter(|_| total.bytes().all(|byte| byte.is_ascii_digit()))
                .ok_or(Failure::Usage("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL takes a whole number of 0 or more"))?;
            builder = builder.max_estimated_input_tokens_total(Some(value));
        }
        Ok(Self {
            base_url: read("THINKTHEN_BASE_URL"), backend: read("THINKTHEN_BACKEND"),
            batch: read("THINKTHEN_BATCH"), max_request_bytes: read("THINKTHEN_MAX_REQUEST_BYTES"),
            cache, builder: Mutex::new(Some(builder)),
            ca_bundle: read("THINKTHEN_CA_BUNDLE").map(PathBuf::from),
            retry_wait_ms: test_only("THINKTHEN_TEST_RETRY_WAIT_MS").and_then(|text| text.parse().ok()),
            input_pause_ms: test_only("THINKTHEN_TEST_INPUT_PAUSE_MS").and_then(|text| text.parse().ok()),
            sigint_ack: test_only("THINKTHEN_TEST_SIGINT_ACK").map(PathBuf::from),
            ..Self::default()
        })
    }
    pub(crate) fn cache(&self) -> Option<&Path> { self.cache.cache_path.as_deref() }
    pub(crate) const fn cache_is_platform_default(&self) -> bool { self.cache.cache_is_platform_default }
    pub(crate) const fn default_cache_enabled(&self) -> bool { self.cache.cache_enabled }
    pub(crate) const fn cache_bytes(&self) -> u64 { self.cache.cache_target_bytes }
    pub(crate) const fn configuration_shared(&self) -> bool { self.cache.configuration_shared }
    pub(crate) fn named_cache(&self) -> bool { self.cache.cache_path_source == "environment" }
    pub(crate) fn batch(&self) -> Option<&str> { self.batch.as_deref() }
    pub(crate) fn request_size_selected(&self, flag: Option<&str>) -> bool {
        flag.is_some() || self.max_request_bytes.is_some()
    }
    pub(crate) fn request_size(&self, flag: Option<&str>) -> Result<usize, Failure> {
        let Some(value) = flag.or(self.max_request_bytes.as_deref()) else { return Ok(96_000); };
        value.parse::<usize>().ok()
            .filter(|&number| number > 0 && value.bytes().all(|byte| byte.is_ascii_digit()))
            .ok_or(Failure::Usage(if flag.is_some() {
                "--max-request-bytes takes a whole number of at least 1"
            } else { "THINKTHEN_MAX_REQUEST_BYTES takes a whole number of at least 1" }))
    }
    pub(crate) fn set_request_size(&self, size: usize) -> Result<(), Failure> {
        self.update_builder(|builder| builder.max_request_bytes(size))
    }
    pub(crate) fn warn_request_size(&self, engine: &Engine, size: usize) -> Result<(), Failure> {
        if engine.is_built_in() && size > 96_000 {
            writeln!(io::stderr().lock(),
                "thinkthen: warning: max_request_bytes {size} is above the default of 96000; the built-in backend refuses a request over 65536 input tokens")
                .map_err(Failure::Output)?;
        }
        Ok(())
    }
    pub(crate) fn retry_wait(&self) -> Duration {
        self.retry_wait_ms.map_or(Duration::from_secs(1), Duration::from_millis)
    }
    pub(crate) fn input_pause(&self) -> Option<Duration> { self.input_pause_ms.map(Duration::from_millis) }
    pub(crate) const fn cancel(&self) -> &crate::cli::interrupt::Signal { &self.cancel }
    pub(crate) fn take_builder(&self) -> Result<EngineBuilder, Failure> {
        self.builder.lock().map_err(|_| Failure::Defect("command settings lock failed"))?
            .take().ok_or(Failure::Defect("command settings were already consumed"))
    }
    pub(crate) fn update_builder(&self, update: impl FnOnce(EngineBuilder) -> Result<EngineBuilder, crate::Error>) -> Result<(), Failure> {
        let builder = update(self.take_builder()?)?;
        self.restore_builder(builder)
    }
    pub(crate) fn restore_builder(&self, builder: EngineBuilder) -> Result<(), Failure> {
        *self.builder.lock().map_err(|_| Failure::Defect("command settings lock failed"))? = Some(builder);
        Ok(())
    }
    pub(crate) fn observe_engine(&self, engine: &Engine) {
        if let Ok(mut observed) = self.engine.lock() { *observed = Some(engine.clone()); }
    }
    pub(crate) fn tally(&self) -> &Tally { &self.tally }
    pub(crate) fn snapshot(&self) -> crate::cli::facts::Snapshot {
        let engine = self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let facts = engine.as_ref().map_or_else(|| self.tally.facts(), |engine| self.tally.facts_with_engine(engine));
        crate::cli::facts::Snapshot { facts,
            counts: engine.as_ref().map_or(crate::Counters::ZERO, Engine::usage),
            http_time: engine.as_ref().and_then(Engine::http_time) }
    }
    pub(crate) fn finish_usage(&self) -> Option<Option<crate::UsageFailure>> {
        let engine = self.engine.lock().ok()?;
        let engine = engine.as_ref()?;
        (engine.finish_usage_status() == crate::UsagePersistence::Failed)
            .then(|| engine.usage_failure())
    }
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
        Some(path) => opened(open_read(path).map_err(Failure::OpenInput)?),
        None => Ok(Box::new(BufReader::new(reader))),
    }
}

#[cfg(not(windows))]
fn open_read(path: &Path) -> io::Result<File> { File::open(path) }

#[cfg(windows)]
fn open_read(path: &Path) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt as _;
    std::fs::OpenOptions::new().read(true).custom_flags(0x0200_0000).open(path)
}

/// Classify and buffer the handle that `source` opened.
fn opened<'a>(file: File) -> Result<Box<dyn BufRead + Send + 'a>, Failure> {
    if file.metadata().map_err(Failure::OpenInput)?.is_dir() {
        Err(Failure::InputDirectory)
    } else {
        Ok(Box::new(BufReader::new(file)))
    }
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
        Err(error) if Failure::closed_output(&error) => Ok(false),
        Err(error) => Err(Failure::Output(error)),
        Ok(()) => Ok(true),
    }
}

#[cfg(test)]
mod deadline_tests;

#[cfg(test)]
mod tests {
    //! Stays: direct I/O fault and secrecy checks cover replaced handles, broken pipes and bounded incremental reads.
    use super::{Environment, open_read, opened, write_line};
    use crate::{FileReader, ReaderOptions, SourceUnit};
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
        let directory = open_read(&path).expect("the directory opens");
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
    #[ignore = "large-input boundary runs in the release suite"]
    fn release_only_a_record_is_read_no_further_than_two_bytes_past_the_limit() {
        let limit = crate::RawRecord::max_bytes();
        for unit in [SourceUnit::Line, SourceUnit::File] {
            let mut source = std::io::BufReader::with_capacity(1, std::io::repeat(b'x').take((limit * 4) as u64));
            {
                let mut reader = FileReader::new("input", &mut source, ReaderOptions { unit, window: None }).expect("reader");
                assert!(reader.next().expect("refused record").is_err());
            }
            assert_eq!(source.get_ref().limit(), (limit * 4 - limit - 2) as u64);
        }
    }

    #[test]
    #[ignore = "large-input boundary runs in the release suite"]
    fn release_only_a_record_of_exactly_the_limit_arrives_whole_with_its_ending() {
        let limit = crate::RawRecord::max_bytes();
        for ending in ["", "\n", "\r\n"] {
            let mut bytes = vec![b'x'; limit];
            bytes.extend_from_slice(ending.as_bytes());
            let mut reader = FileReader::new("input", bytes.as_slice(), ReaderOptions::default()).expect("reader");
            let first = reader.next().expect("one record").expect("text");
            assert_eq!(first.record.len(), limit, "{ending:?}");
        }
    }

    #[test]
    fn a_stream_yields_one_line_at_a_time_and_a_document_yields_every_byte_once() {
        let read = |bytes: &[u8], unit| {
            FileReader::new("input", bytes, ReaderOptions { unit, window: None }).expect("reader")
                .map(|record| record.expect("text").record).collect::<Vec<_>>()
        };
        assert_eq!(read(b"a\nb\n", SourceUnit::Line), ["a", "b"]);
        assert_eq!(read(b"a\nb", SourceUnit::Line), ["a", "b"]);
        assert!(read(b"", SourceUnit::Line).is_empty());
        assert_eq!(read(b"a\nb\n", SourceUnit::File), ["a\nb\n"]);
        assert_eq!(read(b"", SourceUnit::File), [""]);
    }
}
