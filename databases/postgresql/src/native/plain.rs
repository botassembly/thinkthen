//! Plain aggregate arguments retain the ordinary call's admitted request.
use crate::call;
use pgrx::prelude::*;
use thinkthen::{
    Error, Request, RequestArguments, RequestDefinition, RequestEnvironment, RequestFunction,
    RequestInput, RequestItem, RequestQuestion,
};

pub(super) fn run(
    function: RequestFunction,
    definition: RequestDefinition,
    items: Vec<RequestItem>,
    controls: call::Call,
) -> Result<super::Execution, Error> {
    let request = Request::for_function(
        function,
        RequestArguments {
            question: RequestQuestion::Definition { value: definition },
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
    let reading = request.record_reading()?;
    let records = items
        .into_iter()
        .map(|item| item.compose_record(&reading))
        .collect::<Result<Vec<_>, _>>()?;
    call::run_result(controls, move |engine, controls| {
        let feed =
            thinkthen::RequestFeed::from_records("postgresql", records.into_iter().map(Ok)).eager();
        let feed = if function == RequestFunction::Filter {
            feed.with_all_filter_results()
        } else {
            feed
        };
        let events = std::sync::Mutex::new(Vec::new());
        let observer = |event: thinkthen::RecordObservation<'_>| {
            events
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(super::observation(function, event));
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
        let rows = thinkthen_host::sql_request::project_outcome(function, outcome, observations)?;
        Ok(super::Execution {
            value: serde_json::json!({"rows": rows}),
        })
    })
}

pub(super) fn find(
    question: &str,
    units: Array<'_, &str>,
    raw: Option<crate::ffi::RawJson>,
) -> Result<super::Execution, Error> {
    let (settings, mut controls) = crate::forms::controls_result(raw.as_ref(), Default::default())?;
    if let Some(model) = settings.model() {
        controls = controls.with_model(model);
    }
    settings
        .check(thinkthen::For::Find)
        .map_err(|e| call::usage(e.to_string()))?;
    let mut question = thinkthen::Question::find(question)?;
    if settings.none().unwrap_or(false) {
        question = question.offering_none()?;
    }
    let items = units
        .iter()
        .map(|unit| {
            unit.map(|unit| crate::request::text(unit.to_owned()))
                .ok_or_else(|| call::usage("a find unit is text, not NULL"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    run(RequestFunction::Find, question.into(), items, controls)
}
pub(super) fn keyed(
    verb: &str,
    question: &str,
    input: pgrx::datum::JsonB,
    raw: Option<crate::ffi::RawJson>,
) -> Result<super::Execution, Error> {
    let (settings, controls) = crate::forms::controls_result(raw.as_ref(), Default::default())?;
    let records = crate::forms::keyed(Some(input));
    let items = records
        .into_iter()
        .map(|(_, value)| crate::request::text(value))
        .collect();
    let prepared = if verb == "rank" {
        settings
            .check(thinkthen::For::Rank)
            .map_err(|e| call::usage(e.to_string()))?;
        let question = thinkthen::Question::rank(question)?;
        let question = if let Some(model) = settings.model() {
            question.with_model(model)?
        } else {
            question
        };
        thinkthen_host::complete_native::Prepared::Rank(question)
    } else {
        super::prepared(verb, question, &settings)?
    };
    run(
        verb.parse()?,
        thinkthen_host::sql_request::definition(&prepared),
        items,
        controls,
    )
}
pub(super) fn recognize(body: &str, kinds: Array<'_, &str>) -> Result<super::Execution, Error> {
    let mut ask = thinkthen::Recognize::builder();
    for kind in kinds.iter() {
        ask = ask.kind(thinkthen::Kind::new(
            kind.ok_or_else(|| call::usage("kind list contains NULL"))?,
            None,
        )?)?;
    }
    run(
        RequestFunction::Recognize,
        ask.build()?.into(),
        vec![crate::request::text(body.to_owned())],
        call::read_result()?,
    )
}
pub(super) fn relate(query: &str, rules: Array<'_, &str>) -> Result<super::Execution, Error> {
    let (kinds, rows) = crate::relate::rows(query)?;
    let rules = rules
        .iter()
        .map(|value| {
            value
                .map(str::to_owned)
                .ok_or_else(|| call::usage("rule list contains NULL"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let ask = crate::relate::inline(&rules, kinds)?;
    let mut seen = std::collections::HashSet::new();
    let items = rows
        .into_iter()
        .filter(|(_, name, kind)| seen.insert((name.clone(), kind.clone())))
        .map(|(_, name, kind)| {
            crate::request::json(serde_json::json!({"name": name, "kind": kind}))
        })
        .collect::<Result<Vec<_>, _>>()?;
    run(
        RequestFunction::Relate,
        ask.into(),
        items,
        call::read_result()?,
    )
}

pub(super) fn relate_spec(query: &str, spec: &str) -> Result<super::Execution, Error> {
    let (_, rows) = crate::relate::rows(query)?;
    let ask =
        crate::files::Given::read(Some(spec), "relate file", call::file_directory().as_deref())?
            .parse(thinkthen::Relate::from_json)?;
    let mut seen = std::collections::HashSet::new();
    let items = rows
        .into_iter()
        .filter(|(_, name, kind)| seen.insert((name.clone(), kind.clone())))
        .map(|(_, name, kind)| {
            crate::request::json(serde_json::json!({"name": name, "kind": kind}))
        })
        .collect::<Result<Vec<_>, _>>()?;
    run(
        RequestFunction::Relate,
        ask.into(),
        items,
        call::read_result()?,
    )
}
