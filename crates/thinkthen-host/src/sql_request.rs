//! SQLite host arguments enter the native shared Request without serialization.
use crate::complete_native::{Inputs, Prepared, defect};
use serde_json::{Value, json};
use std::sync::Mutex;
use thinkthen::{
    AdmittedRequest, CallOptions, Engine, Error, LoadedQuestion, RecordObservation, Request,
    RequestArguments, RequestDefinition, RequestEnvironment, RequestFeed, RequestInput,
    RequestOptions, RequestOutcome, RequestQuestion, RequestValue, Surface,
};

pub fn run(
    engine: &Engine,
    prepared: &Prepared,
    request: &AdmittedRequest,
    inputs: Inputs,
    options: CallOptions<'_>,
    surface: Surface,
) -> Value {
    let cancelled = thinkthen::CancelToken::new();
    if inputs.cancelled {
        cancelled.cancel();
    }
    let events = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| {
        events
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(
                crate::complete_native::observations::event(&event)
                    .unwrap_or_else(|error| crate::complete_native::failure(&error)),
            );
    };
    let options = options
        .surface(surface)
        .attempts(inputs.attempts)
        .observe(&observer);
    let options = if inputs.cancelled {
        options.cancel(&cancelled)
    } else {
        options
    };
    let result = (|| {
        let reading = request.record_reading()?;
        let records = inputs.deferred_records(prepared.reading().or(Some(&reading)));
        let feed = RequestFeed::from_records("sqlite", records);
        let feed = if matches!(prepared, Prepared::Filter(_)) {
            feed.with_all_filter_results()
        } else {
            feed
        };
        let feed = if inputs.image_inputs()? {
            feed.with_image_inputs()
        } else {
            feed
        };
        let feed = if inputs.incremental {
            feed
        } else {
            feed.eager()
        };
        let outcome = engine.execute_request(
            request,
            RequestEnvironment {
                controls: options,
                feed: Some(feed),
            },
        )?;
        render(outcome)
    })();
    let native = result.unwrap_or_else(|error| crate::complete_native::failure(&error));
    let events = events
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    crate::complete_native::carrier(native, Value::Array(events))
}

pub fn definition(prepared: &Prepared) -> RequestDefinition {
    match prepared {
        Prepared::Atomic(q) => RequestDefinition::Atomic(q.clone()),
        Prepared::Dynamic(q) => RequestDefinition::DynamicChoose(q.clone()),
        Prepared::Filter(q) | Prepared::Find(q, _) => q.clone().into(),
        Prepared::Rank(q) => RequestDefinition::Rank(q.clone()),
        Prepared::RankSet(q) => q.clone().into(),
        Prepared::Annotate(q) => q.clone().into(),
        Prepared::Recognize(q, _) => q.clone().into(),
        Prepared::Relate(q) => q.clone().into(),
    }
}

pub fn admit(prepared: &Prepared) -> Result<AdmittedRequest, Error> {
    request(prepared, RequestOptions::default())?.admit()
}

pub fn request(prepared: &Prepared, options: RequestOptions) -> Result<Request, Error> {
    let definition = definition(prepared);
    let args = RequestArguments {
        question: RequestQuestion::Definition { value: definition },
        input: RequestInput::Feed {
            name: "sqlite".to_owned(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
        options,
    };
    let function = match prepared {
        Prepared::Atomic(LoadedQuestion::Banded(_)) => thinkthen::RequestFunction::Decide,
        Prepared::Atomic(LoadedQuestion::Question(q)) => match q.kind() {
            thinkthen::QuestionKind::Decide => thinkthen::RequestFunction::Decide,
            thinkthen::QuestionKind::Choose => thinkthen::RequestFunction::Choose,
            thinkthen::QuestionKind::Tag => thinkthen::RequestFunction::Tag,
            thinkthen::QuestionKind::Score => thinkthen::RequestFunction::Score,
            _ => return Err(defect()),
        },
        Prepared::Dynamic(_) => thinkthen::RequestFunction::Choose,
        Prepared::Filter(_) => thinkthen::RequestFunction::Filter,
        Prepared::Rank(_) | Prepared::RankSet(_) => thinkthen::RequestFunction::Rank,
        Prepared::Find(_, _) => thinkthen::RequestFunction::Find,
        Prepared::Annotate(_) => thinkthen::RequestFunction::Annotate,
        Prepared::Recognize(_, _) => thinkthen::RequestFunction::Recognize,
        Prepared::Relate(_) => thinkthen::RequestFunction::Relate,
    };
    Ok(Request::for_function(function, args))
}
fn render(outcome: RequestOutcome) -> Result<Value, Error> {
    match outcome {
        RequestOutcome::Complete(call) => {
            let mut value = serde_json::to_value(&call).map_err(|_| defect())?;
            supplement(&mut value, call.value())?;
            Ok(value)
        }
        RequestOutcome::Failed { completed, error } => {
            let mut value = crate::complete_native::failure(&error);
            supplement(&mut value, &completed)?;
            crate::complete_native::put(
                &mut value,
                "completed",
                serde_json::to_value(completed).map_err(|_| defect())?,
            )?;
            Ok(value)
        }
    }
}
pub fn supplement(value: &mut Value, result: &RequestValue) -> Result<(), Error> {
    macro_rules! rows {
        ($rows:expr) => {
            crate::complete_native::put(
                value,
                "ordinals",
                json!($rows.iter().map(|row| row.ordinal()).collect::<Vec<_>>()),
            )
        };
    }
    match result {
        RequestValue::Decisions(v) => rows!(v),
        RequestValue::Choices(v) => rows!(v),
        RequestValue::Tags(v) => rows!(v),
        RequestValue::Scores(v) => rows!(v),
        RequestValue::Filtered(v) => rows!(v),
        RequestValue::Ranked(v) => rows!(v),
        RequestValue::SetRanked(v) => rows!(v),
        RequestValue::Annotations(v) => rows!(v),
        RequestValue::Recognized(v) => rows!(v),
        RequestValue::Found(v) => crate::complete_native::put(
            value,
            "selection",
            json!(match v.selection() {
                thinkthen::FindSelection::None => None,
                thinkthen::FindSelection::Unit(at) => Some(at),
            }),
        ),
        RequestValue::Related(_) => Ok(()),
    }
}
