//! The pure core of ThinkThen. It touches no file, environment, network, process or clock.

pub(crate) mod abi;
mod adapter;
pub use abi::ABI_VERSION;
mod backend_name;
pub use backend_name::BackendName;
pub(crate) mod ask;
mod code;
pub(crate) mod diagnostic;
mod error;
pub mod questions;
pub mod records;
pub mod results;
pub use code::Code;
mod function;
mod pack;
mod probability;
pub(crate) mod record;
pub(crate) mod spans;
pub use spans::OffsetUnit;
mod row;
mod threshold;

pub(crate) use adapter::Adapter;
pub(crate) use ask::{Ask, AskAnswer, ChoiceOption};
pub(crate) use error::ErrorKind;
pub use error::{Error, Family};
#[cfg(feature = "command")]
pub(crate) use error::{escape_diagnostic, local};
pub(crate) use function::annotate::Annotate;
pub(crate) use function::choose::Choose;
pub(crate) use function::decide::Decide;
pub(crate) use function::find::{Find, Found};
pub(crate) use function::rank::RankLimit;
pub(crate) use function::recognize::{Name, Recognize, RecognizeDetail};
pub(crate) use function::score::Score;
pub(crate) use function::tag::Tag;
pub(crate) use function::{Function, FunctionAnswers};
pub(crate) use pack::{Chunk, NextRound, Packer, Request, RequestAnswers};
pub use probability::Probability;
pub use record::Pointer;
pub(crate) use record::{AdmittedRecord, Record};
pub(crate) use row::{Answer, Detail, Outcome, Row};
pub(crate) use threshold::Verdict;
pub use threshold::{Cut, Threshold};

mod toolbox;
#[cfg(feature = "command")]
pub(crate) use questions::TOOLBOX_LIMIT;
pub(crate) use toolbox::{Toolbox, ToolboxFunction, ToolboxQuestion};

pub(crate) use function::relate::{Relate, Relation, RelationName, RelationRule};

mod limits;
pub(crate) use limits::Limits;
