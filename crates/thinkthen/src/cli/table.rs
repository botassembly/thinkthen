//! Adapt path and delimited stdin framing to the existing safe file operation.
use super::{
    input::{Format, Input},
    options::{self, usage, value},
};
use crate::files::{FileError, FileFormat, FileJob, FileOutput, FileReceipt, TemporaryPaths};
use crate::{Cancel, Engine, Error, Request};
use clap::ArgMatches;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub(super) fn validate(input: &Input, args: &ArgMatches, json: bool) -> Result<(), Error> {
    if input.table() {
        if json {
            return Err(usage(
                "json",
                "text output for table framing",
                "JSON row output",
            ));
        }
    } else {
        for key in ["sheet", "column", "into", "output", "in-place"] {
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
) -> Result<u8, Error> {
    let (path, format) = input
        .sources
        .first()
        .ok_or_else(|| usage("files", "a table source", "absent source"))?;
    let format = match format {
        Format::Csv => FileFormat::Csv,
        Format::Tsv => FileFormat::Tsv,
        _ => FileFormat::Xlsx,
    };
    let mut destination = options::group(args, &["output", "in-place"]).into_iter();
    let output = destination.next().flatten().map(PathBuf::from);
    let in_place = options::boolean("in-place", destination.next().flatten())?.unwrap_or(false);
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
    let columns = options::list(args, "column", "THINKTHEN_COLUMNS")?
        .unwrap_or_else(|| serde_json::Value::Array(Vec::new()));
    let columns = <Vec<String> as serde::Deserialize>::deserialize(&columns).map_err(|error| {
        Error::new(
            crate::ErrorKind::Question,
            crate::core::diagnostic::context("column", &error.to_string()),
        )
    })?;
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
    let result = FileJob::new(
        input_path,
        format,
        value(args, "sheet"),
        columns,
        value(args, "into"),
        destination,
    )
    .and_then(|job| job.execute(request, engine, cancel));
    let code = render(result, stdout, cancel);
    drop(source_guard);
    code
}

fn render(
    result: Result<FileReceipt, FileError>,
    stdout: bool,
    cancel: &Cancel,
) -> Result<u8, Error> {
    match result {
        Ok(receipt) => {
            if stdout {
                let mut file = std::fs::File::open(&receipt.output)
                    .map_err(|_| super::local("the completed table could not be opened"))?;
                io::copy(&mut file, &mut io::stdout().lock())
                    .map_err(|error| super::output_error(error, cancel))?;
            } else {
                writeln!(io::stdout().lock(), "{}", receipt.output.display())
                    .map_err(|error| super::output_error(error, cancel))?;
            }
            if let Some(backup) = receipt.backup {
                let _ = writeln!(
                    io::stderr().lock(),
                    "thinkthen: backup: {}",
                    backup.display()
                );
            }
            if let Some(error) = receipt.first_failure {
                super::report(&error);
                Ok(6)
            } else {
                Ok(0)
            }
        }
        Err(FileError::Engine(error)) => Err(error),
        Err(error) => {
            let _ = writeln!(io::stderr().lock(), "thinkthen: {}: {error}", error.code());
            Ok(super::family_code(error.family()))
        }
    }
}
