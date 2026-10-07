//! Authored preparation retained for typed consumers, outside result and key serialization.
use crate::core::{Json, Setting};
#[derive(Clone, Default, Eq)]
pub(crate) struct AuthoredReading {
    pub(crate) model: Option<String>,
    pub(crate) profile: Option<String>,
    pub(crate) batch: Option<Setting>,
    pub(crate) on: Vec<String>,
}
impl AuthoredReading {
    pub(crate) fn of(value: &Json) -> Self {
        let on = match value.member("on") {
            Some(Json::String(pointer)) => vec![pointer.clone()],
            Some(Json::Array(pointers)) => pointers
                .iter()
                .filter_map(Json::as_str)
                .map(str::to_owned)
                .collect(),
            _ => Vec::new(),
        };
        Self {
            model: value
                .member("model")
                .and_then(Json::as_str)
                .map(str::to_owned),
            profile: value
                .member("profile")
                .and_then(Json::as_str)
                .map(str::to_owned),
            batch: value.member("batch").and_then(Setting::of_json),
            on,
        }
    }
}
impl std::fmt::Debug for AuthoredReading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthoredReading").finish_non_exhaustive()
    }
}

impl PartialEq for AuthoredReading {
    fn eq(&self, other: &Self) -> bool {
        let root = |on: &[String]| on.is_empty() || on == [""];
        self.model == other.model
            && self.profile == other.profile
            && self.batch == other.batch
            && (self.on == other.on || (root(&self.on) && root(&other.on)))
    }
}
