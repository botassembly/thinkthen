//! Frame worker refusals and the final account carried with returned errors.

use pyo3::prelude::*;
use serde_json::Value;
use thinkthen::Error;

use crate::result::{self, OwnedFacts};
use crate::worker::{Failure, WorkerError};
use crate::{raised, usage};

#[derive(Debug)]
pub(super) enum Stop {
    Engine(Error),
    Accounted(Box<AccountedFailure>),
    Said(String, Option<OwnedFacts>),
}

#[derive(Debug)]
pub(super) struct AccountedFailure {
    pub(super) error: Error,
    pub(super) facts: OwnedFacts,
    pub(super) details: Vec<Value>,
}

impl Stop {
    pub(super) fn after(sentence: impl Into<String>, facts: OwnedFacts) -> Self {
        Self::Said(sentence.into(), Some(facts))
    }

    pub(super) fn with_facts(self, facts: OwnedFacts) -> Self {
        match self {
            Self::Said(sentence, _) => Self::Said(sentence, Some(facts)),
            other => other,
        }
    }
}

impl WorkerError for Stop {
    fn facts(&self) -> Option<OwnedFacts> {
        match self {
            Self::Engine(error) => error.facts().map(OwnedFacts::from),
            Self::Accounted(account) => Some(account.facts.clone()),
            Self::Said(_, facts) => facts.clone(),
        }
    }

    fn raised(&self, py: Python<'_>) -> PyErr {
        match self {
            Self::Engine(error) => raised(py, error),
            Self::Accounted(account) => {
                let raised = raised(py, &account.error);
                if let Ok(value) = result::python_owned_facts(py, &account.facts) {
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
                    && let Ok(value) = result::python_owned_facts(py, facts)
                {
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
