//! All projections belong to one canonical admitted Request and one call.
use super::{Arguments, input};
use crate::receipt::Receipt;
use crate::{Failure, complete_native, worker};
use rusqlite::ffi::sqlite3;
use serde_json::{Value as Json, json};
use thinkthen::{Error, ErrorKind, Surface};
use thinkthen_host::sql_request;

pub(super) fn root(verb: &str) -> &str {
    match verb {
        "decide" => "decision",
        "choose" => "choice",
        "tag" => "tags",
        "score" => "score",
        "filter" => "filtered",
        "rank" | "rank_set" => "ranked",
        "find" => "found",
        "annotate" => "annotation",
        "recognize" => "recognized",
        "relate" => "related",
        _ => "error",
    }
}

pub(super) fn execute(db: *mut sqlite3, invocation: Invocation) -> Result<Receipt, Failure> {
    let Invocation {
        verb,
        prepared,
        request,
        inputs,
        settings,
        keys,
        ids,
        details,
        recover,
        ..
    } = invocation;
    let shared = settings.context().map(str::to_owned);
    let carrier = worker::run_settings(db, settings, move |engine, controls| {
        let controls = shared
            .as_deref()
            .map_or(controls, |context| controls.context(context));
        if details && recover {
            return sql_request::run_recovering(
                engine,
                &prepared,
                &request,
                inputs,
                controls,
                Surface::Sqlite,
            );
        }
        Ok(sql_request::run(
            engine,
            &prepared,
            &request,
            inputs,
            controls,
            Surface::Sqlite,
        ))
    })?;
    let carrier = if details {
        details_carrier(verb, carrier)?
    } else {
        carrier
    };
    project(verb, carrier, keys, ids)
}

pub(super) fn admission(error: &Error) -> Result<Receipt, Failure> {
    project(
        "error",
        complete_native::admission(error),
        Vec::new(),
        std::collections::HashMap::new(),
    )
}

fn project(
    verb: &str,
    mut carrier: Json,
    keys: Vec<String>,
    ids: crate::tables::Ids,
) -> Result<Receipt, Failure> {
    let mut receipt = Receipt::default();
    let native = carrier
        .get("native")
        .ok_or_else(|| Failure::defect("a native call lost its receipt"))?;
    if let Some(facts) = native.get("facts") {
        receipt.add("facts", facts)?;
    }
    if let Some(error) = native.get("error") {
        receipt.add("error", error)?;
    }
    let call_facts = carrier.pointer("/native/facts").cloned();
    if let Some(rows) = carrier.get_mut("rows").and_then(Json::as_array_mut) {
        for row in rows {
            for (field, kind) in [
                ("result", root(verb)),
                ("failure", "error"),
                ("facts", "facts"),
            ] {
                if let Some(value) = row.get(field).filter(|value| !value.is_null()).cloned() {
                    let index = if field == "facts" && call_facts.as_ref() == Some(&value) {
                        0
                    } else {
                        receipt.add(kind, &value)?
                    };
                    if field == "result" {
                        receipt.add("record_order", &json!({"ordinal":row.get("ordinal"),"result_index":index,"completed":false}))?;
                    }
                    row[field] = json!(index);
                }
            }
            if let Some(events) = row.get("observations").and_then(Json::as_array).cloned() {
                let mut first = None;
                for event in events {
                    let index = receipt.add("event", &event)?;
                    first.get_or_insert(index);
                }
                row["observations"] = json!(first);
            }
        }
    }
    let completed = carrier.get("completed");
    let value = native.get("value").or(completed);
    let values = value
        .filter(|_| carrier.get("rows").is_none())
        .map_or(Vec::new(), |value| {
            value
                .as_array()
                .cloned()
                .unwrap_or_else(|| vec![value.clone()])
        });
    let ordinals = carrier.get("ordinals").and_then(Json::as_array);
    for (at, value) in values.iter().enumerate() {
        let actual = root(verb);
        let index = receipt.add(actual, value)?;
        receipt.add(
            "record_order",
            &json!({"ordinal":ordinals.and_then(|v|v.get(at)).cloned().unwrap_or(json!(at)),
            "result_index":index,"completed":completed.is_some()}),
        )?;
    }
    if let Some(selection) = carrier.get("selection") {
        receipt.add("find_selection", &json!({"selected":selection}))?;
    }
    for event in carrier
        .get("observations")
        .and_then(Json::as_array)
        .into_iter()
        .flatten()
    {
        receipt.add("event", event)?;
    }
    receipt.keys = keys;
    receipt.ids = ids;
    receipt.carrier = carrier;
    Ok(receipt)
}

pub(super) struct Invocation {
    pub(super) identity: String,
    verb: &'static str,
    prepared: complete_native::Prepared,
    request: thinkthen::AdmittedRequest,
    inputs: complete_native::Inputs,
    settings: thinkthen::Settings,
    keys: Vec<String>,
    ids: crate::tables::Ids,
    details: bool,
    recover: bool,
}

pub(super) fn prepare(
    db: *mut sqlite3,
    verb: &'static str,
    args: &Arguments,
    details: bool,
) -> Result<Invocation, Error> {
    if verb == "relate"
        && (4..=9).any(|at| {
            !matches!(
                args.value(&format!("legacy_{at}")),
                rusqlite::types::ValueRef::Null
            )
        })
    {
        return Err(Failure::plain_usage(
            "relate takes a query and rules; pass 'SELECT id, name, kind FROM …'",
        )
        .into());
    }
    let query = if args.text("request_records")?.is_none() {
        args.text("query")?
    } else {
        None
    };
    let placeholder = input::query_arguments(args, &[]);
    let mut inputs = input::inputs(if query.is_some() { &placeholder } else { args })?;
    let mut source = input::source(args)?;
    let settings = input::settings(args)?;
    let actual = if verb == "rank_set" { "rank" } else { verb };
    let legacy_recognize = verb == "recognize" && args.text("request_question")?.is_none();
    let legacy_relate = verb == "relate" && args.text("request_question")?.is_none();
    if verb == "rank" && args.text("request_question")?.is_none() {
        // The existing keyed rank question is literal text, including an @ prefix.
        thinkthen::Question::rank(&source)?;
        source = json!({"decide":source}).to_string();
    }
    let selector = if legacy_recognize || legacy_relate {
        None
    } else {
        crate::complete::selector::admit(actual, &source)?
    };
    let prepared = if legacy_recognize {
        complete_native::Prepared::Recognize(
            crate::tables::Recognizer::ask(args.text("kinds")?.as_deref())?,
            thinkthen::RecordReading::new(&[], None, None)?,
        )
    } else if legacy_relate {
        complete_native::Prepared::Relate(crate::tables::Relater::ask(&source)?)
    } else {
        crate::complete::prepare(actual, &source, &settings)?
    };
    let prepared = if legacy_recognize || legacy_relate {
        prepared.configured(actual, &settings)?
    } else {
        prepared
    };
    if details
        && verb == "find"
        && !args.supplied("request_records")
        && let Some(text) = args.text("text")?
    {
        let units = serde_json::from_str::<Vec<String>>(&text)
            .map_err(|_| Failure::usage("find units is a JSON array of text"))?;
        inputs = complete_native::Inputs::parse_request(
            &json!({"records":units.into_iter()
            .map(|text| json!({"text":text})).collect::<Vec<_>>(),
            "attempts":args.flag("attempts")?,"cancelled":args.flag("cancelled")?})
            .to_string(),
        )?;
    }
    let options = input::options(args)?;
    let source_route = args.text("paths")?.is_some() || options.files_only;
    let native_source = if source_route {
        inputs.request_source()?
    } else {
        None
    };
    let request = admit(&prepared, selector, native_source, options)?;
    let mut identity = input::identity(verb, args)?;
    let mut ids = std::collections::HashMap::new();
    if let Some(query) = query {
        let (entities, rows) = crate::tables::Relater::entities(db, &query)?;
        let records = input::query_arguments(args, &entities);
        identity.push_str(&format!("{:?}", records.values));
        for entity in &entities {
            identity.push_str(&format!(
                "{:?}",
                rows.get(&(entity.name().to_owned(), entity.kind().to_owned()))
            ));
        }
        inputs = input::inputs(&records)?;
        ids = rows;
    }
    Ok(Invocation {
        identity,
        verb,
        prepared,
        request,
        inputs,
        settings,
        details,
        recover: args.supplied("request_records"),
        keys: input::keys(args)?,
        ids,
    })
}

impl std::fmt::Debug for Invocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Invocation")
            .field("verb", &self.verb)
            .finish_non_exhaustive()
    }
}

pub(super) fn legacy_error(receipt: &Receipt) -> Result<(), Failure> {
    let Some(error) = receipt.carrier.pointer("/native/error") else {
        return Ok(());
    };
    let kind = match error.get("kind").and_then(Json::as_str) {
        Some("usage") => ErrorKind::Usage,
        Some("local") => ErrorKind::Local,
        Some("cancelled") => ErrorKind::Cancelled,
        Some("deadline") => ErrorKind::Deadline,
        Some("defect") => ErrorKind::Defect,
        _ => ErrorKind::Backend,
    };
    let mut failure = Failure::of(
        kind,
        error
            .get("message")
            .and_then(Json::as_str)
            .unwrap_or("the native call failed"),
    );
    failure.plain = matches!(
        failure.message.as_str(),
        "thinkthen_recognize takes no relations; relate rows with thinkthen_relate"
            | "relate takes a query and rules; pass 'SELECT id, name, kind FROM …'"
    );
    failure.retryable = error
        .get("retryable")
        .and_then(Json::as_bool)
        .unwrap_or(false);
    Err(failure)
}

fn admit(
    prepared: &complete_native::Prepared,
    selector: Option<thinkthen::AdmittedRequest>,
    native_source: Option<thinkthen::RequestSource>,
    options: thinkthen::RequestOptions,
) -> Result<thinkthen::AdmittedRequest, Error> {
    Ok(match selector {
        Some(selector) => {
            let mut arguments = selector.request().call.arguments().clone();
            arguments.options = options;
            if let Some(source) = native_source {
                arguments.input = thinkthen::RequestInput::Source { source };
            }
            thinkthen::Request::for_function(selector.request().call.function(), arguments)
                .admit()?
                .with_resolved_definition(sql_request::definition(prepared))?
        }
        None => match native_source {
            Some(source) => sql_request::request_source(prepared, source, options)?.admit()?,
            None => sql_request::request(prepared, options)?.admit()?,
        },
    })
}

fn details_carrier(verb: &str, mut carrier: Json) -> Result<Json, Failure> {
    if carrier.get("rows").is_some() {
        return Ok(carrier);
    }
    let error = carrier.pointer("/native/error").cloned();
    let function = match verb {
        "decide" => thinkthen::RequestFunction::Decide,
        "choose" => thinkthen::RequestFunction::Choose,
        "tag" => thinkthen::RequestFunction::Tag,
        "score" => thinkthen::RequestFunction::Score,
        "filter" => thinkthen::RequestFunction::Filter,
        "rank" => thinkthen::RequestFunction::Rank,
        "find" => thinkthen::RequestFunction::Find,
        "annotate" => thinkthen::RequestFunction::Annotate,
        "recognize" => thinkthen::RequestFunction::Recognize,
        "relate" => thinkthen::RequestFunction::Relate,
        _ => return Err(Failure::defect("unknown details function")),
    };
    if matches!(
        error
            .as_ref()
            .and_then(|error| error.get("kind"))
            .and_then(Json::as_str),
        Some("cancelled" | "deadline" | "defect")
    ) {
        let mut receipt = Receipt::default();
        receipt.carrier = carrier;
        legacy_error(&receipt)?;
        return Err(Failure::defect("a hard error was not raised"));
    }
    let rows = sql_request::project_carrier(function, &carrier);
    carrier["rows"] = json!(rows);
    Ok(carrier)
}
