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
        crate::engine::process_width(),
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
    assert_eq!(fs::read_dir(&folder).expect("folder").count(), 3);
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
fn one_write_migrates_every_contaminated_month_without_losing_totals() {
    let folder = folder("legacy-migration");
    for (month, requests, retries) in [("2026-08", 3, 1), ("2026-09", 5, 2)] {
        update(&folder, month, Counts::default()).expect("private directory and files");
        fs::write(
            folder.join(format!("{month}.json")),
            format!("{{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":{requests},\"retries\":{retries},\"input_tokens\":7,\"output_tokens\":4,\"cache_answers\":0}}\n"),
        )
        .expect("contaminated month");
        fs::remove_file(folder.join(format!("retries-{month}.json"))).expect("old layout");
    }
    // A crash after migration's sidecar sync leaves both copies with one total.
    fs::write(
        folder.join("retries-2026-08.json"),
        b"{\"schema\":\"thinkthen.usage.retries/1\",\"retries\":1}\n",
    )
    .expect("transitional sidecar");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(
            folder.join("retries-2026-08.json"),
            fs::Permissions::from_mode(0o600),
        )
        .expect("private sidecar");
    }
    let before = read(&folder, "2026-09").expect("transitional read");
    assert_eq!((before.total.requests_sent, before.total.retries), (8, 3));
    update(
        &folder,
        "2026-09",
        Counts {
            requests_sent: 1,
            retries: 1,
            ..Counts::default()
        },
    )
    .expect("migrate and count");
    let after = read(&folder, "2026-09").expect("migrated totals");
    assert_eq!((after.total.requests_sent, after.total.retries), (9, 4));
    for month in ["2026-08", "2026-09"] {
        let value: serde_json::Value =
            serde_json::from_slice(&fs::read(folder.join(format!("{month}.json"))).expect("row"))
                .expect("monthly JSON");
        assert!(value.get("retries").is_none(), "{month}");
        assert_eq!(value["schema"], "thinkthen.usage/1");
    }
}

#[test]
fn a_failed_retry_sidecar_keeps_the_durable_base_and_warns_once() {
    let folder = folder("retry-sidecar-failure");
    update(&folder, "2026-09", Counts::default()).expect("baseline");
    let counters = Counters::new(Some(folder.clone()));
    FAILURE.with(|failure| failure.set(Some(Stage::RetryWrite)));
    counters.attempt_sent(true);
    assert!(
        counters.finish(),
        "the sidecar failure reaches the warning path"
    );
    let counted = read(&folder, "2026-09").expect("durable base remains readable");
    assert_eq!((counted.month.requests_sent, counted.month.retries), (1, 0));
    counters.attempt_sent(true);
    assert!(counters.finish(), "writer stays failed");
    let counted = read(&folder, "2026-09").expect("no later persistence");
    assert_eq!((counted.month.requests_sent, counted.month.retries), (1, 0));
}

#[test]
fn an_invalid_older_month_refuses_the_whole_migration_before_any_projection() {
    let folder = folder("invalid-old-month");
    for month in ["2026-07", "2026-08"] {
        update(&folder, month, Counts::default()).expect("baseline");
        fs::remove_file(folder.join(format!("retries-{month}.json"))).expect("old layout");
    }
    let july = folder.join("2026-07.json");
    let august = folder.join("2026-08.json");
    let contaminated = b"{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":2,\"retries\":1,\"input_tokens\":0,\"output_tokens\":0,\"cache_answers\":0}\n";
    fs::write(&july, contaminated).expect("contaminated month");
    fs::write(
        &august,
        b"{\"schema\":\"thinkthen.usage/2\",\"requests_sent\":1,\"input_tokens\":0,\"output_tokens\":0,\"cache_answers\":0}\n",
    )
    .expect("future schema");
    assert!(update(&folder, "2026-09", Counts::default()).is_err());
    assert_eq!(fs::read(july).expect("kept month"), contaminated);
    assert!(!folder.join("retries-2026-07.json").exists());
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
    let sidecar = folder.join("retries-2026-08.json");
    let before_month = fs::read(&august).expect("last good bytes");
    let before_sidecar = fs::read(&sidecar).expect("last good retry bytes");
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
    assert_eq!(
        fs::read(sidecar).expect("unchanged sidecar"),
        before_sidecar
    );
    assert!(!folder.join("2026-09.json").exists());
    assert!(!folder.join("retries-2026-09.json").exists());
    assert_eq!(
        read(&folder, "2026-08")
            .expect("readable total")
            .total
            .requests_sent,
        u64::MAX
    );
}

#[test]
fn current_month_overflow_refuses_before_migrating_an_older_month() {
    let folder = folder("overflow-before-migration");
    update(&folder, "2026-08", Counts::default()).expect("old baseline");
    update(
        &folder,
        "2026-09",
        Counts {
            requests_sent: u64::MAX,
            ..Counts::default()
        },
    )
    .expect("current maximum");
    let old = folder.join("2026-08.json");
    fs::remove_file(folder.join("retries-2026-08.json")).expect("old layout");
    let contaminated = b"{\"schema\":\"thinkthen.usage/1\",\"requests_sent\":0,\"retries\":1,\"input_tokens\":0,\"output_tokens\":0,\"cache_answers\":0}\n";
    fs::write(&old, contaminated).expect("retry-extended older month");
    let current = folder.join("2026-09.json");
    let current_sidecar = folder.join("retries-2026-09.json");
    let current_before = fs::read(&current).expect("current bytes");
    let retry_before = fs::read(&current_sidecar).expect("current retry bytes");

    assert!(
        update(
            &folder,
            "2026-09",
            Counts {
                requests_sent: 1,
                ..Counts::default()
            }
        )
        .is_err()
    );
    assert_eq!(fs::read(old).expect("old bytes"), contaminated);
    assert!(!folder.join("retries-2026-08.json").exists());
    assert_eq!(fs::read(current).expect("current bytes"), current_before);
    assert_eq!(
        fs::read(current_sidecar).expect("current retry bytes"),
        retry_before
    );
}

#[test]
fn every_update_stage_warns_once_and_disables_later_persistence() {
    for stage in [
        Stage::Setup,
        Stage::Lock,
        Stage::Validation,
        Stage::Write,
        Stage::RetryWrite,
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
        assert!(!folder.join("retries-2026-09.json").exists());
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
        "sidecar-symlink",
        "sidecar-mode",
        "sidecar-schema",
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
            "sidecar-symlink" => {
                let sidecar = folder.join("retries-2026-09.json");
                fs::rename(&sidecar, folder.join("retry-target")).expect("target");
                symlink(folder.join("retry-target"), &sidecar).expect("symlink");
            }
            "sidecar-mode" => fs::set_permissions(
                folder.join("retries-2026-09.json"),
                fs::Permissions::from_mode(0o644),
            )
            .expect("mode"),
            "sidecar-schema" => fs::write(
                folder.join("retries-2026-09.json"),
                b"{\"schema\":\"thinkthen.usage.retries/2\",\"retries\":0}\n",
            )
            .expect("future sidecar"),
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
