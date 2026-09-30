//! The two ways a sweep case damages what its priming run stored.

use std::fs;
use std::io;
use std::path::Path;

use super::{HOSTILE, written};
use crate::harness::spawn;

/// Turn a priming run's store into its fixture and damage that. A damaged
/// fixture is not JSON; a hostile one keeps each answer's key and state and
/// fills every other text field with hostile text, so its key no longer matches.
pub(crate) fn damage_fixture(dir: &Path, damage: &str) -> io::Result<()> {
    let converted = spawn(&["cache", "convert", &dir.to_string_lossy()], &[], b"")?;
    assert_eq!(
        converted.status.code(),
        Some(0),
        "the priming store converts"
    );
    let fixture = dir.join("thinkthen.jsonl");
    if damage != HOSTILE {
        return fs::write(fixture, damage);
    }
    let hostile = "\u{1b}[31mPWNED\u{1b}[0m marker-evidence-7b3ac5";
    let mut lines = String::new();
    for line in fs::read_to_string(&fixture)?.lines() {
        let mut line: serde_json::Value = serde_json::from_str(line).map_err(io::Error::other)?;
        if line.get("key").is_some() {
            for field in ["url", "model", "question", "answer", "answered_by"] {
                line[field] = hostile.into();
            }
        }
        lines += &format!("{line}\n");
    }
    fs::write(fixture, lines)
}

/// Overwrite every entry a priming run recorded with the damaged bytes.
pub(super) fn damage_entries(case: &str, dir: &Path, damage: &str) -> io::Result<()> {
    let entries = written(dir);
    assert!(!entries.is_empty(), "{case}: an entry to damage");
    for entry in entries.into_iter().filter(|entry| {
        entry
            .file_name()
            .is_some_and(|name| !name.to_string_lossy().starts_with('.'))
    }) {
        fs::write(&entry, damage)?;
    }
    Ok(())
}
