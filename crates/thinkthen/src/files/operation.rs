//! One admitted operation owns its snapshot, stream and publication.

use crate::contract_driver::driver::{Driver, Event};
use crate::files::admission::{check_cancel, copy};
use crate::files::{
    FileError, FileFormat, FileJob, FileOutput, FileReceipt, NAMED_FAILURES,
    cells::{self, Cell},
    csv, headers, local, spool, xlsx,
};
use crate::records::Record;
use crate::{Cancel, Code, Engine, Error, ErrorKind, OffsetUnit, Question, Request};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

impl FileJob {
    #[cfg(feature = "command")]
    pub(crate) fn execute_for_command(
        &self,
        request: &Request,
        client: &Engine,
        cancel: &Cancel,
    ) -> Result<FileReceipt, FileError> {
        if !request.records().is_empty() {
            return Err(invalid(
                Code::UsageTable,
                "a file operation's request must not contain initial records",
            ));
        }
        let (request, narrowed) = table_request(request, &self.columns)?;
        let request = &request;
        let prepared = request.question();
        let annotation = matches!(prepared, Question::Annotate(_));
        let filter = matches!(prepared, Question::Filter(_));
        let names: Vec<String> = if annotation {
            if self.into.is_some() {
                return Err(invalid(
                    Code::UsageColumn,
                    "annotate uses its question names; omit --into",
                ));
            }
            match prepared {
                Question::Annotate(question) => question.questions().keys().cloned().collect(),
                _ => Vec::new(),
            }
        } else {
            vec![self.into.clone().filter(|s| !s.is_empty()).ok_or_else(|| {
                invalid(Code::UsageColumn, "name the answer column with --into NAME")
            })?]
        };
        let _lock = if matches!(self.output, FileOutput::InPlace) {
            Some(crate::files::lock::SourceLock::acquire(&self.input)?)
        } else {
            None
        };
        let output = self.destination()?;
        let parent = output
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        if matches!(self.output, FileOutput::InPlace) {
            if fs::symlink_metadata(&self.input)
                .map_err(local)?
                .file_type()
                .is_symlink()
            {
                return Err(invalid(
                    Code::UsageOutput,
                    "in-place input cannot be a symbolic link; use its real path",
                ));
            }
            check_owner(&self.input)?;
        } else if output.try_exists().map_err(local)? {
            return Err(invalid(
                Code::UsageOutput,
                "output already exists; choose a new --output path",
            ));
        }
        let mut temporary_paths = crate::files::cleanup::TemporaryPaths::default();
        let mut snapshot = temporary_paths.temporary(None).map_err(local)?;
        let mut source = File::open(&self.input).map_err(local)?;
        let permissions = source.metadata().map_err(local)?.permissions();
        copy(&mut source, &mut snapshot, cancel)?;
        drop(source);
        snapshot.flush().map_err(local)?;
        let digest = hash(snapshot.path(), cancel)?;
        let mut records = spool::Records::new()?;
        let mut input_headers = Vec::new();
        let sheet = if self.format == FileFormat::Xlsx {
            let sheet = xlsx::admit(
                snapshot.path(),
                self.sheet.as_deref(),
                &self.columns,
                cancel,
                |row, values| records.push(row, self.field(values, narrowed)),
            )?;
            input_headers.clone_from(&sheet.headers);
            Some(sheet)
        } else {
            let mut reader = csv::CsvReader::new(
                File::open(snapshot.path()).map_err(local)?,
                self.delimiter(),
                cancel,
            )?;
            let header = reader.next()?.ok_or_else(|| {
                invalid(
                    Code::UsageTable,
                    "input has no header row; add column names",
                )
            })?;
            headers(&header.fields, &self.columns).map_err(|error| match error {
                FileError::Invalid(code, message) => {
                    FileError::Invalid(code, format!("CSV/TSV row 1 {message}"))
                }
                error => error,
            })?;
            let positions = self
                .columns
                .iter()
                .map(|name| {
                    header
                        .fields
                        .iter()
                        .position(|h| h == name)
                        .ok_or_else(|| invalid(Code::UsageColumn, "selected column is missing"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            input_headers = header.fields;
            while let Some(row) = reader.next()? {
                check_cancel(cancel)?;
                if row.fields.len() != input_headers.len() {
                    return Err(FileError::Invalid(
                        Code::UsageTable,
                        crate::core::diagnostic::requirement(
                            &format!("CSV/TSV row {} (line {})", row.number, row.line),
                            &format!("{} fields", input_headers.len()),
                            &format!(
                                "{} fields; quote a field containing a separator or remove the extra separator",
                                row.fields.len()
                            ),
                        ),
                    ));
                }
                let values = positions
                    .iter()
                    .map(|p| row.fields.get(*p).cloned().unwrap_or_default())
                    .collect();
                records.push(row.number, self.field(values, narrowed))?;
            }
            None
        };
        let mut generated = Vec::new();
        for name in &names {
            generated.push(name.clone());
            if self.details {
                for suffix in cells::DETAIL_SUFFIXES {
                    generated.push(format!("{name}{suffix}"));
                }
            }
        }
        let mut unique: BTreeSet<_> = input_headers.iter().map(String::as_str).collect();
        for name in &generated {
            if name.is_empty() || !unique.insert(name) {
                return Err(invalid(
                    Code::UsageColumn,
                    &format!(
                        "answer column `{name}` already exists or collides with another generated column; choose a different name"
                    ),
                ));
            }
            if self.format == FileFormat::Xlsx {
                check_text(name)?;
            }
        }
        if let Some(sheet) = &sheet
            && sheet.last_column + generated.len() > xlsx::MAX_COLUMN
        {
            return Err(invalid(
                Code::UsageTable,
                "answer columns exceed Excel's column limit; select a workbook with room to append columns",
            ));
        }
        check_cancel(cancel)?;
        let mut answers = spool::Answers::new()?;
        let mut inputs = records.reader()?;
        let mut call = Driver::open_finite(
            &client.clone().cancel(cancel.clone()),
            request,
            OffsetUnit::CodePoints,
        )?;
        let mut failures: Vec<(usize, Error)> = Vec::new();
        let mut rows = 0_u64;
        let mut answered = 0_u64;
        let mut unsure = 0_u64;
        let mut failed: Vec<(crate::Family, u64)> = Vec::new();
        loop {
            match call.next_event()? {
                Event::NeedsInput => match inputs.next().transpose()? {
                    Some((_, Value::String(text))) => call.send(Record::text(text))?,
                    Some((_, value)) => call.send(Record::json(value))?,
                    None => call.end_input(),
                },
                Event::Row(row) => {
                    if rows == 0
                        && !(filter && matches!(self.output, FileOutput::InPlace))
                        && let Some(error) = row.failure()
                        && error.code() == crate::Code::BackendTransport
                    {
                        return Err(FileError::Engine(error.clone()));
                    }
                    rows += 1;
                    let source_row =
                        records.source_row(usize::try_from(row.position()).map_err(local)?)?;
                    answered += u64::from(row.status() != crate::Status::Failed);
                    unsure += u64::from(row.status() == crate::Status::Unsure);
                    let mut failure = row.failure().cloned();
                    let value = serde_json::to_value(&row).map_err(local)?;
                    let mut cells = Vec::new();
                    if annotation {
                        for name in &names {
                            let named = value
                                .get("answer")
                                .and_then(|a| a.get(name))
                                .unwrap_or(&value);
                            cells.extend(self.file_outcome(named, &mut failure));
                        }
                    } else {
                        cells.extend(self.file_outcome(&value, &mut failure));
                    }
                    if let Some(error) = &failure {
                        let family = error.code().family();
                        if let Some((_, count)) =
                            failed.iter_mut().find(|(kind, _)| *kind == family)
                        {
                            *count += 1;
                        } else {
                            failed.push((family, 1));
                        }
                    }
                    if let Some(failure) = failure {
                        name_failure(&mut failures, source_row, failure);
                    }
                    answers.put(source_row, &cells)?;
                }
                Event::Finished => break,
            }
        }
        check_cancel(cancel)?;
        let mut receipt = FileReceipt {
            output: None,
            backup: None,
            failures,
            answered,
            unsure,
            failed,
        };
        if filter && matches!(self.output, FileOutput::InPlace) && !receipt.failed.is_empty() {
            return Ok(receipt);
        }
        let mut temporary = match self.output {
            FileOutput::InPlace => temporary_paths.temporary(Some(parent)),
            FileOutput::NewFile(_) => temporary_paths.output(parent, &permissions),
        }
        .map_err(local)?;
        if let Some(sheet) = &sheet {
            xlsx::write(
                snapshot.path(),
                temporary.as_file_mut(),
                sheet,
                &generated,
                &mut answers,
                filter,
                cancel,
            )?;
        } else {
            self.write_delimited(
                snapshot.path(),
                &mut temporary,
                &generated,
                &mut answers,
                filter,
                cancel,
            )?;
        }
        temporary.flush().map_err(local)?;
        temporary.as_file().sync_all().map_err(local)?;
        check_cancel(cancel)?;
        let backup = if matches!(self.output, FileOutput::InPlace) {
            let mut backup = self.backup(snapshot.path(), cancel)?;
            check_cancel(cancel)?;
            temporary
                .as_file()
                .set_permissions(permissions)
                .map_err(local)?;
            // Backup creation can be slow. Inspect the live source after it,
            // immediately before replacing the directory entry.
            let conflict = check_owner(&self.input).and_then(|()| {
                if hash(&self.input, cancel)? == digest {
                    Ok(())
                } else {
                    Err(local("input changed during the operation"))
                }
            });
            check_cancel(cancel)?;
            if let Err(error) = conflict {
                let kept = self.keep_completed(temporary)?;
                return Err(local(format!(
                    "{error}; source was not replaced; completed output kept at {}",
                    kept.display()
                )));
            }
            backup
                .replace(temporary, &output)?
                .sync_all()
                .map_err(local)?;
            sync_directory(parent)?;
            Some(backup.path.clone())
        } else {
            temporary.persist_noclobber(&output).map_err(local)?;
            None
        };
        receipt.output = Some(output);
        receipt.backup = backup;
        Ok(receipt)
    }

    /// One column's cell is the field text. Several columns form an object,
    /// and a row whose selected cells are all blank is the blank object `{}`
    /// unless a pointer selects one of its columns.
    fn field(&self, values: Vec<String>, narrowed: bool) -> Value {
        if values.len() == 1 {
            return values
                .into_iter()
                .next()
                .map(Value::String)
                .unwrap_or(Value::Null);
        }
        if !narrowed && values.iter().all(|value| value.trim().is_empty()) {
            return Value::Object(Map::new());
        }
        Value::Object(
            self.columns
                .iter()
                .cloned()
                .zip(values.into_iter().map(Value::String))
                .collect::<Map<_, _>>(),
        )
    }

    fn delimiter(&self) -> u8 {
        if self.format == FileFormat::Tsv {
            b'\t'
        } else {
            b','
        }
    }

    fn destination(&self) -> Result<PathBuf, FileError> {
        let path = match &self.output {
            FileOutput::InPlace => self.input.clone(),
            FileOutput::NewFile(Some(path)) => path.clone(),
            FileOutput::NewFile(None) => sibling(&self.input, "thinkthen")?,
        };
        let expected = match self.format {
            FileFormat::Csv => "csv",
            FileFormat::Tsv => "tsv",
            FileFormat::Xlsx => "xlsx",
        };
        if !path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case(expected))
        {
            return Err(invalid(
                Code::UsageOutput,
                &format!(
                    "output must have a .{expected} extension to preserve the selected format"
                ),
            ));
        }
        if !matches!(self.output, FileOutput::InPlace)
            && path.try_exists().map_err(local)?
            && fs::canonicalize(&path).map_err(local)?
                == fs::canonicalize(&self.input).map_err(local)?
        {
            return Err(invalid(
                Code::UsageOutput,
                "output names the input; use --in-place to retain a backup first",
            ));
        }
        Ok(path)
    }

    fn backup(
        &self,
        snapshot: &Path,
        cancel: &Cancel,
    ) -> Result<crate::files::cleanup::Backup, FileError> {
        for number in 1usize.. {
            check_cancel(cancel)?;
            let suffix = if number == 1 {
                "backup".to_owned()
            } else {
                format!("backup-{number}")
            };
            let path = sibling(&self.input, &suffix)?;
            let parent = path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            let mut temporary_paths = crate::files::cleanup::TemporaryPaths::default();
            let mut temporary = temporary_paths.temporary(Some(parent)).map_err(local)?;
            copy(
                &mut File::open(snapshot).map_err(local)?,
                &mut temporary,
                cancel,
            )?;
            temporary.as_file().sync_all().map_err(local)?;
            check_cancel(cancel)?;
            match crate::files::cleanup::Backup::publish(temporary, path.clone(), cancel) {
                Ok(backup) => {
                    sync_directory(parent)?;
                    return Ok(backup);
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(local(error)),
            }
        }
        Err(FileError::Local("no unused backup name remains".into()))
    }

    fn keep_completed(&self, mut temporary: NamedTempFile) -> Result<PathBuf, FileError> {
        for number in 1usize.. {
            let suffix = if number == 1 {
                "thinkthen".to_owned()
            } else {
                format!("thinkthen-{number}")
            };
            let path = sibling(&self.input, &suffix)?;
            match temporary.persist_noclobber(&path) {
                Ok(file) => {
                    file.sync_all().map_err(local)?;
                    sync_directory(
                        path.parent()
                            .filter(|p| !p.as_os_str().is_empty())
                            .unwrap_or(Path::new(".")),
                    )?;
                    return Ok(path);
                }
                Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
                    temporary = error.file;
                }
                Err(error) => return Err(local(error)),
            }
        }
        Err(local("no unused completed output name remains"))
    }

    fn write_delimited(
        &self,
        source: &Path,
        output: &mut NamedTempFile,
        names: &[String],
        answers: &mut spool::Answers,
        filter: bool,
        cancel: &Cancel,
    ) -> Result<(), FileError> {
        let mut reader =
            csv::CsvReader::new(File::open(source).map_err(local)?, self.delimiter(), cancel)?;
        if reader.bom {
            output.write_all(b"\xef\xbb\xbf").map_err(local)?;
        }
        while let Some(row) = reader.next()? {
            let cells = if row.number == 1 {
                names.iter().cloned().map(Cell::Text).collect()
            } else {
                answers
                    .get(row.number)?
                    .unwrap_or_else(|| vec![Cell::Empty; names.len()])
            };
            // Filter writes only the kept rows, as csvgrep does.
            if filter && row.number != 1 && !cells.first().is_some_and(Cell::kept) {
                continue;
            }
            output.write_all(&row.raw).map_err(local)?;
            for cell in cells {
                output.write_all(&[self.delimiter()]).map_err(local)?;
                output
                    .write_all(&csv::encode(&cell.text(), self.delimiter()))
                    .map_err(local)?;
            }
            output.write_all(&row.ending).map_err(local)?;
        }
        Ok(())
    }
}

impl FileJob {
    fn file_outcome(&self, value: &Value, failure: &mut Option<Error>) -> Vec<Cell> {
        let cells = cells::outcome(value, self.details);
        if self.format == FileFormat::Xlsx
            && let Some(error) = cells.iter().find_map(|cell| check_cell(cell).err())
        {
            let error = Error::new(ErrorKind::Io, error.to_string());
            failure.get_or_insert_with(|| error.clone());
            return cells::outcome(
                &serde_json::json!({"status":"failed", "error": error}),
                self.details,
            );
        }
        cells
    }
}

/// Keep the first failed rows by source row number.
fn name_failure(failures: &mut Vec<(usize, Error)>, row: usize, error: Error) {
    let at = failures.partition_point(|(named, _)| *named < row);
    if at < NAMED_FAILURES {
        failures.insert(at, (row, error));
        failures.truncate(NAMED_FAILURES);
    }
}

/// Table framing selects field with `--column`. One column's cell is the
/// field and replaces every saved pointer. Several columns form an object,
/// and each saved pointer must select one of them. Returns the request and
/// whether any pointer selects a column.
fn table_request(request: &Request, columns: &[String]) -> Result<(Request, bool), FileError> {
    let mut encoded = serde_json::to_value(request).map_err(local)?;
    let question = encoded
        .get_mut("question")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| local("the request question could not be encoded"))?;
    let mut narrowed = select(question, columns)?;
    if let Some(children) = question.get_mut("questions").and_then(Value::as_object_mut) {
        for child in children.values_mut().filter_map(Value::as_object_mut) {
            narrowed |= select(child, columns)?;
        }
    }
    let request = Request::from_json(&encoded.to_string())?;
    Ok((request, narrowed))
}

fn select(question: &mut Map<String, Value>, columns: &[String]) -> Result<bool, FileError> {
    if columns.len() == 1 {
        question.remove("field");
        return Ok(false);
    }
    match question.get("field").and_then(Value::as_str) {
        None | Some("") => Ok(false),
        Some(pointer)
            if columns.iter().any(|column| {
                pointer == format!("/{}", column.replace('~', "~0").replace('/', "~1"))
            }) =>
        {
            Ok(true)
        }
        Some(pointer) => Err(invalid(
            Code::UsageColumn,
            &format!(
                "field pointer `{pointer}` selects none of the --column names; select its column or remove the pointer"
            ),
        )),
    }
}

fn sibling(input: &Path, suffix: &str) -> Result<PathBuf, FileError> {
    let stem = input.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
        invalid(
            Code::UsageOutput,
            "input filename must be UTF-8 and name a file",
        )
    })?;
    let extension = input
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| invalid(Code::UsageOutput, "input must have a format extension"))?;
    Ok(input.with_file_name(format!("{stem}.{suffix}.{extension}")))
}
fn check_owner(input: &Path) -> Result<(), FileError> {
    let filename = input
        .file_name()
        .ok_or_else(|| invalid(Code::UsageOutput, "input must name a file"))?
        .to_string_lossy();
    if input
        .with_file_name(format!("~${filename}"))
        .try_exists()
        .map_err(local)?
    {
        return Err(invalid(
            Code::UsageOutput,
            "Excel appears to have the workbook open; close it before using --in-place",
        ));
    }
    Ok(())
}

#[cfg(not(windows))]
fn sync_directory(path: &Path) -> Result<(), FileError> {
    File::open(path).map_err(local)?.sync_all().map_err(local)
}

#[cfg(windows)]
fn sync_directory(_path: &Path) -> Result<(), FileError> {
    // Windows needs no directory flush. Each caller has already flushed the
    // published file through its handle, after the rename. NTFS keeps the name
    // in that file's own record and journals the rename, so a crash leaves the
    // old name or the new one, never a damaged entry. Unix keeps the name only
    // in the parent, which is why it fsyncs the parent. FlushFileBuffers, the
    // one Windows flush, is documented for files and volumes, not directories.
    Ok(())
}

fn hash(path: &Path, cancel: &Cancel) -> Result<Vec<u8>, FileError> {
    let mut file = File::open(path).map_err(local)?;
    let mut hash = Sha256::new();
    let mut buffer = [0; 65_536];
    loop {
        check_cancel(cancel)?;
        let n = file.read(&mut buffer).map_err(local)?;
        if n == 0 {
            break;
        }
        hash.update(buffer.get(..n).ok_or_else(|| local("invalid read size"))?);
    }
    Ok(hash.finalize().to_vec())
}
fn check_cell(cell: &Cell) -> Result<(), FileError> {
    check_text(&cell.text())
}
fn check_text(text: &str) -> Result<(), FileError> {
    if text.encode_utf16().count() > xlsx::MAX_TEXT_UNITS {
        return Err(invalid(
            Code::UsageOutput,
            "answer exceeds Excel's cell text limit; use CSV or TSV to retain the full answer",
        ));
    }
    if text.chars().any(|c| {
        !matches!(c, '\t' | '\n' | '\r') && (c < ' ' || matches!(c, '\u{fffe}' | '\u{ffff}'))
    }) {
        return Err(invalid(
            Code::UsageOutput,
            "answer contains characters unsupported by Excel XML; use CSV or TSV",
        ));
    }
    Ok(())
}
fn invalid(code: Code, message: &str) -> FileError {
    FileError::Invalid(
        code,
        crate::core::diagnostic::requirement("file", "a valid file operation", message),
    )
}
