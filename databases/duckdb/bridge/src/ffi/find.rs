//! One complete ordered find set, copied before its worker can detach.

use thinkthen::{Evidence, For, Question, Settings};

use super::{BridgeSettings, BridgeStop, asked, probe, run_detached};
use crate::engines;

const MAX_TEXT_BYTES: usize = 16 * 1024 * 1024;

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

fn validated(texts: Vec<String>, none: bool) -> Result<Vec<Indexed>, String> {
    let most = if none { 254 } else { 255 };
    if !(2..=most).contains(&texts.len()) {
        return Err(format!(
            "thinkthen usage: find takes 2 to {most} units{}",
            if none { " when offering none" } else { "" }
        ));
    }
    let mut bytes = 0_usize;
    texts
        .into_iter()
        .enumerate()
        .map(|(position, text)| {
            if text.trim().is_empty() {
                return Err("thinkthen usage: a find unit is text, not white space".to_owned());
            }
            bytes = bytes
                .checked_add(text.len())
                .ok_or_else(|| "thinkthen usage: find units exceed 16 MiB of text".to_owned())?;
            if bytes > MAX_TEXT_BYTES {
                return Err("thinkthen usage: find units exceed 16 MiB of text".to_owned());
            }
            Ok(Indexed { position, text })
        })
        .collect()
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
    units: Vec<Indexed>,
    call: Settings,
    model: Option<String>,
}

fn portable(question: &str, units: Vec<String>, settings: &str) -> Result<PreparedFind, String> {
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
    let units = validated(units, none)?;
    let question = Question::find(question)
        .and_then(|value| {
            if none {
                value.offering_none()
            } else {
                Ok(value)
            }
        })
        .map_err(|error| crate::errors::RowError::from(error).text)?;
    Ok(PreparedFind {
        question,
        units,
        call,
        model,
    })
}

pub(super) fn validate_portable(
    question: &str,
    units: Vec<String>,
    settings: &str,
) -> Result<Vec<u8>, String> {
    portable(question, units, settings).map(|_| Vec::new())
}

pub(super) fn run_portable(
    question: &str,
    units: Vec<String>,
    settings: &str,
    query_deadline_ms: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Result<Vec<u8>, String> {
    let PreparedFind {
        question,
        units,
        call,
        model,
    } = portable(question, units, settings)?;
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
