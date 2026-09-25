//! `thinkthen_usage()` and the `thinkthen_warm` aggregate (ticket 0110
//! decisions 5 and 10).

use std::collections::BTreeSet;

use crate::engines;
use crate::errors::usage;
use crate::questions::inline;
use crate::scalars::decided;
use crate::signal::Invoke;
use crate::worker;

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
    pub(crate) fn finish(&mut self) -> Result<i64, String> {
        let texts: Vec<String> = std::mem::take(&mut self.texts).into_iter().collect();
        let Some(question) = self.question.take().filter(|_| !texts.is_empty()) else {
            return Ok(0);
        };
        if question.starts_with('@') {
            return Err(usage(
                "thinkthen_warm reads no '@file' question; pass the file's text from DuckDB's read_text, which applies this database's file settings",
            ));
        }
        let asked = i64::try_from(texts.len()).unwrap_or(i64::MAX);
        let question = inline(&question)?;
        let engine = engines::from_env()?;
        let invoke = Invoke::begin();
        worker::run(&invoke, move |token| {
            decided(&engine, &question, texts, token, None, false).map(drop)
        })?;
        Ok(asked)
    }
}
