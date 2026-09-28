//! Legacy-readable monthly totals and separately durable retry counts.

use std::fs::{self, File};
use std::io::{self, Read as _, Seek as _, SeekFrom, Write as _};
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{
    Counts, Shared, Stage, make_private_directory, maybe_fail, note_initial_sync, open_private,
    open_stable_lock, open_verified, overflow, pause_after_creation, recognized_month,
    validate_directory, verify_identity,
};

const LEGACY_SCHEMA: &str = "thinkthen.usage/1";
const RETRY_SCHEMA: &str = "thinkthen.usage.retries/1";

pub(crate) struct ReadFailure {
    pub(crate) name: String,
    source: io::Error,
}

impl std::fmt::Debug for ReadFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReadFailure")
            .field("name", &self.name)
            .field("source", &"<withheld>")
            .finish()
    }
}

impl ReadFailure {
    fn at(name: &str, source: io::Error) -> Self {
        Self {
            name: name.to_owned(),
            source,
        }
    }

    pub(crate) fn category(&self) -> &'static str {
        if self.source.kind() == io::ErrorKind::InvalidData {
            "invalid contents"
        } else {
            "unsafe or unreadable state"
        }
    }
}

impl std::fmt::Display for ReadFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.name, self.category())
    }
}

impl std::error::Error for ReadFailure {}

impl From<ReadFailure> for io::Error {
    fn from(value: ReadFailure) -> Self {
        value.source
    }
}

#[derive(Debug)]
pub(crate) struct Totals {
    pub(crate) month: Counts,
    pub(crate) total: Counts,
}

#[derive(Debug)]
struct Month {
    name: String,
    counts: Counts,
    contaminated: bool,
    sidecar: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RetryRow {
    schema: String,
    retries: u64,
}

#[derive(Serialize)]
struct LegacyRow {
    schema: &'static str,
    requests_sent: u64,
    input_tokens: u64,
    output_tokens: u64,
    cache_answers: u64,
}

impl From<Counts> for LegacyRow {
    fn from(value: Counts) -> Self {
        Self {
            schema: LEGACY_SCHEMA,
            requests_sent: value.requests_sent,
            input_tokens: value.input_tokens,
            output_tokens: value.output_tokens,
            cache_answers: value.cache_answers,
        }
    }
}

pub(crate) fn read(path: &Path, month: &str) -> Result<Totals, ReadFailure> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(Totals {
                month: Counts::default(),
                total: Counts::default(),
            });
        }
        Err(error) => return Err(ReadFailure::at("usage directory", error)),
        Ok(metadata) => validate_directory(&metadata)
            .map_err(|error| ReadFailure::at("usage directory", error))?,
    }
    let directory = open_verified(path, true, 0o700)
        .map_err(|error| ReadFailure::at("usage directory", error))?;
    let lock_path = path.join(".lock");
    let lock = open_verified(&lock_path, false, 0o600)
        .map_err(|error| ReadFailure::at("usage directory", error))?;
    File::lock_shared(&lock).map_err(|error| ReadFailure::at("usage directory", error))?;
    verify_identity(path, &directory, true)
        .map_err(|error| ReadFailure::at("usage directory", error))?;
    verify_identity(&lock_path, &lock, false)
        .map_err(|error| ReadFailure::at("usage directory", error))?;
    let mut current = Counts::default();
    let mut total = Counts::default();
    for entry in scan(path)? {
        total = total
            .checked_add(entry.counts)
            .ok_or_else(|| ReadFailure::at(&entry.name, overflow()))?;
        if entry.name == format!("{month}.json") {
            current = entry.counts;
        }
    }
    Ok(Totals {
        month: current,
        total,
    })
}

pub(super) fn update(path: &Path, month: &str, delta: Counts, shared: &Shared) -> io::Result<()> {
    maybe_fail(Stage::Setup)?;
    make_private_directory(path)?;
    let directory = open_verified(path, true, 0o700)?;
    let lock_path = path.join(".lock");
    let (lock, created) = open_stable_lock(&lock_path)?;
    pause_after_creation(created);
    maybe_fail(Stage::Lock)?;
    super::lock::acquire(&lock, shared)?;
    verify_identity(path, &directory, true)?;
    verify_identity(&lock_path, &lock, false)?;
    maybe_fail(Stage::Validation)?;
    let months = scan(path).map_err(|error| io::Error::new(error.source.kind(), error))?;
    for entry in &months {
        if entry.contaminated {
            if !entry.sidecar {
                write_retry(path, &directory, &entry.name, entry.counts.retries)?;
            }
            write_legacy(path, &directory, &entry.name, entry.counts)?;
        }
    }
    let name = format!("{month}.json");
    let old = months
        .iter()
        .find(|entry| entry.name == name)
        .map_or(Counts::default(), |entry| entry.counts);
    if !months.iter().any(|entry| entry.name == name) {
        lock.sync_all()?;
        directory.sync_all()?;
        note_initial_sync();
        note_initial_sync();
    }
    let next = old.checked_add(delta).ok_or_else(overflow)?;
    write_legacy(path, &directory, &name, next)?;
    write_retry(path, &directory, &name, next.retries)
}

fn scan(path: &Path) -> Result<Vec<Month>, ReadFailure> {
    let mut names = Vec::new();
    let mut sidecars = Vec::new();
    for entry in fs::read_dir(path).map_err(|error| ReadFailure::at("usage directory", error))? {
        let entry = entry.map_err(|error| ReadFailure::at("usage directory", error))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if recognized_month(&name) {
            names.push(name);
        } else if let Some(month) = name.strip_prefix("retries-")
            && recognized_month(month)
        {
            sidecars.push((name.clone(), month.to_owned()));
        }
    }
    names.sort();
    sidecars.sort();
    for (name, month) in &sidecars {
        if !names.contains(month) {
            return Err(ReadFailure::at(
                name,
                io::Error::new(io::ErrorKind::InvalidData, "orphan retry sidecar"),
            ));
        }
    }
    let mut rows = Vec::with_capacity(names.len());
    for name in names {
        let monthly = path.join(&name);
        let mut file =
            open_verified(&monthly, false, 0o600).map_err(|error| ReadFailure::at(&name, error))?;
        let (mut counts, contaminated) =
            read_counts(&mut file).map_err(|error| ReadFailure::at(&name, error))?;
        verify_identity(&monthly, &file, false).map_err(|error| ReadFailure::at(&name, error))?;
        let sidecar_name = format!("retries-{name}");
        let sidecar = sidecars.iter().any(|(found, _)| *found == sidecar_name);
        if sidecar {
            let retry_path = path.join(&sidecar_name);
            let mut file = open_verified(&retry_path, false, 0o600)
                .map_err(|error| ReadFailure::at(&sidecar_name, error))?;
            let retries =
                read_retry(&mut file).map_err(|error| ReadFailure::at(&sidecar_name, error))?;
            verify_identity(&retry_path, &file, false)
                .map_err(|error| ReadFailure::at(&sidecar_name, error))?;
            if contaminated && counts.retries != retries {
                return Err(ReadFailure::at(
                    &sidecar_name,
                    io::Error::new(io::ErrorKind::InvalidData, "retry totals differ"),
                ));
            }
            counts.retries = retries;
        }
        rows.push(Month {
            name,
            counts,
            contaminated,
            sidecar,
        });
    }
    Ok(rows)
}

fn read_counts(file: &mut File) -> io::Result<(Counts, bool)> {
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    if value.get("schema").and_then(serde_json::Value::as_str) != Some(LEGACY_SCHEMA) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "usage schema"));
    }
    let contaminated = value.get("retries").is_some();
    let counts = serde_json::from_value(value)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    Ok((counts, contaminated))
}

fn read_retry(file: &mut File) -> io::Result<u64> {
    let row: RetryRow = serde_json::from_reader(file)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    if row.schema != RETRY_SCHEMA {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "retry schema"));
    }
    Ok(row.retries)
}

fn write_legacy(path: &Path, directory: &File, name: &str, counts: Counts) -> io::Result<()> {
    replace(path, directory, name, &LegacyRow::from(counts), false)
}

fn write_retry(path: &Path, directory: &File, month: &str, retries: u64) -> io::Result<()> {
    let row = RetryRow {
        schema: RETRY_SCHEMA.to_owned(),
        retries,
    };
    replace(path, directory, &format!("retries-{month}"), &row, true)
}

fn replace(
    path: &Path,
    directory: &File,
    name: &str,
    row: &impl Serialize,
    retry: bool,
) -> io::Result<()> {
    let temporary = path.join(if retry { ".retry.tmp" } else { ".update.tmp" });
    let mut file = open_private(&temporary, true)?;
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    maybe_fail(if retry {
        Stage::RetryWrite
    } else {
        Stage::Write
    })?;
    serde_json::to_writer(&mut file, row).map_err(io::Error::other)?;
    file.write_all(b"\n")?;
    maybe_fail(Stage::FileSync)?;
    file.sync_all()?;
    verify_identity(&temporary, &file, false)?;
    drop(file);
    maybe_fail(Stage::Rename)?;
    fs::rename(&temporary, path.join(name))?;
    maybe_fail(Stage::DirectorySync)?;
    directory.sync_all()
}
