//! The calibration profile saved beside one question.

use crate::core::backend_profile::ProfileName;
use crate::core::json::Json;
use crate::core::question_file::QuestionFileError;

pub(super) fn profile_in(value: &Json) -> Result<Option<ProfileName>, QuestionFileError> {
    let Some(held) = value.member("profile") else {
        return Ok(None);
    };
    let name = held.as_str().ok_or(QuestionFileError::Shape {
        key: "profile",
        wanted: "is a safe profile name",
    })?;
    ProfileName::new(name)
        .map(Some)
        .map_err(|_| QuestionFileError::Shape {
            key: "profile",
            wanted: "is a safe profile name",
        })
}
