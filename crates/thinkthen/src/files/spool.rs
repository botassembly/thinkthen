//! Operation-owned, disk-backed records and position-indexed outcomes.

use crate::files::{FileError, local};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};

type RecordReader = Box<dyn Iterator<Item = Result<(usize, Value), FileError>>>;

pub(crate) struct Records {
    pub(crate) file: BufWriter<File>,
    pub(crate) count: usize,
    positions: BufWriter<File>,
}

impl Records {
    pub(crate) fn new() -> Result<Self, FileError> {
        Ok(Self {
            file: BufWriter::new(tempfile::tempfile().map_err(local)?),
            count: 0,
            positions: BufWriter::new(tempfile::tempfile().map_err(local)?),
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

    pub(crate) fn reader(&mut self) -> Result<RecordReader, FileError> {
        self.file.flush().map_err(local)?;
        let mut file = self.file.get_ref().try_clone().map_err(local)?;
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
        self.positions
            .get_mut()
            .read_exact(&mut row)
            .map_err(local)?;
        usize::try_from(u64::from_le_bytes(row)).map_err(local)
    }
}

pub(crate) struct Answers {
    data: BufWriter<File>,
    index: BufWriter<File>,
}

impl Answers {
    pub(crate) fn new() -> Result<Self, FileError> {
        let mut data = BufWriter::new(tempfile::tempfile().map_err(local)?);
        data.write_all(b"\n").map_err(local)?; // Offset zero means no outcome.
        Ok(Self {
            data,
            index: BufWriter::new(tempfile::tempfile().map_err(local)?),
        })
    }

    pub(crate) fn put(&mut self, row: usize, cells: &[Value]) -> Result<(), FileError> {
        let offset = self.data.stream_position().map_err(local)?;
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
        match self.index.get_mut().read_exact(&mut offset) {
            Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            result => result.map_err(local)?,
        }
        let offset = u64::from_le_bytes(offset);
        if offset == 0 {
            return Ok(None);
        }
        self.data.seek(SeekFrom::Start(offset)).map_err(local)?;
        let mut line = String::new();
        BufReader::new(self.data.get_mut())
            .read_line(&mut line)
            .map_err(local)?;
        serde_json::from_str(&line).map(Some).map_err(local)
    }
}
