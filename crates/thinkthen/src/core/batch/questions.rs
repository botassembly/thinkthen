//! Complete per-record questions beside the existing fixed-question batch path.

use sha2::{Digest as _, Sha256};

use super::{
    Batch, BatchError, BatchRecord, Batcher, CUT, Closed, Joined, MEMBERS, Open, Question, Setting,
    defect, text,
};
use crate::core::backend::Backend;
use crate::core::backend_profile::BackendProfile;
use crate::core::render::json_line;
use crate::core::text::{Evidence, QuestionText};

impl Batcher {
    /// Add one record with the complete question resolved for that record.
    pub(crate) fn push_with_question(
        &mut self,
        record: BatchRecord,
        question: Question,
        closed: &mut Vec<Batch>,
    ) -> Result<(), BatchError> {
        let line = json_line(&record.value).map_err(|_| defect())?;
        let cut = Sha256::digest(line.as_bytes())
            .first_chunk::<8>()
            .is_some_and(|head| u64::from_be_bytes(*head) % CUT == 0);
        let key = (line.clone(), json_line(&question).map_err(|_| defect())?);
        if let Some(&place) = self.open.seen.get(&key) {
            self.open.members.push(place);
        } else {
            let joined = self.joined(record, &line, question)?;
            if !self.open.distinct.is_empty() && !self.fits(&joined) {
                closed.push(self.close(Closed::Limit)?);
            }
            if self.context.is_some() {
                self.context_fits(joined.share, joined.wire)?;
            }
            self.add(joined, key);
            if self.profile.is_some()
                && self.open.distinct.len() == 1
                && let Err(error) = self.built()
            {
                self.open = Open::default();
                return Err(error);
            }
        }
        if cut {
            closed.push(self.close(Closed::Content)?);
        } else if self.size == Some(self.open.members.len()) {
            closed.push(self.close(Closed::Size)?);
        } else if self.open.members.len() == MEMBERS {
            closed.push(self.close(Closed::Limit)?);
        }
        Ok(())
    }

    /// Quote one record and measure its batched share from one encode.
    fn joined(
        &self,
        record: BatchRecord,
        line: &str,
        base: Question,
    ) -> Result<Joined, BatchError> {
        let question = self.quoted(line, &base)?;
        let (wire, alone) = match &question {
            Some(asked) => self.measured(&[(&record.value, asked)])?,
            None => (1, self.skeleton),
        };
        let share = alone.checked_sub(self.skeleton).ok_or_else(defect)?;
        Ok(Joined {
            record,
            base,
            question,
            wire,
            share,
            line_len: line.len(),
        })
    }

    /// Keep this row's complete options when quoting its text.
    pub(super) fn quoted(
        &self,
        line: &str,
        base: &Question,
    ) -> Result<Option<Question>, BatchError> {
        let mut question = base.clone();
        let Some(asked) = text(&mut question).as_json().as_str().map(str::to_owned) else {
            return Ok(None);
        };
        *text(&mut question) =
            QuestionText::new(format!("The text is {line}. {asked}")).map_err(|_| defect())?;
        Ok(Some(question))
    }
}

/// Rebuild a fixed-question refused batch as two ordinary requests.
#[cfg(test)]
pub(crate) fn halves(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    question: &Question,
    context: Option<&Evidence>,
    records: Vec<BatchRecord>,
) -> Result<[Batch; 2], BatchError> {
    halves_with_questions(
        backend,
        profile,
        context,
        records
            .into_iter()
            .map(|record| (record, question.clone()))
            .collect(),
    )
}

/// Rebuild a refused batch without substituting one row's options for another's.
pub(crate) fn halves_with_questions(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    context: Option<&Evidence>,
    mut records: Vec<(BatchRecord, Question)>,
) -> Result<[Batch; 2], BatchError> {
    if records.len() < 2 {
        return Err(BatchError::Defect("a batch needs two members to halve"));
    }
    let second = records.split_off(records.len().div_ceil(2));
    let build = |records: Vec<(BatchRecord, Question)>| -> Result<Batch, BatchError> {
        let first = records.first().ok_or_else(defect)?.1.clone();
        let mut batcher = Batcher::new(
            backend.clone(),
            profile.cloned(),
            first,
            Setting::Max,
            context.cloned(),
        )?;
        let mut closed = Vec::new();
        for (record, question) in records {
            batcher.push_with_question(record, question, &mut closed)?;
        }
        if let Some(batch) = batcher.finish()? {
            closed.push(batch);
        }
        let [batch] = <[Batch; 1]>::try_from(closed)
            .map_err(|_| BatchError::Defect("a half formed more than one batch"))?;
        Ok(batch)
    };
    Ok([build(records)?, build(second)?])
}
