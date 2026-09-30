//! The bounded local-file edge for one public Ruby question value.

use thinkthen::{ErrorKind, LoadedQuestion, Question, QuestionFileError, Recognize, Relate};

use crate::Fault;

pub(super) fn read(path: &str) -> Result<(String, LoadedQuestion), Fault> {
    let json = bounded_source(path, "question")?;
    let loaded = Question::from_json(&json).map_err(|_| {
        Fault::of(
            ErrorKind::Local,
            "the question file has invalid question content",
        )
    })?;
    Ok((json, loaded))
}

pub(super) fn read_plan(path: &str, verb: &str) -> Result<String, Fault> {
    let json = bounded_source(path, "plan")?;
    match verb {
        "recognize" => {
            Recognize::from_json(&json).map_err(|_| {
                Fault::of(ErrorKind::Local, "the plan file has invalid plan content")
            })?;
        }
        "relate" => {
            Relate::from_json(&json).map_err(|_| {
                Fault::of(ErrorKind::Local, "the plan file has invalid plan content")
            })?;
        }
        _ => {
            return Err(Fault::of(
                ErrorKind::Usage,
                "the plan file needs recognize or relate",
            ));
        }
    }
    Ok(json)
}

fn bounded_source(path: &str, role: &str) -> Result<String, Fault> {
    thinkthen::read_question_file(path).map_err(|reason| {
        Fault::of(
            ErrorKind::Local,
            match reason {
                QuestionFileError::Unreadable(_) => format!("the {role} file could not be read"),
                QuestionFileError::TooLarge => format!("the {role} file is too large"),
                QuestionFileError::NotUtf8 => format!("the {role} file is not UTF-8"),
            },
        )
    })
}
