//! R ownership around the public client and shared pulled call.
#![allow(
    missing_docs,
    reason = "extendr emits undocumented registration metadata; handwritten exports are private"
)]

use std::cell::RefCell;
use std::time::Duration;

use extendr_api::prelude::*;
use thinkthen::{
    Engine, OffsetUnit, Request, Settings,
    native::{Call, Event},
    records::{Record, RecordId},
};

#[derive(Debug)]
struct NativeClient(RefCell<Option<Engine>>, RefCell<Option<thinkthen::Usage>>);

#[derive(Debug)]
struct NativeCall(RefCell<Option<Call>>);

#[derive(Debug)]
struct NativeRequest(Request);

fn failure(error: thinkthen::Error) -> List {
    let http_status = error
        .http_status()
        .map_or_else(|| Robj::from(()), |status| Robj::from(i32::from(status)));
    list!(
        error = list!(
            code = error.code().as_str(),
            family = error.code().family().as_str(),
            message = error.to_string(),
            http_status = http_status
        )
    )
}

/// Return a shared settings conversion failure without exposing the host value.
#[extendr]
fn r_settings_conversion_error() -> List {
    failure(thinkthen::native::settings_conversion_error())
}

/// Return a shared question conversion failure without exposing the host value.
#[extendr]
fn r_question_conversion_error() -> List {
    failure(thinkthen::native::question_conversion_error())
}

/// Admit R client settings and return its owned handle or coded error.
#[extendr]
fn r_client(settings: &str) -> List {
    match Settings::from_json(settings)
        .and_then(|settings| settings.engine(|name| std::env::var(name).ok()))
    {
        Ok(client) => list!(
            value = ExternalPtr::new(NativeClient(RefCell::new(Some(client)), RefCell::new(None)))
        ),
        Err(error) => failure(error),
    }
}

/// Release this client's engine; calls already opened retain their own engine.
#[extendr]
fn r_client_close(client: ExternalPtr<NativeClient>) {
    if let Some(engine) = client.0.borrow_mut().take() {
        *client.1.borrow_mut() = Some(engine.usage());
    }
}

/// Copy cumulative usage without retaining the engine after close.
#[extendr]
fn r_client_usage(client: ExternalPtr<NativeClient>) -> List {
    let result = if let Some(engine) = client.0.borrow().as_ref() {
        serde_json::to_string(&engine.usage()).map_err(|_| thinkthen::native::internal_error())
    } else {
        client
            .1
            .borrow()
            .as_ref()
            .ok_or_else(thinkthen::native::internal_error)
            .and_then(|usage| {
                serde_json::to_string(usage).map_err(|_| thinkthen::native::internal_error())
            })
    };
    match result {
        Ok(value) => list!(value = value),
        Err(error) => failure(error),
    }
}

/// Return immutable canonical snapshots from public shared admission.
#[extendr]
fn r_admit(value: &str, kind: i32) -> List {
    let result = match kind {
        0 => Request::from_json(value).and_then(|request| {
            serde_json::to_string(request.question())
                .map_err(|_| thinkthen::native::internal_error())
        }),
        1 => Settings::from_json(value).and_then(|settings| {
            serde_json::to_string(&settings).map_err(|_| thinkthen::native::internal_error())
        }),
        2 => thinkthen::questions::Toolbox::from_definitions_json(value)
            .and_then(|toolbox| toolbox.definitions_json()),
        3 => thinkthen::questions::Toolbox::from_json(value)
            .and_then(|toolbox| toolbox.definitions_json()),
        4 => thinkthen::questions::Toolbox::from_definitions_json(value)
            .and_then(|toolbox| toolbox.to_json()),
        _ => Err(thinkthen::native::argument_error(
            thinkthen::Code::UsageArgument,
        )),
    };
    match result {
        Ok(value) => list!(value = value),
        Err(error) => failure(error),
    }
}

/// A prepared record transfers ownership once into a finite call.
#[derive(Debug)]
struct NativeRecord(RefCell<Option<Record>>);

fn prepare_record(value: &str, kind: i32, id: Option<RecordId>) -> Record {
    let record = match kind {
        0 => Record::text(value),
        1 => match &id {
            Some(id) => Record::from_json_with_id(value, id.clone()),
            None => Record::from_json(value),
        },
        2 => Record::missing(),
        _ => Record::invalid(),
    };
    match id {
        Some(id) => record.with_id(id),
        None => record,
    }
}
/// Prepare text, JSON, missing input or a conversion failure through shared admission.
#[extendr]
fn r_prepare_record(value: &str, kind: i32, id: Robj) -> List {
    let id = if id.is_null() {
        None
    } else {
        let Ok(id) = ExternalPtr::<NativeId>::try_from(id) else {
            return failure(thinkthen::native::argument_error(
                thinkthen::Code::UsageArgument,
            ));
        };
        Some(id.0.clone())
    };
    list!(
        value = ExternalPtr::new(NativeRecord(RefCell::new(Some(prepare_record(
            value, kind, id
        )))))
    )
}

/// Admit a call and the actual table columns before a client or request is opened.
#[extendr]
fn r_prepare_request(value: &str, columns: Vec<String>, suffixes: Vec<String>) -> List {
    let result = (|| {
        let request = Request::from_json(value)?;
        if let thinkthen::Question::Annotate(question) = request.question() {
            let columns = columns.iter().map(String::as_str).collect::<Vec<_>>();
            let suffixes = suffixes.iter().map(String::as_str).collect::<Vec<_>>();
            question.validate_table_columns(&columns, &suffixes)?;
        }
        let definition = serde_json::to_string(request.question())
            .map_err(|_| thinkthen::native::internal_error())?;
        Ok((request, definition))
    })();
    match result {
        Ok((request, definition)) => list!(
            value = ExternalPtr::new(NativeRequest(request)),
            definition = definition
        ),
        Err(error) => failure(error),
    }
}

/// Transfer a finite collection before the worker starts, preserving batch boundaries.
#[extendr]
fn r_open_finite(
    client: ExternalPtr<NativeClient>,
    request: ExternalPtr<NativeRequest>,
    records: List,
) -> List {
    let engine = client.0.borrow();
    let Some(engine) = engine.as_ref() else {
        return failure(thinkthen::native::closed_client_error());
    };
    let result = (|| {
        let mut initial = Vec::with_capacity(records.len());
        for record in records.values() {
            let record = ExternalPtr::<NativeRecord>::try_from(record)
                .map_err(|_| thinkthen::native::argument_error(thinkthen::Code::UsageArgument))?;
            initial.push(
                record
                    .0
                    .borrow_mut()
                    .take()
                    .ok_or_else(thinkthen::native::abi_error)?,
            );
        }
        Call::open_records(engine, &request.0, initial, OffsetUnit::CodePointsInclusive)
    })();
    match result {
        Ok(call) => list!(value = ExternalPtr::new(NativeCall(RefCell::new(Some(call))))),
        Err(error) => failure(error),
    }
}

/// Validate the canonical saved shape while keeping exact identifiers out of R doubles.
#[extendr]
fn r_saved_row(value: &str, id: Robj) -> List {
    let result = (|| {
        let invalid = || thinkthen::native::argument_error(thinkthen::Code::UsageArgument);
        let mut value: serde_json::Value = serde_json::from_str(value).map_err(|_| invalid())?;
        if !id.is_null() {
            let id = ExternalPtr::<NativeId>::try_from(id).map_err(|_| invalid())?;
            value.as_object_mut().ok_or_else(invalid)?.insert(
                "id".into(),
                serde_json::to_value(&id.0).map_err(|_| thinkthen::native::internal_error())?,
            );
        }
        let row: thinkthen::results::AnyRow =
            serde_json::from_value(value).map_err(|_| invalid())?;
        serde_json::to_string(&row).map_err(|_| thinkthen::native::internal_error())
    })();
    match result {
        Ok(value) => list!(value = value),
        Err(error) => failure(error),
    }
}

/// Wait briefly for an event so R can service interrupts between calls.
#[extendr]
fn r_poll(call: ExternalPtr<NativeCall>) -> List {
    let mut owned = call.0.borrow_mut();
    let Some(call) = owned.as_mut() else {
        return list!(event = "finished");
    };
    match call.poll(Duration::from_millis(20)) {
        Ok(None) => list!(event = "pending"),
        Ok(Some(Event::NeedsInput)) => list!(event = "input"),
        Ok(Some(Event::Row(row))) => {
            let id = row.id().map_or_else(
                || list!(kind = "absent"),
                |id| {
                    if let Some(text) = id.as_str() {
                        list!(kind = "string", text = text)
                    } else if let Some(value) = id.as_i64() {
                        list!(kind = "signed", decimal = value.to_string())
                    } else {
                        list!(
                            kind = "unsigned",
                            decimal = id.as_u64().map(|value| value.to_string())
                        )
                    }
                },
            );
            match thinkthen::native::host_row_json(&row).map(|(value, _)| value.to_string()) {
                Ok(json) => list!(
                    kept = match &row {
                        thinkthen::results::AnyRow::Filter(row) =>
                            Some(row.answer().is_ok_and(|answer| answer)),
                        _ => None,
                    },
                    event = "row",
                    json = json,
                    id = id,
                    position = row.position() as f64
                ),
                Err(_) => failure(thinkthen::native::internal_error()),
            }
        }
        Ok(Some(Event::Finished)) => {
            *owned = None;
            list!(event = "finished")
        }
        Ok(Some(_)) => failure(thinkthen::native::abi_error()),
        Err(error) => {
            *owned = None;
            failure(error)
        }
    }
}

/// Cancel and release the shared call without waiting for its worker.
#[extendr]
fn r_close(call: ExternalPtr<NativeCall>) {
    if let Some(call) = call.0.borrow_mut().take() {
        call.cancel();
    }
}

/// Select an admitted typed toolbox entry without resolving a reference.
#[extendr]
fn r_question(definition: &str, name: &str) -> List {
    let result = (|| {
        let domain = thinkthen::questions::Toolbox::from_definitions_json(definition)?;
        let mut builder = thinkthen::native::ToolboxBuilder::default();
        for (name, question) in domain.iter() {
            builder.insert(name, question.clone().into())?;
        }
        let question = builder.select(name)?.build()?;
        serde_json::to_string(&question).map_err(|_| thinkthen::native::internal_error())
    })();
    match result {
        Ok(value) => list!(value = value),
        Err(error) => failure(error),
    }
}
/// An admitted exact integer or string identifier, never an R double.
#[derive(Debug)]
struct NativeId(RecordId);
/// Admit an exact decimal-backed host integer through shared Rust bounds.
#[extendr]
fn r_id_integer(decimal: &str) -> List {
    match RecordId::integer_decimal(decimal) {
        Ok(id) => {
            let (kind, decimal) = if let Some(value) = id.as_i64() {
                ("signed", value.to_string())
            } else {
                (
                    "unsigned",
                    id.as_u64()
                        .map(|value| value.to_string())
                        .unwrap_or_default(),
                )
            };
            list!(
                value = ExternalPtr::new(NativeId(id)),
                kind = kind,
                decimal = decimal
            )
        }
        Err(error) => failure(error),
    }
}
/// Preserve a string identifier's type, including digit-only strings.
#[extendr]
fn r_id_string(text: &str) -> ExternalPtr<NativeId> {
    ExternalPtr::new(NativeId(RecordId::from(text)))
}
/// Report this direct native module's contract version before client creation.
#[extendr]
fn r_abi_version() -> i32 {
    thinkthen::ABI_VERSION as i32
}

/// Cancel and release an interrupted R call, returning the shared cancellation.
#[extendr]
fn r_cancel_release(call: Robj) -> List {
    if !call.is_null() {
        let Ok(call) = ExternalPtr::<NativeCall>::try_from(call) else {
            return failure(thinkthen::native::argument_error(
                thinkthen::Code::UsageArgument,
            ));
        };
        if let Some(call) = call.0.borrow_mut().take() {
            call.cancel();
        }
    }
    failure(thinkthen::native::cancelled_error())
}
extendr_module! {
    mod thinkthen;
    fn r_admit;
    fn r_client_close;
    fn r_client_usage;
    fn r_cancel_release;
    fn r_settings_conversion_error;
    fn r_question_conversion_error;
    fn r_question;
    fn r_abi_version;
    fn r_id_integer;
    fn r_id_string;
    fn r_client;
    fn r_open_finite;
    fn r_prepare_record;
    fn r_prepare_request;
    fn r_saved_row;
    fn r_poll;
    fn r_close;
}
