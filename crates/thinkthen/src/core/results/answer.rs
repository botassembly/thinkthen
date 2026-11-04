//! A row hands over its typed answer in one step, and prints as that answer.
use super::{AnnotationEntry, Found, Function, Name, Outcome, Relation, Row, Status, sealed::Text};
use crate::core::{Code, Error, Probability};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroUsize;

/// Why a row holds no answer: it was not sure, or its record failed.
///
/// Not sure is a normal result, not an engine failure, so it has no shared
/// error code. Match both variants to treat them differently:
///
/// ```
/// # use thinkthen::{BackendName, Decide, NoAnswer, Settings, Threshold};
/// # let engine = Settings::default().with_backend(BackendName::Stub).engine(|_| None)?;
/// let refund = Decide::new("Does the customer ask for a refund?")?
///     .threshold(Threshold::band(0.2, 0.8)?);
/// let route = match engine.run(&refund, "I want to send this back.")?.into_answer() {
///     Ok(true) => "refund queue",
///     Ok(false) => "no action",
///     Err(NoAnswer::Unsure { .. }) => "a person reviews it",
///     Err(NoAnswer::Failed(error)) => return Err(error.into()),
/// };
/// # let _ = route;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum NoAnswer {
    /// The probability fell inside the question's threshold band.
    Unsure {
        /// The probability behind the result.
        probability: Probability,
    },
    /// The record failed with its own error.
    Failed(Error),
}
impl fmt::Display for NoAnswer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsure { probability } => {
                write!(formatter, "not sure (probability {})", probability.get())
            }
            Self::Failed(error) => fmt::Display::fmt(error, formatter),
        }
    }
}
impl std::error::Error for NoAnswer {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unsure { .. } => None,
            Self::Failed(error) => Some(error),
        }
    }
}

impl<A, D> Outcome<A, D> {
    fn into_answer(self) -> Result<A, NoAnswer> {
        match self {
            Self::Answered { answer, .. } => Ok(answer),
            Self::Unsure { probability, .. } => Err(NoAnswer::Unsure { probability }),
            Self::Failed { error } => Err(NoAnswer::Failed(error)),
        }
    }
}
impl<F: Function> Row<F> {
    /// Borrow the typed answer.
    /// # Errors
    /// Returns [`NoAnswer::Unsure`] for a not-sure row and [`NoAnswer::Failed`]
    /// with the record's error for a failed row.
    pub fn answer(&self) -> Result<&F::Answer, NoAnswer> {
        match self.outcome() {
            Outcome::Answered { answer, .. } => Ok(answer),
            Outcome::Unsure { probability, .. } => Err(NoAnswer::Unsure {
                probability: *probability,
            }),
            Outcome::Failed { error } => Err(NoAnswer::Failed(error.clone())),
        }
    }
    /// Take the typed answer, so `let is_refund: bool = row.into_answer()?;` is the whole pattern.
    /// # Errors
    /// Returns [`NoAnswer::Unsure`] for a not-sure row and [`NoAnswer::Failed`]
    /// with the record's error for a failed row.
    pub fn into_answer(self) -> Result<F::Answer, NoAnswer> {
        self.outcome.into_answer()
    }
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Answered => "answered",
            Self::Unsure => "unsure",
            Self::Failed => "failed",
        })
    }
}
/// Prints the bare answer as the command does, `unsure`, or `failed: ` and the error code.
impl<A: Text, D> fmt::Display for Outcome<A, D> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Answered { answer, .. } => answer.text(formatter),
            Self::Unsure { .. } => formatter.write_str("unsure"),
            Self::Failed { error } => write!(formatter, "failed: {}", error.code().as_str()),
        }
    }
}
/// Prints the row's answer, `unsure`, or `failed: ` and the error code.
impl<F: Function> fmt::Display for Row<F> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.outcome(), formatter)
    }
}

macro_rules! plain_text {
    ($($answer:ty),+) => {$(
        impl Text for $answer {
            fn text(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self, out)
            }
        }
    )+};
}
plain_text!(bool, f64, NonZeroUsize, String);
/// Several values print separated by tabs, as the command prints them.
fn tabbed<T>(
    out: &mut fmt::Formatter<'_>,
    values: &[T],
    mut one: impl FnMut(&mut fmt::Formatter<'_>, &T) -> fmt::Result,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.write_str("\t")?;
        }
        one(out, value)?;
    }
    Ok(())
}
impl Text for Vec<String> {
    fn text(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        tabbed(out, self, |out, label| out.write_str(label))
    }
}
impl Text for Option<Found> {
    fn text(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref()
            .map_or(Ok(()), |found| out.write_str(found.text()))
    }
}
impl Text for Vec<Name> {
    fn text(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        tabbed(out, self, |out, name| {
            write!(
                out,
                "{}: {} ({}..{})",
                name.kind(),
                name.text(),
                name.start(),
                name.end()
            )
        })
    }
}
impl Text for Vec<Relation> {
    fn text(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        tabbed(out, self, |out, relation| {
            write!(
                out,
                "{} --{}--> {}",
                relation.source().text(),
                relation.relation(),
                relation.target().text()
            )
        })
    }
}
/// An annotate answer: one entry for each named question, in name order.
///
/// It reads as a map of names to entries, and
/// [`answer`](Annotations::answer) gives one question's typed answer:
///
/// ```
/// # use thinkthen::{BackendName, Settings};
/// use thinkthen::{AnnotatableQuestion, Annotate, Choose, Decide};
/// # let engine = Settings::default().with_backend(BackendName::Stub).engine(|_| None)?;
/// let refund = Decide::new("Does the customer request a refund?")?;
/// let about = Choose::new("What is the message about?", ["billing", "shipping"])?;
/// let triage = Annotate::new([
///     ("refund", AnnotatableQuestion::from(refund)),
///     ("topic", about.into()),
/// ])?;
/// let answers = engine.run(&triage, "Please refund my order.")?.into_answer()?;
/// let is_refund: bool = answers.answer::<Decide>("refund")?;
/// let topic: String = answers.answer::<Choose>("topic")?;
/// # let _ = (is_refund, topic);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Annotations(BTreeMap<String, AnnotationEntry>);
impl Annotations {
    /// The typed answer to the question with this name.
    /// # Errors
    /// Returns [`NoAnswer::Unsure`] or [`NoAnswer::Failed`] as that question's
    /// row would. A name that is not in the form fails with `usage.name`, and a
    /// question of another function fails with `usage.function`.
    pub fn answer<F: Function>(&self, name: &str) -> Result<F::Answer, NoAnswer> {
        let entry = self.0.get(name).ok_or_else(|| {
            NoAnswer::Failed(Error::from_contract(
                Code::UsageName,
                format!("the annotate form has no question named `{name}`"),
            ))
        })?;
        let typed = if entry.function() == F::NAME {
            entry
                .outcome_any()
                .downcast_ref::<Outcome<F::Answer, F::Detail>>()
        } else {
            None
        };
        let outcome = typed.ok_or_else(|| {
            NoAnswer::Failed(Error::from_contract(
                Code::UsageFunction,
                format!("the annotate question `{name}` asks another function"),
            ))
        })?;
        outcome.clone().into_answer()
    }
}
impl std::ops::Deref for Annotations {
    type Target = BTreeMap<String, AnnotationEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<BTreeMap<String, AnnotationEntry>> for Annotations {
    fn from(entries: BTreeMap<String, AnnotationEntry>) -> Self {
        Self(entries)
    }
}
impl From<Annotations> for BTreeMap<String, AnnotationEntry> {
    fn from(answers: Annotations) -> Self {
        answers.0
    }
}
impl FromIterator<(String, AnnotationEntry)> for Annotations {
    fn from_iter<I: IntoIterator<Item = (String, AnnotationEntry)>>(entries: I) -> Self {
        Self(entries.into_iter().collect())
    }
}
impl IntoIterator for Annotations {
    type Item = (String, AnnotationEntry);
    type IntoIter = std::collections::btree_map::IntoIter<String, AnnotationEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> IntoIterator for &'a Annotations {
    type Item = (&'a String, &'a AnnotationEntry);
    type IntoIter = std::collections::btree_map::Iter<'a, String, AnnotationEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
/// The same schema as the map it wraps.
impl schemars::JsonSchema for Annotations {
    fn inline_schema() -> bool {
        <BTreeMap<String, AnnotationEntry>>::inline_schema()
    }
    fn schema_name() -> std::borrow::Cow<'static, str> {
        <BTreeMap<String, AnnotationEntry>>::schema_name()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        <BTreeMap<String, AnnotationEntry>>::schema_id()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <BTreeMap<String, AnnotationEntry>>::json_schema(generator)
    }
}
impl Text for Annotations {
    fn text(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries: Vec<_> = self.iter().collect();
        tabbed(out, &entries, |out, (name, entry)| {
            write!(out, "{name}={entry}")
        })
    }
}
