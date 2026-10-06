//! Named sources, file-local positions and bounded physical-line windows.

use std::collections::VecDeque;
use std::io::{BufRead, Read};

mod images;
mod snapshot;
pub(crate) use snapshot::Snapshot;
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
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(crate) struct Position {
    pub(crate) file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) first: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) images: Option<Vec<String>>,
    #[serde(skip)]
    pub(crate) source: usize,
    #[serde(skip)]
    pub(crate) located: bool,
}

pub(crate) enum Data {
    Bytes(Vec<u8>),
    Record(Record),
    Images(crate::public::ImageEvidence),
}

pub(crate) struct Item {
    pub(crate) at: usize,
    pub(crate) position: Option<Position>,
    pub(crate) data: Data,
}

struct TextSource {
    source: usize,
    file: Option<PathBuf>,
    chunks: Chunks<Box<dyn BufRead + Send>>,
    line: usize,
    located: bool,
}

enum Source {
    Images(crate::public::SourceItems),
    Attached(images::Attachment),
    Text(TextSource),
    Pending {
        file: PathBuf,
        source: usize,
        kind: Option<Kind>,
        located: bool,
    },
    Table(Box<Rows<Box<dyn BufRead + Send>>>),
}

struct Piece {
    position: Option<Position>,
    data: Data,
    advance: usize,
}

impl TextSource {
    fn next(&mut self, streams: bool, window: usize) -> Option<Result<Piece, Failure>> {
        let unit = self.chunks.next_unit(window)?;
        Some(unit.map(|(bytes, count)| self.piece(bytes, count, streams)))
    }

    fn piece(&mut self, bytes: Vec<u8>, count: usize, streams: bool) -> Piece {
        let first = self.line + 1;
        self.line += count;
        Piece {
            position: Some(Position {
                source: self.source,
                located: self.located,
                file: self
                    .file
                    .as_ref()
                    .map(|path| path.to_string_lossy().into_owned()),
                first: Some(first),
                last: Some(self.line.max(first)),
                images: None,
            }),
            data: Data::Bytes(bytes),
            advance: if streams { count } else { 1 },
        }
    }
}

impl Source {
    fn next(&mut self, streams: bool, window: usize) -> Option<Result<Piece, Failure>> {
        if let Self::Pending {
            file,
            source,
            kind,
            located,
        } = self
        {
            let reader = match edge::source(Some(file), std::io::empty()) {
                Ok(reader) => reader,
                Err(error) => return Some(Err(error)),
            };
            *self = match kind {
                Some(kind) => match Rows::new(reader, *kind) {
                    Ok(rows) => Self::Table(Box::new(rows)),
                    Err(error) => return Some(Err(error)),
                },
                None => Self::Text(TextSource {
                    source: *source,
                    file: Some(file.clone()),
                    chunks: Chunks::new(reader, streams),
                    line: 0,
                    located: *located,
                }),
            };
        }
        match self {
            Self::Images(items) => images::next(items),
            Self::Attached(attachment) => attachment.next(),
            Self::Pending { .. } => None,
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

/// Enumeration finishes before admission; file handles open one at a time.
pub(crate) struct Intake {
    sources: VecDeque<Source>,
    reading: Reading,
    window: usize,
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
        Self::prepare(common, reading, input, has_on, false).map(|(intake, _)| intake)
    }

    pub(crate) fn prepare(
        common: &Common,
        reading: &Reading,
        input: impl Read + Send + 'static,
        has_on: bool,
        snapshot: bool,
    ) -> Result<(Self, Option<Snapshot>), Failure> {
        if common.images() {
            return images::prepare(common, reading, input, has_on).map(|intake| (intake, None));
        }
        let window = window(common, has_on)?;
        let paths = crate::enumerate_files(&common.input)
            .map_err(|error| Failure::OpenInput(std::io::Error::other(error.to_string())))?;
        let kind = common
            .csv
            .then_some(Kind::Csv)
            .or(common.tsv.then_some(Kind::Tsv));
        if !common.input.is_empty() && !snapshot && kind.is_none() {
            let sources = paths
                .into_iter()
                .enumerate()
                .map(|(source, file)| Source::Pending {
                    file,
                    source,
                    kind,
                    located: common.located(),
                })
                .collect();
            return Ok((
                Self {
                    sources,
                    reading: reading.clone(),
                    window,
                    global: 0,
                },
                None,
            ));
        }
        let opened = if common.input.is_empty() {
            vec![(None, edge::source(None, input)?)]
        } else {
            paths
                .iter()
                .map(|path| {
                    edge::source(Some(path), std::io::empty())
                        .map(|reader| (Some(path.clone()), reader))
                })
                .collect::<Result<Vec<_>, _>>()?
        };
        let (opened, snapshot) = if snapshot {
            Snapshot::prepare(opened)?
        } else {
            (opened, None)
        };
        let kind = common
            .csv
            .then_some(Kind::Csv)
            .or(common.tsv.then_some(Kind::Tsv));
        let sources = opened
            .into_iter()
            .enumerate()
            .map(|(source, (file, reader))| match kind {
                Some(kind) => Rows::new(reader, kind).map(|rows| Source::Table(Box::new(rows))),
                None => Ok(Source::Text(TextSource {
                    source,
                    file,
                    chunks: Chunks::new(reader, reading.streams()),
                    line: 0,
                    located: common.located(),
                })),
            })
            .collect::<Result<VecDeque<_>, _>>()?;
        Ok((
            Self {
                sources,
                reading: reading.clone(),
                window,
                global: 0,
            },
            snapshot,
        ))
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
            // The bounded reader can cut an oversized unit and abandon its file.
            // Refuse it here, before blank skipping or worker prefetch of later files.
            if let Data::Bytes(bytes) = &piece.data
                && bytes.len() > crate::core::MAX_RECORD_BYTES
                && let Err(error) = self.reading.record(bytes)
            {
                self.sources.clear();
                return Some(Err(Placed::at(
                    Failure::record(error, self.reading.streams()),
                    at,
                )));
            }
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
        self.reading.skips(bytes)
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

/// Add flat source coordinates to an explicit located result.
#[derive(Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub(crate) struct SourceFields<'a> {
    file: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_line: Option<usize>,
}

pub(crate) fn source_members(
    line: &mut Option<String>,
    position: Option<&Position>,
) -> Result<(), Failure> {
    let (Some(line), Some(position)) = (line, position.filter(|p| p.located)) else {
        return Ok(());
    };
    let fields = crate::core::json_line(&SourceFields {
        file: position.file.as_deref(),
        first_line: position.first,
        last_line: position.last,
    })?;
    let fields = fields
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .ok_or(Failure::Defect("source coordinates are not an object"))?;
    let end = line
        .strip_suffix('}')
        .ok_or(Failure::Defect("a located row is not an object"))?;
    *line = format!("{end},{fields}}}");
    Ok(())
}

/// Retain source and original input beside a value without altering caller keys.
pub(crate) fn source_value(
    record: &impl Serialize,
    value: &str,
    position: &Position,
) -> Result<String, Failure> {
    let mut line = Some(format!(
        "{{\"input\":{},\"value\":{value}}}",
        crate::core::json_line(record)?
    ));
    source_members(&mut line, Some(position))?;
    line.ok_or(Failure::Defect("a located value has no carrier"))
}

/// Retain explicit image evidence beside full result facts.
pub(crate) fn image_input(
    line: &mut Option<String>,
    input: &crate::public::ImageEvidence,
) -> Result<(), Failure> {
    if let Some(line) = line {
        member(line, "input", input)?;
    }
    Ok(())
}
