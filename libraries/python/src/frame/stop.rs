//! Frame worker refusals and the final account carried with returned errors.

use pyo3::prelude::*;
use serde_json::Value;
use thinkthen::{Error, ErrorKind, Facts};

use crate::result;
use crate::worker::{Failure, WorkerError};
use crate::{raised, usage};

#[derive(Debug)]
pub(super) enum Stop {
    Engine(Error),
    Accounted(Box<AccountedFailure>),
    Said(String, Option<Box<Facts>>),
    /// A question event the binding could not write, after the call's facts.
    Unwritten(Box<Facts>),
}

#[derive(Debug)]
pub(super) struct AccountedFailure {
    pub(super) error: Error,
    pub(super) facts: Facts,
    pub(super) details: Vec<Value>,
}

impl Stop {
    pub(super) fn after(sentence: impl Into<String>, facts: Facts) -> Self {
        Self::Said(sentence.into(), Some(Box::new(facts)))
    }

    pub(super) fn with_facts(self, facts: Facts) -> Self {
        match self {
            Self::Said(sentence, _) => Self::Said(sentence, Some(Box::new(facts))),
            other => other,
        }
    }
}

impl WorkerError for Stop {
    fn facts(&self) -> Option<Facts> {
        match self {
            Self::Engine(error) => error.facts().cloned(),
            Self::Accounted(account) => Some(account.facts.clone()),
            Self::Said(_, facts) => facts.as_deref().cloned(),
            Self::Unwritten(facts) => Some(facts.as_ref().clone()),
        }
    }

    fn unaccounted(self, facts: Facts) -> Self {
        match self {
            Self::Said(sentence, None) => Self::Said(sentence, Some(Box::new(facts))),
            other => other,
        }
    }

    fn raised(&self, py: Python<'_>) -> PyErr {
        match self {
            Self::Engine(error) => raised(py, error),
            Self::Accounted(account) => {
                let raised = raised(py, &account.error);
                if let Ok(value) = result::python_facts(py, &account.facts) {
                    let _set = raised.value(py).setattr("facts", value);
                }
                if let Ok(value) = result::python_details(py, &account.details) {
                    let _set = raised.value(py).setattr("details", value);
                }
                raised
            }
            Self::Said(sentence, facts) => {
                let error = usage(py, sentence);
                if let Some(facts) = facts
                    && let Ok(value) = result::python_facts(py, facts)
                {
                    let _set = error.value(py).setattr("facts", value);
                }
                error
            }
            Self::Unwritten(facts) => {
                let error = crate::defect(py, result::UNWRITTEN);
                if let Ok(value) = result::python_facts(py, facts) {
                    let _set = error.value(py).setattr("facts", value);
                }
                error
            }
        }
    }

    fn failure(&self) -> Failure {
        let engine = |error: &Error| Failure {
            kind: error.kind().name(),
            message: error.to_string(),
            retryable: error.retryable(),
        };
        match self {
            Self::Engine(error) => engine(error),
            Self::Accounted(account) => engine(&account.error),
            Self::Said(message, _) => Failure {
                kind: "usage",
                message: message.clone(),
                retryable: false,
            },
            Self::Unwritten(_) => Failure {
                kind: ErrorKind::Defect.name(),
                message: result::UNWRITTEN.to_owned(),
                retryable: false,
            },
        }
    }

    fn details(&self) -> Option<Vec<Value>> {
        match self {
            Self::Accounted(account) => Some(account.details.clone()),
            _ => None,
        }
    }
}

impl From<Error> for Stop {
    fn from(error: Error) -> Self {
        Self::Engine(error)
    }
}

impl From<String> for Stop {
    fn from(sentence: String) -> Self {
        Self::Said(sentence, None)
    }
}

impl From<&'static str> for Stop {
    fn from(sentence: &'static str) -> Self {
        Self::Said(sentence.to_owned(), None)
    }
}
