//! Labelled questions take descriptions by name; the engine refuses a name that is not an answer.
use crate::core::ChoiceOption;
use crate::{Choose, Code, Error, Question, Rank, Score, Tag};
use std::collections::BTreeMap;

/// Describe each named option, and let the shared question admission refuse
/// the names that match none, with its own code and message.
fn describe<N: Into<String>, D: Into<String>>(
    options: &mut [ChoiceOption],
    descriptions: impl IntoIterator<Item = (N, D)>,
) -> BTreeMap<String, String> {
    let mut unmatched = BTreeMap::new();
    for (name, description) in descriptions {
        let (name, description) = (name.into(), description.into());
        match options.iter_mut().find(|option| option.label() == name) {
            Some(option) => *option = option.clone().description(description),
            None => {
                unmatched.insert(name, description);
            }
        }
    }
    unmatched
}

fn admitted<Q: Into<Question> + Clone>(
    question: Q,
    unmatched: BTreeMap<String, String>,
) -> Result<Q, Error> {
    if unmatched.is_empty() {
        return Ok(question);
    }
    let mut definition = serde_json::to_value(question.clone().into()).map_err(|_| {
        Error::from_contract(Code::InternalBug, "the question could not be encoded")
    })?;
    if let Some(fields) = definition.as_object_mut() {
        fields.insert("descriptions".into(), serde_json::json!(unmatched));
    }
    match Question::from_json(&definition.to_string()) {
        Err(error) => Err(error),
        Ok(_) => Err(Error::from_contract(
            Code::InternalBug,
            "the engine admitted descriptions for unknown names",
        )),
    }
}

macro_rules! descriptions {
    ($($question:ident.$($field:ident).+ => $names:literal),+ $(,)?) => {$(
        impl $question {
            #[doc = concat!("Describe ", $names, " by name, as `[(\"name\", \"description\")]` or a map.")]
            /// # Errors
            /// Returns the shared question admission error when a name is not
            #[doc = concat!("one of the ", $names, ".")]
            pub fn descriptions<N: Into<String>, D: Into<String>>(
                mut self,
                descriptions: impl IntoIterator<Item = (N, D)>,
            ) -> Result<Self, Error> {
                let unmatched = describe(&mut self.$($field).+, descriptions);
                admitted(self, unmatched)
            }
        }
    )+};
}
descriptions!(
    Choose.inner.options => "the choices",
    Tag.inner.labels => "the labels",
    Score.inner.levels => "the levels",
    Rank.inner.levels => "the levels",
);
