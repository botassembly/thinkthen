//! Prepared C questions and composed records use the shared request constructors.
use crate::current::{QuestionHandle, question::Native};
use crate::failures::Failure;
use thinkthen::{
    Request, RequestArguments, RequestDefinition, RequestInput, RequestOptions, RequestQuestion,
};

pub(crate) fn prepare(
    kind: u32,
    question: &QuestionHandle,
    input: RequestInput,
) -> Result<Request, Failure> {
    if !question.fits_complete(kind) {
        return Err(Failure::usage(
            "the question kind does not match the named call",
        ));
    }
    let definition = match &question.native {
        Native::Atomic(q) => q.clone().into(),
        Native::DynamicChoose(q) => q.clone().into(),
        Native::Rank(q) | Native::Find(q) => q.clone().into(),
        Native::Set(q) => q.clone().into(),
        Native::RankSet(q) => q.clone().into(),
        Native::Recognize(q) => RequestDefinition::Recognition(q.clone()),
        Native::Relate(q) => q.clone().into(),
    };
    if kind == crate::ffi::values::THINKTHEN_FUNCTION_FILTER_V1 {
        return Ok(Request::for_function(
            thinkthen::RequestFunction::Filter,
            RequestArguments {
                question: RequestQuestion::Definition { value: definition },
                input,
                options: RequestOptions::default(),
            },
        ));
    }
    let definition = if kind == crate::ffi::values::THINKTHEN_FUNCTION_RANK_V1
        && matches!(question.native, Native::Atomic(_))
    {
        question.rank_reading()?.into()
    } else {
        definition
    };
    Ok(Request::for_definition(
        definition,
        input,
        RequestOptions::default(),
    ))
}
