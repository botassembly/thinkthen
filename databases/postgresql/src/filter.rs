//! Plain keyed filter values retain their caller keys.
use crate::call::{self, OrRaise as _};
use pgrx::prelude::*;

crate::descriptions::describe! {
"Return retained keyed text values. NULL question or input returns no rows. Failures raise SQL errors; evidence files must be read by the client.";
[name = "thinkthen_filter", parallel_restricted];
fn filter(
    question: Option<&str>, input: Option<pgrx::datum::JsonB>,
    settings: default!(Option<crate::ffi::RawJson>, "NULL"),
) -> TableIterator<'static, (name!(key, String), name!(value, String))> {
    call::guarded(|| {
        let (Some(question), Some(input)) = (question, input) else { return TableIterator::new(Vec::new()); };
        let (settings, controls) = crate::forms::controls(settings.as_ref(), Default::default());
        let question = crate::forms::question_result(Some(question), None, thinkthen::For::Decide, &settings).or_raise();
        let question = match question { thinkthen::LoadedQuestion::Question(question) => question,
            _ => call::raise(call::usage("filter requires a decide question")) };
        let records = crate::forms::keyed(Some(input));
        let (keys, texts): (Vec<_>, Vec<_>) = records.into_iter().unzip();
        let rows = call::run(controls, move |engine, controls| {
            let result = crate::request::run(engine, thinkthen::RequestFunction::Filter,
                question.into(), texts.into_iter().map(crate::request::text).collect(), controls)?;
            match result.into_value() { thinkthen::RequestValue::Filtered(rows) => Ok(rows), _ => Err(crate::request::wrong_result()) }
        });
        TableIterator::new(rows.into_iter().filter(|row| row.result().value()).map(|row| {
            let key = keys.get(row.ordinal()).cloned().unwrap_or_else(|| call::raise(crate::request::wrong_result()));
            let value = match row.original() { thinkthen::QuestionInput::Text(text) => text.clone(), _ => call::raise(crate::request::wrong_result()) };
            (key, value)
        }).collect::<Vec<_>>())
    })
}
}
