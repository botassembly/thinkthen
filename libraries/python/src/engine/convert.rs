//! Existing bare annotation conversions share one safe native failure table.
use super::judgment;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use thinkthen::{Annotated, FailureCause, Judgment};
/// A failed annotate question's cause, as the shared cases spell it.
pub(crate) const fn cause(cause: FailureCause) -> &'static str {
    match cause {
        FailureCause::MissingAnswer => "missing_answer",
        FailureCause::WrongKind => "wrong_kind",
        FailureCause::MissingProbability => "missing_probability",
        FailureCause::InvalidProbability => "invalid_probability",
        FailureCause::InvalidDistribution => "invalid_distribution",
        FailureCause::UnexpectedProbability => "unexpected_probability",
    }
}

pub(crate) fn annotated(py: Python<'_>, value: Annotated) -> PyResult<Py<PyAny>> {
    judgment(
        py,
        match value {
            Annotated::Decision(held) => Judgment::Decision(held),
            Annotated::Choice(pick) => Judgment::Choice(pick),
            Annotated::Score(position) => Judgment::Score(position),
            Annotated::Tags(labels) => Judgment::Tags(labels),
            Annotated::Failed(failed) => {
                let marker = PyDict::new(py);
                marker.set_item("kind", failed.kind().name())?;
                marker.set_item("cause", cause(failed.cause()))?;
                let outer = PyDict::new(py);
                outer.set_item("failed", marker)?;
                return Ok(outer.into_any().unbind());
            }
        },
    )
}
