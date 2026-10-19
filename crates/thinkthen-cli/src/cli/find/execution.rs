//! Whole-set find retains public native settlement while polling the command signal.
use super::{Common, Environment, Failure, Unit};

pub(super) fn execute(
    admitted: crate::AdmittedRequest, engine: crate::Engine, common: &Common,
    environment: &Environment, units: &[Unit], context: Option<&str>,
) -> Result<crate::CompleteFound<crate::QuestionInput>, Failure> {
    let mut request = super::request(&admitted, "cli-find", context)?;
    // Attempts are command presentation; the native owner retains all facts.
    let mut arguments = request.call.arguments().clone();
    arguments.options.attempts = common.details;
    request = crate::Request::for_function(crate::RequestFunction::Find, arguments);
    let tally = environment.tally().start();
    let session = engine.request_session_with_feed_options(request, crate::Surface::Cli,
        crate::RequestSessionFeedOptions { retain_success_on_cancel: true,
            record_reading: Some(super::composition(common)?), ..Default::default() })?;
    let mut tally = Some(tally);
    let mut next = units.iter();
    let mut input_done = false;
    let mut cancelled = false;
    let mut found = None;
    let mut failure = None;
    let mut terminal = None;
    loop {
        if !cancelled && (environment.cancel().fired() || failure.is_some()) {
            session.cancel(); cancelled = true; input_done = true;
        }
        match session.try_read() {
            crate::RequestSessionRead::Result(packet) => {
                match packet {
                    crate::RequestSessionResult::Aggregate(crate::RequestValue::Found(value)) => { found = Some(value); }
                    crate::RequestSessionResult::Observation { .. } => {}
                    crate::RequestSessionResult::Terminal(value) => {
                        if let Some(facts) = &value.facts {
                            environment.settle_native(facts);
                            if let Some(start) = tally.take() { start.finish(facts)?; }
                        }
                        terminal = Some(value.error.map_or(Ok(()), Err));
                    }
                    _ => { failure.get_or_insert(Failure::Defect("find returned another native result")); }
                }
                continue;
            }
            crate::RequestSessionRead::End => break,
            crate::RequestSessionRead::Pending => {}
        }
        if !input_done {
            match session.try_push_with(|| next.next().ok_or(Input::End).and_then(descriptor)) {
                // Conversion is requested only when intake has room. EOF has no
                // descriptor to transfer and closes the existing feed once.
                Ok(crate::RequestSessionPushStatus::Accepted) => continue,
                Ok(crate::RequestSessionPushStatus::Full) => {}
                Ok(crate::RequestSessionPushStatus::Closed) => input_done = true,
                Err(error) => match error {
                    Input::End => { session.finish(None)?; input_done = true; }
                    Input::Failed(error) => { session.finish_native_reader_error(error)?; input_done = true; }
                },
            }
        }
        let interest = if cancelled { crate::RequestSessionWaitInterest::Settlement }
            else if input_done { crate::RequestSessionWaitInterest::Result }
            else { crate::RequestSessionWaitInterest::Input };
        if session.wait(interest, std::time::Duration::from_millis(50)) == crate::RequestSessionReady::End { break; }
    }
    let result = terminal.ok_or(Failure::Defect("native find ended without settlement"))?;
    if let Some(failure) = failure { return Err(failure); }
    result.map_err(|error| Failure::from(error)
        .with_replay_context(crate::failure::ReplayContext::FindSet(units.len()))
        .with_size_advice(crate::failure::SizeAdvice::Candidates))?;
    found.ok_or(Failure::Defect("find settled without a native result"))
}

enum Input { End, Failed(crate::Error) }
impl From<crate::Error> for Input { fn from(error: crate::Error) -> Self { Self::Failed(error) } }
fn descriptor(unit: &Unit) -> Result<crate::RequestSessionDescriptor, Input> {
    let original = Some(match unit.record.literal() {
        Some(text) => crate::RequestOriginal::Text { text: text.to_owned() },
        None => crate::RequestOriginal::Json { value: unit.record.clone() },
    });
    let location = if unit.position.located {
        unit.position.file.as_ref().map(|file| crate::SourceLocation::new(file.clone(), unit.position.first, unit.position.last)).transpose()?
    } else { None };
    Ok(crate::RequestSessionDescriptor { item: crate::RequestItem { original, images: Vec::new(),
        context: None, options: None, seed_spans: None, examples: None }, location })
}
