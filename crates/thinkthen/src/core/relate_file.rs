use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use thiserror::Error;

use crate::core::digest::hex;
use crate::core::json::Json;
use crate::core::{
    ModelName, Pointer, ProfileName, QuestionFile, RecognizeSpec, RelationRule, RenderError,
    Threshold, json_line,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct RelateFields {
    name: Pointer,
    kind: Pointer,
}

impl RelateFields {
    fn defaults() -> Result<Self, RelateConfigError> {
        Ok(Self {
            name: Pointer::new("/name").map_err(|_| RelateConfigError::Fields)?,
            kind: Pointer::new("/kind").map_err(|_| RelateConfigError::Fields)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct RelatePresence {
    pub(crate) fields: bool,
    pub(crate) threshold: bool,
    pub(crate) model: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RelateSpec {
    fields: RelateFields,
    pub(crate) relations: Vec<RelationRule>,
    pub(crate) threshold: Threshold,
    pub(crate) model: Option<ModelName>,
    pub(crate) profile: Option<ProfileName>,
    presence: RelatePresence,
}

/// The resolved relation question, whose compact bytes the digest hashes.
///
/// Entities, the model, and every runtime backend setting stay outside it.
#[derive(Serialize)]
pub(crate) struct RelateQuestion<'a> {
    verb: &'static str,
    fields: Option<&'a RelateFields>,
    relations: &'a [RelationRule],
    threshold: Threshold,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<&'a ProfileName>,
}

impl RelateQuestion<'_> {
    /// Hash the exact compact bytes the detailed result prints.
    pub(crate) fn sha256(&self) -> Result<String, RenderError> {
        Ok(hex(&Sha256::digest(json_line(self)?.as_bytes())))
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub(crate) enum RelateConfigError {
    #[error("a relate question file is one closed version-one object")]
    Shape,
    #[error("relate takes one or more distinct relation rules")]
    Relations,
    #[error("a relate relation is NAME=SOURCE_KIND:TARGET_KIND, or a bare NAME")]
    Relation,
    #[error("relate fields are RFC 6901 pointers named `name` and `kind`")]
    Fields,
    #[error("the relate threshold is one cut above zero and at most one")]
    Threshold,
    #[error("the question file holds another command's question")]
    WrongVerb,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileShape {
    version: u8,
    relate: RelateShape,
    threshold: Option<CutShape>,
    model: Option<String>,
    profile: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RelateShape {
    fields: Option<FieldsShape>,
    relations: Vec<RelationShape>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FieldsShape {
    name: String,
    kind: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RelationShape {
    name: String,
    source: String,
    target: String,
    reads: Option<String>,
    #[serde(default)]
    either: bool,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CutShape {
    Text(String),
    Number(f64),
}

impl RelateSpec {
    pub(crate) fn inline(relations: &[String], either: bool) -> Result<Self, RelateConfigError> {
        let relations = relations
            .iter()
            .map(|text| inline_rule(text, either))
            .collect::<Result<Vec<_>, _>>()?;
        validate_relations(&relations)?;
        Ok(Self {
            fields: RelateFields::defaults()?,
            relations,
            threshold: Threshold::default(),
            model: None,
            profile: None,
            presence: RelatePresence::default(),
        })
    }

    pub(crate) fn parse(text: &str) -> Result<Self, RelateConfigError> {
        let value = Json::parse(text).map_err(|_| RelateConfigError::Shape)?;
        let Json::Object(members) = &value else {
            return Err(RelateConfigError::Shape);
        };
        if !members.iter().any(|(name, _)| name == "relate")
            && (QuestionFile::parse(text).is_ok() || RecognizeSpec::parse(text).is_ok())
        {
            return Err(RelateConfigError::WrongVerb);
        }
        let parsed: FileShape = serde_json::from_str(text).map_err(|_| RelateConfigError::Shape)?;
        if parsed.version != 1 {
            return Err(RelateConfigError::Shape);
        }
        let fields = parsed
            .relate
            .fields
            .map_or_else(RelateFields::defaults, |fields| {
                Ok(RelateFields {
                    name: Pointer::new(&fields.name).map_err(|_| RelateConfigError::Fields)?,
                    kind: Pointer::new(&fields.kind).map_err(|_| RelateConfigError::Fields)?,
                })
            })?;
        let relations = parsed
            .relate
            .relations
            .into_iter()
            .map(|relation| {
                let name = checked_text(&relation.name)?;
                Ok(RelationRule {
                    reads: relation
                        .reads
                        .map_or_else(|| Ok(name.replace('_', " ")), |reads| checked_text(&reads))?,
                    name,
                    source: checked_text(&relation.source)?,
                    target: checked_text(&relation.target)?,
                    either: relation.either,
                })
            })
            .collect::<Result<Vec<_>, RelateConfigError>>()?;
        validate_relations(&relations)?;
        let threshold = match parsed.threshold {
            None => Threshold::default(),
            Some(CutShape::Text(text)) => parse_typed_cut(&text)?,
            Some(CutShape::Number(number)) => {
                Threshold::cut(number).map_err(|_| RelateConfigError::Threshold)?
            }
        };
        let model = parsed
            .model
            .as_deref()
            .map(ModelName::new)
            .transpose()
            .map_err(|_| RelateConfigError::Shape)?;
        let profile = parsed
            .profile
            .as_deref()
            .map(ProfileName::new)
            .transpose()
            .map_err(|_| RelateConfigError::Shape)?;
        Ok(Self {
            fields,
            relations,
            threshold,
            model,
            profile,
            presence: RelatePresence {
                fields: members
                    .iter()
                    .any(|(name, value)| name == "relate" && value.member("fields").is_some()),
                threshold: members.iter().any(|(name, _)| name == "threshold"),
                model: members.iter().any(|(name, _)| name == "model"),
            },
        })
    }

    pub(crate) fn override_threshold(&mut self, value: &str) -> Result<(), RelateConfigError> {
        self.threshold = parse_typed_cut(value)?;
        Ok(())
    }

    pub(crate) fn override_fields(
        &mut self,
        name: Option<&str>,
        kind: Option<&str>,
    ) -> Result<(), RelateConfigError> {
        if let Some(name) = name {
            self.fields.name = Pointer::new(name).map_err(|_| RelateConfigError::Fields)?;
        }
        if let Some(kind) = kind {
            self.fields.kind = Pointer::new(kind).map_err(|_| RelateConfigError::Fields)?;
        }
        Ok(())
    }

    pub(crate) const fn fields(&self) -> &RelateFields {
        &self.fields
    }

    /// The resolved question; line input has no pointers, so `fields` is null.
    pub(crate) fn question(&self, lines: bool) -> RelateQuestion<'_> {
        RelateQuestion {
            verb: "relate",
            fields: (!lines).then_some(&self.fields),
            relations: &self.relations,
            threshold: self.threshold,
            profile: self.profile.as_ref(),
        }
    }

    pub(crate) const fn name_field(&self) -> &Pointer {
        &self.fields.name
    }

    pub(crate) const fn kind_field(&self) -> &Pointer {
        &self.fields.kind
    }

    pub(crate) const fn presence(&self) -> RelatePresence {
        self.presence
    }
}

fn inline_rule(text: &str, either: bool) -> Result<RelationRule, RelateConfigError> {
    let (name, source, target) = if let Some((name, ends)) = text.split_once('=') {
        let (source, target) = ends.split_once(':').ok_or(RelateConfigError::Relation)?;
        if name.contains('=') || source.contains(['=', ':']) || target.contains(['=', ':']) {
            return Err(RelateConfigError::Relation);
        }
        (name, source, target)
    } else if text.contains(':') {
        return Err(RelateConfigError::Relation);
    } else {
        (text, "*", "*")
    };
    let rule = RelationRule {
        name: checked_text(name)?,
        source: checked_text(source)?,
        target: checked_text(target)?,
        reads: name.replace('_', " "),
        either,
    };
    Ok(rule)
}

fn validate_relations(relations: &[RelationRule]) -> Result<(), RelateConfigError> {
    if relations.is_empty() {
        return Err(RelateConfigError::Relations);
    }
    for (place, relation) in relations.iter().enumerate() {
        if relations
            .iter()
            .skip(place + 1)
            .any(|other| other.name == relation.name)
        {
            return Err(RelateConfigError::Relations);
        }
    }
    Ok(())
}

fn checked_text(value: &str) -> Result<String, RelateConfigError> {
    (!value.trim().is_empty() && !value.chars().any(char::is_control))
        .then(|| value.to_owned())
        .ok_or(RelateConfigError::Relation)
}

fn parse_typed_cut(value: &str) -> Result<Threshold, RelateConfigError> {
    value
        .parse::<Threshold>()
        .ok()
        .filter(|threshold| threshold.is_cut())
        .ok_or(RelateConfigError::Threshold)
}

#[cfg(test)]
#[path = "relate_file/tests.rs"]
mod tests;
