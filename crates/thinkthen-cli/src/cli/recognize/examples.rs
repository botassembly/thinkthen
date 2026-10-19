//! Files are read only at the edge; every example uses core admission and rendering.
use crate::core::RecognizeSpec;
use crate::failure::Failure;
use std::path::Path;

pub(super) fn failure(error: crate::core::ExamplesError) -> Failure {
    crate::engine::error::Error::RecognitionExamples(error).into()
}

pub(super) fn at_record(mut failure: Failure, at: usize) -> Failure {
    if let Failure::Recognize(crate::failure::recognize::Error::Examples { record, .. }) =
        &mut failure
    {
        *record = Some(at + 1);
    }
    failure
}

pub(super) fn shared(path: Option<&Path>, spec: &mut RecognizeSpec) -> Result<(), Failure> {
    let Some(path) = path else {
        return Ok(());
    };
    let text = std::fs::read_to_string(path)
        .map_err(|_| Failure::Usage("recognition examples file must be readable UTF-8"))?;
    let examples = crate::core::example_file(&text).map_err(failure)?;
    crate::core::render_examples(spec, &examples).map_err(failure)?;
    spec.examples = examples;
    Ok(())
}
