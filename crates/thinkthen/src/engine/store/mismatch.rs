//! Detect excluded historical models only within the configured route and question.
use super::{Store, found, images};
use crate::core::pack::Ask;
use crate::engine::{Cancel, error::Error};
use rusqlite::Connection;
impl Store {
    pub(super) fn held_model_mismatch(
        &self,
        asks: &[Ask],
        found: &[Option<super::Found>],
        url: &str,
        model: &str,
        cancel: &Cancel,
    ) -> Result<bool, Error> {
        if self.mode != super::Mode::Cache {
            return Ok(false);
        }
        let Some(connection) = self.connection.as_ref() else {
            return Ok(false);
        };
        for (ask, found) in asks.iter().zip(found) {
            if found.is_none()
                && self.waiting(cancel, || excluded(connection, ask, url, model))??
            {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
fn excluded(
    connection: &Connection,
    ask: &Ask,
    url: &str,
    model: &str,
) -> rusqlite::Result<Result<bool, Error>> {
    let sql = format!(
        "SELECT {} FROM answers a LEFT JOIN states s ON s.id=a.state
        WHERE a.url=?1 AND a.model=?2 AND a.question=?3 AND s.sha256=?4 AND a.answered_by<>?2",
        images::COLUMNS
    );
    let mut statement = connection.prepare(&sql)?;
    let mut rows = statement.query((url, model, &*ask.question, ask.state.sha256().as_slice()))?;
    if let Some(row) = rows.next()? {
        let (_, answer) = found(row)?;
        let answer = match answer {
            Ok(answer) => answer,
            Err(error) => return Ok(Err(error)),
        };
        if !images::valid(row, &answer, Some(ask))? {
            return Ok(Err(super::versioned::invalid()));
        }
        return Ok(Ok(true));
    }
    Ok(Ok(false))
}
