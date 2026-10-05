//! Explicit file selection and located records outside the pure core.

use std::collections::VecDeque;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::Error;
use crate::core::{Framing, Reading};

/// The physical unit an explicit reader returns.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceUnit {
    /// One physical text line, excluding its final ending.
    #[default]
    Line,
    /// A nonoverlapping group of physical lines.
    Window,
    /// The entire original file, including its endings.
    File,
}

/// Controls shared by the command, native libraries and database readers.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReaderOptions {
    /// Line units by default.
    #[serde(default)]
    pub unit: SourceUnit,
    /// Required only for window units, and greater than zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<usize>,
}

impl ReaderOptions {
    /// Refuse contradictory or empty windows before opening content.
    ///
    /// # Errors
    /// Returns a usage error for an invalid unit/window combination.
    pub fn validate(self) -> Result<Self, Error> {
        match (self.unit, self.window) {
            (SourceUnit::Window, Some(n)) if n > 0 => Ok(self),
            (SourceUnit::Line | SourceUnit::File, None) => Ok(self),
            _ => Err(Error::usage(
                "reader window requires unit window and a positive whole number",
            )),
        }
    }
}

/// An original record with physical source coordinates. Metadata is never evidence.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceRecord<T> {
    /// Original record, without metadata inserted into its content.
    pub record: T,
    /// Explicitly selected file name.
    pub file: String,
    /// First physical line, one-based and inclusive.
    pub first_line: usize,
    /// Last physical line, one-based and inclusive.
    pub last_line: usize,
}

impl<T> std::fmt::Debug for SourceRecord<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SourceRecord")
            .field("record", &"<withheld>")
            .field("file", &self.file)
            .field("first_line", &self.first_line)
            .field("last_line", &self.last_line)
            .finish()
    }
}

impl<T: AsRef<str>> AsRef<str> for SourceRecord<T> {
    fn as_ref(&self) -> &str {
        self.record.as_ref()
    }
}

impl<T: AsRef<str>> super::Evidence for SourceRecord<T> {
    fn evidence(&self) -> &str {
        self.record.as_ref()
    }
}

impl<T: AsRef<str>> SourceRecord<T> {
    /// Map a local character span to physical lines, retaining host offsets.
    ///
    /// # Errors
    /// Refuses empty or out-of-range spans. Offsets count Unicode scalar values.
    pub fn span_lines(&self, start: usize, end: usize) -> Result<(usize, usize), Error> {
        let text = self.record.as_ref();
        if start >= end || end > text.chars().count() {
            return Err(Error::usage("source span is outside its record"));
        }
        let first = self.first_line + text.chars().take(start).filter(|&c| c == '\n').count();
        let last = self.first_line + text.chars().take(end - 1).filter(|&c| c == '\n').count();
        Ok((first, last))
    }
}

/// A bounded reader over one authorized host handle. It opens no filesystem path.
#[derive(Debug)]
pub struct FileReader<R> {
    chunks: crate::chunks::Chunks<R>,
    file: String,
    options: ReaderOptions,
    line: usize,
    stopped: bool,
}

impl<R: BufRead> FileReader<R> {
    /// Read an already authorized handle using the shared bounded framing.
    ///
    /// # Errors
    /// Returns a usage error for invalid reader options.
    pub fn new(file: impl Into<String>, reader: R, options: ReaderOptions) -> Result<Self, Error> {
        let options = options.validate()?;
        Ok(Self {
            chunks: crate::chunks::Chunks::new(reader, options.unit != SourceUnit::File),
            file: file.into(),
            options,
            line: 0,
            stopped: false,
        })
    }

    fn read_record(&mut self) -> Result<Option<SourceRecord<String>>, Error> {
        let Some(unit) = self.chunks.next_unit(self.options.window.unwrap_or(1)) else {
            return Ok(None);
        };
        let (bytes, count) = unit.map_err(|_| Error::local("source file could not be read"))?;
        let streams = self.options.unit != SourceUnit::File;
        let first_line = self.line + 1;
        self.line += count;
        let reading = Reading::new(
            if streams {
                Framing::Lines
            } else {
                Framing::Document
            },
            Vec::new(),
        )
        .map_err(Error::refused)?;
        let text = reading.as_it_arrived(&bytes).map_err(Error::refused)?;
        if text.len() > crate::core::MAX_RECORD_BYTES {
            return Err(Error::usage("source record is over the 16 MiB byte limit"));
        }
        Ok(Some(SourceRecord {
            record: text.to_owned(),
            file: self.file.clone(),
            first_line,
            last_line: self.line.max(first_line),
        }))
    }
}

impl<R: BufRead> Iterator for FileReader<R> {
    type Item = Result<SourceRecord<String>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }
        loop {
            match self.read_record() {
                Ok(Some(record))
                    if self.options.unit != SourceUnit::File && record.record.trim().is_empty() =>
                {
                    continue;
                }
                Ok(Some(record)) => return Some(Ok(record)),
                Ok(None) => {
                    self.stopped = true;
                    return None;
                }
                Err(error) => {
                    self.stopped = true;
                    return Some(Err(error));
                }
            }
        }
    }
}

/// Validate operands and enumerate a bounded manifest before reading any content.
///
/// Each operand retains its order; each folder's regular descendants sort by path.
/// Descendant symlinks are skipped. Explicit file symlinks follow existing policy.
///
/// # Errors
/// Returns local errors for missing or unsupported operands and oversized manifests.
pub fn enumerate_files(
    paths: impl IntoIterator<Item = impl AsRef<Path>>,
) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    let mut bytes = 0usize;
    for path in paths {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)
            .map_err(|_| Error::local("source operand could not be inspected"))?;
        if metadata.is_file() {
            add(&mut files, path.to_owned(), &mut bytes)?;
        } else if metadata.is_dir() {
            let mut folder = Vec::new();
            descend(path, &mut folder, &mut bytes)?;
            folder.sort();
            files.extend(folder);
        } else {
            return Err(Error::local(
                "source operand must be a regular file or folder",
            ));
        }
    }
    Ok(files)
}

fn add(files: &mut Vec<PathBuf>, path: PathBuf, bytes: &mut usize) -> Result<(), Error> {
    *bytes = bytes
        .checked_add(path.as_os_str().len() + 1)
        .ok_or_else(|| Error::local("source manifest exceeds 16 MiB"))?;
    if *bytes > crate::core::MAX_RECORD_BYTES {
        return Err(Error::local("source manifest exceeds 16 MiB"));
    }
    files.push(path);
    Ok(())
}

fn descend(path: &Path, files: &mut Vec<PathBuf>, bytes: &mut usize) -> Result<(), Error> {
    for entry in std::fs::read_dir(path)
        .map_err(|_| Error::local("source folder could not be enumerated"))?
    {
        let entry = entry.map_err(|_| Error::local("source folder could not be enumerated"))?;
        let kind = entry
            .file_type()
            .map_err(|_| Error::local("source entry could not be inspected"))?;
        if kind.is_symlink() {
            continue;
        }
        if kind.is_dir() {
            descend(&entry.path(), files, bytes)?;
        } else if kind.is_file() {
            add(files, entry.path(), bytes)?;
        } else {
            return Err(Error::local(
                "source folder contains an unsupported file kind",
            ));
        }
    }
    Ok(())
}

/// A native file selection that holds a bounded manifest and opens one file at a time.
#[derive(Debug)]
pub struct SourceRecords {
    paths: VecDeque<PathBuf>,
    options: ReaderOptions,
    current: Option<FileReader<std::io::BufReader<std::fs::File>>>,
    stopped: bool,
}

/// Select explicit paths, with no path inference from evidence strings.
///
/// # Errors
/// Returns errors for invalid options or enumeration failures before any file is read.
pub fn read_files(
    paths: impl IntoIterator<Item = impl AsRef<Path>>,
    options: ReaderOptions,
) -> Result<SourceRecords, Error> {
    let options = options.validate()?;
    Ok(SourceRecords {
        paths: enumerate_files(paths)?.into(),
        options,
        current: None,
        stopped: false,
    })
}

impl Iterator for SourceRecords {
    type Item = Result<SourceRecord<String>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }
        loop {
            if let Some(record) = self.current.as_mut().and_then(Iterator::next) {
                self.stopped = record.is_err();
                return Some(record);
            }
            self.current = None;
            let path = self.paths.pop_front()?;
            let opened = open_regular(&path);
            let file = match opened {
                Ok(file) => file,
                Err(_) => {
                    self.stopped = true;
                    return Some(Err(Error::local("source file could not be opened")));
                }
            };
            self.current = match FileReader::new(
                path.to_string_lossy(),
                std::io::BufReader::new(file),
                self.options,
            ) {
                Ok(reader) => Some(reader),
                Err(error) => {
                    self.stopped = true;
                    return Some(Err(error));
                }
            };
        }
    }
}

fn open_regular(path: &Path) -> std::io::Result<std::fs::File> {
    let file = crate::engine::usage::open_read(path)?;
    if !file.metadata()?.is_file() {
        return Err(std::io::Error::other("source is not regular"));
    }
    Ok(file)
}
