//! Bounded immutable source occurrences for physical neighbor views.

use std::io::{BufRead, BufReader, Cursor, Read};
use std::path::PathBuf;
use std::sync::Arc;

use crate::core::MAX_RECORD_BYTES;
use crate::failure::Failure;

type Opened = Vec<(Option<PathBuf>, Box<dyn BufRead + Send>)>;

pub(crate) struct Snapshot {
    sources: Vec<Arc<[u8]>>,
}

impl Snapshot {
    pub(crate) fn prepare(opened: Opened) -> Result<(Opened, Option<Self>), Failure> {
        let mut remaining = MAX_RECORD_BYTES;
        let mut sources = Vec::new();
        let mut readers: Opened = Vec::new();
        for (path, mut reader) in opened {
            let mut bytes = Vec::new();
            reader
                .by_ref()
                .take((remaining + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(Failure::Input)?;
            if bytes.len() > remaining {
                return Err(Failure::Usage(
                    "--around reads at most 16 MiB across all input sources",
                ));
            }
            remaining -= bytes.len();
            let bytes: Arc<[u8]> = bytes.into();
            readers.push((path, Box::new(BufReader::new(Cursor::new(bytes.clone())))));
            sources.push(bytes);
        }
        Ok((readers, Some(Self { sources })))
    }

    /// Scan without a line-offset allocation, omitting a phantom trailing line.
    pub(crate) fn lines(
        &self,
        source: usize,
        first: usize,
        last: usize,
        mut emit: impl FnMut(usize, &[u8]) -> Result<bool, Failure>,
    ) -> Result<bool, Failure> {
        let bytes = self.sources.get(source).ok_or(Failure::Defect(
            "a displayed row names no source occurrence",
        ))?;
        let mut line = 0usize;
        let mut start = 0usize;
        for end in bytes
            .iter()
            .enumerate()
            .filter_map(|(at, &byte)| (byte == b'\n').then_some(at + 1))
            .chain((!bytes.is_empty() && !bytes.ends_with(b"\n")).then_some(bytes.len()))
        {
            line += 1;
            if line > last {
                break;
            }
            let raw = bytes
                .get(start..end)
                .ok_or(Failure::Defect("a source line interval is invalid"))?;
            start = end;
            if line < first {
                continue;
            }
            let raw = raw
                .strip_suffix(b"\n")
                .map_or(raw, |raw| raw.strip_suffix(b"\r").unwrap_or(raw));
            if !emit(line, raw)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
