//! Runtime-question record details on the question pipeline.

use serde::Serialize;

use crate::public::batch::Batch;
use crate::public::engine::{DetailQuestion, Engine};
use crate::public::error::Error;
use crate::public::options::CallOptions;
use crate::public::results::{Details, Row};

impl Engine {
    /// Full details over fallible explicit input items, including image files.
    /// # Errors
    /// Reader failures retain earlier completed rows; image admission sends nothing.
    pub(crate) fn try_details_input_many_with<'a, I, T, Q: DetailQuestion + ?Sized>(
        &'a self,
        question: &'a Q,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, Row<T, Details>>
    where
        I: IntoIterator<Item = Result<T, Error>> + 'a,
        T: crate::public::InputEvidence + Serialize + 'a,
    {
        let question = question.question();
        if !matches!(
            question.kind(),
            crate::QuestionKind::Decide
                | crate::QuestionKind::Choose
                | crate::QuestionKind::Tag
                | crate::QuestionKind::Score
        ) {
            return Batch::failed(Error::usage("details_many takes a primitive question"));
        }
        Batch::of(crate::public::request::pull::project(
            self,
            crate::public::request::RequestFunction::for_definition(&question.clone().into())
                .input_function(),
            question.clone(),
            records
                .into_iter()
                .map(|row| row.map(crate::public::record_input::plain)),
            options,
            |original, _, canonical| {
                let details = Details::of_complete(&canonical, Some(&original))?;
                Ok(Some(Row::new(original, details, 0.0)))
            },
        ))
    }
}
