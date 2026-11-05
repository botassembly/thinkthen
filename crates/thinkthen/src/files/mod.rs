//! Read tabular field and append judgments without replacing existing cells.

mod admission;
mod cells;
mod cleanup;
mod csv;
mod lock;
mod operation;
mod spool;
mod xlsx;
mod xlsx_dates;
mod xlsx_formats;

pub(crate) use cleanup::TemporaryPaths;
pub(crate) use cleanup::force_cleanup;

use crate::core::diagnostic;
use crate::{Error, Family};
use std::path::PathBuf;

pub(crate) const MAX_RECORD_BYTES: usize = 64 * 1024 * 1024;
pub(crate) const RECORD_LIMIT_MESSAGE: &str = "record exceeds the 64 MiB limit";
pub(crate) fn record_limit() -> FileError {
    FileError::Invalid(RECORD_LIMIT_MESSAGE.into())
}

/// The explicitly selected table encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FileFormat {
    /// UTF-8 comma-separated records.
    Csv,
    /// UTF-8 tab-separated records.
    Tsv,
    /// An Open XML workbook.
    Xlsx,
}

/// Where a completed file operation publishes its output.
#[derive(Debug, Clone)]
pub(crate) enum FileOutput {
    /// Create a new file; absence selects the input's `.thinkthen` sibling.
    NewFile(Option<PathBuf>),
    /// Replace the source only after retaining an exclusive backup.
    InPlace,
}

/// A caller's file operation, independent of client routing and credentials.
#[derive(Debug)]
pub(crate) struct FileJob {
    input: PathBuf,
    format: FileFormat,
    sheet: Option<String>,
    columns: Vec<String>,
    into: Option<String>,
    output: FileOutput,
    details: bool,
}

/// The most failed rows a receipt names; `failed` counts them all.
pub(crate) const NAMED_FAILURES: usize = 20;

/// Table outcomes and the file published when the operation can safely complete.
#[derive(Debug)]
pub(crate) struct FileReceipt {
    /// The published file, absent when failed filter rows prevent replacement.
    pub(crate) output: Option<PathBuf>,
    /// The original input retained before in-place publication.
    pub(crate) backup: Option<PathBuf>,
    /// The first failed rows in input order, by source row number, at most
    /// [`NAMED_FAILURES`]. A row's failure is its first, including annotations.
    pub(crate) failures: Vec<(usize, Error)>,
    /// Rows with an answered or unsure outcome, including partial annotations.
    pub(crate) answered: u64,
    /// Rows whose outcome is not sure.
    pub(crate) unsure: u64,
    /// Failed rows by error family, including nested annotation failures.
    pub(crate) failed: Vec<(Family, u64)>,
}

/// A file operation that could not complete its requested publication.
#[derive(Debug, thiserror::Error)]
pub(crate) enum FileError {
    /// The original engine admission or final call error.
    #[error("{0}")]
    Engine(#[from] Error),
    /// Invalid table arguments or data.
    #[error("{}", crate::escape_diagnostic(.0))]
    Invalid(String),
    /// File access, preservation or publication failed.
    #[error("{}", crate::escape_diagnostic(.0))]
    Local(String),
}

impl FileError {
    /// Stable failure code without repeating engine mappings.
    #[must_use]
    pub(crate) fn code(&self) -> &str {
        match self {
            Self::Engine(error) => error.code().as_str(),
            Self::Invalid(_) => crate::Code::UsageQuestion.as_str(),
            Self::Local(_) => crate::Code::LocalIo.as_str(),
        }
    }

    /// The existing command error family.
    #[must_use]
    pub(crate) fn family(&self) -> Family {
        match self {
            Self::Engine(error) => error.code().family(),
            Self::Invalid(_) => Family::Usage,
            Self::Local(_) => Family::Local,
        }
    }
}

impl FileJob {
    /// Admit a file operation's explicit framing and selected column names.
    ///
    /// # Errors
    /// Returns `usage.question` for missing or repeated column selections.
    pub(crate) fn new(
        input: PathBuf,
        format: FileFormat,
        sheet: Option<String>,
        columns: Vec<String>,
        into: Option<String>,
        output: FileOutput,
    ) -> Result<Self, FileError> {
        if input
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                extension.eq_ignore_ascii_case("xls") || extension.eq_ignore_ascii_case("ods")
            })
        {
            return Err(FileError::Invalid(diagnostic::requirement(
                "input.format",
                "xlsx, CSV or TSV",
                "unsupported workbook format; save as .xlsx or CSV before running ThinkThen",
            )));
        }
        if columns.is_empty() {
            return Err(FileError::Invalid(diagnostic::requirement(
                "columns",
                "at least one field column with --column",
                "an empty selection",
            )));
        }
        let mut unique = std::collections::BTreeSet::new();
        for column in &columns {
            if column.is_empty() || !unique.insert(column) {
                return Err(FileError::Invalid(diagnostic::requirement(
                    "columns",
                    "nonempty, distinct field column names",
                    "an empty or repeated name",
                )));
            }
        }
        if format != FileFormat::Xlsx && sheet.is_some() {
            return Err(FileError::Invalid(diagnostic::requirement(
                "sheet",
                "xlsx framing",
                "CSV or TSV framing",
            )));
        }
        Ok(Self {
            input,
            format,
            sheet,
            columns,
            into,
            output,
            details: false,
        })
    }

    /// Add status, probability, error, detail and source columns after each answer.
    ///
    /// By default each question writes one answer column.
    #[must_use]
    pub(crate) fn details(mut self, details: bool) -> Self {
        self.details = details;
        self
    }
}

fn local(error: impl std::fmt::Display) -> FileError {
    FileError::Local(error.to_string())
}

fn headers(headers: &[String], selected: &[String]) -> Result<(), FileError> {
    let mut unique = std::collections::BTreeSet::new();
    if headers.is_empty() || headers.iter().any(|h| h.is_empty() || !unique.insert(h)) {
        return Err(FileError::Invalid(diagnostic::requirement(
            "headers",
            "nonempty, unique column headers",
            "empty or repeated column headers",
        )));
    }
    for name in selected {
        if !unique.contains(name) {
            return Err(FileError::Invalid(diagnostic::unknown(
                "column",
                name,
                &headers.iter().map(String::as_str).collect::<Vec<_>>(),
            )));
        }
    }
    Ok(())
}
