//! The answer key: the right value for each record, and the part it belongs to.

use std::collections::BTreeMap;

use crate::core::json::Json;
use crate::core::measure::answer::{Answer, Said, Verb};
use crate::core::measure::{MeasureError, record_id};

/// The part of the split a key line names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Part {
    /// The records a suggested cut is tuned on.
    Tune,
    /// The records a suggested cut is checked on.
    Held,
}

/// The key's value for one answer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Want {
    /// A yes: a `decide` or `rank` key of yes, or a `tag` label the key lists.
    Yes,
    /// A no.
    No,
    /// The right option, level, or unit.
    Option(String),
}

impl Want {
    /// The value as the output names it.
    pub(crate) fn text(&self) -> &str {
        match self {
            Self::Yes => "yes",
            Self::No => "no",
            Self::Option(option) => option,
        }
    }
}

/// How one answer fared against the key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Outcome {
    /// The answer equals the key.
    Right,
    /// The answer differs from the key.
    Wrong,
    /// The answer said nothing under the rule.
    Unresolved,
    /// The top options tied.
    Tied,
}

/// Grade one answer against its key value.
pub(crate) fn outcome(said: &Said<'_>, want: &Want) -> Outcome {
    let right = match (said, want) {
        (Said::Tied, _) => return Outcome::Tied,
        (Said::Unresolved, _) => return Outcome::Unresolved,
        (Said::Yes, Want::Yes) | (Said::No, Want::No) => true,
        (Said::Option(option), Want::Option(key)) => option == key,
        _ => false,
    };
    if right {
        Outcome::Right
    } else {
        Outcome::Wrong
    }
}

/// One key line.
#[derive(Clone, Debug, PartialEq)]
struct Entry {
    line: usize,
    value: Json,
    part: Option<Part>,
}

/// Every key line by record id. Members other than `id`, `value`, and `part` are ignored.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Key(BTreeMap<String, Entry>);

impl Key {
    /// Read numbered key lines.
    ///
    /// # Errors
    ///
    /// Returns [`MeasureError::KeyLine`] for a line without a usable id or a
    /// value, a repeated id, or a part other than `tune` or `held`.
    pub(crate) fn read(lines: &[(usize, Json)]) -> Result<Self, MeasureError> {
        let mut key = BTreeMap::new();
        for (line, row) in lines {
            let refused = MeasureError::KeyLine(*line);
            let id = row.member("id").and_then(record_id).ok_or(refused)?;
            let value = row.member("value").ok_or(refused)?.clone();
            let part = match row.member("part") {
                None | Some(Json::Null) => None,
                Some(held) => match held.as_str() {
                    Some("tune") => Some(Part::Tune),
                    Some("held") => Some(Part::Held),
                    _ => return Err(refused),
                },
            };
            let entry = Entry {
                line: *line,
                value,
                part,
            };
            if key.insert(id, entry).is_some() {
                return Err(refused);
            }
        }
        Ok(Self(key))
    }

    /// The part the key gives a record, if any.
    pub(crate) fn part(&self, id: &str) -> Option<Part> {
        self.0.get(id).and_then(|entry| entry.part)
    }

    /// The key's value for this answer, or `None` when it is unlabeled.
    ///
    /// # Errors
    ///
    /// Returns [`MeasureError::ChooseKey`] for a `choose` value that is not text, and
    /// [`MeasureError::KeyUnknown`] for a label, level, or unit the answer does not hold.
    pub(crate) fn want(&self, answer: &Answer) -> Result<Option<Want>, MeasureError> {
        let Some(entry) = self.0.get(&answer.id) else {
            return Ok(None);
        };
        let value = match &answer.name {
            None => &entry.value,
            Some(name) => match &entry.value {
                Json::Object(_) => entry.value.member(name).unwrap_or(&Json::Null),
                _ => return Ok(None),
            },
        };
        let unknown = MeasureError::KeyUnknown(entry.line);
        Ok(match (answer.verb, value) {
            (_, Json::Null) => None,
            (Verb::Tag, Json::Array(items)) => {
                let mut listed = Vec::new();
                for item in items {
                    let label = item
                        .as_str()
                        .filter(|label| answer.options.iter().any(|held| held == label));
                    listed.push(label.ok_or(unknown)?);
                }
                let label = answer.label.as_deref().unwrap_or_default();
                Some(if listed.contains(&label) {
                    Want::Yes
                } else {
                    Want::No
                })
            }
            (Verb::Tag, _) => None,
            (Verb::Decide | Verb::Rank, Json::Bool(true)) => Some(Want::Yes),
            (Verb::Decide | Verb::Rank, Json::Bool(false)) => Some(Want::No),
            (Verb::Decide | Verb::Rank, Json::String(text)) if text == "yes" => Some(Want::Yes),
            (Verb::Decide | Verb::Rank, Json::String(text)) if text == "no" => Some(Want::No),
            (Verb::Decide | Verb::Rank, _) => None,
            (Verb::Choose, Json::String(option)) => Some(Want::Option(option.clone())),
            (Verb::Choose, _) => return Err(MeasureError::ChooseKey(entry.line)),
            (Verb::Score | Verb::Find, Json::String(option))
                if answer.options.iter().any(|held| held == option) =>
            {
                Some(Want::Option(option.clone()))
            }
            (Verb::Score | Verb::Find, _) => return Err(unknown),
        })
    }
}
