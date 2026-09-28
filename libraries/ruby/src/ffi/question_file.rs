//! The bounded local-file edge for one public Ruby question value.

use std::io::Read;

use thinkthen::{ErrorKind, LoadedQuestion, Question, Recognize, Relate};

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
    const LIMIT: u64 = 1_048_576;
    let unreadable = || {
        Fault::of(
            ErrorKind::Local,
            format!("the {role} file could not be read"),
        )
    };
    let file = std::fs::File::open(path).map_err(|_| unreadable())?;
    let mut bytes = Vec::new();
    file.take(LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| unreadable())?;
    if bytes.len() as u64 > LIMIT {
        return Err(Fault::of(
            ErrorKind::Local,
            format!("the {role} file is too large"),
        ));
    }
    String::from_utf8(bytes)
        .map_err(|_| Fault::of(ErrorKind::Local, format!("the {role} file is not UTF-8")))
}
