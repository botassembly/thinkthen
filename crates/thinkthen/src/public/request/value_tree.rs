//! One host-neutral projection of the already settled native call.
use crate::{AnswerShape, Call, Error, RequestValue, Settled, SettledAnswer};
use serde::Serialize;
use serde_json::value::RawValue;
use std::collections::BTreeMap;

/// Native values for host adapters, with absence separate from an explicit null.
/// Numbers retain the exact token emitted by the existing native serializer.
/// Debug withholds all caller content, including map keys and number tokens.
#[derive(Clone, PartialEq, Eq)]
pub enum RequestValueTree {
    /// Named native fields or an opaque caller object.
    Map(BTreeMap<String, Self>),
    /// Ordered native fields or an opaque caller array.
    List(Vec<Self>),
    /// Exact text.
    String(String),
    /// An exact JSON number token, without conversion through a floating point value.
    Number(String),
    /// A boolean.
    Bool(bool),
    /// An explicit null value.
    Null,
    /// No value exists for this native field or failed annotation member.
    Absent,
}
impl std::fmt::Debug for RequestValueTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestValueTree").finish_non_exhaustive()
    }
}

impl Call<RequestValue> {
    /// Project the complete call and its retained answer for native host adapters.
    /// No judgments, selections, probabilities or resolutions are recomputed.
    /// Missing optional answer fields and failed annotation members become Absent;
    /// authored objects remain opaque even when their keys resemble native fields.
    /// # Errors
    /// Refuses a call without settlement or a native serialization failure.
    pub fn value_tree(&self) -> Result<RequestValueTree, Error> {
        let answer = self
            .answer()
            .ok_or_else(|| projection_error("native call has no settled answer"))?;
        let answer = settled_answer(answer)?;
        let mut tree = serialized(self)?;
        if let RequestValueTree::Map(fields) = &mut tree {
            if let Some(results) = fields.remove("value") {
                fields.insert("results".into(), results);
            }
            if let RequestValueTree::Map(answer_fields) = &answer {
                for name in ["value", "probability", "pick", "index", "resolution"] {
                    if let Some(value) = answer_fields.get(name) {
                        fields.insert(name.into(), value.clone());
                    }
                }
            }
            fields.remove("answer");
        }
        Ok(tree)
    }
}

fn settled_answer(answer: &SettledAnswer) -> Result<RequestValueTree, Error> {
    match answer {
        SettledAnswer::Decide(value) => settled("decide", value),
        SettledAnswer::Choose(value) => settled("choose", value),
        SettledAnswer::Tag(value) => settled("tag", value),
        SettledAnswer::Score(value) => settled("score", value),
        SettledAnswer::Filter(value) => settled("filter", value),
        SettledAnswer::Rank(value) => settled("rank", value),
        SettledAnswer::Find(value) => settled("find", value),
        SettledAnswer::Recognize(value) => settled("recognize", value),
        SettledAnswer::Relate(value) => settled("relate", value),
        SettledAnswer::Annotate(value) => {
            let mut tree = settled("annotate", value)?;
            if let RequestValueTree::Map(fields) = &mut tree
                && let Some(values) = fields.get_mut("value")
            {
                match (values, &value.resolution) {
                    (values, AnswerShape::Scalar(resolution)) => {
                        missing_members(values, resolution);
                    }
                    (RequestValueTree::List(values), AnswerShape::Collection(resolutions)) => {
                        for (value, resolution) in values.iter_mut().zip(resolutions) {
                            missing_members(value, resolution);
                        }
                    }
                    _ => {}
                }
            }
            Ok(tree)
        }
    }
}

fn settled<T: Serialize, P: Serialize>(
    function: &str,
    value: &Settled<T, P>,
) -> Result<RequestValueTree, Error> {
    Ok(RequestValueTree::Map(BTreeMap::from([
        ("function".into(), RequestValueTree::String(function.into())),
        ("value".into(), optional_shape(&value.value)?),
        ("probability".into(), optional_shape(&value.probability)?),
        ("pick".into(), optional_shape(&value.pick)?),
        ("index".into(), optional_shape(&value.index)?),
        ("resolution".into(), serialized(&value.resolution)?),
    ])))
}

fn optional_shape<T: Serialize>(shape: &AnswerShape<Option<T>>) -> Result<RequestValueTree, Error> {
    let one = |value: &Option<T>| match value {
        Some(value) => serialized(value),
        None => Ok(RequestValueTree::Absent),
    };
    match shape {
        AnswerShape::Scalar(value) => one(value),
        AnswerShape::Collection(values) => values
            .iter()
            .map(one)
            .collect::<Result<Vec<_>, _>>()
            .map(RequestValueTree::List),
    }
}

fn missing_members(value: &mut RequestValueTree, resolution: &crate::AnswerResolutions) {
    if let RequestValueTree::Map(fields) = value
        && let crate::AnswerResolutions::Members(members) = resolution
    {
        for (name, resolution) in members {
            if *resolution == crate::AnswerResolution::Failed {
                fields.insert(name.clone(), RequestValueTree::Absent);
            }
        }
    }
}

fn serialized(value: &impl Serialize) -> Result<RequestValueTree, Error> {
    let raw = serde_json::value::to_raw_value(value)
        .map_err(|_| projection_error("native value serialization failed"))?;
    raw_tree(&raw).map_err(|_| projection_error("native value projection failed"))
}

fn raw_tree(raw: &RawValue) -> Result<RequestValueTree, serde_json::Error> {
    let text = raw.get();
    Ok(match text.as_bytes().first() {
        Some(b'{') => {
            let fields: BTreeMap<String, Box<RawValue>> = serde_json::from_str(text)?;
            RequestValueTree::Map(
                fields
                    .into_iter()
                    .map(|(name, value)| raw_tree(&value).map(|value| (name, value)))
                    .collect::<Result<_, _>>()?,
            )
        }
        Some(b'[') => {
            let values: Vec<Box<RawValue>> = serde_json::from_str(text)?;
            RequestValueTree::List(
                values
                    .iter()
                    .map(|value| raw_tree(value))
                    .collect::<Result<_, _>>()?,
            )
        }
        Some(b'"') => RequestValueTree::String(serde_json::from_str(text)?),
        Some(b't') => RequestValueTree::Bool(true),
        Some(b'f') => RequestValueTree::Bool(false),
        Some(b'n') => RequestValueTree::Null,
        _ => RequestValueTree::Number(text.to_owned()),
    })
}

fn projection_error(message: &str) -> Error {
    Error::new(crate::ErrorKind::Local, message)
}
