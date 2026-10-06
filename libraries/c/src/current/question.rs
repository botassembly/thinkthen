//! Select native question grammar; never parse a question independently.
use super::QuestionHandle;
use crate::failures::Failure;
use serde_json::value::RawValue;
use thinkthen::{LoadedQuestion, Question, QuestionSet, RankSet, Recognize, Relate};
#[derive(Clone)]
pub(crate) enum Native {
    Atomic(LoadedQuestion),
    Rank(Question),
    Find(Question),
    Set(QuestionSet),
    RankSet(RankSet),
    Recognize(Recognize),
    Relate(Relate),
}
pub(crate) fn parse(kind: u32, json: String) -> Result<QuestionHandle, Failure> {
    let native = match kind {
        1..=5 => Native::Atomic(Question::from_json(&json)?),
        6 => Native::RankSet(RankSet::from_json(&json)?),
        8 => Native::Set(QuestionSet::from_json(&json)?),
        9 => Native::Recognize(Recognize::from_json(&json)?),
        10 => Native::Relate(Relate::from_json(&json)?),
        _ => return Err(Failure::usage("invalid question kind")),
    };
    Ok(QuestionHandle { json, native })
}
pub(crate) fn load(path: &str) -> Result<QuestionHandle, Failure> {
    let json = thinkthen::read_question_file(path)
        .map_err(|_| Failure::local("the question file could not supply bounded UTF-8 content"))?;
    let body: std::collections::BTreeMap<String, Box<RawValue>> = serde_json::from_str(&json)
        .map_err(|_| Failure::local("the question file has invalid question content"))?;
    let kind = if body.contains_key("questions") {
        8
    } else if body.contains_key("recognize") {
        9
    } else if body.contains_key("relate") {
        10
    } else {
        1
    };
    parse(kind, json).map_err(|_| Failure::local("the question file has invalid question content"))
}
impl QuestionHandle {
    pub(crate) fn fits(&self, kind: u32) -> bool {
        match (&self.native, kind) {
            (Native::Atomic(question), 1..=5) => {
                use thinkthen::QuestionKind as K;
                matches!(
                    (question.kind(), kind),
                    (K::Decide, 1 | 5) | (K::Choose, 2) | (K::Tag, 3) | (K::Score, 4)
                )
            }
            (Native::Rank(_) | Native::RankSet(_), 6)
            | (Native::Find(_), 7)
            | (Native::Set(_), 6 | 8)
            | (Native::Recognize(_), 9)
            | (Native::Relate(_), 10) => true,
            _ => false,
        }
    }
}

pub(crate) fn plain(
    kind: u32,
    text: String,
    none: bool,
    json: String,
) -> Result<QuestionHandle, Failure> {
    let native = if kind == 6 {
        Native::Rank(Question::rank(&text)?)
    } else {
        let q = Question::find(&text)?;
        Native::Find(if none { q.offering_none()? } else { q })
    };
    Ok(QuestionHandle { json, native })
}
