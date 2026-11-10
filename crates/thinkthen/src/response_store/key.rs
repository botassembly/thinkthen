//! Complete execution identity, independent of transport slots and thresholds.
use super::ResponseStore;
use sha2::{Digest, Sha256};

pub(super) const FORMAT: i64 = 0;
pub(super) const FINGERPRINT: &str = env!("THINKTHEN_ADAPTER_FINGERPRINT");

#[expect(
    clippy::too_many_arguments,
    reason = "the identity explicitly names every independent execution boundary"
)]
pub(crate) fn identity(
    config: &ResponseStore,
    backend: &str,
    endpoint: &str,
    model: &str,
    wire: &str,
    variant: &str,
    state: &str,
    question: &str,
) -> [u8; 32] {
    let mut hash = Sha256::new();
    for part in [
        "thinkthen-response-ask/0",
        FINGERPRINT,
        wire,
        backend,
        endpoint,
        model,
        &config.generation,
        &config.namespace,
        variant,
        state,
        question,
    ] {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part.as_bytes());
    }
    hash.finalize().into()
}

pub(super) fn digest(parts: &[&[u8]]) -> Vec<u8> {
    let mut hash = Sha256::new();
    for part in parts {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part);
    }
    hash.finalize().to_vec()
}
