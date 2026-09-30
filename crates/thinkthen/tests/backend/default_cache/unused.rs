//! `cache unused` names the question keys a folder holds and a list lacks,
//! from its fixture or its live store, without editing the folder.

use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::folder;
use crate::harness::spawn;

const USED: &str = "2bc2f3db71bf8156dd2d2060d777a34c80a88cb1c0afa665d469e3b055712f71";
const OTHER: &str = "0311afcb8eba767b756924bd17cba8bdf973788f9103d8feb9ec00d4a7366474";
const INVALID: &str =
    "thinkthen: --used takes one lowercase 64-character question key per nonblank line\n";

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
        fs::copy(
            source.join("recording/thinkthen.jsonl"),
            recording.join("thinkthen.jsonl"),
        )?;
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
/// A replay's `meta.requests` names question keys by ADR 0111, so the
/// caller's list is those keys.
fn supplied_keys_name_the_unused_answers_without_editing_the_folder() {
    let case = Fixture::new("one-run").expect("scratch fixtures");
    let before = state(&case.recording).expect("before state");
    let cases = [
        (
            format!("{USED}\n"),
            format!("unused from supplied keys: 1\nunused {OTHER}\n"),
        ),
        (
            format!("{USED}\n{USED}\n\n"),
            format!("unused from supplied keys: 1\nunused {OTHER}\n"),
        ),
        (
            format!("{USED}\n{}\n", "f".repeat(64)),
            format!("unused from supplied keys: 1\nunused {OTHER}\n"),
        ),
        (
            String::new(),
            format!("unused from supplied keys: 2\nunused {OTHER}\nunused {USED}\n"),
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
    assert!(
        !case.xdg.exists(),
        "neither default cache nor usage was created"
    );
}

#[cfg(unix)]
#[test]
fn manifest_refusals_leave_the_folder_unchanged() {
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
        b"thinkthen: the --used key file could not be read\n"
    );
    assert_eq!(
        state(&case.recording).expect("after unreadable list"),
        before
    );
}

#[cfg(unix)]
#[test]
fn store_refusals_leave_the_folder_unchanged() {
    let case = Fixture::new("store-refusals").expect("scratch fixtures");
    fs::write(&case.manifest, format!("{USED}\n")).expect("valid manifest");
    fs::write(case.recording.join("thinkthen.sqlite"), b"").expect("a second store");
    let both = state(&case.recording).expect("both state");
    let refused = case.report(&case.manifest).expect("two stores refusal");
    assert_eq!(refused.status.code(), Some(5));
    assert!(refused.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: the replay folder holds both thinkthen.jsonl and thinkthen.sqlite; \
         run `thinkthen cache convert DIR` to merge them into thinkthen.jsonl\n"
    );
    assert_eq!(state(&case.recording).expect("after two stores"), both);
    fs::remove_file(case.recording.join("thinkthen.sqlite")).expect("second store");

    fs::write(case.recording.join("thinkthen.jsonl"), b"not JSON\n").expect("damaged fixture");
    let damaged = state(&case.recording).expect("damaged state");
    let refused = case
        .report(&case.manifest)
        .expect("damaged fixture refusal");
    assert_eq!(refused.status.code(), Some(5));
    assert!(refused.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "thinkthen: the entry `thinkthen.jsonl` was refused: line 1 is not a question entry\n"
    );
    assert_eq!(
        state(&case.recording).expect("after damaged fixture"),
        damaged
    );
}

#[cfg(unix)]
#[test]
fn the_live_store_names_its_unused_keys() {
    let folder = folder("unused-live-store");
    let filled = super::prune::fill(
        &folder,
        super::EVIDENCE,
        &[
            (
                crate::support::DEFAULT_MODEL,
                "asks for a refund",
                super::ANSWERED,
            ),
            (
                crate::support::DEFAULT_MODEL,
                "asks for a repair",
                super::ANSWERED,
            ),
        ],
    );
    let used = folder.with_extension("used");
    fs::write(&used, format!("{}\n", filled[0])).expect("key list");
    let before = fs::read(folder.join("thinkthen.sqlite")).expect("store");
    let output = spawn(
        &[
            "cache",
            "unused",
            folder.to_str().expect("folder"),
            "--used",
            used.to_str().expect("list"),
        ],
        &[],
        &[],
    )
    .expect("unused report");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("unused from supplied keys: 1\nunused {}\n", filled[1])
    );
    assert_eq!(
        fs::read(folder.join("thinkthen.sqlite")).expect("store"),
        before
    );
}
