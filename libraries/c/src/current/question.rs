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
        10 => Native::Relate(Relate::from_records_json(&json)?),
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
        1 => Native::Atomic(load!(Question)),
        2 => Native::Set(load!(QuestionSet)),
        3 => Native::DynamicChoose(load!(RecordChooseQuestion)),
        4 => {
            let (q, selected) = load!(thinkthen::RecognizeQuestionFile).into_parts();
            reading = Some(selected);
            Native::Recognize(q)
        }
        5 => Native::Relate(if reference {
            Relate::load_records_reference(value)?
        } else {
            Relate::load_records_named(value)?
        }),
        6 => Native::Rank(if reference {
            Question::load_rank_reference(value)?
        } else {
            Question::load_rank_named(value)?
        }),
        7 => Native::RankSet(load!(RankSet)),
        8 => {
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
    pub(crate) fn fits_complete(&self, kind: u32) -> bool {
        self.fits(kind)
            || kind == 6
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

/// Import an explicit native saved grammar role, without host shape detection.
pub(crate) fn parse_role(role: u32, json: &str) -> Result<QuestionHandle, Failure> {
    let mut reading = None;
    let native = match role {
        1 => Native::Atomic(Question::from_json(json)?),
        2 => Native::Set(QuestionSet::from_json(json)?),
        3 => Native::DynamicChoose(RecordChooseQuestion::from_json(json)?),
        4 => {
            let (q, selected) = thinkthen::RecognizeQuestionFile::from_json(json)?.into_parts();
            reading = Some(selected);
            Native::Recognize(q)
        }
        5 => Native::Relate(Relate::from_records_json(json)?),
        6 => Native::Rank(Question::rank_from_json(json)?),
        7 => Native::RankSet(RankSet::from_json(json)?),
        8 => {
            let (q, selected) = thinkthen::FindQuestionFile::from_json(json)?.into_parts();
            reading = Some(selected);
            Native::Find(q)
        }
        _ => return Err(Failure::usage("invalid native question grammar role")),
    };
    Ok(finish(json.to_owned(), native, reading))
}
