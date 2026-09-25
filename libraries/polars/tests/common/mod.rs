//! The one way a test builds an engine.
//!
//! `check.sh` sets a fake key and a closed loopback address. The helper
//! refuses to build an engine under any other key or address, so a real key
//! in the caller's shell never reaches a test. Every engine names its own
//! loopback backend and its own cache folder.

#![allow(dead_code, reason = "each test file uses its own part of the helper")]

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use thinkthen::{Engine, EngineBuilder};
use thinkthen_polars::polars::prelude::{NamedFrom, Series};

/// The fake key `check.sh` sets.
pub(crate) const FAKE_KEY: &str = "sk-polars-loopback";

/// Refuse any environment but the fake key and a loopback address.
fn guarded() {
    let key = std::env::var("THINKTHEN_API_KEY").unwrap_or_default();
    let base = std::env::var("THINKTHEN_BASE_URL").unwrap_or_default();
    assert!(
        key == FAKE_KEY && base.starts_with("http://127.0.0.1:"),
        "run the tests through check.sh, which sets the fake key and a loopback address"
    );
}

/// A fresh cache folder no other engine in this run uses.
fn cache() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "polars-cache-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _absent = std::fs::remove_dir_all(&folder);
    folder
}

/// The environment's builder, sent to this base with its own cache.
pub(crate) fn builder(base: &str) -> EngineBuilder {
    guarded();
    EngineBuilder::from_env()
        .and_then(|builder| builder.base_url(base))
        .and_then(|builder| builder.cache_at(cache()))
        .expect("an engine builder")
}

/// An engine on this base that follows the process throttle.
pub(crate) fn engine(base: &str) -> Engine {
    builder(base).build().expect("an engine")
}

/// A text column named `body`.
pub(crate) fn column(texts: &[&str]) -> Series {
    Series::new("body".into(), texts)
}

/// This many distinct texts.
pub(crate) fn distinct(count: usize) -> Vec<String> {
    (0..count).map(|place| format!("note {place}")).collect()
}
