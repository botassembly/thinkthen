//! Caller wording and list overrides reuse native question resolution.
use super::RequestOptions;
use crate::{Description, Error, RecordOption, core};
use serde::{Deserialize, Serialize, Serializer};

pub(super) fn apply(question: &mut crate::Question, options: &RequestOptions) -> Result<(), Error> {
    if options.yes.is_none()
        && options.no.is_none()
        && options.labels.is_none()
        && options.levels.is_none()
    {
        return Ok(());
    }
    let file = core::QuestionFile::from_question(&question.core);
    let typed = typed(options)?;
    let resolved = core::resolve(file.verb(), None, Some(&file), &typed).map_err(Error::refused)?;
    question.core = resolved
        .question()
        .cloned()
        .ok_or_else(|| Error::defect("wording resolution lost its question"))?;
    Ok(())
}
fn typed(options: &RequestOptions) -> Result<core::Typed, Error> {
    let described = options
        .labels
        .as_ref()
        .or(options.levels.as_ref())
        .map(|values| {
            values
                .iter()
                .map(|value| {
                    Ok((
                        value.name.clone(),
                        value
                            .description
                            .as_ref()
                            .map(Description::core)
                            .transpose()?,
                    ))
                })
                .collect::<Result<Vec<_>, Error>>()
        })
        .transpose()?;
    Ok(core::Typed {
        yes: options
            .yes
            .as_ref()
            .map(|value| value.core().map(|native| native.as_json().clone()))
            .transpose()?,
        no: options
            .no
            .as_ref()
            .map(|value| value.core().map(|native| native.as_json().clone()))
            .transpose()?,
        labels: described,
        ..core::Typed::default()
    })
}

pub(super) fn description<'de, D: serde::Deserializer<'de>>(
    de: D,
) -> Result<Option<Description>, D::Error> {
    let value = core::Json::deserialize(de)?;
    if core::Description::of_json(&value).is_none() {
        return Err(serde::de::Error::custom(
            "a description is text, an object, a list or null",
        ));
    }
    Description::of(value)
        .map(Some)
        .map_err(serde::de::Error::custom)
}
pub(super) fn serialize_description<S: Serializer>(
    value: &Option<Description>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => value
            .core()
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer),
        None => serializer.serialize_none(),
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OptionValue {
    name: String,
    #[serde(default, deserialize_with = "description")]
    description: Option<Description>,
}
#[derive(Serialize)]
struct OptionRef<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<core::Description>,
}
pub(super) fn list<'de, D: serde::Deserializer<'de>>(
    de: D,
) -> Result<Option<Vec<RecordOption>>, D::Error> {
    Vec::<OptionValue>::deserialize(de).map(|values| {
        Some(
            values
                .into_iter()
                .map(|value| RecordOption {
                    name: value.name,
                    description: value.description,
                })
                .collect(),
        )
    })
}
pub(super) fn serialize_list<S: Serializer>(
    value: &Option<Vec<RecordOption>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let values = value
        .as_ref()
        .map(|values| {
            values
                .iter()
                .map(|value| {
                    Ok(OptionRef {
                        name: &value.name,
                        description: value
                            .description
                            .as_ref()
                            .map(Description::core)
                            .transpose()?,
                    })
                })
                .collect::<Result<Vec<_>, Error>>()
        })
        .transpose()
        .map_err(serde::ser::Error::custom)?;
    values.serialize(serializer)
}

pub(super) fn text(
    function: super::RequestFunction,
    text: &str,
    options: &RequestOptions,
) -> Result<super::RequestDefinition, Error> {
    use super::RequestFunction as F;
    let (verb, kind) = match function {
        F::Choose => (core::Verb::Choose, crate::public::question::Kind::Choose),
        F::Tag => (core::Verb::Tag, crate::public::question::Kind::Tag),
        F::Score => (core::Verb::Score, crate::public::question::Kind::Score),
        _ => return Err(Error::defect("list wording entered another function")),
    };
    let resolved =
        core::resolve(verb, Some(text), None, &typed(options)?).map_err(Error::refused)?;
    let question = crate::Question {
        core: resolved
            .question()
            .cloned()
            .ok_or_else(|| Error::defect("text resolution lost its question"))?,
        metadata: core::declaration::QuestionMetadata::default(),
        threshold: resolved.threshold(),
        authored_threshold: false,
        model: None,
        profile: None,
        batch: None,
        kind,
    };
    Ok(super::RequestDefinition::Atomic(
        crate::LoadedQuestion::Question(question),
    ))
}

pub(super) fn validate(
    definition: &super::RequestDefinition,
    options: &RequestOptions,
) -> Result<(), Error> {
    use super::RequestDefinition as D;
    let mut question = match definition {
        D::Atomic(crate::LoadedQuestion::Question(question)) | D::Rank(question) => {
            question.clone()
        }
        D::Atomic(crate::LoadedQuestion::Banded(question)) => question.0.clone(),
        D::Find(question) => question.question().clone(),
        _ => return Ok(()),
    };
    apply(&mut question, options)
}

#[cfg(test)]
#[derive(schemars::JsonSchema)]
#[expect(
    dead_code,
    reason = "schema derivation describes the existing optional ordered description shape"
)]
pub(super) struct OptionSchema {
    name: String,
    description: Option<core::Description>,
}
