//! One complete ordered find set, copied before its worker can detach.

use thinkthen::{Evidence, For, Question, Settings};

use super::{BridgeSettings, BridgeStop, BridgeText, asked, probe, run_detached, text};
use crate::engines;

#[derive(Debug)]
struct Indexed {
    position: usize,
    text: String,
}

impl Evidence for Indexed {
    fn evidence(&self) -> &str {
        &self.text
    }
}

fn frame(found: &thinkthen::Found<Indexed>) -> Result<Vec<u8>, String> {
    let candidates = found.candidates();
    let count = u32::try_from(candidates.len())
        .map_err(|_| "thinkthen defect: find returned too many candidates".to_owned())?;
    let selected = found
        .selected()
        .map(|unit| i64::try_from(unit.position))
        .transpose()
        .map_err(|_| "thinkthen defect: a find index overflowed".to_owned())?
        .unwrap_or(-1);
    let mut bytes = Vec::with_capacity(12 + candidates.len() * 16);
    bytes.extend_from_slice(&count.to_ne_bytes());
    bytes.extend_from_slice(&selected.to_ne_bytes());
    for candidate in candidates {
        let index = candidate
            .input()
            .map(|unit| i64::try_from(unit.position))
            .transpose()
            .map_err(|_| "thinkthen defect: a find index overflowed".to_owned())?
            .unwrap_or(-1);
        bytes.extend_from_slice(&index.to_ne_bytes());
        bytes.extend_from_slice(&candidate.probability().to_ne_bytes());
    }
    Ok(bytes)
}

struct PreparedFind {
    question: Question,
    call: Settings,
    model: Option<String>,
}

fn portable(question: &str, units: &[BridgeText], settings: &str) -> Result<PreparedFind, String> {
    let call = Settings::parse(settings)
        .and_then(|value| {
            value.check(For::Find)?;
            Ok(value)
        })
        .map_err(|error| crate::errors::RowError::usage(&error.to_string()).text)?;
    // The shared parser has already rejected duplicate/unknown members. This
    // read only extracts the validated model for the existing engine builder.
    let model = serde_json::from_str::<serde_json::Value>(settings)
        .ok()
        .and_then(|value| value.get("model")?.as_str().map(str::to_owned));
    let none = call.none().unwrap_or(false);
    let question = Question::find(question)
        .and_then(|value| {
            if none {
                value.offering_none()
            } else {
                Ok(value)
            }
        })
        .map_err(|error| crate::errors::RowError::from(error).text)?;
    let texts = units
        .iter()
        .map(|unit| text(unit.bytes, unit.len))
        .collect::<Result<Vec<_>, _>>()?;
    question
        .admit_find_units(texts.into_iter().map(Ok))
        .map_err(|error| crate::errors::RowError::from(error).text)?;
    Ok(PreparedFind {
        question,
        call,
        model,
    })
}

pub(super) fn validate_portable(
    question: &str,
    units: &[BridgeText],
    settings: &str,
) -> Result<Vec<u8>, String> {
    portable(question, units, settings).map(|_| Vec::new())
}

pub(super) fn run_portable(
    question: &str,
    units: &[BridgeText],
    settings: &str,
    query_deadline_ms: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Result<Vec<u8>, String> {
    let PreparedFind {
        question,
        call,
        model,
    } = portable(question, units, settings)?;
    let units = units
        .iter()
        .enumerate()
        .map(|(position, unit)| {
            text(unit.bytes, unit.len).map(|text| Indexed {
                position,
                text: text.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut asked = asked(&session)?;
    if let Some(model) = model {
        asked.model = Some(model);
    }
    let engine = engines::engine_for(&asked, |path| probe(&session, path))?;
    let total = asked.max_requests_total;
    let deadline_ms = match call.deadline_ms() {
        None | Some(-1) => query_deadline_ms,
        Some(value) if query_deadline_ms < 0 => value,
        Some(value) => query_deadline_ms.min(value),
    };
    let context = call.context().map(str::to_owned);
    let batch = if call.batch_max() {
        Some("max".to_owned())
    } else {
        call.batch_records().map(|value| value.to_string())
    };
    run_detached(stop, move |token| {
        let options = engines::options_for(
            deadline_ms,
            &token,
            total,
            batch.as_deref(),
            context.as_deref(),
        )?;
        let found = engine
            .find_with(&question, units, options)
            .map_err(|error| engines::call_error(error, total).text)?;
        frame(found.value())
    })
}
