//! Native freshness selection and the one existing folder/mode resolution.

use super::{Cache, EngineBuilder, NO_DEFAULT_CACHE};
use crate::{config, engine::facade::Storage, public::Error};

impl EngineBuilder {
    /// Bypass held answers and send no-cache on actual requests.
    /// Successful nonstorable refreshes evict the previous working answers.
    #[must_use]
    pub fn refresh_cache(mut self, value: bool) -> Self {
        self.refresh_cache = value;
        self
    }

    pub(super) fn storage(&self) -> Result<Storage, Error> {
        if matches!((&self.record, &self.replay), (Some(record), Some(replay)) if record != replay)
        {
            return Err(Error::usage(
                "record and replay name two different folders, and one engine keeps one",
            ));
        }
        if self.record.is_some() || self.replay.is_some() {
            if matches!(self.cache, Cache::At(_)) {
                return Err(Error::usage(
                    "a cache folder is record and replay on one folder, so it stands beside neither",
                ));
            }
            return Ok(Storage {
                record: self.record.clone(),
                replay: self.replay.clone(),
                private_default: false,
                cache_answers: false,
                refresh_cache: self.refresh_cache,
            });
        }
        let (folder, private_default) = match &self.cache {
            Cache::Off => {
                return Ok(Storage {
                    refresh_cache: self.refresh_cache,
                    ..Storage::default()
                });
            }
            Cache::At(folder) => (folder.clone(), false),
            Cache::Default => match &self.seeded {
                Some(seeded) if seeded.platform && (self.server || !seeded.enabled) => {
                    return Ok(Storage {
                        refresh_cache: self.refresh_cache,
                        ..Storage::default()
                    });
                }
                None if self.server => {
                    return Ok(Storage {
                        refresh_cache: self.refresh_cache,
                        ..Storage::default()
                    });
                }
                Some(seeded) => (
                    seeded
                        .folder
                        .clone()
                        .ok_or_else(|| Error::usage(NO_DEFAULT_CACHE))?,
                    seeded.platform,
                ),
                None => (
                    config::cache_path().ok_or_else(|| Error::usage(NO_DEFAULT_CACHE))?,
                    true,
                ),
            },
        };
        Ok(Storage {
            record: Some(folder.clone()),
            replay: Some(folder),
            private_default,
            cache_answers: true,
            refresh_cache: self.refresh_cache,
        })
    }
}
