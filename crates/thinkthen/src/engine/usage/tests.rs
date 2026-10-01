use std::fs;
use std::io::ErrorKind;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::http::{Client, Exchange, Key};

use super::{
    CREATION_PAUSE, Counters, Counts, FAILURE, INITIAL_SYNC, Shared, Stage, month_now, read,
    recognized_month, year_month,
};

static FOLDERS: AtomicU64 = AtomicU64::new(0);
/// Exercise the production update with a fresh queue before finalization.
fn update(path: &std::path::Path, month: &str, delta: Counts) -> std::io::Result<()> {
    super::update(path, month, delta, &Shared::default())
}

#[test]
fn an_old_usage_row_without_retries_reads_as_zero() {
    let row: Counts = serde_json::from_str(
        r#"{"schema":"thinkthen.usage/1","requests_sent":3,"input_tokens":1,"output_tokens":2,"cache_answers":0}"#,
    )
    .expect("old usage schema");
    assert_eq!(row.requests_sent, 3);
    assert_eq!(row.retries, 0);
}

#[test]
fn an_uncountable_attempt_is_refused_before_transport() {
    let counts = Counters::new(None);
    counts.add(Counts {
        requests_sent: u64::MAX,
        ..Counts::default()
    });
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!(
        "http://{}/v1/systemone",
        listener.local_addr().expect("address")
    );
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url: &url,
        body: b"{}",
        key: &key,
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
    };
    let result = Client::new(
        Duration::from_secs(1),
        false,
        &crate::engine::limits::process().widths,
    )
    .post_observed_with_retry(&exchange, &Cancel::default(), &counts, |_| ());

    assert!(matches!(
        result,
        Err(Error::Defect("request attempt count overflow"))
    ));
    assert_eq!(counts.snapshot().requests_sent, u64::MAX);
    assert_eq!(counts.snapshot().retries, 0);
    assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
}

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
    assert!(!String::from_utf8_lossy(&bytes).contains("retries"));
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
fn an_invalid_older_month_refuses_the_update_before_any_write() {
    let folder = folder("invalid-old-month");
    update(&folder, "2026-08", Counts::default()).expect("baseline");
    fs::write(
        folder.join("2026-08.json"),
        b"{\"schema\":\"thinkthen.usage/2\",\"requests_sent\":1,\"input_tokens\":0,\"output_tokens\":0,\"cache_answers\":0}\n",
    )
    .expect("future schema");
    assert!(update(&folder, "2026-09", Counts::default()).is_err());
    assert!(!folder.join("2026-09.json").exists());
}

#[test]
fn checked_month_and_total_addition_refuse_overflow_without_replacing_good_state() {
    let folder = folder("overflow");
    update(
        &folder,
        "2026-08",
        Counts {
            requests_sent: u64::MAX,
            retries: 1,
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
    let august = folder.join("2026-08.json");
    let before_month = fs::read(&august).expect("last good bytes");
    assert!(
        update(
            &folder,
            "2026-09",
            Counts {
                requests_sent: 1,
                ..Counts::default()
            },
        )
        .is_err(),
        "individually valid month would overflow the aggregate"
    );
    assert_eq!(fs::read(august).expect("unchanged month"), before_month);
    assert!(!folder.join("2026-09.json").exists());
    assert_eq!(
        read(&folder, "2026-08")
            .expect("readable total")
            .total
            .requests_sent,
        u64::MAX
    );
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

#[test]
fn interrupted_first_write_never_publishes_an_empty_month() {
    for stage in [Stage::Write, Stage::FileSync, Stage::Rename] {
        let folder = folder(&format!("first-write-{stage:?}"));
        FAILURE.with(|failure| failure.set(Some(stage)));
        assert!(update(&folder, "2026-09", Counts::default()).is_err());
        assert!(
            !folder.join("2026-09.json").exists(),
            "{stage:?} published a canonical month"
        );
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
        update(
            &folder,
            "2026-09",
            Counts {
                retries: 1,
                ..Counts::default()
            },
        )
        .expect("baseline");
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

/// QA's case on main: an older build's month file holds `retries` beside a
/// `retries-` file with another value, and main refused to choose. One file
/// now holds all five counts, and any `retries-` file is not a month.
#[cfg(unix)]
#[test]
fn one_month_file_holds_every_count_and_an_old_retries_file_is_ignored() {
    use std::os::unix::fs::PermissionsExt as _;

    let folder = folder("one-file");
    update(&folder, "2026-09", Counts::default()).expect("private folder and lock");
    let month = folder.join("2026-09.json");
    fs::write(&month, b"{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":4,\"retries\":2,\"input_tokens\":7,\"output_tokens\":3,\"cache_answers\":1}\n").expect("month");
    let old = folder.join("retries-2026-09.json");
    fs::write(&old, b"{\"schema\":\"thinkthen.usage.retries/1\",\"retries\":5}\n").expect("old");
    fs::set_permissions(&old, fs::Permissions::from_mode(0o600)).expect("private");
    let totals = read(&folder, "2026-09").expect("the month file alone");
    assert_eq!((totals.month.requests_sent, totals.month.retries), (4, 2));
    let retry = Counts {
        requests_sent: 1,
        retries: 1,
        ..Counts::default()
    };
    update(&folder, "2026-10", retry).expect("a new month");
    assert_eq!(
        fs::read_to_string(folder.join("2026-10.json")).expect("written"),
        "{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":1,\"retries\":1,\"input_tokens\":0,\"output_tokens\":0,\"cache_answers\":0}\n"
    );
    assert!(!folder.join("retries-2026-10.json").exists());
    let total = read(&folder, "2026-10").expect("both months").total;
    assert_eq!((total.requests_sent, total.retries), (5, 3));
}

/// The writer makes `.lock` before any month, so a folder without one holds
/// nothing thinkthen wrote. Main refused it, so two runs that started
/// together on a new folder could refuse each other.
#[cfg(unix)]
#[test]
fn a_folder_without_a_lock_reads_as_zero() {
    use std::os::unix::fs::DirBuilderExt as _;

    let folder = folder("no-lock");
    fs::DirBuilder::new().mode(0o700).create(&folder).expect("private folder");
    let totals = read(&folder, "2026-09").expect("zero");
    assert_eq!(totals.total, Counts::default());
    assert!(Counters::new(Some(folder)).check_readable().is_ok());
}

/// Main waited without end for a held lock. The read now gives up after one
/// second; status names the lock as busy, and the send check passes.
#[test]
fn a_held_lock_is_busy_after_one_second_and_the_send_check_passes() {
    let folder = folder("held-lock");
    update(&folder, "2026-09", Counts::default()).expect("baseline");
    let holder = fs::File::open(folder.join(".lock")).expect("lock");
    holder.lock().expect("exclusive");
    let started = std::time::Instant::now();
    let failure = read(&folder, "2026-09").expect_err("busy");
    assert!(failure.busy());
    assert!(started.elapsed() < Duration::from_secs(3));
    assert_eq!(
        failure.sentence(Some(&folder)),
        format!(
            "cannot read the usage totals: {} is locked by another process. Try again when it finishes.",
            folder.join(".lock").display()
        )
    );
    assert!(Counters::new(Some(folder)).check_readable().is_ok());
}

/// The check keeps its result, so a repaired file does not resume counting
/// mid-process and every later send in the process refuses too.
#[test]
fn a_refused_check_stays_refused_and_names_only_the_file() {
    let folder = folder("kept-refusal");
    let month = month_now();
    update(&folder, &month, Counts::default()).expect("baseline");
    let file = folder.join(format!("{month}.json"));
    fs::write(&file, b"not JSON").expect("malformed");
    let counters = Counters::new(Some(folder.clone()));
    let sentence = format!(
        "cannot read the usage totals: {month}.json has invalid contents. Move it out of the usage folder that thinkthen status names, and counting starts again."
    );
    for _ in 0..2 {
        match counters.check_readable() {
            Err(Error::UsageUnreadable(said)) => assert_eq!(said, sentence),
            other => panic!("{other:?}"),
        }
    }
    fs::remove_file(&file).expect("repaired");
    assert!(counters.check_readable().is_err());
    assert!(!sentence.contains(&folder.display().to_string()));
}

#[cfg(unix)]
#[test]
fn each_unsafe_sentence_names_the_fix() {
    use std::os::unix::fs::PermissionsExt as _;

    let folder = folder("unsafe-sentence");
    update(&folder, "2026-09", Counts::default()).expect("baseline");
    let month = folder.join("2026-09.json");
    fs::set_permissions(&month, fs::Permissions::from_mode(0o644)).expect("mode");
    let failure = read(&folder, "2026-09").expect_err("unsafe");
    assert_eq!(
        failure.sentence(None),
        "cannot read the usage totals: 2026-09.json has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it out of the usage folder that thinkthen status names."
    );
    assert_eq!(
        failure.sentence(Some(&folder)),
        format!(
            "cannot read the usage totals: {} has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it aside.",
            month.display()
        )
    );
    fs::set_permissions(&month, fs::Permissions::from_mode(0o600)).expect("mode");
    fs::set_permissions(&folder, fs::Permissions::from_mode(0o755)).expect("mode");
    assert_eq!(
        read(&folder, "2026-09").expect_err("folder").sentence(None),
        "cannot read the usage totals: the usage folder that thinkthen status names has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it aside."
    );
}
