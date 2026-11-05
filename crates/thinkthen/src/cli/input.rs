//! Incremental record framing across ordered file and stdin sources.
use super::options::{usage, value};
use crate::{Error, records::Record};
use clap::ArgMatches;
use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader, Read};

const UTF8_PREAMBLE: &[u8] = b"\xef\xbb\xbf";

/// Decode one transport preamble while retaining original bytes for projections.
pub(super) fn source_body(bytes: &[u8], stream_start: bool) -> &[u8] {
    if stream_start {
        bytes.strip_prefix(UTF8_PREAMBLE).unwrap_or(bytes)
    } else {
        bytes
    }
}

pub(super) fn read_record(
    input: &mut dyn BufRead,
    format: Format,
    stream_start: bool,
) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    if stream_start {
        for expected in UTF8_PREAMBLE {
            if input.fill_buf()?.first() != Some(expected) {
                break;
            }
            bytes.push(*expected);
            input.consume(1);
        }
    }
    let preamble = if bytes == UTF8_PREAMBLE {
        UTF8_PREAMBLE.len()
    } else {
        0
    };
    let remaining = crate::files::MAX_RECORD_BYTES + 1 + preamble - bytes.len();
    let mut reader = input.take(remaining as u64);
    if format == Format::Text {
        reader.read_to_end(&mut bytes)?;
    } else {
        reader.read_until(b'\n', &mut bytes)?;
    }
    Ok(bytes)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub(super) enum Format {
    Text,
    Lines,
    Jsonl,
    Csv,
    Tsv,
    Xlsx,
}
impl Format {
    pub(super) fn parse(text: &str) -> Result<Self, Error> {
        super::options::parse_choice("from", text)
    }
    pub(super) const fn table(self) -> bool {
        matches!(self, Self::Csv | Self::Tsv | Self::Xlsx)
    }
}

pub(super) struct Input {
    pub sources: Vec<(String, Format)>,
    pending: VecDeque<(String, Format)>,
    current: Option<(Box<dyn BufRead + Send>, Format, String)>,
    pub origin: String,
    pub format: Format,
    pub selection: Vec<crate::Pointer>,
    literal: bool,
    stream_start: bool,
}
impl Input {
    pub(super) fn resolve(function: &str, args: &ArgMatches) -> Result<Self, Error> {
        if let Some(text) = args.get_one::<String>("text") {
            // Explicit literal input fixes framing to one text record, ahead
            // of inherited THINKTHEN_FROM, without opening a file or stdin.
            let origin = "--text".to_owned();
            return Ok(Self {
                sources: vec![(origin.clone(), Format::Text)],
                pending: VecDeque::new(),
                current: Some((
                    Box::new(io::Cursor::new(text.as_bytes().to_vec())),
                    Format::Text,
                    origin,
                )),
                origin: String::new(),
                format: Format::Text,
                selection: super::question_options::field(args)?,
                literal: true,
                stream_start: false,
            });
        }
        Self::from_files(function, args, files(args))
    }
    pub(super) fn from_files(
        function: &str,
        args: &ArgMatches,
        mut files: Vec<String>,
    ) -> Result<Self, Error> {
        if files.is_empty() {
            files.push("-".into());
        }
        if files.iter().filter(|p| *p == "-").count() > 1 {
            return Err(usage("files", "stdin at most once", "repeated stdin"));
        }
        let selected = value(args, "from").map(|v| Format::parse(&v)).transpose()?;
        let default = match function {
            "filter" | "rank" | "annotate" => Format::Lines,
            "run" => Format::Jsonl,
            _ => Format::Text,
        };
        let sources: Vec<_> = files
            .into_iter()
            .map(|path| {
                let inferred = std::path::Path::new(&path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .and_then(|e| Format::parse(&e.to_ascii_lowercase()).ok())
                    .filter(|f| *f == Format::Jsonl || f.table());
                (path, selected.or(inferred).unwrap_or(default))
            })
            .collect();
        let selection = super::question_options::field(args)?;
        let selected_field = !selection.is_empty();
        if selected_field
            && sources
                .iter()
                .any(|(_, format)| matches!(format, Format::Lines))
        {
            return Err(usage(
                "field",
                "JSON or table framing; omit --field for lines",
                "text framing",
            ));
        }
        let tables = sources.iter().filter(|(_, f)| f.table()).count();
        if tables != 0 && (tables != sources.len() || sources.len() != 1) {
            return Err(usage(
                "files",
                "one table source without streams",
                "multiple or mixed sources",
            ));
        }
        Ok(Self {
            pending: sources.clone().into(),
            sources,
            current: None,
            origin: String::new(),
            format: default,
            selection,
            literal: false,
            stream_start: false,
        })
    }
    pub(super) fn table(&self) -> bool {
        self.sources.first().is_some_and(|(_, f)| f.table())
    }
    pub(super) fn check(&self, function: &str) -> Result<(), Error> {
        if function != "filter"
            && (self.sources.len() != 1
                || self
                    .sources
                    .first()
                    .is_some_and(|(_, f)| *f != Format::Text))
        {
            return Err(usage(
                "check",
                "text framing and exactly one source",
                "other framing or multiple sources",
            ));
        }
        if self.table() {
            return Err(usage("check", "stream framing", "table framing"));
        }
        Ok(())
    }
    pub(super) fn next(&mut self) -> Result<Option<(Record, Vec<u8>, bool)>, Error> {
        loop {
            if self.current.is_none() {
                let Some((path, format)) = self.pending.pop_front() else {
                    return Ok(None);
                };
                let input: Box<dyn BufRead + Send> = if path == "-" {
                    Box::new(BufReader::new(io::stdin()))
                } else {
                    Box::new(BufReader::new(std::fs::File::open(&path).map_err(
                        |_| super::local(&format!("input {path} could not be opened")),
                    )?))
                };
                self.current = Some((input, format, path));
                self.stream_start = true;
            }
            let Some((input, format, origin)) = self.current.as_mut() else {
                continue;
            };
            self.origin.clone_from(origin);
            let format = *format;
            self.format = format;
            let stream_start = std::mem::take(&mut self.stream_start);
            let bytes = read_record(input.as_mut(), format, stream_start)
                .map_err(|_| super::local(&format!("input {} could not be read", self.origin)))?;
            let body = source_body(&bytes, stream_start);
            if body.len() > crate::files::MAX_RECORD_BYTES {
                return Err(Error::new(
                    crate::ErrorKind::Question,
                    crate::files::record_limit().to_string(),
                ));
            }
            if format == Format::Text {
                self.current = None;
            } else if bytes.is_empty() {
                self.current = None;
                continue;
            }
            if format != Format::Text && body.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let record = if format == Format::Jsonl
                || (format == Format::Text && !self.selection.is_empty())
            {
                std::str::from_utf8(body).map_or_else(|_| Record::invalid(), Record::from_json)
            } else {
                std::str::from_utf8(body).map_or_else(
                    |_| Record::invalid(),
                    |s| Record::text(if self.literal { s } else { super::field(s) }),
                )
            };
            return Ok(Some((
                record.select_field(self.selection.clone()),
                bytes,
                stream_start,
            )));
        }
    }
}

/// The positional input files, in order.
pub(super) fn files(args: &ArgMatches) -> Vec<String> {
    args.get_many::<String>("files")
        .map(|v| v.cloned().collect())
        .unwrap_or_default()
}

/// Source boundaries retain the file for each original record position.
#[derive(Default)]
pub(super) struct Origins(Vec<(u64, String)>);
impl Origins {
    pub(super) fn record(&mut self, position: u64, path: String) {
        if self.0.last().is_none_or(|(_, previous)| previous != &path) {
            self.0.push((position, path));
        }
    }
    pub(super) fn at(&self, position: u64) -> &str {
        self.0
            .iter()
            .rev()
            .find(|(first, _)| *first <= position)
            .map_or("stdin", |(_, path)| path.as_str())
    }
}

/// One bounded reader lets cancellation finish while a pipe remains open.
pub(super) struct StreamInput {
    receiver: std::sync::mpsc::Receiver<Result<FramedRecord, Error>>,
    cancel: crate::Cancel,
    origins: Origins,
    read: u64,
    pub format: Format,
}
type FramedRecord = (Record, Vec<u8>, String, Format, bool);
pub(super) enum InputEvent {
    Record(Record, Vec<u8>, bool),
    Pending,
    Finished,
}

impl StreamInput {
    pub(super) fn new(mut input: Input, cancel: crate::Cancel) -> Result<Self, Error> {
        let format = input.format;
        let receiver = spawn(
            move || {
                input.next().map(|record| {
                    record.map(|(record, bytes, start)| {
                        (record, bytes, input.origin.clone(), input.format, start)
                    })
                })
            },
            cancel.clone(),
        )?;
        Ok(Self {
            receiver,
            cancel,
            origins: Origins::default(),
            read: 0,
            format,
        })
    }
    pub(super) fn next(&mut self) -> Result<Option<(Record, Vec<u8>, bool)>, Error> {
        loop {
            match self.poll()? {
                InputEvent::Record(record, bytes, start) => {
                    return Ok(Some((record, bytes, start)));
                }
                InputEvent::Finished => return Ok(None),
                InputEvent::Pending => {}
            }
        }
    }
    pub(super) fn poll(&mut self) -> Result<InputEvent, Error> {
        use std::sync::mpsc::RecvTimeoutError;
        if self.cancel.is_cancelled() {
            return Err(crate::retry::cancelled());
        }
        let (record, bytes, path, format, start) = match self
            .receiver
            .recv_timeout(std::time::Duration::from_millis(20))
        {
            Ok(item) => item?,
            Err(RecvTimeoutError::Timeout) => return Ok(InputEvent::Pending),
            Err(RecvTimeoutError::Disconnected) => return Ok(InputEvent::Finished),
        };
        self.read += 1;
        self.format = format;
        self.origins.record(self.read, path);
        Ok(InputEvent::Record(record, bytes, start))
    }
    pub(super) fn origin(&self, position: u64) -> &str {
        self.origins.at(position)
    }
}

/// The process owns blocking readers; bounded queues stop read-ahead when sinks close.
pub(super) fn spawn<T: Send + 'static>(
    mut read: impl FnMut() -> Result<Option<T>, Error> + Send + 'static,
    cancel: crate::Cancel,
) -> Result<std::sync::mpsc::Receiver<Result<T, Error>>, Error> {
    let (send, receiver) = std::sync::mpsc::sync_channel(1);
    // An upstream writer may hold stdin open. The command process closes this
    // reader when main returns; pipeline and backend workers still join normally.
    std::thread::Builder::new()
        .spawn(move || {
            while !cancel.is_cancelled() {
                let item = match read() {
                    Ok(Some(item)) => Ok(item),
                    Ok(None) => break,
                    Err(error) => Err(error),
                };
                let failed = item.is_err();
                if send.send(item).is_err() || failed {
                    break;
                }
            }
        })
        .map_err(|_| super::local("the input reader could not be started"))?;
    Ok(receiver)
}

pub(super) fn receive<T>(
    receiver: &std::sync::mpsc::Receiver<Result<T, Error>>,
    cancel: &crate::Cancel,
) -> Result<Option<T>, Error> {
    use std::sync::mpsc::RecvTimeoutError;
    loop {
        if cancel.is_cancelled() {
            return Err(crate::retry::cancelled());
        }
        match receiver.recv_timeout(std::time::Duration::from_millis(20)) {
            Ok(item) => return item.map(Some),
            Err(RecvTimeoutError::Disconnected) => return Ok(None),
            Err(RecvTimeoutError::Timeout) => {}
        }
    }
}
