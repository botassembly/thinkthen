//! Answer folders for a host whose callers do not own the process, such as
//! a database server. Ticket 0318.

use super::EngineBuilder;
use crate::engine::facade::Storage;
use crate::public::error::Error;

/// The refusal for a folder another user owns or others can write. It names
/// no path, as the command's warning names none.
pub(crate) const NOT_PRIVATE: &str = "the answer folder belongs to another user or others can write it, so they could choose its answers; make it this process user's own with mode 0700, or name another folder";

/// The refusal for a folder this process cannot create or read.
pub(crate) const NOT_READY: &str =
    "the answer folder cannot be created or read by this process user; name another folder";

impl EngineBuilder {
    /// Supply a reserved proxy activation. Build refuses before reading local resources.
    #[must_use]
    pub fn proxy(mut self, _value: &crate::public::ProxyActivation) -> Self {
        self.proxy_supplied = true;
        self
    }

    /// Settle folders for a host that serves callers who do not own it, such
    /// as a database server. The platform default cache is off, so answers
    /// are cached only in a folder named by `THINKTHEN_CACHE` or
    /// [`EngineBuilder::cache_at`]. `build` refuses a named cache, record, or
    /// replay folder that another user owns or others can write. A group
    /// write bit alone passes, as it does for the command's warning. `build`
    /// first creates a missing named folder with mode 0700.
    #[must_use]
    pub fn shared_host(mut self) -> Self {
        self.server = true;
        self
    }

    /// Check a shared host's folders; any other engine keeps its storage as
    /// is. A missing folder is created private first, so no other user can
    /// create it between this check and the first write.
    pub(super) fn served(&self, storage: Storage) -> Result<Storage, Error> {
        if !self.server {
            return Ok(storage);
        }
        for folder in [&storage.record, &storage.replay].into_iter().flatten() {
            let metadata = crate::engine::store::make_folder(folder)
                .ok()
                .and_then(|()| std::fs::metadata(folder).ok())
                .ok_or_else(|| Error::local(NOT_READY))?;
            if crate::config::writable_by_another(&metadata) {
                return Err(Error::usage(NOT_PRIVATE));
            }
        }
        Ok(storage)
    }
}
