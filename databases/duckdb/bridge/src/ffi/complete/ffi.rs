//! Counted named complete calls share the session engine and final native facts.
use super::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, batch, probe, reply_boundary,
    run_detached, text,
};
use crate::{complete_native, engines};

/// Execute one named complete call; C++ retains readable ranges through return.
/// # Safety
/// All counted ranges and the stop callback follow the existing bridge contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete(
    verb: BridgeText,
    question: BridgeText,
    inputs: BridgeText,
    settings: BridgeText,
    from_file: i32,
    deadline: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let verb = text(verb.bytes, verb.len)?;
        let question = text(question.bytes, question.len)?;
        let inputs = text(inputs.bytes, inputs.len)?;
        let raw = text(settings.bytes, settings.len)?;
        let call = thinkthen::Settings::parse(raw)
            .map_err(|e| crate::errors::RowError::usage(&e.to_string()).text)?;
        let parsed = (|| {
            Ok::<_, thinkthen::Error>((
                if from_file != 0 {
                    complete_native::settings::prepare_file(verb, question, &call)?
                } else {
                    complete_native::prepare(verb, question, &call)?
                },
                complete_native::Inputs::parse(inputs, false)?,
            ))
        })();
        let (prepared, inputs) = match parsed {
            Ok(value) => value,
            Err(error) => return Ok(complete_native::failure(&error).to_string().into_bytes()),
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
            Ok(complete_native::run(
                &engine,
                &prepared,
                inputs,
                options,
                thinkthen::Surface::Duckdb,
            )
            .to_string()
            .into_bytes())
        })
    })
}
