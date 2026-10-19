//! Counted named complete calls share the session engine and final native facts.
use super::{BridgeSettings, BridgeText, Reply, asked, batch, probe, reply_boundary, text};
use crate::{complete_native, engines};

pub(super) use thinkthen_host::sql_request as request;

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
    native_reading: BridgeText,
    out: *mut *mut thinkthen::RequestRecoveringSession,
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
            let native_reading = text(native_reading.bytes, native_reading.len)
                .map_err(|_| complete_native::defect())?;
            let native = if native_reading.is_empty() {
                None
            } else {
                Some(super::native_records::header(
                    verb,
                    &prepared,
                    native_reading,
                )?)
            };
            let reading = match &native {
                Some((_, reading)) => reading.clone(),
                None => input.record_reading(prepared.reading())?,
            };
            Ok::<_, thinkthen::Error>((call, prepared, input, reading, native))
        })();
        let (call, prepared, input, reading, native) = match parsed {
            Ok(v) => v,
            Err(e) => return admission(e),
        };
        let held = asked(&session)?;
        let engine = engines::engine_for(&held, |path| probe(&session, path))?;
        let batch = source_batch(&call, &session)?;
        let due = super::portable::due(deadline, call.deadline_ms());
        let options = thinkthen::RequestOptions {
            batch,
            context: call.context().map(str::to_owned),
            deadline_ms: Some(due),
            attempts: input.attempts,
            max_requests_total: source_total(held.max_requests_total)?,
            ..Default::default()
        };
        let started = (|| {
            let request = source_request(native.map(|(request, _)| request), &prepared, options)?;
            let all_filter_results = matches!(prepared, complete_native::Prepared::Filter(_))
                && !request.call.arguments().options.files_only;
            engine.request_recovering_session_with_feed_options(
                request,
                thinkthen::Surface::Duckdb,
                thinkthen::RequestSessionFeedOptions {
                    retain_success_on_cancel: false,
                    eager: !input.incremental,
                    image_inputs: input.image_inputs()?,
                    all_filter_results,
                    record_reading: Some(reading),
                },
            )
        })();
        match started {
            Err(e) => admission(e),
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

fn admission(error: thinkthen::Error) -> Result<Vec<u8>, String> {
    if matches!(
        error.kind(),
        thinkthen::ErrorKind::Cancelled
            | thinkthen::ErrorKind::Deadline
            | thinkthen::ErrorKind::Defect
    ) {
        Err(crate::errors::RowError::from(error).text)
    } else {
        Ok(complete_native::admission(&error).to_string().into_bytes())
    }
}

fn source_total(total: Option<i64>) -> Result<Option<u64>, String> {
    total
        .map(u64::try_from)
        .transpose()
        .map_err(|_| "thinkthen usage: a request total is a whole number of 0 or more".to_owned())
}

fn source_batch(
    call: &thinkthen::Settings,
    session: &BridgeSettings,
) -> Result<Option<thinkthen::RequestBatch>, String> {
    Ok(if call.batch_max() {
        Some(thinkthen::RequestBatch::Named("max".to_owned()))
    } else if let Some(n) = call.batch_records() {
        Some(thinkthen::RequestBatch::Count(n))
    } else {
        batch(session)?.map(request_batch)
    })
}

fn source_request(
    native: Option<thinkthen::AdmittedRequest>,
    prepared: &complete_native::Prepared,
    options: thinkthen::RequestOptions,
) -> Result<thinkthen::Request, thinkthen::Error> {
    if let Some(native) = native {
        let mut args = native.request().call.arguments().clone();
        args.options.batch = args.options.batch.or(options.batch);
        args.options.context = args.options.context.or(options.context);
        args.options.deadline_ms = Some(super::portable::due(
            options.deadline_ms.unwrap_or(-1),
            args.options.deadline_ms,
        ));
        args.options.attempts |= options.attempts;
        args.options.max_requests_total =
            match (args.options.max_requests_total, options.max_requests_total) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, b) => a.or(b),
            };
        Ok(thinkthen::Request::for_function(
            native.request().call.function(),
            args,
        ))
    } else {
        request::request(prepared, options)
    }
}

fn request_batch(value: String) -> thinkthen::RequestBatch {
    if value == "max" {
        thinkthen::RequestBatch::Named(value)
    } else {
        thinkthen::RequestBatch::Count(value.parse().unwrap_or_default())
    }
}
