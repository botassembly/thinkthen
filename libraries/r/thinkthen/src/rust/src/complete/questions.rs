//! Select existing native grammar and loader roles at the host edge.
use super::usage;
use serde::Deserialize;
use serde_json::value::RawValue;
use thinkthen::{
    Error, FindQuestionFile, LoadedQuestion, QuestionSet, RankSet, RecognizeQuestionFile,
    RecordChooseQuestion, RecordReading, Relate,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Question {
    pub(crate) role: String,
    #[serde(default)]
    pub(crate) none: bool,
    pub(crate) body: Option<Box<RawValue>>,
    pub(crate) raw: Option<String>,
    pub(crate) path: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) reference: Option<String>,
}
pub(crate) enum Asked {
    Atomic(LoadedQuestion),
    Dynamic(RecordChooseQuestion),
    Rank(thinkthen::Question),
    SetRank(RankSet),
    Find(thinkthen::Question, RecordReading),
    Set(QuestionSet),
    Recognize(thinkthen::Recognize, RecordReading),
    Relate(Relate),
}
impl Asked {
    pub(crate) fn reading(&self) -> Option<&RecordReading> {
        match self {
            Self::Find(_, r) | Self::Recognize(_, r) => Some(r),
            _ => None,
        }
    }
}
pub(crate) fn load(verb: &str, q: Question) -> Result<Asked, Error> {
    validate(&q)?;
    let body = q.body.as_deref().map(RawValue::get).or(q.raw.as_deref());
    macro_rules! read {
        ($ty:ty) => {
            if let Some(body) = body {
                <$ty>::from_json(body)?
            } else if let Some(path) = &q.path {
                <$ty>::load(path)?
            } else if let Some(name) = &q.name {
                <$ty>::load_named(name)?
            } else {
                <$ty>::load_reference(q.reference.as_deref().unwrap_or_default())?
            }
        };
    }
    Ok(match q.role.as_str() {
        "atomic" => Asked::Atomic(read!(thinkthen::Question)),
        "dynamic" => Asked::Dynamic(read!(RecordChooseQuestion)),
        "set" if verb == "rank" => Asked::SetRank(read!(RankSet)),
        "set" => Asked::Set(read!(QuestionSet)),
        "find" => find(read!(FindQuestionFile), q.none)?,
        "recognize" => {
            let (asked, reading) = read!(RecognizeQuestionFile).into_parts();
            Asked::Recognize(asked, reading)
        }
        "relate" => Asked::Relate(relate(&q, body)?),
        "rank" => Asked::Rank(rank(&q, body)?),
        _ => return Err(usage("invalid question grammar role")),
    })
}

fn relate(q: &Question, body: Option<&str>) -> Result<Relate, Error> {
    Ok(if let Some(body) = body {
        Relate::from_records_json(body)?
    } else if let Some(path) = &q.path {
        Relate::load_records(path)?
    } else if let Some(name) = &q.name {
        Relate::load_records_named(name)?
    } else {
        Relate::load_records_reference(q.reference.as_deref().unwrap_or_default())?
    })
}
fn rank(q: &Question, body: Option<&str>) -> Result<thinkthen::Question, Error> {
    Ok(if let Some(body) = body {
        thinkthen::Question::rank_from_json(body)?
    } else if let Some(path) = &q.path {
        thinkthen::Question::load_rank(path)?
    } else if let Some(name) = &q.name {
        thinkthen::Question::load_rank_named(name)?
    } else {
        thinkthen::Question::load_rank_reference(q.reference.as_deref().unwrap_or_default())?
    })
}

fn validate(q: &Question) -> Result<(), Error> {
    if usize::from(q.body.is_some())
        + usize::from(q.raw.is_some())
        + usize::from(q.path.is_some())
        + usize::from(q.name.is_some())
        + usize::from(q.reference.is_some())
        != 1
    {
        return Err(usage(
            "give exactly one question body, file, name or reference",
        ));
    }
    Ok(())
}

fn find(q: FindQuestionFile, none: bool) -> Result<Asked, Error> {
    let (asked, reading) = q.into_parts();
    Ok(Asked::Find(
        if none { asked.offering_none()? } else { asked },
        reading,
    ))
}
