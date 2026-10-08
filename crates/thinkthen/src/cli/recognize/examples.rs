//! Files are read only at the edge; every example uses core admission and rendering.
use crate::core::{RecognizeSpec, Record};
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

pub(super) fn selected(
    record: &Record,
    pointer: Option<&str>,
    spec: &RecognizeSpec,
) -> Result<RecognizeSpec, Failure> {
    let mut spec = spec.clone();
    if let Some(pointer) = pointer {
        let pointer = crate::core::Pointer::new(pointer)
            .map_err(|_| Failure::Usage("--examples-field needs a valid JSON Pointer"))?;
        if let Some(examples) = crate::core::selected_examples(record, &pointer).map_err(failure)? {
            crate::core::render_examples(&spec, &examples).map_err(failure)?;
            spec.examples = examples;
        }
    }
    Ok(spec)
}

pub(super) fn seeds(
    record: &Record,
    pointer: Option<&str>,
    mut spec: RecognizeSpec,
) -> Result<RecognizeSpec, Failure> {
    if let Some(pointer) = pointer {
        let pointer = crate::core::Pointer::new(pointer)
            .map_err(|_| Failure::Usage("--seed-spans-field needs a valid JSON Pointer"))?;
        if let Some(seeds) =
            crate::core::selected_seeds(record, &pointer).map_err(Failure::Usage)?
        {
            spec.seed_spans = seeds;
        }
    }
    Ok(spec)
}
