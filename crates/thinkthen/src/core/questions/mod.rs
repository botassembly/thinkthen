//! The canonical typed question contract, independent of backend wire formats.
#![allow(
    clippy::disallowed_types,
    reason = "schema derives construct metadata; question values remain typed"
)]

use crate::core::{Cut, Error, ErrorKind, Pointer, Threshold};
mod annotation;
mod choose_execution;
pub(crate) use choose_execution::complete_choice;
mod collection;
mod domains;
mod predicates;
#[cfg(feature = "native")]
pub(crate) mod recording;
mod relations;
mod saved;
#[cfg(feature = "command")]
pub(crate) use saved::validate_saved_name;
#[cfg(feature = "command")]
pub(crate) mod command;
mod threshold_input;
pub use annotation::Annotate;
pub(crate) use annotation::AnnotateWire;
use collection::RankWire;
pub use collection::{Rank, Toolbox};
pub use domains::{Choose, Recognize, Score, Tag};
use domains::{ChooseWire, RecognizeWire, ScoreWire, TagWire};
pub use predicates::{Decide, Filter, Find};
use predicates::{DecideWire, FilterWire, FindWire};
use relations::RelateWire;
pub use relations::{Relate, RelationRule};
use threshold_input::ThresholdInput;

pub(crate) const TOOLBOX_LIMIT: usize = 1024 * 1024;

const ANNOTATE_ABI: u32 = 10;
pub(crate) const MIN_CHOICES: usize = 2;
pub(crate) const MIN_LABELS: usize = 1;
pub(crate) const MIN_LEVELS: usize = 2;
pub(crate) const MIN_KINDS: usize = 1;
pub(crate) const MIN_RULES: usize = 1;
pub(crate) const MIN_QUESTIONS: usize = 1;

fn question_error(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Question, message)
}

macro_rules! functions {
    (annotatable { $($member:ident : $member_wire:ident = $member_abi:literal => $member_command:expr),+ $(,)? } collection { $rank:ident : $rank_wire:ident = $rank_abi:literal => $rank_command:expr } annotation { $annotate_command:expr }) => {
        functions!(@all [$($member:$member_wire=$member_abi => $member_command,)+ $rank:$rank_wire=$rank_abi => $rank_command] [$($member:$member_wire),+] $annotate_command);
    };
    (@all [$($variant:ident:$wire:ident=$abi:literal => $command:expr),+] [$($member:ident:$member_wire:ident),+] $annotate_command:expr) => {
        /// A question's stable function discriminator.
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #[serde(rename_all = "lowercase")]
        #[repr(u32)]
        pub enum FunctionName { $(#[doc = stringify!($variant)] $variant=$abi,)+ /// Named independent questions.
            Annotate=ANNOTATE_ABI }
        impl FunctionName {
            /// Every function in the shared contract.
            pub const ALL:&'static [Self]=&[$(Self::$variant,)+Self::Annotate];
            #[cfg(feature = "command")]
            pub(crate) fn command(self) -> command::CommandSpec {
                let (description, threshold, check, list) = match self {
                    $(Self::$variant => $command,)+
                    Self::Annotate => $annotate_command,
                };
                command::CommandSpec { description, threshold, check, list }
            }
            /// Its explicit stable native discriminator.
            #[must_use]
            pub const fn abi(self)->u32{self as u32}
            /// Reject unknown native discriminators.
            #[must_use]
            pub const fn from_abi(value:u32)->Option<Self>{match value{$($abi=>Some(Self::$variant),)+ANNOTATE_ABI=>Some(Self::Annotate),_=>None}}
        }

        /// A typed question that may be selected from a toolbox.
        #[non_exhaustive]
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #[serde(tag = "function", rename_all = "lowercase", try_from = "RawSimpleQuestion")]
        #[schemars(with = "RawSimpleQuestion")]
        pub enum SimpleQuestion { $(#[doc = stringify!($variant)] $variant($variant),)+ }

        /// A per-record question that may be nested under annotation.
        /// ```compile_fail
        /// use thinkthen::questions::{AnnotatableQuestion, Rank};
        /// let question = Rank::new("How much?", ["low", "high"]).unwrap();
        /// let _: AnnotatableQuestion = question.into();
        /// ```
        /// ```compile_fail
        /// use thinkthen::questions::{AnnotatableQuestion, Annotate, Decide};
        /// let entry: AnnotatableQuestion = Decide::new("Ready?").unwrap().into();
        /// let annotation = Annotate::new(std::collections::BTreeMap::from([("ready".into(), entry)])).unwrap();
        /// let _: AnnotatableQuestion = annotation.into();
        /// ```
        #[non_exhaustive]
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #[serde(tag = "function", rename_all = "lowercase", try_from = "RawAnnotatableQuestion")]
        #[schemars(with = "RawAnnotatableQuestion")]
        pub enum AnnotatableQuestion { $(#[doc = stringify!($member)] $member($member),)+ }

        #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #[serde(tag = "function", rename_all = "lowercase")]
        pub(crate) enum RawAnnotatableQuestion { $($member($member_wire),)+ }
        impl TryFrom<RawAnnotatableQuestion> for AnnotatableQuestion {
            type Error = Error;
            fn try_from(value: RawAnnotatableQuestion) -> Result<Self, Error> {
                match value { $(RawAnnotatableQuestion::$member(value) => value.try_into().map(Self::$member),)+ }
            }
        }
        impl From<AnnotatableQuestion> for RawAnnotatableQuestion {
            fn from(value: AnnotatableQuestion) -> Self {
                match value { $(AnnotatableQuestion::$member(value) => Self::$member(value.into()),)+ }
            }
        }
        impl TryFrom<SimpleQuestion> for AnnotatableQuestion {
            type Error = Error;
            fn try_from(value: SimpleQuestion) -> Result<Self, Error> {
                match value { $(SimpleQuestion::$member(value) => Ok(Self::$member(value)),)+ SimpleQuestion::Rank(_) => Err(question_error(crate::core::diagnostic::requirement("entries", "annotatable questions", "a rank question"))) }
            }
        }
        impl AnnotatableQuestion {
            pub(crate) fn as_question(&self) -> Question { match self { $(Self::$member(question) => Question::$member(question.clone()),)+ } }
            /// The child's explicit evidence selection, or none to inherit annotation evidence.
            #[must_use]
            pub fn evidence_override(&self) -> Option<&Pointer> {
                match self { $(Self::$member(value) => value.evidence.as_ref(),)+ }
            }
        }
        $(impl From<$member> for AnnotatableQuestion { fn from(value: $member) -> Self { Self::$member(value) } })+

        /// Any admitted function with its own required domain and options.
        #[non_exhaustive]
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #[serde(tag = "function", rename_all = "lowercase", try_from = "RawQuestion")]
        #[schemars(with = "RawQuestion")]
        pub enum Question { $(#[doc = stringify!($variant)] $variant($variant),)+ /// Named independent questions.
            Annotate(Annotate) }

        #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #[serde(tag = "function", rename_all = "lowercase")]
        pub(crate) enum RawSimpleQuestion { $($variant($wire),)+ }
        #[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
        #[serde(tag = "function", rename_all = "lowercase")]
        pub(crate) enum RawQuestion { $($variant($wire),)+ Annotate(AnnotateWire) }

        macro_rules! result_contract { () => { crate::core::results::function_set! {all[$($variant,)+ Annotate] entries[$($member),+]} }; }
        pub(crate) use result_contract;

        impl TryFrom<RawQuestion> for Question {
            type Error = Error;
            fn try_from(value: RawQuestion) -> Result<Self, Error> {
                match value { $(RawQuestion::$variant(value) => value.try_into().map(Self::$variant),)+ RawQuestion::Annotate(value) => value.try_into().map(Self::Annotate) }
            }
        }
        impl TryFrom<RawSimpleQuestion> for SimpleQuestion {
            type Error = Error;
            fn try_from(value: RawSimpleQuestion) -> Result<Self, Error> {
                match value { $(RawSimpleQuestion::$variant(value) => value.try_into().map(Self::$variant),)+ }
            }
        }
        impl From<SimpleQuestion> for RawSimpleQuestion {
            fn from(value: SimpleQuestion) -> Self {
                match value { $(SimpleQuestion::$variant(value) => Self::$variant(value.into()),)+ }
            }
        }
        impl TryFrom<Question> for SimpleQuestion {
            type Error=Error;
            fn try_from(value:Question)->Result<Self,Error>{match value{$(Question::$variant(value)=>Ok(Self::$variant(value)),)+Question::Annotate(_)=>Err(question_error(crate::core::diagnostic::requirement("questions", "simple questions", "an annotate question")))}}
        }
        impl From<SimpleQuestion> for Question {
            fn from(value: SimpleQuestion) -> Self { match value { $(SimpleQuestion::$variant(value) => Self::$variant(value),)+ } }
        }
        $(impl From<$variant> for Question { fn from(value: $variant) -> Self { Self::$variant(value) } }
          impl From<$variant> for SimpleQuestion { fn from(value: $variant) -> Self { Self::$variant(value) } })+
        impl Question {
            /// The function this admitted question names.
            #[must_use]
            pub const fn function(&self) -> FunctionName { match self { $(Self::$variant(_) => FunctionName::$variant,)+ Self::Annotate(_) => FunctionName::Annotate } }
            pub(crate) fn without_evidence(&self) -> Self {
                let mut question = self.clone();
                match &mut question { $(Self::$variant(value) => value.evidence = None,)+ Self::Annotate(value) => value.evidence = None }
                question
            }
            #[cfg(feature="native")]
            pub(crate) fn evidence_override(&self)->Option<&Pointer>{match self{$(Self::$variant(value)=>value.evidence.as_ref(),)+Self::Annotate(value)=>value.evidence.as_ref()}}
            /// The admitted evidence selection for this question.
            #[must_use]
            pub fn evidence_pointer(&self) -> &Pointer { match self { $(Self::$variant(value) => value.evidence_pointer(),)+ Self::Annotate(value) => value.evidence_pointer() } }
        }
    };
}
functions! {
    annotatable {
        Decide: DecideWire=1 => ("Judge whether evidence answers a question", command::ThresholdOption::Band, true, None),
        Choose: ChooseWire=2 => ("Choose one answer from ordered choices", command::ThresholdOption::Band, true, Some(("choice", "choices", "THINKTHEN_CHOICES", "Choice; repeat in order"))),
        Tag: TagWire=3 => ("Assign labels to evidence", command::ThresholdOption::Cut, false, Some(("label", "labels", "THINKTHEN_LABELS", "Label; repeat in order"))),
        Score: ScoreWire=4 => ("Score evidence on ordered levels", command::ThresholdOption::None, false, Some(("level", "levels", "THINKTHEN_LEVELS", "Scale level; repeat low to high"))),
        Filter: FilterWire=5 => ("Keep evidence that answers a question", command::ThresholdOption::Band, true, None),
        Find: FindWire=7 => ("Find the evidence unit that answers a question", command::ThresholdOption::Cut, true, None),
        Recognize: RecognizeWire=8 => ("Recognize named kinds in evidence", command::ThresholdOption::Cut, false, Some(("kind", "kinds", "THINKTHEN_KINDS", "Kind; repeat in order"))),
        Relate: RelateWire=9 => ("Find relations between recognized names", command::ThresholdOption::Cut, false, None)
    }
    collection {
        Rank: RankWire=6 => ("Rank evidence on ordered levels", command::ThresholdOption::None, false, Some(("level", "levels", "THINKTHEN_LEVELS", "Scale level; repeat low to high")))
    }
    annotation { ("Answer named questions about each record", command::ThresholdOption::None, false, None) }
}
impl From<Annotate> for Question {
    fn from(value: Annotate) -> Self {
        Self::Annotate(value)
    }
}
impl Question {
    /// Parse and semantically admit a canonical question once.
    /// # Errors
    /// Returns the shared question, pointer, domain or threshold error.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str::<RawQuestion>(json)
            .map_err(|error| {
                question_error(crate::core::diagnostic::context(
                    "question",
                    &error.to_string(),
                ))
            })?
            .try_into()
    }
}

// Omission inherits annotation evidence; an explicit empty pointer selects the root.
fn deserialize_evidence<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    crate::core::diagnostic::evidence(deserializer).map(Some)
}
fn admit_evidence(value: Option<String>) -> Result<Option<Pointer>, Error> {
    value.map(|pointer| pointer.parse()).transpose()
}
