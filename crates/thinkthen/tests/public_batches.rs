//! Bulk calls through the public API: shared engines, input order, bounded
//! pull-ahead, flat memory, and engine churn, on loopback listeners.
//!
//! Every engine here uses throttle 2, the one explicit throttle this process
//! registers, so a peak above 2 anywhere is a second gate. Rows hold one lock
//! so thread and memory counts see no other row.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

#[path = "public_batches/contracts.rs"]
mod contracts;

#[path = "public_batches/splits.rs"]
mod splits;

#[path = "public_batches/native.rs"]
mod native;

#[path = "public_batches/recognition.rs"]
mod recognition;

#[path = "public_batches/identity.rs"]
mod identity;

#[path = "public_batches/attempts.rs"]
mod attempts;

#[path = "public_batches/portable.rs"]
mod portable;

#[path = "public_batches/interactive.rs"]
mod interactive;

#[path = "public_batches/details.rs"]
mod details;

#[path = "public_batches/tiers.rs"]
mod tiers;

#[path = "../src/test_deadline/wait.rs"]
#[allow(dead_code, reason = "only the child deadline bounds the churn here")]
mod wait;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use conformance_backend::{Backend, Canned, Listener};
use thinkthen::{
    AnnotatedRecord, Answer, BatchSetting, CallOptions, CancelToken, Counters, Description, Engine,
    EngineBuilder, Error, ErrorKind, Found, Judgment, ObservedRow, Question, QuestionSet,
    QuestionSetBuilder, Ranked, RecordObservation, Row,
};

const DECIDED: &str = r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":3,"output_tokens":1}}"#;
const THROTTLE: u8 = 2;

thinkthen::choices! { enum BulkLabel { First => "first", Second => "second" } }

static SERIAL: Mutex<()> = Mutex::new(());

fn serial() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(PoisonError::into_inner)
}

fn engine(base: &str) -> Engine {
    Engine::builder()
        .base_url(base)
        .and_then(|builder| builder.api_key("sk-public-batches"))
        .and_then(|builder| builder.throttle(THROTTLE))
        .map(EngineBuilder::no_cache)
        .and_then(EngineBuilder::build)
        .expect("engine")
}

fn question() -> Question {
    Question::decide("Does this ask for a refund?")
        .expect("question")
        .cut()
}

/// Every handle a host shares across threads.
const fn shared<T: Send + Sync>() {}
const _: () = {
    shared::<Engine>();
    shared::<CancelToken>();
    shared::<CallOptions<'static>>();
    shared::<BatchSetting>();
    shared::<Question>();
    shared::<thinkthen::BandedQuestion>();
    shared::<QuestionSet>();
    shared::<Description>();
    shared::<Counters>();
    shared::<Error>();
    shared::<thinkthen::Details>();
    shared::<Row<String, Answer>>();
    shared::<Ranked<String>>();
    shared::<Found<String>>();
    shared::<AnnotatedRecord<String>>();
    shared::<thinkthen::Recognize>();
    shared::<thinkthen::Relate>();
};

/// The threads of this process. Only Linux lists them, so the worker check runs there.
#[cfg(target_os = "linux")]
fn threads() -> usize {
    let count = std::fs::read_dir("/proc/self/task")
        .expect("/proc lists this process's threads")
        .count();
    assert!(count > 0, "/proc listed no thread");
    count
}

#[test]
fn explicit_batch_one_and_a_question_file_tier_keep_one_record_requests() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED)).expect("listener");
    let engine = engine(listener.base());
    let options = CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN));
    let asked = question();
    let mut batch = engine.decide_many_with(&asked, ["alpha", "beta"], options);
    assert!(
        batch.facts().is_none(),
        "facts wait for the final ordered row"
    );
    let decided = batch
        .by_ref()
        .collect::<Result<Vec<_>, _>>()
        .expect("explicit batch one");
    assert_eq!(decided.len(), 2);
    let facts = batch.facts().expect("the completed batch has final facts");
    assert_eq!((facts.records(), facts.requests_sent()), (2, 2));
    assert_eq!(listener.count(), 2);

    let thinkthen::LoadedQuestion::Question(saved) =
        Question::from_json(r#"{"decide":"Refund?","batch":1}"#).expect("question file")
    else {
        panic!("a decide question with no band");
    };
    let decided = engine
        .decide_many(&saved, ["gamma", "delta"])
        .collect::<Result<Vec<_>, _>>()
        .expect("question-file batch one");
    assert_eq!(decided.len(), 2);
    assert_eq!(listener.count(), 4);

    let engine_default = Engine::builder()
        .base_url(listener.base())
        .and_then(|builder| builder.api_key("sk-public-batches"))
        .map(EngineBuilder::no_cache)
        .map(|builder| builder.batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)))
        .and_then(EngineBuilder::build)
        .expect("engine with a batch default");
    let thinkthen::LoadedQuestion::Question(file_max) =
        Question::from_json(r#"{"decide":"Refund?","batch":"max"}"#).expect("question file")
    else {
        panic!("a decide question with no band");
    };
    let decided = engine_default
        .decide_many(&file_max, ["one", "two"])
        .collect::<Result<Vec<_>, _>>()
        .expect("engine batch one outranks file max");
    assert_eq!(decided.len(), 2);
    assert_eq!(listener.count(), 6);

    let thinkthen::LoadedQuestion::Question(invalid_file_tier) =
        Question::from_json(r#"{"decide":"Refund?","batch":"bad"}"#)
            .expect("a lower-priority file tier")
    else {
        panic!("a decide question with no band");
    };
    let failed = engine.decide_many(&invalid_file_tier, ["epsilon"]).next();
    assert!(failed.is_some_and(|row| row.is_err_and(|error| error.kind() == ErrorKind::Usage)));
    assert_eq!(
        listener.count(),
        6,
        "selected invalid file tier sends nothing"
    );
    let decided = engine
        .decide_many_with(&invalid_file_tier, ["epsilon"], options)
        .collect::<Result<Vec<_>, _>>()
        .expect("typed batch wins over the unused file tier");
    assert_eq!(decided.len(), 1);
    assert_eq!(listener.count(), 7);

    let blank = CallOptions::new()
        .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
        .context(" ");
    let failed = engine
        .decide_many_with(&question(), ["epsilon"], blank)
        .next();
    assert!(failed.is_some_and(|row| row.is_err_and(|error| error.kind() == ErrorKind::Usage)));
    assert_eq!(listener.count(), 7, "invalid context sends nothing");
}

#[test]
fn one_engine_serves_two_threads_under_one_throttle_and_leaves_no_worker() {
    let _serial = serial();
    let listener = Listener::answering(|_| Canned::ok(DECIDED).after(40)).expect("listener");
    #[cfg(target_os = "linux")]
    let before = threads();
    let engine = engine(listener.base());
    let asked = question();
    let texts: Vec<String> = (0..8).map(|at| format!("record {at}")).collect();
    let rows: Vec<Vec<Row<&str, Answer>>> = thread::scope(|scope| {
        let callers: Vec<_> = (0..2)
            .map(|_| {
                let records = texts.iter().map(String::as_str);
                let (engine, asked) = (&engine, &asked);
                scope.spawn(move || {
                    let rows: Result<Vec<Row<&str, Answer>>, Error> = engine
                        .decide_many_with(
                            asked,
                            records,
                            CallOptions::new()
                                .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
                        )
                        .collect();
                    rows
                })
            })
            .collect();
        callers
            .into_iter()
            .map(|caller| caller.join().expect("caller").expect("rows"))
            .collect()
    });
    for rows in rows {
        let inputs: Vec<&str> = rows.iter().map(|row| *row.input()).collect();
        assert_eq!(
            inputs,
            texts.iter().map(String::as_str).collect::<Vec<_>>(),
            "input order"
        );
        assert!(rows.iter().all(|row| *row.value() == Answer::Yes));
    }
    assert_eq!(listener.count(), 16);
    assert!(
        listener.peak() <= usize::from(THROTTLE),
        "peak {}",
        listener.peak()
    );
    assert_eq!(engine.usage().requests_sent(), 16);
    // Each kept connection holds one listener thread; no engine thread stays.
    #[cfg(target_os = "linux")]
    {
        let settled = Instant::now();
        while threads() > before + listener.connections()
            && settled.elapsed() < Duration::from_secs(10)
        {
            thread::sleep(Duration::from_millis(20));
        }
        assert!(
            threads() <= before + listener.connections(),
            "an engine worker stayed"
        );
    }
}

/// Owned texts behind a plain iterator, as a host's cursor would give them.
fn cursor<'a>(texts: &'a [&str]) -> impl Iterator<Item = String> + 'a {
    texts.iter().map(|text| (*text).to_owned())
}

/// Each row as its input text and value, or the failure's kind.
fn keyed<T, V>(
    rows: impl Iterator<Item = Result<T, Error>>,
    key: impl Fn(T) -> (String, V),
) -> Vec<Result<(String, V), ErrorKind>> {
    rows.map(|row| row.map(&key).map_err(|error| error.kind()))
        .collect()
}

#[test]
fn a_slice_and_an_iterator_give_equal_ordered_results_and_partial_rows() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let engine = engine(&format!("{}/generic/v1", backend.origin()));
    let asked = question();
    let texts = ["alpha", "beta", "gamma", "delta", "epsilon"];
    let set = QuestionSet::builder()
        .question("refund", asked.clone())
        .and_then(QuestionSetBuilder::build)
        .expect("set");

    let by_slice = keyed(engine.filter(&asked, texts), |t| (t.to_owned(), ()));
    let by_cursor = keyed(engine.filter(&asked, cursor(&texts)), |t| (t, ()));
    assert_eq!((by_slice.len(), &by_slice), (texts.len(), &by_cursor));
    let by_slice = keyed(engine.decide_many(&asked, texts), |row| {
        let (text, answer) = row.into_parts();
        (text.to_owned(), answer)
    });
    let by_cursor = keyed(engine.decide_many(&asked, cursor(&texts)), Row::into_parts);
    assert_eq!(by_slice, by_cursor);
    let by_slice = keyed(engine.annotate(&set, texts), |row| {
        (row.input().to_string(), row.values().to_vec())
    });
    let by_cursor = keyed(engine.annotate(&set, cursor(&texts)), |row| {
        (row.input().clone(), row.values().to_vec())
    });
    assert_eq!(by_slice, by_cursor);

    // Equal probabilities keep input order in a ranking, on both paths.
    let ranked = Question::rank("Which asks for a refund?").expect("rank");
    let by_slice: Vec<String> = engine
        .rank(&ranked, texts)
        .expect("rank")
        .into_value()
        .into_iter()
        .map(|row| row.into_input().to_owned())
        .collect();
    assert_eq!(by_slice, texts.map(str::to_owned));
    let by_cursor = engine.rank(&ranked, cursor(&texts)).expect("rank");
    assert_eq!(
        by_slice,
        by_cursor
            .into_value()
            .into_iter()
            .map(Ranked::into_input)
            .collect::<Vec<_>>()
    );
    let found = Question::find("Which asks for a refund?").expect("find");
    let by_slice = engine
        .find(&found, texts)
        .expect("find")
        .into_value()
        .into_selected();
    let by_cursor = engine
        .find(&found, cursor(&texts))
        .expect("find")
        .into_value()
        .into_selected();
    assert_eq!(by_slice.map(str::to_owned), by_cursor);

    // A failed record ends a batch: earlier rows in order, the error, then nothing.
    let broken = ["alpha", "beta", "   ", "delta"];
    let expected = vec![
        Ok(("alpha".to_owned(), ())),
        Ok(("beta".to_owned(), ())),
        Err(ErrorKind::Usage),
    ];
    let by_slice = keyed(engine.filter(&asked, broken), |t| (t.to_owned(), ()));
    assert_eq!(by_slice, expected);
    assert_eq!(
        keyed(engine.filter(&asked, cursor(&broken)), |t| (t, ())),
        expected
    );
}

#[test]
fn a_batch_reads_its_input_at_most_one_throttle_ahead_of_its_rows() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let engine = engine(&format!("{}/arm/held/v1", backend.origin()));
    let asked = question();
    let pulled = AtomicUsize::new(0);
    let records = (0..1_000).map(|at| {
        pulled.fetch_add(1, Ordering::SeqCst);
        format!("record {at}")
    });
    let bound = usize::from(THROTTLE) + 1;
    let (held_pull, rows) = thread::scope(|scope| {
        let watcher = scope.spawn(|| {
            backend.wait(usize::from(THROTTLE));
            thread::sleep(Duration::from_millis(300));
            let held_pull = pulled.load(Ordering::SeqCst);
            backend.release();
            held_pull
        });
        let mut batch = engine.filter_with(
            &asked,
            records,
            CallOptions::new().batch(BatchSetting::Records(std::num::NonZeroUsize::MIN)),
        );
        let mut rows = 0;
        while rows < 50 {
            assert!(batch.next().is_some_and(|row| row.is_ok()));
            rows += 1;
            let ahead = pulled.load(Ordering::SeqCst) - rows;
            assert!(ahead <= bound, "{ahead} records read ahead of {rows} rows");
        }
        drop(batch);
        (watcher.join().expect("watcher"), rows)
    });
    assert!(
        held_pull <= bound,
        "{held_pull} records read while every send was held"
    );
    let stopped = pulled.load(Ordering::SeqCst);
    assert!(stopped <= rows + bound, "a dropped batch kept reading");
    assert!(backend.count() <= stopped, "a send without its record");
}

/// The resident set of this process in kilobytes. Only Linux reports it.
#[cfg(target_os = "linux")]
fn resident() -> usize {
    let status = std::fs::read_to_string("/proc/self/status").expect("/proc reports this process");
    let kilobytes = status
        .lines()
        .find_map(|line| line.strip_prefix("VmRSS:"))
        .and_then(|value| value.trim().trim_end_matches(" kB").parse().ok())
        .expect("a VmRSS line");
    assert!(kilobytes > 0, "/proc reported no resident set");
    kilobytes
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "20,000-record memory campaign; run sdlc/scripts/test-stress --run"]
fn a_long_batch_keeps_memory_flat_as_its_input_grows() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let folder = std::env::temp_dir().join(format!("thinkthen-flat-{}", std::process::id()));
    let _gone = std::fs::remove_dir_all(&folder);
    // One text, so the cache answers every record after the first without a
    // send. Each record is a fresh 16 KB string: keeping the 15,000 after the
    // early sample would add 240 MB.
    let engine = Engine::builder()
        .base_url(&format!("{}/generic/v1", backend.origin()))
        .and_then(|builder| builder.api_key("sk-public-batches"))
        .and_then(|builder| builder.throttle(THROTTLE))
        .and_then(|builder| builder.cache_at(&folder))
        .and_then(EngineBuilder::build)
        .expect("engine");
    let records = (0..20_000).map(|_| "x".repeat(16_384));
    let (mut early, mut rows) = (0, 0);
    for row in engine.filter(&question(), records) {
        assert!(row.is_ok(), "{row:?}");
        rows += 1;
        if rows == 5_000 {
            early = resident();
        }
    }
    let late = resident();
    let _gone = std::fs::remove_dir_all(&folder);
    assert_eq!((rows, backend.count()), (20_000, 1));
    assert!(
        late < early + 32 * 1_024,
        "resident set grew from {early} kB to {late} kB"
    );
}

#[test]
#[ignore = "repeated engine churn; run sdlc/scripts/test-stress --run"]
fn engines_built_and_dropped_across_threads_fail_fast_on_a_refused_port() {
    let _serial = serial();
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .and_then(|listener| listener.local_addr())
        .expect("a free port")
        .port();
    let base = format!("http://127.0.0.1:{port}/v1");
    let asked = question();
    let started = Instant::now();
    thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| churn(&base, &asked));
        }
    });
    assert!(
        started.elapsed() < wait::CHILD_DEADLINE,
        "{:?}",
        started.elapsed()
    );
}

/// Build, call, and drop twenty engines on one thread; each call fails fast.
fn churn(base: &str, asked: &Question) {
    for _ in 0..20 {
        let engine = engine(base);
        let options = CallOptions::new().deadline_after(Duration::from_secs(2));
        let result = engine.decide_with(asked, "Refund me.", options.expect("options"));
        assert_eq!(
            result.err().map(|error| error.kind()),
            Some(ErrorKind::Backend)
        );
    }
}
