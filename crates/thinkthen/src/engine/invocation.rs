//! Engine-edge correlation uses no caller data and retains prepared-send identity.

use crate::core::hex;
use crate::core::{CallId, SdkRequestId, Surface};
use crate::engine::error::Error;
use sha2::{Digest as _, Sha256};
use std::fmt;
use std::hash::{BuildHasher as _, RandomState};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

// RandomState is the existing retry entropy primitive. Add the current PID
// on every allocation, including after fork, and a checked process sequence.
// Correlation is not authentication; no evidence, route, model or key is hashed.
fn fresh(domain: &[u8]) -> Result<String, Error> {
    let sequence = SEQUENCE
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |held| {
            held.checked_add(1)
        })
        .map_err(|_| Error::Defect("the correlation identity sequence is exhausted"))?;
    let pid = std::process::id();
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update([0]);
    digest.update(pid.to_be_bytes());
    digest.update(sequence.to_be_bytes());
    for part in 0_u64..4 {
        digest.update(
            RandomState::new()
                .hash_one((pid, sequence, part))
                .to_be_bytes(),
        );
    }
    Ok(hex(&digest.finalize()))
}

#[derive(Clone, Debug)]
pub(crate) struct Invocation {
    pub(crate) call_id: CallId,
    pub(crate) surface: Surface,
}

impl Invocation {
    fn new(surface: Surface) -> Result<Self, Error> {
        let call_id = CallId::new(fresh(b"thinkthen.call-id/1")?)
            .map_err(|_| Error::Defect("a generated call identity is invalid"))?;
        Ok(Self { call_id, surface })
    }

    pub(crate) fn user_agent(&self) -> String {
        format!("thinkthen/{} ({})", env!("CARGO_PKG_VERSION"), self.surface)
    }
}

pub(crate) fn request_id() -> Result<SdkRequestId, Error> {
    SdkRequestId::new(fresh(b"thinkthen.sdk-request-id/1")?)
        .map_err(|_| Error::Defect("a generated request identity is invalid"))
}

/// A shared invocation starts once after admission; clones share its identity.
#[derive(Clone)]
pub(crate) struct Context {
    surface: Surface,
    held: Arc<OnceLock<Result<Invocation, Error>>>,
}

impl Context {
    pub(crate) fn new(surface: Surface) -> Self {
        Self {
            surface,
            held: Arc::default(),
        }
    }

    pub(crate) fn get(&self) -> Result<&Invocation, Error> {
        self.held
            .get_or_init(|| Invocation::new(self.surface))
            .as_ref()
            .map_err(Clone::clone)
    }

    pub(crate) fn call_id(&self) -> Option<CallId> {
        self.held
            .get()?
            .as_ref()
            .ok()
            .map(|invocation| invocation.call_id.clone())
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(Surface::Rust)
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("surface", &self.surface)
            .finish_non_exhaustive()
    }
}
