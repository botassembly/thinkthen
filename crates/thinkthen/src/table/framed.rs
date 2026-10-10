//! Complete-row descriptors reuse the stream decoder and retain only its header.
use super::{Error, Kind, ReadError, Rows};
use crate::core::{MAX_RECORD_BYTES, Record};
use std::io::Cursor;

pub(crate) struct Framed {
    rows: Rows<Cursor<String>>,
}

impl Framed {
    pub(crate) fn new(kind: Kind) -> Self {
        Self {
            rows: Rows::empty(Cursor::new(String::new()), kind),
        }
    }

    pub(crate) fn needs_header(&self) -> bool {
        self.rows.header.is_none()
    }

    pub(crate) fn push(&mut self, text: String) -> Result<Option<Record>, crate::Error> {
        if text.len() > MAX_RECORD_BYTES + 2 {
            return Err(self.rows.too_large().native());
        }
        let header = self.needs_header();
        self.rows.reader = Cursor::new(text);
        self.rows.start = 0;
        self.rows.end = 0;
        self.rows.eof = false;
        self.rows.parser.reset();
        if !header {
            // reset restores csv-core's initial BOM handling. Consume an ignored
            // blank line before real input so a data-cell BOM remains data.
            self.rows.parser.read_field(b"\n", &mut [0; 1]);
        }
        let row = self.rows.parsed_row().map_err(ReadError::native)?;
        if self.rows.parsed_row().map_err(ReadError::native)?.is_some() {
            return Err(crate::Error::usage(
                "table session descriptor contains more than one logical row",
            ));
        }
        // Release the current encoded row before yielding its decoded original.
        self.rows.reader = Cursor::new(String::new());
        if header {
            let row = row.ok_or_else(|| ReadError::Table(Error::Empty(self.rows.kind)).native())?;
            self.rows.read_header(row).map_err(ReadError::native)?;
            Ok(None)
        } else {
            row.map(|row| self.rows.record(row).map_err(ReadError::native))
                .transpose()
        }
    }

    pub(crate) fn finish(&self) -> Result<(), crate::Error> {
        if self.needs_header() {
            Err(ReadError::Table(Error::Empty(self.rows.kind)).native())
        } else {
            Ok(())
        }
    }
}
