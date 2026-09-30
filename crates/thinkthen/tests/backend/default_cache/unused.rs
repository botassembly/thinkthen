//! A complete offline run identifies unused entries without editing the folder.

use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::folder;
use crate::harness::spawn;

const USED: &str = "9f84b17aaf3d4cced930b7c18340bf0388ca4f451b55d2d83feb207843bca6ab";
const OTHER: &str = "86523943957338bf3e7ba2d08659a4ccbd246edc081b0d856779a703696f1786";
const INVALID: &str =
    "thinkthen: --used takes one lowercase 64-character request digest per nonblank line\n";

#[derive(Debug, PartialEq, Eq)]
struct EntryState {
    name: OsString,
    bytes: Vec<u8>,
    mode: u32,
    modified: SystemTime,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    directory_modified: SystemTime,
    entries: Vec<EntryState>,
}

#[cfg(unix)]
fn state(folder: &Path) -> io::Result<State> {
    use std::os::unix::fs::PermissionsExt as _;

    let mut entries = fs::read_dir(folder)?
        .map(|item| {
            let item = item?;
            let metadata = fs::symlink_metadata(item.path())?;
            assert!(
                metadata.file_type().is_file(),
                "all planted objects are regular"
            );
            Ok(EntryState {
                name: item.file_name(),
                bytes: fs::read(item.path())?,
                mode: metadata.permissions().mode(),
                modified: metadata.modified()?,
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    entries.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(State {
        directory_modified: fs::metadata(folder)?.modified()?,
        entries,
    })
}

struct Fixture {
    recording: PathBuf,
    manifest: PathBuf,
    xdg: PathBuf,
}

impl Fixture {
    fn new(label: &str) -> io::Result<Self> {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../demos/01-refund-gate");
        let recording = folder(&format!("unused-{label}-recording"));
        fs::create_dir(&recording)?;
        for digest in [USED, OTHER] {
            let name = format!("{digest}.json");
            fs::copy(source.join("recording").join(&name), recording.join(name))?;
        }
        let manifest_root = folder(&format!("unused-{label}-manifests"));
        fs::create_dir(&manifest_root)?;
        Ok(Self {
            recording,
            manifest: manifest_root.join("used.txt"),
            xdg: manifest_root.join("unused-xdg"),
        })
    }

    fn report(&self, manifest: &Path) -> io::Result<std::process::Output> {
        let folder = self.recording.to_string_lossy();
        let manifest = manifest.to_string_lossy();
        let xdg = self.xdg.to_string_lossy();
        spawn(
            &["cache", "unused", &folder, "--used", &manifest],
            &[("XDG_CACHE_HOME", &xdg)],
            &[],
        )
    }
}

#[cfg(unix)]
#[test]
/// A replay's `meta.requests` names question keys by ADR 0111; `cache
/// unused` reads request digests until slice 5, so the caller supplies them.
fn supplied_digests_name_the_unused_entries_without_editing_the_folder() {
    let case = Fixture::new("one-run").expect("scratch fixtures");
    let before = state(&case.recording).expect("before state");
    let cases = [
        (
            format!("{USED}\n"),
            format!("unused from supplied digests: 1\nunused {OTHER}.json\n"),
        ),
        (
            format!("{USED}\n{USED}\n\n"),
            format!("unused from supplied digests: 1\nunused {OTHER}.json\n"),
        ),
        (
            format!("{USED}\n{}\n", "f".repeat(64)),
            format!("unused from supplied digests: 1\nunused {OTHER}.json\n"),
        ),
        (
            String::new(),
            format!("unused from supplied digests: 2\nunused {OTHER}.json\nunused {USED}.json\n"),
        ),
    ];
    for (text, expected) in cases {
        fs::write(&case.manifest, text).expect("write caller manifest");
        let output = case.report(&case.manifest).expect("unused report");
        assert_eq!(output.status.code(), Some(0), "{output:?}");
        assert_eq!(output.stdout, expected.as_bytes());
        assert!(output.stderr.is_empty());
        assert_eq!(state(&case.recording).expect("after report"), before);
    }
    assert!(!case.recording.join(".locks").exists());
    assert!(!case.recording.join(".thinkthen-backend.json").exists());
    assert!(
        !case.xdg.exists(),
        "neither default cache nor usage was created"
    );
}

#[cfg(unix)]
#[test]
fn manifest_and_bad_entry_refusals_leave_the_folder_unchanged() {
    let case = Fixture::new("refusals").expect("scratch fixtures");
    let before = state(&case.recording).expect("before state");
    let absent_option = spawn(
        &[
            "cache",
            "unused",
            case.recording.to_str().expect("folder path"),
        ],
        &[],
        &[],
    )
    .expect("required option refusal");
    assert_eq!(absent_option.status.code(), Some(2));
    assert!(absent_option.stdout.is_empty());
    assert_eq!(
        state(&case.recording).expect("after missing option"),
        before
    );
    fs::write(&case.manifest, format!("{USED}\nBAD\n")).expect("invalid manifest");
    let missing = case
        .manifest
        .parent()
        .expect("parent")
        .join("folder-was-not-opened");
    let refused = spawn(
        &[
            "cache",
            "unused",
            missing.to_str().expect("missing path"),
            "--used",
            case.manifest.to_str().expect("manifest path"),
        ],
        &[],
        &[],
    )
    .expect("invalid manifest refusal");
    assert_eq!(refused.status.code(), Some(2));
    assert!(refused.stdout.is_empty());
    assert_eq!(refused.stderr, INVALID.as_bytes());
    assert!(!missing.exists());
    assert_eq!(state(&case.recording).expect("after invalid list"), before);

    let unreadable = case
        .manifest
        .parent()
        .expect("parent")
        .join("private-manifest-name");
    let refused = case
        .report(&unreadable)
        .expect("unreadable manifest refusal");
    assert_eq!(refused.status.code(), Some(5));
    assert!(refused.stdout.is_empty());
    assert_eq!(
        refused.stderr,
        b"thinkthen: the --used digest file could not be read\n"
    );
    assert_eq!(
        state(&case.recording).expect("after unreadable list"),
        before
    );

    fs::write(&case.manifest, format!("{USED}\n")).expect("valid manifest");
    fs::write(
        case.recording.join(format!("{}.json", "0".repeat(64))),
        b"not JSON",
    )
    .expect("damaged final entry");
    let damaged = state(&case.recording).expect("damaged state");
    let refused = case.report(&case.manifest).expect("bad entry refusal");
    assert_eq!(refused.status.code(), Some(5));
    assert!(refused.stdout.is_empty());
    assert_eq!(
        refused.stderr,
        b"thinkthen: the cache contains a malformed final entry\n"
    );
    assert_eq!(state(&case.recording).expect("after bad entry"), damaged);
}
