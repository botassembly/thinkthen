//! Plain refusals for pre-0.1 forms replaced by named and keyed calls.

use pgrx::datum::{Array, JsonB};
use pgrx::prelude::*;
use thinkthen::Error;

use crate::call;

fn context_moved() -> Error {
    call::usage(
        "the context argument moved into the settings object or the context named parameter",
    )
}

#[pg_extern(name = "thinkthen_warm", parallel_restricted)]
fn warm(_question: Option<&str>, _input: Option<&str>) -> i64 {
    call::guarded(|| {
        call::raise(call::usage(
            "thinkthen_warm was removed; pack records with thinkthen_decide_many",
        ))
    })
}

#[pg_extern(name = "thinkthen_warm", parallel_restricted)]
fn warm_context(_question: Option<&str>, _input: Option<&str>, _context: Option<&str>) -> i64 {
    call::guarded(|| {
        call::raise(call::usage(
            "thinkthen_warm was removed; pack records with thinkthen_decide_many",
        ))
    })
}

#[pg_extern(name = "thinkthen_probability", parallel_restricted)]
fn probability(_question: Option<&str>, _input: Option<&str>) -> Option<f64> {
    call::guarded(|| {
        call::raise(call::usage(
            "thinkthen_probability was removed; order records with thinkthen_rank",
        ))
    })
}

#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn decide_array(_question: Option<&str>, _inputs: Option<Array<'_, &str>>) -> i64 {
    call::guarded(|| {
        call::raise(call::usage(
            "the array form was removed; pass a keyed jsonb object to thinkthen_decide_many",
        ))
    })
}

#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn decide_array_context(
    _question: Option<&str>,
    _inputs: Option<Array<'_, &str>>,
    _context: Option<&str>,
) -> i64 {
    call::guarded(|| {
        call::raise(call::usage(
            "the array form was removed; pass a keyed jsonb object to thinkthen_decide_many",
        ))
    })
}

#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn decide_context(
    _question: Option<&str>,
    _input: Option<&str>,
    _context: Option<&str>,
) -> Option<bool> {
    call::guarded(|| call::raise(context_moved()))
}

#[pg_extern(name = "thinkthen_choose", parallel_restricted)]
fn choose_context(
    _question: Option<&str>,
    _input: Option<&str>,
    _members: Option<Array<'_, &str>>,
    _context: Option<&str>,
) -> Option<String> {
    call::guarded(|| call::raise(context_moved()))
}

#[pg_extern(name = "thinkthen_score", parallel_restricted)]
fn score_context(
    _question: Option<&str>,
    _input: Option<&str>,
    _members: Option<Array<'_, &str>>,
    _context: Option<&str>,
) -> Option<f64> {
    call::guarded(|| call::raise(context_moved()))
}

#[pg_extern(name = "thinkthen_tag", parallel_restricted)]
fn tag_context(
    _question: Option<&str>,
    _input: Option<&str>,
    _members: Option<Array<'_, &str>>,
    _context: Option<&str>,
) -> Option<Vec<String>> {
    call::guarded(|| call::raise(context_moved()))
}

#[pg_extern(name = "thinkthen_details", parallel_restricted)]
fn details_context(
    _question: Option<&str>,
    _input: Option<&str>,
    _context: Option<&str>,
) -> Option<JsonB> {
    call::guarded(|| call::raise(context_moved()))
}

#[pg_extern(name = "thinkthen_try_details", parallel_restricted)]
fn try_details_context(
    _question: Option<&str>,
    _input: Option<&str>,
    _context: Option<&str>,
) -> Option<JsonB> {
    call::guarded(|| call::raise(context_moved()))
}

#[pg_extern(name = "thinkthen_probability", parallel_restricted)]
fn probability_context(
    _question: Option<&str>,
    _input: Option<&str>,
    _context: Option<&str>,
) -> Option<f64> {
    call::guarded(|| {
        call::raise(call::usage(
            "thinkthen_probability was removed; order records with thinkthen_rank",
        ))
    })
}
