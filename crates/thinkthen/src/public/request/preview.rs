//! Canonical previews compose native records and prepare questions without execution.
use super::execution::{apply, controls, feed_projection, image_descriptors, image_route};
use super::{AdmittedRequest, RequestDefinition, RequestEnvironment, RequestFunction};
use crate::{Engine, Error, InputFunction, LoadedQuestion, PlanEstimate};

impl Engine {
    /// Preview an admitted atomic or single-question rank request without sending.
    /// Explicit sources and feeds use the same composition and admission as execution.
    /// No key, cache or answer is read. Unsupported functions refuse before input advances.
    /// # Errors
    /// Retains native question, record, caller-control and packing refusals.
    pub fn plan_request<'a>(
        &self,
        request: &'a AdmittedRequest,
        environment: RequestEnvironment<'a>,
    ) -> Result<PlanEstimate, Error> {
        let function = match request.request.call.function() {
            RequestFunction::Decide => InputFunction::Decide,
            RequestFunction::Choose => InputFunction::Choose,
            RequestFunction::Tag => InputFunction::Tag,
            RequestFunction::Score => InputFunction::Score,
            RequestFunction::Filter => InputFunction::Filter,
            RequestFunction::Rank => InputFunction::Rank,
            _ => return Err(Error::usage("plan requires an atomic or rank question")),
        };
        feed_projection(request, environment.feed.as_ref())?;
        let options = &request.request.call.arguments().options;
        let controls = controls(options, environment.controls)?.started()?;
        controls.admission()?;
        let mut definition = request.resolve_question()?;
        request.admit_inline(&definition)?;
        let reading_definition = definition.clone();
        apply(&mut definition, options)?;
        controls.admission()?;
        let question = match &definition {
            RequestDefinition::Atomic(LoadedQuestion::Question(q)) | RequestDefinition::Rank(q) => {
                q
            }
            RequestDefinition::Atomic(LoadedQuestion::Banded(q)) => &q.0,
            _ => {
                return Err(Error::usage(
                    "plan requires a fixed atomic or rank question",
                ));
            }
        };
        let engine = self.asking(question)?;
        let image_refusal = image_route(self, &definition)
            .err()
            .map(|error| error.detail().message().to_owned());
        if request.attachment_limit.is_none()
            && (image_descriptors(&request.request.call.arguments().input)
                || environment
                    .feed
                    .as_ref()
                    .is_some_and(|feed| feed.image_inputs))
            && let Some(message) = &image_refusal
        {
            return Err(Error::usage(message.clone()));
        }
        let setting = crate::public::bulk::selected_batch(question, &controls, self.batch)?;
        let rows = request.records(&reading_definition, environment, controls, image_refusal)?;
        let asks = rows.enumerate().map(|(at, row)| {
            controls.admission()?;
            let row = row.map_err(|error| error.at_record(at))?;
            self.check_record_limit(at)?;
            super::inline::validate_composed(&reading_definition, options, &row)
                .map_err(|error| error.at_record(at))?;
            crate::public::complete::preview_asks(
                &engine,
                function,
                question,
                row,
                controls.context_text(),
                at,
            )
            .map_err(|error| error.at_record(at))
        });
        // Native record preparation embeds each resolved context in its own asks.
        crate::public::plan::estimate(&engine, setting, None, asks)
    }
}
