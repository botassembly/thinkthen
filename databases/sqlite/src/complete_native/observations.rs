//! SQL observation snapshots read actual native typed fields on the owning call.
use super::defect;
use serde_json::{Value, json};
use thinkthen::{QuestionInput, RecordObservation};
pub(crate) fn event(event: &RecordObservation<'_>) -> Result<Value, thinkthen::Error> {
    match event {
        RecordObservation::Question {
            index,
            member,
            stage,
            position,
            detail,
        } => {
            let inputs = detail.inputs().map(input).collect::<Result<Vec<_>, _>>()?;
            let mut value = serde_json::to_value(event).map_err(|_| defect())?;
            super::put(&mut value, "kind", json!("question"))?;
            super::put(&mut value, "index", json!(index))?;
            super::put(&mut value, "member", json!(member))?;
            super::put(&mut value, "stage", json!(stage))?;
            super::put(&mut value, "position", json!(position))?;
            super::put(&mut value, "answer_id", json!(detail.answer_id()))?;
            super::put(&mut value, "failure_id", json!(detail.failure_id()))?;
            super::put(
                &mut value,
                "question_sources",
                json!(detail.question_sources()),
            )?;
            super::put(&mut value, "observations", json!(detail.observations()))?;
            super::put(&mut value, "inputs", json!(inputs))?;
            super::put(&mut value, "raw_pick", json!(detail.raw_pick()))?;
            super::put(
                &mut value,
                "author",
                json!({"name":detail.question().name(),"wording_version":detail.question().wording_version()}),
            )?;
            Ok(value)
        }
        RecordObservation::Row { index, value } => {
            Ok(json!({"kind":"row","index":index,"value":row(value)?}))
        }
    }
}
fn input(input: &QuestionInput) -> Result<Value, thinkthen::Error> {
    let mut value = match input {
        QuestionInput::Text(text) => json!({"input":text}),
        QuestionInput::Record(record) => {
            json!({"input":record.original(),"source":record.location()})
        }
        QuestionInput::Images(images) => json!({"input":images.text(),"source":images.location()}),
    };
    let images = match input {
        QuestionInput::Record(record) => record.images(),
        QuestionInput::Images(images) => images.images(),
        QuestionInput::Text(_) => &[],
    };
    super::put(
        &mut value,
        "images",
        serde_json::to_value(images).unwrap_or_else(|_| super::failure(&defect())),
    )?;
    Ok(value)
}

fn row(value: &thinkthen::ObservedRow<'_>) -> Result<Value, thinkthen::Error> {
    use thinkthen::{Annotated, Judgment, ObservedRow};
    Ok(match value {
        ObservedRow::Judgment(value)=>match value {
            Judgment::Decision(value)=>decision(*value),Judgment::Choice(v)=>json!(v),Judgment::Score(v)=>json!(v),Judgment::Tags(v)=>json!(v),
        },
        ObservedRow::Find(index)=>json!(index),
        ObservedRow::Recognized(value)=>serde_json::from_str(&value.to_json()).map_err(|_|defect())?,
        ObservedRow::Relations(edges)=>Value::Array(edges.iter().map(|edge|serde_json::from_str(&edge.to_json()).map_err(|_|defect())).collect::<Result<Vec<_>,_>>()?),
        ObservedRow::Annotated(members)=>Value::Object(members.iter().map(|member|{
            let value=match member.value(){Annotated::Decision(v)=>decision(*v),Annotated::Choice(v)=>json!(v),Annotated::Score(v)=>json!(v),Annotated::Tags(v)=>json!(v),Annotated::Failed(v)=>json!({"failed":{"kind":v.kind().name(),"cause":match v.cause(){thinkthen::FailureCause::MissingAnswer=>"missing_answer",thinkthen::FailureCause::WrongKind=>"wrong_kind",thinkthen::FailureCause::MissingProbability=>"missing_probability",thinkthen::FailureCause::InvalidProbability=>"invalid_probability",thinkthen::FailureCause::InvalidDistribution=>"invalid_distribution",thinkthen::FailureCause::UnexpectedProbability=>"unexpected_probability"}}})};
            (member.name().to_owned(),value)
        }).collect()),
    })
}
fn decision(answer: thinkthen::Answer) -> Value {
    match answer {
        thinkthen::Answer::Yes => json!(true),
        thinkthen::Answer::No => json!(false),
        thinkthen::Answer::Unsure => Value::Null,
    }
}

/// Borrow an owned native snapshot through the existing SQL event serializer.
pub(crate) fn owned_event(
    event: &thinkthen::OwnedRecordObservation,
) -> Result<Value, thinkthen::Error> {
    use thinkthen::{ObservedRow, OwnedObservedRow as Row, OwnedRecordObservation as Event};
    match event {
        Event::Question {
            index,
            member,
            stage,
            position,
            detail,
        } => self::event(&RecordObservation::Question {
            index: *index,
            member: member.as_deref(),
            stage: *stage,
            position: *position,
            detail: detail.detail(),
        }),
        Event::Row { index, value } => {
            let borrowed = match value {
                Row::Judgment(v) => ObservedRow::Judgment(v),
                Row::Annotated(v) => ObservedRow::Annotated(v),
                Row::Recognized(v) => ObservedRow::Recognized(v),
                Row::Find(v) => ObservedRow::Find(*v),
                Row::Relations(v) => ObservedRow::Relations(v),
            };
            self::event(&RecordObservation::Row {
                index: *index,
                value: borrowed,
            })
        }
    }
}
