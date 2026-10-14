//! Typed file rows share the native Source selector and existing SQL worker.
use crate::tables::Table;
use crate::{Failure, named};
use rusqlite::ffi::sqlite3;
use rusqlite::types::Value;
use std::borrow::Cow;
use std::ffi::CStr;

#[derive(Debug)]
pub(crate) struct Filter;
impl Table for Filter {
    const NAME: &'static str = "thinkthen_filter";
    const SCHEMA: &'static CStr = c"CREATE TABLE x(file TEXT, first_line INTEGER, last_line INTEGER, ordinal INTEGER, value INTEGER, probability REAL, answer_id TEXT, call_id TEXT, requests_sent INTEGER, cache_answers INTEGER, records INTEGER, input_tokens INTEGER, output_tokens INTEGER, seconds REAL, question HIDDEN, paths HIDDEN, unit HIDDEN, window HIDDEN, media HIDDEN, format HIDDEN, files_only HIDDEN, fields HIDDEN, context_field HIDDEN, settings HIDDEN)";
    const FIRST_HIDDEN: usize = 14;
    const COLUMNS: usize = Self::FIRST_HIDDEN + 10 + named::COUNT;
    const REQUIRED: usize = 2;
    fn schema() -> Result<Cow<'static, CStr>, Failure> {
        named::schema(Self::SCHEMA)
    }
    fn rows(_: *mut sqlite3, _: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
        Err(Failure::defect("a source table lost its retained receipt"))
    }
}
