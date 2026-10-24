//! Operation-owned, disk-backed records and position-indexed outcomes.

use crate::files::{FileError, local};
use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use tempfile::NamedTempFile;

type RecordReader = Box<dyn Iterator<Item = Result<(usize, Value), FileError>>>;

pub(crate) struct Records {
    pub(crate) file: NamedTempFile,
    pub(crate) count: usize,
    positions: NamedTempFile,
    _paths: crate::files::cleanup::TemporaryPaths,
}

impl Records {
    pub(crate) fn new() -> Result<Self, FileError> {
        let mut paths = crate::files::cleanup::TemporaryPaths::default();
        Ok(Self {
            file: paths.temporary(None).map_err(local)?,
            count: 0,
            positions: paths.temporary(None).map_err(local)?,
            _paths: paths,
        })
    }

    pub(crate) fn push(&mut self, row: usize, value: Value) -> Result<(), FileError> {
        serde_json::to_writer(&mut self.file, &(row, value)).map_err(local)?;
        self.file.write_all(b"\n").map_err(local)?;
        self.positions
            .write_all(&(row as u64).to_le_bytes())
            .map_err(local)?;
        self.count += 1;
        Ok(())
    }

    pub(crate) fn reader(&self) -> Result<RecordReader, FileError> {
        let mut file = self.file.reopen().map_err(local)?;
        file.rewind().map_err(local)?;
        Ok(Box::new(BufReader::new(file).lines().map(|line| {
            serde_json::from_str(&line.map_err(local)?).map_err(local)
        })))
    }

    pub(crate) fn source_row(&mut self, position: usize) -> Result<usize, FileError> {
        let ordinal = position
            .checked_sub(1)
            .ok_or_else(|| FileError::Local("engine returned an invalid input position".into()))?;
        self.positions
            .seek(SeekFrom::Start((ordinal as u64) * 8))
            .map_err(local)?;
        let mut row = [0; 8];
        self.positions.read_exact(&mut row).map_err(local)?;
        usize::try_from(u64::from_le_bytes(row)).map_err(local)
    }
}

pub(crate) struct Answers {
    data: NamedTempFile,
    index: NamedTempFile,
    _paths: crate::files::cleanup::TemporaryPaths,
}

impl Answers {
    pub(crate) fn new() -> Result<Self, FileError> {
        let mut paths = crate::files::cleanup::TemporaryPaths::default();
        let mut data = paths.temporary(None).map_err(local)?;
        data.write_all(b"\n").map_err(local)?; // Offset zero means no outcome.
        Ok(Self {
            data,
            index: paths.temporary(None).map_err(local)?,
            _paths: paths,
        })
    }

    pub(crate) fn put(&mut self, row: usize, cells: &[Value]) -> Result<(), FileError> {
        let offset = self
            .data
            .as_file_mut()
            .seek(SeekFrom::End(0))
            .map_err(local)?;
        serde_json::to_writer(&mut self.data, cells).map_err(local)?;
        self.data.write_all(b"\n").map_err(local)?;
        self.index
            .seek(SeekFrom::Start((row as u64) * 8))
            .map_err(local)?;
        self.index.write_all(&offset.to_le_bytes()).map_err(local)
    }

    pub(crate) fn get(&mut self, row: usize) -> Result<Option<Vec<Value>>, FileError> {
        self.index
            .seek(SeekFrom::Start((row as u64) * 8))
            .map_err(local)?;
        let mut offset = [0; 8];
        match self.index.read_exact(&mut offset) {
            Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            result => result.map_err(local)?,
        }
        let offset = u64::from_le_bytes(offset);
        if offset == 0 {
            return Ok(None);
        }
        self.data.seek(SeekFrom::Start(offset)).map_err(local)?;
        let mut line = String::new();
        BufReader::new(self.data.as_file_mut())
            .read_line(&mut line)
            .map_err(local)?;
        serde_json::from_str(&line).map(Some).map_err(local)
    }
}
