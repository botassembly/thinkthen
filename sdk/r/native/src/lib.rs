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
struct NativeClient(RefCell<Option<Engine>>);

#[derive(Debug)]
struct NativeCall(RefCell<Option<Call>>);

fn failure(error: thinkthen::Error) -> List {
    list!(
        error = list!(
            code = error.code().as_str(),
            family = error.code().family().as_str(),
            message = error.to_string()
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
        Ok(client) => list!(value = ExternalPtr::new(NativeClient(RefCell::new(Some(client))))),
        Err(error) => failure(error),
    }
}

// The shared lifecycle diagnostic belongs to call setup in R.
fn closed_client() -> List {
    let error = thinkthen::native::closed_client_error();
    list!(
        error = list!(
            code = error.code().as_str(),
            family = "setup",
            message = error.to_string()
        )
    )
}

/// Release this client's engine; calls already opened retain their own engine.
#[extendr]
fn r_client_close(client: ExternalPtr<NativeClient>) {
    client.0.borrow_mut().take();
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
        _ => Err(thinkthen::native::abi_error()),
    };
    match result {
        Ok(value) => list!(value = value),
        Err(error) => failure(error),
    }
}

/// Open the shared streamed call with code-point positions.
#[extendr]
fn r_open(client: ExternalPtr<NativeClient>, request: &str) -> List {
    let engine = client.0.borrow();
    let Some(engine) = engine.as_ref() else {
        return closed_client();
    };
    let result = Request::from_json(request)
        .and_then(|request| Call::open(engine, &request, OffsetUnit::CodePointsInclusive));
    match result {
        Ok(call) => list!(value = ExternalPtr::new(NativeCall(RefCell::new(Some(call))))),
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
            return failure(thinkthen::native::abi_error());
        };
        Some(id.0.clone())
    };
    list!(
        value = ExternalPtr::new(NativeRecord(RefCell::new(Some(prepare_record(
            value, kind, id
        )))))
    )
}
/// Transfer a finite collection before the worker starts, preserving batch boundaries.
#[extendr]
fn r_open_finite(client: ExternalPtr<NativeClient>, request: &str, records: List) -> List {
    let engine = client.0.borrow();
    let Some(engine) = engine.as_ref() else {
        return closed_client();
    };
    let result = (|| {
        let request = Request::from_json(request)?;
        let mut initial = Vec::with_capacity(records.len());
        for record in records.values() {
            let record = ExternalPtr::<NativeRecord>::try_from(record)
                .map_err(|_| thinkthen::native::abi_error())?;
            initial.push(
                record
                    .0
                    .borrow_mut()
                    .take()
                    .ok_or_else(thinkthen::native::abi_error)?,
            );
        }
        Call::open_records(engine, &request, initial, OffsetUnit::CodePointsInclusive)
    })();
    match result {
        Ok(call) => list!(value = ExternalPtr::new(NativeCall(RefCell::new(Some(call))))),
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
            match serde_json::to_string(&row) {
                Ok(json) => list!(
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

fn send(call: &ExternalPtr<NativeCall>, value: &str, json: bool) -> List {
    let mut owned = call.0.borrow_mut();
    if let Some(call) = owned.as_mut() {
        let result = call.send(if json {
            Record::from_json(value)
        } else {
            Record::text(value)
        });
        if let Err(error) = result {
            return failure(error);
        }
    }
    list!()
}

/// Supply unwrapped UTF-8 text to the shared call.
#[extendr]
fn r_text(call: ExternalPtr<NativeCall>, text: &str) -> List {
    send(&call, text, false)
}

/// Supply one structured JSON record to the shared call.
#[extendr]
fn r_json(call: ExternalPtr<NativeCall>, json: &str) -> List {
    send(&call, json, true)
}

/// Finish the input collection.
#[extendr]
fn r_end(call: ExternalPtr<NativeCall>) {
    if let Some(call) = call.0.borrow_mut().as_mut() {
        call.end_input();
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
/// Supply a record with its separately admitted exact identifier.
#[extendr]
fn r_record_id(
    call: ExternalPtr<NativeCall>,
    value: &str,
    kind: i32,
    id: ExternalPtr<NativeId>,
) -> List {
    let record = prepare_record(value, kind, Some(id.0.clone()));
    if let Some(call) = call.0.borrow_mut().as_mut()
        && let Err(error) = call.send(record)
    {
        return failure(error);
    }
    list!()
}
/// Retain a host conversion failure as a failed record without a backend call.
#[extendr]
fn r_invalid(call: ExternalPtr<NativeCall>) -> List {
    if let Some(call) = call.0.borrow_mut().as_mut()
        && let Err(error) = call.send(Record::invalid())
    {
        return failure(error);
    }
    list!()
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
            return failure(thinkthen::native::abi_error());
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
    fn r_cancel_release;
    fn r_settings_conversion_error;
    fn r_question_conversion_error;
    fn r_question;
    fn r_abi_version;
    fn r_id_integer;
    fn r_id_string;
    fn r_record_id;
    fn r_invalid;
    fn r_client;
    fn r_open;
    fn r_open_finite;
    fn r_prepare_record;
    fn r_poll;
    fn r_text;
    fn r_json;
    fn r_end;
    fn r_close;
}
