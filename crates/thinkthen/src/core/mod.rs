//! The pure core of ThinkThen. It touches no file, environment, network, process or clock.

mod adapter;
mod ask;
mod error;
mod function;
mod pack;
mod probability;
mod record;
mod row;
mod threshold;

pub use adapter::Adapter;
pub use ask::{Ask, AskAnswer, ChoiceOption};
pub use error::{Error, ErrorKind, Family, escape_diagnostic};
pub use function::annotate::Annotate;
pub use function::choose::Choose;
pub use function::decide::Decide;
pub use function::filter::{Filter, passes};
pub use function::find::{Find, Found};
pub use function::rank::{Rank, RankLimit, Ranking};
pub use function::recognize::{KindDetail, Name, PieceDetail, Recognize, RecognizeDetail};
pub use function::score::Score;
pub use function::tag::Tag;
pub use function::{Function, FunctionAnswers};
pub use pack::{Chunk, NextRound, Packer, Request, RequestAnswers};
pub use probability::Probability;
pub use record::{AdmittedRecord, Pointer, Record};
pub use row::{Answer, Detail, Outcome, Row, Source};
pub use threshold::{Threshold, Verdict};

mod toolbox;
pub use toolbox::{TOOLBOX_LIMIT, Toolbox, ToolboxFunction, ToolboxQuestion};

pub use function::relate::{Relate, Relation, RelationName, RelationRule};

mod limits;
pub use limits::Limits;
