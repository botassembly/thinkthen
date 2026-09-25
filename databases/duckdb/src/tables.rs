//! `thinkthen_usage()` and the `thinkthen_warm` aggregate (ticket 0110
//! decisions 5 and 10).

use std::collections::BTreeSet;

use thinkthen::{ErrorKind, LoadedQuestion, Question};

use crate::errors::{prefix, usage};
use crate::questions::{from_file, inline, read_named};
use crate::scalars::decided;
use crate::signal::Invoke;
use crate::worker;
use crate::{connections, engines};

mod ffi;

pub(crate) use ffi::{register_usage, register_warm};

/// The usage table's rows: each counter summed over every engine this
/// process keeps. Counters have no reset.
pub(crate) fn usage_rows() -> Vec<(&'static str, i64)> {
    engines::usage_totals()
        .into_iter()
        .map(|(name, count)| (name, i64::try_from(count).unwrap_or(i64::MAX)))
        .collect()
}

/// One warm group: the question its first row bound and its distinct texts.
#[derive(Debug, Default)]
pub(crate) struct Warm {
    question: Option<String>,
    texts: BTreeSet<String>,
}

impl Warm {
    /// Take one row. A group judges one question.
    pub(crate) fn add(&mut self, question: &str, text: &str) -> Result<(), String> {
        self.bind(question)?;
        self.texts.insert(text.to_owned());
        Ok(())
    }

    /// Take another partial group of the same query.
    pub(crate) fn merge(&mut self, other: &mut Self) -> Result<(), String> {
        if let Some(question) = other.question.take() {
            self.bind(&question)?;
        }
        self.texts.append(&mut other.texts);
        Ok(())
    }

    fn bind(&mut self, question: &str) -> Result<(), String> {
        match self.question.as_deref() {
            None => {
                self.question = Some(question.to_owned());
                Ok(())
            }
            Some(bound) if bound == question => Ok(()),
            Some(_) => Err(usage(
                "thinkthen_warm judges one question per group, and this group carries more than one",
            )),
        }
    }

    /// Judge every distinct text once on the environment's engine, and
    /// answer how many were asked. A failure is the query's own error.
    /// `serial` names the kept connection of the database that registered
    /// this aggregate, which reads an `@file` question.
    pub(crate) fn finish(&mut self, serial: u64) -> Result<i64, String> {
        let texts: Vec<String> = std::mem::take(&mut self.texts).into_iter().collect();
        let Some(question) = self.question.take().filter(|_| !texts.is_empty()) else {
            return Ok(0);
        };
        let asked = i64::try_from(texts.len()).unwrap_or(i64::MAX);
        let invoke = Invoke::begin();
        let question = match question.strip_prefix('@') {
            Some(path) => kept_file(serial, path, &invoke)?,
            None => inline(&question)?,
        };
        let engine = engines::from_env()?;
        worker::run(&invoke, move |token| {
            decided(&engine, &question, texts, token, None, false).map(drop)
        })?;
        Ok(asked)
    }
}

/// Warm's `@file` (ticket 0129 decisions 3, 4, 6, and 7): read through the kept
/// connection of the database that registered warm, under its gate, so
/// DuckDB's own file settings decide. A running relate query refuses at
/// once, because a warm inside it would wait on the gate its own query holds.
fn kept_file(serial: u64, path: &str, invoke: &Invoke) -> Result<LoadedQuestion, String> {
    // The kept connection's session would expand `~` from its own
    // `home_directory`, not the caller's (decision 7).
    if path.starts_with('~') {
        return Err(usage(
            "thinkthen_warm cannot read an '@~' path, because home_directory is a session setting it cannot see; write the full path",
        ));
    }
    let kept = connections::by_serial(serial).ok_or_else(|| {
        usage(
            "thinkthen_warm cannot read '@file' here, because this database's kept connection was released when its last connection closed; reopen the database and LOAD the extension, or pass the file's JSON text",
        )
    })?;
    let _gate = kept.gate(|| {
        if kept.running() {
            Some(usage(
                "thinkthen_warm cannot read '@file' while a relate query runs on this database; run it before or after the relate, or pass the file's JSON text",
            ))
        } else if invoke.stopped() {
            Some(format!("{}the call was cancelled", prefix(ErrorKind::Cancelled)))
        } else {
            None
        }
    })?;
    let files = kept.connection().files()?;
    let text = read_named(&files, path, "question file")?;
    drop(files);
    Question::from_json(&text).map_err(|error| from_file(&error))
}
