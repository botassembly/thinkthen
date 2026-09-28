use std::fs;
use std::io;
use std::path::Path;

use super::{ANSWERED, EVIDENCE, folder, plant, run};
use crate::support::{
    DEFAULT_BASE, DEFAULT_MODEL, ENDPOINT_PATH, encoded_decide, plant_backend_identity,
    plant_recording,
};

#[test]
fn prune_selects_by_model_and_names_a_bad_entry() {
    let folder = folder("cache-prune-model");
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    let current = plant_recording(
        &folder,
        &url,
        &encoded_decide(EVIDENCE, "jev-latest", "asks for a refund"),
        ANSWERED,
    )
    .expect("current");
    plant_backend_identity(&folder, &url).expect("marker");
    let old_response = ANSWERED.replace("jev-1.13.0", "jev-old");
    let old = plant_recording(
        &folder,
        &url,
        &encoded_decide(EVIDENCE, "jev-latest", "another question"),
        &old_response,
    )
    .expect("old");
    let output = run(
        &[
            "cache",
            "prune",
            folder.to_str().expect("folder"),
            "--max-size",
            "100000000",
            "--answered-by-other-than",
            "jev-1.13.0",
        ],
        &[],
    )
    .expect("prune");
    assert_eq!(output.status.code(), Some(0));
    assert!(folder.join(&current).exists());
    assert!(!folder.join(old).exists());

    let removable = plant_recording(
        &folder,
        &url,
        &encoded_decide(EVIDENCE, "jev-latest", "third question"),
        &old_response,
    )
    .expect("old");
    let bad = plant_recording(
        &folder,
        &url,
        &encoded_decide("private malformed marker", "jev-latest", "bad question"),
        &ANSWERED.replace("jev-1.13.0", "jev-latest"),
    )
    .expect("bad source");
    let bad_name = "0".repeat(64) + ".json";
    fs::rename(folder.join(bad), folder.join(&bad_name)).expect("mismatch");
    let output = run(
        &[
            "cache",
            "prune",
            folder.to_str().expect("folder"),
            "--answered-by-other-than",
            "jev-1.13.0",
        ],
        &[],
    )
    .expect("prune");
    assert_eq!(output.status.code(), Some(0));
    assert!(!folder.join(removable).exists());
    assert!(folder.join(&current).exists());
    assert!(folder.join(&bad_name).exists());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!("thinkthen: cache prune: left `{bad_name}` in place; it is not a valid entry\n")
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("private malformed marker"));
    let alias = run(
        &[
            "cache",
            "prune",
            folder.to_str().expect("folder"),
            "--answered-by-other-than",
            "jev-latest",
        ],
        &[],
    )
    .expect("alias");
    assert_eq!(alias.status.code(), Some(2));
    assert!(alias.stdout.is_empty());
    assert!(alias.stderr.starts_with(
        b"thinkthen: --answered-by-other-than names the model the requests asked for"
    ));
    assert!(!String::from_utf8_lossy(&alias.stderr).contains(&bad_name));
    assert!(folder.join(current).exists());
    assert!(folder.join(bad_name).exists());
}

/// The allocated bytes of one entry, as prune counts them.
fn allocated(folder: &Path, name: &str) -> io::Result<u64> {
    use std::os::unix::fs::MetadataExt as _;
    Ok(fs::metadata(folder.join(name))?.blocks() * 512)
}

#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a malformed CLI response stops the shared outside-in fixture assertion"
)]
fn assert_one_good_one_bad(folder: &Path) {
    let status = run(
        &["status", "--json"],
        &[("THINKTHEN_CACHE", folder.to_str().expect("folder"))],
    )
    .expect("status");
    assert_eq!(status.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&status.stdout).expect("status JSON");
    assert_eq!(value["cache"]["entries"], 1);
    assert_eq!(value["cache"]["bad_entries"], 1);
}

/// MODEL, other options, the second entry's reply, and which of the two
/// entries leave, or the refusal that deletes nothing.
type PruneRow<'a> = (&'a str, &'a [&'a str], &'a str, Result<[bool; 2], &'a str>);

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
        "or delete the folder to remove every entry\n",
    );
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    let echoed = ANSWERED.replace("jev-1.13.0", "jev-latest");
    let upgraded = ANSWERED.replace("jev-1.13.0", "jev-1.14.0");
    let rows: [PruneRow; 9] = [
        ("jev-latest", &[], ANSWERED, Err(ALIAS)),
        ("jev-latest", &["--older-than", "1d"], ANSWERED, Err(ALIAS)),
        ("jev-1.14.0", &[], ANSWERED, Err(UNKNOWN)),
        ("no-such-model", &[], ANSWERED, Err(UNKNOWN)),
        ("jev-1.13", &["--older-than", "1d"], ANSWERED, Err(UNKNOWN)),
        ("JEV-LATEST", &[], ANSWERED, Err(UNKNOWN)),
        ("jev-1.13.0", &[], ANSWERED, Ok([false, false])),
        ("jev-latest", &[], &echoed, Ok([true, false])),
        ("jev-1.14.0", &[], &upgraded, Ok([true, false])),
    ];
    for (row, (model, options, second_reply, removed)) in rows.into_iter().enumerate() {
        let folder = folder(&format!("prune-alias-{row}"));
        let first = plant(&folder, ANSWERED).expect("first entry");
        let request = encoded_decide(EVIDENCE, "jev-latest", "another question");
        let second = plant_recording(&folder, &url, &request, second_reply).expect("second");
        let names = [&first, &second];
        let bytes = [&first, &second].map(|name| allocated(&folder, name).expect("allocated"));
        let before = [
            fs::read(folder.join(&first)),
            fs::read(folder.join(&second)),
        ]
        .map(|read| read.expect("entry"));
        let mut arguments = vec!["cache", "prune", folder.to_str().expect("folder")];
        arguments.extend(["--answered-by-other-than", model]);
        arguments.extend_from_slice(options);
        let output = run(&arguments, &[]).expect("prune");
        let removed = match removed {
            Ok(removed) => removed,
            Err(refusal) => {
                assert_eq!(output.status.code(), Some(2), "{row}");
                assert!(output.stdout.is_empty(), "{row}");
                assert_eq!(String::from_utf8_lossy(&output.stderr), refusal, "{row}");
                for (name, bytes) in names.iter().zip(&before) {
                    assert_eq!(&fs::read(folder.join(name)).expect("kept"), bytes, "{row}");
                }
                continue;
            }
        };
        let out = removed.iter().filter(|leaves| **leaves).count();
        let out_bytes: u64 = (0..2)
            .filter(|index| removed[*index])
            .map(|index| bytes[index])
            .sum();
        let (kept, kept_bytes) = (2 - out, bytes.iter().sum::<u64>() - out_bytes);
        assert_eq!(output.status.code(), Some(0), "{row}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            format!(
                "removed {out} entries and {out_bytes} bytes; {kept} entries and {kept_bytes} bytes remain\n"
            ),
            "{row}"
        );
        for (index, name) in names.iter().enumerate() {
            assert_eq!(folder.join(name).exists(), !removed[index], "{row}");
        }
    }

    let empty = folder("prune-alias-empty");
    fs::create_dir_all(&empty).expect("empty folder");
    plant_backend_identity(&empty, &url).expect("marker");
    let output = run(
        &[
            "cache",
            "prune",
            empty.to_str().expect("folder"),
            "--answered-by-other-than",
            "jev-latest",
        ],
        &[],
    )
    .expect("prune");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "removed 0 entries and 0 bytes; 0 entries and 0 bytes remain\n"
    );
}

#[test]
fn prune_leaves_and_names_each_kind_of_bad_entry() {
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    for (case, malformed) in [
        "mismatch",
        "blank-model",
        "directory",
        "schema",
        "unopenable",
    ]
    .into_iter()
    .enumerate()
    {
        let folder = folder(malformed);
        let keep = plant(&folder, ANSWERED).expect("removable entry");
        let bytes = allocated(&folder, &keep).expect("allocated entry");
        let mut bad_name = "0".repeat(64) + ".json";
        match case {
            0 => {
                let planted = plant_recording(
                    &folder,
                    &url,
                    &encoded_decide(EVIDENCE, DEFAULT_MODEL, "different"),
                    ANSWERED,
                )
                .expect("mismatched source");
                fs::rename(folder.join(planted), folder.join(&bad_name)).expect("mismatched name");
            }
            1 => {
                let response = ANSWERED.replace("jev-1.13.0", " ");
                bad_name = plant_recording(
                    &folder,
                    &url,
                    &encoded_decide(EVIDENCE, DEFAULT_MODEL, "blank model"),
                    &response,
                )
                .expect("blank-model entry");
            }
            2 => fs::create_dir(folder.join(&bad_name)).expect("digest-shaped directory"),
            3 | 4 => {
                let planted = plant_recording(
                    &folder,
                    &url,
                    &encoded_decide(EVIDENCE, DEFAULT_MODEL, malformed),
                    ANSWERED,
                )
                .expect("source");
                bad_name = planted;
                if case == 3 {
                    let text = fs::read_to_string(folder.join(&bad_name)).expect("entry");
                    assert!(text.contains("thinkthen.recording/1"));
                    fs::write(
                        folder.join(&bad_name),
                        text.replace("thinkthen.recording/1", "thinkthen.recording/2"),
                    )
                    .expect("foreign schema");
                } else {
                    use std::os::unix::fs::PermissionsExt as _;
                    fs::set_permissions(folder.join(&bad_name), fs::Permissions::from_mode(0o000))
                        .expect("unopenable");
                }
            }
            _ => unreachable!(),
        }
        assert_one_good_one_bad(&folder);
        let output = run(
            &[
                "cache",
                "prune",
                folder.to_str().expect("folder"),
                "--max-size",
                "1",
            ],
            &[],
        )
        .expect("prune");
        assert_eq!(output.status.code(), Some(0), "{malformed}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            format!("removed 1 entries and {bytes} bytes; 0 entries and 0 bytes remain\n"),
            "{malformed}"
        );
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!(
                "thinkthen: cache prune: left `{bad_name}` in place; it is not a valid entry\n"
            ),
            "{malformed}"
        );
        assert!(!folder.join(keep).exists(), "{malformed}");
        assert!(folder.join(bad_name).exists(), "{malformed}");
    }
}

#[test]
fn prune_names_multiple_bad_entries_in_name_order() {
    let folder = folder("cache-prune-bad-order");
    let high = format!("{}.json", "f".repeat(64));
    let low = format!("{}.json", "0".repeat(64));
    fs::create_dir_all(&folder).expect("folder");
    fs::create_dir(folder.join(&high)).expect("high bad entry");
    fs::create_dir(folder.join(&low)).expect("low bad entry");
    let output = run(&["cache", "prune", folder.to_str().expect("folder")], &[]).expect("prune");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "removed 0 entries and 0 bytes; 0 entries and 0 bytes remain\n"
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "thinkthen: cache prune: left `{low}` in place; it is not a valid entry\n\
             thinkthen: cache prune: left `{high}` in place; it is not a valid entry\n"
        )
    );
}

#[cfg(unix)]
#[test]
fn prune_names_a_digest_shaped_symlink_without_following_it() {
    use std::os::unix::fs::symlink;

    let folder = folder("cache-prune-symlink");
    let keep = plant(&folder, ANSWERED).expect("removable entry");
    let bytes = allocated(&folder, &keep).expect("allocated entry");
    let outside = folder.with_extension("outside");
    let url = format!("{DEFAULT_BASE}/{ENDPOINT_PATH}");
    let name = plant_recording(
        &outside,
        &url,
        &encoded_decide(EVIDENCE, DEFAULT_MODEL, "outside question"),
        ANSWERED,
    )
    .expect("outside entry");
    let before = fs::read(outside.join(&name)).expect("outside bytes");
    symlink(outside.join(&name), folder.join(&name)).expect("symlink");
    assert_one_good_one_bad(&folder);
    let output = run(
        &[
            "cache",
            "prune",
            folder.to_str().expect("folder"),
            "--max-size",
            "1",
        ],
        &[],
    )
    .expect("prune");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("removed 1 entries and {bytes} bytes; 0 entries and 0 bytes remain\n")
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!("thinkthen: cache prune: left `{name}` in place; it is not a valid entry\n")
    );
    assert!(!folder.join(keep).exists());
    assert!(folder.join(&name).exists());
    assert_eq!(
        fs::read(outside.join(name)).expect("outside target"),
        before
    );
    fs::remove_dir_all(outside).expect("outside cleanup");
}
