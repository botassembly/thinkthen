//! Explicit native recovery leaves ordinary results and packet serialization intact.
use super::{
    AdmittedRequest, RequestEnvironment, RequestFunction, RequestOutcome, RequestRecoveredRow,
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
            return self.recover_aggregate(request, environment);
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
                }]
            })),
            Ok(RequestOutcome::Failed { error, .. }) | Err(error) => aggregate_failure(self, error),
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

fn aggregate_failure(
    engine: &Engine,
    error: Error,
) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
    if super::recovered::fatal(&error) {
        return Err(error);
    }
    if let Some(facts) = error.facts().cloned() {
        return Ok(Call::new(
            vec![RequestRecoveredRow {
                ordinal: 0,
                outcome: Err(error),
            }],
            facts,
        ));
    }
    let stop =
        crate::public::options::Stop::begin(crate::CallOptions::new())?.with_prices(engine.prices);
    stop.run_call(0, |_| {
        Ok(vec![RequestRecoveredRow {
            ordinal: 0,
            outcome: Err(error),
        }])
    })
}
