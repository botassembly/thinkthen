//! Real-fork proofs of ticket 0096 through the public API.
//!
//! Each proof forks this test process with [`fork_probe::in_child`]. The
//! conformance backend runs on the parent's threads, so a child's request
//! reaches it through the inherited address. One lock keeps the proofs from
//! sharing the process's permits at once.

use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use conformance_backend::Backend;
use fork_probe::in_child;
use thinkthen::{Answer, CallOptions, Engine, ErrorKind, Question};

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
const KEY: &str = "sk-fork-loopback";
const CHILD: &str = "FORK_PROBE_DEFAULT_BASE";

fn engine(base: &str, cache: Option<&PathBuf>) -> Result<Engine, thinkthen::Error> {
    let builder = Engine::builder().base_url(base)?.api_key(KEY)?;
    let builder = match cache {
        Some(folder) => builder.cache_at(folder)?,
        None => builder.no_cache(),
    };
    builder.build()
}

fn folder(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("fork-probe-{name}-{}", std::process::id()));
    let _absent = std::fs::remove_dir_all(&folder);
    folder
}

fn decide() -> Question {
    Question::decide("Is this urgent?").map_or_else(
        |_| unreachable!("the text is not blank"),
        thinkthen::DecideBuilder::cut,
    )
}

#[test]
fn a_warm_parent_engine_and_its_clone_answer_in_the_child() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let engine =
        engine(&format!("{}/generic/v1", backend.origin()), None).expect("a loopback engine");
    assert_eq!(engine.decide(&decide(), "warm").ok(), Some(Answer::Yes));
    let clone = engine.clone();
    in_child(|| {
        let before = engine.usage().requests_sent();
        let answered = clone.decide(&decide(), "in the child").ok() == Some(Answer::Yes);
        answered && engine.usage().requests_sent() == before + 1
    })
    .expect("the child answered, and the clone counted on its source");
    assert_eq!(backend.count(), 2, "one send from each process");
    assert_eq!(
        engine.usage().requests_sent(),
        1,
        "the child's send stays the child's"
    );
}

#[test]
fn a_child_of_a_busy_parent_gets_its_own_permits() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let cache = folder("busy");
    let held = engine(&format!("{}/arm/held/v1", backend.origin()), Some(&cache))
        .expect("a loopback engine");
    // A recording folder belongs to one backend address, so the child's has its own.
    let open = engine(
        &format!("{}/generic/v1", backend.origin()),
        Some(&folder("busy-child")),
    )
    .expect("a loopback engine");
    let held = &held;
    thread::scope(|scope| {
        // Four calls hold the process's four permits, each with its reply held.
        let calls: Vec<_> = (0..4)
            .map(|at| scope.spawn(move || held.decide(&decide(), &format!("held {at}")).ok()))
            .collect();
        assert_eq!(backend.wait(4), 4, "every permit is in flight");
        let child = in_child(|| open.decide(&decide(), "the child").ok() == Some(Answer::Yes));
        backend.release();
        child.expect("the child sent past the parent's full gate");
        for call in calls {
            assert_eq!(call.join().expect("a parent call"), Some(Answer::Yes));
        }
    });
    assert_eq!(backend.count(), 5);
}

#[test]
fn a_child_replays_the_parents_warm_cache_without_sending() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let cache = folder("warm-cache");
    let engine = engine(&format!("{}/generic/v1", backend.origin()), Some(&cache))
        .expect("a loopback engine");
    assert!(
        !engine
            .details(&decide(), "cached note")
            .expect("a first answer")
            .cached()
    );
    in_child(|| {
        engine
            .details(&decide(), "cached note")
            .is_ok_and(|details| details.cached() && details.requests_sent() == 0)
    })
    .expect("the child read the parent's recording");
    assert_eq!(backend.count(), 1, "only the parent sent");
}

/// Ticket 0096 F6: a digest lock held when the child forked is released when
/// the parent's request ends, not when the child exits.
#[test]
fn a_parents_released_digest_lock_frees_its_waiter_while_the_child_lives() {
    let _one = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let backend = Backend::start().expect("backend");
    let cache = folder("digest-lock");
    let held = engine(&format!("{}/arm/held/v1", backend.origin()), Some(&cache))
        .expect("a loopback engine");
    thread::scope(|scope| {
        let owner = scope.spawn(|| held.decide(&decide(), "one note").ok());
        assert_eq!(backend.wait(1), 1, "the owner holds the digest lock");
        let waiter = scope.spawn(|| {
            let answer = held.decide(&decide(), "one note").ok();
            (answer, Instant::now())
        });
        thread::sleep(Duration::from_millis(200));
        let child = scope.spawn(|| {
            in_child(|| {
                thread::sleep(Duration::from_secs(3));
                true
            })
        });
        thread::sleep(Duration::from_millis(200));
        let released = Instant::now();
        backend.release();
        assert_eq!(owner.join().expect("the owner"), Some(Answer::Yes));
        let (answer, done) = waiter.join().expect("the waiter");
        assert_eq!(answer, Some(Answer::Yes));
        assert!(
            done - released < Duration::from_millis(1500),
            "the waiter waited for the child: {:?}",
            done - released
        );
        child
            .join()
            .expect("the child thread")
            .expect("the child slept and left");
    });
    assert_eq!(backend.count(), 1, "the waiter replayed the owner's answer");
}

#[test]
fn a_default_engine_from_the_parent_answers_in_the_child() {
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let output = Command::new(std::env::current_exe().expect("this test binary"))
        .args(["--exact", "default_engine_child"])
        .env_clear()
        .env(CHILD, "1")
        .env("THINKTHEN_BASE_URL", &base)
        .env("THINKTHEN_API_KEY", KEY)
        .env("THINKTHEN_CACHE", folder("default-engine"))
        .output()
        .expect("the process ran");
    let said = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success() && said.contains("1 passed"),
        "{said}"
    );
    assert_eq!(
        backend.count(),
        2,
        "one send from the parent and one from its child"
    );
}

/// The process half of the proof above. It does nothing unless named.
#[test]
fn default_engine_child() {
    if std::env::var_os(CHILD).is_none() {
        return;
    }
    assert_eq!(
        thinkthen::decide(&decide(), "parent").ok(),
        Some(Answer::Yes)
    );
    in_child(|| thinkthen::decide(&decide(), "child").ok() == Some(Answer::Yes))
        .expect("the process engine answered in the child");
    let spent = CallOptions::new()
        .deadline_after(Duration::ZERO)
        .expect("a budget");
    let error = thinkthen::decide_with(&decide(), "late", spent).expect_err("no time left");
    assert_eq!(error.kind(), ErrorKind::Deadline);
}
