//! Explicit file-role admission shares the ordinary ordered JSON reader.
use crate::core::Json;
/// The existing native saved-file grammar selected by a caller.
#[derive(Clone, Copy, Debug)]
pub enum QuestionRole {
    /// One decide, choose, tag or score question.
    Atomic,
    /// A choose reading with per-record candidates.
    Choose,
    /// A decide or score reading for ordinary rank.
    Rank,
    /// A question set, including a rank set.
    Set,
    /// A find reading.
    Find,
    /// A recognition file.
    Recognize,
    /// A relation file.
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
