//! Framed, checksummed answer retention independent of disposable output spools.

use super::{FileError, local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

const MAGIC: &[u8] = b"THINKTHEN-RECOVERY-1\n";
const MAX_FRAME: u64 = 128 * 1024 * 1024;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum Frame {
    Manifest {
        metadata: Value,
        admitted: u64,
    },
    Occurrence {
        position: u64,
        source: Value,
    },
    Row {
        position: u64,
        row: Value,
    },
    Closing {
        complete: bool,
        emitted: u64,
        output: Option<PathBuf>,
    },
}

/// A retained file is deliberately absent from forced scratch cleanup.
#[derive(Debug)]
pub(crate) struct Journal {
    file: File,
    path: PathBuf,
    closed: bool,
    append_failed: bool,
    emitted: u64,
}

impl Journal {
    /// Persist an already private, exclusively created file before writing content.
    pub(crate) fn create(
        file: NamedTempFile,
        path: PathBuf,
        metadata: Value,
        admitted: u64,
    ) -> Result<Self, FileError> {
        crate::access::AccessList::private(file.as_file(), file.path()).map_err(local)?;
        let file = file
            .persist_noclobber(&path)
            .map_err(|error| local(error.error))?;
        let mut journal = Self {
            file,
            path,
            closed: false,
            append_failed: false,
            emitted: 0,
        };
        journal.file.write_all(MAGIC).map_err(local)?;
        journal.append(&Frame::Manifest { metadata, admitted })?;
        Ok(journal)
    }

    pub(crate) fn instruction(&self) -> String {
        format!(
            "completed answers retained at {}; export with thinkthen recover FILE",
            self.path.display()
        )
    }

    pub(crate) fn retained_error(&self, error: crate::Error) -> crate::Error {
        let mut retained = crate::Error::from_contract(
            error.code(),
            format!(
                "{error}; completed answers retained at {}; export with thinkthen recover FILE",
                self.path.display()
            ),
        );
        retained.source = error.source;
        retained.http_status = error.http_status;
        retained
    }

    pub(crate) fn append(&mut self, frame: &Frame) -> Result<(), FileError> {
        let bytes = serde_json::to_vec(frame).map_err(local)?;
        if bytes.len() as u64 > MAX_FRAME {
            return Err(local("recovery frame exceeds the size limit"));
        }
        if self.append_failed {
            return Err(local("recovery append previously failed"));
        }
        let emitted = if matches!(frame, Frame::Row { .. }) {
            self.emitted
                .checked_add(1)
                .ok_or_else(|| local("too many recovery rows"))?
        } else {
            self.emitted
        };
        let result = self
            .file
            .write_all(&(bytes.len() as u64).to_le_bytes())
            .and_then(|()| self.file.write_all(&Sha256::digest(&bytes)))
            .and_then(|()| self.file.write_all(&bytes));
        if result.is_err() {
            self.append_failed = true;
        } else {
            self.emitted = emitted;
        }
        result.map_err(local)
    }

    pub(crate) fn close(
        &mut self,
        complete: bool,
        output: Option<PathBuf>,
    ) -> Result<(), FileError> {
        if !self.append_failed {
            self.append(&Frame::Closing {
                complete,
                emitted: self.emitted,
                output,
            })?;
        }
        self.file.sync_all().map_err(local)?;
        self.closed = true;
        Ok(())
    }

    /// Delete only a name that still resolves to this operation's open file.
    pub(crate) fn discard(self) -> Result<(), FileError> {
        let current = File::open(&self.path).map_err(local)?;
        if !crate::access::AccessList::same_file(&self.file, &current).map_err(local)? {
            return Err(local("recovery path was replaced; refusing to remove it"));
        }
        fs::remove_file(&self.path).map_err(local)
    }
}

impl Drop for Journal {
    fn drop(&mut self) {
        if !self.closed {
            // Ordinary unwinding retains complete frames and attempts a sync.
            // Explicit callers report sync errors; Drop cannot replace their error.
            let _ = self.file.sync_all();
        }
    }
}

/// Validate each complete frame; only an incomplete trailing frame may be dropped.
pub(crate) fn read(
    path: &Path,
    mut frame: impl FnMut(Frame) -> Result<(), FileError>,
) -> Result<bool, FileError> {
    if fs::symlink_metadata(path)
        .map_err(local)?
        .file_type()
        .is_symlink()
    {
        return Err(local("recovery input cannot be a symbolic link"));
    }
    let mut file = File::open(path).map_err(local)?;
    let mut magic = vec![0; MAGIC.len()];
    file.read_exact(&mut magic).map_err(local)?;
    if magic != MAGIC {
        return Err(local("unsupported recovery format"));
    }
    loop {
        let mut length = [0; 8];
        let count = file.read(&mut length).map_err(local)?;
        if count == 0 {
            return Ok(false);
        }
        if count < 8 {
            match file.read_exact(
                length
                    .get_mut(count..)
                    .ok_or_else(|| local("invalid recovery length"))?,
            ) {
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(true),
                result => result.map_err(local)?,
            }
        }
        let length = u64::from_le_bytes(length);
        if length > MAX_FRAME {
            return Err(local("recovery frame exceeds the size limit"));
        }
        let mut checksum = [0; 32];
        let mut bytes = vec![0; usize::try_from(length).map_err(local)?];
        match file
            .read_exact(&mut checksum)
            .and_then(|()| file.read_exact(&mut bytes))
        {
            Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(true),
            result => result.map_err(local)?,
        }
        if Sha256::digest(&bytes).as_slice() != checksum {
            return Err(local("corrupt recovery frame checksum"));
        }
        frame(serde_json::from_slice(&bytes).map_err(local)?)?;
    }
}

/// Retain resolved routing without credentials or potentially secret query values.
pub(crate) fn settings(engine: &crate::Engine) -> Result<Value, FileError> {
    let mut settings = serde_json::to_value(engine.resolved_settings()).map_err(local)?;
    if let Some(url) = settings.get_mut("url")
        && let Some(text) = url.as_str()
    {
        *url = Value::String(text.split(['?', '#']).next().unwrap_or_default().to_owned());
    }
    Ok(settings)
}
