//! Frozen replies project completed native values without asking again.
use super::{Request, flag, raw, written};
use crate::failures::Failure;
use serde::Serialize;
use serde_json::{Value, json};
use thinkthen::{Answer, CompleteRecord, Probabilities, QuestionInput, RequestValue};

pub(super) fn value(value: &RequestValue, request: &Request) -> Result<String, Failure> {
    let detailed = flag(&request.envelope, "details")?;
    let located = request.envelope.contains_key("source");
    let many = located || request.envelope.contains_key("records");
    match value {
        RequestValue::Decisions(v) => atomic(v, detailed, located, many),
        RequestValue::Choices(v) => atomic(v, detailed, located, many),
        RequestValue::Tags(v) => atomic(v, detailed, located, many),
        RequestValue::Scores(v) => atomic(v, detailed, located, many),
        RequestValue::Filtered(v) => written(serde_json::to_string(
            &v.iter()
                .map(|r| {
                    if located {
                        project(r.original(), json!(true))
                    } else {
                        original(r.original())
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
        )),
        RequestValue::Ranked(v) => ranked(v, located),
        RequestValue::Found(v) => found(v, located),
        RequestValue::Annotations(v) => annotations(v, located),
        RequestValue::Recognized(v) => recognized(v, located),
        RequestValue::Related(v) => related(v, located),
        RequestValue::SetRanked(_) => Err(Failure::defect(
            "a frozen rank request produced set rankings",
        )),
    }
}
trait Atomic {
    fn bare(&self) -> Value;
    fn details(
        row: &CompleteRecord<QuestionInput, Self>,
        many: bool,
    ) -> Result<String, thinkthen::Error>
    where
        Self: Sized;
}
macro_rules! atomic_projection {
    ($ty:ty, $value:expr) => {
        impl Atomic for $ty {
            fn bare(&self) -> Value {
                $value(self)
            }
            fn details(
                row: &CompleteRecord<QuestionInput, Self>,
                many: bool,
            ) -> Result<String, thinkthen::Error> {
                if many {
                    Ok(row.compatibility_details()?.to_json())
                } else {
                    row.result().to_unbatched_scalar_json()
                }
            }
        }
    };
}
atomic_projection!(
    thinkthen::CompleteDecision,
    |r: &thinkthen::CompleteDecision| match r.value() {
        Answer::Yes => json!(true),
        Answer::No => json!(false),
        Answer::Unsure => Value::Null,
    }
);
atomic_projection!(
    thinkthen::CompleteChoice,
    |r: &thinkthen::CompleteChoice| json!(r.value())
);
atomic_projection!(
    thinkthen::CompleteTags,
    |r: &thinkthen::CompleteTags| json!(r.value())
);
atomic_projection!(
    thinkthen::CompleteScore,
    |r: &thinkthen::CompleteScore| json!(r.value())
);
fn atomic<T: Atomic>(
    rows: &[CompleteRecord<QuestionInput, T>],
    detailed: bool,
    located: bool,
    many: bool,
) -> Result<String, Failure> {
    let rows = rows
        .iter()
        .map(|r| {
            let text = if detailed {
                T::details(r, many)?
            } else {
                written(serde_json::to_string(&r.result().bare()))?
            };
            if !located {
                return raw(text);
            }
            let value = serde_json::from_str(&text)
                .map_err(|_| Failure::defect("a scalar answer could not be read"))?;
            raw(written(serde_json::to_string(&project(
                r.original(),
                value,
            )?))?)
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    if many {
        written(serde_json::to_string(&rows))
    } else {
        rows.into_iter()
            .next()
            .map(|r| r.get().to_owned())
            .ok_or_else(|| Failure::defect("a scalar request held no answer"))
    }
}
fn ranked(
    rows: &[CompleteRecord<QuestionInput, thinkthen::CompleteRank>],
    located: bool,
) -> Result<String, Failure> {
    #[derive(Serialize)]
    struct Ranked {
        index: usize,
        record: Value,
        probability: f64,
    }
    let rows = rows
        .iter()
        .map(|r| {
            let Probabilities::YesNo { yes } = r.result().probabilities() else {
                return Err(Failure::defect("a rank answer held no probability of yes"));
            };
            if !located {
                return raw(written(serde_json::to_string(&Ranked {
                    index: r.ordinal(),
                    record: original(r.original())?,
                    probability: yes,
                }))?);
            }
            let mut row = project(r.original(), json!(yes))?;
            let object = row
                .as_object_mut()
                .ok_or_else(|| Failure::defect("a located rank has no object"))?;
            object.insert("index".into(), json!(r.ordinal()));
            object.insert("probability".into(), json!(yes));
            raw(written(serde_json::to_string(&row))?)
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    written(serde_json::to_string(&rows))
}
fn found(
    found: &thinkthen::CompleteFound<QuestionInput>,
    located: bool,
) -> Result<String, Failure> {
    #[derive(Serialize)]
    struct Picked {
        index: usize,
        unit: Value,
        probability: f64,
    }
    let thinkthen::FindSelection::Unit(index) = found.selection() else {
        return Ok("null".into());
    };
    let candidate = found
        .candidates()
        .get(index)
        .ok_or_else(|| Failure::defect("a find selection lost its candidate"))?;
    let input = candidate
        .input()
        .ok_or_else(|| Failure::defect("a find selection held no unit"))?;
    let picked = Picked {
        index,
        unit: original(input)?,
        probability: candidate.probability(),
    };
    if !located {
        return written(serde_json::to_string(&picked));
    }
    let value = serde_json::to_value(picked)
        .map_err(|_| Failure::defect("a find selection could not be written"))?;
    written(serde_json::to_string(&project(input, value)?))
}
fn annotations(
    rows: &[CompleteRecord<QuestionInput, thinkthen::CompleteAnnotated>],
    located: bool,
) -> Result<String, Failure> {
    let rows = rows
        .iter()
        .map(|r| {
            let text = r.result().value_json()?;
            if !located {
                return raw(text);
            }
            let value = serde_json::from_str(&text)
                .map_err(|_| Failure::defect("an annotation value could not be read"))?;
            raw(written(serde_json::to_string(&project(
                r.original(),
                value,
            )?))?)
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    written(serde_json::to_string(&rows))
}
fn recognized(
    rows: &[CompleteRecord<QuestionInput, thinkthen::CompleteRecognized>],
    located: bool,
) -> Result<String, Failure> {
    if !located {
        return rows
            .first()
            .map(|r| r.result().value().to_json())
            .ok_or_else(|| Failure::defect("a recognition request held no answer"));
    }
    let rows = rows
        .iter()
        .map(|r| {
            let answer = serde_json::to_value(r.result().source_value())
                .map_err(|_| Failure::defect("a recognition value could not be written"))?;
            project(r.original(), answer)
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    written(serde_json::to_string(&rows))
}
fn related(
    row: &CompleteRecord<Vec<QuestionInput>, thinkthen::CompleteRelated>,
    located: bool,
) -> Result<String, Failure> {
    if !located {
        let edges = row
            .result()
            .value()
            .iter()
            .map(|e| raw(e.to_json()))
            .collect::<Result<Vec<_>, _>>()?;
        return written(serde_json::to_string(&crate::typed::Edges { edges }));
    }
    let edges = row.result().source_edges().unwrap_or_default().iter().map(|edge| {
        let mut value = json!({"relation":edge.edge().relation(),"source":endpoint(edge.source()),"target":endpoint(edge.target()),"probability":edge.edge().probability()});
        if edge.edge().either() && let Some(object) = value.as_object_mut() { object.insert("either".into(), json!(true)); }
        value
    }).collect::<Vec<_>>();
    let text = written(serde_json::to_string(&json!({"edges":edges})))?;
    if text.len() > thinkthen::Relate::max_source_output_bytes() {
        return Err(Failure::usage("source relate output exceeds 16 MiB"));
    }
    Ok(text)
}
fn endpoint(endpoint: &thinkthen::SourceRelationEndpoint) -> Value {
    let location = endpoint.location();
    json!({"record":endpoint.record(),"file":location.file(),"first_line":location.first_line(),"last_line":location.last_line(),
        "value":{"name":endpoint.entity().name(),"kind":endpoint.entity().kind()}})
}
fn original(input: &QuestionInput) -> Result<Value, Failure> {
    serde_json::to_value(input).map_err(|_| Failure::defect("an original could not be written"))
}
fn project(input: &QuestionInput, value: Value) -> Result<Value, Failure> {
    let QuestionInput::Record(record) = input else {
        return Err(Failure::defect("a located original has no record"));
    };
    let location = record
        .location()
        .ok_or_else(|| Failure::defect("a located original has no location"))?;
    Ok(
        json!({"record":record.original(),"file":location.file(),"first_line":location.first_line(),"last_line":location.last_line(),"value":value}),
    )
}
