//! Private repository fixture adapter over the shared test server.
#[path = "../../thinkthen/test-support/server/mod.rs"]
mod server;
pub use server::*;
use server::{case, lock};
mod stress;
pub use stress::{StressFixture, stress_table};
