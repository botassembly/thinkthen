//! Explicit recordings retain original bodies in the answers' transaction.

use super::{Mode, Row, Store, insert};
use crate::engine::{Cancel, error::Error};

#[derive(Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Original {
    #[serde(rename = "exchange_sha256")]
    pub(crate) digest: String,
    pub(crate) url: String,
    #[serde(with = "utf8")]
    pub(crate) request: Vec<u8>,
    #[serde(with = "utf8")]
    pub(crate) response: Vec<u8>,
}

impl std::fmt::Debug for Original {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Original(<withheld>)")
    }
}

impl Original {
    pub(super) fn validate(&self) -> Result<(), Error> {
        let url = crate::core::Url::new(&self.url).map_err(|_| super::versioned::invalid())?;
        crate::core::posting_address(&self.url).map_err(|_| super::versioned::invalid())?;
        if crate::core::recording::Exchange::new(&url, &self.request)
            .digest()
            .as_str()
            != self.digest
        {
            return Err(super::versioned::invalid());
        }
        for body in [&self.request, &self.response] {
            let text = std::str::from_utf8(body).map_err(|_| super::versioned::invalid())?;
            crate::core::Json::parse(text).map_err(|_| super::versioned::invalid())?;
        }
        Ok(())
    }

    pub(super) fn insert(&self, connection: &rusqlite::Connection) -> rusqlite::Result<()> {
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS exchanges (
                digest TEXT PRIMARY KEY, url TEXT NOT NULL,
                request BLOB NOT NULL, response BLOB NOT NULL
            )",
        )?;
        connection.execute(
            "INSERT INTO exchanges (digest, url, request, response) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(digest) DO UPDATE SET response=excluded.response",
            (&self.digest, &self.url, &self.request, &self.response),
        )?;
        Ok(())
    }
}

impl super::Entries {
    pub(super) fn read_originals(
        &mut self,
        connection: &rusqlite::Connection,
    ) -> Result<(), Error> {
        let exists: u32 = connection
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='exchanges'",
                [],
                |row| row.get(0),
            )
            .map_err(super::storage)?;
        if exists == 0 {
            return Ok(());
        }
        let mut statement = connection
            .prepare("SELECT digest,url,request,response FROM exchanges")
            .map_err(super::storage)?;
        let rows = statement
            .query_map([], |row| {
                Ok(Original {
                    digest: row.get(0)?,
                    url: row.get(1)?,
                    request: row.get(2)?,
                    response: row.get(3)?,
                })
            })
            .map_err(super::storage)?;
        for original in rows {
            let original = original.map_err(|_| super::versioned::invalid())?;
            original.validate()?;
            self.originals.insert(original.digest.clone(), original);
        }
        Ok(())
    }
}

mod utf8 {
    use serde::{Deserialize as _, Deserializer, Serializer};

    pub(super) fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| serde::ser::Error::custom("a recorded body is not UTF-8"))?;
        serializer.serialize_str(text)
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<u8>, D::Error> {
        String::deserialize(deserializer).map(String::into_bytes)
    }
}

impl Store {
    pub(super) fn write_original(
        &mut self,
        rows: &[Row<'_>],
        original: Option<&Original>,
        attempts: &[crate::core::AttemptObservation],
        cancel: &Cancel,
    ) -> Result<(), Error> {
        let Some(original) = original.filter(|_| self.mode == Mode::Record) else {
            return self.write(rows, cancel);
        };
        if self.connection.is_none() {
            self.connect(cancel)?;
        }
        let timing =
            super::timing::Timing::prepare(&self.folder, rows, attempts, cancel, self.busy_limit)?;
        let connection = self.connection.as_ref().ok_or(Error::RecordingStorage)?;
        self.transaction(cancel, connection, || {
            self.waiting(cancel, || {
                insert(connection, rows)?;
                original.insert(connection)?;
                Ok(())
            })
        })?;
        timing.map_or(Ok(()), super::timing::Timing::commit)
    }
}
