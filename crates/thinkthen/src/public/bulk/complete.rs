//! Complete-set rank and find admission, shared by fallible and original callers.

use crate::core::{self, Find};
use crate::public::engine::{Engine, Evidence, evidence, evidence_error, only};
use crate::public::error::Error;
use crate::public::options::CallOptions;
use crate::public::question::{Kind, Question};
use crate::public::results::{Call, CompleteFound, CompleteRank, CompleteRecord};
type PreparedFind<T> = (Vec<T>, Find, std::sync::Arc<crate::engine::facade::Engine>);

impl Engine {
    /// Every record, most likely yes first; ties keep input order.
    ///
    /// # Errors
    ///
    /// The first record's [`Error`], and [`Error::Usage`] for another kind of
    /// question or more records than the engine's request limit.
    #[allow(
        clippy::type_complexity,
        reason = "the public return carries ranked rows and facts"
    )]
    pub fn rank<I>(
        &self,
        question: &Question,
        records: I,
    ) -> Result<Call<Vec<usize>, Vec<CompleteRecord<I::Item, CompleteRank>>, Vec<Option<f64>>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.rank_with(question, records, CallOptions::new())
    }

    /// [`Engine::rank`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::rank`].
    #[allow(
        clippy::type_complexity,
        reason = "the public return carries ranked rows and facts"
    )]
    pub fn rank_with<I>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<usize>, Vec<CompleteRecord<I::Item, CompleteRank>>, Vec<Option<f64>>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        only(question, &[Kind::Rank], "rank")?;
        self.try_rank_records_complete_with(
            question,
            records
                .into_iter()
                .map(|row| Ok(crate::public::record_input::plain(row))),
            options,
        )?
        .project(|rows| {
            let values = rows.iter().map(|row| row.ordinal).collect();
            let probabilities = rows.iter().map(|row| row.result.probability()).collect();
            Ok((values, rows, probabilities))
        })
    }

    /// Select the one unit that best answers the question, from 2 to 255.
    /// A question from [`Question::offering_none`] takes 1 to 254 units and
    /// may select none.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`], and [`Error::Usage`] for another kind of
    /// question or a unit count outside its range.
    pub fn find<I>(
        &self,
        question: &Question,
        units: I,
    ) -> Result<Call<Option<I::Item>, CompleteFound<I::Item>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence + Clone,
    {
        self.find_with(question, units, CallOptions::new())
    }

    /// [`Engine::find`] under these controls.
    ///
    /// # Errors
    ///
    /// As [`Engine::find`].
    pub fn find_with<I>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Option<I::Item>, CompleteFound<I::Item>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence + Clone,
    {
        self.try_find_complete_with(question, units.into_iter().map(Ok), options)?
            .project(|result| {
                let value = result.found.selected().cloned();
                let probability = result.probability();
                Ok((value, result, probability))
            })
    }

    pub(in crate::public) fn prepare_find<T>(
        &self,
        question: &Question,
        units: Vec<T>,
        options: &CallOptions<'_>,
    ) -> Result<PreparedFind<T>, Error>
    where
        T: Evidence,
    {
        let none = question.kind == Kind::FindNone;
        let texts = units
            .iter()
            .map(|unit| evidence(unit.evidence()))
            .collect::<Result<Vec<_>, _>>()?;
        let core::Question::Decide { text, .. } = &question.core else {
            return Err(Error::defect("a find question held no text"));
        };
        let engine = crate::public::complete::contextual(self.asking(question)?, options)?;
        let find = Find::new(text.clone(), &texts, engine.backend().model().clone(), none)
            .map_err(|_| Error::usage(count_message(none)))?;
        let find = crate::public::find_question::profiled(find, question);
        Ok((units, find, engine))
    }

    /// Hold a finite input whole, and refuse it over the request limit.
    pub(in crate::public) fn within_limit<I: IntoIterator>(
        &self,
        records: I,
    ) -> Result<std::vec::IntoIter<I::Item>, Error> {
        self.try_within_admission(records.into_iter().map(Ok), &CallOptions::new())
    }
}

impl Question {
    /// Validate a complete borrowed find set before copying or sending it.
    /// Reader errors and the first excess unit stop intake without reading the tail.
    /// This checks find semantics; engine limits and call controls apply at execution.
    ///
    /// # Errors
    /// Returns the first reader failure or [`Error::Usage`] for a wrong question
    /// kind, invalid unit count, aggregate byte overflow or blank evidence.
    pub fn admit_find_units<'a>(
        &self,
        units: impl IntoIterator<Item = Result<&'a str, Error>>,
    ) -> Result<(), Error> {
        let units = admit_find(self, units, &CallOptions::new(), |_| Ok(()))?;
        let none = self.kind == Kind::FindNone;
        Find::validate_count(units.len(), none).map_err(|_| Error::usage(count_message(none)))
    }
}

pub(in crate::public) fn count_message(none: bool) -> &'static str {
    if none {
        "a find question offering none takes 1 to 254 units"
    } else {
        "find takes 2 to 255 units"
    }
}

fn admit_find<I, T>(
    question: &Question,
    units: I,
    options: &CallOptions<'_>,
    check_limit: impl Fn(usize) -> Result<(), Error>,
) -> Result<Vec<T>, Error>
where
    I: IntoIterator<Item = Result<T, Error>>,
    T: Evidence,
{
    only(question, &[Kind::Find, Kind::FindNone], "find")?;
    let none = question.kind == Kind::FindNone;
    let maximum = Find::maximum(none);
    let mut held = Vec::new();
    let mut bytes = crate::public::SourceBudget::find();
    let mut units = units.into_iter();
    loop {
        options.admission()?;
        let Some(unit) = units.next() else { break };
        options.admission()?;
        let unit = unit?;
        check_limit(held.len())?;
        if held.len() == maximum {
            return Err(Error::usage(count_message(none)));
        }
        bytes.charge(unit.evidence().len())?;
        held.push(unit);
    }
    for unit in &held {
        core::Evidence::validate_text(unit.evidence()).map_err(evidence_error)?;
    }
    Ok(held)
}
