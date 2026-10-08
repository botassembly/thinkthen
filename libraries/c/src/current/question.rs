//! Select native question grammar; never parse a question independently.
use super::QuestionHandle;
use crate::failures::Failure;
use crate::ffi::values as abi;
use crate::ffi::values::{
    THINKTHEN_FUNCTION_ANNOTATE_V1 as ANNOTATE, THINKTHEN_FUNCTION_CHOOSE_V1 as CHOOSE,
    THINKTHEN_FUNCTION_DECIDE_V1 as DECIDE, THINKTHEN_FUNCTION_FILTER_V1 as FILTER,
    THINKTHEN_FUNCTION_FIND_V1 as FIND, THINKTHEN_FUNCTION_RANK_V1 as RANK,
    THINKTHEN_FUNCTION_RECOGNIZE_V1 as RECOGNIZE, THINKTHEN_FUNCTION_RELATE_V1 as RELATE,
    THINKTHEN_FUNCTION_SCORE_V1 as SCORE, THINKTHEN_FUNCTION_TAG_V1 as TAG,
};
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
        DECIDE | CHOOSE | TAG | SCORE | FILTER => Native::Atomic(Question::from_json(&json)?),
        RANK => Native::RankSet(RankSet::from_json(&json)?),
        ANNOTATE => Native::Set(QuestionSet::from_json(&json)?),
        FIND => {
            let (question, selected) = thinkthen::FindQuestionFile::from_json(&json)?.into_parts();
            reading = Some(selected);
            Native::Find(question)
        }
        RECOGNIZE => {
            let (question, selected) =
                thinkthen::RecognizeQuestionFile::from_json(&json)?.into_parts();
            reading = Some(selected);
            Native::Recognize(question)
        }
        RELATE => Native::Relate(Relate::from_records_json(&json)?),
        _ => return Err(Failure::usage("invalid question kind")),
    };
    Ok(finish(json, native, reading))
}
pub(crate) fn dynamic(json: String) -> Result<QuestionHandle, Failure> {
    let native = Native::DynamicChoose(RecordChooseQuestion::from_json(&json)?);
    Ok(finish(json, native, None))
}
pub(crate) fn rank(json: String) -> Result<QuestionHandle, Failure> {
    let native = Native::Rank(Question::rank_from_json(&json)?);
    Ok(finish(json, native, None))
}
fn finish(
    json: String,
    native: Native,
    reading: Option<thinkthen::RecordReading>,
) -> QuestionHandle {
    use super::author::{Author, AuthorOwner, native_author};
    let author = match &native {
        Native::Atomic(q) => native_author!(q),
        Native::Rank(q) | Native::Find(q) => native_author!(q),
        Native::DynamicChoose(q) => native_author!(q),
        Native::Recognize(q) => native_author!(q),
        Native::Relate(q) => native_author!(q),
        Native::Set(_) | Native::RankSet(_) => Author::default(),
    };
    QuestionHandle {
        json,
        native,
        descriptor: None,
        reading,
        author: Box::new(AuthorOwner::new(author)),
    }
}
/// Explicit native loader role; no host filesystem/name resolver is involved.
pub(crate) fn named(role: u32, value: &str, reference: bool) -> Result<QuestionHandle, Failure> {
    macro_rules! load {
        ($ty:ty) => {
            if reference {
                <$ty>::load_reference(value)?
            } else {
                <$ty>::load_named(value)?
            }
        };
    }
    let mut reading = None;
    let native = match role {
        abi::LOAD_ATOMIC_V1 => Native::Atomic(load!(Question)),
        abi::LOAD_SET_V1 => Native::Set(load!(QuestionSet)),
        abi::LOAD_DYNAMIC_CHOOSE_V1 => Native::DynamicChoose(load!(RecordChooseQuestion)),
        abi::LOAD_RECOGNIZE_V1 => {
            let (q, selected) = load!(thinkthen::RecognizeQuestionFile).into_parts();
            reading = Some(selected);
            Native::Recognize(q)
        }
        abi::LOAD_RELATE_V1 => Native::Relate(if reference {
            Relate::load_records_reference(value)?
        } else {
            Relate::load_records_named(value)?
        }),
        abi::LOAD_RANK_V1 => Native::Rank(if reference {
            Question::load_rank_reference(value)?
        } else {
            Question::load_rank_named(value)?
        }),
        abi::LOAD_RANK_SET_V1 => Native::RankSet(load!(RankSet)),
        abi::LOAD_FIND_V1 => {
            let (q, selected) = load!(thinkthen::FindQuestionFile).into_parts();
            reading = Some(selected);
            Native::Find(q)
        }
        _ => return Err(Failure::usage("invalid native named question role")),
    };
    let json = match &native {
        Native::Atomic(q) => q.to_json()?,
        Native::Rank(q) => q.to_json()?,
        _ => String::new(),
    };
    Ok(finish(json, native, reading))
}

pub(crate) fn load(path: &str) -> Result<QuestionHandle, Failure> {
    let json = thinkthen::read_question_file(path)
        .map_err(|_| Failure::local("the question file could not supply bounded UTF-8 content"))?;
    let body: std::collections::BTreeMap<String, Box<RawValue>> = serde_json::from_str(&json)
        .map_err(|_| Failure::local("the question file has invalid question content"))?;
    let kind = if body.contains_key("questions") {
        ANNOTATE
    } else if body.contains_key("recognize") {
        RECOGNIZE
    } else if body.contains_key("relate") {
        RELATE
    } else if body.contains_key("find") {
        FIND
    } else {
        DECIDE
    };
    if kind == DECIDE && (!body.contains_key("choose") || body.contains_key("options")) {
        return Ok(finish(json, Native::Atomic(Question::load(path)?), None));
    }
    let parsed = if body.contains_key("choose") && !body.contains_key("options") {
        dynamic(json)
    } else {
        parse(kind, json)
    };
    parsed.map_err(|_| Failure::local("the question file has invalid question content"))
}
impl QuestionHandle {
    pub(crate) fn fits_complete(&self, kind: u32) -> bool {
        self.fits(kind)
            || kind == RANK
                && matches!(&self.native,
            Native::Atomic(LoadedQuestion::Question(q)) if matches!(q.kind(), thinkthen::QuestionKind::Decide | thinkthen::QuestionKind::Score))
    }

    pub(crate) fn rank_reading(&self) -> Result<Question, Failure> {
        if self.json.is_empty() {
            return Err(Failure::usage(
                "native named rank conversion awaits authored rank API",
            ));
        }
        Ok(Question::rank_from_json(&self.json)?)
    }

    pub(crate) fn fits(&self, kind: u32) -> bool {
        match (&self.native, kind) {
            (Native::Atomic(question), DECIDE | CHOOSE | TAG | SCORE | FILTER) => {
                use thinkthen::QuestionKind as K;
                matches!(
                    (question.kind(), kind),
                    (K::Decide, DECIDE | FILTER)
                        | (K::Choose, CHOOSE)
                        | (K::Tag, TAG)
                        | (K::Score, SCORE)
                )
            }
            (Native::DynamicChoose(_), CHOOSE) => true,
            (Native::Rank(_) | Native::RankSet(_), RANK)
            | (Native::Find(_), FIND)
            | (Native::Set(_), RANK | ANNOTATE)
            | (Native::Recognize(_), RECOGNIZE)
            | (Native::Relate(_), RELATE) => true,
            _ => false,
        }
    }
}

/// Import an explicit native saved grammar role, without host shape detection.
pub(crate) fn parse_role(role: u32, json: &str) -> Result<QuestionHandle, Failure> {
    let mut reading = None;
    let native = match role {
        abi::LOAD_ATOMIC_V1 => Native::Atomic(Question::from_json(json)?),
        abi::LOAD_SET_V1 => Native::Set(QuestionSet::from_json(json)?),
        abi::LOAD_DYNAMIC_CHOOSE_V1 => {
            Native::DynamicChoose(RecordChooseQuestion::from_json(json)?)
        }
        abi::LOAD_RECOGNIZE_V1 => {
            let (q, selected) = thinkthen::RecognizeQuestionFile::from_json(json)?.into_parts();
            reading = Some(selected);
            Native::Recognize(q)
        }
        abi::LOAD_RELATE_V1 => Native::Relate(Relate::from_records_json(json)?),
        abi::LOAD_RANK_V1 => Native::Rank(Question::rank_from_json(json)?),
        abi::LOAD_RANK_SET_V1 => Native::RankSet(RankSet::from_json(json)?),
        abi::LOAD_FIND_V1 => {
            let (q, selected) = thinkthen::FindQuestionFile::from_json(json)?.into_parts();
            reading = Some(selected);
            Native::Find(q)
        }
        _ => return Err(Failure::usage("invalid native question grammar role")),
    };
    Ok(finish(json.to_owned(), native, reading))
}
