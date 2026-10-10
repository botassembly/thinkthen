//! Authorized table handles use the same bounded grammar as the command.
use super::{Error, RawRecord, SourceRecord};
use crate::table::{Kind, Rows};
use std::io::Read;
use std::sync::Arc;

/// Delimited table grammar. Cells remain strings in header order.
#[derive(Clone, Copy, Debug)]
pub enum TableFormat {
    /// Comma-separated fields.
    Csv,
    /// Tab-separated fields.
    Tsv,
}

/// Bounded rows from one authorized handle, with physical source coordinates.
/// The reader opens no path and retains only its header and current record.
pub struct TableReader<R> {
    rows: Rows<R>,
    file: String,
}
impl<R> std::fmt::Debug for TableReader<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TableReader(<withheld>)")
    }
}
impl<R: Read> TableReader<R> {
    /// Admit the header and read rows using the maintained CSV/TSV grammar.
    /// # Errors
    /// Refuses invalid headers and unreadable input without disclosing content.
    pub fn new(file: impl Into<String>, reader: R, format: TableFormat) -> Result<Self, Error> {
        let kind = match format {
            TableFormat::Csv => Kind::Csv,
            TableFormat::Tsv => Kind::Tsv,
        };
        Ok(Self {
            rows: Rows::new(reader, kind).map_err(crate::table::ReadError::native)?,
            file: file.into(),
        })
    }
}
impl<R: Read> Iterator for TableReader<R> {
    type Item = Result<SourceRecord<RawRecord>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let row = self.rows.next()?;
        Some(
            row.map_err(crate::table::ReadError::native)
                .and_then(|record| {
                    let (first_line, last_line) = self
                        .rows
                        .position
                        .ok_or_else(|| Error::defect("table record lost its source coordinates"))?;
                    Ok(SourceRecord {
                        record: RawRecord(Arc::new(record)),
                        file: self.file.clone(),
                        first_line,
                        last_line,
                    })
                }),
        )
    }
}
