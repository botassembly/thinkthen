//! The defect kind is the Rust surface's own error, carried whole.
//!
//! The surface returns the contract's `Error` itself, so the mapping is
//! identity: a defect stays kind `defect`, never a wrong answer, and the
//! retry signal rides with it. The construction stands in for the injected
//! fault main's engine-only case uses; no public fault hook exists.

use thinkthen::{Error, ErrorKind};

#[test]
fn the_defect_kind_is_carried_whole() {
    let held = Error::defect("the engine broke its own contract");
    assert_eq!(held.kind, ErrorKind::Defect);
    assert!(!held.retryable);
    assert!(held.message.contains("the engine broke its own contract"));
    assert_eq!(held.kind.to_string(), "defect");
}
