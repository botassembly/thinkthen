//! Additive concrete complete calls retain the existing transport and scalar interfaces.
mod annotate;
mod dynamic_choose;
mod find;
#[cfg(test)]
mod ownership_tests;
mod rank;
mod rank_set;
pub(crate) mod recognize;
pub(in crate::public) mod records;
mod recovering;
mod recovering_stream;
pub(crate) mod relate;
mod streaming;
use crate::core::{self, Value};
use crate::public::question::Question;
use crate::public::{
    self, Call, CallOptions, CompleteChoice, CompleteDecision, CompleteScore, CompleteTags,
    DecisionQuestion, Engine, Error, InputFunction, QuestionInput,
};
use crate::result_json::{
    Run,
    complete::{AtomicSpec, atomic},
};

/// Prepare a preview through the same original/context/shortlist admission as execution.
pub(in crate::public) fn preview_asks(
    engine: &std::sync::Arc<crate::engine::facade::Engine>,
    function: InputFunction,
    question: &Question,
    record: crate::RecordInput<QuestionInput>,
    fallback: Option<&str>,
    at: usize,
) -> Result<(Vec<crate::core::pack::Ask>, bool), Error> {
    use crate::engine::pipeline::Asker as _;
    let (held, input) = records::prepare_record(function, question, record, fallback, at)?;
    let dropped = crate::core::adapters::built_in::drops_detail_of(
        engine.backend().descriptions(),
        &held.question.core,
    );
    let asks = records::Records(std::sync::Arc::clone(engine), false)
        .asks(&input)
        .map_err(records::pipeline_failure)?;
    Ok((asks, dropped))
}

impl Engine {
    /// Complete decision for explicitly typed text or ordered images.
    ///
    /// # Errors
    /// Refuses unsupported image routes before sending, otherwise as decide_with.
    pub(crate) fn decide_input_complete_with<Q: DecisionQuestion + ?Sized>(
        &self,
        question: &Q,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteDecision>, Error> {
        self.atomic_complete(InputFunction::Decide, question.question(), input, options)?
            .try_map(decision)
    }

    pub(in crate::public) fn atomic_complete(
        &self,
        function: InputFunction,
        question: &Question,
        input: &QuestionInput,
        options: CallOptions<'_>,
    ) -> Result<Call<core::CompleteAtomic>, Error> {
        crate::public::request::scalar(self, function, question, input, options)
    }
}

fn run<'a>(
    engine: &'a crate::engine::facade::Engine,
    question: &'a Question,
    profile: Option<&'a core::BackendProfile>,
) -> Run<'a> {
    Run {
        backend: engine.backend(),
        tuned_for: question.profile.as_ref(),
        warning: core::ProfileWarning::between(
            question.profile.as_ref(),
            profile.map(core::BackendProfile::name),
        ),
        batch_setting: None,
        batch_warning: None,
        context_sha256: None,
    }
}

fn batch_run<'a>(
    engine: &'a crate::engine::facade::Engine,
    question: &'a Question,
    profile: Option<&'a core::BackendProfile>,
    setting: core::Setting,
) -> Run<'a> {
    let text = match &question.core {
        core::Question::Decide { text, .. }
        | core::Question::Choose { text, .. }
        | core::Question::Tag { text, .. }
        | core::Question::Score { text, .. } => text,
    };
    let setting = if text.as_json().as_str().is_some() {
        setting
    } else {
        core::Setting::Records(std::num::NonZeroUsize::MIN)
    };
    let mut result = run(engine, question, profile);
    result.batch_setting = Some(setting.into());
    result.batch_warning = super::results::batch_warning(question, setting);
    result
}

pub(in crate::public) fn spec(
    function: InputFunction,
    question: &Question,
    shown: Value,
    record: usize,
) -> AtomicSpec {
    AtomicSpec {
        declarations: question.reading_metadata(),
        function,
        record,
        question: question.core.clone(),
        threshold: question.threshold,
        shown,
        rank_position: None,
    }
}

pub(crate) fn decision(canonical: core::CompleteAtomic) -> Result<CompleteDecision, Error> {
    let Value::YesNo(value) = *canonical.value() else {
        return Err(wrong());
    };
    let value = match value {
        Some(true) => public::Answer::Yes,
        Some(false) => public::Answer::No,
        None => public::Answer::Unsure,
    };
    Ok(CompleteDecision { canonical, value })
}
pub(crate) fn choice(canonical: core::CompleteAtomic) -> Result<CompleteChoice, Error> {
    let Value::Choice(value) = canonical.value() else {
        return Err(wrong());
    };
    Ok(CompleteChoice {
        value: value.clone(),
        canonical,
    })
}
pub(crate) fn tags(canonical: core::CompleteAtomic) -> Result<CompleteTags, Error> {
    let Value::Tag(value) = canonical.value() else {
        return Err(wrong());
    };
    Ok(CompleteTags {
        value: value.clone(),
        canonical,
    })
}
pub(crate) fn score(canonical: core::CompleteAtomic) -> Result<CompleteScore, Error> {
    let Value::Score(value) = *canonical.value() else {
        return Err(wrong());
    };
    Ok(CompleteScore { canonical, value })
}
fn wrong() -> Error {
    Error::defect("a complete answer has another function's value")
}

pub(crate) fn contextual(
    engine: std::sync::Arc<crate::engine::facade::Engine>,
    options: &CallOptions<'_>,
) -> Result<std::sync::Arc<crate::engine::facade::Engine>, Error> {
    let Some(context) = options.context_text().filter(|text| !text.is_empty()) else {
        return Ok(engine);
    };
    crate::public::engine::evidence(context)?;
    Ok(std::sync::Arc::new(
        (*engine)
            .clone()
            .with_aggregate_context(Some(context.to_owned())),
    ))
}

pub(super) fn physical_source(input: &QuestionInput) -> Option<core::CompletePhysicalSource> {
    let source = match input {
        QuestionInput::Text(_) => None,
        QuestionInput::Images(images) => images.location(),
        QuestionInput::Record(record) => record.location(),
    }?;
    Some(core::CompletePhysicalSource {
        file: source.file().to_owned(),
        first_line: source.first_line(),
        last_line: source.last_line(),
    })
}

fn ancillary_images(input: &QuestionInput) -> Option<Vec<core::image::Image>> {
    match input {
        QuestionInput::Images(images) => Some(
            images
                .images()
                .iter()
                .map(|image| image.0.clone())
                .collect(),
        ),
        QuestionInput::Record(record) if !record.images().is_empty() => Some(
            record
                .images()
                .iter()
                .map(|image| image.0.clone())
                .collect(),
        ),
        _ => None,
    }
}

pub(in crate::public) fn preview_definition_asks(
    engine: &std::sync::Arc<crate::engine::facade::Engine>,
    function: InputFunction,
    definition: &crate::RequestDefinition,
    record: crate::RecordInput<crate::QuestionInput>,
    options: &CallOptions<'_>,
    at: usize,
) -> Result<(Vec<crate::core::pack::Ask>, bool), Error> {
    use crate::{LoadedQuestion, RequestDefinition as Definition};
    match definition {
        Definition::Atomic(LoadedQuestion::Question(question)) | Definition::Rank(question) => {
            preview_asks(
                engine,
                function,
                question,
                record,
                options.context_text(),
                at,
            )
        }
        Definition::Atomic(LoadedQuestion::Banded(question)) => preview_asks(
            engine,
            function,
            &question.0,
            record,
            options.context_text(),
            at,
        ),
        Definition::RankSet(set) => rank_set::preview_asks(engine, set, record, options, at),
        Definition::DynamicChoose(question) => {
            dynamic_choose::preview_asks(engine, question, record, options.context_text(), at)
        }
        Definition::Annotate(set) => annotate::preview_asks(engine, set, record, options, at),
        _ => Err(Error::usage("plan requires a record question")),
    }
}

pub(in crate::public) use annotate::preview_grouped_asks as annotation_preview_asks;
