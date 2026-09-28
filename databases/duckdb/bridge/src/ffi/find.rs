//! One complete ordered find set, copied before its worker can detach.

use thinkthen::{Evidence, Question};

use super::{
    BridgeSettings, BridgeStop, BridgeText, asked, copied_texts, probe, run_detached, text,
};
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

fn prepared(
    question_bytes: *const u8,
    question_len: usize,
    units: *const BridgeText,
    count: usize,
    none: i32,
) -> Result<(Question, Vec<Indexed>), String> {
    if none != 0 && none != 1 {
        return Err("thinkthen defect: find got another none flag".to_owned());
    }
    let question = text(question_bytes, question_len)?;
    let units = validated(copied_texts(units, count)?, none != 0)?;
    let question = Question::find(question)
        .and_then(|question| {
            if none == 0 {
                Ok(question)
            } else {
                question.offering_none()
            }
        })
        .map_err(|error| crate::errors::RowError::from(error).text)?;
    Ok((question, units))
}

pub(super) fn validate(
    question_bytes: *const u8,
    question_len: usize,
    units: *const BridgeText,
    count: usize,
    none: i32,
) -> Result<Vec<u8>, String> {
    prepared(question_bytes, question_len, units, count, none).map(|_| Vec::new())
}

pub(super) struct Input {
    pub(super) question: *const u8,
    pub(super) question_len: usize,
    pub(super) units: *const BridgeText,
    pub(super) count: usize,
    pub(super) none: i32,
    pub(super) deadline_ms: i64,
    pub(super) settings: BridgeSettings,
    pub(super) stop: BridgeStop,
}

pub(super) fn run(input: Input) -> Result<Vec<u8>, String> {
    let (question, units) = prepared(
        input.question,
        input.question_len,
        input.units,
        input.count,
        input.none,
    )?;
    let asked = asked(&input.settings)?;
    let engine = engines::engine_for(&asked, |path| probe(&input.settings, path))?;
    let total = asked.max_requests_total;
    run_detached(input.stop, move |token| {
        let options = engines::options(input.deadline_ms, &token, total)?;
        let found = engine
            .find_with(&question, units, options)
            .map_err(|error| engines::call_error(error, total).text)?;
        frame(found.value())
    })
}
