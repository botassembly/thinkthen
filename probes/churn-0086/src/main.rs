//! Engine churn (G3, R7-1): 32 threads share 70 engines, each with its own
//! timeout, and call a refused port 20,000 times each. The C door's version
//! of this load crashed in a new thread's start, 1 run in 83. The same
//! source drives the stand-in API at the tag and the public API on main.

use std::net::TcpListener;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

const THREADS: usize = 32;
const ENGINES: usize = 70;
const QUESTION: &str = r#"{"decide":"Does the writer ask for a refund?","threshold":0.5}"#;

fn main() {
    let iterations: u64 = std::env::var("ITERS").ok().and_then(|v| v.parse().ok()).unwrap_or(20_000);
    let refused = {
        let listener = TcpListener::bind("127.0.0.1:0").expect("a free port");
        listener.local_addr().expect("its address").port()
    };
    let base = format!("http://127.0.0.1:{refused}/v1");
    let engines: Vec<Engine> = (0..ENGINES).map(|at| Engine::new(&base, timeout(at))).collect();
    let failed = AtomicU64::new(0);
    std::thread::scope(|scope| {
        for worker in 0..THREADS {
            let (engines, failed) = (&engines, &failed);
            scope.spawn(move || {
                for round in 0..iterations {
                    let at = (worker * 7 + round as usize) % ENGINES;
                    if engines[at].call(&format!("refund {worker} {round}")) {
                        failed.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }
    });
    let calls = THREADS as u64 * iterations;
    let failed = failed.load(Ordering::Relaxed);
    assert_eq!(failed, calls, "every call to a refused port fails");
    println!("churn: {calls} calls over {ENGINES} engines on {THREADS} threads, all failed");
    println!("churn: one failure reads: {}", engines[0].failure("refund sample"));
}

/// Each engine's own timeout: seventy distinct values.
fn timeout(at: usize) -> Duration {
    Duration::from_millis(1_000 + (at as u64 * 7_919) % 977)
}

#[cfg(feature = "standin")]
struct Engine(thinkthen::Engine, thinkthen::Question);

#[cfg(feature = "standin")]
impl Engine {
    fn new(base: &str, timeout: Duration) -> Self {
        let settings = thinkthen::Settings {
            address: Some(base.to_owned()),
            timeout: Some(timeout),
            max_retries: Some(0),
            ..thinkthen::Settings::default()
        };
        let question = thinkthen::Question::from_json(QUESTION).expect("the question");
        Self(thinkthen::Engine::from_settings(settings), question)
    }

    /// Ask once; true when the call failed.
    fn call(&self, evidence: &str) -> bool {
        self.0.decide(&self.1, evidence).is_err()
    }

    fn failure(&self, evidence: &str) -> String {
        self.0.decide(&self.1, evidence).err().map(|e| e.to_string()).unwrap_or_default()
    }
}

/// The public API has no transport timeout, so each engine carries its own
/// call deadline instead.
#[cfg(not(feature = "standin"))]
struct Engine(thinkthen::Engine, thinkthen::Question, Duration);

#[cfg(not(feature = "standin"))]
impl Engine {
    fn new(base: &str, timeout: Duration) -> Self {
        let engine = thinkthen::Engine::builder()
            .base_url(base)
            .and_then(|b| b.api_key("sk-churn-loopback"))
            .map(thinkthen::EngineBuilder::no_cache)
            .and_then(thinkthen::EngineBuilder::build)
            .expect("the engine");
        let question = match thinkthen::Question::from_json(QUESTION).expect("the question") {
            thinkthen::LoadedQuestion::Question(question) => question,
            thinkthen::LoadedQuestion::Banded(_) => unreachable!("no band"),
        };
        Self(engine, question, timeout)
    }

    /// Ask once under this engine's deadline; true when the call failed.
    fn call(&self, evidence: &str) -> bool {
        let options = thinkthen::CallOptions::new().deadline_after(self.2).expect("a deadline");
        self.0.decide_with(&self.1, evidence, options).is_err()
    }

    fn failure(&self, evidence: &str) -> String {
        let options = thinkthen::CallOptions::new().deadline_after(self.2).expect("a deadline");
        let failed = self.0.decide_with(&self.1, evidence, options).err();
        failed.map(|e| format!("{:?}: {e}", e.kind())).unwrap_or_default()
    }
}
