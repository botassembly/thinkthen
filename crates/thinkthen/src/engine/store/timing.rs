//! Bounded optional timing history, serialized separately from accepted answer bodies.
use super::Row;
use crate::core::{AttemptObservation, AttemptOutcome};
use crate::engine::{Cancel, error::Error};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions, TryLockError},
    io::{self, Read as _, Write as _},
    num::NonZeroU64,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};
const NAME: &str = "thinkthen.timing.jsonl";
const MAX_BYTES: usize = 8 * 1024 * 1024;
const MAX_ENTRIES: usize = 65_536;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Outcome {
    Ok,
    Status,
    Transport,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    key: String,
    ordinal: NonZeroU64,
    wall_ms: u64,
    outcome: Outcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_ms: Option<u64>,
}
pub(super) struct Timing {
    folder: PathBuf,
    temporary: Option<PathBuf>,
    // Keep one stable writer lock through the answer transaction and file replacement.
    _lock: File,
}
impl Drop for Timing {
    fn drop(&mut self) {
        if let Some(path) = &self.temporary {
            let _removed = fs::remove_file(path);
        }
    }
}
impl Timing {
    pub(super) fn prepare(
        folder: &Path,
        rows: &[Row<'_>],
        attempts: &[AttemptObservation],
        cancel: &Cancel,
        limit: Duration,
    ) -> Result<Option<Self>, Error> {
        if rows.is_empty() || attempts.is_empty() {
            return Ok(None);
        }
        let lock = open_lock(&folder.join(".thinkthen.timing.lock"))?;
        acquire(&lock, cancel, limit)?;
        let (mut body, mut entries) = read(folder)?;
        for row in rows {
            for event in attempts {
                append(&mut body, &mut entries, row, event)?;
            }
        }
        let (temporary, mut file) = temporary(folder)?;
        let timing = Self {
            folder: folder.to_owned(),
            temporary: Some(temporary),
            _lock: lock,
        };
        let written = file.write_all(&body).and_then(|()| file.sync_all());
        drop(file);
        written.map_err(|_| Error::RecordingStorage)?;
        Ok(Some(timing))
    }
    pub(super) fn commit(mut self) -> Result<(), Error> {
        let path = self.temporary.as_ref().ok_or(Error::RecordingStorage)?;
        fs::rename(path, self.folder.join(NAME)).map_err(|_| Error::RecordingStorage)?;
        self.temporary = None;
        super::convert::sync_folder(&self.folder).map_err(|_| Error::RecordingStorage)
    }
}
fn append(
    body: &mut Vec<u8>,
    entries: &mut usize,
    row: &Row<'_>,
    event: &AttemptObservation,
) -> Result<(), Error> {
    if *entries == MAX_ENTRIES {
        return Err(Error::RecordingStorage);
    }
    let entry = Entry {
        key: row.key.hex(),
        ordinal: NonZeroU64::new(event.ordinal()).ok_or(Error::RecordingStorage)?,
        wall_ms: event.wall_ms(),
        outcome: match event.outcome() {
            AttemptOutcome::Ok => Outcome::Ok,
            AttemptOutcome::Status => Outcome::Status,
            AttemptOutcome::Transport => Outcome::Transport,
        },
        status: event.status(),
        server_ms: event.server_ms(),
    };
    serde_json::to_writer(&mut *body, &entry).map_err(|_| Error::RecordingStorage)?;
    body.push(b'\n');
    if body.len() > MAX_BYTES {
        return Err(Error::RecordingStorage);
    }
    *entries += 1;
    Ok(())
}
fn read(folder: &Path) -> Result<(Vec<u8>, usize), Error> {
    let mut body = Vec::new();
    match File::open(folder.join(NAME)) {
        Ok(file) => {
            file.take((MAX_BYTES + 1) as u64)
                .read_to_end(&mut body)
                .map_err(|_| Error::RecordingStorage)?;
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok((body, 0)),
        Err(_) => return Err(Error::RecordingStorage),
    }
    if body.len() > MAX_BYTES || (!body.is_empty() && !body.ends_with(b"\n")) {
        return Err(Error::RecordingStorage);
    }
    let mut entries = 0;
    if body.is_empty() {
        return Ok((body, entries));
    }
    for line in body
        .strip_suffix(b"\n")
        .ok_or(Error::RecordingStorage)?
        .split(|byte| *byte == b'\n')
    {
        let entry: Entry = serde_json::from_slice(line).map_err(|_| Error::RecordingStorage)?;
        if crate::core::pack::QuestionKey::parse(&entry.key).is_none() || entries == MAX_ENTRIES {
            return Err(Error::RecordingStorage);
        }
        entries += 1;
    }
    Ok((body, entries))
}
fn open_lock(path: &Path) -> Result<File, Error> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(path).map_err(|_| Error::RecordingStorage)?;
    if !file.metadata().is_ok_and(|metadata| metadata.is_file()) {
        return Err(Error::RecordingStorage);
    }
    Ok(file)
}
fn acquire(file: &File, cancel: &Cancel, limit: Duration) -> Result<(), Error> {
    let start = Instant::now();
    let mut pause = Duration::from_millis(2);
    loop {
        match file.try_lock() {
            Ok(()) => return Ok(()),
            Err(TryLockError::WouldBlock) => {}
            Err(TryLockError::Error(_)) => return Err(Error::RecordingStorage),
        }
        let left = limit.saturating_sub(start.elapsed());
        if left.is_zero() {
            return Err(Error::RecordingStorage);
        }
        if let Some(stop) = cancel.wait(pause.min(left)) {
            return Err(stop);
        }
        pause = (pause * 2).min(Duration::from_millis(100));
    }
}
fn temporary(folder: &Path) -> Result<(PathBuf, File), Error> {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    for _ in 0..32 {
        let path = folder.join(format!(
            ".thinkthen.timing-{}-{}.tmp",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        match options.open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(_) => return Err(Error::RecordingStorage),
        }
    }
    Err(Error::RecordingStorage)
}
