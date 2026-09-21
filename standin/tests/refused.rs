//! The counter counts what left the machine: a refused connection counts
//! nothing (ruling 5 of the adversarial review, 2026-09-21).
//!
//! One test per file, because the engine reads the environment once per
//! process; this binary owns its process.

use thinkthen_contract::{Engine, ErrorKind, Question};
use thinkthen_standin::BlockingEngine;

#[test]
fn a_refused_send_counts_nothing() {
    // Sound in this binary: the test owns the process, and no engine call
    // runs before this set.
    unsafe { std::env::set_var("THINKTHEN_BASE_URL", "http://127.0.0.1:1/v1") };
    let tt = BlockingEngine::from_env();
    let before = tt.usage().requests;
    let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#).expect("parses");
    let failed = tt.decide(&question, "i want a refund").expect_err("port 1 refuses");
    assert_eq!(failed.kind, ErrorKind::Backend, "{failed}");
    assert!(failed.message.contains("refused"), "{failed}");
    let after = tt.usage().requests;
    assert_eq!(
        after, before,
        "a refused connection left nothing, so nothing counts (was {before}, now {after})"
    );
}
