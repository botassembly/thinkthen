//! One admitted operation owns its snapshot, stream and publication.

use crate::contract_driver::driver::{Driver, Event};
use crate::files::admission::{check_cancel, copy};
use crate::files::{
    FileError, FileFormat, FileJob, FileOutput, FileReceipt, csv, headers, local, spool, xlsx,
};
use crate::records::Record;
use crate::{Cancel, Engine, Error, ErrorKind, OffsetUnit, Question, Request};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

impl FileJob {
    /// Execute one function over the file and atomically publish complete outcomes.
    ///
    /// # Errors
    /// Returns an error for admission, cancellation, final-call, format or
    /// filesystem failure. An in-place source conflict retains the completed
    /// output at the path named in the error. Completed row failures stay in the receipt.
    pub fn execute(
        &self,
        request: &Request,
        client: &Engine,
        cancel: &Cancel,
    ) -> Result<FileReceipt, FileError> {
        if !request.records().is_empty() {
            return Err(invalid(
                "a file operation's request must not contain initial records",
            ));
        }
        let prepared = request.question();
        let annotation = matches!(prepared, Question::Annotate(_));
        if !annotation
            && self.columns.len() == 1
            && !prepared.evidence_pointer().to_string().is_empty()
        {
            return Err(invalid(
                "a single --column supplies text; omit --evidence or select several named columns",
            ));
        }
        let names: Vec<String> = if annotation {
            if self.into.is_some() {
                return Err(invalid("annotate uses its question names; omit --into"));
            }
            match prepared {
                Question::Annotate(question) => question.questions().keys().cloned().collect(),
                _ => Vec::new(),
            }
        } else {
            vec![
                self.into
                    .clone()
                    .filter(|s| !s.is_empty())
                    .ok_or_else(|| invalid("name the answer column with --into NAME"))?,
            ]
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
                    "in-place input cannot be a symbolic link; use its real path",
                ));
            }
            check_owner(&self.input)?;
        } else if output.try_exists().map_err(local)? {
            return Err(invalid("output already exists; choose a new --output path"));
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
                |row, values| records.push(row, self.evidence(values, annotation)),
            )?;
            input_headers.clone_from(&sheet.headers);
            Some(sheet)
        } else {
            let mut reader = csv::CsvReader::new(
                File::open(snapshot.path()).map_err(local)?,
                self.delimiter(),
                cancel,
            )?;
            let header = reader
                .next()?
                .ok_or_else(|| invalid("input has no header row; add column names"))?;
            headers(&header.fields, &self.columns).map_err(|error| match error {
                FileError::Invalid(message) => {
                    FileError::Invalid(format!("CSV/TSV row 1 {message}"))
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
                        .ok_or_else(|| invalid("selected column is missing"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            input_headers = header.fields;
            while let Some(row) = reader.next()? {
                check_cancel(cancel)?;
                if row.fields.len() != input_headers.len() {
                    return Err(FileError::Invalid(crate::core::diagnostic::requirement(
                        &format!("CSV/TSV row {} (line {})", row.number, row.line),
                        &format!("{} fields", input_headers.len()),
                        &format!(
                            "{} fields; quote a field containing a separator or remove the extra separator",
                            row.fields.len()
                        ),
                    )));
                }
                let values = positions
                    .iter()
                    .map(|p| row.fields.get(*p).cloned().unwrap_or_default())
                    .collect();
                records.push(row.number, self.evidence(values, annotation))?;
            }
            None
        };
        let mut generated = Vec::new();
        for name in &names {
            generated.push(name.clone());
            for suffix in ["_status", "_probability", "_error", "_detail", "_source"] {
                generated.push(format!("{name}{suffix}"));
            }
        }
        let mut unique: BTreeSet<_> = input_headers.iter().map(String::as_str).collect();
        for name in &generated {
            if name.is_empty() || !unique.insert(name) {
                return Err(invalid(&format!(
                    "answer column `{name}` already exists or collides with another generated column; choose a different name"
                )));
            }
            if self.format == FileFormat::Xlsx {
                check_cell(&Value::String(name.clone()))?;
            }
        }
        if let Some(sheet) = &sheet
            && sheet.last_column + generated.len() > xlsx::MAX_COLUMN
        {
            return Err(invalid(
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
        let mut first_failure = None;
        loop {
            match call.next_event()? {
                Event::NeedsInput => match inputs.next().transpose()? {
                    Some((_, Value::String(text))) => call.send(Record::text(text))?,
                    Some((_, value)) => call.send(Record::json(value))?,
                    None => call.end_input(),
                },
                Event::Row(row) => {
                    let source_row =
                        records.source_row(usize::try_from(row.position()).map_err(local)?)?;
                    let mut failure = row.failure().cloned();
                    let value = serde_json::to_value(&row).map_err(local)?;
                    let mut cells = Vec::new();
                    if annotation {
                        for name in &names {
                            let named = value
                                .get("answer")
                                .and_then(|a| a.get(name))
                                .unwrap_or(&value);
                            cells.extend(file_outcome(named, self.format, &mut failure));
                        }
                    } else {
                        cells.extend(file_outcome(&value, self.format, &mut failure));
                    }
                    if let Some(failure) = failure
                        && first_failure
                            .as_ref()
                            .is_none_or(|(position, _)| source_row < *position)
                    {
                        first_failure = Some((source_row, failure));
                    }
                    answers.put(source_row, &cells)?;
                }
                Event::Finished => break,
            }
        }
        check_cancel(cancel)?;
        let mut temporary = match self.output {
            FileOutput::InPlace => temporary_paths.temporary(Some(parent)),
            FileOutput::NewFile(_) => temporary_paths.output(parent),
        }
        .map_err(local)?;
        if let Some(sheet) = &sheet {
            xlsx::write(
                snapshot.path(),
                temporary.as_file_mut(),
                sheet,
                &generated,
                &mut answers,
                cancel,
            )?;
        } else {
            self.write_delimited(
                snapshot.path(),
                &mut temporary,
                &generated,
                &mut answers,
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
        Ok(FileReceipt {
            output,
            backup,
            first_failure: first_failure.map(|(_, error)| error),
        })
    }

    fn evidence(&self, values: Vec<String>, annotation: bool) -> Value {
        if values.len() == 1 && !annotation {
            return values
                .into_iter()
                .next()
                .map(Value::String)
                .unwrap_or(Value::Null);
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
            return Err(invalid(&format!(
                "output must have a .{expected} extension to preserve the selected format"
            )));
        }
        if !matches!(self.output, FileOutput::InPlace)
            && path.try_exists().map_err(local)?
            && fs::canonicalize(&path).map_err(local)?
                == fs::canonicalize(&self.input).map_err(local)?
        {
            return Err(invalid(
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
        cancel: &Cancel,
    ) -> Result<(), FileError> {
        let mut reader =
            csv::CsvReader::new(File::open(source).map_err(local)?, self.delimiter(), cancel)?;
        if reader.bom {
            output.write_all(b"\xef\xbb\xbf").map_err(local)?;
        }
        while let Some(row) = reader.next()? {
            output.write_all(&row.raw).map_err(local)?;
            let cells = if row.number == 1 {
                names.iter().cloned().map(Value::String).collect()
            } else {
                answers
                    .get(row.number)?
                    .unwrap_or_else(|| vec![Value::Null; names.len()])
            };
            for cell in cells {
                output.write_all(&[self.delimiter()]).map_err(local)?;
                output
                    .write_all(&csv::encode(
                        &crate::files::cell_text(&cell),
                        self.delimiter(),
                    ))
                    .map_err(local)?;
            }
            output.write_all(&row.ending).map_err(local)?;
        }
        Ok(())
    }
}

fn file_outcome(value: &Value, format: FileFormat, failure: &mut Option<Error>) -> Vec<Value> {
    let cells = outcome(value);
    if format == FileFormat::Xlsx
        && let Some(error) = cells.iter().find_map(|cell| check_cell(cell).err())
    {
        let error = Error::new(ErrorKind::Io, error.to_string());
        failure.get_or_insert_with(|| error.clone());
        return outcome(&serde_json::json!({"status":"failed", "error": error}));
    }
    cells
}

fn outcome(value: &Value) -> Vec<Value> {
    let answer = match value.get("status").and_then(Value::as_str) {
        Some("answered") => match value.get("answer").unwrap_or(&Value::Null) {
            Value::Null => Value::String("null".into()),
            Value::Array(_) | Value::Object(_) => {
                Value::String(value.get("answer").unwrap_or(&Value::Null).to_string())
            }
            answer => answer.clone(),
        },
        _ => Value::Null,
    };
    let mut cells = vec![answer];
    for key in ["status", "probability", "error", "detail", "source"] {
        let value = value.get(key).cloned().unwrap_or(Value::Null);
        cells.push(if matches!(value, Value::Array(_) | Value::Object(_)) {
            Value::String(value.to_string())
        } else {
            value
        });
    }
    cells
}
fn sibling(input: &Path, suffix: &str) -> Result<PathBuf, FileError> {
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| invalid("input filename must be UTF-8 and name a file"))?;
    let extension = input
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| invalid("input must have a format extension"))?;
    Ok(input.with_file_name(format!("{stem}.{suffix}.{extension}")))
}
fn check_owner(input: &Path) -> Result<(), FileError> {
    let filename = input
        .file_name()
        .ok_or_else(|| invalid("input must name a file"))?
        .to_string_lossy();
    if input
        .with_file_name(format!("~${filename}"))
        .try_exists()
        .map_err(local)?
    {
        return Err(invalid(
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
    // Windows publication flushes the published file. It does not promise
    // the Unix directory-entry durability provided by fsync on its parent.
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
fn check_cell(value: &Value) -> Result<(), FileError> {
    let text = crate::files::cell_text(value);
    if text.encode_utf16().count() > xlsx::MAX_TEXT_UNITS {
        return Err(invalid(
            "answer exceeds Excel's cell text limit; use CSV or TSV to retain the full answer",
        ));
    }
    if text.chars().any(|c| {
        !matches!(c, '\t' | '\n' | '\r') && (c < ' ' || matches!(c, '\u{fffe}' | '\u{ffff}'))
    }) {
        return Err(invalid(
            "answer contains characters unsupported by Excel XML; use CSV or TSV",
        ));
    }
    Ok(())
}
fn invalid(message: &str) -> FileError {
    FileError::Invalid(crate::core::diagnostic::requirement(
        "file",
        "a valid file operation",
        message,
    ))
}
