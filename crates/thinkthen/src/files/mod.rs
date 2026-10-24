//! Read tabular evidence and append judgments without replacing existing cells.

mod admission;
mod cleanup;
mod csv;
mod lock;
mod operation;
mod spool;
mod xlsx;

pub use cleanup::force_cleanup;

use crate::{Error, Family};
use std::path::PathBuf;

/// The explicitly selected table encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// UTF-8 comma-separated records.
    Csv,
    /// UTF-8 tab-separated records.
    Tsv,
    /// An Open XML workbook.
    Xlsx,
}

/// Where a completed file operation publishes its output.
#[derive(Debug, Clone)]
pub enum FileOutput {
    /// Create a new file; absence selects the input's `.thinkthen` sibling.
    NewFile(Option<PathBuf>),
    /// Replace the source only after retaining an exclusive backup.
    InPlace,
}

/// A caller's file operation, independent of client routing and credentials.
#[derive(Debug)]
pub struct FileJob {
    input: PathBuf,
    format: FileFormat,
    sheet: Option<String>,
    columns: Vec<String>,
    into: Option<String>,
    output: FileOutput,
}

/// A completely published file and its outcome failure, if any.
#[derive(Debug)]
pub struct FileReceipt {
    /// The file that was published.
    pub output: PathBuf,
    /// The original input retained before in-place publication.
    pub backup: Option<PathBuf>,
    /// The first engine outcome failure in input order, including annotations.
    pub first_failure: Option<Error>,
}

/// A file operation that could not complete its requested publication.
#[derive(Debug, thiserror::Error)]
pub enum FileError {
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
    pub fn code(&self) -> &str {
        match self {
            Self::Engine(error) => error.kind().code(),
            Self::Invalid(_) => "usage.file",
            Self::Local(_) => "local.file",
        }
    }

    /// The existing command error family.
    #[must_use]
    pub fn family(&self) -> Family {
        match self {
            Self::Engine(error) => error.kind().family(),
            Self::Invalid(_) => Family::Usage,
            Self::Local(_) => Family::Local,
        }
    }
}

impl FileJob {
    /// Admit a file operation's explicit framing and selected column names.
    ///
    /// # Errors
    /// Returns `usage.file` for missing or repeated column selections.
    pub fn new(
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
            return Err(FileError::Invalid(
                "unsupported workbook format; save as .xlsx or CSV before running ThinkThen".into(),
            ));
        }
        if columns.is_empty() {
            return Err(FileError::Invalid(
                "name at least one evidence column with --column".into(),
            ));
        }
        let mut unique = std::collections::BTreeSet::new();
        for column in &columns {
            if column.is_empty() || !unique.insert(column) {
                return Err(FileError::Invalid(
                    "evidence column names must be nonempty and not repeated".into(),
                ));
            }
        }
        if format != FileFormat::Xlsx && sheet.is_some() {
            return Err(FileError::Invalid("--sheet requires --excel".into()));
        }
        Ok(Self {
            input,
            format,
            sheet,
            columns,
            into,
            output,
        })
    }
}

fn local(error: impl std::fmt::Display) -> FileError {
    FileError::Local(error.to_string())
}

fn headers(headers: &[String], selected: &[String]) -> Result<(), FileError> {
    let mut unique = std::collections::BTreeSet::new();
    if headers.is_empty() || headers.iter().any(|h| h.is_empty() || !unique.insert(h)) {
        return Err(FileError::Invalid(
            "column headers must be nonempty and unique; name every column once".into(),
        ));
    }
    for name in selected {
        if !unique.contains(name) {
            return Err(FileError::Invalid(format!(
                "evidence column `{name}` is missing; choose a header present in the input"
            )));
        }
    }
    Ok(())
}

fn cell_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => String::new(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Bool(b) => {
            if *b {
                "TRUE".into()
            } else {
                "FALSE".into()
            }
        }
        value => value.to_string(),
    }
}
