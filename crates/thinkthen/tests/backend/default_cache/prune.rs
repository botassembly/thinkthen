//! `cache prune` trims the question store, `thinkthen.sqlite`, by ADR 0111
//! section 10. Each answer here comes from a real cached run on loopback.

use std::fs;
use std::path::Path;

use super::{ANSWERED, EVIDENCE, folder, run};
use crate::harness::{Canned, Listener};
use crate::support::{DEFAULT_MODEL, ENDPOINT_PATH, encoded_decide, keys};

/// One cached answer: the model asked for, the question, and the reply.
type Asked<'a> = (&'a str, &'a str, &'a str);

/// Cache one answer per row in `folder` through a loopback backend, and
/// return each row's question key.
pub(super) fn fill(folder: &Path, evidence: &str, rows: &[Asked<'_>]) -> Vec<String> {
    let mut filled = Vec::new();
    for (model, question, reply) in rows {
        let listener = Listener::serving(vec![Canned::ok(reply)]).expect("listener");
        let output = crate::harness::spawn(
            &["decide", question, "--model", model],
            &[
                ("THINKTHEN_BASE_URL", listener.base()),
                ("THINKTHEN_API_KEY", "test-key"),
                ("THINKTHEN_CACHE", folder.to_str().expect("folder")),
            ],
            evidence.as_bytes(),
        )
        .expect("cached run");
        assert_eq!(output.status.code(), Some(0), "{output:?}");
        assert_eq!(listener.requests().len(), 1);
        let url = format!("{}/{ENDPOINT_PATH}", listener.base());
        let [key] = <[String; 1]>::try_from(keys(&url, &encoded_decide(evidence, model, question)))
            .expect("one question");
        filled.push(key);
    }
    filled
}

/// Set each answer's taken-at second, so age and oldest-first are fixed.
fn taken_at(folder: &Path, times: &[(&str, i64)]) {
    let connection =
        rusqlite::Connection::open(folder.join("thinkthen.sqlite")).expect("the live store");
    for (key, time) in times {
        let key: Vec<u8> = (0..key.len())
            .step_by(2)
            .map(|at| u8::from_str_radix(&key[at..at + 2], 16).expect("hex"))
            .collect();
        let changed = connection
            .execute(
                "UPDATE answers SET taken_at = ?1 WHERE key = ?2",
                rusqlite::params![time, key],
            )
            .expect("taken-at");
        assert_eq!(changed, 1);
    }
}

/// The question keys the store holds, as `cache unused` names them against
/// an empty list.
fn held(folder: &Path) -> Vec<String> {
    let used = folder.with_extension("used");
    fs::write(&used, "").expect("empty key list");
    let output = run(
        &[
            "cache",
            "unused",
            folder.to_str().expect("folder"),
            "--used",
            used.to_str().expect("list"),
        ],
        &[],
    )
    .expect("unused");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    String::from_utf8(output.stdout)
        .expect("text")
        .lines()
        .skip(1)
        .filter_map(|line| line.strip_prefix("unused ").map(str::to_owned))
        .collect()
}

fn store_bytes(folder: &Path) -> u64 {
    fs::metadata(folder.join("thinkthen.sqlite"))
        .expect("the live store")
        .len()
}

fn prune(folder: &Path, options: &[&str]) -> std::process::Output {
    let mut arguments = vec!["cache", "prune", folder.to_str().expect("folder")];
    arguments.extend_from_slice(options);
    run(&arguments, &[]).expect("prune")
}

/// A commit's summary line, with the bytes the store file shows.
fn removed(out: usize, kept: usize, before: u64, after: u64) -> String {
    format!(
        "removed {out} answers and {} bytes; {kept} answers and {after} bytes remain\n",
        before - after
    )
}

/// MODEL, the second answer's reply, and which of the two answers leave,
/// or the refusal that deletes nothing.
type PruneRow<'a> = (&'a str, &'a str, Result<[bool; 2], &'a str>);

#[test]
fn prune_refuses_the_alias_and_an_unknown_model_and_keeps_the_upgrade() {
    const ALIAS: &str = concat!(
        "thinkthen: --answered-by-other-than names the model the requests asked for, ",
        "and no reply names it, so prune removed nothing; ",
        "name the version a result's meta.model shows, not the alias passed to --model\n",
    );
    const UNKNOWN: &str = concat!(
        "thinkthen: --answered-by-other-than names a model no reply in the folder names, ",
        "so prune removed nothing; name the version a result's meta.model shows, ",
        "or delete the folder to remove every answer\n",
    );
    let echoed = ANSWERED.replace("jev-1.13.0", "jev-latest");
    let upgraded = ANSWERED.replace("jev-1.13.0", "jev-1.14.0");
    let rows: [PruneRow; 8] = [
        ("jev-latest", ANSWERED, Err(ALIAS)),
        ("jev-1.14.0", ANSWERED, Err(UNKNOWN)),
        ("no-such-model", ANSWERED, Err(UNKNOWN)),
        ("jev-1.13", ANSWERED, Err(UNKNOWN)),
        ("JEV-LATEST", ANSWERED, Err(UNKNOWN)),
        ("jev-1.13.0", ANSWERED, Ok([false, false])),
        ("jev-latest", &echoed, Ok([true, false])),
        ("jev-1.14.0", &upgraded, Ok([true, false])),
    ];
    for (row, (model, second_reply, leaves)) in rows.into_iter().enumerate() {
        let folder = folder(&format!("prune-alias-{row}"));
        let filled = fill(
            &folder,
            EVIDENCE,
            &[
                (DEFAULT_MODEL, "asks for a refund", ANSWERED),
                ("jev-latest", "another question", second_reply),
            ],
        );
        let before = fs::read(folder.join("thinkthen.sqlite")).expect("store bytes");
        let options = ["--answered-by-other-than", model];
        let leaves = match leaves {
            Ok(leaves) => leaves,
            Err(refusal) => {
                for dry_run in [&["--dry-run"][..], &[]] {
                    let output = prune(&folder, &[&options[..], dry_run].concat());
                    assert_eq!(output.status.code(), Some(2), "{row}");
                    assert!(output.stdout.is_empty(), "{row}");
                    assert_eq!(String::from_utf8_lossy(&output.stderr), refusal, "{row}");
                    assert_eq!(
                        fs::read(folder.join("thinkthen.sqlite")).expect("kept"),
                        before,
                        "{row}"
                    );
                }
                continue;
            }
        };
        let size = store_bytes(&folder);
        let output = prune(&folder, &options);
        let out = leaves.iter().filter(|leaves| **leaves).count();
        assert_eq!(output.status.code(), Some(0), "{row}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            removed(out, 2 - out, size, store_bytes(&folder)),
            "{row}"
        );
        let mut kept: Vec<String> = filled
            .iter()
            .zip(leaves)
            .filter(|(_, leaves)| !leaves)
            .map(|(key, _)| key.clone())
            .collect();
        kept.sort();
        assert_eq!(held(&folder), kept, "{row}");
    }

    let empty = folder("prune-alias-empty");
    fs::create_dir_all(&empty).expect("empty folder");
    let output = prune(&empty, &["--answered-by-other-than", "jev-latest"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "removed 0 answers and 0 bytes; 0 answers and 0 bytes remain\n"
    );
    assert!(!empty.join("thinkthen.sqlite").exists());
}

#[test]
fn prune_shrinks_the_store_file_and_takes_the_oldest_answer_first() {
    let folder = folder("prune-shrinks");
    // Long records make each answer span many pages, so a removed answer
    // frees pages the file can give back.
    let evidence = "A long record about a refund. ".repeat(1_500);
    let filled = fill(
        &folder,
        &evidence,
        &[
            (DEFAULT_MODEL, "asks for a refund", ANSWERED),
            (DEFAULT_MODEL, "asks for a repair", ANSWERED),
            (DEFAULT_MODEL, "asks for a discount", ANSWERED),
        ],
    );
    taken_at(
        &folder,
        &[(&filled[0], 30), (&filled[1], 10), (&filled[2], 20)],
    );
    let before = store_bytes(&folder);
    // Two thirds of the file is what two answers weigh, so a target just
    // above that keeps two and removes the oldest.
    let target = (before * 4 / 5).to_string();
    let preview = prune(&folder, &["--max-size", &target, "--dry-run"]);
    assert_eq!(preview.status.code(), Some(0));
    let text = String::from_utf8(preview.stdout).expect("preview text");
    let lines: Vec<&str> = text.lines().collect();
    assert!(lines[0].starts_with("selected 1 answers and "), "{text}");
    assert!(lines[0].contains("; 2 answers and "), "{text}");
    assert_eq!(&lines[1..], [format!("selected {}", filled[1])], "{text}");
    assert_eq!(store_bytes(&folder), before);

    let output = prune(&folder, &["--max-size", &target]);
    assert_eq!(output.status.code(), Some(0));
    let after = store_bytes(&folder);
    assert!(after < before, "{after} < {before}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        removed(1, 2, before, after)
    );
    let mut kept = vec![filled[0].clone(), filled[2].clone()];
    kept.sort();
    assert_eq!(held(&folder), kept);
}

#[test]
fn older_than_selects_by_taken_at_and_old_entries_stay_beside_the_store() {
    let folder = folder("prune-older-than");
    let filled = fill(
        &folder,
        EVIDENCE,
        &[
            (DEFAULT_MODEL, "asks for a refund", ANSWERED),
            (DEFAULT_MODEL, "asks for a repair", ANSWERED),
        ],
    );
    taken_at(&folder, &[(&filled[0], 0)]);
    // Prune reads only the store; an old digest-named entry, a stray file
    // and a directory are the owner's, and stay byte for byte.
    let old = format!("{}.json", "0".repeat(64));
    fs::write(folder.join(&old), b"an old entry").expect("old entry");
    fs::write(folder.join("sentinel"), b"keep me").expect("sentinel");
    fs::create_dir(folder.join(format!("{}.json", "f".repeat(64)))).expect("directory");
    let before = store_bytes(&folder);
    let output = prune(&folder, &["--older-than", "1d"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        removed(1, 1, before, store_bytes(&folder))
    );
    assert_eq!(held(&folder), [filled[1].clone()]);
    assert_eq!(fs::read(folder.join(old)).expect("old"), b"an old entry");
    assert_eq!(fs::read(folder.join("sentinel")).expect("kept"), b"keep me");
}

#[test]
fn an_unreadable_folder_fails_prune_before_any_change() {
    use std::os::unix::fs::PermissionsExt as _;

    let folder = folder("prune-unreadable-folder");
    fill(
        &folder,
        EVIDENCE,
        &[(DEFAULT_MODEL, "asks for a refund", ANSWERED)],
    );
    let before = fs::read(folder.join("thinkthen.sqlite")).expect("before");
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o400)).expect("remove search");
    // A privileged host can still inspect this folder; it cannot prove this error path.
    let inaccessible = fs::symlink_metadata(folder.join("thinkthen.sqlite")).is_err();
    let output = inaccessible.then(|| prune(&folder, &[]));
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o700)).expect("restore search");
    if let Some(output) = output {
        assert_eq!(output.status.code(), Some(5));
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8(output.stderr).expect("diagnostic"),
            "thinkthen: the recording folder could not be read or written; check its permissions and free space\n"
        );
        assert_eq!(
            fs::read(folder.join("thinkthen.sqlite")).expect("after"),
            before
        );
    }
}
