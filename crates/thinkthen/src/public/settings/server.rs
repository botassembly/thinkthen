//! Answer folders for a host whose callers do not own the process, such as
//! a database server. Ticket 0318.

use super::EngineBuilder;
use crate::engine::facade::Storage;
use crate::public::error::Error;

/// The refusal for a folder another user owns or others can write. It names
/// no path, as the command's warning names none.
pub(crate) const NOT_PRIVATE: &str = "the answer folder belongs to another user or others can write it, so they could choose its answers; make it this process user's own with mode 0700, or name another folder";

impl EngineBuilder {
    /// Settle folders for a host that serves callers who do not own it, such
    /// as a database server. The platform default cache is off, so answers
    /// are cached only in a folder named by `THINKTHEN_CACHE` or
    /// [`EngineBuilder::cache_at`]. `build` refuses a named cache, record, or
    /// replay folder that another user owns or others can write. A group
    /// write bit alone passes, as it does for the command's warning.
    #[must_use]
    pub fn shared_host(mut self) -> Self {
        self.server = true;
        self
    }

    /// Check a shared host's folders; any other engine keeps its storage as is.
    pub(super) fn served(&self, storage: Storage) -> Result<Storage, Error> {
        if self.server
            && [&storage.record, &storage.replay]
                .into_iter()
                .flatten()
                .filter_map(|folder| std::fs::metadata(folder).ok())
                .any(|metadata| metadata.is_dir() && crate::config::writable_by_another(&metadata))
        {
            return Err(Error::usage(NOT_PRIVATE));
        }
        Ok(storage)
    }
}
