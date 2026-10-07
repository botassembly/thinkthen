//! Public originals delegate to the same complete annotation renderer as the CLI.
use super::Held;
use crate::core;
use crate::engine::facade;
use crate::public::{CompleteAnnotated, Error};
use crate::result_json::complete::{AnnotationRow, annotation};

pub(super) fn complete<T>(
    engine: &facade::Engine,
    set: &core::QuestionSet,
    answered: facade::Annotation,
    held: &Held<T>,
    at: usize,
    attempts: bool,
) -> Result<CompleteAnnotated, Error> {
    let context_sha256 = held
        .context
        .as_ref()
        .map(crate::public::record_context::digest)
        .transpose()?;
    let canonical = annotation(
        engine,
        set,
        answered,
        AnnotationRow {
            input: held.input.clone(),
            record: at,
            context_sha256,
            attempts,
        },
    )
    .map_err(|_| Error::defect("a complete annotation could not be constructed"))?;
    Ok(CompleteAnnotated { canonical })
}
