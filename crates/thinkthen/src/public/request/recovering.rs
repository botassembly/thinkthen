//! Explicit native recovery leaves ordinary results and packet serialization intact.
use super::{
    AdmittedRequest, RequestEnvironment, RequestFunction, RequestInput, RequestOutcome,
    RequestRecoveredRow,
};
use crate::{Call, Engine, Error};

impl Engine {
    /// Execute independent record rows while retaining recoverable row failures.
    /// Whole-set rank, find and relate produce one aggregate outcome. Cancellation,
    /// deadlines and defects terminate the invocation after workers join.
    /// # Errors
    /// Refuses malformed admission or call limits before sending, and returns fatal failures.
    pub fn execute_request_recovering<'a>(
        &self,
        request: &'a AdmittedRequest,
        mut environment: RequestEnvironment<'a>,
    ) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
        let function = request.request.call.function();
        if matches!(
            function,
            RequestFunction::Rank | RequestFunction::Find | RequestFunction::Relate
        ) {
            return self
                .recover_aggregate(request, environment)
                .map(|call| super::settlement::recovered(request, call));
        }
        super::execution::feed_projection(request, environment.feed.as_ref())?;
        environment.controls.recovery_rows = true;
        super::execution::execute(
            self,
            request,
            environment.controls,
            environment
                .feed
                .as_ref()
                .is_some_and(|feed| feed.image_inputs),
            |prepared| {
                request.records(
                    &prepared.reading,
                    environment,
                    prepared.controls,
                    prepared.image_refusal.clone(),
                )
            },
            |prepared, rows| {
                let mut records = Vec::new();
                for row in rows {
                    prepared.controls.admission()?;
                    prepared.engine.check_record_limit(records.len())?;
                    records.push(Ok(row?));
                }
                let mut controls = prepared.controls;
                if let Some(context) = &prepared.context {
                    controls = controls.context(context);
                }
                dispatch(
                    &prepared.engine,
                    function,
                    prepared.definition,
                    records,
                    controls,
                )
            },
        )
        .map(|call| super::settlement::recovered(request, call))
    }
    fn recover_aggregate<'a>(
        &self,
        request: &'a AdmittedRequest,
        environment: RequestEnvironment<'a>,
    ) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
        match self.execute_request(request, environment) {
            Ok(RequestOutcome::Complete(call)) => Ok(call.map(|value| {
                vec![RequestRecoveredRow {
                    ordinal: 0,
                    outcome: Ok(value),
                    cause: None,
                }]
            })),
            Ok(RequestOutcome::Failed { error, .. }) | Err(error) => aggregate_failure(error),
        }
    }
}

fn dispatch(
    engine: &Engine,
    function: RequestFunction,
    definition: super::RequestDefinition,
    records: Vec<Result<crate::RecordInput<crate::QuestionInput>, Error>>,
    controls: crate::CallOptions<'_>,
) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
    match function {
        RequestFunction::Decide
        | RequestFunction::Choose
        | RequestFunction::Score
        | RequestFunction::Tag
        | RequestFunction::Filter => {
            if let super::RequestDefinition::DynamicChoose(question) = &definition {
                return engine.recover_dynamic_choose(question, records, controls);
            }
            let question = definition.atomic()?;
            engine.recover_atomic(function.input_function(), &question, records, controls)
        }
        RequestFunction::Annotate => {
            let super::RequestDefinition::Annotate(questions) = &definition else {
                return Err(Error::defect("an annotation lost its question set"));
            };
            engine.recover_annotations(questions, records, controls)
        }
        RequestFunction::Recognize => {
            let ask = definition.recognition()?;
            engine.recover_recognition(&ask, records, controls)
        }
        _ => Err(Error::defect(
            "recovering dispatch has no scheduler for this function",
        )),
    }
}

fn aggregate_failure(error: Error) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
    if super::recovered::fatal(&error) {
        return Err(error);
    }
    match error.facts().cloned() {
        Some(facts) => Ok(Call::new(
            vec![RequestRecoveredRow {
                ordinal: 0,
                outcome: Err(error.normalize_recovery_budget()),
                cause: None,
            }],
            facts,
        )),
        None => Err(error),
    }
}

impl Engine {
    /// Deliver recovering rows incrementally through the canonical request scheduler.
    /// # Errors
    /// Producer, reader and admission failures terminate; native stops join workers.
    pub fn execute_request_recovering_sink<'a>(
        &self,
        request: &'a AdmittedRequest,
        mut environment: RequestEnvironment<'a>,
        sink: &dyn Fn(RequestRecoveredRow),
    ) -> Result<Call<()>, Error> {
        let function = request.request.call.function();
        let eager = environment.feed.as_ref().is_some_and(|feed| feed.eager)
            || !matches!(
                request.request.call.arguments().input,
                RequestInput::Source { .. } | RequestInput::Feed { .. }
            );
        if eager
            || matches!(
                function,
                RequestFunction::Rank | RequestFunction::Find | RequestFunction::Relate
            )
        {
            let call = self.execute_request_recovering(request, environment)?;
            return Ok(call.map(|rows| rows.into_iter().for_each(sink)));
        }
        super::execution::feed_projection(request, environment.feed.as_ref())?;
        environment.controls.recovery_rows = true;
        super::execution::execute(
            self,
            request,
            environment.controls,
            environment
                .feed
                .as_ref()
                .is_some_and(|feed| feed.image_inputs),
            |prepared| {
                request.records(
                    &prepared.reading,
                    environment,
                    prepared.controls,
                    prepared.image_refusal.clone(),
                )
            },
            |prepared, records| {
                let dynamic = match &prepared.definition {
                    super::RequestDefinition::DynamicChoose(question) => Some(question.clone()),
                    _ => None,
                };
                let batch = match (function, dynamic) {
                    (RequestFunction::Choose, Some(question)) => {
                        prepared.engine.recover_dynamic_choose_stream(
                            question,
                            records,
                            prepared.controls,
                            prepared.context,
                        )?
                    }
                    (RequestFunction::Annotate, _) => prepared.engine.recover_annotations_stream(
                        prepared.definition.annotation()?,
                        records,
                        prepared.controls,
                        prepared.context,
                    )?,
                    (RequestFunction::Recognize, _) => {
                        let ask = prepared.definition.recognition()?;
                        let controls = match &prepared.context {
                            Some(context) => prepared.controls.context(context),
                            None => prepared.controls,
                        };
                        return prepared
                            .engine
                            .recover_recognition_stream(&ask, records, controls, sink);
                    }
                    _ => prepared.engine.recover_atomic_stream(
                        function.input_function(),
                        prepared.definition.atomic()?,
                        records,
                        prepared.controls,
                        prepared.context,
                    )?,
                };
                let mut batch = batch;
                for row in batch.by_ref() {
                    sink(row?);
                }
                let facts = batch
                    .facts()
                    .cloned()
                    .ok_or_else(|| Error::defect("a recovered stream has no final facts"))?;
                Ok(Call::new((), facts))
            },
        )
    }
}
