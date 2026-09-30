//! `thinkthen cache convert DIR`, by ADR 0111 section 9: one fixture from the
//! union of what a folder holds.
//!
//! The fixture is read first, then the live file, then every old
//! `DIGEST.json` entry in name order. On one key the newer `taken_at` wins, and
//! a tie keeps the entry read first, so the fixture keeps its own. Every
//! converted answer takes `taken_at` 0, so the same folder always writes the
//! same bytes. The live file is removed once its entries are in the fixture,
//! because a replay refuses a folder holding both. Old files stay.

use std::fs;
use std::io::{self, Write as _};
use std::path::Path;

use super::{Answer, Entries, JSONL, SQLITE, exists, read_only};
use crate::core::bytes_sha256;
use crate::core::recording::{Converting, convert as entry};
use crate::engine::error::Error;

/// What one conversion did.
#[derive(Debug, Default)]
pub(crate) struct Summary {
    /// Answers the written fixture holds.
    pub(crate) answers: usize,
    /// Answers the old entries gave, before merging.
    pub(crate) converted: usize,
    /// Each skipped old entry's file name and why it was skipped.
    pub(crate) skipped: Vec<(String, &'static str)>,
}

/// Write `DIR/thinkthen.jsonl` from what the folder holds, then remove its
/// live file.
///
/// # Errors
///
/// A fixture that does not parse, a live file that cannot be read, or an old
/// entry that cannot be read or is no recording entry refuses the whole
/// conversion before anything is written.
pub(crate) fn convert(folder: &Path, quote: bool) -> Result<Summary, Error> {
    let (jsonl, sqlite) = (folder.join(JSONL), folder.join(SQLITE));
    let mut held = if exists(&jsonl)? {
        Entries::parse(&fs::read_to_string(&jsonl).map_err(|_| Error::RecordingStorage)?)?
    } else {
        Entries::default()
    };
    let has_sqlite = exists(&sqlite)?;
    if has_sqlite {
        held.merge(Entries::read(&read_only(&sqlite)?)?);
    }
    let mut summary = Summary::default();
    held.merge(old_entries(folder, quote, &mut summary)?);
    summary.answers = held.answers.len();
    replace(folder, &jsonl, held.written()?.as_bytes())?;
    if has_sqlite {
        fs::remove_file(&sqlite).map_err(|_| Error::RecordingStorage)?;
    }
    Ok(summary)
}

/// Every good answer of the folder's old entries, the first file in name
/// order keeping a key two of them give.
fn old_entries(folder: &Path, quote: bool, summary: &mut Summary) -> Result<Entries, Error> {
    let mut names = Vec::new();
    for item in fs::read_dir(folder).map_err(|_| Error::RecordingStorage)? {
        let name = item.map_err(|_| Error::RecordingStorage)?.file_name();
        if let Some(name) = name.to_str().filter(|name| digest_named(name)) {
            names.push(name.to_owned());
        }
    }
    names.sort();
    let mut entries = Entries::default();
    for name in names {
        let bytes = fs::read(folder.join(&name))
            .map_err(|_| Error::Entry(name.clone(), "the file could not be read".to_owned()))?;
        let converted = match entry(&bytes, quote) {
            Ok(converted) => converted,
            Err(Converting::Unreadable(error)) => {
                return Err(Error::Entry(name, error.to_string()));
            }
            Err(skipped) => {
                summary.skipped.push((name, skipped.reason()));
                continue;
            }
        };
        summary.converted += converted.len();
        for answer in converted {
            let state = bytes_sha256(answer.state.as_bytes());
            let (input_tokens, output_tokens) =
                answer.usage.map(|usage| usage.token_counts()).unzip();
            entries.states.insert(state.clone(), answer.state);
            entries.answers.entry(answer.key.hex()).or_insert(Answer {
                key: answer.key.hex(),
                url: answer.url,
                model: answer.model,
                state,
                question: answer.question,
                answer: answer.answer,
                answered_by: answer.answered_by,
                input_tokens,
                output_tokens,
                taken_at: 0,
                origin: answer.origin.as_str().to_owned(),
            });
        }
    }
    Ok(entries)
}

/// A name the old recorder gave an entry: 64 lowercase hex figures and `.json`.
fn digest_named(name: &str) -> bool {
    name.strip_suffix(".json").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
    })
}

/// Write `path` through a temporary file in the same folder, then rename it.
fn replace(folder: &Path, path: &Path, text: &[u8]) -> Result<(), Error> {
    let temporary = folder.join(format!(".{JSONL}.{}.tmp", std::process::id()));
    let written = (|| -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(text)?;
        file.sync_all()?;
        fs::rename(&temporary, path)
    })();
    if written.is_err() {
        let _removed = fs::remove_file(&temporary);
    }
    written.map_err(|_| Error::RecordingStorage)
}
