//! Read actual recognition questions from a saved generic receipt fixture.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

use thinkthen::{
    CallOptions, Engine, QuestionInput, Recognize, RecordInput, RecordObservation, Request,
    RequestEnvironment, RequestFeed, RequestInput, RequestOptions, RequestOutcome, RequestValue,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/recognize/caller-defined");
    let text = std::fs::read_to_string(fixture.join("text.txt"))?;
    let ask = Recognize::from_json(&std::fs::read_to_string(fixture.join("question.json"))?)?;
    let url = std::fs::read_to_string(fixture.join("url.txt"))?;
    let engine = Engine::builder()
        .base_url(url.trim())?
        .no_cache()
        .replay(fixture.join("recording"))?
        .timeout(Duration::from_millis(100))?
        .max_retries(0)
        .build()?;
    let questions = Mutex::new(BTreeMap::new());
    let finished = Mutex::new(Vec::new());
    let callback = |event: RecordObservation<'_>| match event {
        RecordObservation::Question {
            index,
            stage: Some(stage),
            position,
            detail,
            ..
        } => {
            if let Ok(mut held) = questions.lock() {
                held.insert((index, stage, position), detail.to_owned());
            }
        }
        RecordObservation::Row { index, .. } => {
            if let Ok(mut held) = finished.lock() {
                held.push(index);
            }
        }
        _ => {}
    };
    let records = [text.clone(), text.clone()].map(|original| RecordInput {
        seed_spans: None,
        original: QuestionInput::Text(original),
        context: None,
        options: None,
        examples: None,
    });
    let request = Request::for_definition(
        ask.into(),
        RequestInput::Feed {
            name: "observed".to_owned(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
        RequestOptions::default(),
    )
    .admit()?;
    let call = match engine.execute_request(
        &request,
        RequestEnvironment {
            controls: CallOptions::new().observe(&callback),
            feed: Some(RequestFeed::from_records("observed", records.into_iter().map(Ok)).eager()),
        },
    )? {
        RequestOutcome::Complete(call) => call,
        RequestOutcome::Failed { error, .. } => return Err(error.into()),
    };
    let held = questions.lock().map_err(|_| "question lock poisoned")?;
    let mut out = std::io::stdout().lock();
    for ((index, stage, position), owned) in held.iter() {
        let detail = owned.detail();
        let question = detail.question();
        writeln!(out, "record={index} stage={stage} position={position}")?;
        writeln!(
            out,
            "question={}",
            question.text().text().ok_or("text missing")?
        )?;
        for option in question.options() {
            let description = option
                .description()
                .map(|value| value.to_json())
                .transpose()?;
            writeln!(out, "option={} description={description:?}", option.name())?;
        }
        writeln!(
            out,
            "answer={:?} failure={:?} probabilities={:?}",
            detail.value(),
            detail.failure(),
            detail.probabilities()
        )?;
        writeln!(
            out,
            "requests={:?} cached={} sources={:?}",
            detail.requests(),
            detail.cached(),
            detail.question_sources()
        )?;
    }
    writeln!(
        out,
        "completed={:?}",
        finished.lock().map_err(|_| "row lock poisoned")?
    )?;
    let RequestValue::Recognized(rows) = call.value() else {
        return Err("recognition returned another result kind".into());
    };
    for row in rows {
        let QuestionInput::Text(original) = row.original() else {
            return Err("recognition lost its text original".into());
        };
        writeln!(out, "original[{}]={}", row.ordinal(), original)?;
    }
    writeln!(out, "requests_sent={}", call.facts().requests_sent())?;
    Ok(())
}
