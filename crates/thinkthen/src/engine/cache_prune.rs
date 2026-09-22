//! Explicit maintenance for a bounded recording cache.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::core::recording::Entry;
use crate::engine::cache_lock;
use crate::engine::error::Error;

#[derive(Debug)]
pub(crate) struct Prune {
    pub(crate) max_size: u64,
    pub(crate) older_than: Option<Duration>,
    pub(crate) answered_by_other_than: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Pruned {
    pub(crate) removed_entries: u64,
    pub(crate) removed_bytes: u64,
    pub(crate) remaining_entries: u64,
    pub(crate) remaining_bytes: u64,
}

#[derive(Debug)]
pub(crate) struct Inspected {
    pub(crate) entries: u64,
    pub(crate) bytes: u64,
}

pub(crate) fn inspect(folder: &Path, private: bool) -> Result<Inspected, Error> {
    let metadata = match fs::symlink_metadata(folder) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(Inspected {
                entries: 0,
                bytes: 0,
            });
        }
        Err(error) => return Err(storage(error)),
        Ok(metadata) => metadata,
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(Error::CacheEntry);
    }
    #[cfg(unix)]
    if private {
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.permissions().mode() & 0o777 != 0o700 {
            return Err(Error::DefaultCachePrivate);
        }
    }
    let gate = cache_lock::shared_folder(folder).map_err(storage)?;
    let current = fs::symlink_metadata(folder).map_err(storage)?;
    let opened = gate.file().metadata().map_err(storage)?;
    if current.file_type().is_symlink() || !current.is_dir() || !same_identity(&current, &opened) {
        return Err(Error::CacheEntry);
    }
    let found = scan(folder)?;
    let entries = u64::try_from(found.len()).map_err(|_| Error::RecordingStorage)?;
    let bytes = found
        .iter()
        .try_fold(0u64, |sum, entry| sum.checked_add(entry.bytes))
        .ok_or(Error::RecordingStorage)?;
    Ok(Inspected { entries, bytes })
}

#[derive(Debug)]
struct Found {
    path: PathBuf,
    name: String,
    bytes: u64,
    modified: SystemTime,
    model: String,
    remove: bool,
}

pub(crate) fn run(folder: &Path, options: &Prune) -> Result<Pruned, Error> {
    let _gate = cache_lock::exclusive_folder(folder).map_err(storage)?;
    run_at(folder, options, SystemTime::now())
}

fn run_at(folder: &Path, options: &Prune, now: SystemTime) -> Result<Pruned, Error> {
    let mut found = scan(folder)?;
    for entry in &mut found {
        let old = options.older_than.is_some_and(|age| {
            now.checked_sub(age)
                .is_some_and(|edge| entry.modified < edge)
        });
        let other = options
            .answered_by_other_than
            .as_deref()
            .is_some_and(|model| entry.model != model);
        entry.remove = old || other;
    }
    let mut kept_bytes = found
        .iter()
        .filter(|entry| !entry.remove)
        .try_fold(0u64, |sum, entry| sum.checked_add(entry.bytes))
        .ok_or(Error::RecordingStorage)?;
    let mut oldest: Vec<_> = found.iter_mut().filter(|entry| !entry.remove).collect();
    oldest.sort_by(|left, right| {
        left.modified
            .cmp(&right.modified)
            .then_with(|| left.name.cmp(&right.name))
    });
    for entry in oldest {
        if kept_bytes <= options.max_size {
            break;
        }
        entry.remove = true;
        kept_bytes = kept_bytes
            .checked_sub(entry.bytes)
            .ok_or(Error::RecordingStorage)?;
    }

    let mut selected: Vec<_> = found.iter().filter(|entry| entry.remove).collect();
    selected.sort_by(|left, right| {
        left.modified
            .cmp(&right.modified)
            .then_with(|| left.name.cmp(&right.name))
    });
    let mut removed_entries = 0u64;
    let mut removed_bytes = 0u64;
    for entry in selected {
        let lock = match cache_lock::try_acquire(folder, entry.name.trim_end_matches(".json"))
            .map_err(storage)?
        {
            cache_lock::TryAcquire::Acquired(lock) => lock,
            cache_lock::TryAcquire::Active => continue,
        };
        maybe_fail(removed_entries)?;
        fs::remove_file(&entry.path).map_err(storage)?;
        lock.unlink().map_err(storage)?;
        lock.sync_folder().map_err(storage)?;
        removed_entries += 1;
        removed_bytes = removed_bytes
            .checked_add(entry.bytes)
            .ok_or(Error::RecordingStorage)?;
    }
    if removed_entries > 0 {
        cache_lock::sync_directory(folder).map_err(storage)?;
    }
    let remaining_entries = u64::try_from(found.len())
        .map_err(|_| Error::RecordingStorage)?
        .checked_sub(removed_entries)
        .ok_or(Error::RecordingStorage)?;
    let total = found
        .iter()
        .try_fold(0u64, |sum, entry| sum.checked_add(entry.bytes))
        .ok_or(Error::RecordingStorage)?;
    Ok(Pruned {
        removed_entries,
        removed_bytes,
        remaining_entries,
        remaining_bytes: total
            .checked_sub(removed_bytes)
            .ok_or(Error::RecordingStorage)?,
    })
}

#[cfg(test)]
thread_local! {
    static FAIL_AFTER: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn maybe_fail(removed: u64) -> Result<(), Error> {
    FAIL_AFTER.with(|place| {
        if place.get() == Some(removed) {
            place.set(None);
            Err(Error::RecordingStorage)
        } else {
            Ok(())
        }
    })
}

#[cfg(not(test))]
const fn maybe_fail(_removed: u64) -> Result<(), Error> {
    Ok(())
}

fn scan(folder: &Path) -> Result<Vec<Found>, Error> {
    let mut found = Vec::new();
    for item in fs::read_dir(folder).map_err(storage)? {
        let item = item.map_err(storage)?;
        let name = item.file_name().to_string_lossy().into_owned();
        if !digest_name(&name) {
            continue;
        }
        let metadata = fs::symlink_metadata(item.path()).map_err(storage)?;
        if !metadata.file_type().is_file() {
            return Err(Error::CacheEntry);
        }
        let file = fs::File::open(item.path()).map_err(storage)?;
        let opened = file.metadata().map_err(storage)?;
        if !same_identity(&metadata, &opened) {
            return Err(Error::CacheEntry);
        }
        let bytes = {
            use std::io::Read as _;
            let mut file = file;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).map_err(storage)?;
            bytes
        };
        let final_metadata = fs::symlink_metadata(item.path()).map_err(storage)?;
        if final_metadata.file_type().is_symlink() || !same_identity(&opened, &final_metadata) {
            return Err(Error::CacheEntry);
        }
        let (digest, model) = Entry::inspected(&bytes).map_err(|_| Error::CacheEntry)?;
        if digest.file_name() != name {
            return Err(Error::CacheEntry);
        }
        found.push(Found {
            path: item.path(),
            name,
            bytes: allocated(&metadata),
            modified: metadata.modified().map_err(storage)?,
            model,
            remove: false,
        });
    }
    Ok(found)
}

#[cfg(unix)]
fn same_identity(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt as _;
    (left.dev(), left.ino()) == (right.dev(), right.ino())
}

#[cfg(not(unix))]
fn same_identity(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    left.len() == right.len() && left.file_type() == right.file_type()
}

fn digest_name(name: &str) -> bool {
    name.len() == 69
        && name.ends_with(".json")
        && name.as_bytes().get(..64).is_some_and(|bytes| {
            bytes
                .iter()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
        })
}

#[cfg(unix)]
fn allocated(metadata: &fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt as _;
    metadata.blocks().saturating_mul(512)
}

#[cfg(not(unix))]
fn allocated(metadata: &fs::Metadata) -> u64 {
    metadata.len()
}

fn storage(_error: io::Error) -> Error {
    Error::RecordingStorage
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File, FileTimes};
    use std::time::{Duration, SystemTime};

    use crate::core::Url;
    use crate::core::recording::{Entry, Exchange};
    use crate::engine::cache_lock;
    use crate::engine::error::Error;

    use super::{FAIL_AFTER, Prune, allocated, run_at};

    fn folder(name: &str) -> std::path::PathBuf {
        let folder =
            std::env::temp_dir().join(format!("thinkthen-prune-{name}-{}", std::process::id()));
        let _absent = fs::remove_dir_all(&folder);
        fs::create_dir(&folder).expect("folder");
        folder
    }

    fn plant(
        folder: &std::path::Path,
        request: &'static [u8],
        model: &str,
        modified: SystemTime,
    ) -> (String, u64) {
        let url = Url::new("http://127.0.0.1:1/v1/systemone").expect("url");
        let exchange = Exchange::new(&url, request);
        let name = exchange.digest().file_name();
        let response = format!(r#"{{"model":"{model}"}}"#);
        let bytes = Entry::of(&exchange, response.as_bytes())
            .expect("entry")
            .written()
            .expect("written");
        let path = folder.join(&name);
        fs::write(&path, bytes).expect("entry file");
        File::options()
            .write(true)
            .open(&path)
            .expect("entry handle")
            .set_times(FileTimes::new().set_modified(modified))
            .expect("mtime");
        let bytes = allocated(&fs::metadata(path).expect("metadata"));
        (name, bytes)
    }

    fn options(max_size: u64) -> Prune {
        Prune {
            max_size,
            older_than: None,
            answered_by_other_than: None,
        }
    }

    #[test]
    fn an_active_digest_is_skipped_and_remains_in_the_counts() {
        let folder = folder("active");
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
        let (name, bytes) = plant(&folder, br#"{"state":"active"}"#, "old", now);
        let lock = cache_lock::acquire(&folder, name.trim_end_matches(".json")).expect("owner");
        let result = run_at(&folder, &options(1), now).expect("prune");
        assert_eq!((result.removed_entries, result.removed_bytes), (0, 0));
        assert_eq!(
            (result.remaining_entries, result.remaining_bytes),
            (1, bytes)
        );
        assert!(folder.join(name).exists());
        drop(lock);
    }

    #[test]
    fn a_late_failure_leaves_the_deterministic_oldest_prefix_deleted() {
        let folder = folder("partial");
        let base = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
        let first = plant(&folder, br#"{"state":"first"}"#, "old", base).0;
        let second = plant(&folder, br#"{"state":"second"}"#, "old", base).0;
        let third = plant(
            &folder,
            br#"{"state":"third"}"#,
            "old",
            base + Duration::from_secs(1),
        )
        .0;
        let mut tied = [first, second];
        tied.sort();
        FAIL_AFTER.with(|place| place.set(Some(1)));
        assert!(matches!(
            run_at(&folder, &options(1), base),
            Err(Error::RecordingStorage)
        ));
        assert!(!folder.join(&tied[0]).exists());
        assert!(folder.join(&tied[1]).exists());
        assert!(folder.join(third).exists());
    }

    #[test]
    fn age_is_strict_and_model_selection_forms_a_union() {
        let folder = folder("selectors");
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
        let edge = plant(
            &folder,
            br#"{"state":"edge"}"#,
            "keep",
            now - Duration::from_secs(10),
        )
        .0;
        let old = plant(
            &folder,
            br#"{"state":"old"}"#,
            "keep",
            now - Duration::from_secs(11),
        )
        .0;
        let model = plant(&folder, br#"{"state":"model"}"#, "other", now).0;
        let result = run_at(
            &folder,
            &Prune {
                max_size: u64::MAX,
                older_than: Some(Duration::from_secs(10)),
                answered_by_other_than: Some("keep".to_owned()),
            },
            now,
        )
        .expect("prune");
        assert_eq!(result.removed_entries, 2);
        assert!(folder.join(edge).exists());
        assert!(!folder.join(old).exists());
        assert!(!folder.join(model).exists());
    }

    #[test]
    fn allocated_size_stays_at_the_target_and_leaves_one_byte_over() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
        let at = folder("size-at");
        let (name, bytes) = plant(&at, br#"{"state":"at"}"#, "keep", now);
        assert_eq!(
            run_at(&at, &options(bytes), now)
                .expect("at target")
                .removed_entries,
            0
        );
        assert!(at.join(name).exists());

        let over = folder("size-over");
        let (name, bytes) = plant(&over, br#"{"state":"over"}"#, "keep", now);
        assert_eq!(
            run_at(&over, &options(bytes - 1), now)
                .expect("over target")
                .removed_entries,
            1
        );
        assert!(!over.join(name).exists());
    }
}
