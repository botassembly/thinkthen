//! Plain refusals for pre-0.1 forms replaced by named and keyed calls.

use pgrx::datum::Array;
use pgrx::prelude::*;

use crate::call::{self, Refusal};

#[pg_extern(name = "thinkthen_warm", parallel_restricted)]
fn warm(_question: Option<&str>, _input: Option<&str>) -> i64 {
    call::raise(Refusal::usage(
        "thinkthen_warm was removed; pack records with thinkthen_decide_many",
    ))
}

#[pg_extern(name = "thinkthen_warm", parallel_restricted)]
fn warm_context(_question: Option<&str>, _input: Option<&str>, _context: Option<&str>) -> i64 {
    call::raise(Refusal::usage(
        "thinkthen_warm was removed; pack records with thinkthen_decide_many",
    ))
}

#[pg_extern(name = "thinkthen_probability", parallel_restricted)]
fn probability(_question: Option<&str>, _input: Option<&str>) -> Option<f64> {
    call::raise(Refusal::usage(
        "thinkthen_probability was removed; read the probability column of thinkthen_decide_many",
    ))
}

#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn decide_array(_question: Option<&str>, _inputs: Option<Array<'_, &str>>) -> i64 {
    call::raise(Refusal::usage(
        "the array form was removed; pass a keyed jsonb object to thinkthen_decide_many",
    ))
}

#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn decide_array_context(
    _question: Option<&str>,
    _inputs: Option<Array<'_, &str>>,
    _context: Option<&str>,
) -> i64 {
    call::raise(Refusal::usage(
        "the array form was removed; pass a keyed jsonb object to thinkthen_decide_many",
    ))
}
