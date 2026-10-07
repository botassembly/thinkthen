//! A native selection is retained while DuckDB authorizes and reads its content.
use super::{BridgeText, Reply, reply_boundary, text};
use thinkthen::QuestionFileReference;

/// Resolve metadata without opening content, transferring one selection to the host.
/// # Safety
/// The text and writable out pointer remain live through return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_question_resolve(
    source: BridgeText,
    out: *mut *mut QuestionFileReference,
) -> Reply {
    reply_boundary(|| {
        if out.is_null() {
            return Err(
                crate::complete_native::failure(&crate::complete_native::defect()).to_string(),
            );
        }
        let source = text(source.bytes, source.len)?;
        let reference = if let Some(name) = source.strip_prefix("@@") {
            QuestionFileReference::named(name)
        } else {
            QuestionFileReference::reference(source)
        }
        .map_err(|e| crate::complete_native::failure(&e).to_string())?;
        let path = reference
            .path()
            .to_str()
            .ok_or_else(|| {
                crate::complete_native::failure(&thinkthen::Error::new(
                    thinkthen::ErrorKind::Local,
                    "the question path is not UTF-8",
                ))
                .to_string()
            })?
            .as_bytes()
            .to_vec();
        // SAFETY: the host owns the output range and takes the allocation once.
        unsafe {
            out.write(Box::into_raw(Box::new(reference)));
        }
        Ok(path)
    })
}
/// Release the selection after native parsing and synchronous execution return.
/// # Safety
/// The host relinquishes its one live allocation exactly once.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_question_free(
    reference: *mut QuestionFileReference,
) {
    let _ = super::panic::caught(|| {
        if !reference.is_null() {
            // SAFETY: this is the allocation transferred by resolve, freed once.
            drop(unsafe { Box::from_raw(reference) });
        }
    });
}
