//! A usage folder without `.lock` (tickets 0360 and 0367).

use std::fs;

use super::super::{Counters, read};
use super::folder;
use crate::engine::error::Error;

/// The writer makes `.lock` before any month, so a folder without one holds
/// nothing thinkthen wrote. Ticket 0360 refused such a folder, so two runs
/// that started together on a new folder could refuse each other; main then
/// read it as zero without looking, so a malformed month passed the send
/// check and failed the writer after the send (ticket 0367). Its months are
/// now read without the lock.
#[cfg(unix)]
#[test]
fn a_folder_without_a_lock_reads_its_months_without_the_lock() {
    use std::os::unix::fs::{DirBuilderExt as _, PermissionsExt as _};

    let valid = b"{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":2,\"input_tokens\":0,\"output_tokens\":0,\"cache_answers\":0}\n";
    let malformed = "cannot read the usage totals: 2026-09.json has invalid contents. Move it out of the usage folder that thinkthen status names, and counting starts again.";
    let unsafe_mode = "cannot read the usage totals: 2026-09.json has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it out of the usage folder that thinkthen status names.";
    let cases: [(&str, Option<&[u8]>, u32, Result<u64, &str>); 5] = [
        ("empty", None, 0o600, Ok(0)),
        ("valid", Some(valid), 0o600, Ok(2)),
        ("garbage", Some(b"garbage\n"), 0o600, Err(malformed)),
        ("zero-bytes", Some(b""), 0o600, Err(malformed)),
        ("shared", Some(valid), 0o644, Err(unsafe_mode)),
    ];
    for (label, month, mode, expected) in cases {
        let folder = folder(&format!("no-lock-{label}"));
        fs::DirBuilder::new()
            .mode(0o700)
            .create(&folder)
            .expect("private folder");
        if let Some(bytes) = month {
            let file = folder.join("2026-09.json");
            fs::write(&file, bytes).expect("month");
            fs::set_permissions(&file, fs::Permissions::from_mode(mode)).expect("mode");
        }
        let read_back = read(&folder, "2026-09").map(|totals| totals.total.requests_sent);
        let checked = Counters::new(Some(folder.clone())).check_readable();
        match expected {
            Ok(sent) => {
                assert_eq!(read_back.expect(label), sent, "{label}");
                assert!(checked.is_ok(), "{label}");
            }
            Err(sentence) => {
                assert_eq!(
                    read_back.expect_err(label).sentence(None),
                    sentence,
                    "{label}"
                );
                assert!(
                    matches!(checked, Err(Error::UsageUnreadable(said)) if said == sentence),
                    "{label}"
                );
            }
        }
        assert!(
            !folder.join(".lock").exists(),
            "{label}: the read made a lock"
        );
    }
}
