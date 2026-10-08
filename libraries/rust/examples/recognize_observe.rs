//! Read actual recognition questions from a saved generic receipt fixture.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

use thinkthen::{CallOptions, Engine, Recognize, RecordInput, RecordObservation};

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
    let records = [text.as_str(), text.as_str()].map(|original| RecordInput {
            seed_spans: None,
        original,
        context: None,
        options: None,
        examples: None,
    });
    let call = engine.recognize_records_complete_with(
        &ask,
        records,
        CallOptions::new().observe(&callback),
    )?;
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
    for row in call.value() {
        writeln!(out, "original[{}]={}", row.ordinal(), row.original())?;
    }
    writeln!(out, "requests_sent={}", call.facts().requests_sent())?;
    Ok(())
}
