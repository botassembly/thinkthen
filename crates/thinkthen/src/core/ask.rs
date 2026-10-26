//! The kinds of question a model answers, with no wire format.

use crate::core::probability::Probability;

/// One question for the model. Adapters turn asks into request bodies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Ask {
    /// A question whose request carries this shared evidence.
    WithState {
        /// The exact text shared by the request's questions.
        state: std::sync::Arc<str>,
        /// The question answered against that text.
        question: Box<Ask>,
    },
    /// A question choosing among options in caller order.
    Choice {
        /// The question with its quoted evidence.
        instructions: String,
        /// The ordered options and descriptions.
        options: Vec<ChoiceOption>,
    },
    /// A yes or no question.
    YesNo {
        /// The question, with its evidence quoted in it.
        instructions: String,
        /// What a yes means, if the question says.
        yes: Option<String>,
        /// What a no means, if the question says.
        no: Option<String>,
    },
    /// A number on an ordered scale.
    Scale {
        /// The question, with its evidence quoted in it.
        instructions: String,
        /// The levels, from lowest to highest.
        levels: Vec<String>,
    },
}

pub(crate) const DEFAULT_STATE: &str = "Each question quotes the text it asks about.";

impl Ask {
    pub(crate) fn with_state(self, state: std::sync::Arc<str>) -> Self {
        Self::WithState {
            state,
            question: Box::new(self),
        }
    }

    pub(crate) fn option_count(&self) -> Option<usize> {
        match self {
            Self::WithState { question, .. } => question.option_count(),
            Self::Choice { options, .. } => Some(options.len()),
            _ => None,
        }
    }

    pub(crate) fn shared_state(&self) -> std::sync::Arc<str> {
        match self {
            Self::WithState { state, .. } => state.clone(),
            _ => DEFAULT_STATE.into(),
        }
    }

    pub(crate) fn state(&self) -> &str {
        match self {
            Self::WithState { state, .. } => state,
            _ => DEFAULT_STATE,
        }
    }
}

/// Probabilities decoded for one ask, in the ask's option or level order.
///
/// Only adapters construct answers. They check the wire kind, option identities,
/// probability ranges and count before exposing a value to a function.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AskAnswer {
    probabilities: Vec<Probability>,
}

impl AskAnswer {
    pub(crate) fn choice(probabilities: Vec<Probability>) -> Self {
        Self { probabilities }
    }

    pub(crate) fn yes_no(p: Probability) -> Self {
        Self {
            probabilities: vec![p],
        }
    }

    pub(crate) fn scale(probabilities: Vec<Probability>) -> Result<Self, crate::core::Error> {
        let total: f64 = probabilities.iter().map(|p| p.get()).sum();
        if !(0.99..=1.01).contains(&total) {
            return Err(crate::core::Error::new(
                crate::core::ErrorKind::Reply,
                "the backend's scale probabilities do not sum to one",
            ));
        }
        Ok(Self { probabilities })
    }

    /// The decoded probabilities, in the order of the original ask.
    #[must_use]
    pub(crate) fn probabilities(&self) -> &[Probability] {
        &self.probabilities
    }

    #[expect(
        clippy::expect_used,
        reason = "a yes/no ask always decodes one probability"
    )]
    pub(crate) fn yes(&self) -> Probability {
        *self.probabilities.first().expect("decoded yes/no answer")
    }
}

/// The first largest probability in a validated nonempty choice.
#[expect(
    clippy::expect_used,
    reason = "validated choice asks have nonempty options"
)]
pub(crate) fn argmax(probabilities: &[Probability]) -> (usize, Probability) {
    probabilities
        .iter()
        .copied()
        .enumerate()
        .reduce(|best, next| if best.1 >= next.1 { best } else { next })
        .expect("nonempty choice")
}

/// Compare canonical forms only; admitted labels retain their original bytes.
pub(crate) fn distinct_labels<'a>(labels: impl IntoIterator<Item = &'a str>) -> bool {
    use unicode_normalization::UnicodeNormalization;

    let mut seen = std::collections::HashSet::new();
    labels
        .into_iter()
        .all(|label| seen.insert(label.nfc().collect::<String>()))
}

pub(crate) fn admit_name(name: &str) -> Result<(), crate::core::Error> {
    if name.contains('\0') {
        return Err(crate::core::Error::new(
            crate::core::ErrorKind::Name,
            crate::core::diagnostic::requirement(
                "name",
                "text without U+0000",
                "a string containing U+0000 (contents redacted)",
            ),
        ));
    }
    Ok(())
}

/// One option in a choice question.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(crate) struct ChoiceOption {
    label: String,
    description: Option<String>,
}

impl ChoiceOption {
    /// An option with a nonblank label.
    ///
    /// # Errors
    /// Returns `usage.question` for a blank label or one containing U+0000.
    pub(crate) fn new(label: impl Into<String>) -> Result<Self, crate::core::Error> {
        let label = label.into();
        admit_name(&label)?;
        if label.trim().is_empty() {
            return Err(crate::core::Error::new(
                crate::core::ErrorKind::Question,
                crate::core::diagnostic::requirement("label", "nonblank text", "blank text"),
            ));
        }
        Ok(Self {
            label,
            description: None,
        })
    }

    /// Describe when this option fits.
    #[must_use]
    pub(crate) fn description(self, description: impl Into<String>) -> Self {
        Self {
            description: Some(description.into()),
            ..self
        }
    }

    /// The option's label.
    #[must_use]
    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    /// The optional description.
    #[must_use]
    pub(crate) fn description_text(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
