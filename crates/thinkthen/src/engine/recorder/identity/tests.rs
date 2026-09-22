use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use crate::core::Url;
use crate::engine::error::Error;
use crate::engine::recorder::fault::{STORAGE_FAULT, StorageStage};

use super::{BackendIdentity, NAME, check, publish_at, read, read_after_open};

static FOLDERS: AtomicU64 = AtomicU64::new(0);

fn folder(name: &str) -> PathBuf {
    let number = FOLDERS.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "thinkthen-backend-identity-{}-{number}-{name}",
        std::process::id()
    ));
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("test folder");
    path
}

fn identity(address: &str) -> BackendIdentity {
    BackendIdentity::new(&Url::new(address).expect("test address"))
}

fn marker(folder: &Path) -> PathBuf {
    folder.join(NAME)
}

fn complete(folder: &Path, expected: &BackendIdentity) {
    assert_eq!(
        read(&marker(folder)).expect("marker reads").as_ref(),
        Some(expected)
    );
}

#[test]
fn every_identity_storage_failure_recovers_from_an_absent_or_complete_marker() {
    let cases = [
        (StorageStage::IdentityCreate, false),
        (StorageStage::IdentityWrite, false),
        (StorageStage::IdentityFileSync, false),
        (StorageStage::IdentityInstall, false),
        (StorageStage::IdentityDirectorySync, true),
        (StorageStage::IdentityCleanup, true),
    ];
    for (stage, installed) in cases {
        let folder = folder(&format!("fault-{stage:?}"));
        let expected = identity("http://127.0.0.1:1/v1/systemone");
        STORAGE_FAULT.with(|fault| fault.set(Some(stage)));
        assert!(matches!(
            check(&folder, &expected, true),
            Err(Error::RecordingStorage)
        ));
        assert_eq!(marker(&folder).exists(), installed, "{stage:?}");
        if installed {
            complete(&folder, &expected);
        }
        check(&folder, &expected, true).expect("a later use recovers");
        complete(&folder, &expected);
        let _removed = fs::remove_dir_all(folder);
    }
}

#[test]
fn a_stale_candidate_is_preserved_and_the_next_unique_name_is_used() {
    let folder = folder("stale-candidate");
    let expected = identity("http://127.0.0.1:1/v1/systemone");
    let first = 7_000_000;
    let stale = folder.join(format!(
        ".thinkthen-backend.{}.{first}.tmp",
        std::process::id()
    ));
    fs::write(&stale, b"private stale bytes").expect("stale candidate");
    publish_at(&folder, &marker(&folder), &expected, first).expect("later candidate publishes");
    assert_eq!(
        fs::read(stale).expect("stale candidate remains"),
        b"private stale bytes"
    );
    complete(&folder, &expected);
}

#[test]
fn matching_concurrent_first_users_share_one_complete_marker() {
    let folder = folder("matching-race");
    let expected = identity("http://127.0.0.1:1/v1/systemone");
    thread::scope(|scope| {
        let first = scope.spawn(|| check(&folder, &expected, true));
        let second = scope.spawn(|| check(&folder, &expected, true));
        first.join().expect("first joins").expect("first binds");
        second.join().expect("second joins").expect("second binds");
    });
    complete(&folder, &expected);
}

#[test]
fn hostile_marker_documents_and_objects_are_storage_failures() {
    let folder = folder("hostile-markers");
    let path = marker(&folder);
    let documents: &[&[u8]] = &[
        b"not json",
        br#"{"schema":"foreign","backend_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#,
        br#"{"schema":"thinkthen.backend-folder/1"}"#,
        br#"{"schema":"thinkthen.backend-folder/1","backend_sha256":4}"#,
        br#"{"schema":"thinkthen.backend-folder/1","backend_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","extra":1}"#,
        br#"{"schema":"thinkthen.backend-folder/1","backend_sha256":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}"#,
        &[b'x'; 257],
    ];
    for bytes in documents {
        fs::write(&path, bytes).expect("hostile marker");
        assert!(matches!(read(&path), Err(Error::RecordingStorage)));
    }
    let _removed = fs::remove_file(&path);
    fs::create_dir(&path).expect("nonregular marker");
    assert!(matches!(read(&path), Err(Error::RecordingStorage)));
}

#[cfg(unix)]
#[test]
fn symlink_unreadable_and_replaced_markers_are_storage_failures() {
    use std::os::unix::fs::{PermissionsExt as _, symlink};

    let folder = folder("unsafe-marker-files");
    let path = marker(&folder);
    let outside = folder.join("outside");
    fs::write(&outside, b"outside").expect("outside file");
    symlink(&outside, &path).expect("marker symlink");
    assert!(matches!(read(&path), Err(Error::RecordingStorage)));
    fs::remove_file(&path).expect("remove symlink");

    fs::write(&path, b"unreadable").expect("unreadable marker");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).expect("remove access");
    assert!(matches!(read(&path), Err(Error::RecordingStorage)));
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).expect("restore access");

    let expected = identity("http://127.0.0.1:1/v1/systemone");
    fs::write(&path, expected.written().expect("marker bytes")).expect("original marker");
    assert!(matches!(
        read_after_open(&path, || {
            fs::rename(&path, folder.join("old-marker")).expect("move opened marker");
            fs::write(&path, expected.written().expect("replacement bytes"))
                .expect("replacement marker");
        }),
        Err(Error::RecordingStorage)
    ));
}

#[cfg(unix)]
#[test]
fn a_long_address_still_writes_one_private_fixed_marker() {
    use std::os::unix::fs::PermissionsExt as _;

    let folder = folder("long-address");
    let address = format!("https://example.test/{}/systemone", "a".repeat(2_000));
    let expected = identity(&address);
    check(&folder, &expected, true).expect("long address binds");
    let metadata = fs::metadata(marker(&folder)).expect("marker metadata");
    assert_eq!(metadata.permissions().mode() & 0o777, 0o600);
    assert!(metadata.len() < 256);
    complete(&folder, &expected);
}

#[test]
fn interruption_before_and_after_publication_allows_later_reuse() {
    for stage in ["before-install", "after-install"] {
        let folder = folder(stage);
        let ready = folder.join("ready");
        let mut child = Command::new(std::env::current_exe().expect("test executable"))
            .args([
                "--ignored",
                "--exact",
                "engine::recorder::identity::tests::interruption_child",
            ])
            .env("THINKTHEN_TEST_IDENTITY_FOLDER", &folder)
            .env("THINKTHEN_TEST_IDENTITY_READY", &ready)
            .env("THINKTHEN_TEST_IDENTITY_PAUSE", stage)
            .spawn()
            .expect("interruption child");
        for _ in 0..300 {
            if ready.exists() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        assert!(ready.exists(), "child reached {stage}");
        child.kill().expect("kill interrupted writer");
        let _status = child.wait().expect("reap interrupted writer");

        let expected = identity("http://127.0.0.1:1/v1/systemone");
        assert_eq!(marker(&folder).exists(), stage == "after-install");
        if marker(&folder).exists() {
            complete(&folder, &expected);
        }
        check(&folder, &expected, true).expect("later use recovers");
        complete(&folder, &expected);
    }
}

#[test]
#[ignore = "run as an interrupted subprocess by the parent test"]
fn interruption_child() {
    let Some(folder) = std::env::var_os("THINKTHEN_TEST_IDENTITY_FOLDER") else {
        return;
    };
    let folder = PathBuf::from(folder);
    fs::create_dir_all(&folder).expect("child folder");
    let expected = identity("http://127.0.0.1:1/v1/systemone");
    check(&folder, &expected, true).expect("pause interrupts this call");
}
