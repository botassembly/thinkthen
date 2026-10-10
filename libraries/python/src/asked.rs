//! Native question-set resolution for dataframe presentation.
use crate::{raised, usage};
use pyo3::prelude::*;
use std::path::PathBuf;
use thinkthen::QuestionKind;
fn kind_name(kind: QuestionKind) -> &'static str {
    match kind {
        QuestionKind::Decide => "decide",
        QuestionKind::Choose => "choose",
        QuestionKind::Tag => "tag",
        QuestionKind::Score => "score",
        QuestionKind::Rank => "rank",
        QuestionKind::Find => "find",
    }
}

/// A named question set for `annotate`.
#[pyclass(frozen, name = "_QuestionSet", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct QuestionSet(pub(crate) thinkthen::QuestionSet);

#[pymethods]
impl QuestionSet {
    #[staticmethod]
    fn _from_json(py: Python<'_>, text: &str) -> PyResult<Self> {
        thinkthen::QuestionSet::from_json(text)
            .map(Self)
            .map_err(|e| raised(py, &e))
    }

    #[staticmethod]
    fn _load(py: Python<'_>, path: PathBuf) -> PyResult<Self> {
        thinkthen::QuestionSet::load(path)
            .map(Self)
            .map_err(|e| raised(py, &e))
    }

    #[staticmethod]
    fn _resolve(py: Python<'_>, request: &str) -> PyResult<Self> {
        crate::guard(py, || {
            let definition = thinkthen::Request::from_json(request)
                .and_then(thinkthen::Request::admit)
                .and_then(|admitted| admitted.resolve_question())
                .map_err(|error| raised(py, &error))?;
            match definition {
                thinkthen::RequestDefinition::Annotate(set) => Ok(Self(set)),
                _ => Err(usage(py, "frame annotation requires a question set")),
            }
        })
    }

    /// Native declared kinds for dataframe column construction, including empty frames.
    fn _members(&self) -> Vec<(String, &'static str)> {
        self.0
            .members()
            .map(|(name, kind)| (name.to_owned(), kind_name(kind)))
            .collect()
    }

    /// The question names, in set order, for a pandas frame's clash check.
    fn _names(&self) -> Vec<String> {
        self.0.members().map(|(name, _)| name.to_owned()).collect()
    }
}
