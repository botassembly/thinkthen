//! The Polars door's tests, as one test binary (ticket 0338). They run only
//! with the feature, in the Rust Polars lane (`libraries/polars/check.sh`).
//! The deadline and throttle tests each rerun alone in a fresh process,
//! because the throttle is process-wide (ticket 0077).

#[path = "../../src/test_deadline/child.rs"]
mod child;
#[allow(clippy::expect_used, reason = "a failed fixture stops the proof")]
mod common;
use child::wait;

mod cases;
mod collections;
mod deadline;
mod inputs;
mod prices;
// A path keeps `door`'s own modules beside it rather than under `door/`.
#[path = "door.rs"]
mod door;
mod lazy;
mod throttle_equality;
mod typed;
mod typed_aggregate;
mod typed_rank;
mod typed_rows;
