//! Adapt path and delimited stdin framing to the existing safe file operation.
use super::{
    input::{Format, Input},
    options::{self, explicit, usage},
};
use crate::files::{FileError, FileFormat, FileJob, FileOutput, FileReceipt, TemporaryPaths};
use crate::{Cancel, Engine, Error, Request};
use clap::ArgMatches;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub(super) fn validate(input: &Input, args: &ArgMatches, json: bool) -> Result<(), Error> {
    if !input.table()
        && options::explicit(args, "into").is_none()
        && (options::explicit(args, "recovery").is_some()
            || options::explicit(args, "no-recovery").is_some())
    {
        return Err(usage(
            "recovery",
            "a table or JSONL enrichment operation",
            "ordinary stream",
        ));
    }
    if input.table() {
        if json {
            return Err(usage(
                "json",
                "text output for table framing",
                "JSON row output",
            ));
        }
    } else {
        for key in ["sheet", "column", "into", "output", "in-place", "details"] {
            if key == "into"
                && options::explicit(args, key).is_some()
                && input.sources.iter().all(|(_, f)| *f == Format::Jsonl)
            {
                if json {
                    return Err(usage(
                        "into",
                        "answer fields without --json",
                        "JSON row output",
                    ));
                }
                continue;
            }
            if options::explicit(args, key).is_some() {
                return Err(usage(key, "csv, tsv or xlsx framing", "stream framing"));
            }
        }
    }
    Ok(())
}

pub(super) fn run(
    input: &Input,
    args: &ArgMatches,
    request: &Request,
    engine: &Engine,
    cancel: &Cancel,
    quiet: bool,
) -> Result<u8, Error> {
    // Table choices are per call, so they come from options and never the environment.
    let into = explicit(args, "into");
    if !matches!(request.question(), crate::Question::Annotate(_))
        && into.as_deref().is_none_or(str::is_empty)
    {
        return Err(usage("into", "an answer column name", "absent name"));
    }
    let (path, format) = input
        .sources
        .first()
        .ok_or_else(|| usage("files", "a table source", "absent source"))?;
    let format = match format {
        Format::Csv => FileFormat::Csv,
        Format::Tsv => FileFormat::Tsv,
        _ => FileFormat::Xlsx,
    };
    let output = explicit(args, "output").map(PathBuf::from);
    let in_place = options::boolean("in-place", explicit(args, "in-place"))?.unwrap_or(false);
    let details = options::boolean("details", explicit(args, "details"))?.unwrap_or(false);
    if in_place && output.is_some() {
        return Err(usage("output", "output or in-place", "both forms"));
    }
    if path == "-" && (format == FileFormat::Xlsx || in_place) {
        return Err(usage(
            "files",
            "a source path for xlsx or in-place",
            "stdin",
        ));
    }
    let columns: Vec<String> = args
        .get_many::<String>("column")
        .map(|values| values.cloned().collect())
        .unwrap_or_default();
    let columns = if input.selection.len() > 1 {
        if !columns.is_empty() {
            return Err(usage("field", "field pointers or columns", "both forms"));
        }
        input
            .selection
            .iter()
            .map(|pointer| {
                if pointer
                    .to_string()
                    .strip_prefix('/')
                    .is_some_and(|path| path.contains('/'))
                {
                    return Err(usage(
                        "field",
                        "top-level column pointers for a table",
                        "nested pointer",
                    ));
                }
                pointer
                    .leaf()
                    .map(str::to_owned)
                    .ok_or_else(|| usage("field", "a column pointer", "root pointer"))
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        // One column is the field; a saved pointer gives way to it, a typed one conflicts.
        if columns.len() == 1 && args.get_many::<String>("field").is_some() {
            return Err(usage(
                "field",
                "one --column alone; omit --field",
                "both forms",
            ));
        }
        columns
    };
    let mut paths = TemporaryPaths::default();
    let stdin = path == "-";
    let mut source_guard = if stdin {
        let mut source = paths
            .temporary(None)
            .map_err(|_| super::local("stdin table could not be retained"))?;
        let mut stdin = io::stdin();
        let reader = super::input::spawn(
            move || {
                let mut bytes = vec![0; 64 * 1024];
                let count = stdin
                    .read(&mut bytes)
                    .map_err(|_| super::local("stdin table could not be read"))?;
                bytes.truncate(count);
                Ok((count != 0).then_some(bytes))
            },
            cancel.clone(),
        )?;
        while let Some(bytes) = super::input::receive(&reader, cancel)? {
            source
                .write_all(&bytes)
                .map_err(|_| super::local("stdin table could not be retained"))?;
        }
        Some(source)
    } else {
        None
    };
    let input_path = source_guard
        .as_mut()
        .map_or_else(|| PathBuf::from(path), |file| file.path().to_path_buf());
    let stdout = stdin && output.is_none();
    let output_guard = if stdout {
        Some(
            paths
                .temporary_with_suffix(
                    None,
                    if format == FileFormat::Csv {
                        ".csv"
                    } else {
                        ".tsv"
                    },
                )
                .map_err(|_| super::local("table output could not be retained"))?,
        )
    } else {
        None
    };
    // File publication requires a new path; remove the reserved empty file first.
    let output_path = output_guard.as_ref().map(|file| file.path().to_path_buf());
    if let Some(path) = &output_path {
        std::fs::remove_file(path)
            .map_err(|_| super::local("table output could not be prepared"))?;
    }
    let destination = if in_place {
        FileOutput::InPlace
    } else {
        FileOutput::NewFile(output.or(output_path.clone()))
    };
    let recovery = recovery(args, stdin)?;
    let result = FileJob::new(
        input_path,
        format,
        explicit(args, "sheet"),
        columns,
        into,
        destination,
    )
    .and_then(|job| {
        job.details(details)
            .recovery(recovery)
            .execute_for_command(request, engine, cancel)
    });
    let (result, journal) = match result {
        Ok(mut receipt) => {
            let journal = receipt.recovery.take();
            (Ok(receipt), journal)
        }
        Err(error) => (Err(error), None),
    };
    let completed_output = result
        .as_ref()
        .ok()
        .and_then(|receipt| receipt.output.clone());
    let code = render(result, stdout, cancel, quiet);
    drop(source_guard);
    if let Some(mut journal) = journal {
        if code.is_err() && stdout {
            drop(paths);
            if let Some(output_guard) = output_guard {
                output_guard
                    .keep()
                    .map_err(|error| journal.retained_error(super::local(&error.to_string())))?;
            }
        }
        journal
            .close(code.is_ok(), completed_output)
            .map_err(|error| journal.retained_error(super::local(&error.to_string())))?;
        match code {
            Ok(code) => {
                journal
                    .discard()
                    .map_err(|error| super::local(&error.to_string()))?;
                Ok(code)
            }
            Err(error) => Err(journal.retained_error(error)),
        }
    } else {
        code
    }
}

fn render(
    result: Result<FileReceipt, FileError>,
    stdout: bool,
    cancel: &Cancel,
    quiet: bool,
) -> Result<u8, Error> {
    match result {
        Ok(receipt) => {
            let Some(output) = &receipt.output else {
                summarize(&receipt, false);
                return Ok(6);
            };
            if quiet {
                // The file operation still completes; quiet suppresses its projection.
            } else if stdout {
                let mut file = std::fs::File::open(output)
                    .map_err(|_| super::local("the completed table could not be opened"))?;
                io::copy(&mut file, &mut io::stdout().lock())
                    .map_err(|error| super::output_error(error, cancel))?;
            } else {
                writeln!(io::stdout().lock(), "{}", output.display())
                    .map_err(|error| super::output_error(error, cancel))?;
            }
            if !quiet && let Some(backup) = &receipt.backup {
                let _ = writeln!(
                    io::stderr().lock(),
                    "thinkthen: backup: {}",
                    backup.display()
                );
            }
            summarize(&receipt, quiet);
            if receipt.failures.is_empty() {
                Ok(0)
            } else {
                Ok(
                    if receipt.answered == 0
                        && receipt
                            .failed
                            .iter()
                            .all(|(family, _)| *family == crate::Family::Backend)
                    {
                        4
                    } else {
                        6
                    },
                )
            }
        }
        Err(FileError::Engine(error)) => Err(error),
        Err(error) => {
            let _ = writeln!(io::stderr().lock(), "thinkthen: {}: {error}", error.code());
            Ok(super::family_code(error.family()))
        }
    }
}

/// Name each failed row with its code, and count the rows whose answer cell is missing.
fn summarize(receipt: &FileReceipt, quiet: bool) {
    let mut stderr = io::stderr().lock();
    for (row, error) in &receipt.failures {
        let _ = writeln!(
            stderr,
            "thinkthen: {}: row {row}: {error}",
            error.code().as_str()
        );
    }
    let failed: u64 = receipt.failed.iter().map(|(_, count)| count).sum();
    let named = receipt.failures.len() as u64;
    if failed > named {
        let _ = writeln!(stderr, "thinkthen: {} more failed rows", failed - named);
    }
    drop(stderr);
    report_counts(receipt.unsure, failed, quiet);
}

pub(super) fn report_counts(unsure: u64, failed: u64, quiet: bool) {
    if !quiet && (failed > 0 || unsure > 0) {
        let _ = writeln!(
            io::stderr().lock(),
            "thinkthen: {unsure} rows not sure, {failed} rows failed"
        );
    }
}

/// Resolve only explicit retention choices; stdin has no implicit persistent path.
pub(super) fn recovery(args: &ArgMatches, stdin: bool) -> Result<crate::files::Recovery, Error> {
    let path = explicit(args, "recovery").map(PathBuf::from);
    let disabled = options::boolean("no-recovery", explicit(args, "no-recovery"))?.unwrap_or(false);
    if disabled && path.is_some() {
        return Err(usage("recovery", "a path or --no-recovery", "both"));
    }
    Ok(match path {
        Some(path) => crate::files::Recovery::Path(path),
        None if disabled || stdin => crate::files::Recovery::Disabled,
        None => crate::files::Recovery::Default,
    })
}
