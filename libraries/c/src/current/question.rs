//! Select native question grammar; never parse a question independently.
use super::QuestionHandle;
use crate::failures::Failure;
use serde_json::value::RawValue;
use thinkthen::{
    LoadedQuestion, Question, QuestionSet, RankSet, Recognize, RecordChooseQuestion, Relate,
};
#[derive(Clone)]
pub(crate) enum Native {
    Atomic(LoadedQuestion),
    DynamicChoose(RecordChooseQuestion),
    Rank(Question),
    Find(Question),
    Set(QuestionSet),
    RankSet(RankSet),
    Recognize(Recognize),
    Relate(Relate),
}
pub(crate) fn parse(kind: u32, json: String) -> Result<QuestionHandle, Failure> {
    let mut reading = None;
    let native = match kind {
        1..=5 => Native::Atomic(Question::from_json(&json)?),
        6 => Native::RankSet(RankSet::from_json(&json)?),
        8 => Native::Set(QuestionSet::from_json(&json)?),
        7 => {
            let (question, selected) = thinkthen::FindQuestionFile::from_json(&json)?.into_parts();
            reading = Some(selected);
            Native::Find(question)
        }
        9 => {
            let (question, selected) =
                thinkthen::RecognizeQuestionFile::from_json(&json)?.into_parts();
            reading = Some(selected);
            Native::Recognize(question)
        }
        10 => Native::Relate(Relate::from_json(&json)?),
        _ => return Err(Failure::usage("invalid question kind")),
    };
    Ok(QuestionHandle {
        json,
        native,
        descriptor: None,
        reading,
    })
}
pub(crate) fn dynamic(json: String) -> Result<QuestionHandle, Failure> {
    let native = Native::DynamicChoose(RecordChooseQuestion::from_json(&json)?);
    Ok(QuestionHandle {
        json,
        native,
        descriptor: None,
        reading: None,
    })
}
pub(crate) fn rank(json: String) -> Result<QuestionHandle, Failure> {
    let native = Native::Rank(Question::rank_from_json(&json)?);
    Ok(QuestionHandle {
        json,
        native,
        descriptor: None,
        reading: None,
    })
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
    } else if body.contains_key("find") {
        7
    } else {
        1
    };
    let parsed = if body.contains_key("choose") && !body.contains_key("options") {
        dynamic(json)
    } else {
        parse(kind, json)
    };
    parsed.map_err(|_| Failure::local("the question file has invalid question content"))
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
            (Native::DynamicChoose(_), 2) => true,
            (Native::Rank(_) | Native::RankSet(_), 6)
            | (Native::Find(_), 7)
            | (Native::Set(_), 6 | 8)
            | (Native::Recognize(_), 9)
            | (Native::Relate(_), 10) => true,
            _ => false,
        }
    }
}
