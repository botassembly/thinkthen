//! Read one named context once and carry its raw-byte identity into each row.

use std::fs;
use std::path::Path;

use crate::core::{Evidence, bytes_sha256};
use crate::failure::Failure;
use crate::failure::context::Error;

#[derive(Clone)]
pub(super) struct Context {
    evidence: Evidence,
    digest: String,
}

pub(super) fn for_run(
    path: Option<&Path>,
    streams: bool,
    text_question: bool,
) -> Result<Option<Context>, Failure> {
    let Some(path) = path else {
        return Ok(None);
    };
    if !streams {
        return Err(Failure::Context(Error::SingleDocument));
    }
    if !text_question {
        return Err(Failure::Context(Error::StructuredQuestion));
    }
    Context::read(path).map(Some)
}

impl Context {
    pub(super) fn read(path: &Path) -> Result<Self, Failure> {
        let bytes = fs::read(path).map_err(|error| Failure::Context(Error::Open(error)))?;
        let digest = bytes_sha256(&bytes);
        let text = String::from_utf8(bytes).map_err(|_| Failure::Context(Error::NotUtf8))?;
        let evidence = Evidence::new(text).map_err(|_| Failure::Context(Error::Empty))?;
        Ok(Self { evidence, digest })
    }

    pub(super) fn evidence(&self) -> Evidence {
        self.evidence.clone()
    }

    pub(super) fn digest(&self) -> &str {
        &self.digest
    }
}
