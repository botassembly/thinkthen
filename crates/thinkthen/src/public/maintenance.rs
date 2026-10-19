//! Explicit local cache operations shared by the command and library callers.
//! The caller names each directory; these operations send nothing.
use crate::{Error, engine::store};
use std::collections::HashSet;
use std::path::Path;

pub use store::{
    ConversionSummary as CacheConversion, Counts as CacheCounts, Prune as CachePruneOptions,
    Pruned as CachePruned, Selection as CachePruneSelection,
};

/// Inspect a cache directory without changing its contents.
/// `private_default` retains the platform cache's owner and permission checks.
/// # Errors
/// Returns the existing store format, ownership and read failures.
pub fn cache_counts(path: &Path, private_default: bool) -> Result<CacheCounts, Error> {
    store::counts(path, private_default).map_err(Error::from)
}

/// List stored question keys absent from the caller's complete used-key set.
/// The caller owns manifest parsing; this operation changes nothing.
/// # Errors
/// Returns the existing store format and read failures.
pub fn cache_unused(path: &Path, used: &HashSet<String>) -> Result<Vec<String>, Error> {
    store::unused(path, used).map_err(Error::from)
}

/// Merge a named cache's fixture and live store into its saved fixture.
/// The operation retains the existing write lock and conflict checks.
/// # Errors
/// Refuses incompatible histories or unsafe or unreadable local state.
pub fn convert_cache(path: &Path) -> Result<CacheConversion, Error> {
    store::convert(path).map_err(Error::from)
}

/// Select answers for pruning without changing the named directory.
/// The library captures current time once for the existing selection rules.
/// # Errors
/// Returns the existing selection, store format and read failures.
pub fn preview_cache_prune(
    path: &Path,
    options: &CachePruneOptions,
) -> Result<CachePruneSelection, Error> {
    store::preview(path, options, store::now()).map_err(Error::from)
}

/// Remove the selected answers from the caller's named cache directory.
/// Selection and removal retain the existing write lock and timestamp rules.
/// # Errors
/// Returns the existing selection, store format and write failures.
pub fn prune_cache(path: &Path, options: &CachePruneOptions) -> Result<CachePruned, Error> {
    store::prune(path, options, store::now()).map_err(Error::from)
}

/// Best-effort warning signal for an explicitly selected cache or recording folder.
/// Uses the existing native owner/world-write predicate on directory metadata.
/// Missing paths, regular files, metadata failures and non-Unix platforms return false.
/// This does not establish security or storage validity; storage admission still owns
/// those checks. The caller owns its private-default exemption and warning timing.
#[must_use]
pub fn storage_writable_by_another(path: &Path) -> bool {
    std::fs::metadata(path).is_ok_and(|metadata| {
        metadata.is_dir() && crate::config::writable_by_another(&metadata)
    })
}
