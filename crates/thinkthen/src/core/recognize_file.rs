//! The closed recognize question-file shape and its canonical identity.

use serde::ser::SerializeMap as _;
use serde::{Serialize, Serializer};
use sha2::{Digest as _, Sha256};
use thiserror::Error;

use crate::core::json::Json;
use crate::core::{Description, Labels, ModelName, Pointer, ProfileName, RelationRule, Threshold};

pub(crate) type RecognizeKinds = Vec<(String, Option<Description>)>;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RecognizeSpec {
    pub(crate) kinds: RecognizeKinds,
    pub(crate) relations: Vec<RelationRule>,
    pub(crate) threshold: Threshold,
    pub(crate) relation_threshold: Threshold,
    pub(crate) model: Option<ModelName>,
    pub(crate) profile: Option<ProfileName>,
    pub(crate) on: Vec<Pointer>,
}

impl Serialize for RecognizeSpec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("verb", "recognize")?;
        map.serialize_entry("kinds", &Kinds(&self.kinds))?;
        if !self.relations.is_empty() {
            map.serialize_entry("relations", &self.relations)?;
        }
        map.serialize_entry("threshold", &self.threshold)?;
        map.serialize_entry("relation_threshold", &self.relation_threshold)?;
        if let Some(profile) = &self.profile {
            map.serialize_entry("profile", profile)?;
        }
        map.end()
    }
}

struct Kinds<'a>(&'a [(String, Option<Description>)]);

impl Serialize for Kinds<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(name, description)| (name, description)))
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub(crate) enum RecognizeConfigError {
    #[error("a recognize question file is one closed version-one object")]
    Shape,
    #[error("recognize takes 1 to 20 distinct, nonblank kinds")]
    Kinds,
    #[error(
        "a recognize relation has a distinct name, source, target, optional reads, and optional either"
    )]
    Relation,
    #[error("a relation source and target name a kind or explicit `*`")]
    Reference,
    #[error("recognize thresholds are single cuts above zero and at most one")]
    Threshold,
}

impl RecognizeSpec {
    pub(crate) fn from_parts(
        kinds: RecognizeKinds,
        relations: Vec<RelationRule>,
        threshold: Option<&str>,
        relation_threshold: Option<&str>,
    ) -> Result<Self, RecognizeConfigError> {
        validate_kinds(&kinds)?;
        validate_relations(&kinds, &relations)?;
        Ok(Self {
            kinds,
            relations,
            threshold: parse_typed_cut(threshold)?,
            relation_threshold: parse_typed_cut(relation_threshold)?,
            model: None,
            profile: None,
            on: Vec::new(),
        })
    }

    pub(crate) fn parse(text: &str) -> Result<Self, RecognizeConfigError> {
        let value = Json::parse(text).map_err(|_| RecognizeConfigError::Shape)?;
        let Json::Object(members) = &value else {
            return Err(RecognizeConfigError::Shape);
        };
        if members.iter().any(|(name, _)| {
            ![
                "version",
                "recognize",
                "threshold",
                "relation_threshold",
                "model",
                "profile",
                "on",
            ]
            .contains(&name.as_str())
        }) {
            return Err(RecognizeConfigError::Shape);
        }
        if value.member("version").and_then(number_u64) != Some(1) {
            return Err(RecognizeConfigError::Shape);
        }
        let Some(Json::Object(recognize)) = value.member("recognize") else {
            return Err(RecognizeConfigError::Shape);
        };
        if recognize
            .iter()
            .any(|(name, _)| !["kinds", "relations"].contains(&name.as_str()))
        {
            return Err(RecognizeConfigError::Shape);
        }
        let kinds = parse_kinds(
            value
                .member("recognize")
                .and_then(|held| held.member("kinds")),
        )?;
        let relations = parse_relations(
            value
                .member("recognize")
                .and_then(|held| held.member("relations")),
        )?;
        validate_relations(&kinds, &relations)?;
        Ok(Self {
            kinds,
            relations,
            threshold: parse_json_cut(value.member("threshold"))?,
            relation_threshold: parse_json_cut(value.member("relation_threshold"))?,
            model: optional_text(&value, "model")?
                .map(ModelName::new)
                .transpose()
                .map_err(|_| RecognizeConfigError::Shape)?,
            profile: optional_text(&value, "profile")?
                .as_deref()
                .map(ProfileName::new)
                .transpose()
                .map_err(|_| RecognizeConfigError::Shape)?,
            on: parse_on(value.member("on"))?,
        })
    }
}

fn number_u64(value: &Json) -> Option<u64> {
    match value {
        Json::Number(number) => number.as_u64(),
        _ => None,
    }
}

fn parse_kinds(value: Option<&Json>) -> Result<RecognizeKinds, RecognizeConfigError> {
    let Some(Json::Object(members)) = value else {
        return Err(RecognizeConfigError::Kinds);
    };
    let kinds = members
        .iter()
        .map(|(name, description)| {
            Description::of_json(description)
                .map(|held| (name.clone(), Some(held)))
                .ok_or(RecognizeConfigError::Kinds)
        })
        .collect::<Result<Vec<_>, _>>()?;
    validate_kinds(&kinds)?;
    Ok(kinds)
}

fn validate_kinds(kinds: &[(String, Option<Description>)]) -> Result<(), RecognizeConfigError> {
    Labels::tags(kinds.to_vec())
        .map(|_| ())
        .map_err(|_| RecognizeConfigError::Kinds)
}

fn parse_relations(value: Option<&Json>) -> Result<Vec<RelationRule>, RecognizeConfigError> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let Json::Array(items) = value else {
        return Err(RecognizeConfigError::Relation);
    };
    items.iter().map(parse_relation).collect()
}

fn parse_relation(value: &Json) -> Result<RelationRule, RecognizeConfigError> {
    let Json::Object(members) = value else {
        return Err(RecognizeConfigError::Relation);
    };
    if members
        .iter()
        .any(|(name, _)| !["name", "source", "target", "reads", "either"].contains(&name.as_str()))
    {
        return Err(RecognizeConfigError::Relation);
    }
    let name = required_text(value, "name")?;
    let source = required_text(value, "source")?;
    let target = required_text(value, "target")?;
    let reads = optional_text(value, "reads")?.unwrap_or_else(|| name.replace('_', " "));
    let either = match value.member("either") {
        None => false,
        Some(Json::Bool(value)) => *value,
        Some(_) => return Err(RecognizeConfigError::Relation),
    };
    Ok(RelationRule {
        name,
        source,
        target,
        reads,
        either,
    })
}

fn required_text(value: &Json, key: &str) -> Result<String, RecognizeConfigError> {
    optional_text(value, key)?
        .filter(|text| !text.trim().is_empty() && !text.chars().any(char::is_control))
        .ok_or(RecognizeConfigError::Relation)
}

fn optional_text(value: &Json, key: &str) -> Result<Option<String>, RecognizeConfigError> {
    match value.member(key) {
        None => Ok(None),
        Some(Json::String(text)) => Ok(Some(text.clone())),
        Some(_) => Err(RecognizeConfigError::Shape),
    }
}

fn validate_relations(
    kinds: &[(String, Option<Description>)],
    relations: &[RelationRule],
) -> Result<(), RecognizeConfigError> {
    for (place, rule) in relations.iter().enumerate() {
        if rule.name.trim().is_empty()
            || rule.reads.trim().is_empty()
            || relations
                .iter()
                .skip(place + 1)
                .any(|other| other.name == rule.name)
        {
            return Err(RecognizeConfigError::Relation);
        }
        for endpoint in [&rule.source, &rule.target] {
            if endpoint != "*" && !kinds.iter().any(|(kind, _)| kind == endpoint) {
                return Err(RecognizeConfigError::Reference);
            }
        }
    }
    Ok(())
}

fn parse_typed_cut(value: Option<&str>) -> Result<Threshold, RecognizeConfigError> {
    value
        .unwrap_or("0.5")
        .parse::<Threshold>()
        .ok()
        .filter(|rule| rule.is_cut())
        .ok_or(RecognizeConfigError::Threshold)
}

fn parse_json_cut(value: Option<&Json>) -> Result<Threshold, RecognizeConfigError> {
    match value {
        None => Ok(Threshold::default()),
        Some(Json::String(text)) => parse_typed_cut(Some(text)),
        Some(Json::Number(number)) => number
            .as_f64()
            .and_then(|held| Threshold::cut(held).ok())
            .ok_or(RecognizeConfigError::Threshold),
        Some(_) => Err(RecognizeConfigError::Threshold),
    }
}

fn parse_on(value: Option<&Json>) -> Result<Vec<Pointer>, RecognizeConfigError> {
    let texts: Vec<&str> = match value {
        None => return Ok(Vec::new()),
        Some(Json::String(text)) => vec![text],
        Some(Json::Array(items)) => items
            .iter()
            .map(Json::as_str)
            .collect::<Option<Vec<_>>>()
            .ok_or(RecognizeConfigError::Shape)?,
        Some(_) => return Err(RecognizeConfigError::Shape),
    };
    texts
        .into_iter()
        .map(Pointer::new)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| RecognizeConfigError::Shape)
}

pub(crate) fn recognize_sha256(spec: &RecognizeSpec) -> Result<String, crate::core::RenderError> {
    let canonical = crate::core::json_line(spec)?;
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    Ok(crate::core::digest::hex(&hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::{RecognizeSpec, recognize_sha256};

    #[test]
    fn file_defaults_and_canonical_identity_are_stable_across_whitespace() {
        let compact =
            RecognizeSpec::parse(r#"{"version":1,"recognize":{"kinds":{"person":"A person."}}}"#)
                .expect("file");
        let spaced = RecognizeSpec::parse(
            "{ \"version\": 1, \"recognize\": { \"kinds\": { \"person\": \"A person.\" } } }",
        )
        .expect("file");
        assert_eq!(compact, spaced);
        assert_eq!(compact.threshold.to_string(), "0.5");
        assert_eq!(compact.relation_threshold.to_string(), "0.5");
        assert_eq!(
            crate::core::json_line(&compact).unwrap(),
            "{\"verb\":\"recognize\",\"kinds\":{\"person\":\"A person.\"},\"threshold\":0.5,\"relation_threshold\":0.5}"
        );
        assert_eq!(
            recognize_sha256(&compact).expect("digest"),
            recognize_sha256(&spaced).expect("digest")
        );
        assert_eq!(
            recognize_sha256(&compact).expect("digest"),
            "0ad7c0a7f97a1e1a4f609448e61f95e69d448121cfcc6ec8c940627c170e201c"
        );
        let complete = RecognizeSpec::parse(r#"{"version":1,"recognize":{"kinds":{"person":"A person.","organization":"An org."},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":true}]},"threshold":0.6,"relation_threshold":0.7,"model":"ignored","profile":"measured-profile","on":"/body"}"#).unwrap();
        assert_eq!(
            crate::core::json_line(&complete).unwrap(),
            r#"{"verb":"recognize","kinds":{"person":"A person.","organization":"An org."},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":true}],"threshold":0.6,"relation_threshold":0.7,"profile":"measured-profile"}"#
        );
        assert_eq!(
            recognize_sha256(&complete).unwrap(),
            "324488ea0ddc9dc5998e8e397c395967ee648757b2e0780f24661dade832a1c9"
        );
    }

    #[test]
    fn invalid_references_wildcard_spelling_and_policy_members_are_refused() {
        for text in [
            r#"{"version":1,"recognize":{"kinds":{"person":"A person."},"relations":[{"name":"x","source":"person","target":"organization"}]}}"#,
            r#"{"version":1,"recognize":{"kinds":{"person":"A person."},"relations":[{"name":"x","source":"any","target":"person"}]}}"#,
            r#"{"version":1,"recognize":{"kinds":{"person":"A person."},"depth":2}}"#,
        ] {
            assert!(RecognizeSpec::parse(text).is_err(), "{text}");
        }
    }
}
