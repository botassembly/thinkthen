//! Ranking and immutable named question collections.
use super::*;
use std::collections::BTreeMap;

/// A scale-based ranking question.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "RankWire", into = "RankWire")]
#[schemars(with = "RankWire")]
pub struct Rank {
    pub(crate) inner: crate::core::Score,
    pub(crate) top: Option<crate::core::RankLimit>,
    pub(super) evidence: Option<Pointer>,
}
impl Rank {
    /// Admit a question with its required levels.
    /// # Errors
    /// Returns the shared scale admission error.
    pub fn new(
        question: impl Into<String>,
        levels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::Score::new(question, levels)?,
            top: None,
            evidence: None,
        })
    }
    /// Retain at most this many successful ranks.
    /// # Errors
    /// Returns the shared question error outside the portable signed count range.
    pub fn top(mut self, top: std::num::NonZeroUsize) -> Result<Self, Error> {
        self.top = Some(crate::core::RankLimit::new(top.get())?);
        Ok(self)
    }
    /// Select evidence with an admitted JSON Pointer.
    #[must_use]
    pub fn evidence(mut self, pointer: Pointer) -> Self {
        self.evidence = Some(pointer);
        self
    }
    /// The admitted evidence pointer.
    #[must_use]
    pub fn evidence_pointer(&self) -> &Pointer {
        self.evidence
            .as_ref()
            .unwrap_or(&crate::core::record::ROOT_POINTER)
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct RankWire {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: String,
    #[schemars(length(min = MIN_LEVELS))]
    #[serde(deserialize_with = "crate::core::diagnostic::levels")]
    levels: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "Option<std::num::NonZeroUsize>", range(max = crate::core::RankLimit::MAX))]
    #[serde(deserialize_with = "crate::core::diagnostic::top")]
    top: Option<usize>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_evidence"
    )]
    #[schemars(with = "Pointer")]
    evidence: Option<String>,
}
impl TryFrom<RankWire> for Rank {
    type Error = Error;
    fn try_from(value: RankWire) -> Result<Self, Error> {
        let mut question = Self::new(value.question, value.levels)?;
        question.evidence = admit_evidence(value.evidence)?;
        question.top = value.top.map(crate::core::RankLimit::new).transpose()?;
        Ok(question)
    }
}
impl From<Rank> for RankWire {
    fn from(value: Rank) -> Self {
        Self {
            question: value.inner.text,
            levels: value.inner.levels,
            top: value.top.map(|top| top.get()),
            evidence: value.evidence.map(|pointer| pointer.to_string()),
        }
    }
}

/// An immutable named domain of non-annotation questions.
#[non_exhaustive]
#[derive(Debug, Clone, schemars::JsonSchema)]
#[schemars(with = "super::saved::RawSavedToolbox")]
pub struct Toolbox(BTreeMap<String, SimpleQuestion>);
impl Toolbox {
    /// Admit nonempty named typed questions.
    /// # Errors
    /// Returns a shared question or name error.
    pub fn new(questions: BTreeMap<String, SimpleQuestion>) -> Result<Self, Error> {
        admit_names(&questions)?;
        Ok(Self(questions))
    }
    /// Admit a canonical named domain without resolving files or references.
    /// # Errors
    /// Returns the shared per-function question or duplicate-name error.
    #[doc(hidden)]
    pub fn from_definitions_json(json: &str) -> Result<Self, Error> {
        if json.len() > TOOLBOX_LIMIT {
            return Err(question_error(crate::core::diagnostic::requirement(
                "toolbox",
                "JSON within the toolbox byte limit",
                "oversized JSON",
            )));
        }
        serde_json::from_str::<RawToolbox>(json)
            .map_err(|error| {
                question_error(crate::core::diagnostic::context(
                    "questions",
                    &error.to_string(),
                ))
            })?
            .try_into()
    }
    /// Select a question that carries its own domain and options.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&SimpleQuestion> {
        self.0.get(name)
    }
    /// Visit every named typed question.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &SimpleQuestion)> {
        self.0
            .iter()
            .map(|(name, question)| (name.as_str(), question))
    }
}
pub(crate) fn admit_names<T>(questions: &BTreeMap<String, T>) -> Result<(), Error> {
    if questions.len() < MIN_QUESTIONS {
        return Err(question_error(crate::core::diagnostic::requirement(
            "questions",
            "at least one named question",
            "an empty collection",
        )));
    }
    for name in questions.keys() {
        crate::core::ask::admit_name(name)?;
        if name.trim().is_empty() {
            return Err(question_error(crate::core::diagnostic::requirement(
                "name",
                "nonblank text",
                "blank text",
            )));
        }
    }
    Ok(())
}
impl TryFrom<RawToolbox> for Toolbox {
    type Error = Error;
    fn try_from(value: RawToolbox) -> Result<Self, Error> {
        Self::new(
            value
                .0
                .into_iter()
                .map(|(name, question)| Ok((name, question.try_into()?)))
                .collect::<Result<_, Error>>()?,
        )
    }
}
impl From<Toolbox> for BTreeMap<String, SimpleQuestion> {
    fn from(value: Toolbox) -> Self {
        value.0
    }
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
#[serde(transparent)]
#[schemars(extend("minProperties" = MIN_QUESTIONS))]
pub(crate) struct RawNamed<T>(pub(crate) BTreeMap<String, T>);
pub(crate) type RawToolbox = RawNamed<RawSimpleQuestion>;
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for RawNamed<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor<T>(std::marker::PhantomData<T>);
        impl<'de, T: serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<T> {
            type Value = RawNamed<T>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("unique named questions")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                mut map: M,
            ) -> Result<RawNamed<T>, M::Error> {
                let mut entries = BTreeMap::new();
                while let Some(name) = map.next_key::<String>()? {
                    let question = map.next_value::<T>().map_err(|error| {
                        <M::Error as serde::de::Error>::custom(crate::core::diagnostic::context(
                            &format!("[{}]", entries.len()),
                            &error.to_string(),
                        ))
                    })?;
                    if entries.insert(name, question).is_some() {
                        return Err(serde::de::Error::custom(
                            "names: expected unique question names; received a repeated name",
                        ));
                    }
                }
                Ok(RawNamed(entries))
            }
        }
        deserializer.deserialize_map(Visitor(std::marker::PhantomData))
    }
}
