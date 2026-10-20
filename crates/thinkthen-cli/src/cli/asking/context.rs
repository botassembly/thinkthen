//! Read one named context once and carry its raw-byte identity into each row.

use std::fs;
use std::path::Path;

use crate::failure::Failure;
use crate::failure::context::Error;

#[derive(Clone)]
pub(super) struct Context {
    evidence: String,
}

pub(super) fn for_run(
    path: Option<&Path>,
    text_question: bool,
) -> Result<Option<Context>, Failure> {
    let Some(path) = path else {
        return Ok(None);
    };
    if !text_question {
        return Err(Failure::Context(Error::StructuredQuestion));
    }
    Context::read(path).map(Some)
}

pub(crate) fn shared(path: Option<&Path>) -> Result<Option<String>, Failure> {
    path.map(|path| Context::read(path).map(|context| context.evidence))
        .transpose()
}

pub(crate) fn record(
    record: &crate::RawRecord,
    pointer: Option<&str>,
    schema: Option<&crate::InputDeclaration>,
) -> Result<Option<crate::RecordContext>, Failure> {
    let Some(pointer) = pointer else {
        return Ok(None);
    };
    let mut reading = crate::RecordReading::new(&[], Some(pointer), None)
        .map_err(|_| Failure::Usage("--context-field needs a valid JSON Pointer"))?;
    if let Some(schema) = schema {
        reading = reading.with_context_schema(schema.clone());
    }
    reading
        .compose(record.clone())
        .map(|record| record.context)
        .map_err(|_| Failure::Usage("the per-item context does not match context_schema"))
}

impl Context {
    pub(super) fn read(path: &Path) -> Result<Self, Failure> {
        let bytes = fs::read(path).map_err(|error| Failure::Context(Error::Open(error)))?;
        let text = String::from_utf8(bytes).map_err(|_| Failure::Context(Error::NotUtf8))?;
        if text.trim().is_empty() {
            return Err(Failure::Context(Error::Empty));
        }
        Ok(Self { evidence: text })
    }

    pub(super) fn evidence(&self) -> String {
        self.evidence.clone()
    }
}
