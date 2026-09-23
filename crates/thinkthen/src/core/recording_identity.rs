//! The closed identity stored beside a folder of backend exchanges.

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::core::adapters::built_in;
use crate::core::digest::hex;
use crate::core::text::Url;

const SCHEMA: &str = "thinkthen.backend-folder/1";

/// The backend interface and address one recording folder belongs to.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BackendIdentity(String);

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Document {
    schema: String,
    backend_sha256: String,
}

impl BackendIdentity {
    /// Derive the fixed identity from the canonical adapter and endpoint URL.
    #[must_use]
    pub(crate) fn new(url: &Url) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(built_in::NAME.as_bytes());
        hasher.update(b"\n");
        hasher.update(url.as_str().as_bytes());
        Self(hex(&hasher.finalize()))
    }

    /// Write the complete bounded marker document.
    pub(crate) fn written(&self) -> Result<Vec<u8>, ()> {
        let mut bytes = serde_json::to_vec(&Document {
            schema: SCHEMA.to_owned(),
            backend_sha256: self.0.clone(),
        })
        .map_err(|_| ())?;
        bytes.push(b'\n');
        Ok(bytes)
    }

    /// Read and validate the closed marker shape without exposing its bytes.
    pub(crate) fn read(bytes: &[u8]) -> Result<Self, ()> {
        let document: Document = serde_json::from_slice(bytes).map_err(|_| ())?;
        if document.schema != SCHEMA || !digest(&document.backend_sha256) {
            return Err(());
        }
        Ok(Self(document.backend_sha256))
    }
}

fn digest(value: &str) -> bool {
    value.len() == 64
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

#[cfg(test)]
mod tests {
    use crate::core::text::Url;

    use super::BackendIdentity;

    #[test]
    fn identity_is_closed_bounded_and_round_trips() {
        let url = Url::new("https://example.test/a/very/long/systemone").expect("url");
        let identity = BackendIdentity::new(&url);
        let written = identity.written().expect("identity writes");
        assert!(written.len() < 256);
        assert_eq!(BackendIdentity::read(&written), Ok(identity));
        assert!(BackendIdentity::read(br#"{"schema":"thinkthen.backend-folder/1","backend_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","extra":1}"#).is_err());
        assert!(BackendIdentity::read(br#"{"schema":"foreign","backend_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#).is_err());
        assert!(
            BackendIdentity::read(
                br#"{"schema":"thinkthen.backend-folder/1","backend_sha256":"AAAA"}"#
            )
            .is_err()
        );
    }
}
