//! Runtime-question record details over the shared stopping batch planner.

use serde::Serialize;

use crate::core;
use crate::public::asking::Decisions;
use crate::public::batch::Batch;
use crate::public::error::Error;
use crate::public::pull;
use crate::public::engine::{DetailQuestion, Engine, Evidence, evidence, only};
use crate::public::options::{CallOptions, Stop};
use crate::public::question::Kind;
use crate::public::results::{Details, Row};

use super::selected_batch;

impl Engine {
    /// Each record's full detail under a runtime or typed question, in input order.
    /// The input must serialize to JSON so its whole original value can appear
    /// under `input` in the detail document.
    pub fn details_many<'a, I, Q: DetailQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        records: I,
    ) -> Batch<'a, Row<I::Item, Details>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence + Serialize,
    {
        self.details_many_with(question, records, CallOptions::new())
    }

    /// [`Engine::details_many`] under these controls.
    pub fn details_many_with<'a, I, Q: DetailQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<I::Item, Details>>
    where
        I: IntoIterator + 'a,
        I::Item: Evidence + Serialize,
    {
        let question = question.question();
        Batch::of((|| {
            only(
                question,
                &[
                    Kind::Decide,
                    Kind::Banded,
                    Kind::Choose,
                    Kind::Tag,
                    Kind::Score,
                ],
                "details_many",
            )?;
            let setting = selected_batch(question, &options, self.batch)?;
            let context_sha256 = options
                .context_text()
                .map(|text| core::bytes_sha256(text.as_bytes()));
            let context = options.context_text().map(evidence).transpose()?;
            let stop = Stop::begin(options)?.with_prices(self.prices);
            let engine = self.asking(question)?;
            let asker = Decisions::new(&engine, question, context.clone());
            let backend = engine.backend().clone();
            let profile = self.profile.clone();
            let call = pull::Call {
                packing: pull::packing(setting, context.is_some(), false),
                engine,
                stop,
                most: self.most,
            };
            Ok(pull::start(
                call,
                asker,
                records.into_iter(),
                Box::new(move |stop, index, item, row| {
                    let item = item.ok_or_else(|| Error::defect("a row arrived with no record"));
                    let decided = match row {
                        Ok(decided) => decided,
                        failed => {
                            super::judged(stop, question, &backend, index, failed)?;
                            return Err(Error::defect("a failed row returned a value"));
                        }
                    };
                    let member = decided.member(question)?;
                    super::judged(stop, question, &backend, index, Ok(decided))?;
                    let item = item?;
                    let details = Details::of_member(
                        member,
                        &item,
                        question,
                        &backend,
                        profile.as_ref(),
                        setting,
                        context_sha256.as_deref(),
                    )?;
                    Ok(Some(Row::new(item, details, 0.0)))
                }),
            ))
        })())
    }
}
