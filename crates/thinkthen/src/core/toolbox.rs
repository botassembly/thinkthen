//! Strict named questions and their inherited settings.

use std::collections::BTreeMap;
use std::fmt;

use serde::de::{DeserializeOwned, Error as _, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

use crate::core::{
    Ask, ChoiceOption, Choose, Decide, Error, ErrorKind, Find, Function, Outcome, Pointer,
    Recognize, Relate, Score, Tag, Threshold,
};

/// The maximum size of a toolbox in bytes.
pub const TOOLBOX_LIMIT: usize = 1024 * 1024;

/// A validated collection of named questions.
#[derive(Debug, Clone)]
pub struct Toolbox(BTreeMap<String, ToolboxQuestion>);

/// One function and its validated question.
#[derive(Debug, Clone, serde::Serialize)]
pub enum ToolboxFunction {
    /// A line selection.
    Find(Find),
    /// A yes or no question.
    Decide(Decide),
    /// A yes or no question that keeps matching records.
    Filter(Decide),
    /// A single choice.
    Choose(Choose),
    /// Independent labels.
    Tag(Tag),
    /// An ordered scale.
    Score(Score),
    /// Names with optional ordered kinds.
    Recognize(Recognize),
    /// Relations stated between recognized names.
    Relate(Relate),
}

/// A question with shared settings resolved against its own settings.
///
/// Its [`Function`] implementation applies the question and threshold. Callers
/// apply [`Self::on`] to their records before calling it.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolboxQuestion {
    function: ToolboxFunction,
    #[serde(skip)]
    on: Option<Pointer>,
}

impl ToolboxQuestion {
    pub(crate) fn override_on(&mut self, on: &Pointer) {
        self.on = Some(on.clone());
    }

    /// Require the caller's function to match this admitted question.
    /// Ranking accepts a decide or score question.
    ///
    /// # Errors
    /// Returns local.toolbox for a mismatched function.
    pub fn ensure_function(&self, function: &str) -> Result<(), Error> {
        if self.function.name() != function
            && !(function == "rank"
                && matches!(
                    self.function,
                    ToolboxFunction::Decide(_) | ToolboxFunction::Score(_)
                ))
        {
            return Err(Error::new(
                ErrorKind::Toolbox,
                format!(
                    "the toolbox question uses {}, not {function}",
                    self.function.name()
                ),
            ));
        }
        Ok(())
    }

    /// The function this question runs.
    #[must_use]
    pub const fn function(&self) -> &ToolboxFunction {
        &self.function
    }
    /// The question's evidence pointer, including an inherited pointer.
    #[must_use]
    pub const fn on(&self) -> Option<&Pointer> {
        self.on.as_ref()
    }
}

macro_rules! dispatch {
    ($function:expr, $q:ident => $call:expr) => {
        match $function {
            ToolboxFunction::Decide($q) | ToolboxFunction::Filter($q) => $call,
            ToolboxFunction::Choose($q) => $call,
            ToolboxFunction::Tag($q) => $call,
            ToolboxFunction::Score($q) => $call,
            ToolboxFunction::Recognize($q) => $call,
            ToolboxFunction::Relate($q) => $call,
            ToolboxFunction::Find($q) => $call,
        }
    };
}

impl Function for ToolboxFunction {
    const NAME: &'static str = "toolbox";
    fn identity(&self) -> String {
        self.identity_as(self.name())
    }
    fn identity_as(&self, function: &str) -> String {
        dispatch!(self, q => q.identity_as(function))
    }

    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        dispatch!(self, q => q.try_asks_record(record, limits))
    }
    fn followup_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        dispatch!(context.question(), q => Function::followup_record(context.for_function(q), limits))
    }
    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Outcome {
        dispatch!(context.question(), q => Function::read_record(context.for_function(q), limits))
    }
}

impl Function for ToolboxQuestion {
    const NAME: &'static str = "toolbox";
    fn identity(&self) -> String {
        self.function.identity()
    }
    fn identity_as(&self, function: &str) -> String {
        self.function.identity_as(function)
    }

    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        dispatch!(&self.function, q => q.try_asks_record(record, limits))
    }
    fn followup_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        dispatch!(&context.question().function, q => Function::followup_record(context.for_function(q), limits))
    }
    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Outcome {
        dispatch!(&context.question().function, q => Function::read_record(context.for_function(q), limits))
    }
}

impl ToolboxFunction {
    /// Published question fields used by execution identity and recording metadata.
    pub fn definition(&self) -> impl serde::Serialize + '_ {
        self.definition_view()
    }

    pub(crate) fn definition_view(&self) -> crate::core::function::identity::Definition<'_> {
        dispatch!(self, question => question.definition())
    }

    /// The command name of this function.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Decide(_) => "decide",
            Self::Filter(_) => "filter",
            Self::Choose(_) => "choose",
            Self::Tag(_) => "tag",
            Self::Score(_) => "score",
            Self::Recognize(_) => "recognize",
            Self::Relate(_) => "relate",
            Self::Find(_) => "find",
        }
    }
}

impl Toolbox {
    /// Parse and validate every question before any call can send it.
    /// A single top-level question has the name `default`.
    ///
    /// # Errors
    /// Returns `local.toolbox` for oversized, malformed or invalid toolboxes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > TOOLBOX_LIMIT {
            return Err(invalid("the toolbox exceeds its byte limit"));
        }
        let mut fields: Fields =
            serde_json::from_slice(bytes).map_err(|e| invalid(format!("invalid toolbox: {e}")))?;
        let shared = Settings::take(&mut fields)?;
        let questions = if let Some(raw) = fields.0.remove("questions") {
            fields.finish()?;
            let raw: Fields = parse(&raw, "questions")?;
            if raw.0.is_empty() {
                return Err(invalid("questions must contain at least one question"));
            }
            raw.0
                .into_iter()
                .map(|(name, raw)| {
                    crate::core::ask::admit_name(&name)?;
                    if name.trim().is_empty() {
                        return Err(invalid("a question name is blank"));
                    }
                    let fields = parse(&raw, &name)?;
                    Ok((name, build(fields, &shared)?))
                })
                .collect::<Result<_, Error>>()?
        } else {
            BTreeMap::from([("default".to_owned(), build(fields, &shared)?)])
        };
        Ok(Self(questions))
    }

    /// Get a named question. Without a name, require exactly one question.
    ///
    /// # Errors
    /// Returns `local.toolbox` for a missing or ambiguous question name.
    pub fn question(&self, name: Option<&str>) -> Result<&ToolboxQuestion, Error> {
        match name {
            Some(name) => self
                .0
                .get(name)
                .ok_or_else(|| invalid(format!("unknown question `{name}`"))),
            None if self.0.len() == 1 => self
                .0
                .values()
                .next()
                .ok_or_else(|| invalid("the toolbox is empty")),
            None => Err(invalid("the toolbox needs a question name")),
        }
    }

    /// Visit the named questions in name order.
    pub fn questions(&self) -> impl Iterator<Item = (&str, &ToolboxQuestion)> {
        self.0
            .iter()
            .map(|(name, question)| (name.as_str(), question))
    }
}

#[derive(Default)]
struct Settings {
    on: Option<Pointer>,
}

impl Settings {
    fn take(fields: &mut Fields) -> Result<Self, Error> {
        let on = fields
            .take::<String>("on")?
            .map(|text| text.parse())
            .transpose()
            .map_err(|e: Error| invalid(format!("field `on`: {e}")))?;
        Ok(Self { on })
    }
}

fn build(mut fields: Fields, shared: &Settings) -> Result<ToolboxQuestion, Error> {
    let own = Settings::take(&mut fields)?;
    let kinds = [
        "decide",
        "filter",
        "choose",
        "tag",
        "score",
        "recognize",
        "relate",
        "find",
    ];
    if let Some(key) = fields.0.keys().find(|key| {
        !kinds.contains(&key.as_str())
            && !["yes", "no", "threshold", "options", "labels", "levels"].contains(&key.as_str())
    }) {
        return Err(invalid(format!("unknown field `{key}`")));
    }
    let selected: Vec<_> = kinds
        .into_iter()
        .filter(|key| fields.0.contains_key(*key))
        .collect();
    let [kind] = selected.as_slice() else {
        return Err(invalid(format!(
            "a question needs exactly one of {}",
            kinds.join(", ")
        )));
    };
    let kind = *kind;
    let recognize = if kind == "recognize" {
        fields.take::<Vec<String>>(kind)?
    } else {
        None
    };
    let relate = if kind == "relate" {
        fields.take::<Vec<crate::core::function::relate::RawRule>>(kind)?
    } else {
        None
    };
    let text: String = if recognize.is_some() || relate.is_some() {
        String::new()
    } else {
        fields
            .take(kind)?
            .ok_or_else(|| invalid("the question is missing"))?
    };
    let allowed: &[&str] = match kind {
        "decide" | "filter" => &["yes", "no", "threshold"],
        "choose" => &["options", "threshold"],
        "tag" => &["labels", "threshold"],
        "recognize" | "relate" => &["threshold"],
        "find" => &["threshold"],
        _ => &["levels"],
    };
    if let Some(key) = fields.0.keys().find(|key| !allowed.contains(&key.as_str())) {
        return Err(invalid(format!("field `{key}` is not accepted by {kind}")));
    }
    let threshold: Threshold = fields
        .take::<String>("threshold")?
        .map(|text| text.parse())
        .transpose()
        .map_err(|e: Error| invalid(format!("field `threshold`: {e}")))?
        .unwrap_or_default();
    let function = (|| -> Result<ToolboxFunction, Error> {
        Ok(match kind {
            "decide" | "filter" => {
                let mut q = Decide::new(text)?.threshold(threshold);
                if let Some(yes) = fields.take::<String>("yes")? {
                    q = q.yes(yes);
                }
                if let Some(no) = fields.take::<String>("no")? {
                    q = q.no(no);
                }
                if kind == "decide" {
                    ToolboxFunction::Decide(q)
                } else {
                    ToolboxFunction::Filter(q)
                }
            }
            "choose" => {
                let options = fields
                    .take::<Vec<String>>("options")?
                    .unwrap_or_default()
                    .into_iter()
                    .map(ChoiceOption::new)
                    .collect::<Result<Vec<_>, _>>()?;
                ToolboxFunction::Choose(Choose::new(text, options)?.threshold(threshold))
            }
            "relate" => ToolboxFunction::Relate(
                Relate::new(
                    relate
                        .unwrap_or_default()
                        .into_iter()
                        .map(crate::core::function::relate::RawRule::validate)
                        .collect::<Result<Vec<_>, _>>()?,
                )?
                .threshold(threshold)?,
            ),
            "recognize" => ToolboxFunction::Recognize(
                Recognize::new(recognize.unwrap_or_default())?.threshold(threshold)?,
            ),
            "find" => ToolboxFunction::Find(Find::new(text)?.threshold(threshold)?),
            "tag" => ToolboxFunction::Tag(
                Tag::new(
                    text,
                    fields.take::<Vec<String>>("labels")?.unwrap_or_default(),
                )?
                .threshold(threshold)?,
            ),
            _ => ToolboxFunction::Score(Score::new(
                text,
                fields.take::<Vec<String>>("levels")?.unwrap_or_default(),
            )?),
        })
    })()
    .map_err(|e| {
        if e.kind() == ErrorKind::Name {
            e
        } else {
            invalid(format!("question `{kind}`: {e}"))
        }
    })?;
    fields.finish()?;
    Ok(ToolboxQuestion {
        function,
        on: own.on.or_else(|| shared.on.clone()),
    })
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Toolbox, message)
}

fn parse<T: DeserializeOwned>(raw: &RawValue, key: &str) -> Result<T, Error> {
    serde_json::from_str(raw.get()).map_err(|e| invalid(format!("field `{key}`: {e}")))
}

struct Fields(BTreeMap<String, Box<RawValue>>);
impl Fields {
    fn take<T: DeserializeOwned>(&mut self, key: &str) -> Result<Option<T>, Error> {
        self.0.remove(key).map(|raw| parse(&raw, key)).transpose()
    }
    fn finish(self) -> Result<(), Error> {
        self.0
            .keys()
            .next()
            .map_or(Ok(()), |key| Err(invalid(format!("unknown field `{key}`"))))
    }
}
impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FieldsVisitor;
        impl<'de> Visitor<'de> for FieldsVisitor {
            type Value = Fields;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an object with distinct field names")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Fields, M::Error> {
                let mut fields = BTreeMap::new();
                while let Some(key) = map.next_key::<String>()? {
                    if fields.contains_key(&key) {
                        return Err(M::Error::custom(format!("duplicate key `{key}`")));
                    }
                    fields.insert(key, map.next_value()?);
                }
                Ok(Fields(fields))
            }
        }
        deserializer.deserialize_map(FieldsVisitor)
    }
}
