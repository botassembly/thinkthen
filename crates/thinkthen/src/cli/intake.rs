//! Named sources, file-local positions and bounded physical-line windows.

use std::collections::VecDeque;
use std::io::{BufRead, Read};
use std::path::PathBuf;

use serde::Serialize;

use crate::args::Common;
use crate::core::{Reading, Record};
use crate::edge::{self, Chunks};
use crate::failure::Failure;
use crate::schedule::Placed;
use crate::table::{Kind, Rows};

/// A display location, independent of the pipeline's global label.
#[derive(Clone, Serialize)]
pub(crate) struct Position {
    file: Option<String>,
    first: usize,
    last: usize,
}

pub(crate) enum Data {
    Bytes(Vec<u8>),
    Record(Record),
}

pub(crate) struct Item {
    pub(crate) at: usize,
    pub(crate) position: Option<Position>,
    pub(crate) data: Data,
}

struct TextSource {
    file: Option<PathBuf>,
    chunks: Chunks<Box<dyn BufRead + Send>>,
    line: usize,
}

enum Source {
    Text(TextSource),
    Table(Box<Rows<Box<dyn BufRead + Send>>>),
}

struct Piece {
    position: Option<Position>,
    data: Data,
    advance: usize,
}

impl TextSource {
    fn next(&mut self, streams: bool, window: usize) -> Option<Result<Piece, Failure>> {
        let bytes = self.chunks.next()?;
        Some(bytes.and_then(|bytes| self.join(bytes, streams, window)))
    }

    fn join(&mut self, mut bytes: Vec<u8>, streams: bool, window: usize) -> Result<Piece, Failure> {
        if !streams {
            let count = bytes.iter().filter(|&&b| b == b'\n').count()
                + usize::from(!bytes.is_empty() && !bytes.ends_with(b"\n"));
            return Ok(self.piece(bytes, count, false));
        }
        let mut count = 1;
        while count < window && bytes.len() <= crate::core::MAX_RECORD_BYTES + 2 {
            let Some(next) = self.chunks.next() else {
                break;
            };
            bytes.extend(next?);
            count += 1;
        }
        Ok(self.piece(bytes, count, true))
    }

    fn piece(&mut self, bytes: Vec<u8>, count: usize, streams: bool) -> Piece {
        let first = self.line + 1;
        self.line += count;
        Piece {
            position: Some(Position {
                file: self
                    .file
                    .as_ref()
                    .map(|path| path.to_string_lossy().into_owned()),
                first,
                last: self.line.max(first),
            }),
            data: Data::Bytes(bytes),
            advance: if streams { count } else { 1 },
        }
    }
}

impl Source {
    fn next(&mut self, streams: bool, window: usize) -> Option<Result<Piece, Failure>> {
        match self {
            Self::Table(rows) => rows.next().map(|row| {
                row.map(|record| Piece {
                    position: None,
                    data: Data::Record(record),
                    advance: 1,
                })
            }),
            Self::Text(text) => text.next(streams, window),
        }
    }
}

/// Every source is opened before iteration can admit its first item.
pub(crate) struct Intake {
    sources: VecDeque<Source>,
    reading: Reading,
    window: usize,
    windowed: bool,
    global: usize,
}

pub(crate) fn window(common: &Common, has_on: bool) -> Result<usize, Failure> {
    let Some(text) = common.window.as_deref() else {
        return Ok(1);
    };
    let count = text
        .parse::<usize>()
        .ok()
        .filter(|&n| n > 0 && text.bytes().all(|b| b.is_ascii_digit()))
        .ok_or(Failure::Usage(
            "--window takes a whole number of at least 1",
        ))?;
    if common.jsonl || common.csv || common.tsv || !common.field.is_empty() || has_on {
        return Err(Failure::Usage(
            "--window needs text lines without --jsonl, --csv, --tsv, --field or saved on",
        ));
    }
    Ok(count)
}

impl Intake {
    pub(crate) fn new(
        common: &Common,
        reading: &Reading,
        input: impl Read + Send + 'static,
        has_on: bool,
    ) -> Result<Self, Failure> {
        let window = window(common, has_on)?;
        let opened = if common.input.is_empty() {
            vec![(None, edge::source(None, input)?)]
        } else {
            common
                .input
                .iter()
                .map(|path| {
                    edge::source(Some(path), std::io::empty())
                        .map(|reader| (Some(path.clone()), reader))
                })
                .collect::<Result<Vec<_>, _>>()?
        };
        let kind = common
            .csv
            .then_some(Kind::Csv)
            .or(common.tsv.then_some(Kind::Tsv));
        let sources = opened
            .into_iter()
            .map(|(file, reader)| match kind {
                Some(kind) => Rows::new(reader, kind).map(|rows| Source::Table(Box::new(rows))),
                None => Ok(Source::Text(TextSource {
                    file,
                    chunks: Chunks::new(reader, reading.streams()),
                    line: 0,
                })),
            })
            .collect::<Result<VecDeque<_>, _>>()?;
        Ok(Self {
            sources,
            reading: reading.clone(),
            window,
            windowed: common.window.is_some(),
            global: 0,
        })
    }
}

impl Iterator for Intake {
    type Item = Result<Item, Placed>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let source = self.sources.front_mut()?;
            let Some(piece) = source.next(self.reading.streams(), self.window) else {
                self.sources.pop_front();
                continue;
            };
            let at = self.global + 1;
            let piece = match piece {
                Ok(piece) => piece,
                Err(error) => return Some(Err(Placed::at(error, at))),
            };
            self.global += piece.advance;
            if self.skips(&piece.data) {
                continue;
            }
            return Some(Ok(Item {
                at,
                position: piece.position,
                data: piece.data,
            }));
        }
    }
}

impl Intake {
    fn skips(&self, data: &Data) -> bool {
        let Data::Bytes(bytes) = data else {
            return false;
        };
        // An oversized joined whitespace window must be refused before admission.
        self.reading.skips(bytes)
            && (!self.windowed
                || self
                    .reading
                    .as_it_arrived(bytes)
                    .is_ok_and(|text| text.len() <= crate::core::MAX_RECORD_BYTES))
    }
}

/// Append edge metadata without reserializing existing result members.
fn member(line: &mut String, name: &str, value: &impl Serialize) -> Result<(), Failure> {
    let end = line
        .strip_suffix('}')
        .ok_or(Failure::Defect("a detailed row is not an object"))?;
    *line = format!("{end},\"{name}\":{}}}", crate::core::json_line(value)?);
    Ok(())
}

/// Add location only to a JSON result, keeping textual views untouched.
pub(crate) fn locate(
    line: &mut Option<String>,
    position: Option<&Position>,
) -> Result<(), Failure> {
    if let (Some(line), Some(position)) = (line, position) {
        member(line, "position", position)?;
    }
    Ok(())
}

/// Associate each default-document answer with its named source.
pub(crate) fn document(
    line: &mut Option<String>,
    position: Option<&Position>,
    details: bool,
) -> Result<(), Failure> {
    let (Some(line), Some(position)) = (line, position) else {
        return Ok(());
    };
    if details {
        member(line, "input_file", &position.file)?;
    } else {
        *line = format!(
            "{{\"input_file\":{},\"value\":{line}}}",
            crate::core::json_line(&position.file)?
        );
    }
    Ok(())
}
