//! Relations between given entities.

use std::fmt;
use std::path::Path;

use serde::Serialize;

use crate::core::{self, ModelName, RelateSpec, RelationEntityView, Withheld};
use crate::engine::facade;
use crate::public::engine::Engine;
use crate::public::error::{Error, ErrorKind};
use crate::public::options::{CallOptions, Stop};
use crate::public::recognize::{RelationRule, add_rule, cut, model};
use crate::public::results::Written;

/// The most entities one `relate` call takes.
const MOST_ENTITIES: usize = 255;

/// A relate request: its rules, cut, and model.
#[derive(Clone, Debug, PartialEq)]
pub struct Relate(RelateSpec);

impl Relate {
    /// Start one.
    #[must_use]
    pub fn builder() -> RelateBuilder {
        RelateBuilder {
            relations: Vec::new(),
            threshold: None,
            model: None,
        }
    }

    /// Read one version-one relate file. A `fields` pointer other than the
    /// default `/name` and `/kind` is refused, because a library entity is
    /// already a name and a kind.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] naming what the file breaks.
    pub fn from_json(value: &str) -> Result<Self, Error> {
        let spec = RelateSpec::parse(value).map_err(Error::refused)?;
        if spec.name_field().as_str() != "/name" || spec.kind_field().as_str() != "/kind" {
            return Err(Error::usage(
                "a library relate reads each entity's name and kind, so `fields` keeps /name and /kind",
            ));
        }
        Ok(Self(spec))
    }

    /// Read one relate file from disk.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Local`] when the file cannot be read or breaks a rule.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = std::fs::read_to_string(path)
            .map_err(|_| Error::local("the relate file could not be read"))?;
        Self::from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }
}

/// A relate request under construction.
#[derive(Debug)]
pub struct RelateBuilder {
    relations: Vec<core::RelationRule>,
    threshold: Option<f64>,
    model: Option<ModelName>,
}

/// The relate file a builder stands for, which the production parser reads.
#[derive(Serialize)]
struct RelateFile<'a> {
    version: u8,
    relate: Rules<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<&'a str>,
}

#[derive(Serialize)]
struct Rules<'a> {
    relations: &'a [core::RelationRule],
}

impl RelateBuilder {
    /// Ask about this rule, after the rules already added.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a repeated rule name.
    pub fn relation(mut self, value: RelationRule) -> Result<Self, Error> {
        add_rule(&mut self.relations, value)?;
        Ok(self)
    }

    /// The probability an edge must reach; 0.5 by default.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a cut outside zero to one.
    pub fn threshold(mut self, value: f64) -> Result<Self, Error> {
        self.threshold = Some(cut(value)?);
        Ok(self)
    }

    /// Ask this model instead of the engine's.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a blank model.
    pub fn model(mut self, value: &str) -> Result<Self, Error> {
        model(&mut self.model, value)?;
        Ok(self)
    }

    /// The finished request.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] with no rule.
    pub fn build(self) -> Result<Relate, Error> {
        let file = RelateFile {
            version: 1,
            relate: Rules {
                relations: &self.relations,
            },
            threshold: self.threshold,
            model: self.model.as_ref().map(ModelName::as_str),
        };
        let text = serde_json::to_string(&file)
            .map_err(|_| Error::defect("a relate request could not be written"))?;
        RelateSpec::parse(&text).map(Relate).map_err(Error::refused)
    }
}

/// One entity `relate` reads: a name and its kind.
#[derive(Clone, Eq, PartialEq)]
pub struct Entity {
    name: String,
    kind: String,
}

impl fmt::Debug for Entity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Entity")
            .field("name", &Withheld(self.name.len()))
            .field("kind", &Withheld(self.kind.len()))
            .finish()
    }
}

impl Entity {
    /// An entity.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a blank name or kind.
    pub fn new(name: &str, kind: &str) -> Result<Self, Error> {
        core::RelationEntity::new(name, kind).map_err(Error::refused)?;
        Ok(Self {
            name: name.to_owned(),
            kind: kind.to_owned(),
        })
    }

    /// The name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    fn of(held: &impl RelationEntityView) -> Self {
        Self {
            name: held.name().to_owned(),
            kind: held.kind().to_owned(),
        }
    }
}

/// One relation between two given entities.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    relation: String,
    source: Entity,
    target: Entity,
    probability: f64,
    json: Written,
}

impl Edge {
    /// The rule's name.
    #[must_use]
    pub fn relation(&self) -> &str {
        &self.relation
    }

    /// The source entity.
    #[must_use]
    pub fn source(&self) -> &Entity {
        &self.source
    }

    /// The target entity.
    #[must_use]
    pub fn target(&self) -> &Entity {
        &self.target
    }

    /// The edge's probability.
    #[must_use]
    pub fn probability(&self) -> f64 {
        self.probability
    }

    /// This edge as one line of the bare `relate` output.
    #[must_use]
    pub fn to_json(&self) -> String {
        self.json.text()
    }
}

impl Engine {
    /// The relations among up to 255 distinct entities, in rule order.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a repeated entity or more than 255, before
    /// any send, and [`Error::Backend`] when the backend fails any question.
    pub fn relate<I>(&self, ask: &Relate, entities: I) -> Result<Vec<Edge>, Error>
    where
        I: IntoIterator<Item = Entity>,
    {
        self.relate_with(ask, entities, CallOptions::new())
    }

    /// [`Engine::relate`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::relate`].
    pub fn relate_with<I>(
        &self,
        ask: &Relate,
        entities: I,
        options: CallOptions<'_>,
    ) -> Result<Vec<Edge>, Error>
    where
        I: IntoIterator<Item = Entity>,
    {
        let pairs: Vec<(String, String)> = entities
            .into_iter()
            .take(MOST_ENTITIES + 1)
            .map(|entity| (entity.name, entity.kind))
            .collect();
        let admitted = ask.0.admit(&pairs).map_err(Error::refused)?;
        let stop = Stop::begin(options)?;
        if admitted.is_empty() {
            return Ok(Vec::new());
        }
        let engine = self.for_model(ask.0.model.as_ref())?;
        let prepared =
            facade::relations(&admitted, &ask.0, engine.backend(), self.profile.as_ref())?;
        let threshold = ask.0.threshold.cut_value().unwrap_or(0.5);
        let execution = stop.run(|cancel| {
            engine
                .relate(prepared, &admitted, threshold, cancel)
                .map_err(Error::from)
        })?;
        if execution.failed > 0 {
            return Err(Error::of(
                ErrorKind::Backend,
                format!(
                    "the backend failed {} of {} relation questions",
                    execution.failed,
                    execution.logical.len()
                ),
            ));
        }
        execution
            .edges
            .into_iter()
            .map(|edge| {
                Ok(Edge {
                    json: Written::of(&edge)?,
                    source: Entity::of(&edge.source),
                    target: Entity::of(&edge.target),
                    probability: edge.probability,
                    relation: edge.relation,
                })
            })
            .collect()
    }
}
