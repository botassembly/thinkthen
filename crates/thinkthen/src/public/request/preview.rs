//! Canonical previews compose native records and prepare questions without execution.
use super::execution::{apply, controls, feed_projection, image_descriptors, image_route};
use super::{AdmittedRequest, RequestDefinition, RequestEnvironment, RequestFunction};
use crate::{Engine, Error, InputFunction, LoadedQuestion, PlanEstimate};

type Preview = (crate::core::PlanSummary, bool, usize, Vec<usize>);

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
        let (summary, dropped, _, _) = self.preview_request(request, environment, false)?;
        Ok((summary, dropped))
    }

    pub(crate) fn plan_annotation_request<'a>(
        &self,
        request: &'a AdmittedRequest,
        environment: RequestEnvironment<'a>,
    ) -> Result<(crate::core::PlanSummary, usize, Vec<usize>), Error> {
        let (summary, _, count, groups) = self.preview_request(request, environment, true)?;
        Ok((summary, count, groups))
    }

    fn preview_request<'a>(
        &self,
        request: &'a AdmittedRequest,
        environment: RequestEnvironment<'a>,
        annotation: bool,
    ) -> Result<Preview, Error> {
        let function = match request.request.call.function() {
            RequestFunction::Decide => InputFunction::Decide,
            RequestFunction::Choose => InputFunction::Choose,
            RequestFunction::Tag => InputFunction::Tag,
            RequestFunction::Score => InputFunction::Score,
            RequestFunction::Filter => InputFunction::Filter,
            RequestFunction::Rank => InputFunction::Rank,
            RequestFunction::Annotate if annotation => InputFunction::Annotate,
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
        let rows =
            request.records_for_plan(&reading_definition, environment, controls, image_refusal)?;
        let mut groups = annotation_groups(&definition, annotation)?;
        let mut dropped = false;
        let asks = rows.enumerate().map(|(at, row)| {
            controls.admission()?;
            let row = row.map_err(|error| error.at_record(at))?;
            self.check_record_limit(at)?;
            super::inline::validate_composed(&reading_definition, options, &row)
                .map_err(|error| error.at_record(at))?;
            if annotation && let RequestDefinition::Annotate(set) = &definition {
                return crate::public::complete::annotation_preview_asks(
                    &engine,
                    set,
                    row,
                    controls.context_text(),
                    at,
                )
                .map_err(|error| error.at_record(at));
            }
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
            Ok(asks.into_iter().map(|ask| (ask, 0_usize)).collect())
        });
        // Native record preparation embeds each resolved context in its own asks.
        let mut count = 0_usize;
        let summary = crate::public::plan::estimate_tagged_summary(
            &engine,
            setting,
            None,
            asks,
            |request| {
                count = count
                    .checked_add(1)
                    .ok_or_else(|| Error::usage("the planned input is too large to count"))?;
                count_groups(&mut groups, request)?;
                Ok(())
            },
        )?;
        Ok((summary, dropped, count, groups))
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

fn count_groups(groups: &mut [usize], items: &[usize]) -> Result<(), Error> {
    for group in items.iter().collect::<std::collections::BTreeSet<_>>() {
        if let Some(total) = groups.get_mut(*group) {
            *total = total
                .checked_add(1)
                .ok_or_else(|| Error::usage("the planned input is too large to count"))?;
        }
    }
    Ok(())
}

fn annotation_groups(
    definition: &RequestDefinition,
    annotation: bool,
) -> Result<Vec<usize>, Error> {
    if !annotation {
        return Ok(Vec::new());
    }
    match definition {
        RequestDefinition::Annotate(set) => Ok(vec![0; set.0.groups().len()]),
        _ => Err(Error::usage("annotation plan requires a question set")),
    }
}

impl AdmittedRequest {
    pub(crate) fn plan_find<'a>(
        &'a self,
        engine: &Engine,
        environment: RequestEnvironment<'a>,
    ) -> Result<(crate::core::Find, crate::core::PlanSummary), Error> {
        let (definition, rows, controls) = route_preview(self, environment)?;
        let question = match &definition {
            RequestDefinition::Find(file) => file.question(),
            RequestDefinition::Atomic(LoadedQuestion::Question(question)) => question,
            _ => return Err(Error::usage("find plan requires a find question")),
        };
        engine.preview_find_records(question, rows, controls)
    }
    pub(crate) fn plan_recognition<'a>(
        &'a self,
        backend: &crate::core::Backend,
        profile: Option<&crate::core::BackendProfile>,
        environment: RequestEnvironment<'a>,
        limit: usize,
    ) -> Result<crate::public::complete::recognize::RecognitionPreview, Error> {
        let (definition, rows, controls) = route_preview(self, environment)?;
        let ask = match &definition {
            RequestDefinition::Recognition(ask) => ask,
            RequestDefinition::Recognize(file) => file.question(),
            _ => {
                return Err(Error::usage(
                    "recognize plan requires a recognition question",
                ));
            }
        };
        crate::public::complete::recognize::preview_recognition(
            backend,
            profile,
            ask,
            rows,
            controls.context_text(),
            limit,
        )
    }
    pub(crate) fn plan_relations<'a>(
        &'a self,
        backend: &crate::core::Backend,
        profile: Option<&crate::core::BackendProfile>,
        environment: RequestEnvironment<'a>,
    ) -> Result<crate::public::complete::relate::RelationPreview, Error> {
        let (definition, rows, controls) = route_preview(self, environment)?;
        let RequestDefinition::Relate(ask) = definition else {
            return Err(Error::usage("relate plan requires a relation question"));
        };
        crate::public::complete::relate::preview_relations(backend, profile, &ask, rows, &controls)
    }
}

fn route_preview<'a>(
    request: &'a AdmittedRequest,
    environment: RequestEnvironment<'a>,
) -> Result<
    (
        RequestDefinition,
        super::composition::Inputs<'a>,
        crate::CallOptions<'a>,
    ),
    Error,
> {
    feed_projection(request, environment.feed.as_ref())?;
    let options = &request.request.call.arguments().options;
    let controls = controls(options, environment.controls)?.started()?;
    controls.admission()?;
    let mut definition = request.resolve_question()?;
    request.admit_inline(&definition)?;
    let reading = definition.clone();
    apply(&mut definition, options)?;
    let rows = request.records(&reading, environment, controls, None)?;
    let rows = rows.enumerate().map(move |(at, row)| {
        controls.admission()?;
        let row = row?;
        super::inline::validate_composed(&reading, options, &row)
            .map_err(|error| error.at_record(at))?;
        Ok(row)
    });
    Ok((definition, Box::new(rows), controls))
}
