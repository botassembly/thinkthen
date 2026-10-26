//! Text projections preserve original bytes while machine output retains typed rows.
use super::{local, output_error};
use crate::files::TemporaryPaths;
use crate::results::{AnyRow, Outcome};
use crate::{Cancel, Code, Error};
use std::io::{self, Read, Seek, SeekFrom, Write};
use tempfile::NamedTempFile;

pub(super) struct Originals {
    data: NamedTempFile,
    index: NamedTempFile,
    _paths: TemporaryPaths,
}
impl Originals {
    pub(super) fn new() -> Result<Self, Error> {
        let mut paths = TemporaryPaths::default();
        let data = paths
            .temporary(None)
            .map_err(|_| local("original records could not be retained"))?;
        let index = paths
            .temporary(None)
            .map_err(|_| local("original positions could not be retained"))?;
        Ok(Self {
            data,
            index,
            _paths: paths,
        })
    }
    pub(super) fn push(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let result = (|| -> io::Result<()> {
            let offset = self.data.seek(SeekFrom::End(0))?;
            self.data.write_all(bytes)?;
            self.index.seek(SeekFrom::End(0))?;
            self.index.write_all(&offset.to_le_bytes())?;
            self.index.write_all(&(bytes.len() as u64).to_le_bytes())
        })();
        result.map_err(|_| local("original records could not be retained"))
    }
    pub(super) fn write(&mut self, position: u64, cancel: &Cancel) -> Result<(), Error> {
        let offset = position
            .checked_sub(1)
            .and_then(|n| n.checked_mul(16))
            .ok_or_else(|| {
                Error::from_contract(
                    Code::InternalBug,
                    "the engine returned an invalid record position",
                )
            })?;
        let result = (|| -> io::Result<()> {
            self.index.seek(SeekFrom::Start(offset))?;
            let mut number = [0; 8];
            self.index.read_exact(&mut number)?;
            self.data
                .seek(SeekFrom::Start(u64::from_le_bytes(number)))?;
            self.index.read_exact(&mut number)?;
            let length = u64::from_le_bytes(number);
            let copied = io::copy(
                &mut self.data.as_file_mut().take(length),
                &mut io::stdout().lock(),
            )?;
            if copied != length {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
            Ok(())
        })();
        result.map_err(|error| output_error(error, cancel))
    }
}

pub(super) fn row(row: &AnyRow, json: bool, cancel: &Cancel) -> Result<(), Error> {
    let mut output = io::stdout().lock();
    if json || row.failure().is_some() || matches!(row, AnyRow::Annotate(_)) {
        serde_json::to_writer(&mut output, row).map_err(|error| match error.io_error_kind() {
            Some(kind) => output_error(io::Error::new(kind, error), cancel),
            None => local("the row could not be encoded"),
        })?;
        return writeln!(output).map_err(|error| output_error(error, cancel));
    }
    let value = serde_json::to_value(row).map_err(|_| local("the row could not be encoded"))?;
    if value.get("status").and_then(serde_json::Value::as_str) == Some("unsure") {
        return writeln!(output, "unsure").map_err(|error| output_error(error, cancel));
    }
    let Some(answer) = value.get("answer") else {
        return Ok(());
    };
    let result = match row {
        AnyRow::Find(_) => match answer.get("text").and_then(serde_json::Value::as_str) {
            Some(text) => writeln!(output, "{text}"),
            None => Ok(()),
        },
        AnyRow::Tag(_) | AnyRow::Recognize(_) | AnyRow::Relate(_) => {
            for value in answer.as_array().into_iter().flatten() {
                (if let Some(text) = value.as_str() {
                    writeln!(output, "{text}")
                } else {
                    writeln!(output, "{value}")
                })
                .map_err(|error| output_error(error, cancel))?;
            }
            Ok(())
        }
        _ => match answer.as_str() {
            Some(text) => writeln!(output, "{text}"),
            None => writeln!(output, "{answer}"),
        },
    };
    result.map_err(|error| output_error(error, cancel))
}

pub(super) fn keeps(row: &AnyRow) -> bool {
    matches!(row, AnyRow::Filter(row) if matches!(row.outcome(), Outcome::Answered { answer: true, .. }))
        || matches!(row, AnyRow::Rank(row) if matches!(row.outcome(), Outcome::Answered { .. }))
}
