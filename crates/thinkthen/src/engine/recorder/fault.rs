//! Deterministic storage failures for recorder tests.

use std::io;

#[cfg(test)]
use std::cell::Cell;

use crate::engine::error::Error;

#[cfg(test)]
thread_local! {
    pub(super) static STORAGE_FAULT: Cell<Option<StorageStage>> = const { Cell::new(None) };
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum StorageStage {
    Write,
    FileSync,
    Install,
    DirectorySync,
    Cleanup,
    LockRemove,
    LockDirectorySync,
    FinalRead,
}

#[derive(Clone, Copy)]
pub(super) enum StorageStageName {
    Write,
    FileSync,
    Install,
    DirectorySync,
    Cleanup,
    LockRemove,
    LockDirectorySync,
    FinalRead,
}

#[cfg(test)]
pub(super) fn maybe_fail(stage: StorageStageName) -> Result<(), Error> {
    maybe_fail_io(stage).map_err(|_| Error::RecordingStorage)
}

#[cfg(not(test))]
pub(super) const fn maybe_fail(_stage: StorageStageName) -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
pub(super) fn maybe_fail_io(stage: StorageStageName) -> io::Result<()> {
    let requested = match stage {
        StorageStageName::Write => StorageStage::Write,
        StorageStageName::FileSync => StorageStage::FileSync,
        StorageStageName::Install => StorageStage::Install,
        StorageStageName::DirectorySync => StorageStage::DirectorySync,
        StorageStageName::Cleanup => StorageStage::Cleanup,
        StorageStageName::LockRemove => StorageStage::LockRemove,
        StorageStageName::LockDirectorySync => StorageStage::LockDirectorySync,
        StorageStageName::FinalRead => StorageStage::FinalRead,
    };
    STORAGE_FAULT.with(|fault| {
        if fault.get() == Some(requested) {
            fault.set(None);
            Err(io::Error::other("injected recording storage failure"))
        } else {
            Ok(())
        }
    })
}

#[cfg(not(test))]
pub(super) const fn maybe_fail_io(_stage: StorageStageName) -> io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::core::Url;
    use crate::core::recording::{Entry, Exchange};
    use crate::engine::error::Error;
    use crate::engine::recorder::{PreparedRecording, Recorder};

    use super::{STORAGE_FAULT, StorageStage};

    const REQUEST: &[u8] = br#"{"state":"private evidence"}"#;
    const RESPONSE: &[u8] = br#"{"answer":true}"#;
    const OTHER_RESPONSE: &[u8] = br#"{"answer":false}"#;

    fn live(prepared: PreparedRecording) -> crate::engine::recorder::WritePermit {
        match prepared {
            PreparedRecording::Live(permit) => permit,
            PreparedRecording::Replay(_) => panic!("test unexpectedly replays"),
        }
    }

    #[test]
    fn every_applicable_storage_failure_preserves_valid_old_bytes() {
        let cases = [
            (StorageStage::Write, false, false),
            (StorageStage::FileSync, false, false),
            (StorageStage::Install, false, false),
            (StorageStage::DirectorySync, true, false),
            (StorageStage::Cleanup, true, false),
            (StorageStage::LockRemove, true, true),
            (StorageStage::LockDirectorySync, true, true),
            (StorageStage::FinalRead, true, true),
        ];
        for (number, (stage, same_answer, plant_after_lock)) in cases.into_iter().enumerate() {
            let folder = std::env::temp_dir().join(format!(
                "thinkthen-old-entry-{}-{number}",
                std::process::id()
            ));
            let _absent = fs::remove_dir_all(&folder);
            let url = Url::new("http://127.0.0.1:1/v1/systemone").expect("test address");
            let exchange = Exchange::new(&url, REQUEST);
            let digest = exchange.digest();
            let path = folder.join(digest.file_name());
            let old = Entry::of(&exchange, RESPONSE)
                .expect("old entry")
                .written()
                .expect("old entry serializes")
                .into_bytes();
            let permit = if plant_after_lock {
                let recorder = Recorder::of(Some(&folder), Some(&folder)).expect("cache recorder");
                let permit = live(
                    recorder
                        .prepare(&exchange, &digest)
                        .expect("cache prepares"),
                );
                fs::write(&path, &old).expect("test plants a valid winner");
                permit
            } else {
                fs::create_dir_all(&folder).expect("recording folder");
                fs::write(&path, &old).expect("test seeds a valid entry");
                let recorder = Recorder::of(Some(&folder), None).expect("record recorder");
                live(
                    recorder
                        .prepare(&exchange, &digest)
                        .expect("record prepares"),
                )
            };
            let before = fs::read(&path).expect("old entry is readable");
            STORAGE_FAULT.with(|fault| fault.set(Some(stage)));
            let response = if same_answer {
                RESPONSE
            } else {
                OTHER_RESPONSE
            };
            let error = permit
                .finish(&exchange, response, &digest.file_name())
                .expect_err("injected storage stage fails");
            assert!(matches!(error, Error::RecordingStorage), "{stage:?}");
            assert_eq!(
                fs::read(&path).expect("old entry remains"),
                before,
                "{stage:?}"
            );
            let _removed = fs::remove_dir_all(folder);
        }
    }
}
