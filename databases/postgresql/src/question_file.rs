//! Explicit privileged loader; authored JSON never passes through jsonb.

use pgrx::prelude::*;
use thinkthen::{Error, ErrorKind};

use crate::{call, files};
use call::OrRaise as _;

#[pg_extern(parallel_restricted)]
fn thinkthen_question_file(path: Option<&str>) -> Option<String> {
    call::guarded(|| {
        let path = path?;
        if path.is_empty() || path.contains('\0') {
            call::raise(call::usage(
                "question file path must be nonempty text without NUL",
            ));
        }
        let source =
            files::read_named("question", path, call::file_directory().as_deref()).or_raise();
        // Both grammars are owned by native Rust. Keep the original bytes for
        // the eventual consumer, including whitespace and authored ordering.
        if !source.trim_start().starts_with('{')
            || [
                "decide",
                "choose",
                "tag",
                "score",
                "filter",
                "rank",
                "find",
                "annotate",
                "recognize",
                "relate",
            ]
            .iter()
            .all(|verb| crate::complete_native::Prepared::parse(verb, &source).is_err())
        {
            call::raise(Error::new(
                ErrorKind::Local,
                "the question file does not parse as a question or question set",
            ));
        }
        Some(source)
    })
}
