//! Concrete SQL row metadata and lossless reported token counts.

use super::{Found, Row, Stored, versioned};
use crate::core::{ReportedUsage, pack::QuestionKey};
use rusqlite::Connection;

pub(super) fn found(row: &rusqlite::Row<'_>) -> rusqlite::Result<Stored> {
    let usage = Some(ReportedUsage::new(count(row, 3)?, count(row, 4)?))
        .filter(|usage| usage.input_tokens().is_some() || usage.output_tokens().is_some());
    let key: Vec<u8> = row.get(0)?;
    let identity: String = row.get(5)?;
    let batch: Option<u32> = row.get(6)?;
    let version: u32 = row.get(7)?;
    let adapter: String = row.get(8)?;
    if version != 2 || crate::core::adapters::ApiType::from_name(&adapter).is_none() {
        return Ok((key, Err(versioned::invalid())));
    }
    let Some(key_id) = QuestionKey::parse(&crate::core::hex(&key)) else {
        return Ok((key, Err(versioned::invalid())));
    };
    let Ok(observation_id) = crate::core::ObservationId::new(identity) else {
        return Ok((key, Err(versioned::invalid())));
    };
    let batch_size = match batch {
        Some(batch) => match std::num::NonZeroU32::new(batch) {
            Some(batch) => Some(batch),
            None => return Ok((key, Err(versioned::invalid()))),
        },
        None => None,
    };
    let answer = Found {
        key: key_id,
        observation_id,
        batch_size,
        answer: row.get(1)?,
        answered_by: row.get(2)?,
        usage,
    };
    Ok((key, Ok(answer)))
}

pub(super) fn insert(connection: &Connection, rows: &[Row<'_>]) -> rusqlite::Result<()> {
    let mut add_state =
        connection.prepare("INSERT OR IGNORE INTO states (sha256, state) VALUES (?1, ?2)")?;
    let mut state = connection.prepare("SELECT id FROM states WHERE sha256 = ?1")?;
    let mut answer = connection
        .prepare(
            "INSERT INTO answers (key, url, model, state, question, answer, answered_by, input_tokens, output_tokens, taken_at, origin, key_version, adapter, observation_id, batch_size)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 2, ?12, ?13, ?14)
             ON CONFLICT(key) DO UPDATE SET url = excluded.url, model = excluded.model, state = excluded.state,
               question = excluded.question, answer = excluded.answer, answered_by = excluded.answered_by,
               input_tokens = excluded.input_tokens, output_tokens = excluded.output_tokens,
               taken_at = excluded.taken_at, origin = excluded.origin,
               key_version=2, adapter=excluded.adapter, observation_id=excluded.observation_id, batch_size=excluded.batch_size",
        )?;
    for row in rows {
        add_state.execute((row.state.sha256().as_slice(), row.state.json()))?;
        let id: i64 = state.query_row([row.state.sha256().as_slice()], |found| found.get(0))?;
        let input = row.usage.and_then(ReportedUsage::input_tokens);
        let output = row.usage.and_then(ReportedUsage::output_tokens);
        answer.execute(rusqlite::params![
            row.key.bytes().as_slice(),
            row.url,
            row.model,
            id,
            row.question,
            row.answer,
            row.answered_by,
            signed(input)?,
            signed(output)?,
            row.taken_at,
            row.origin,
            row.state.api().name(),
            row.observation_id.as_str(),
            row.batch_size.map(std::num::NonZeroU32::get),
        ])?;
    }
    Ok(())
}

/// SQLite's integer width must never turn a reported count into unknown usage.
pub(super) fn signed(value: Option<u64>) -> rusqlite::Result<Option<i64>> {
    value.map(i64::try_from).transpose().map_err(|_| {
        rusqlite::Error::ToSqlConversionFailure(
            "a reported token count exceeds storage bounds".into(),
        )
    })
}

pub(super) fn count(row: &rusqlite::Row<'_>, at: usize) -> rusqlite::Result<Option<u64>> {
    row.get::<_, Option<i64>>(at)?
        .map(u64::try_from)
        .transpose()
        .map_err(|_| rusqlite::Error::IntegralValueOutOfRange(at, -1))
}
