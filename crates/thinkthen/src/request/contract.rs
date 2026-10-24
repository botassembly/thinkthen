//! Function-specific shapes are the admission grammar and schema source.

use super::*;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub(super) enum Tagged<T> {
    Inline {
        value: T,
    },
    Toolbox {
        value: BTreeMap<String, Value>,
        name: Option<String>,
    },
    Reference {
        name: String,
    },
}
impl<T: serde::Serialize> Tagged<T> {
    fn into_question(self) -> Result<QuestionInput, Error> {
        Ok(match self {
            Self::Inline { value } => QuestionInput::Inline {
                value: serde_json::to_value(value).map_err(|error| invalid(error.to_string()))?,
            },
            Self::Toolbox { value, name } => QuestionInput::Toolbox { value, name },
            Self::Reference { name } => {
                if name.trim().is_empty() {
                    return Err(invalid("a question reference name is blank"));
                }
                QuestionInput::Reference { name }
            }
        })
    }
}

macro_rules! options {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
        #[serde(deny_unknown_fields)]
        pub(super) struct $name { $( $field: Option<$ty>, )* }
        impl TryFrom<$name> for Options {
            type Error = Error;
            fn try_from(value: $name) -> Result<Self, Error> {
                Ok(Self { $( $field: value.$field, )* ..Self::default() })
            }
        }
    };
}
options!(DecisionOptions {
    on: String,
    threshold: Value
});
options!(ChoiceOptions { on: String, threshold: Value, options: Vec<String> });
options!(TagOptions { on: String, threshold: Value, labels: Vec<String> });
options!(ScaleOptions { on: String, levels: Vec<String> });
#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RankOptions {
    on: Option<String>,
    levels: Option<Vec<String>>,
    #[schemars(with = "Option<std::num::NonZeroUsize>")]
    top: Option<usize>,
}

impl TryFrom<RankOptions> for Options {
    type Error = Error;
    fn try_from(value: RankOptions) -> Result<Self, Error> {
        Ok(Self {
            on: value.on,
            levels: value.levels,
            top: value.top.map(RankLimit::new).transpose()?,
            ..Self::default()
        })
    }
}
options!(AnnotationOptions { on: String });

macro_rules! contract {
    ($( $variant:ident: $question:ty, $options:ty; )*) => {
        #[derive(Debug, Deserialize, schemars::JsonSchema)]
        #[serde(tag = "function", rename_all = "lowercase", deny_unknown_fields)]
        pub(super) enum Contract {
            $( $variant {
                question: Tagged<$question>,
                #[serde(default)] options: $options,
                #[serde(default)] records: Vec<Value>,
            }, )*
        }
        pub(super) fn options(function: FunctionName, json: &str) -> Result<Options, Error> {
            match function {
                $(FunctionName::$variant => serde_json::from_str::<$options>(json).map_err(|error| invalid(error.to_string()))?.try_into(),)*
            }
        }
        impl TryFrom<Contract> for SdkRequest {
            type Error = Error;
            fn try_from(value: Contract) -> Result<Self, Error> {
                match value {
                    $( Contract::$variant { question, options, records } => Ok(Self {
                        function: FunctionName::$variant,
                        question: question.into_question()?,
                        options: options.try_into()?, records,
                        loaded_question: None, loaded_toolbox: None,
                    }), )*
                }
            }
        }
    };
}
contract! {
    Decide: String, DecisionOptions;
    Choose: String, ChoiceOptions;
    Tag: String, TagOptions;
    Score: String, ScaleOptions;
    Filter: String, DecisionOptions;
    Rank: String, RankOptions;
    Find: String, DecisionOptions;
    Annotate: BTreeMap<String, Value>, AnnotationOptions;
    Recognize: Vec<String>, DecisionOptions;
    Relate: Vec<BTreeMap<String, Value>>, DecisionOptions;
}
