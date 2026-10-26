//! Incremental record framing across ordered file and stdin sources.
use super::options::{usage, value};
use crate::{Error, records::Record};
use clap::ArgMatches;
use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        match text {
            "text" => Ok(Self::Text),
            "lines" => Ok(Self::Lines),
            "jsonl" => Ok(Self::Jsonl),
            "csv" => Ok(Self::Csv),
            "tsv" => Ok(Self::Tsv),
            "xlsx" => Ok(Self::Xlsx),
            _ => Err(super::options::unknown_value(
                "from",
                text,
                &["text", "lines", "jsonl", "csv", "tsv", "xlsx"],
            )),
        }
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
}
impl Input {
    pub(super) fn resolve(function: &str, args: &ArgMatches) -> Result<Self, Error> {
        let mut files = args
            .get_many::<String>("files")
            .map(|v| v.cloned().collect::<Vec<_>>())
            .unwrap_or_default();
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
    pub(super) fn next(&mut self) -> Result<Option<(Record, Vec<u8>)>, Error> {
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
            }
            let Some((input, format, origin)) = self.current.as_mut() else {
                continue;
            };
            self.origin.clone_from(origin);
            let format = *format;
            self.format = format;
            let mut bytes = Vec::new();
            let length = if format == Format::Text {
                input.read_to_end(&mut bytes)
            } else {
                input.read_until(b'\n', &mut bytes)
            }
            .map_err(|_| super::local(&format!("input {} could not be read", self.origin)))?;
            if format == Format::Text {
                self.current = None;
            } else if length == 0 {
                self.current = None;
                continue;
            }
            if format != Format::Text && bytes.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let record = if format == Format::Jsonl {
                std::str::from_utf8(&bytes).map_or_else(|_| Record::invalid(), Record::from_json)
            } else if format == Format::Lines {
                std::str::from_utf8(&bytes)
                    .map_or_else(|_| Record::invalid(), |s| Record::text(super::evidence(s)))
            } else {
                Record::from_utf8(&bytes)
            };
            return Ok(Some((record, bytes)));
        }
    }
}

/// One bounded reader lets cancellation finish while a pipe remains open.
pub(super) struct StreamInput {
    receiver: std::sync::mpsc::Receiver<Result<FramedRecord, Error>>,
    cancel: crate::Cancel,
    origins: Vec<(u64, String)>,
    read: u64,
}
type FramedRecord = (Record, Vec<u8>, String);
pub(super) enum InputEvent {
    Record(Record, Vec<u8>),
    Pending,
    Finished,
}

impl StreamInput {
    pub(super) fn new(mut input: Input, cancel: crate::Cancel) -> Result<Self, Error> {
        let receiver = spawn(
            move || {
                input.next().map(|record| {
                    record.map(|(record, bytes)| (record, bytes, input.origin.clone()))
                })
            },
            cancel.clone(),
        )?;
        Ok(Self {
            receiver,
            cancel,
            origins: Vec::new(),
            read: 0,
        })
    }
    pub(super) fn next(&mut self) -> Result<Option<(Record, Vec<u8>)>, Error> {
        loop {
            match self.poll()? {
                InputEvent::Record(record, bytes) => return Ok(Some((record, bytes))),
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
        let (record, bytes, path) = match self
            .receiver
            .recv_timeout(std::time::Duration::from_millis(20))
        {
            Ok(item) => item?,
            Err(RecvTimeoutError::Timeout) => return Ok(InputEvent::Pending),
            Err(RecvTimeoutError::Disconnected) => return Ok(InputEvent::Finished),
        };
        self.read += 1;
        if self
            .origins
            .last()
            .is_none_or(|(_, previous)| previous != &path)
        {
            self.origins.push((self.read, path));
        }
        Ok(InputEvent::Record(record, bytes))
    }
    pub(super) fn origin(&self, position: u64) -> &str {
        self.origins
            .iter()
            .rev()
            .find(|(first, _)| *first <= position)
            .map_or("stdin", |(_, path)| path.as_str())
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
