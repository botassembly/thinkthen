//! Complete-set rank and find admission, shared by fallible and original callers.

use super::observation::observe_find;
use crate::core::{self, Find, ranking};
use crate::public::engine::{Engine, Evidence, evidence, only};
use crate::public::error::Error;
use crate::public::options::{CallOptions, Stop};
use crate::public::question::{Kind, Question};
use crate::public::results::{Call, Found, Ranked};
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
    ) -> Result<Call<Vec<Ranked<I::Item>>>, Error>
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
    ) -> Result<Call<Vec<Ranked<I::Item>>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.try_rank_with(question, records.into_iter().map(Ok), options)
    }

    /// Fallible complete-set input for [`Engine::rank_with`].
    ///
    /// # Errors
    /// Returns an input failure before sending, or the call errors of
    /// [`Engine::rank_with`].
    pub fn try_rank_with<I, T>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<Ranked<T>>>, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
        T: Evidence,
    {
        only(question, &[Kind::Rank], "rank")?;
        let records = self.try_within_limit(records)?;
        // A rank judges every record before it orders any, so a blank record
        // is refused before the first send.
        for (at, record) in records.as_slice().iter().enumerate() {
            question
                .metadata
                .validate_item(&super::super::InputEvidence::question_input(record))
                .map_err(|error| error.at_record(at))?;
            evidence(record.evidence())?;
        }
        let mut batch = self.decisions(question, records, options, |item, (_, yes)| {
            Ok(Some((item, yes)))
        })?;
        let rows = batch.by_ref().collect::<Result<Vec<_>, _>>()?;
        let facts = batch
            .facts()
            .cloned()
            .ok_or_else(|| Error::defect("a completed rank has no facts"))?;
        let order = ranking(&rows.iter().map(|(_, yes)| *yes).collect::<Vec<_>>(), None);
        // Rows arrive in input order, so a row's place is its input index.
        let mut rows: Vec<Option<(T, f64)>> = rows.into_iter().map(Some).collect();
        Ok(Call::new(
            order
                .into_iter()
                .filter_map(|place| {
                    let (item, yes) = rows.get_mut(place).and_then(Option::take)?;
                    Some(Ranked::new(place, item, yes))
                })
                .collect(),
            facts,
        ))
    }

    /// Select the one unit that best answers the question, from 2 to 255.
    /// A question from [`Question::offering_none`] takes 2 to 254 units and
    /// may select none.
    ///
    /// # Errors
    ///
    /// As [`Engine::decide`], and [`Error::Usage`] for another kind of
    /// question or a unit count outside its range.
    pub fn find<I>(&self, question: &Question, units: I) -> Result<Call<Found<I::Item>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
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
    ) -> Result<Call<Found<I::Item>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.try_find_with(question, units.into_iter().map(Ok), options)
    }

    /// Fallible complete-set input for [`Engine::find_with`].
    /// Admission stops at the first count, evidence-byte or reader failure.
    ///
    /// # Errors
    /// Returns an input failure before sending, or the call errors of
    /// [`Engine::find_with`].
    pub fn try_find_with<I, T>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Found<T>>, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
        T: Evidence,
    {
        let (units, find, engine) = self.prepare_find(question, units, &options)?;
        for (at, unit) in units.iter().enumerate() {
            question
                .metadata
                .validate_item(&super::super::InputEvidence::question_input(unit))
                .map_err(|error| error.at_record(at))?;
        }
        let none = question.kind == Kind::FindNone;
        let stop = Stop::begin(options)?.with_prices(self.prices);
        stop.run_call(1, |cancel| {
            let found = engine.find(&find, cancel).map_err(Error::from)?;
            observe_find(&stop, &engine, question, &find, &found)?;
            Ok(found)
        })?
        .try_map(|found| Found::new(units, none, &found))
    }

    pub(in crate::public) fn prepare_find<I, T>(
        &self,
        question: &Question,
        units: I,
        options: &CallOptions<'_>,
    ) -> Result<PreparedFind<T>, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
        T: Evidence,
    {
        only(question, &[Kind::Find, Kind::FindNone], "find")?;
        let none = question.kind == Kind::FindNone;
        let maximum = if none { 254 } else { 255 };
        let count_message = if none {
            "a find question offering none takes 2 to 254 units"
        } else {
            "find takes 2 to 255 units"
        };
        let mut held = Vec::new();
        let mut bytes = 0usize;
        for unit in units {
            let unit = unit?;
            self.check_record_limit(held.len())?;
            if held.len() == maximum {
                return Err(Error::usage(count_message));
            }
            bytes = bytes
                .checked_add(unit.evidence().len())
                .filter(|&bytes| bytes <= 16 * 1024 * 1024)
                .ok_or_else(|| Error::usage("find input exceeds 16 MiB"))?;
            held.push(unit);
        }
        let units = held;
        let texts = units
            .iter()
            .map(|unit| evidence(unit.evidence()))
            .collect::<Result<Vec<_>, _>>()?;
        let core::Question::Decide { text, .. } = &question.core else {
            return Err(Error::defect("a find question held no text"));
        };
        let engine = crate::public::complete::contextual(self.asking(question)?, options)?;
        let find = Find::new(text.clone(), &texts, engine.backend().model().clone(), none)
            .map_err(|_| Error::usage(count_message))?;
        let find = crate::public::find_question::profiled(find, question);
        Ok((units, find, engine))
    }

    /// Hold a finite input whole, and refuse it over the request limit.
    pub(in crate::public) fn within_limit<I: IntoIterator>(
        &self,
        records: I,
    ) -> Result<std::vec::IntoIter<I::Item>, Error> {
        self.try_within_limit(records.into_iter().map(Ok))
    }

    pub(in crate::public) fn try_within_limit<I, T>(
        &self,
        records: I,
    ) -> Result<std::vec::IntoIter<T>, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
    {
        self.try_within_admission(records, &CallOptions::new())
    }
}
