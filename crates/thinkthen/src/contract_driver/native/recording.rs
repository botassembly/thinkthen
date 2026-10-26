//! The private record-store bridge shares admission and never executes a backend.
use crate::{Error, Pointer, Question};

/// Validate stored answer JSON against its admitted typed function and evidence.
/// This bridge supports the unexposed record store; applications consume typed rows.
/// # Errors
/// Returns usage.question for a mismatched typed shape, domain or span.
pub fn validate_recording_answer(
    question: &Question,
    answer: &str,
    evidence: &str,
) -> Result<(), Error> {
    crate::core::questions::recording::validate(question, answer, evidence)
}
/// Materialize exact structured evidence through ordinary record admission.
/// # Errors
/// Returns the shared malformed or missing record error.
pub fn recording_evidence(value: &str) -> Result<String, Error> {
    let pointer: Pointer = "/selected".parse()?;
    let record = crate::core::Record::json(&format!("{{\"selected\":{value}}}"), &pointer);
    crate::core::AdmittedRecord::new(record).map(|record| record.evidence().to_owned())
}
