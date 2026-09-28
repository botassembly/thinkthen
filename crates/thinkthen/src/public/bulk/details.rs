//! Runtime-question record details over the shared stopping batch planner.

use serde::Serialize;

use crate::core;
use crate::public::batch::{self, Batch};
use crate::public::engine::{DetailQuestion, Engine, Evidence, Sealed, evidence, only};
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
            let stop = Stop::begin(options)?;
            let engine = self.asking(question)?;
            batch::start_details(
                engine,
                records.into_iter(),
                stop,
                self.most,
                question,
                setting,
                context,
                context_sha256,
                self.profile.as_ref(),
            )
        })())
    }
}
