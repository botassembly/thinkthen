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
        crate::public::plan::from_summary(self.plan_request_summary(request, environment)?.0)
    }

    pub(crate) fn plan_request_summary<'a>(
        &self,
        request: &'a AdmittedRequest,
        environment: RequestEnvironment<'a>,
    ) -> Result<(crate::core::PlanSummary, bool), Error> {
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
        let (engine, setting) = configuration(self, &definition, &controls)?;
        if controls.cli_reader.is_some()
            && let Some(context) = controls.context_text()
        {
            crate::public::complete::records::cli_context(&engine, context)?;
        }
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
        let rows = request.records(&reading_definition, environment, controls, image_refusal)?;
        let mut dropped = false;
        let asks = rows.enumerate().map(|(at, row)| {
            controls.admission()?;
            let row = row.map_err(|error| error.at_record(at))?;
            self.check_record_limit(at)?;
            super::inline::validate_composed(&reading_definition, options, &row)
                .map_err(|error| error.at_record(at))?;
            let (asks, loses_detail) = crate::public::complete::preview_definition_asks(
                &engine,
                function,
                &definition,
                row,
                &controls,
                at,
            )
            .map_err(|error| error.at_record(at))?;
            dropped |= loses_detail;
            Ok(asks)
        });
        // Native record preparation embeds each resolved context in its own asks.
        let summary = crate::public::plan::estimate_summary(&engine, setting, None, asks)?;
        Ok((summary, dropped))
    }
}

fn configuration(
    engine: &Engine,
    definition: &RequestDefinition,
    controls: &crate::CallOptions<'_>,
) -> Result<
    (
        std::sync::Arc<crate::engine::facade::Engine>,
        crate::core::Setting,
    ),
    Error,
> {
    use crate::public::bulk::{selected_batch, selected_batch_file, selected_set_batch};
    Ok(match definition {
        RequestDefinition::Atomic(LoadedQuestion::Question(question))
        | RequestDefinition::Rank(question) => (
            engine.asking(question)?,
            selected_batch(question, controls, engine.batch)?,
        ),
        RequestDefinition::Atomic(LoadedQuestion::Banded(question)) => (
            engine.asking(&question.0)?,
            selected_batch(&question.0, controls, engine.batch)?,
        ),
        RequestDefinition::RankSet(set) => (
            std::sync::Arc::clone(&engine.inner),
            selected_set_batch(&set.0, controls, engine.batch)?,
        ),
        RequestDefinition::Annotate(set) => (
            std::sync::Arc::clone(&engine.inner),
            selected_set_batch(&set.0, controls, engine.batch)?,
        ),
        RequestDefinition::DynamicChoose(question) => (
            engine.for_model(question.model.as_ref())?,
            selected_batch_file(question.batch.as_ref(), controls, engine.batch)?,
        ),
        _ => return Err(Error::usage("plan requires a record question")),
    })
}
