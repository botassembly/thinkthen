//! Native PostgreSQL records execute canonical requests without transport JSON.
use crate::{call, complete_native, files};
use pgrx::prelude::*;
use thinkthen::{
    Error, Request, RequestArguments, RequestEnvironment, RequestInput, RequestOutcome,
    RequestQuestion,
};
mod collections;
mod constructors;
mod functions;
mod input;
mod options;
mod output;
mod plan;
mod reading;
mod results_generated;

fn request(
    verb: &'static str,
    question: &str,
    options: thinkthen::RequestOptions,
) -> Result<thinkthen::AdmittedRequest, Error> {
    let settings = thinkthen::Settings::default();
    let prepared = if question.starts_with('@') {
        let (reference, source) =
            files::read_resolved_question(question, call::file_directory().as_deref())?;
        let parsed = complete_native::settings::parse_file(verb, &source, &reference)?;
        complete_native::settings::prepare_file(verb, &source, &settings, parsed)?
    } else {
        complete_native::prepare(verb, question, &settings)?
    };
    let definition = thinkthen_host::sql_request::definition(&prepared);
    Request::named(
        verb,
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
    )?
    .admit()
}

#[derive(Debug)]
pub(super) struct Execution {
    pub(super) outcome: RequestOutcome,
    pub(super) observations: Vec<serde_json::Value>,
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
    let request = request(verb, question, options::read(reading)?)?;
    let projection = request.record_reading()?;
    let mut args = request.request().call.arguments().clone();
    // Composed feeds own projection; transfer it after canonical header admission.
    args.options.field = None;
    args.options.context_field = None;
    args.options.options_field = None;
    args.options.seed_spans_field = None;
    args.options.examples_field = None;
    let request = Request::named(verb, args)?.admit()?;
    let function = verb.parse()?;
    let records = input::deferred(records);
    let images = records.iter().any(|record| {
        record
            .as_ref()
            .is_ok_and(|record| !record.item.images.is_empty())
    });
    call::run_result(call::read_result()?, move |engine, controls| {
        let records = records
            .into_iter()
            .map(|record| input::compose(record?, &projection));
        let mut feed = thinkthen::RequestFeed::from_records("postgresql", records);
        if verb == "filter" {
            feed = feed.with_all_filter_results();
        }
        if !incremental {
            feed = feed.eager();
        }
        if images {
            feed = feed.with_image_inputs();
        }
        let token = thinkthen::CancelToken::new();
        if cancelled {
            token.cancel();
        }
        let controls = if cancelled {
            controls.cancel(&token)
        } else {
            controls
        };
        let events = std::sync::Mutex::new(Vec::new());
        let observer = |event: thinkthen::RecordObservation<'_>| {
            let packet = thinkthen::RequestSessionResult::Observation {
                function,
                value: event.to_owned(),
            };
            let value = packet.to_json().and_then(|text| {
                serde_json::from_str::<serde_json::Value>(&text)
                    .map_err(|_| call::defect("native observation could not be projected"))
            });
            events
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(value);
        };
        let outcome = engine.execute_request(
            &request,
            RequestEnvironment {
                controls: controls
                    .surface(thinkthen::Surface::Postgresql)
                    .observe(&observer),
                feed: Some(feed),
            },
        )?;
        let observations = events
            .into_inner()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Execution {
            outcome,
            observations,
        })
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
