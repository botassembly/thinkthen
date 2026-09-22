//! The recording folders one command selected.

use std::path::Path;

use crate::args::Common;
use crate::failure::Failure;

/// The folders `--record`, `--replay`, and `--cache` name between them.
#[derive(Debug)]
pub(crate) struct Folders<'a> {
    pub(crate) record: Option<&'a Path>,
    pub(crate) replay: Option<&'a Path>,
}

impl<'a> Folders<'a> {
    /// Read the two folders, with `--cache` standing for both at once.
    pub(crate) fn of(common: &'a Common) -> Result<Self, Failure> {
        let Some(cached) = common.cache.as_deref() else {
            if matches!((&common.record, &common.replay), (Some(record), Some(replay)) if record != replay)
            {
                return Err(Failure::TwoFolders);
            }
            return Ok(Self {
                record: common.record.as_deref(),
                replay: common.replay.as_deref(),
            });
        };
        if common.record.is_some() || common.replay.is_some() {
            return Err(Failure::CacheWithRecording);
        }
        Ok(Self {
            record: Some(cached),
            replay: Some(cached),
        })
    }

    /// True when a folder is named at all, which a plan may not name.
    pub(crate) const fn named(&self) -> bool {
        self.record.is_some() || self.replay.is_some()
    }
}
