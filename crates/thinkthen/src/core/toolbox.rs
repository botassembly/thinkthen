//! Strict named questions and their inherited settings.

use std::collections::BTreeMap;

use crate::core::{
    Ask, Choose, Decide, Error, Find, Function, Outcome, Pointer, Recognize, Relate, Score, Tag,
};

/// A validated collection of named questions.
#[derive(Debug, Clone)]
pub(crate) struct Toolbox(BTreeMap<String, ToolboxQuestion>);

/// One function and its validated question.
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) enum ToolboxFunction {
    /// A line selection.
    Find(Find),
    /// A yes or no question.
    Decide(Decide),
    /// A yes or no question that keeps matching records.
    Filter(Decide),
    /// A single choice.
    Choose(Choose),
    /// Independent labels.
    Tag(Tag),
    /// An ordered scale.
    Score(Score),
    /// Names with optional ordered kinds.
    Recognize(Recognize),
    /// Relations stated between recognized names.
    Relate(Relate),
}

/// A question with shared settings resolved against its own settings.
///
/// Its [`Function`] implementation applies the question and threshold. Callers
/// apply [`Self::on`] to their records before calling it.
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ToolboxQuestion {
    function: ToolboxFunction,
    #[serde(skip)]
    on: Option<Pointer>,
    #[serde(skip)]
    identity: String,
}

impl ToolboxQuestion {
    pub(crate) fn admitted(function: ToolboxFunction, on: Pointer, identity: String) -> Self {
        Self {
            function,
            on: Some(on),
            identity,
        }
    }
    pub(crate) fn stub_identity(&self) -> &str {
        &self.identity
    }
    /// The function this question runs.
    #[must_use]
    pub(crate) const fn function(&self) -> &ToolboxFunction {
        &self.function
    }
    /// The question's evidence pointer, including an inherited pointer.
    #[must_use]
    pub(crate) const fn on(&self) -> Option<&Pointer> {
        self.on.as_ref()
    }
}

macro_rules! dispatch {
    ($function:expr, $q:ident => $call:expr) => {
        match $function {
            ToolboxFunction::Decide($q) | ToolboxFunction::Filter($q) => $call,
            ToolboxFunction::Choose($q) => $call,
            ToolboxFunction::Tag($q) => $call,
            ToolboxFunction::Score($q) => $call,
            ToolboxFunction::Recognize($q) => $call,
            ToolboxFunction::Relate($q) => $call,
            ToolboxFunction::Find($q) => $call,
        }
    };
}

impl Function for ToolboxFunction {
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        dispatch!(self, q => q.try_asks_record(record, limits))
    }
    fn followup_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        dispatch!(context.question(), q => Function::followup_record(context.for_function(q), limits))
    }
    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Outcome {
        dispatch!(context.question(), q => Function::read_record(context.for_function(q), limits))
    }
}

impl Function for ToolboxQuestion {
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        dispatch!(&self.function, q => q.try_asks_record(record, limits))
    }
    fn followup_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        dispatch!(&context.question().function, q => Function::followup_record(context.for_function(q), limits))
    }
    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Outcome {
        dispatch!(&context.question().function, q => Function::read_record(context.for_function(q), limits))
    }
}

impl Toolbox {
    pub(crate) fn admitted(questions: BTreeMap<String, ToolboxQuestion>) -> Self {
        Self(questions)
    }
    /// Visit the named questions in name order.
    pub(crate) fn questions(&self) -> impl Iterator<Item = (&str, &ToolboxQuestion)> {
        self.0
            .iter()
            .map(|(name, question)| (name.as_str(), question))
    }
}
