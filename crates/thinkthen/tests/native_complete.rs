//! Outside-in concrete complete results use saved responses and independent expected exchanges.
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use std::sync::atomic::{AtomicUsize, Ordering};
use thinkthen::{
    Answer, CallOptions, CompleteDecision, Engine, ErrorKind, Observation, Origin, Question,
};

#[cfg(test)]
fn engine(listener: &Listener) -> Engine {
    Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("complete-private")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap()
}
#[cfg(test)]
fn folder() -> std::path::PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let folder = std::env::temp_dir().join(format!(
        "thinkthen-complete-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir(&folder).unwrap();
    folder
}
#[cfg(test)]
fn framed(parts: &[&str]) -> String {
    let mut hash = Sha256::new();
    hash.update(b"thinkthen.answer-id/1\0");
    for part in parts {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part.as_bytes());
    }
    hash.finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
#[path = "native_complete/atomic.rs"]
mod atomic;
#[path = "native_complete/records.rs"]
mod records;

#[path = "native_complete/aggregates.rs"]
mod aggregates;

#[path = "native_complete/readings.rs"]
mod readings;

#[path = "native_complete/calls.rs"]
mod calls;
#[path = "native_complete/composition.rs"]
mod composition;
#[path = "native_complete/dynamic_choose.rs"]
mod dynamic_choose;
#[path = "native_complete/find_reading.rs"]
mod find_reading;
#[path = "native_complete/observers.rs"]
mod observers;
#[path = "native_complete/streaming.rs"]
mod streaming;
#[path = "native_complete/whole_sources.rs"]
mod whole_sources;

#[path = "native_complete/recognize_records.rs"]
mod recognize_records;

#[path = "native_complete/relate_records.rs"]
mod relate_records;

#[path = "native_complete/declarations.rs"]
mod declarations;

#[path = "native_complete/consumer_views.rs"]
mod consumer_views;
#[path = "native_complete/context.rs"]
mod context;
#[path = "native_complete/declaration_batches.rs"]
mod declaration_batches;
#[path = "native_complete/tally.rs"]
mod tally;

#[path = "native_complete/set_rank.rs"]
mod set_rank;

#[path = "native_complete/admission_cancel.rs"]
mod admission_cancel;

#[path = "native_complete/schema.rs"]
mod schema;
