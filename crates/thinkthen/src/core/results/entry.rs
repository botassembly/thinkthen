//! Typed form entries carry their own state without invented record metadata.
use super::*;
/// One named form entry, with the same loud read and state checks as a result row.
#[derive(Debug, Clone, Serialize)]
#[serde(bound(serialize = ""))]
pub struct Entry<F: Function> {
    function: FunctionTag<F>,
    #[serde(flatten)]
    outcome: Outcome<F::Answer, F::Detail>,
}
impl<F: Function> Entry<F> {
    pub(super) fn new(outcome: Outcome<F::Answer, F::Detail>) -> Self {
        Self {
            function: FunctionTag(std::marker::PhantomData),
            outcome,
        }
    }
    /// Read the plain answer, or fail loudly when there is none.
    /// # Errors
    /// Returns not-sure or failed, just as the enclosing row's answer read does.
    pub fn answer(&self) -> Result<F::Read<'_>, NoAnswer> {
        self.outcome.read::<F>()
    }
    /// Take ownership of the answer.
    /// # Errors
    /// Returns not-sure or failed when there is no answer.
    pub fn into_answer(self) -> Result<F::Owned, NoAnswer> {
        self.outcome.into_answer().map(F::take)
    }
    /// The entry's function.
    #[must_use]
    pub const fn function(&self) -> FunctionName {
        F::NAME
    }
    /// The entry's independent state.
    #[must_use]
    pub fn status(&self) -> Status {
        self.outcome.status()
    }
    /// Whether this entry answered.
    #[must_use]
    pub fn is_answered(&self) -> bool {
        self.status() == Status::Answered
    }
    /// Whether this entry was not sure.
    #[must_use]
    pub fn is_unsure(&self) -> bool {
        self.status() == Status::Unsure
    }
    /// Whether this entry failed.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        self.status() == Status::Failed
    }
    /// The optional probability, always safe to inspect.
    #[must_use]
    pub fn probability(&self) -> Option<Probability> {
        self.outcome.probability()
    }
    /// The optional error, always safe to inspect.
    #[must_use]
    pub fn error(&self) -> Option<&Error> {
        self.outcome.error()
    }
    /// The function's optional detail.
    #[must_use]
    pub fn detail(&self) -> Option<&F::Detail> {
        self.outcome.detail()
    }
    /// Return this entry unchanged unless it failed.
    /// # Errors
    /// Returns the entry's record error, never an error for uncertainty.
    pub fn err_if_failed(self) -> Result<Self, Error> {
        if let Some(error) = self.outcome.failure::<F>() {
            return Err(error.clone());
        }
        Ok(self)
    }
}
impl<F: Function> PartialEq for Entry<F> {
    fn eq(&self, other: &Self) -> bool {
        self.outcome == other.outcome
    }
}
impl<F: Function> std::fmt::Display for Entry<F> {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::answer::display(&self.outcome, out, F::format)
    }
}
impl<'de, F: Function> Deserialize<'de> for Entry<F> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let entry = AnnotationEntry::deserialize(deserializer)?;
        if entry.function() != F::NAME {
            return Err(serde::de::Error::custom(
                "the form entry asks another function",
            ));
        }
        F::entry(&entry, Projection::new())
            .map(Self::new)
            .map_err(serde::de::Error::custom)
    }
}
impl Annotations {
    fn select<F: Function>(&self, name: &str) -> Result<Entry<F>, Error> {
        let entry = self.get(name).ok_or_else(|| {
            Error::from_contract(
                super::super::Code::UsageName,
                format!("the annotate form has no question named `{name}`"),
            )
        })?;
        if entry.function() != F::NAME {
            return Err(Error::from_contract(
                super::super::Code::UsageFunction,
                format!("the annotate question `{name}` asks another function"),
            ));
        }
        Ok(Entry::new(F::entry(entry, Projection::new())?))
    }
}
macro_rules! selector {
    ($method:ident,$question:ident) => {
        impl Annotations {
            #[doc=concat!("Read the named ",stringify!($question)," entry, preserving its state.")]
            /// # Errors
            /// Refuses a missing name or a question under another function.
            pub fn $method(&self, name: &str) -> Result<Entry<questions::$question>, Error> {
                self.select(name)
            }
        }
    };
    (generic $method:ident,$question:ident) => {
        impl Annotations {
            #[doc=concat!("Read the named ",stringify!($question)," entry with enum answers.")]
            /// # Errors
            /// Refuses a missing name, wrong function or an answer outside the enum.
            pub fn $method<C: questions::Choice>(
                &self,
                name: &str,
            ) -> Result<Entry<questions::$question<C>>, Error> {
                self.select(name)
            }
        }
    };
}
selector!(decide, Decide);
selector!(generic choose,Choose);
selector!(generic tag,Tag);
selector!(score, Score);
selector!(find, Find);
selector!(recognize, Recognize);
selector!(relate, Relate);
