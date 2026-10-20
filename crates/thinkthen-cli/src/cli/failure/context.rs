//! Fixed, evidence-free diagnostics for a shared context.

use std::io;

use crate::failure::Failure;

#[derive(Debug)]
pub(crate) enum Error {
    Open(io::Error),
    NotUtf8,
    Empty,
    StructuredQuestion,
    OverLimit {
        initial: bool,
        dimension: &'static str,
        limit: usize,
        actual: usize,
        profile: Option<String>,
    },
}

pub(super) fn message(failure: &Failure) -> Option<(u8, String)> {
    let Failure::Context(error) = failure else {
        return None;
    };
    Some(match error {
        Error::Open(error) => (5, format!("--context could not be opened: {error}")),
        Error::NotUtf8 => (5, "--context is not UTF-8 text".to_owned()),
        Error::Empty => (2, "--context names an empty file".to_owned()),
        Error::StructuredQuestion => (2, "--context needs a question written as text; a question written as JSON cannot quote a record".to_owned()),
        Error::OverLimit { initial, dimension, limit, actual, profile } => {
            let message = if let Some(profile) = profile {
                if *initial {
                    format!("--context: profile {} allows at most {limit} {}; the context's request has {actual}", profile, dimension)
                } else {
                    format!("--context: profile {} allows at most {limit} {}; this record and the context make {actual}", profile, dimension)
                }
            } else if *initial {
                format!("--context: the context and the question make a request of {actual} bytes, over the request size of {limit} bytes; raise --max-request-bytes or shorten the context")
            } else {
                format!("--context: this record and the context make a request of {actual} bytes, over the request size of {limit} bytes; raise --max-request-bytes, or shorten the context or the record")
            };
            (2, message)
        }
    })
}
