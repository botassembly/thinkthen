//! Validate explicit question grammar through native parsers.
use super::{
    QuestionHandle,
    author::{Author, AuthorOwner, native_author as metadata},
};
use crate::failures::Failure;
use crate::ffi::values as abi;
use thinkthen::{Question, QuestionSet, RankSet, RecordChooseQuestion, Relate};

/// Import an explicit native saved grammar role, without host shape detection.
pub(crate) fn parse_role(role: u32, json: &str) -> Result<QuestionHandle, Failure> {
    let author = match role {
        abi::LOAD_ATOMIC_V1 => metadata!(Question::from_json(json)?),
        abi::LOAD_SET_V1 => {
            QuestionSet::from_json(json)?;
            Author::default()
        }
        abi::LOAD_DYNAMIC_CHOOSE_V1 => {
            metadata!(RecordChooseQuestion::from_json(json)?)
        }
        abi::LOAD_RECOGNIZE_V1 => {
            let (q, _) = thinkthen::RecognizeQuestionFile::from_json(json)?.into_parts();
            metadata!(q)
        }
        abi::LOAD_RELATE_V1 => metadata!(Relate::from_records_json(json)?),
        abi::LOAD_RANK_V1 => metadata!(Question::rank_from_json(json)?),
        abi::LOAD_RANK_SET_V1 => {
            RankSet::from_json(json)?;
            Author::default()
        }
        abi::LOAD_FIND_V1 => {
            let (q, _) = thinkthen::FindQuestionFile::from_json(json)?.into_parts();
            metadata!(q)
        }
        _ => return Err(Failure::usage("invalid native question grammar role")),
    };
    Ok(QuestionHandle {
        author: Box::new(AuthorOwner::new(author)),
    })
}
