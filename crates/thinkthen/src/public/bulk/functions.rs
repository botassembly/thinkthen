//! Released bulk convenience functions on the default engine.

use crate::public::{
    Answer, Batch, CallOptions, Choice, ChooseQuestion, DecisionQuestion, Evidence, Question, Row,
    TagQuestion, default_engine,
};

/// [`crate::Engine::decide_many`] on the [`default_engine`]; a failed build is the batch's first item.
pub fn decide_many<'a, I, Q: DecisionQuestion + ?Sized>(
    question: &'a Q,
    records: I,
) -> Batch<'a, Row<I::Item, Answer>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    decide_many_with(question, records, CallOptions::new())
}

/// [`crate::Engine::decide_many_with`] on the [`default_engine`]; a failed build is the batch's first item.
pub fn decide_many_with<'a, I, Q: DecisionQuestion + ?Sized>(
    question: &'a Q,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, Row<I::Item, Answer>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.decide_many_with(question, records, options),
        Err(error) => Batch::failed(error),
    }
}

/// [`crate::Engine::choose_many`] on the [`default_engine`]; a failed build is the first row.
pub fn choose_many<'a, I, C: Choice>(
    question: &'a ChooseQuestion<C>,
    records: I,
) -> Batch<'a, Row<I::Item, Option<C>>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    choose_many_with(question, records, CallOptions::new())
}

/// [`crate::Engine::choose_many_with`] on the [`default_engine`].
pub fn choose_many_with<'a, I, C: Choice>(
    question: &'a ChooseQuestion<C>,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, Row<I::Item, Option<C>>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.choose_many_with(question, records, options),
        Err(error) => Batch::failed(error),
    }
}

/// [`crate::Engine::score_many`] on the [`default_engine`]; a failed build is the first row.
pub fn score_many<'a, I>(question: &'a Question, records: I) -> Batch<'a, Row<I::Item, f64>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    score_many_with(question, records, CallOptions::new())
}

/// [`crate::Engine::score_many_with`] on the [`default_engine`].
pub fn score_many_with<'a, I>(
    question: &'a Question,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, Row<I::Item, f64>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.score_many_with(question, records, options),
        Err(error) => Batch::failed(error),
    }
}

/// [`crate::Engine::tag_many`] on the [`default_engine`]; a failed build is the first row.
pub fn tag_many<'a, I, C: Choice>(
    question: &'a TagQuestion<C>,
    records: I,
) -> Batch<'a, Row<I::Item, Vec<C>>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    tag_many_with(question, records, CallOptions::new())
}

/// [`crate::Engine::tag_many_with`] on the [`default_engine`].
pub fn tag_many_with<'a, I, C: Choice>(
    question: &'a TagQuestion<C>,
    records: I,
    options: CallOptions<'a>,
) -> Batch<'a, Row<I::Item, Vec<C>>>
where
    I: IntoIterator + 'a,
    I::Item: Evidence,
{
    match default_engine() {
        Ok(engine) => engine.tag_many_with(question, records, options),
        Err(error) => Batch::failed(error),
    }
}
