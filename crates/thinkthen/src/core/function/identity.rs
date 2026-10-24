//! Explicit published names and field order for execution identity.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::core::{ChoiceOption, Threshold};

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum Definition<'a> {
    YesNo {
        no: Option<&'a str>,
        question: &'a str,
        threshold: String,
        yes: Option<&'a str>,
    },
    Choice {
        options: Vec<OptionDefinition<'a>>,
        question: &'a str,
        threshold: String,
    },
    Tags {
        labels: &'a [String],
        question: &'a str,
        threshold: String,
    },
    Scale {
        levels: &'a [String],
        question: &'a str,
    },
    Find {
        question: &'a str,
        threshold: String,
    },
    Names {
        kinds: Vec<OptionDefinition<'a>>,
        threshold: String,
    },
    Relations {
        rules: Vec<RuleDefinition<'a>>,
        threshold: String,
    },
    Rank {
        question: Box<(&'static str, Self)>,
    },
    Annotation {
        questions: BTreeMap<&'a str, (&'static str, Self)>,
    },
}

#[derive(Serialize)]
pub(crate) struct OptionDefinition<'a> {
    description: Option<&'a str>,
    label: &'a str,
}

impl<'a> From<&'a ChoiceOption> for OptionDefinition<'a> {
    fn from(option: &'a ChoiceOption) -> Self {
        Self {
            description: option.description_text(),
            label: option.label(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct RuleDefinition<'a> {
    pub(crate) either: bool,
    pub(crate) name: &'a str,
    pub(crate) reads: &'a str,
    pub(crate) source: &'a str,
    pub(crate) target: &'a str,
}

impl Threshold {
    pub(crate) fn canonical(self) -> String {
        self.to_string()
    }
}
