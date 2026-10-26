//! One strict decoder supplies both admission and byte-preserving output.

use crate::Cancel;
use crate::files::{FileError, admission::check_cancel};

const MAX_RECORD_BYTES: usize = 64 * 1024 * 1024;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
pub(crate) struct CsvRow {
    pub(crate) raw: Vec<u8>,
    pub(crate) ending: Vec<u8>,
    pub(crate) fields: Vec<String>,
    pub(crate) number: usize,
    pub(crate) line: usize,
}

pub(crate) struct CsvReader<R> {
    reader: BufReader<R>,
    delimiter: u8,
    cancel: Cancel,
    row: usize,
    line: usize,
    pub(crate) bom: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Start,
    Plain,
    Quoted,
    Closed,
}

impl<R: Read> CsvReader<R> {
    pub(crate) fn new(reader: R, delimiter: u8, cancel: &Cancel) -> Result<Self, FileError> {
        check_cancel(cancel)?;
        let mut reader = BufReader::new(reader);
        let bom = reader
            .fill_buf()
            .map_err(super::local)?
            .starts_with(b"\xef\xbb\xbf");
        if bom {
            reader.consume(3);
        }
        Ok(Self {
            reader,
            delimiter,
            cancel: cancel.clone(),
            row: 0,
            line: 1,
            bom,
        })
    }

    pub(crate) fn next(&mut self) -> Result<Option<CsvRow>, FileError> {
        let mut raw = Vec::new();
        let mut field = Vec::new();
        let mut fields = Vec::new();
        let mut state = State::Start;
        let start = self.line;
        let mut ending = Vec::new();
        loop {
            if raw.len().is_multiple_of(8192) {
                check_cancel(&self.cancel)?;
            }
            let mut byte = [0];
            if self.reader.read(&mut byte).map_err(super::local)? == 0 {
                if raw.is_empty() && fields.is_empty() {
                    return Ok(None);
                }
                if state == State::Quoted {
                    return Err(self.bad(start, "unclosed quoted field; close its double quote"));
                }
                break;
            }
            let value = byte.first().copied().unwrap_or_default();
            if state != State::Quoted && (value == b'\n' || value == b'\r') {
                ending.push(value);
                if value == b'\r' {
                    if self.reader.read(&mut byte).map_err(super::local)? == 0
                        || byte.first().copied().unwrap_or_default() != b'\n'
                    {
                        return Err(self.bad(
                            start,
                            "a bare carriage return ends a row; save with LF or CRLF line endings",
                        ));
                    }
                    ending.push(b'\n');
                }
                self.line += 1;
                break;
            }
            if raw.len() == MAX_RECORD_BYTES {
                return Err(self.bad(start, "record exceeds the 64 MiB limit"));
            }
            raw.push(value);
            if value == b'\n' {
                self.line += 1;
            }
            match state {
                State::Quoted if value == b'"' => state = State::Closed,
                State::Quoted => field.push(value),
                State::Closed if value == b'"' => { field.push(value); state = State::Quoted; }
                State::Start if value == b'"' => state = State::Quoted,
                State::Start | State::Plain | State::Closed if value == self.delimiter => {
                    if fields.len() == crate::files::xlsx::MAX_COLUMN - 1 {
                        return Err(self.bad(start, "record exceeds the 16,384 field limit"));
                    }
                    fields.push(self.text(std::mem::take(&mut field), start)?);
                    state = State::Start;
                }
                State::Closed => return Err(self.bad(start, "text follows a closing quote; put the separator immediately after it")),
                State::Plain if value == b'"' => return Err(self.bad(start, "a quote appears inside an unquoted field; quote the whole field and double internal quotes")),
                State::Start | State::Plain => { field.push(value); state = State::Plain; }
            }
        }
        fields.push(self.text(field, start)?);
        self.row += 1;
        Ok(Some(CsvRow {
            raw,
            ending,
            fields,
            number: self.row,
            line: start,
        }))
    }

    fn text(&self, bytes: Vec<u8>, line: usize) -> Result<String, FileError> {
        String::from_utf8(bytes)
            .map_err(|_| self.bad(line, "input is not UTF-8; save as CSV UTF-8 or TSV UTF-8"))
    }

    fn bad(&self, line: usize, cause: &str) -> FileError {
        FileError::Invalid(crate::core::diagnostic::requirement(
            &format!("CSV/TSV row {} (line {line})", self.row + 1),
            "valid UTF-8 delimited fields",
            cause,
        ))
    }
}

pub(crate) fn encode(text: &str, delimiter: u8) -> Vec<u8> {
    if text
        .bytes()
        .any(|b| b == delimiter || matches!(b, b'"' | b'\r' | b'\n'))
    {
        format!("\"{}\"", text.replace('"', "\"\"")).into_bytes()
    } else {
        text.as_bytes().to_vec()
    }
}
