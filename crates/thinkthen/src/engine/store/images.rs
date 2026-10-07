//! Image hits validate their referenced constituents in the answer's read snapshot.
use super::{Found, SQLITE, Store, Stored, found};
use crate::core::pack::{Ask, QuestionKey};
use crate::engine::{Cancel, error::Error};
use rusqlite::{Connection, params_from_iter};
use std::collections::HashMap;

pub(crate) struct Lookup {
    pub(crate) answers: Vec<Option<Found>>,
    pub(crate) held_model_mismatch: bool,
}

pub(super) const COLUMNS: &str = "a.key, a.answer, a.answered_by, a.input_tokens, a.output_tokens,
    a.observation_id, a.batch_size, a.key_version, a.adapter,
    a.url, a.model, a.question, s.sha256, s.state, a.taken_at, a.origin";

impl Store {
    pub(crate) fn lookup_asks(
        &mut self,
        asks: &[Ask],
        cancel: &Cancel,
        route: (&str, &str),
    ) -> Result<Lookup, Error> {
        let keys: Vec<_> = asks.iter().map(|ask| ask.key).collect();
        let images: HashMap<_, _> = asks
            .iter()
            .filter(|ask| ask.state.body_limit().is_some())
            .map(|ask| (*ask.key.bytes(), ask))
            .collect();
        let found = if images.is_empty() {
            self.lookup(&keys, cancel)?
        } else {
            self.lookup_with(&keys, cancel, &images)?
        };
        let mismatch = self.held_model_mismatch(asks, &found, route.0, route.1, cancel)?;
        Ok(Lookup {
            answers: found,
            held_model_mismatch: mismatch,
        })
    }
}

pub(super) fn select(
    connection: &Connection,
    chunk: &[QuestionKey],
    images: &HashMap<[u8; 32], &Ask>,
) -> rusqlite::Result<Vec<Stored>> {
    let marks = vec!["?"; chunk.len()].join(",");
    let sql = format!(
        "SELECT {COLUMNS} FROM answers a LEFT JOIN states s ON s.id=a.state WHERE a.key IN ({marks})"
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

pub(super) fn valid(
    row: &rusqlite::Row<'_>,
    found: &Found,
    image: Option<&Ask>,
) -> rusqlite::Result<bool> {
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
        adapter: Some(row.get(8)?),
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
    Ok(
        super::versioned::complete_key(&saved, state, &saved.answered_by)
            .is_ok_and(|key| key == found.key),
    )
}
