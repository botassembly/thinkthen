use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

use super::{
    CREATION_PAUSE, Counters, Counts, FAILURE, INITIAL_SYNC, Stage, month_now, read,
    recognized_month, update, year_month,
};

static FOLDERS: AtomicU64 = AtomicU64::new(0);

fn folder(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "thinkthen-usage-{name}-{}-{}",
        std::process::id(),
        FOLDERS.fetch_add(1, Ordering::Relaxed)
    ));
    let _absent = fs::remove_dir_all(&path);
    path
}

#[test]
fn utc_calendar_months_and_recognized_names_are_exact() {
    assert_eq!(year_month(0), (1970, 1));
    assert_eq!(year_month(20_718), (2026, 9));
    for name in ["0001-01.json", "2026-09.json", "9999-12.json"] {
        assert!(recognized_month(name));
    }
    for name in [
        "0000-01.json",
        "2026-00.json",
        "2026-13.json",
        "26-09.json",
        "2026-09.jsonx",
    ] {
        assert!(!recognized_month(name));
    }
}

#[test]
fn concurrent_updates_keep_every_count_in_one_monthly_aggregate() {
    let folder = folder("concurrent");
    let _absent = fs::remove_dir_all(&folder);
    let counters = Arc::new(Counters::new(Some(folder.clone())));
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let counters = Arc::clone(&counters);
            thread::spawn(move || {
                (0..10).for_each(|_| counters.request_sent());
            })
        })
        .collect();
    for worker in workers {
        worker.join().expect("counter worker");
    }
    counters.finish();
    let totals = read(&folder, &month_now()).expect("usage reads");
    assert_eq!(totals.month.requests_sent, 40);
    assert_eq!(totals.total.requests_sent, 40);
    assert_eq!(fs::read_dir(&folder).expect("folder").count(), 2);
    let bytes = fs::read(folder.join(format!("{}.json", month_now()))).expect("month");
    let row: Counts = serde_json::from_slice(&bytes).expect("closed row");
    assert_eq!(row.requests_sent, 40);
    fs::remove_dir_all(folder).expect("cleanup");
}

#[test]
fn an_existing_opener_that_overtakes_the_creator_establishes_lock_durability() {
    let folder = folder("overtaking-opener");
    let created = Arc::new(std::sync::Barrier::new(2));
    let release = Arc::new(std::sync::Barrier::new(2));
    let creator_folder = folder.clone();
    let creator_created = Arc::clone(&created);
    let creator_release = Arc::clone(&release);
    let creator = thread::spawn(move || {
        CREATION_PAUSE.with(|pause| {
            *pause.borrow_mut() = Some((creator_created, creator_release));
        });
        update(
            &creator_folder,
            "2026-09",
            Counts {
                requests_sent: 1,
                ..Counts::default()
            },
        )
    });
    created.wait();
    let opener_folder = folder.clone();
    let opener = thread::spawn(move || {
        let result = update(
            &opener_folder,
            "2026-09",
            Counts {
                requests_sent: 1,
                ..Counts::default()
            },
        );
        let established = INITIAL_SYNC.with(std::cell::Cell::get);
        (result, established)
    });
    let (result, established) = opener.join().expect("existing opener");
    result.expect("existing opener update");
    assert!(established, "the opener established the stable lock");
    release.wait();
    creator.join().expect("creator").expect("creator update");
    assert_eq!(
        read(&folder, "2026-09")
            .expect("totals")
            .month
            .requests_sent,
        2
    );
}

#[test]
fn rollover_total_and_crash_residue_keep_the_last_complete_months() {
    let folder = folder("rollover");
    update(
        &folder,
        "2026-08",
        Counts {
            requests_sent: 2,
            ..Counts::default()
        },
    )
    .expect("August");
    update(
        &folder,
        "2026-09",
        Counts {
            cache_answers: 3,
            ..Counts::default()
        },
    )
    .expect("September");
    fs::write(folder.join(".update.tmp"), b"interrupted private bytes").expect("residue");
    let totals = read(&folder, "2026-09").expect("totals");
    assert_eq!(totals.month.cache_answers, 3);
    assert_eq!(totals.total.requests_sent, 2);
    assert_eq!(totals.total.cache_answers, 3);
}

#[test]
fn checked_month_and_total_addition_refuse_overflow_without_replacing_good_state() {
    let folder = folder("overflow");
    update(
        &folder,
        "2026-08",
        Counts {
            requests_sent: u64::MAX,
            ..Counts::default()
        },
    )
    .expect("maximum");
    assert!(
        update(
            &folder,
            "2026-08",
            Counts {
                requests_sent: 1,
                ..Counts::default()
            }
        )
        .is_err()
    );
    assert_eq!(
        read(&folder, "2026-08")
            .expect("last good")
            .month
            .requests_sent,
        u64::MAX
    );
    update(
        &folder,
        "2026-09",
        Counts {
            requests_sent: 1,
            ..Counts::default()
        },
    )
    .expect("other month");
    assert!(read(&folder, "2026-09").is_err(), "total overflow");
}

#[test]
fn every_update_stage_warns_once_and_disables_later_persistence() {
    for stage in [
        Stage::Setup,
        Stage::Lock,
        Stage::Validation,
        Stage::Write,
        Stage::FileSync,
        Stage::Rename,
        Stage::DirectorySync,
    ] {
        let folder = folder(&format!("failure-{stage:?}"));
        if stage != Stage::Setup {
            update(&folder, "2026-09", Counts::default()).expect("baseline");
        }
        let counters = Counters::new(Some(folder.clone()));
        FAILURE.with(|failure| failure.set(Some(stage)));
        counters.request_sent();
        assert!(counters.finish(), "{stage:?}");
        let after_failure = read(&folder, "2026-09")
            .map(|totals| totals.month.requests_sent)
            .unwrap_or(0);
        counters.request_sent();
        counters.finish();
        let after_disabled = read(&folder, "2026-09")
            .map(|totals| totals.month.requests_sent)
            .unwrap_or(0);
        assert_eq!(after_disabled, after_failure, "{stage:?}");
    }
}

#[cfg(unix)]
#[test]
fn strict_reader_refuses_symlink_nonregular_and_unsafe_modes() {
    use std::os::unix::fs::{PermissionsExt as _, symlink};

    for case in [
        "symlink",
        "directory",
        "month-mode",
        "lock-mode",
        "folder-mode",
    ] {
        let folder = folder(case);
        update(&folder, "2026-09", Counts::default()).expect("baseline");
        let month = folder.join("2026-09.json");
        match case {
            "symlink" => {
                fs::rename(&month, folder.join("target")).expect("target");
                symlink(folder.join("target"), &month).expect("symlink");
            }
            "directory" => {
                fs::remove_file(&month).expect("month");
                fs::create_dir(&month).expect("directory");
            }
            "month-mode" => {
                fs::set_permissions(&month, fs::Permissions::from_mode(0o644)).expect("mode")
            }
            "lock-mode" => {
                fs::set_permissions(folder.join(".lock"), fs::Permissions::from_mode(0o644))
                    .expect("mode")
            }
            "folder-mode" => {
                fs::set_permissions(&folder, fs::Permissions::from_mode(0o755)).expect("mode")
            }
            _ => unreachable!(),
        }
        assert!(read(&folder, "2026-09").is_err(), "{case}");
    }
}
