//! The Polars door's tests, as one test binary (ticket 0338). They run only
//! with the feature, in the Rust Polars lane (`libraries/polars/check.sh`).
//! The deadline and throttle tests each rerun alone in a fresh process,
//! because the throttle is process-wide (ticket 0077).

#[allow(clippy::expect_used, reason = "a failed fixture stops the proof")]
mod common;
#[path = "../../src/test_deadline/wait.rs"]
mod wait;

mod cases;
mod deadline;
// A path keeps `door`'s own modules beside it rather than under `door/`.
#[path = "door.rs"]
mod door;
mod lazy;
mod throttle_equality;
