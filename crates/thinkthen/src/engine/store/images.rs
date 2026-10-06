//! Image hits validate their referenced constituents in the answer's read snapshot.
use super::{Found, SQLITE, Store, Stored, found};
use crate::core::pack::{Ask, QuestionKey, model_json};
use crate::engine::{Cancel, error::Error};
use rusqlite::{Connection, params_from_iter};
use std::collections::HashMap;

impl Store {
    pub(crate) fn lookup_asks(
        &mut self,
        asks: &[Ask],
        cancel: &Cancel,
    ) -> Result<Vec<Option<Found>>, Error> {
        let keys: Vec<_> = asks.iter().map(|ask| ask.key).collect();
        let images: HashMap<_, _> = asks
            .iter()
            .filter(|ask| ask.state.body_limit().is_some())
            .map(|ask| (*ask.key.bytes(), ask))
            .collect();
        if images.is_empty() {
            self.lookup(&keys, cancel)
        } else {
            self.lookup_with(&keys, cancel, &images)
        }
    }
}

pub(super) fn select(
    connection: &Connection,
    chunk: &[QuestionKey],
    images: &HashMap<[u8; 32], &Ask>,
) -> rusqlite::Result<Vec<Stored>> {
    let marks = vec!["?"; chunk.len()].join(",");
    let sql = format!(
        "SELECT a.key, a.answer, a.answered_by, a.input_tokens, a.output_tokens,
         a.observation_id, a.batch_size, a.key_version, a.adapter,
         a.url, a.model, a.question, s.sha256, s.state, a.taken_at, a.origin
         FROM answers a LEFT JOIN states s ON s.id=a.state WHERE a.key IN ({marks})"
    );
    let mut statement = connection.prepare(&sql)?;
    let rows = statement.query_map(
        params_from_iter(chunk.iter().map(QuestionKey::bytes)),
        |row| {
            let (key, answer) = found(row)?;
            let answer = match answer {
                Ok(answer) if valid(row, &answer, images.get(key.as_slice()).copied())? => {
                    Ok(answer)
                }
                Ok(_) => Err(Error::Entry(
                    SQLITE.to_owned(),
                    "stored constituents or identity are invalid".to_owned(),
                )),
                Err(error) => Err(error),
            };
            Ok((key, answer))
        },
    )?;
    rows.collect()
}

fn valid(row: &rusqlite::Row<'_>, found: &Found, image: Option<&Ask>) -> rusqlite::Result<bool> {
    let Some(state) = row.get_ref(13)?.as_str().ok() else {
        return Ok(false);
    };
    let Some(digest) = row.get_ref(12)?.as_blob().ok() else {
        return Ok(false);
    };
    // An image hit also compares each original ordered constituent with the
    // current validated input in this same read snapshot.
    if image.is_some_and(|ask| state != ask.state.json() || digest != ask.state.sha256()) {
        return Ok(false);
    }
    let saved = super::Answer {
        key_version: Some(2),
        adapter: Some(crate::core::adapters::built_in::NAME.to_owned()),
        key: found.key.hex(),
        observation_id: Some(found.observation_id.clone()),
        batch_size: found.batch_size,
        url: row.get(9)?,
        model: row.get(10)?,
        question: row.get(11)?,
        state: crate::core::hex(digest),
        answer: found.answer.clone(),
        answered_by: found.answered_by.clone(),
        input_tokens: found
            .usage
            .and_then(crate::core::ReportedUsage::input_tokens),
        output_tokens: found
            .usage
            .and_then(crate::core::ReportedUsage::output_tokens),
        taken_at: row.get(14)?,
        origin: row.get(15)?,
    };
    let verified = || {
        let (question, _) = super::versioned::canonical_parts(&saved, state)?;
        let state = super::versioned::canonical_state(&saved.state, state)?;
        let url =
            crate::core::posting_address(&saved.url).map_err(|_| super::versioned::invalid())?;
        let model = model_json(&saved.model).map_err(|_| super::versioned::invalid())?;
        let reported = model_json(&saved.answered_by).map_err(|_| super::versioned::invalid())?;
        Ok::<_, Error>(
            QuestionKey::complete(&url, &model, &reported, &state, &question) == found.key,
        )
    };
    Ok(verified().unwrap_or(false))
}
