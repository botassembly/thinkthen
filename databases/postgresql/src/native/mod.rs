//! Native PostgreSQL records execute canonical requests without transport JSON.
use crate::call::OrRaise as _;
use crate::{call, complete_native, files};
use pgrx::prelude::*;
use thinkthen::{Error, Request, RequestArguments, RequestInput, RequestQuestion};
mod constructors;
mod context_schema;
mod functions_generated;
mod input;
mod options;
mod output;
mod plain;
mod plan;
mod question_settings;
mod reading;
mod results_generated;

fn request(
    verb: &str,
    question: &str,
    options: thinkthen::RequestOptions,
    settings: &thinkthen::Settings,
) -> Result<thinkthen::AdmittedRequest, Error> {
    let prepared = prepared(verb, question, settings)?;
    request_prepared(verb, &prepared, options)
}

fn request_prepared(
    verb: &str,
    prepared: &complete_native::Prepared,
    options: thinkthen::RequestOptions,
) -> Result<thinkthen::AdmittedRequest, Error> {
    let function = verb.parse()?;
    let definition = thinkthen_host::sql_request::definition(prepared);
    Request::for_function(
        function,
        RequestArguments {
            question: RequestQuestion::Definition { value: definition },
            input: RequestInput::Feed {
                name: "postgresql".to_owned(),
                framing: Default::default(),
                reading: Default::default(),
                images: Vec::new(),
            },
            options,
        },
    )
    .admit()
}

pub(super) struct Execution {
    pub(super) value: serde_json::Value,
}

impl std::fmt::Debug for Execution {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Execution")
            .field(
                "rows",
                &self
                    .value
                    .get("rows")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len),
            )
            .finish_non_exhaustive()
    }
}

pub(super) fn execute(
    verb: &'static str,
    question: &str,
    records: Array<'_, pgrx::composite_type!("thinkthen_record")>,
    reading: Option<pgrx::composite_type!("thinkthen_reading")>,
) -> Result<Execution, Error> {
    let incremental = reading
        .as_ref()
        .map(|tuple| field::<bool>(tuple, "incremental"))
        .transpose()?
        .flatten()
        .unwrap_or(false);
    let cancelled = reading
        .as_ref()
        .map(|tuple| field::<bool>(tuple, "cancelled"))
        .transpose()?
        .flatten()
        .unwrap_or(false);
    let context_schema = context_schema::read(reading.as_ref())?;
    let settings = question_settings::read(reading.as_ref())?;
    let prepared = prepared(verb, question, &settings)?;
    let request = request_prepared(verb, &prepared, options::read(reading)?)?;
    let mut projection = request.record_reading()?;
    if let Some(schema) = context_schema {
        projection = projection.with_context_schema(schema);
    }
    let request = composed_request(&request)?;
    let records = input::deferred(records);
    let image_inputs = records.iter().any(|record| {
        record
            .as_ref()
            .is_ok_and(|record| !record.item.images.is_empty())
    });
    let records = if incremental {
        records
    } else {
        records
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(Ok)
            .collect()
    };
    call::run_result(call::read_result()?, move |engine, controls| {
        let records = records
            .into_iter()
            .map(|record| input::compose(record?, &projection));
        let records: Box<
            dyn Iterator<Item = Result<thinkthen::RecordInput<thinkthen::QuestionInput>, Error>>,
        > = if incremental {
            Box::new(records)
        } else {
            Box::new(records.collect::<Result<Vec<_>, _>>()?.into_iter().map(Ok))
        };
        let feed = thinkthen::RequestFeed::from_records("postgresql", records);
        let feed = if incremental { feed } else { feed.eager() };
        let feed = if image_inputs {
            feed.with_image_inputs()
        } else {
            feed
        };
        let token = thinkthen::CancelToken::new();
        if cancelled {
            token.cancel();
        }
        let controls = if cancelled {
            controls.cancel(&token)
        } else {
            controls
        };
        let value = thinkthen_host::sql_request::run_feed_recovering(
            engine,
            &prepared,
            &request,
            feed,
            controls.surface(thinkthen::Surface::Postgresql),
            thinkthen::Surface::Postgresql,
        )?;
        Ok(Execution { value })
    })
}

pub(super) fn field<
    'a,
    T: pgrx::FromDatum + pgrx::IntoDatum + pgrx::datum::UnboxDatum<As<'a> = T> + 'a,
>(
    tuple: &'a PgHeapTuple<'_, pgrx::AllocatedByRust>,
    name: &str,
) -> Result<Option<T>, Error> {
    tuple
        .get_by_name(name)
        .map_err(|_| call::usage(format!("invalid {name} field")))
}

pub(super) fn number(value: i64) -> Result<usize, Error> {
    usize::try_from(value).map_err(|_| call::usage("reading counts cannot be negative"))
}

fn composed_request(
    request: &thinkthen::AdmittedRequest,
) -> Result<thinkthen::AdmittedRequest, Error> {
    let mut args = request.request().call.arguments().clone();
    args.options.field = None;
    args.options.context_field = None;
    args.options.options_field = None;
    args.options.seed_spans_field = None;
    args.options.examples_field = None;
    Request::for_function(request.request().call.function(), args).admit()
}

fn observation(
    function: thinkthen::RequestFunction,
    event: thinkthen::RecordObservation<'_>,
) -> Result<serde_json::Value, Error> {
    let inputs = complete_native::observations::event(&event)?
        .get("inputs")
        .cloned();
    let packet = thinkthen::RequestSessionResult::Observation {
        function,
        value: event.to_owned(),
    };
    let mut value: serde_json::Value = serde_json::from_str(&packet.to_json()?)
        .map_err(|_| call::defect("native observation could not be projected"))?;
    if let Some(inputs) = inputs {
        value
            .get_mut("value")
            .and_then(serde_json::Value::as_object_mut)
            .ok_or_else(|| call::defect("native observation is invalid"))?
            .insert("inputs".to_owned(), inputs);
    }
    Ok(value)
}

fn prepared(
    verb: &str,
    question: &str,
    settings: &thinkthen::Settings,
) -> Result<complete_native::Prepared, Error> {
    let prepared = if question.starts_with('@') {
        let (reference, source) =
            files::read_resolved_question(question, call::file_directory().as_deref())?;
        let parsed = complete_native::settings::parse_file(verb, &source, &reference)?;
        complete_native::settings::prepare_file(verb, &source, settings, parsed)?
    } else {
        complete_native::prepare(verb, question, settings)?
    };
    Ok(prepared)
}

pub(super) fn row_values(
    function: thinkthen::RequestFunction,
    result: Result<Execution, Error>,
) -> Vec<serde_json::Value> {
    let value = match result {
        Ok(execution) => execution.value,
        Err(error) => thinkthen_host::sql_request::failure_carrier(function, error).or_raise(),
    };
    value
        .get("rows")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_else(|| call::raise(call::defect("recovery returned no rows")))
}
#[allow(clippy::too_many_arguments, reason = "native SQL argument mapping")]
pub(super) fn execute_plain(
    verb: &'static str,
    question: &str,
    input: &str,
    members: Option<Array<'_, &str>>,
    raw: Option<crate::ffi::RawJson>,
    named: crate::forms::Named<'_>,
) -> Result<Execution, Error> {
    let (settings, controls) = crate::forms::controls_result(raw.as_ref(), named)?;
    let prepared = match verb {
        "decide" | "choose" | "score" | "tag" => {
            let role = match verb {
                "choose" => thinkthen::For::Choose,
                "score" => thinkthen::For::Score,
                "tag" => thinkthen::For::Tag,
                _ => thinkthen::For::Decide,
            };
            complete_native::Prepared::Atomic(crate::forms::question_result(
                Some(question),
                members,
                role,
                &settings,
            )?)
        }
        _ => prepared(verb, question, &settings)?,
    };
    let request = Request::for_function(
        verb.parse()?,
        RequestArguments {
            question: RequestQuestion::Definition {
                value: thinkthen_host::sql_request::definition(&prepared),
            },
            input: RequestInput::Feed {
                name: "postgresql".to_owned(),
                framing: Default::default(),
                reading: Default::default(),
                images: Vec::new(),
            },
            options: Default::default(),
        },
    )
    .admit()?;
    let projection = request.record_reading()?;
    let request = composed_request(&request)?;
    let item = if verb == "annotate" {
        crate::request::document(input.to_owned())?
    } else {
        crate::request::text(input.to_owned())
    };
    let record = item.compose_record(&projection)?;
    call::run_result(controls, move |engine, controls| {
        let value = thinkthen_host::sql_request::run_records_recovering(
            engine,
            &prepared,
            &request,
            std::iter::once(Ok(record)),
            controls,
            thinkthen::Surface::Postgresql,
        )?;
        Ok(Execution { value })
    })
}
