use std::fs;
use std::os::unix::fs::MetadataExt as _;

use super::{Counts, FAILURE, Stage, folder, read, update};

#[test]
fn a_clean_month_keeps_zero_retries_absent_and_writes_changed_retries() {
    let folder = folder("clean-sidecar");
    let sidecar = folder.join("retries-2026-09.json");
    update(
        &folder,
        "2026-09",
        Counts {
            requests_sent: 1,
            ..Counts::default()
        },
    )
    .expect("first call");
    assert!(!sidecar.exists(), "zero retries need no sidecar");
    assert_eq!(
        read(&folder, "2026-09")
            .expect("totals")
            .month
            .requests_sent,
        1
    );

    update(
        &folder,
        "2026-09",
        Counts {
            requests_sent: 1,
            retries: 1,
            ..Counts::default()
        },
    )
    .expect("retry call");
    let totals = read(&folder, "2026-09").expect("totals").month;
    assert_eq!((totals.requests_sent, totals.retries), (2, 1));
    assert!(sidecar.is_file(), "changed retries persist a sidecar");
}

#[test]
fn an_unrelated_delta_preserves_a_validated_retry_sidecar_inode() {
    let folder = folder("unchanged-sidecar");
    let sidecar = folder.join("retries-2026-09.json");
    update(
        &folder,
        "2026-09",
        Counts {
            requests_sent: 1,
            retries: 7,
            ..Counts::default()
        },
    )
    .expect("retry baseline");
    let before = fs::metadata(&sidecar).expect("sidecar metadata");
    let bytes = fs::read(&sidecar).expect("sidecar bytes");

    FAILURE.with(|failure| failure.set(Some(Stage::RetryWrite)));
    update(
        &folder,
        "2026-09",
        Counts {
            requests_sent: 1,
            ..Counts::default()
        },
    )
    .expect("no-retry delta skips retry write");
    FAILURE.with(|failure| failure.set(None));
    let after = fs::metadata(&sidecar).expect("retained sidecar metadata");
    assert_eq!((after.dev(), after.ino()), (before.dev(), before.ino()));
    assert_eq!(fs::read(&sidecar).expect("retained sidecar bytes"), bytes);
    let totals = read(&folder, "2026-09").expect("totals").month;
    assert_eq!((totals.requests_sent, totals.retries), (2, 7));
}

#[test]
fn a_retry_only_delta_replaces_the_sidecar_after_the_base() {
    let folder = folder("retry-only");
    let sidecar = folder.join("retries-2026-09.json");
    update(
        &folder,
        "2026-09",
        Counts {
            requests_sent: 1,
            retries: 1,
            ..Counts::default()
        },
    )
    .expect("baseline");
    let prior = fs::metadata(&sidecar).expect("sidecar metadata");
    update(
        &folder,
        "2026-09",
        Counts {
            retries: 1,
            ..Counts::default()
        },
    )
    .expect("retry only");
    let current = fs::metadata(&sidecar).expect("changed sidecar metadata");
    assert_ne!((current.dev(), current.ino()), (prior.dev(), prior.ino()));
    let totals = read(&folder, "2026-09").expect("totals").month;
    assert_eq!((totals.requests_sent, totals.retries), (1, 2));
}

#[test]
fn a_no_retry_delta_still_refuses_an_invalid_sidecar_before_base_mutation() {
    let folder = folder("invalid-unchanged-sidecar");
    let month = folder.join("2026-09.json");
    let sidecar = folder.join("retries-2026-09.json");
    update(
        &folder,
        "2026-09",
        Counts {
            requests_sent: 1,
            retries: 1,
            ..Counts::default()
        },
    )
    .expect("baseline");
    fs::write(
        &sidecar,
        b"{\"schema\":\"thinkthen.usage.retries/2\",\"retries\":1}\n",
    )
    .expect("corrupt recognized sidecar");
    let before = fs::read(&month).expect("durable base");
    assert!(
        update(
            &folder,
            "2026-09",
            Counts {
                requests_sent: 1,
                ..Counts::default()
            },
        )
        .is_err()
    );
    assert_eq!(fs::read(&month).expect("base remains"), before);
}
