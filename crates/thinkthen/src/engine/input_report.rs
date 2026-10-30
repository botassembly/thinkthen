//! Optional private packing diagnostics; no records or credentials are retained.
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};

static NEXT_CALL: AtomicU64 = AtomicU64::new(0);
static WRITER: Mutex<()> = Mutex::new(());

pub(super) struct InputReport {
    destination: Option<(std::ffi::OsString, String)>,
    call: u64,
    batch: usize,
    records: usize,
    packed_requests: usize,
    idle_flushes: usize,
    idle_unused_slots: usize,
}
impl InputReport {
    pub(super) fn new(batch: usize) -> Self {
        let destination = std::env::var_os("THINKTHEN_INPUT_REPORT")
            .zip(std::env::var("THINKTHEN_INPUT_REPORT_RUN").ok())
            .filter(|(_, run)| run.len() == 32 && run.bytes().all(|byte| byte.is_ascii_hexdigit()));
        Self {
            destination,
            call: NEXT_CALL.fetch_add(1, Ordering::Relaxed),
            batch,
            records: 0,
            packed_requests: 0,
            idle_flushes: 0,
            idle_unused_slots: 0,
        }
    }
    pub(super) fn record(&mut self) {
        self.records = self.records.saturating_add(1);
    }
    pub(super) fn packed(&mut self) {
        self.packed_requests = self.packed_requests.saturating_add(1);
    }
    pub(super) fn idle(&mut self, pending: usize) {
        if pending != 0 {
            self.idle_flushes = self.idle_flushes.saturating_add(1);
            self.idle_unused_slots = self
                .idle_unused_slots
                .saturating_add(self.batch.saturating_sub(pending));
        }
    }
    pub(super) fn finish(self, complete: bool) {
        let Some((path, run)) = self.destination else {
            return;
        };
        // Closing a partial pack leaves unused batch slots. Only that measured
        // waste can increase the ideal batch count; an empty flush adds none.
        let idle_added_requests = self
            .records
            .saturating_add(self.idle_unused_slots)
            .div_ceil(self.batch)
            .saturating_sub(self.records.div_ceil(self.batch));
        let report = serde_json::json!({
            "run": run, "pid": std::process::id(), "call": self.call,
            "complete": complete, "batch": self.batch, "records": self.records,
            "packed_requests": self.packed_requests, "idle_flushes": self.idle_flushes,
            "idle_added_requests": idle_added_requests,
        });
        let _guard = WRITER.lock().unwrap_or_else(PoisonError::into_inner);
        let written = (|| -> std::io::Result<()> {
            let mut file = std::fs::OpenOptions::new().append(true).open(path)?;
            serde_json::to_writer(&mut file, &report)?;
            file.write_all(b"\n")
        })();
        if written.is_err() {
            // Diagnostics must preserve the product result. The requesting
            // controller rejects missing or incomplete per-call reports.
            log::warn!("input packing diagnostics could not be written");
        }
    }
}
