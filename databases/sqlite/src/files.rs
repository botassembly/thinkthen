//! Explicit caller-filesystem selection and native physical span mapping.

use rusqlite::Connection;
use rusqlite::functions::FunctionFlags;
use rusqlite::types::{Value, ValueRef};
use thinkthen::{ReaderOptions, SourceRecord, SourceRecords, read_files};

use crate::{Failure, guard, question::text};

pub(crate) fn selection(path: &Value, options: &Value) -> Result<SourceRecords, Failure> {
    let options = match text(ValueRef::from(options), "reader options")? {
        Some(json) => serde_json::from_str::<ReaderOptions>(&json)
            .map_err(|_| Failure::usage("reader options must be a unit/window JSON object"))?,
        None => ReaderOptions::default(),
    };
    options.validate()?;
    let path = text(ValueRef::from(path), "source path")?
        .ok_or_else(|| Failure::usage("source path must be text or a JSON array of paths"))?;
    if path.len() > 16 * 1024 * 1024 {
        return Err(Failure::usage("source operands exceed 16 MiB"));
    }
    let paths = if path.starts_with('[') {
        serde_json::from_str::<Vec<String>>(&path)
            .map_err(|_| Failure::usage("source paths must be a JSON array of text"))?
    } else {
        vec![path]
    };
    if paths.is_empty()
        || paths
            .iter()
            .any(|path| path.is_empty() || path.contains('\0'))
    {
        return Err(Failure::usage(
            "source paths must be nonempty file or folder names",
        ));
    }
    Ok(read_files(paths, options)?)
}

pub(crate) fn values(ordinal: i64, record: SourceRecord<String>) -> Result<Vec<Value>, Failure> {
    let integer = |value| {
        i64::try_from(value)
            .map(Value::Integer)
            .map_err(|_| Failure::defect("source position is too large"))
    };
    Ok(vec![
        Value::Integer(ordinal),
        Value::Text(record.record),
        Value::Text(record.file),
        integer(record.first_line)?,
        integer(record.last_line)?,
    ])
}

fn span(context: &rusqlite::functions::Context<'_>) -> Result<String, Failure> {
    let record = context
        .get::<String>(0)
        .map_err(|_| Failure::usage("span record must be text"))?;
    let offset = |at| {
        context
            .get::<i64>(at)
            .ok()
            .and_then(|v| usize::try_from(v).ok())
            .ok_or_else(|| Failure::usage("span positions must be nonnegative whole numbers"))
    };
    let first_line = offset(1)?;
    if first_line == 0 {
        return Err(Failure::usage("source first_line must be positive"));
    }
    let source = SourceRecord {
        record,
        file: String::new(),
        first_line,
        last_line: first_line,
    };
    let (first, last) = source.span_lines(offset(2)?, offset(3)?)?;
    let sql_line =
        |line| i64::try_from(line).map_err(|_| Failure::usage("source line exceeds SQL INTEGER"));
    Ok(
        serde_json::json!({"first_line": sql_line(first)?, "last_line": sql_line(last)?})
            .to_string(),
    )
}

pub(crate) fn register_span(connection: &Connection) -> rusqlite::Result<()> {
    connection.create_scalar_function(
        "thinkthen_span_lines",
        4,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DIRECTONLY,
        |context| Ok(guard("thinkthen_span_lines", || span(context))?),
    )
}
