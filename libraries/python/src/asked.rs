//! What a call asks, and what `recognize` and `relate` return.
//!
//! A question comes from question-file JSON the package composes from the
//! caller's keywords, or from a file, so parts and files give one digest
//! (decision 8). Recognize and relate asks come from their builders, a file,
//! or JSON (decision 10).

use std::path::PathBuf;

use pyo3::prelude::*;
use thinkthen::{
    BandedQuestion, DecisionQuestion, Description, DetailQuestion, For, Kind, LoadedQuestion,
    QuestionKind, RelationRule, Settings,
};

use crate::{raised, usage};

/// A question under one cut, or a `decide` question under a band.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Asked {
    Plain(thinkthen::Question),
    Banded(BandedQuestion),
}

impl Asked {
    /// The word for its kind. A banded question reads `decide`.
    pub(crate) fn kind(&self) -> &'static str {
        let kind = match self {
            Self::Plain(question) => question.kind(),
            Self::Banded(_) => QuestionKind::Decide,
        };
        match kind {
            QuestionKind::Decide => "decide",
            QuestionKind::Choose => "choose",
            QuestionKind::Tag => "tag",
            QuestionKind::Score => "score",
            QuestionKind::Rank => "rank",
            QuestionKind::Find => "find",
        }
    }

    /// Either form, for a call that takes a cut or a band.
    pub(crate) fn decision(&self) -> &dyn DecisionQuestion {
        match self {
            Self::Plain(question) => question,
            Self::Banded(question) => question,
        }
    }

    /// Either form, for `details`.
    pub(crate) fn detail(&self) -> &dyn DetailQuestion {
        match self {
            Self::Plain(question) => question,
            Self::Banded(question) => question,
        }
    }
}

/// An engine result as a Python result, with the engine's error raised.
fn loaded<T>(py: Python<'_>, made: Result<T, thinkthen::Error>) -> PyResult<T> {
    made.map_err(|error| raised(py, &error))
}

impl From<LoadedQuestion> for Asked {
    fn from(loaded: LoadedQuestion) -> Self {
        match loaded {
            LoadedQuestion::Question(question) => Self::Plain(question),
            LoadedQuestion::Banded(banded) => Self::Banded(banded),
        }
    }
}

/// One built question. Make one with `tt.question`.
#[pyclass(frozen, name = "Question", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Question(pub(crate) Asked);

#[pymethods]
impl Question {
    /// Validate Python's keyword JSON with the shared core settings grammar.
    /// A built question can take call controls without rebuilding its identity.
    #[staticmethod]
    fn _settings(
        py: Python<'_>,
        verb: &str,
        text: Option<&str>,
        keywords: &str,
    ) -> PyResult<(
        Option<Self>,
        Option<usize>,
        bool,
        Option<String>,
        Option<i64>,
    )> {
        let settings = Settings::parse(keywords).map_err(|error| usage(py, &error.to_string()))?;
        let asked = if let Some(text) = text {
            let kind = match verb {
                "decide" => For::Decide,
                "choose" => For::Choose,
                "score" => For::Score,
                "tag" => For::Tag,
                _ => return Err(usage(py, "this verb takes no question keywords")),
            };
            let json = settings
                .question_json(kind, text)
                .map_err(|error| usage(py, &error.to_string()))?;
            Some(Self(
                loaded(py, thinkthen::Question::from_json(&json))?.into(),
            ))
        } else {
            None
        };
        Ok((
            asked,
            settings.batch_records(),
            settings.batch_max(),
            settings.context().map(str::to_owned),
            settings.deadline_ms(),
        ))
    }

    /// Question-file JSON the caller's arguments made. A broken rule is usage.
    #[staticmethod]
    fn _from_json(py: Python<'_>, text: &str) -> PyResult<Self> {
        loaded(py, thinkthen::Question::from_json(text)).map(|made| Self(made.into()))
    }

    /// A question file. A broken rule is local.
    #[staticmethod]
    fn _load(py: Python<'_>, path: PathBuf) -> PyResult<Self> {
        loaded(py, thinkthen::Question::load(path)).map(|made| Self(made.into()))
    }

    /// The question `rank` or `find` asks, from its text.
    #[staticmethod]
    fn _ordering(py: Python<'_>, verb: &str, text: &str) -> PyResult<Self> {
        let made = if verb == "find" {
            thinkthen::Question::find(text)
        } else {
            thinkthen::Question::rank(text)
        };
        made.map(|question| Self(Asked::Plain(question)))
            .map_err(|error| raised(py, &error))
    }

    /// This find question with a `none` candidate beside the units.
    fn _offering_none(&self, py: Python<'_>) -> PyResult<Self> {
        match &self.0 {
            Asked::Plain(question) => question
                .clone()
                .offering_none()
                .map(|question| Self(Asked::Plain(question)))
                .map_err(|error| raised(py, &error)),
            Asked::Banded(_) => Err(usage(py, "only a find question offers none")),
        }
    }

    /// What the question asks: `decide`, `choose`, `tag`, `score`, `rank`, or `find`.
    #[getter]
    fn kind(&self) -> &'static str {
        self.0.kind()
    }

    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        other
            .cast::<Self>()
            .is_ok_and(|other| other.get().0 == self.0)
    }

    fn __repr__(&self) -> String {
        format!("Question(kind='{}')", self.0.kind())
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
        loaded(py, thinkthen::QuestionSet::from_json(text)).map(Self)
    }

    #[staticmethod]
    fn _load(py: Python<'_>, path: PathBuf) -> PyResult<Self> {
        loaded(py, thinkthen::QuestionSet::load(path)).map(Self)
    }

    /// The question names, in set order, for a pandas frame's clash check.
    fn _names(&self) -> Vec<String> {
        self.0.members().map(|(name, _)| name.to_owned()).collect()
    }
}

/// One relation rule from the keyword form: name, source, target, and
/// whether it reads both ways.
type Rule = (String, String, String, bool);

fn rule(py: Python<'_>, (name, source, target, either): &Rule) -> PyResult<RelationRule> {
    let made = if *either {
        RelationRule::both_ways
    } else {
        RelationRule::one_way
    };
    made(name, source, target).map_err(|error| raised(py, &error))
}

/// What `recognize` asks.
#[pyclass(frozen, name = "_Recognize", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Recognize(pub(crate) thinkthen::Recognize);

#[pymethods]
impl Recognize {
    #[staticmethod]
    fn _from_json(py: Python<'_>, text: &str) -> PyResult<Self> {
        loaded(py, thinkthen::Recognize::from_json(text)).map(Self)
    }

    #[staticmethod]
    fn _load(py: Python<'_>, path: PathBuf) -> PyResult<Self> {
        loaded(py, thinkthen::Recognize::load(path)).map(Self)
    }

    /// The keyword form: kinds with optional descriptions, rules, and the cuts.
    #[staticmethod]
    fn _build(
        py: Python<'_>,
        kinds: Vec<(String, Option<String>)>,
        relations: Vec<Rule>,
        threshold: Option<f64>,
        relation_threshold: Option<f64>,
    ) -> PyResult<Self> {
        let refused = |error: thinkthen::Error| raised(py, &error);
        let mut builder = thinkthen::Recognize::builder();
        for (name, meaning) in kinds {
            let described = meaning
                .map(|text| Description::text(&text))
                .transpose()
                .map_err(refused)?;
            builder = builder
                .kind(Kind::new(&name, described).map_err(refused)?)
                .map_err(refused)?;
        }
        for one in &relations {
            builder = builder.relation(rule(py, one)?).map_err(refused)?;
        }
        if let Some(cut) = threshold {
            builder = builder.threshold(cut).map_err(refused)?;
        }
        if let Some(cut) = relation_threshold {
            builder = builder.relation_threshold(cut).map_err(refused)?;
        }
        builder.build().map(Self).map_err(refused)
    }
}

/// What `relate` asks.
#[pyclass(frozen, name = "_Relate", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Relate(pub(crate) thinkthen::Relate);

#[pymethods]
impl Relate {
    #[staticmethod]
    fn _from_json(py: Python<'_>, text: &str) -> PyResult<Self> {
        loaded(py, thinkthen::Relate::from_json(text)).map(Self)
    }

    #[staticmethod]
    fn _load(py: Python<'_>, path: PathBuf) -> PyResult<Self> {
        loaded(py, thinkthen::Relate::load(path)).map(Self)
    }

    #[staticmethod]
    fn _build(py: Python<'_>, relations: Vec<Rule>, threshold: Option<f64>) -> PyResult<Self> {
        let refused = |error: thinkthen::Error| raised(py, &error);
        let mut builder = thinkthen::Relate::builder();
        for one in &relations {
            builder = builder.relation(rule(py, one)?).map_err(refused)?;
        }
        if let Some(cut) = threshold {
            builder = builder.threshold(cut).map_err(refused)?;
        }
        builder.build().map(Self).map_err(refused)
    }
}

/// A name and its kind, as `relate` reads and returns them.
#[pyclass(frozen, skip_from_py_object, module = "thinkthen._thinkthen")]
#[derive(Clone, Debug)]
pub(crate) struct Entity {
    #[pyo3(get)]
    pub(crate) name: String,
    #[pyo3(get)]
    pub(crate) kind: String,
}

#[pymethods]
impl Entity {
    #[new]
    fn new(name: String, kind: String) -> Self {
        Self { name, kind }
    }

    /// Withholds the name and kind, as the Rust `Entity`'s `Debug` does.
    fn __repr__(&self) -> String {
        format!(
            "Entity(name=<{} bytes withheld>, kind=<{} bytes withheld>)",
            self.name.len(),
            self.kind.len()
        )
    }
}

/// One name `recognize` found: its text, where it sits counted in Python
/// string positions, its length in the same unit, its kind, and its strength.
#[pyclass(frozen, skip_from_py_object, module = "thinkthen._thinkthen")]
#[derive(Clone, Debug)]
pub(crate) struct RecognizedEntity {
    #[pyo3(get)]
    pub(crate) text: String,
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize,
    #[pyo3(get)]
    length: usize,
    #[pyo3(get)]
    pub(crate) kind: String,
    #[pyo3(get)]
    strength: f64,
}

#[pymethods]
impl RecognizedEntity {
    /// Withholds the text and kind, as `Entity` does.
    fn __repr__(&self) -> String {
        format!(
            "RecognizedEntity(text=<{} bytes withheld>, start={}, end={}, length={}, kind=<{} bytes withheld>, strength={})",
            self.text.len(),
            self.start,
            self.end,
            self.length,
            self.kind.len(),
            self.strength
        )
    }
}

impl From<&thinkthen::RecognizedEntity> for RecognizedEntity {
    fn from(entity: &thinkthen::RecognizedEntity) -> Self {
        Self {
            text: entity.text().to_owned(),
            start: entity.start(),
            end: entity.end(),
            length: entity.length(),
            kind: entity.kind().to_owned(),
            strength: entity.strength(),
        }
    }
}

/// One relation between two entities and its probability.
#[pyclass(frozen, skip_from_py_object, module = "thinkthen._thinkthen")]
#[derive(Clone, Debug)]
pub(crate) struct Edge {
    #[pyo3(get)]
    relation: String,
    #[pyo3(get)]
    source: Entity,
    #[pyo3(get)]
    target: Entity,
    #[pyo3(get)]
    probability: f64,
}

#[pymethods]
impl Edge {
    fn __repr__(&self) -> String {
        format!(
            "Edge(relation={:?}, source={}, target={}, probability={})",
            self.relation,
            self.source.__repr__(),
            self.target.__repr__(),
            self.probability
        )
    }
}

impl From<&thinkthen::Edge> for Edge {
    fn from(edge: &thinkthen::Edge) -> Self {
        let entity =
            |one: &thinkthen::Entity| Entity::new(one.name().to_owned(), one.kind().to_owned());
        Self {
            relation: edge.relation().to_owned(),
            source: entity(edge.source()),
            target: entity(edge.target()),
            probability: edge.probability(),
        }
    }
}

/// One relation between two recognized names and its probability.
#[pyclass(frozen, skip_from_py_object, module = "thinkthen._thinkthen")]
#[derive(Clone, Debug)]
pub(crate) struct Relation {
    #[pyo3(get)]
    relation: String,
    #[pyo3(get)]
    source: RecognizedEntity,
    #[pyo3(get)]
    target: RecognizedEntity,
    #[pyo3(get)]
    probability: f64,
}

#[pymethods]
impl Relation {
    fn __repr__(&self) -> String {
        format!(
            "Relation(relation={:?}, source={}, target={}, probability={})",
            self.relation,
            self.source.__repr__(),
            self.target.__repr__(),
            self.probability
        )
    }
}

/// What `recognize` found: the names, and the relations when rules were given.
#[pyclass(frozen, skip_from_py_object, module = "thinkthen._thinkthen")]
#[derive(Clone, Debug)]
pub(crate) struct Recognized {
    #[pyo3(get)]
    entities: Vec<RecognizedEntity>,
    #[pyo3(get)]
    relations: Option<Vec<Relation>>,
}

#[pymethods]
impl Recognized {
    fn __repr__(&self) -> String {
        format!(
            "Recognized(entities={}, relations={})",
            self.entities.len(),
            self.relations
                .as_ref()
                .map_or_else(|| "None".to_owned(), |held| held.len().to_string())
        )
    }
}

impl From<&thinkthen::Recognized> for Recognized {
    fn from(found: &thinkthen::Recognized) -> Self {
        Self {
            entities: found
                .entities()
                .iter()
                .map(RecognizedEntity::from)
                .collect(),
            relations: found.relations().map(|relations| {
                relations
                    .iter()
                    .map(|one| Relation {
                        relation: one.relation().to_owned(),
                        source: RecognizedEntity::from(one.source()),
                        target: RecognizedEntity::from(one.target()),
                        probability: one.probability(),
                    })
                    .collect()
            }),
        }
    }
}
