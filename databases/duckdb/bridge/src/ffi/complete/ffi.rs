//! Counted named complete calls share the session engine and final native facts.
use super::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, batch, probe, reply_boundary,
    run_detached, text,
};
use crate::{complete_native, engines};

pub(super) use thinkthen_host::sql_request as request;

/// Execute one named complete call; C++ retains readable ranges through return.
/// # Safety
/// All counted ranges and the stop callback follow the existing bridge contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete(
    verb: BridgeText,
    question: BridgeText,
    inputs: BridgeText,
    settings: BridgeText,
    reference: *const thinkthen::QuestionFileReference,
    deadline: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    complete(
        verb, question, inputs, settings, reference, deadline, session, stop, None,
    )
}

/// Execute an existing native binary image collection through the same Request.
/// # Safety
/// C++ retains all counted image ranges through synchronous return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_images(
    question: BridgeText,
    images: *const super::images::BridgeImage,
    count: usize,
    settings: BridgeText,
    reference: *const thinkthen::QuestionFileReference,
    deadline: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    let verb = BridgeText {
        bytes: b"decide".as_ptr(),
        len: 6,
    };
    let inputs = BridgeText {
        bytes: b"".as_ptr(),
        len: 0,
    };
    complete(
        verb,
        question,
        inputs,
        settings,
        reference,
        deadline,
        session,
        stop,
        Some((images, count)),
    )
}

#[allow(
    clippy::too_many_arguments,
    reason = "the private counted host ABI retains its existing ranges"
)]
fn complete(
    verb: BridgeText,
    question: BridgeText,
    inputs: BridgeText,
    settings: BridgeText,
    reference: *const thinkthen::QuestionFileReference,
    deadline: i64,
    session: BridgeSettings,
    stop: BridgeStop,
    images: Option<(*const super::images::BridgeImage, usize)>,
) -> Reply {
    reply_boundary(|| {
        let verb = text(verb.bytes, verb.len)?;
        let question = text(question.bytes, question.len)?;
        let inputs = text(inputs.bytes, inputs.len)?;
        let raw = text(settings.bytes, settings.len)?;
        let parsed = (|| {
            // SAFETY: C++ retains this selection through synchronous return.
            let saved = unsafe { reference.as_ref() }
                .map(|held| complete_native::settings::parse_file(verb, question, held))
                .transpose()?;
            let call = thinkthen::Settings::parse(raw)
                .map_err(|e| complete_native::usage(&e.to_string()))?;
            if saved.is_none() && question.starts_with('@') {
                return Err(complete_native::usage(
                    "the question reference was not read by this database",
                ));
            }
            let prepared = if let Some(saved) = saved {
                complete_native::settings::prepare_file(verb, question, &call, saved)?
            } else {
                complete_native::prepare(verb, question, &call)?
            };
            // Explicit files reach this call only after DuckDB authorizes readers.
            if images.is_none() {
                complete_native::Inputs::parse(inputs, false)?;
            }
            let request = request::admit(&prepared)?;
            let inputs = if let Some((images, count)) = images {
                complete_native::Inputs::from_record(
                    super::images::complete_record(images, count)
                        .map_err(|error| complete_native::usage(&error))?,
                )
            } else {
                complete_native::Inputs::parse_request(inputs)?
            };
            Ok::<_, thinkthen::Error>((call, prepared, inputs, request))
        })();
        let (call, prepared, inputs, request) = match parsed {
            Ok(value) => value,
            Err(error) => return Ok(complete_native::admission(&error).to_string().into_bytes()),
        };
        let held = asked(&session)?;
        let engine = engines::engine_for(&held, |path| probe(&session, path))?;
        let batch = if call.batch_max() {
            Some("max".to_owned())
        } else {
            call.batch_records()
                .map(|n| n.to_string())
                .or(batch(&session)?)
        };
        let context = call.context().map(str::to_owned);
        let due = match call.deadline_ms() {
            None | Some(-1) => deadline,
            Some(value) if deadline < 0 => value,
            Some(value) => value.min(deadline),
        };
        run_detached(stop, move |token| {
            let options = engines::options_for(
                due,
                &token,
                held.max_requests_total,
                batch.as_deref(),
                context.as_deref(),
            )?;
            Ok(request::run(
                &engine,
                &prepared,
                &request,
                inputs,
                options,
                thinkthen::Surface::Duckdb,
            )
            .to_string()
            .into_bytes())
        })
    })
}

/// Wrap an already typed native admission error without adding native fields.
/// # Safety
/// The counted native JSON range is readable until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_failure_envelope(
    error: BridgeText,
) -> Reply {
    reply_boundary(|| {
        let native = serde_json::from_str(text(error.bytes, error.len)?)
            .map_err(|_| "thinkthen defect: invalid native admission error".to_owned())?;
        Ok(complete_native::carrier(native, serde_json::json!([]))
            .to_string()
            .into_bytes())
    })
}

/// Admit a file feed with all borrowed SQL state copied before return.
/// # Safety
/// All counted ranges and the selection remain readable through return; out takes one session.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_feed_new(
    verb: BridgeText,
    question: BridgeText,
    inputs: BridgeText,
    settings: BridgeText,
    reference: *const thinkthen::QuestionFileReference,
    deadline: i64,
    session: BridgeSettings,
    out: *mut *mut thinkthen::RequestSession,
) -> Reply {
    reply_boundary(|| {
        if out.is_null() {
            return Err("thinkthen defect: missing session output".to_owned());
        }
        let parsed = (|| {
            let verb = text(verb.bytes, verb.len).map_err(|_| complete_native::defect())?;
            let question =
                text(question.bytes, question.len).map_err(|_| complete_native::defect())?;
            let raw = text(settings.bytes, settings.len).map_err(|_| complete_native::defect())?;
            let call = thinkthen::Settings::parse(raw)
                .map_err(|e| complete_native::usage(&e.to_string()))?;
            // SAFETY: the caller retains the selection only for this call. Preparation owns it.
            let saved = unsafe { reference.as_ref() }
                .map(|held| complete_native::settings::parse_file(verb, question, held))
                .transpose()?;
            if saved.is_none() && question.starts_with('@') {
                return Err(complete_native::usage(
                    "the question reference was not read by this database",
                ));
            }
            let prepared = if let Some(saved) = saved {
                complete_native::settings::prepare_file(verb, question, &call, saved)?
            } else {
                complete_native::prepare(verb, question, &call)?
            };
            let input = complete_native::Inputs::parse_request(
                text(inputs.bytes, inputs.len).map_err(|_| complete_native::defect())?,
            )?;
            let reading = input.record_reading(prepared.reading())?;
            Ok::<_, thinkthen::Error>((call, prepared, input, reading))
        })();
        let (call, prepared, input, reading) = match parsed {
            Ok(v) => v,
            Err(e) => return Ok(complete_native::admission(&e).to_string().into_bytes()),
        };
        let held = asked(&session)?;
        let engine = engines::engine_for(&held, |path| probe(&session, path))?;
        let batch = if call.batch_max() {
            Some(thinkthen::RequestBatch::Named("max".to_owned()))
        } else if let Some(n) = call.batch_records() {
            Some(thinkthen::RequestBatch::Count(n))
        } else {
            batch(&session)?.map(request_batch)
        };
        let due = match call.deadline_ms() {
            None | Some(-1) => deadline,
            Some(value) if deadline < 0 => value,
            Some(value) => value.min(deadline),
        };
        let options = thinkthen::RequestOptions {
            batch,
            context: call.context().map(str::to_owned),
            deadline_ms: Some(due),
            attempts: input.attempts,
            max_requests_total: held
                .max_requests_total
                .map(u64::try_from)
                .transpose()
                .map_err(|_| {
                    "thinkthen usage: a request total is a whole number of 0 or more".to_owned()
                })?,
            ..Default::default()
        };
        let started = (|| {
            let request = request::request(&prepared, options)?;
            engine.request_session_with_feed_options(
                request,
                thinkthen::Surface::Duckdb,
                thinkthen::RequestSessionFeedOptions {
                    eager: !input.incremental,
                    image_inputs: input.image_inputs()?,
                    all_filter_results: matches!(prepared, complete_native::Prepared::Filter(_)),
                    record_reading: Some(reading),
                },
            )
        })();
        match started {
            Err(e) => Ok(complete_native::admission(&e).to_string().into_bytes()),
            Ok(feed) => {
                if input.cancelled {
                    feed.cancel();
                }
                // SAFETY: exclusive ownership of the existing session allocation passes to the caller.
                unsafe {
                    out.write(Box::into_raw(Box::new(feed)));
                }
                Ok(Vec::new())
            }
        }
    })
}

pub(super) fn supplement(
    value: &mut serde_json::Value,
    result: &thinkthen::RequestValue,
) -> Result<(), thinkthen::Error> {
    request::supplement(value, result)
}

fn request_batch(value: String) -> thinkthen::RequestBatch {
    if value == "max" {
        thinkthen::RequestBatch::Named(value)
    } else {
        thinkthen::RequestBatch::Count(value.parse().unwrap_or_default())
    }
}
