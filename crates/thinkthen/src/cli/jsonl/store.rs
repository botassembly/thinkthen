//! Operation-owned disk storage indexed by the original record position.
use crate::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

pub(super) struct Store {
    data: File,
    index: File,
}
impl Store {
    pub(super) fn new() -> Result<Self, Error> {
        Ok(Self {
            data: tempfile::tempfile().map_err(storage)?,
            index: tempfile::tempfile().map_err(storage)?,
        })
    }
    pub(super) fn put(&mut self, position: u64, bytes: &[u8]) -> Result<(), Error> {
        let at = position
            .checked_sub(1)
            .and_then(|n| n.checked_mul(16))
            .ok_or_else(|| super::super::local("invalid JSON Lines record position"))?;
        let offset = self.data.seek(SeekFrom::End(0)).map_err(storage)?;
        self.data.write_all(bytes).map_err(storage)?;
        self.index.seek(SeekFrom::Start(at)).map_err(storage)?;
        self.index
            .write_all(&offset.to_le_bytes())
            .map_err(storage)?;
        self.index
            .write_all(&(bytes.len() as u64).to_le_bytes())
            .map_err(storage)
    }
    pub(super) fn get(&mut self, position: u64) -> Result<Vec<u8>, Error> {
        let at = position
            .checked_sub(1)
            .and_then(|n| n.checked_mul(16))
            .ok_or_else(|| super::super::local("invalid JSON Lines record position"))?;
        if at >= self.index.metadata().map_err(storage)?.len() {
            return Ok(Vec::new());
        }
        self.index.seek(SeekFrom::Start(at)).map_err(storage)?;
        let mut number = [0; 8];
        self.index.read_exact(&mut number).map_err(storage)?;
        self.data
            .seek(SeekFrom::Start(u64::from_le_bytes(number)))
            .map_err(storage)?;
        self.index.read_exact(&mut number).map_err(storage)?;
        let mut bytes = vec![0; usize::try_from(u64::from_le_bytes(number)).map_err(storage)?];
        self.data.read_exact(&mut bytes).map_err(storage)?;
        Ok(bytes)
    }
}
fn storage(_: impl std::fmt::Display) -> Error {
    super::super::local("JSON Lines records could not be retained")
}
