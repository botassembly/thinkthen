//! Frozen envelope translation retains legacy parsing and diagnostics at its edge.
use super::{Request as LegacyRequest, alone, flag, member, object};
use crate::failures::Failure;
use thinkthen::{
    AdmittedRequest, LoadedQuestion, Question, QuestionInput, QuestionSet, RawRecord, Request,
    RequestArguments, RequestBatch, RequestCall, RequestDefinition, RequestInput, RequestItem,
    RequestOptions, RequestOriginal, RequestQuestion,
};

pub(super) fn translate(request: &LegacyRequest) -> Result<AdmittedRequest, Failure> {
    let definition = definition(request)?;
    refuse_banded_filter(request, &definition)?;
    let input = if let Some(source) = request.envelope.get("source") {
        RequestInput::Source {
            source: super::source::parse(source.get())?.request_source(),
        }
    } else if matches!(
        request.verb.as_str(),
        "rank" | "filter" | "annotate" | "relate"
    ) || request.envelope.contains_key("records")
    {
        if request.envelope.contains_key("evidence") {
            return Err(Failure::usage(
                "one request takes evidence or records, not both",
            ));
        }
        let items = if request.verb == "relate" {
            let rows = member(request, "records", |raw| {
                serde_json::from_str::<Vec<Box<serde_json::value::RawValue>>>(raw)
            })?;
            rows.iter()
                .map(|raw| {
                    RawRecord::json(raw.get()).map(|value| item(RequestOriginal::Json { value }))
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            member(request, "records", |raw| {
                serde_json::from_str::<Vec<String>>(raw)
            })?
            .into_iter()
            .map(|text| string_record(text, request.verb == "annotate"))
            .collect::<Result<Vec<_>, Failure>>()?
        };
        if request.verb == "relate" {
            RequestInput::Entities { items }
        } else {
            RequestInput::Records { items }
        }
    } else if request.verb == "find" {
        let items = member(request, "units", |raw| {
            serde_json::from_str::<Vec<String>>(raw)
        })?
        .into_iter()
        .map(|text| item(RequestOriginal::Text { text }))
        .collect();
        RequestInput::Units { items }
    } else {
        RequestInput::Text {
            text: member(request, "evidence", |raw| {
                serde_json::from_str::<String>(raw)
            })?,
            images: Vec::new(),
        }
    };
    let mut options = RequestOptions {
        attempts: request.attempts,
        details: flag(&request.envelope, "details")?,
        ..RequestOptions::default()
    };
    if let Some(call) = &request.call {
        options.context = call.context.clone();
        options.batch = call.batch.map(|batch| match batch {
            thinkthen::BatchSetting::Records(n) => RequestBatch::Count(n.get()),
            thinkthen::BatchSetting::Max => RequestBatch::Named("max".to_owned()),
        });
    }
    let args = RequestArguments {
        question: RequestQuestion::Definition { value: definition },
        input,
        options,
    };
    let call = match request.verb.as_str() {
        "decide" => RequestCall::Decide(args),
        "choose" => RequestCall::Choose(args),
        "tag" => RequestCall::Tag(args),
        "score" => RequestCall::Score(args),
        "filter" => RequestCall::Filter(args),
        "rank" => RequestCall::Rank(args),
        "find" => RequestCall::Find(args),
        "annotate" => RequestCall::Annotate(args),
        "recognize" => RequestCall::Recognize(args),
        _ => RequestCall::Relate(args),
    };
    Ok(Request::new(call).admit()?)
}
fn string_record(text: String, annotate: bool) -> Result<RequestItem, Failure> {
    if !annotate {
        return Ok(item(RequestOriginal::Text { text }));
    }
    let QuestionInput::Record(record) = QuestionInput::annotation_document(&text)? else {
        return Err(Failure::defect(
            "an annotation document has no original record",
        ));
    };
    let value = record.original();
    Ok(item(if value.literal().is_some() {
        RequestOriginal::Text { text }
    } else {
        RequestOriginal::Json {
            value: value.clone(),
        }
    }))
}
fn item(original: RequestOriginal) -> RequestItem {
    RequestItem {
        original: Some(original),
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
        images: Vec::new(),
    }
}

fn definition(request: &LegacyRequest) -> Result<RequestDefinition, Failure> {
    let mut question = request.question.clone();
    if let Some(text) = question.remove("filter") {
        question.insert("decide".to_owned(), text);
    }
    let definition = match request.verb.as_str() {
        "rank" => Question::rank(&alone(&request.verb, &question)?)?.into(),
        "find" => {
            let none = question.remove("none").map_or(Ok(false), |raw| {
                serde_json::from_str::<bool>(raw.get())
                    .map_err(|_| Failure::usage("find takes `none` as true or false"))
            })?;
            let asked = Question::find(&alone(&request.verb, &question)?)?;
            RequestDefinition::Atomic(LoadedQuestion::Question(if none {
                asked.offering_none()?
            } else {
                asked
            }))
        }
        "annotate" => match question.get("annotate") {
            Some(set) if question.len() == 1 => QuestionSet::from_json(set.get())?.into(),
            _ => return Err(Failure::usage("annotate takes its question set alone")),
        },
        "recognize" => {
            RequestDefinition::Recognition(thinkthen::Recognize::from_json(&object(&question)?)?)
        }
        "relate" => RequestDefinition::Relate(thinkthen::Relate::from_json(&object(&question)?)?),
        _ => Question::from_json(&object(&question)?)?.into(),
    };
    Ok(definition)
}

fn refuse_banded_filter(
    request: &LegacyRequest,
    definition: &RequestDefinition,
) -> Result<(), Failure> {
    if request.verb == "filter"
        && matches!(
            definition,
            RequestDefinition::Atomic(LoadedQuestion::Banded(_))
        )
    {
        return Err(Failure::usage("filter keeps a record at a cut, not a band"));
    }
    Ok(())
}
