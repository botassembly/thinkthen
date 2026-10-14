//! Typed file rows share the native Source selector and existing SQL worker.
use crate::tables::Table;
use crate::{Failure, complete_native, named, question, worker};
use rusqlite::ffi::sqlite3;
use rusqlite::types::{Value, ValueRef};
use serde_json::json;
use std::borrow::Cow;
use std::ffi::CStr;
use thinkthen::{RequestEnvironment, RequestOptions, RequestOutcome, RequestValue};

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
    fn rows(db: *mut sqlite3, args: &[Value]) -> Result<Vec<Vec<Value>>, Failure> {
        let question = text(args, 0, "the question")?
            .ok_or_else(|| Failure::usage("the question is required"))?;
        let paths = text(args, 1, "paths")?.ok_or_else(|| Failure::usage("paths is required"))?;
        let paths: Vec<String> = serde_json::from_str(&paths)
            .map_err(|_| Failure::usage("file paths is a text array"))?;
        let mut reading = serde_json::Map::new();
        if let Some(unit) = text(args, 2, "unit")? {
            reading.insert("unit".into(), json!(unit));
        }
        if let Some(window) = integer(args, 3, "window")? {
            reading.insert("window".into(), json!(window));
        }
        let mut reader = serde_json::Map::new();
        reader.insert("reading".into(), json!(reading));
        if let Some(media) = text(args, 4, "media")? {
            reader.insert("media".into(), json!(media));
        }
        let mut files = serde_json::Map::new();
        files.insert("paths".into(), json!(paths));
        files.insert("options".into(), json!(reader));
        if let Some(format) = text(args, 5, "format")? {
            files.insert("format".into(), json!(format));
        }
        let inputs = complete_native::Inputs::parse_request(&json!({"files":files}).to_string())?;
        let source = inputs
            .request_source()?
            .ok_or_else(|| Failure::defect("file selection lost its source"))?;
        let options = RequestOptions {
            files_only: match integer(args, 6, "files_only")? {
                None | Some(0) => false,
                Some(1) => true,
                _ => return Err(Failure::usage("files_only is the SQL integer 0 or 1")),
            },
            field: text(args, 7, "fields")?
                .map(|fields| {
                    serde_json::from_str(&fields)
                        .map_err(|_| Failure::usage("fields is a text array"))
                })
                .transpose()?,
            context_field: text(args, 8, "context_field")?,
            ..RequestOptions::default()
        };
        let settings = text(args, 9, "settings")?.unwrap_or_else(|| "{}".into());
        let values = args.iter().skip(10).map(ValueRef::from).collect::<Vec<_>>();
        let settings = named::settings(settings.as_bytes(), &values)?;
        let settings = question::call_settings(ValueRef::Text(&settings))?;
        let prepared = crate::complete::prepare("filter", &question, &settings)?;
        let request =
            thinkthen_host::sql_request::request_source(&prepared, source, options)?.admit()?;
        let context = settings.context().map(str::to_owned);
        worker::run_settings(db, settings, move |engine, controls| {
            let controls = context
                .as_deref()
                .map_or(controls, |context| controls.context(context));
            let call = match engine.execute_request(
                &request,
                RequestEnvironment {
                    controls: controls.surface(thinkthen::Surface::Sqlite),
                    feed: None,
                },
            )? {
                RequestOutcome::Complete(call) => call,
                RequestOutcome::Failed { error, .. } => return Err(error.into()),
            };
            let RequestValue::Filtered(rows) = call.value() else {
                return Err(crate::request::wrong_result().into());
            };
            let facts = call.facts();
            rows.iter().map(|row| columns(row, facts)).collect()
        })
    }
}

fn columns(
    row: &thinkthen::CompleteRecord<thinkthen::QuestionInput, thinkthen::CompleteFilter>,
    facts: &thinkthen::Facts,
) -> Result<Vec<Value>, Failure> {
    let thinkthen::QuestionInput::Record(original) = row.original() else {
        return Err(Failure::defect("a filtered source lost its original"));
    };
    let source = original
        .location()
        .ok_or_else(|| Failure::defect("a filtered source lost its location"))?;
    let probability = match row.result().probabilities() {
        thinkthen::Probabilities::YesNo { yes } => yes,
        _ => return Err(crate::request::wrong_result().into()),
    };
    Ok(vec![
        Value::Text(source.file().to_owned()),
        number(source.first_line().map(|v| v as u64))?,
        number(source.last_line().map(|v| v as u64))?,
        number(Some(row.ordinal() as u64))?,
        Value::Integer(i64::from(row.result().value())),
        Value::Real(probability),
        Value::Text(row.result().answer_id().to_string()),
        facts
            .call_id()
            .map_or(Value::Null, |id| Value::Text(id.to_string())),
        number(Some(facts.requests_sent()))?,
        number(Some(facts.cache_answers()))?,
        number(Some(facts.records()))?,
        number(facts.input_tokens())?,
        number(facts.output_tokens())?,
        Value::Real(facts.seconds()),
    ])
}

fn text(args: &[Value], at: usize, name: &str) -> Result<Option<String>, Failure> {
    question::text(args.get(at).map_or(ValueRef::Null, ValueRef::from), name)
}
fn integer(args: &[Value], at: usize, name: &str) -> Result<Option<i64>, Failure> {
    match args.get(at) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Integer(v)) => Ok(Some(*v)),
        _ => Err(Failure::usage(format!("{name} is a SQL integer"))),
    }
}
fn number(value: Option<u64>) -> Result<Value, Failure> {
    value
        .map(|value| {
            i64::try_from(value)
                .map(Value::Integer)
                .map_err(|_| Failure::usage("a native count exceeds SQL INTEGER"))
        })
        .transpose()
        .map(|value| value.unwrap_or(Value::Null))
}
