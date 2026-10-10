//! Logical framing over authorized handles and the native explicit path manifest.
use super::{RequestFraming, RequestSource};
use crate::{
    Error, FileReader, RawRecord, ReaderMedia, ReaderOptions, SourceRecord, SourceUnit,
    TableFormat, TableReader,
};
use std::io::BufRead;

pub(super) fn validate(
    framing: Option<RequestFraming>,
    reading: ReaderOptions,
    media: ReaderMedia,
) -> Result<(), Error> {
    reading.validate()?;
    let Some(framing) = framing else {
        return Ok(());
    };
    let unit = if matches!(framing, RequestFraming::Document) {
        SourceUnit::File
    } else {
        SourceUnit::Line
    };
    if media != ReaderMedia::Text || reading.unit != unit || reading.window.is_some() {
        return Err(Error::usage(
            "explicit source framing requires text and matching file or line units without a window",
        ));
    }
    Ok(())
}

/// Decode one authorized text handle with explicit logical framing and physical locations.
/// This reader opens no path and retains only the current record and any table header.
pub struct RequestSourceReader<R> {
    mode: Mode<R>,
    framing: RequestFraming,
    stopped: bool,
}
enum Mode<R> {
    Text(FileReader<R>),
    Table(Box<TableReader<R>>),
}
impl<R> std::fmt::Debug for RequestSourceReader<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RequestSourceReader(<withheld>)")
    }
}
impl<R: BufRead> RequestSourceReader<R> {
    /// Decode an already authorized handle using shared bounded text/table readers.
    /// # Errors
    /// Refuses contradictory physical controls or invalid table headers.
    pub fn new(
        file: impl Into<String>,
        reader: R,
        framing: RequestFraming,
        reading: ReaderOptions,
    ) -> Result<Self, Error> {
        validate(Some(framing), reading, ReaderMedia::Text)?;
        let file = file.into();
        let mode = match framing {
            RequestFraming::Csv => {
                Mode::Table(Box::new(TableReader::new(file, reader, TableFormat::Csv)?))
            }
            RequestFraming::Tsv => {
                Mode::Table(Box::new(TableReader::new(file, reader, TableFormat::Tsv)?))
            }
            _ => Mode::Text(FileReader::new(file, reader, reading)?),
        };
        Ok(Self {
            mode,
            framing,
            stopped: false,
        })
    }
}
impl<R: BufRead> Iterator for RequestSourceReader<R> {
    type Item = Result<SourceRecord<RawRecord>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }
        let row = match &mut self.mode {
            Mode::Table(reader) => reader.next()?,
            Mode::Text(reader) => text_record(reader, self.framing)?,
        };
        self.stopped = row.is_err();
        Some(row)
    }
}

fn text_record<R: BufRead>(
    reader: &mut FileReader<R>,
    framing: RequestFraming,
) -> Option<Result<SourceRecord<RawRecord>, Error>> {
    loop {
        let source = match reader.next()? {
            Ok(source) => source,
            Err(error) => return Some(Err(error)),
        };
        if !matches!(framing, RequestFraming::Document) && source.record.trim().is_empty() {
            continue;
        }
        let original = if matches!(framing, RequestFraming::Jsonl) {
            RawRecord::json(&source.record)
        } else {
            RawRecord::text(&source.record)
        };
        return Some(original.map(|record| SourceRecord {
            record,
            file: source.file,
            first_line: source.first_line,
            last_line: source.last_line,
        }));
    }
}

pub(super) fn records<'a>(
    source: &RequestSource,
    reading: crate::RecordReading,
    controls: crate::CallOptions<'a>,
    annotate: bool,
    rank: bool,
    attachment_remaining: Option<usize>,
) -> Result<super::composition::Inputs<'a>, Error> {
    let Some(framing) = source.framing else {
        let items = super::composition::read_source(source, attachment_remaining)?;
        return Ok(super::transport::source_records(
            reading, items, controls, annotate, rank,
        ));
    };
    let mut paths = crate::enumerate_files(&source.paths)?.into_iter();
    let options = source.reading;
    let mut current: Option<RequestSourceReader<std::io::BufReader<std::fs::File>>> = None;
    let mut remaining = crate::core::MAX_RECORD_BYTES;
    let mut stopped = false;
    Ok(Box::new(std::iter::from_fn(move || {
        if stopped {
            return None;
        }
        let result =
            next_source(&mut paths, &mut current, framing, options, controls).and_then(|source| {
                source
                    .map(|source| compose(source, &reading, annotate, rank, &mut remaining))
                    .transpose()
            });
        stopped = result.is_err();
        result.transpose()
    })))
}

fn next_source(
    paths: &mut impl Iterator<Item = std::path::PathBuf>,
    current: &mut Option<RequestSourceReader<std::io::BufReader<std::fs::File>>>,
    framing: RequestFraming,
    options: ReaderOptions,
    controls: crate::CallOptions<'_>,
) -> Result<Option<SourceRecord<RawRecord>>, Error> {
    controls.admission()?;
    loop {
        if let Some(row) = current.as_mut().and_then(Iterator::next) {
            return row.map(Some);
        }
        let Some(path) = paths.next() else {
            return Ok(None);
        };
        controls.admission()?;
        let name = crate::public::files::source_name(&path)?;
        let file = crate::public::files::open_regular(&path)
            .map_err(|_| Error::local("source file could not be opened"))?;
        *current = Some(RequestSourceReader::new(
            name,
            std::io::BufReader::new(file),
            framing,
            options,
        )?);
    }
}
fn compose(
    source: SourceRecord<RawRecord>,
    reading: &crate::RecordReading,
    annotate: bool,
    rank: bool,
    remaining: &mut usize,
) -> Result<crate::RecordInput<crate::QuestionInput>, Error> {
    if rank {
        *remaining = remaining
            .checked_sub(source.record.retained_bytes()?)
            .ok_or_else(super::transport::rank_budget_error)?;
    }
    let location =
        crate::SourceLocation::new(source.file, Some(source.first_line), Some(source.last_line))?;
    let row = if annotate && let Some(text) = source.record.literal() {
        super::composition::document_row(reading, text, true)?
    } else {
        super::composition::compose_original(reading, source.record)?
    };
    super::composition::located_row(row, Some(location))
}
