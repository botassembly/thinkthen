#![cfg(test)]
//! The release leak check's workload: one client makes `THINKTHEN_LEAK_CALLS` calls over HTTP
//! to the loopback server, then closes. `scripts/leak-check` runs it under valgrind memcheck at
//! two sizes. `THINKTHEN_LEAK_FIXTURE=leak` leaks one small block per call, only to prove that
//! the check fails on a per-call leak. This file holds one test, so the process runs nothing else.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, StressFixture, stress_table};
use thinkthen::{Decide, Record, Settings};

const RECORDS: usize = 4;

#[test]
#[ignore = "run under valgrind by scripts/leak-check"]
fn calls_on_one_client_then_close() {
    let calls: usize = std::env::var("THINKTHEN_LEAK_CALLS")
        .expect("scripts/leak-check sets THINKTHEN_LEAK_CALLS")
        .parse()
        .unwrap();
    let leak = std::env::var("THINKTHEN_LEAK_FIXTURE").is_ok_and(|value| value == "leak");
    let table = stress_table().unwrap();
    let fixture = StressFixture::open("healthy").unwrap();
    let mut settings = table["client"].clone();
    settings["backend"] = "systemone".into();
    let engine = Settings::from_json(&settings.to_string())
        .unwrap()
        .url(fixture.base())
        .api_key(KEY.parse().unwrap())
        .engine(|_| None)
        .unwrap();
    let question = Decide::new("Ready?").unwrap();
    for call in 0..calls {
        let records = (0..RECORDS).map(move |place| Record::text(format!("record {call} {place}")));
        let rows = engine.rows(&question, records).unwrap();
        assert_eq!(rows.filter(Result::is_ok).count(), RECORDS);
        if leak {
            let _ = Box::leak(Box::new([0_u8; 64]));
        }
    }
    let _ = engine.settled_usage();
    drop(engine);
    drop(fixture);
}
