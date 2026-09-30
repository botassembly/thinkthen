//! The recording folders one command selected.

use std::fs;

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
    pub(crate) refresh_cache: bool,
}

impl Folders {
    /// Read the two folders, with `--cache` standing for both at once.
    pub(crate) fn of(common: &Common, environment: &Environment) -> Result<Self, Failure> {
        let default_disabled =
            !environment.default_cache_enabled() && environment.cache_is_platform_default();
        if common.refresh_cache
            && common.cache.is_none()
            && (common.record.is_some()
                || common.replay.is_some()
                || common.no_cache
                || default_disabled)
        {
            return Err(Failure::Usage(
                "--refresh-cache needs an enabled answer cache; use --cache DIR or enable the default cache",
            ));
        }
        let Some(cached) = common.cache.as_deref() else {
            if matches!((&common.record, &common.replay), (Some(record), Some(replay)) if record != replay)
            {
                return Err(Failure::TwoFolders);
            }
            if common.record.is_some()
                || common.replay.is_some()
                || common.no_cache
                || common.dry_run
                || default_disabled
            {
                return Ok(Self {
                    record: common.record.clone(),
                    replay: common.replay.clone(),
                    private_default: false,
                    cache_answers: false,
                    refresh_cache: false,
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
                refresh_cache: common.refresh_cache,
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
            refresh_cache: common.refresh_cache,
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

    /// A best-effort Unix signal; the recorder still decides storage validity.
    pub(crate) fn writable_by_another(&self) -> bool {
        if self.private_default {
            return false;
        }
        self.record
            .as_deref()
            .or(self.replay.as_deref())
            .and_then(|folder| fs::metadata(folder).ok())
            .is_some_and(|metadata| {
                metadata.is_dir() && crate::config::writable_by_another(&metadata)
            })
    }
}
