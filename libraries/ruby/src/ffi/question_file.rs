//! The bounded local-file edge for one public Ruby question value.

use std::io::Read;

use thinkthen::{ErrorKind, LoadedQuestion, Question};

use crate::Fault;

pub(super) fn read(path: &str) -> Result<(String, LoadedQuestion), Fault> {
    const LIMIT: u64 = 1_048_576;
    let file = std::fs::File::open(path)
        .map_err(|_| Fault::of(ErrorKind::Local, "the question file could not be read"))?;
    let mut bytes = Vec::new();
    file.take(LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Fault::of(ErrorKind::Local, "the question file could not be read"))?;
    if bytes.len() as u64 > LIMIT {
        return Err(Fault::of(
            ErrorKind::Local,
            "the question file is too large",
        ));
    }
    let json = String::from_utf8(bytes)
        .map_err(|_| Fault::of(ErrorKind::Local, "the question file is not UTF-8"))?;
    let loaded = Question::from_json(&json)
        .map_err(|error| Fault::of(ErrorKind::Local, error.detail().message()))?;
    Ok((json, loaded))
}
