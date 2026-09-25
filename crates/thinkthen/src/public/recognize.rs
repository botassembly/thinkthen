//! Named-entity recognition and the relation rules `relate` shares.

use std::fmt;
use std::ops::Range;
use std::path::Path;

use crate::core::{self, ModelName, RecognizeSpec, RecognizedName, default_kinds};
use crate::public::engine::Engine;
use crate::public::error::Error;
use crate::public::options::{CallOptions, Stop};
use crate::public::question::{self, Description};
use crate::public::results::Written;

fn nonblank(value: &str, what: &str) -> Result<String, Error> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(Error::usage(format!(
            "{what} is text with no control characters, not white space"
        )));
    }
    Ok(value.to_owned())
}

pub(super) fn cut(value: f64) -> Result<f64, Error> {
    question::cut(value).map(|_| value)
}

/// One kind of name recognition looks for, and what it means.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Kind {
    name: String,
    description: Option<Description>,
}

impl Kind {
    /// A kind and its optional description.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a blank name.
    pub fn new(name: &str, description: Option<Description>) -> Result<Self, Error> {
        Ok(Self {
            name: nonblank(name, "a kind")?,
            description,
        })
    }

    /// The kind's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// What it means, when given.
    #[must_use]
    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }
}

/// One named relation between a source kind and a target kind; `*` is any kind.
#[derive(Clone, Debug, PartialEq)]
pub struct RelationRule(core::RelationRule);

impl RelationRule {
    /// A relation from `source` to `target`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn one_way(name: &str, source: &str, target: &str) -> Result<Self, Error> {
        Ok(Self(core::RelationRule {
            name: nonblank(name, "a relation name")?,
            source: nonblank(source, "a relation source")?,
            target: nonblank(target, "a relation target")?,
            reads: name.replace('_', " "),
            either: false,
        }))
    }

    /// A relation that holds in either direction.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn both_ways(name: &str, source: &str, target: &str) -> Result<Self, Error> {
        let mut rule = Self::one_way(name, source, target)?;
        rule.0.either = true;
        Ok(rule)
    }

    /// The words the model reads for this relation; the name with spaces by default.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn reads(mut self, value: &str) -> Result<Self, Error> {
        self.0.reads = nonblank(value, "a relation's reading")?;
        Ok(self)
    }

    /// The relation's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.0.name
    }
}

pub(super) fn add_rule(
    rules: &mut Vec<core::RelationRule>,
    rule: RelationRule,
) -> Result<(), Error> {
    if rules.iter().any(|held| held.name == rule.0.name) {
        return Err(Error::usage("each relation has its own name"));
    }
    rules.push(rule.0);
    Ok(())
}

pub(super) use crate::public::question::model_of as model;

/// A recognition request: its kinds, relations, cuts, and model.
#[derive(Clone, Debug, PartialEq)]
pub struct Recognize(RecognizeSpec);

impl Recognize {
    /// Start one; with no kind it looks for people, organizations, and places.
    #[must_use]
    pub fn builder() -> RecognizeBuilder {
        RecognizeBuilder {
            kinds: Vec::new(),
            relations: Vec::new(),
            threshold: None,
            relation_threshold: None,
            model: None,
        }
    }

    /// Read one `recognize @FILE` question file. An `on` naming a part of a
    /// record is refused, because a library call's evidence is one whole text.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] naming what the file breaks.
    pub fn from_json(value: &str) -> Result<Self, Error> {
        let spec = RecognizeSpec::parse(value).map_err(Error::refused)?;
        if spec.on.iter().any(|pointer| !pointer.as_str().is_empty()) {
            return Err(Error::usage(
                "a library recognize reads its evidence whole, so it takes no `on`",
            ));
        }
        Ok(Self(spec))
    }

    /// Read one recognize question file from disk.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Local`] when the file cannot be read or breaks a rule.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = std::fs::read_to_string(path)
            .map_err(|_| Error::local("the recognize file could not be read"))?;
        Self::from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }
}

/// A recognition request under construction.
#[derive(Debug)]
pub struct RecognizeBuilder {
    kinds: Vec<Kind>,
    relations: Vec<core::RelationRule>,
    threshold: Option<f64>,
    relation_threshold: Option<f64>,
    model: Option<ModelName>,
}

impl RecognizeBuilder {
    /// Look for this kind, after the kinds already added.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a repeated kind or a twenty-first.
    pub fn kind(mut self, value: Kind) -> Result<Self, Error> {
        if self.kinds.len() == 20 || self.kinds.iter().any(|held| held.name == value.name) {
            return Err(Error::usage(
                "recognize takes 1 to 20 distinct, nonblank kinds",
            ));
        }
        self.kinds.push(value);
        Ok(self)
    }

    /// Relate the recognized names under this rule.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a repeated rule name.
    pub fn relation(mut self, value: RelationRule) -> Result<Self, Error> {
        add_rule(&mut self.relations, value)?;
        Ok(self)
    }

    /// The strength a name must reach; 0.5 by default.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a cut outside zero to one.
    pub fn threshold(mut self, value: f64) -> Result<Self, Error> {
        self.threshold = Some(cut(value)?);
        Ok(self)
    }

    /// The probability a relation must reach; 0.5 by default.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a cut outside zero to one.
    pub fn relation_threshold(mut self, value: f64) -> Result<Self, Error> {
        self.relation_threshold = Some(cut(value)?);
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
    /// Returns [`Error::Usage`] for a relation naming an absent kind.
    pub fn build(self) -> Result<Recognize, Error> {
        let kinds = if self.kinds.is_empty() {
            default_kinds()
        } else {
            self.kinds
                .into_iter()
                .map(|kind| {
                    Ok((
                        kind.name,
                        kind.description.map(|held| held.core()).transpose()?,
                    ))
                })
                .collect::<Result<_, Error>>()?
        };
        let written = |value: Option<f64>| value.map(|cut| cut.to_string());
        let mut spec = RecognizeSpec::from_parts(
            kinds,
            self.relations,
            written(self.threshold).as_deref(),
            written(self.relation_threshold).as_deref(),
        )
        .map_err(Error::refused)?;
        spec.model = self.model;
        Ok(Recognize(spec))
    }
}

/// One recognized name, its kind, its place in the text, and its strength.
/// Offsets count Unicode scalar values.
#[derive(Clone, PartialEq)]
pub struct RecognizedEntity(RecognizedName);

impl fmt::Debug for RecognizedEntity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl RecognizedEntity {
    /// The name as it reads in the text.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// Its kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.0.kind
    }

    /// The first scalar value's place.
    #[must_use]
    pub fn start(&self) -> usize {
        self.0.start
    }

    /// The place after the last scalar value.
    #[must_use]
    pub fn end(&self) -> usize {
        self.0.end
    }

    /// How strongly the tokens read as this kind.
    #[must_use]
    pub fn strength(&self) -> f64 {
        self.0.strength
    }

    /// The byte range in `text`, or `None` when the offsets do not fit it.
    #[must_use]
    pub fn byte_range(&self, text: &str) -> Option<Range<usize>> {
        let byte = |place: usize| {
            text.char_indices()
                .map(|(byte, _)| byte)
                .chain([text.len()])
                .nth(place)
        };
        let range = byte(self.0.start)?..byte(self.0.end)?;
        (range.start <= range.end).then_some(range)
    }

    /// The name's text in `text`, or `None` when the offsets do not fit it.
    #[must_use]
    pub fn name_in<'a>(&self, text: &'a str) -> Option<&'a str> {
        text.get(self.byte_range(text)?)
    }
}

/// One relation between two recognized names.
#[derive(Clone, Debug, PartialEq)]
pub struct Relation {
    relation: String,
    source: RecognizedEntity,
    target: RecognizedEntity,
    probability: f64,
}

impl Relation {
    /// The rule's name.
    #[must_use]
    pub fn relation(&self) -> &str {
        &self.relation
    }

    /// The source name.
    #[must_use]
    pub fn source(&self) -> &RecognizedEntity {
        &self.source
    }

    /// The target name.
    #[must_use]
    pub fn target(&self) -> &RecognizedEntity {
        &self.target
    }

    /// The relation's probability.
    #[must_use]
    pub fn probability(&self) -> f64 {
        self.probability
    }
}

/// The names one text holds, and their relations when rules were given.
#[derive(Clone, Debug, PartialEq)]
pub struct Recognized {
    entities: Vec<RecognizedEntity>,
    relations: Option<Vec<Relation>>,
    json: Written,
}

impl Recognized {
    /// Every recognized name, in text order.
    #[must_use]
    pub fn entities(&self) -> &[RecognizedEntity] {
        &self.entities
    }

    /// The relations, or `None` when no rule was given.
    #[must_use]
    pub fn relations(&self) -> Option<&[Relation]> {
        self.relations.as_deref()
    }

    /// The bare `recognize` object the command prints for this text.
    #[must_use]
    pub fn to_json(&self) -> String {
        self.json.text()
    }
}

impl Engine {
    /// Recognize the names in one text, then relate them under the rules.
    ///
    /// # Errors
    ///
    /// Returns the call's [`Error`]; a failed answer is [`Error::Backend`].
    pub fn recognize(&self, ask: &Recognize, evidence: &str) -> Result<Recognized, Error> {
        self.recognize_with(ask, evidence, CallOptions::new())
    }

    /// [`Engine::recognize`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::recognize`].
    pub fn recognize_with(
        &self,
        ask: &Recognize,
        evidence: &str,
        options: CallOptions<'_>,
    ) -> Result<Recognized, Error> {
        let engine = self.for_model(ask.0.model.as_ref())?;
        let stop = Stop::begin(options)?;
        let found = stop
            .run(|cancel| {
                engine
                    .recognize(&ask.0, evidence, cancel)
                    .map_err(Error::from)
            })?
            .value;
        let json = Written::of(&found)?;
        Ok(Recognized {
            json,
            entities: found.entities.into_iter().map(RecognizedEntity).collect(),
            relations: found.relations.map(|edges| {
                edges
                    .into_iter()
                    .map(|edge| Relation {
                        relation: edge.relation,
                        source: RecognizedEntity(edge.source),
                        target: RecognizedEntity(edge.target),
                        probability: edge.probability,
                    })
                    .collect()
            }),
        })
    }
}
