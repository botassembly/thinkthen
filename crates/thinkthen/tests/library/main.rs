//! The public library through its doors, as one test binary (ticket 0338).
//!
//! These pages share no process state. `public_batches`, `public_controls`,
//! `public_cap` and `public_estimated` stay their own binaries, because they
//! read the process throttle or the process send totals in
//! `engine/limits.rs`. A page that reruns itself as a child names its test by
//! its path here, such as `public_env::child_case`.

#[path = "../../src/test_deadline/child.rs"]
mod child;
#[path = "../../src/test_deadline/run.rs"]
mod run;
#[path = "../../src/test_deadline/wait.rs"]
mod wait;

mod ca_bundle;
mod compile_contract;
mod key_address;
mod public_backoff;
mod public_env;
mod public_hosts;
mod public_members;
mod public_plan;
mod public_size_retry;
mod public_tally;
mod question_file;
