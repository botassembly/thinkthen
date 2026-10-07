//! Explicit file-role admission shares the ordinary ordered JSON reader.
use crate::core::Json;
#[derive(Clone, Copy)]
pub(crate) enum QuestionRole {
    Atomic,
    Choose,
    Rank,
    Set,
    Find,
    Recognize,
    Relate,
}
impl QuestionRole {
    pub(crate) fn differs(self, text: &str) -> bool {
        // Syntax, duplicate members, unsupported keys and multiple roles stay
        // with the owning grammar; only an unambiguous different role is Usage.
        let Ok(Json::Object(members)) = Json::parse(text) else {
            return false;
        };
        let mut roles = members.iter().filter_map(|(key, _)| {
            [
                "decide",
                "choose",
                "tag",
                "score",
                "questions",
                "find",
                "recognize",
                "relate",
            ]
            .contains(&key.as_str())
            .then_some(key.as_str())
        });
        let Some(role) = roles.next() else {
            return false;
        };
        if roles.next().is_some() {
            return false;
        }
        let admitted: &[&str] = match self {
            Self::Atomic => &["decide", "choose", "tag", "score"],
            Self::Choose => &["choose"],
            Self::Rank => &["decide", "score"],
            Self::Set => &["questions"],
            Self::Find => &["find"],
            Self::Recognize => &["recognize"],
            Self::Relate => &["relate"],
        };
        !admitted.contains(&role)
    }
}
