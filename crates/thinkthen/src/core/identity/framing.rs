//! Versioned length framing used by saved inputs and logical answers.

use sha2::{Digest as _, Sha256};

pub(crate) fn digest(tag: &str, parts: &[&[u8]]) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(tag.as_bytes());
    hash.update([0]);
    for part in parts {
        // A byte slice cannot exceed u64::MAX on the supported platforms.
        hash.update(u64::try_from(part.len()).unwrap_or(u64::MAX).to_be_bytes());
        hash.update(part);
    }
    hash.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::digest;

    #[test]
    fn framing_retains_empty_parts_and_distinguishes_boundaries_and_domains() {
        assert_ne!(digest("v1", &[b"ab", b"c"]), digest("v1", &[b"a", b"bc"]));
        assert_ne!(digest("v1", &[b"", b"x"]), digest("v1", &[b"x"]));
        assert_ne!(digest("v1", &[b"x"]), digest("v2", &[b"x"]));
        assert_ne!(digest("v1", &[]), digest("v1", &[b""]));
    }
}
