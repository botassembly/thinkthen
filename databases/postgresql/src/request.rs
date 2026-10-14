//! PostgreSQL projections execute the same admitted typed request as complete calls.
use thinkthen::{
    Call, CallOptions, Engine, Error, Request, RequestArguments, RequestDefinition,
    RequestEnvironment, RequestFunction, RequestInput, RequestItem, RequestOptions,
    RequestOriginal, RequestOutcome, RequestQuestion, RequestValue,
};

pub(crate) fn text(text: String) -> RequestItem {
    RequestItem {
        original: Some(RequestOriginal::Text { text }),
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
        images: Vec::new(),
    }
}

pub(crate) fn json(value: serde_json::Value) -> Result<RequestItem, Error> {
    let mut item = text(String::new());
    item.original = Some(RequestOriginal::Json {
        value: thinkthen::RawRecord::json(&value.to_string())?,
    });
    Ok(item)
}

pub(crate) fn document(text: String) -> Result<RequestItem, Error> {
    match thinkthen::QuestionInput::annotation_document(&text)? {
        thinkthen::QuestionInput::Record(record) => {
            let mut item = self::text(String::new());
            item.original = Some(RequestOriginal::Json {
                value: record.original().clone(),
            });
            Ok(item)
        }
        _ => Ok(self::text(text)),
    }
}

pub(crate) fn run(
    engine: &Engine,
    function: RequestFunction,
    definition: RequestDefinition,
    items: Vec<RequestItem>,
    controls: CallOptions<'_>,
) -> Result<Call<RequestValue>, Error> {
    let args = RequestArguments {
        question: RequestQuestion::Definition { value: definition },
        input: RequestInput::Records { items },
        options: RequestOptions::default(),
    };
    let request = Request::for_function(function, args).admit()?;
    match engine.execute_request(
        &request,
        RequestEnvironment {
            controls: controls.surface(thinkthen::Surface::Postgresql),
            feed: None,
        },
    )? {
        RequestOutcome::Complete(call) => Ok(call),
        RequestOutcome::Failed { error, .. } => Err(error),
    }
}

pub(crate) fn wrong_result() -> Error {
    crate::call::defect("the admitted request returned another function's result")
}

pub(crate) fn details(
    engine: &Engine,
    question: &thinkthen::LoadedQuestion,
    items: Vec<RequestItem>,
    options: CallOptions<'_>,
) -> Result<Vec<JudgmentRow>, Error> {
    let function = RequestFunction::for_definition(&question.clone().into());
    let call = run(engine, function, question.clone().into(), items, options)?;
    macro_rules! project {
        ($rows:expr, $value:expr) => {
            $rows
                .into_iter()
                .map(|row| {
                    let result = row.result();
                    Ok(JudgmentRow {
                        value: $value(result),
                        probabilities: result.probabilities(),
                        json: result.to_scalar_json()?,
                    })
                })
                .collect()
        };
    }
    match call.into_value() {
        RequestValue::Decisions(rows) => project!(rows, |r: &thinkthen::CompleteDecision| {
            thinkthen::Judgment::Decision(r.value())
        }),
        RequestValue::Choices(rows) => project!(rows, |r: &thinkthen::CompleteChoice| {
            thinkthen::Judgment::Choice(r.value().map(str::to_owned))
        }),
        RequestValue::Tags(rows) => project!(rows, |r: &thinkthen::CompleteTags| {
            thinkthen::Judgment::Tags(r.value().to_vec())
        }),
        RequestValue::Scores(rows) => project!(rows, |r: &thinkthen::CompleteScore| {
            thinkthen::Judgment::Score(r.value())
        }),
        _ => Err(wrong_result()),
    }
}

/// Retain the typed question receipt while keeping ordinary details scalar-only.
pub(crate) fn receipt(
    engine: &Engine,
    question: &thinkthen::LoadedQuestion,
    input: String,
    options: CallOptions<'_>,
) -> Result<serde_json::Value, Error> {
    let receipts = std::sync::Mutex::new(Vec::new());
    let function = RequestFunction::for_definition(&question.clone().into());
    let observer = |event: thinkthen::RecordObservation<'_>| {
        if matches!(event, thinkthen::RecordObservation::Question { .. }) {
            let packet = thinkthen::RequestSessionResult::Observation {
                function,
                value: event.to_owned(),
            };
            receipts
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(packet.to_json());
        }
    };
    run(
        engine,
        function,
        question.clone().into(),
        vec![text(input)],
        options.observe(&observer),
    )?;
    let held = receipts
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .into_iter()
        .next()
        .ok_or_else(|| crate::call::defect("details returned no receipt"))??;
    let mut packet: serde_json::Value = serde_json::from_str(&held)
        .map_err(|_| crate::call::defect("details receipt could not be projected"))?;
    let mut detail = packet
        .get_mut("value")
        .and_then(|value| value.get_mut("detail"))
        .ok_or_else(|| crate::call::defect("details receipt is invalid"))?
        .take();
    let object = detail
        .as_object_mut()
        .ok_or_else(|| crate::call::defect("details receipt is invalid"))?;
    object.remove("input");
    object.remove("inputs");
    Ok(detail)
}

/// SQL-owned ordinary judgment columns and their existing scalar document.
pub(crate) struct JudgmentRow {
    pub(crate) value: thinkthen::Judgment,
    pub(crate) probabilities: thinkthen::Probabilities,
    pub(crate) json: String,
}
