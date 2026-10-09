//! Owned native sessions and packets have no foreign or engine-table borrows.
pub(crate) mod errors;
use thinkthen::{RequestSession, RequestSessionResult};

/// Independent session owner. Free once after concurrent operations return.
#[derive(Debug)]
pub struct SessionHandle(pub(crate) RequestSession);

/// Independent packet owner. It remains live after its session and engine are freed.
#[derive(Debug)]
pub struct SessionResultHandle {
    pub(crate) _packet: RequestSessionResult,
}
