//! The kinds of question a model answers, with no wire format.

use crate::core::probability::Probability;

/// One question for the model. Adapters turn asks into request bodies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Ask {
    /// A question whose request carries this shared field.
    WithState {
        /// The exact text shared by the request's questions.
        state: std::sync::Arc<str>,
        /// The question answered against that text.
        question: Box<Ask>,
    },
    /// A question choosing among options in caller order.
    Choice {
        /// The question with its quoted field.
        instructions: String,
        /// The ordered options and descriptions.
        options: Vec<ChoiceOption>,
    },
    /// A yes or no question.
    YesNo {
        /// The question, with its field quoted in it.
        instructions: String,
        /// What a yes means, if the question says.
        yes: Option<String>,
        /// What a no means, if the question says.
        no: Option<String>,
    },
    /// A number on an ordered scale.
    Scale {
        /// The question, with its field quoted in it.
        instructions: String,
        /// The levels, from lowest to highest.
        levels: Vec<String>,
    },
}

pub(crate) const DEFAULT_STATE: &str = "Each question quotes the text it asks about.";

impl Ask {
    pub(crate) fn variant(&self) -> &'static str {
        match self {
            Self::WithState { question, .. } => question.variant(),
            Self::Choice { .. } => "choice",
            Self::YesNo { .. } => "yes-no",
            Self::Scale { .. } => "scale",
        }
    }
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
    /// Serialize admitted probabilities without thresholding or normalization.
    pub(crate) fn stored(&self) -> Result<Vec<u8>, crate::core::Error> {
        serde_json::to_vec(&self.probabilities).map_err(|_| {
            crate::core::Error::from_contract(
                crate::core::Code::LocalStoreAnswer,
                "the stored answer cannot be encoded",
            )
        })
    }

    /// Admit a persisted answer against the exact materialized ask again.
    pub(crate) fn restore(ask: &Ask, body: &[u8]) -> Result<Self, crate::core::Error> {
        let bad = || {
            crate::core::Error::from_contract(
                crate::core::Code::LocalStoreAnswer,
                "the stored answer has an invalid shape or probability",
            )
        };
        if let Ask::WithState { question, .. } = ask {
            return Self::restore(question, body);
        }
        let probabilities: Vec<Probability> = serde_json::from_slice(body).map_err(|_| bad())?;
        match ask {
            Ask::YesNo { .. } if probabilities.len() == 1 => Ok(Self::choice(probabilities)),
            Ask::Choice { options, .. } if probabilities.len() == options.len() => {
                Ok(Self::choice(probabilities))
            }
            Ask::Scale { levels, .. } if probabilities.len() == levels.len() => {
                Self::scale(probabilities).map_err(|_| bad())
            }
            _ => Err(bad()),
        }
    }
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

pub(crate) use crate::core::questions::AnswerOption as ChoiceOption;
