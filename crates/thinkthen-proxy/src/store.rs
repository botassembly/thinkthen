use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::definition::QuestionDefinition;
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::*;

/// One connection to a single tenant's store. Independent services may share it.
#[derive(Debug)]
pub struct Service {
    connection: Connection,
}

impl Service {
    /// Open or create a store with bounded lock waiting and durable transactions.
    ///
    /// # Errors
    /// Returns `proxy.storage` when opening or initializing the store fails.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let connection = Connection::open(path)?;
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.execute_batch(include_str!("schema.sql"))?;
        Ok(Self { connection })
    }

    /// Explicitly admit an owner-written essential ask and its first wording.
    ///
    /// # Errors
    /// Returns `proxy.question` for invalid admission, or `proxy.storage`.
    pub fn admit_question(
        &mut self,
        essential_ask: &str,
        request: &str,
    ) -> Result<Admission, Error> {
        if essential_ask.trim().is_empty() {
            return Err(Error::Question);
        }
        bounded(essential_ask)?;
        let definition = admit(request)?;
        let body = encoded(&json!([
            essential_ask,
            definition.function(),
            definition.domain()
        ]))?;
        let question = QuestionId(identity("question", &body));
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT OR IGNORE INTO questions(id, body, request) VALUES(?1, ?2, ?3)",
            params![question.0, body, request],
        )?;
        let wording = insert_wording(&tx, &question, &definition)?;
        tx.commit()?;
        Ok(Admission { question, wording })
    }

    /// Admit an unlinked inline question without inferring wording equivalence.
    ///
    /// # Errors
    /// Returns the same admission or storage error as explicit admission.
    pub fn admit_inline(&mut self, request: &str) -> Result<Admission, Error> {
        let definition = admit(request)?;
        let ask = definition
            .wording()
            .get("text")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .unwrap_or_else(|| definition.wording().to_string());
        self.admit_question(&ask, request)
    }

    /// Link an equivalent wording explicitly; its function and domain must match.
    ///
    /// # Errors
    /// Returns `proxy.question` for an absent question or changed fixed domain.
    pub fn attach_wording(
        &mut self,
        question: &QuestionId,
        request: &str,
    ) -> Result<WordingId, Error> {
        let existing = self.definition(question)?;
        let definition = admit(request)?;
        if existing.function() != definition.function() || existing.domain() != definition.domain()
        {
            return Err(Error::Question);
        }
        insert_wording(&self.connection, question, &definition)
    }

    /// Read one question's immutable essential ask, function and fixed domain.
    ///
    /// # Errors
    /// Returns `proxy.storage` for a store failure.
    pub fn question(&self, id: &QuestionId) -> Result<Option<Value>, Error> {
        self.content("SELECT body FROM questions WHERE id = ?1", &id.0)
    }

    /// Read one immutable wording, including its explicit question link.
    ///
    /// # Errors
    /// Returns `proxy.storage` for a store failure.
    pub fn wording(&self, id: &WordingId) -> Result<Option<Value>, Error> {
        self.content("SELECT body FROM wordings WHERE id = ?1", &id.0)
    }

    /// Admit exact text or structured evidence without changing its contents.
    ///
    /// # Errors
    /// Returns `proxy.storage` for an oversized value or store failure.
    pub fn admit_evidence(&self, evidence: &Evidence) -> Result<EvidenceId, Error> {
        let body = encoded(evidence)?;
        let id = EvidenceId(identity("evidence", &body));
        self.connection.execute(
            "INSERT OR IGNORE INTO evidence(id, body) VALUES(?1, ?2)",
            params![id.0, body],
        )?;
        Ok(id)
    }

    /// Read one exact typed evidence value.
    ///
    /// # Errors
    /// Returns `proxy.storage` for a store failure.
    pub fn evidence(&self, id: &EvidenceId) -> Result<Option<Value>, Error> {
        self.content("SELECT body FROM evidence WHERE id = ?1", &id.0)
    }

    /// Append a typed gold assertion and atomically advance its current pointer.
    /// The expected identifier is absent only for the first assertion.
    ///
    /// # Errors
    /// Returns `proxy.conflict` for a stale pointer, `proxy.question` for a wrong
    /// answer or attribution, and `proxy.storage` for a store failure.
    pub fn set_gold(
        &mut self,
        question: &QuestionId,
        evidence: &EvidenceId,
        expected: Option<GoldId>,
        answer: &Value,
        attribution: Attribution,
    ) -> Result<Gold, Error> {
        let definition = self.definition(question)?;
        let stored_evidence = self.evidence(evidence)?.ok_or(Error::Question)?;
        let value = stored_evidence.get("value").ok_or(Error::Storage)?;
        let text = if stored_evidence.get("type").and_then(Value::as_str) == Some("text") {
            value.as_str().ok_or(Error::Storage)?.to_owned()
        } else {
            thinkthen::native::recording_evidence(&serde_json::to_string(value)?)
                .map_err(|_| Error::Question)?
        };
        definition
            .validate_answer(answer, &text)
            .map_err(|_| Error::Question)?;
        if attribution.actor.trim().is_empty() || attribution.reason.trim().is_empty() {
            return Err(Error::Question);
        }
        let answer = encoded(answer)?;
        let attribution = encoded(&attribution)?;
        let recorded_at = i64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| Error::Storage)?
                .as_millis(),
        )
        .map_err(|_| Error::Storage)?;
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let current: Option<i64> = tx
            .query_row(
                "SELECT gold FROM current_gold WHERE question = ?1 AND evidence = ?2",
                params![question.0, evidence.0],
                |row| row.get(0),
            )
            .optional()?;
        if current.map(GoldId) != expected {
            return Err(Error::Conflict);
        }
        tx.execute("INSERT INTO gold(question, evidence, answer, attribution, recorded_at, supersedes) VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
            params![question.0, evidence.0, answer, attribution, recorded_at, current])?;
        let id = GoldId(tx.last_insert_rowid());
        tx.execute("INSERT INTO audit(gold) VALUES(?1)", [id.0])?;
        tx.execute("INSERT INTO current_gold(question, evidence, gold) VALUES(?1, ?2, ?3) ON CONFLICT(question, evidence) DO UPDATE SET gold = excluded.gold",
            params![question.0, evidence.0, id.0])?;
        tx.commit()?;
        self.gold(id)?.ok_or(Error::Storage)
    }

    /// Read the current assertion for one question and exact evidence item.
    ///
    /// # Errors
    /// Returns `proxy.storage` for a store failure.
    pub fn current_gold(
        &self,
        question: &QuestionId,
        evidence: &EvidenceId,
    ) -> Result<Option<Gold>, Error> {
        let id: Option<i64> = self
            .connection
            .query_row(
                "SELECT gold FROM current_gold WHERE question = ?1 AND evidence = ?2",
                params![question.0, evidence.0],
                |row| row.get(0),
            )
            .optional()?;
        id.map(|id| self.gold(GoldId(id)))
            .transpose()
            .map(Option::flatten)
    }

    /// Read one retained assertion and its audit provenance by identifier.
    ///
    /// # Errors
    /// Returns `proxy.storage` for a store failure.
    pub fn gold(&self, id: GoldId) -> Result<Option<Gold>, Error> {
        self.connection.query_row("SELECT g.id, g.question, g.evidence, g.answer, g.attribution, g.recorded_at, g.supersedes FROM gold g JOIN audit a ON a.gold = g.id WHERE g.id = ?1",
            [id.0], gold_row).optional()?.map(decode_gold).transpose()
    }

    /// Read a bounded history page after an optional assertion identifier.
    ///
    /// # Errors
    /// Returns `proxy.storage` for a store failure.
    pub fn gold_history(
        &self,
        question: &QuestionId,
        evidence: &EvidenceId,
        after: Option<GoldId>,
    ) -> Result<Vec<Gold>, Error> {
        let mut statement = self.connection.prepare("SELECT g.id, g.question, g.evidence, g.answer, g.attribution, g.recorded_at, g.supersedes FROM gold g JOIN audit a ON a.gold = g.id WHERE g.question = ?1 AND g.evidence = ?2 AND g.id > ?3 ORDER BY g.id LIMIT ?4")?;
        let rows = statement.query_map(
            params![
                question.0,
                evidence.0,
                after.map_or(0, |id| id.0),
                i64::try_from(HISTORY_PAGE).map_err(|_| Error::Storage)?
            ],
            gold_row,
        )?;
        rows.map(|row| decode_gold(row?)).collect()
    }

    fn definition(&self, question: &QuestionId) -> Result<QuestionDefinition, Error> {
        let request: String = self
            .connection
            .query_row(
                "SELECT request FROM questions WHERE id = ?1",
                [&question.0],
                |row| row.get(0),
            )
            .optional()?
            .ok_or(Error::Question)?;
        admit(&request)
    }

    fn content(&self, sql: &str, id: &str) -> Result<Option<Value>, Error> {
        let body: Option<String> = self
            .connection
            .query_row(sql, [id], |row| row.get(0))
            .optional()?;
        body.map(|body| serde_json::from_str(&body).map_err(Error::from))
            .transpose()
    }
}

fn admit(request: &str) -> Result<QuestionDefinition, Error> {
    bounded(request)?;
    let definition = QuestionDefinition::admit(request)?;
    Ok(definition)
}

fn insert_wording(
    connection: &Connection,
    question: &QuestionId,
    definition: &QuestionDefinition,
) -> Result<WordingId, Error> {
    let body = encoded(&json!([question, definition.wording()]))?;
    let id = WordingId(identity("wording", &body));
    connection.execute(
        "INSERT OR IGNORE INTO wordings(id, question, body) VALUES(?1, ?2, ?3)",
        params![id.0, question.0, body],
    )?;
    Ok(id)
}

fn encoded(value: &impl Serialize) -> Result<String, Error> {
    let text = serde_json::to_string(value)?;
    bounded(&text)?;
    Ok(text)
}

fn bounded(text: &str) -> Result<(), Error> {
    if text.len() > RECORD_BYTES {
        Err(Error::Storage)
    } else {
        Ok(())
    }
}

fn identity(kind: &str, body: &str) -> String {
    // JSON arrays and objects carry field/type boundaries; the outer tag keeps
    // identical representations in different record namespaces distinct.
    let mut hash = Sha256::new();
    hash.update((kind.len() as u64).to_be_bytes());
    hash.update(kind.as_bytes());
    hash.update((body.len() as u64).to_be_bytes());
    hash.update(body.as_bytes());
    format!("{hash:x}", hash = hash.finalize())
}

type StoredGold = (i64, String, String, String, String, i64, Option<i64>);
fn gold_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredGold> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
    ))
}
fn decode_gold(
    (id, question, evidence, answer, attribution, recorded_at, supersedes): StoredGold,
) -> Result<Gold, Error> {
    Ok(Gold {
        id: GoldId(id),
        question: QuestionId(question),
        evidence: EvidenceId(evidence),
        answer: serde_json::from_str(&answer)?,
        attribution: serde_json::from_str(&attribution)?,
        recorded_at,
        supersedes: supersedes.map(GoldId),
    })
}
