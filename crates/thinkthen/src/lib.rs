//! Ask typed questions about evidence and branch on the answer.
//!
//! ```no_run
//! use thinkthen::{Backend, Decide, Engine, Record};
//!
//! let backend = Backend::resolve(None, None, |name| std::env::var(name).ok())?;
//! let question = Decide::new("Does the customer ask for money back?")?;
//! let records = ["Please refund my order.", "Where is my parcel?"].map(Record::text);
//! let mut rows = Vec::new();
//! Engine::new(backend).call(&question, records, |row| rows.push(row))?;
//! # Ok::<(), thinkthen::Error>(())
//! ```

mod client;
pub use client::Client;

mod backend;
mod batch;
mod engine;
mod rank;
mod retry;
mod transport;

pub use backend::{Backend, BackendEntry, BackendTable, Key};
pub use batch::Batch;
pub use engine::Engine;
pub use retry::{Cancel, RetryPolicy};
pub use thinkthen_core::{
    Adapter, AdmittedRecord, Annotate, Answer, Ask, AskAnswer, ChoiceOption, Choose, Decide,
    Detail, Error, ErrorKind, Family, Filter, Find, Found, Function, FunctionAnswers, KindDetail,
    Limits, Name, Outcome, PieceDetail, Pointer, Probability, Rank, RankLimit, Ranking, Recognize,
    RecognizeDetail, Record, Row, Score, Source, TOOLBOX_LIMIT, Tag, Threshold, Toolbox,
    ToolboxFunction, ToolboxQuestion, escape_diagnostic, passes,
};

mod toolbox;
pub use toolbox::read_toolbox;

pub use thinkthen_core::{Relate, Relation, RelationName, RelationRule};

mod request;
pub use request::{QuestionDefinition, SdkCall, SdkRequest, SdkSettings};

mod pull;
pub use pull::{OffsetUnit, PullCall, PullEvent, PullRow, utf16_text};
