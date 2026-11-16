//! A row hands over its typed answer in one step, and prints as that answer.
use super::{AnnotationEntry, Function, Name, Outcome, Relation, Row, Span, Status, sealed::Text};
use crate::core::{Error, Probability};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroUsize;

/// One string leaf in the command's tab-delimited result text.
#[derive(Debug)]
pub(super) struct Leaf<'a>(pub(super) &'a str);
impl fmt::Display for Leaf<'_> {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in self.0.chars() {
            fmt::Write::write_char(
                out,
                if matches!(ch, '\r' | '\n' | '\t') {
                    ' '
                } else {
                    ch
                },
            )?;
        }
        Ok(())
    }
}

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
    pub(super) fn into_answer(self) -> Result<A, NoAnswer> {
        match self {
            Self::Answered { answer, .. } => Ok(answer),
            Self::Unsure { probability, .. } => Err(NoAnswer::Unsure { probability }),
            Self::Failed { error } => Err(NoAnswer::Failed(error)),
        }
    }
}
impl<A, D> Outcome<A, D> {
    pub(super) fn read<F: Function<Answer = A, Detail = D>>(
        &self,
    ) -> Result<F::Read<'_>, NoAnswer> {
        match self {
            Self::Answered { answer, .. } => Ok(F::read(answer)),
            Self::Unsure { probability, .. } => Err(NoAnswer::Unsure {
                probability: *probability,
            }),
            Self::Failed { error } => Err(NoAnswer::Failed(error.clone())),
        }
    }
    pub(super) fn probability(&self) -> Option<Probability> {
        match self {
            Self::Answered { probability, .. } => *probability,
            Self::Unsure { probability, .. } => Some(*probability),
            Self::Failed { .. } => None,
        }
    }
    pub(super) fn error(&self) -> Option<&Error> {
        match self {
            Self::Failed { error } => Some(error),
            _ => None,
        }
    }
    pub(super) fn detail(&self) -> Option<&D> {
        match self {
            Self::Answered { detail, .. } | Self::Unsure { detail, .. } => detail.as_ref(),
            Self::Failed { .. } => None,
        }
    }
    pub(super) fn failure<F: Function<Answer = A, Detail = D>>(&self) -> Option<&Error> {
        self.error().or_else(|| match self {
            Self::Answered { answer, .. } => F::answer_failure(answer),
            _ => None,
        })
    }
}
pub(super) fn display<A, D>(
    outcome: &Outcome<A, D>,
    out: &mut fmt::Formatter<'_>,
    answer: impl FnOnce(&A, &mut fmt::Formatter<'_>) -> fmt::Result,
) -> fmt::Result {
    match outcome {
        Outcome::Answered { answer: value, .. } => answer(value, out),
        Outcome::Unsure { .. } => out.write_str("unsure"),
        Outcome::Failed { error } => write!(
            out,
            "failed: {}: {}",
            Leaf(error.code().as_str()),
            Leaf(error.message())
        ),
    }
}
fn sealed_text<A: Text>(answer: &A, out: &mut fmt::Formatter<'_>) -> fmt::Result {
    answer.text(out)
}
impl<F: Function, T> Row<F, T> {
    /// Read the typed answer as its plain value or borrow.
    /// # Errors
    /// Returns not-sure or failed when there is no answer.
    pub fn answer(&self) -> Result<F::Read<'_>, NoAnswer> {
        self.outcome.read::<F>()
    }
    /// Take the typed answer, so `let is_refund: bool = row.into_answer()?;` is the whole pattern.
    /// # Errors
    /// Returns [`NoAnswer::Unsure`] for a not-sure row and [`NoAnswer::Failed`]
    /// with the record's error for a failed row.
    pub fn into_answer(self) -> Result<F::Owned, NoAnswer> {
        self.outcome.into_answer().map(F::take)
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
        display(self, formatter, sealed_text::<A>)
    }
}
/// Prints the row's answer, `unsure`, or `failed: ` and the error code.
impl<F: Function, T> fmt::Display for Row<F, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        display(self.outcome(), formatter, F::format)
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
plain_text!(bool, f64, NonZeroUsize);
impl Text for String {
    fn text(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{}", Leaf(self))
    }
}
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
        tabbed(out, self, |out, label| write!(out, "{}", Leaf(label)))
    }
}
impl Text for Option<Span> {
    fn text(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref()
            .map_or(Ok(()), |found| write!(out, "{}", Leaf(found.text())))
    }
}
impl Text for Vec<Name> {
    fn text(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        tabbed(out, self, |out, name| {
            write!(
                out,
                "{}: {} ({}..{})",
                Leaf(name.kind()),
                Leaf(name.text()),
                name.start.canonical(),
                name.end.canonical()
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
                Leaf(relation.source().text()),
                Leaf(relation.relation()),
                Leaf(relation.target().text())
            )
        })
    }
}
/// An annotate answer: one entry for each named question, in name order.
///
/// It reads as a map of names to entries, and
/// typed selectors such as [`decide`](Annotations::decide) give each entry its own loud answer read:
///
/// ```
/// # use thinkthen::{BackendName, Settings};
/// use thinkthen::{AnnotatableQuestion, Annotate, Choose, Decide};
/// # let engine = Settings::default().with_backend(BackendName::Stub).engine(|_| None)?;
/// let refund = Decide::new("Does the customer request a refund?")?;
/// let about = Choose::with_strings("What is the message about?", ["billing", "shipping"])?;
/// let triage = Annotate::new([
///     ("refund", AnnotatableQuestion::from(refund)),
///     ("topic", about.into()),
/// ])?;
/// let answers = engine.run(&triage, "Please refund my order.")?.into_answer()?;
/// let is_refund: bool = answers.decide("refund")?.answer()?;
/// let topic: String = answers.choose::<String>("topic")?.into_answer()?;
/// # let _ = (is_refund, topic);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Annotations(BTreeMap<String, AnnotationEntry>);
impl Annotations {
    pub(crate) fn entries_mut(&mut self) -> &mut BTreeMap<String, AnnotationEntry> {
        &mut self.0
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
            write!(out, "{}={entry}", Leaf(name))
        })
    }
}

macro_rules! no_entry_failures {
    ($($answer:ty),+ $(,)?) => {$(impl super::sealed::Failures for $answer {})+};
}
no_entry_failures!(
    bool,
    String,
    Vec<String>,
    f64,
    NonZeroUsize,
    Option<Span>,
    Vec<Name>,
    Vec<Relation>
);
impl super::sealed::Failures for Annotations {
    fn failure(&self) -> Option<&Error> {
        self.values().find_map(super::AnnotationEntry::error)
    }
}
