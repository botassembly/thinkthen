//! R5-3: a forked child's first call rebuilds before it touches inherited state.
//!
//! A forked child is its parent's memory under a new process ID. So each
//! proof builds an engine as a parent process that is not this one, lets a
//! parent thread use that parent's state, and then calls the engine as this
//! process, which is the child. The crate forbids `unsafe`, so the real
//! `fork()` proofs belong to ticket 0086's consumer crate.
//!
//! The width state belongs to the process, so each proof runs alone in a
//! child copy of the test binary with a 60 s bound. Every held call waits on
//! `recv_timeout`, so a child that waits on an inherited lock fails the case.

use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::{env, fs};

use conformance_backend::Backend as Loopback;

use super::{Engine, Key, Settings, Storage};
use crate::cli::schedule::width_tests::{child, in_child_at};
use crate::core::adapters::built_in;
use crate::core::{Backend, Evidence, Plan, Question, QuestionText};
use crate::engine::error::Error;
use crate::engine::http::Exchange;
use crate::engine::usage::{self, Counters};
use crate::engine::{Cancel, Width, limits};

/// A process ID this test process does not have.
fn parent_pid() -> u32 {
    std::process::id() + 1
}

const BOUND: Duration = Duration::from_secs(5);

fn settings(base: &str, width: Option<u64>, usage: Arc<Counters>) -> Settings {
    Settings {
        backend: Backend::resolve(Some(base), None, "local-1").expect("backend"),
        profile: None,
        timeout: Duration::from_secs(5),
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
        width: width.and_then(|width| Width::new(width).ok()),
        per_minute: None,
        storage: Storage::default(),
        key: Arc::new(|| Ok(Key::of("sk-test-value"))),
        usage,
    }
}

fn decide() -> Question {
    Question::Decide {
        text: QuestionText::new("Is it late?").expect("question"),
        yes: None,
        no: None,
    }
}

/// The state the parent process built, which this test uses as a parent thread.
fn parent_state(engine: &Engine) -> Arc<super::State> {
    engine
        .state
        .current(
            parent_pid(),
            || Err(Error::Cancelled),
            || Err(Error::Cancelled),
        )
        .expect("the parent's own state")
}

/// Send one request through the parent's state, as a parent thread would.
fn ask_as_parent(engine: &Engine, parent: &super::State) -> Result<(), Error> {
    let evidence = Evidence::new("parent").expect("evidence");
    let plan =
        Plan::authored(evidence, engine.backend.model().clone(), vec![decide()]).expect("plan");
    let body = built_in::encode(&plan).map_err(|_| Error::Defect("unwritable plan"))?;
    let key = (engine.key)()?;
    let transport = engine.transport(parent);
    let exchange = Exchange {
        url: engine.backend.url().as_str(),
        body: &body,
        key: &key,
        max_retries: transport.max_retries,
        retry_wait: transport.retry_wait,
    };
    transport
        .client
        .post_marked_with_retry(
            &exchange,
            &Cancel::default(),
            transport.usage,
            |_| (),
            || (),
        )
        .map(|_| ())
}

#[test]
fn a_child_beside_a_busy_parent_uses_fresh_state() {
    in_child_at("engine::facade::fork_tests::busy_parent_child");
}

#[test]
#[ignore = "runs alone in a child process"]
fn busy_parent_child() {
    if !child() {
        return;
    }
    let home = env::temp_dir().join(format!("thinkthen-fork-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&home);
    let usage_path = home.join("usage");
    let loopback = Loopback::start().expect("loopback");
    let base = format!("{}/arm/held/v1", loopback.origin());
    let counters = Arc::new(Counters::new(Some(usage_path.clone())));
    let engine = Engine::built_by(
        settings(&base, Some(1), Arc::clone(&counters)),
        parent_pid(),
    )
    .expect("the parent's engine");
    let parent = parent_state(&engine);
    counters.request_sent();

    // A parent thread holds the one permit of width 1 in a held send when
    // the child starts.
    let ((parent_done, parent_answer), (child_done, child_answer)) = (channel(), channel());
    thread::scope(|scope| {
        scope.spawn(|| parent_done.send(ask_as_parent(&engine, &parent)));
        assert_eq!(loopback.wait(1), 1, "the parent's send is held");
        scope.spawn(|| {
            let child = Evidence::new("child").expect("evidence");
            let judged = engine.judge(&decide(), None, child, &Cancel::default());
            child_done.send(judged.map(|judged| judged.answer.yes()))
        });
        assert_eq!(
            loopback.wait(2),
            2,
            "the child sent past the parent's full gate"
        );
        loopback.release();
        let answer = child_answer
            .recv_timeout(BOUND)
            .expect("the child answered");
        assert_eq!(answer.expect("the child's call"), Some(0.9));
        let parent = parent_answer
            .recv_timeout(BOUND)
            .expect("the parent answered");
        parent.expect("the parent's call");
    });

    let child_counts = engine.usage().expect("the child's counters");
    assert_eq!(
        (
            counters.snapshot().requests_sent,
            child_counts.requests_sent
        ),
        (2, 1),
        "the child counts only its own request"
    );
    // Dropping the engine finishes the child's writer.
    drop((parent, engine));
    counters.finish();
    let durable = usage::read(&usage_path, &usage::month_now()).expect("usage totals");
    assert_eq!(
        durable.total.requests_sent, 3,
        "each request is counted once"
    );
    let _removed = fs::remove_dir_all(home);
}

#[test]
fn a_child_selects_its_width_as_a_fresh_process() {
    in_child_at("engine::facade::fork_tests::child_width_child");
}

#[test]
#[ignore = "runs alone in a child process"]
fn child_width_child() {
    if !child() {
        return;
    }
    let base = "http://127.0.0.1:1/v1";
    let build = |width| Engine::built_by(settings(base, width, Arc::default()), parent_pid());
    let explicit = build(Some(2)).expect("the parent's explicit width");
    let implicit = build(None).expect("the parent's implicit engine");
    assert_eq!(
        parent_state(&implicit).width,
        2,
        "the parent follows its width"
    );

    let width = |engine: &Engine| engine.state(&Cancel::default()).map(|state| state.width);
    // (row, engine, width its first child call follows, the child's selection)
    let rows = [
        (
            "an implicit engine follows the fallback",
            &implicit,
            4,
            None,
        ),
        (
            "an explicit engine selects its width",
            &explicit,
            2,
            Some(2),
        ),
    ];
    for (row, engine, follows, selected) in rows {
        assert_eq!(width(engine).expect("the child's state"), follows, "{row}");
        assert_eq!(
            limits::process().widths.selected(),
            selected.and_then(|width| Width::new(width).ok()),
            "{row}"
        );
    }
    let same = Engine::new(settings(base, Some(2), Arc::default()));
    let other = Engine::new(settings(base, Some(3), Arc::default()));
    assert!(same.is_ok(), "the same explicit width is accepted");
    assert!(
        matches!(other, Err(Error::WidthActive(_))),
        "a conflicting width is refused"
    );
}

#[test]
fn a_child_finishing_usage_leaves_its_parents_counters_alone() {
    in_child_at("engine::facade::fork_tests::finish_usage_child");
}

#[test]
#[ignore = "runs alone in a child process"]
fn finish_usage_child() {
    if !child() {
        return;
    }
    let home = env::temp_dir().join(format!("thinkthen-fork-finish-{}", std::process::id()));
    let _absent = fs::remove_dir_all(&home);
    let usage_path = home.join("usage");
    let counters = Arc::new(Counters::new(Some(usage_path.clone())));
    let settings = || settings("http://127.0.0.1:1/v1", None, Arc::clone(&counters));
    let engine = Engine::built_by(settings(), parent_pid()).expect("the parent's engine");
    counters.request_sent();
    assert!(!counters.finish(), "the parent writes its request");

    // A parent thread holds the queue lock when the child's exit hook runs.
    let held = counters.hold_queue();
    let (finished, returned) = channel();
    thread::spawn(move || {
        engine.finish_usage();
        finished.send(())
    });
    assert_eq!(
        returned.recv_timeout(BOUND),
        Ok(()),
        "the child returns without waiting on its parent's lock"
    );
    drop(held);
    let durable = || usage::read(&usage_path, &usage::month_now()).expect("usage totals");
    assert_eq!(durable().total.requests_sent, 1, "the child wrote nothing");

    // The process that built its own state flushes its own pending request.
    let own = Engine::built_by(settings(), std::process::id()).expect("this process's engine");
    counters.request_sent();
    own.finish_usage();
    assert_eq!(
        durable().total.requests_sent,
        2,
        "each request is counted once"
    );
    let _removed = fs::remove_dir_all(home);
}

#[test]
fn a_child_dropping_an_inherited_engine_leaves_its_state_alone() {
    in_child_at("engine::facade::fork_tests::drop_inherited_child");
}

#[test]
#[ignore = "runs alone in a child process"]
fn drop_inherited_child() {
    if !child() {
        return;
    }
    let home = env::temp_dir().join(format!("thinkthen-fork-drop-{}", std::process::id()));
    let usage_path = home.join("usage");
    let counters = Arc::new(Counters::new(Some(usage_path)));
    let parents = Arc::downgrade(&counters);
    let engine = Engine::built_by(
        settings("http://127.0.0.1:1/v1", None, counters),
        parent_pid(),
    )
    .expect("the parent's engine");
    drop(engine);
    assert!(
        parents.upgrade().is_some(),
        "the child leaks its parent's counters and never joins their writer"
    );
    let own = Arc::new(Counters::new(None));
    let mine = Arc::downgrade(&own);
    drop(Engine::built_by(
        settings("http://127.0.0.1:1/v1", None, own),
        std::process::id(),
    ));
    assert!(
        mine.upgrade().is_none(),
        "a process drops the state it built"
    );
}
