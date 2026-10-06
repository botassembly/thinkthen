//! Image hits validate their referenced constituents in the answer's read snapshot.
use super::{Found, SQLITE, Store, Stored, found};
use crate::core::Url;
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
    let sql = if images.is_empty() {
        format!(
            "SELECT key, answer, answered_by, input_tokens, output_tokens FROM answers WHERE key IN ({marks})"
        )
    } else {
        format!(
            "SELECT a.key, a.answer, a.answered_by, a.input_tokens, a.output_tokens,
            a.url, a.model, a.question, s.sha256, s.state
            FROM answers a LEFT JOIN states s ON s.id=a.state WHERE a.key IN ({marks})"
        )
    };
    let mut statement = connection.prepare(&sql)?;
    let rows = statement.query_map(
        params_from_iter(chunk.iter().map(QuestionKey::bytes)),
        |row| {
            let (key, answer) = found(row)?;
            let answer = if let Some(ask) = images.get(key.as_slice())
                && !valid(row, ask)?
            {
                Err(Error::Entry(
                    SQLITE.to_owned(),
                    "stored image constituents or identity are invalid".to_owned(),
                ))
            } else {
                answer
            };
            Ok((key, answer))
        },
    )?;
    rows.collect()
}

fn valid(row: &rusqlite::Row<'_>, ask: &Ask) -> rusqlite::Result<bool> {
    // Exact canonical envelope equality checks schema version, ancillary state,
    // every ordered media/base64 byte and duplicate against the current typed
    // input, whose compressed pixels were already validated at the edge. No
    // marker guessing, second decoder, or large stored-state copy is needed.
    if row.get_ref(9)?.as_str().ok() != Some(ask.state.json())
        || row.get_ref(8)?.as_blob().ok() != Some(ask.state.sha256().as_slice())
    {
        return Ok(false);
    }
    let url = Url::new(row.get::<_, String>(5)?).ok();
    let model = model_json(&row.get::<_, String>(6)?).ok();
    let question: String = row.get(7)?;
    Ok(url
        .zip(model)
        .is_some_and(|(url, model)| ask.state.key(&url, &model, &question) == ask.key))
}
