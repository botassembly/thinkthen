//! The recording folders one command selected.

use crate::args::Common;
use crate::edge::Environment;
use crate::failure::Failure;

/// The folders `--record`, `--replay`, and `--cache` name between them.
#[derive(Debug)]
pub(crate) struct Folders {
    pub(crate) record: Option<std::path::PathBuf>,
    pub(crate) replay: Option<std::path::PathBuf>,
    pub(crate) private_default: bool,
    pub(crate) cache_answers: bool,
}

impl Folders {
    /// Read the two folders, with `--cache` standing for both at once.
    pub(crate) fn of(common: &Common, environment: &Environment) -> Result<Self, Failure> {
        let Some(cached) = common.cache.as_deref() else {
            if matches!((&common.record, &common.replay), (Some(record), Some(replay)) if record != replay)
            {
                return Err(Failure::TwoFolders);
            }
            if common.record.is_some()
                || common.replay.is_some()
                || common.no_cache
                || common.dry_run
                || (!environment.default_cache_enabled() && environment.cache_is_platform_default())
            {
                return Ok(Self {
                    record: common.record.clone(),
                    replay: common.replay.clone(),
                    private_default: false,
                    cache_answers: false,
                });
            }
            let default = environment
                .cache()
                .ok_or(Failure::DefaultCacheUnavailable)?;
            return Ok(Self {
                record: Some(default.to_owned()),
                replay: Some(default.to_owned()),
                private_default: environment.cache_is_platform_default(),
                cache_answers: true,
            });
        };
        if common.record.is_some() || common.replay.is_some() {
            return Err(Failure::CacheWithRecording);
        }
        Ok(Self {
            record: Some(cached.to_owned()),
            replay: Some(cached.to_owned()),
            private_default: false,
            cache_answers: true,
        })
    }

    /// True when a folder is named at all, which a plan may not name.
    pub(crate) const fn named(&self) -> bool {
        self.record.is_some() || self.replay.is_some()
    }

    /// Whether the user explicitly selected the folder and expects recording counts.
    pub(crate) const fn reported(&self) -> bool {
        self.named() && !self.private_default
    }
}
