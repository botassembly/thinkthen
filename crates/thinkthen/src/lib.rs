//! Ask typed questions and retain one typed row for every input record.
//!
//! ```no_run
//! use thinkthen::{Decide, Record, Settings};
//! let engine = Settings::default().engine(|name| std::env::var(name).ok())?;
//! let question = Decide::new("Does the customer ask for money back?")?;
//! let row = engine.run(&question, Record::text("Please refund my order."))?;
//! # Ok::<(), thinkthen::Error>(())
//! ```
#[cfg(all(feature = "fixture-routing", not(debug_assertions)))]
compile_error!("fixture routing cannot enter a production release build");
/// Process-owned loopback routes, available only in explicit test builds.
#[cfg(all(feature = "fixture-routing", debug_assertions))]
pub mod test_fixture;
extern crate self as thinkthen;
/// The engine package version, shared with its command and bindings.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
mod backend;
mod batch;
mod contract_driver;
mod contract_request;
mod core;
mod engine;
mod retry;
mod settings;
mod transport;
pub(crate) use backend::BackendTable;
pub use backend::Key;
pub use batch::Batch;
#[cfg(feature = "native")]
#[doc(hidden)]
pub use contract_driver::native;
pub use contract_driver::{AnyCall, AnyRows, Call, Filtered, RecordSender, Rows};
pub use contract_request::Request;
pub use core::{
    ABI_VERSION, Code, Cut, Error, Family, OffsetUnit, Pointer, Probability, Threshold,
};
pub(crate) use core::{ErrorKind, Limits};
#[cfg(feature = "command")]
pub(crate) use core::{TOOLBOX_LIMIT, escape_diagnostic};
pub use core::{questions, records, results};
pub use engine::Engine;
pub use questions::{
    AnnotatableQuestion, Annotate, Choose, Decide, Filter, Find, FunctionName, Question, Rank,
    Recognize, Relate, RelationRule, Score, SimpleQuestion, Tag, Toolbox,
};
pub use records::{Record, RecordId};
pub use results::*;
pub use retry::Cancel;
pub(crate) use retry::RetryPolicy;
pub use settings::BackendName as Backend;
pub use settings::{BackendName, Retry, Settings};
#[cfg(feature = "command")]
#[path = "cli/main.rs"]
mod command;
/// Table input and output for the command.
#[cfg(feature = "command")]
pub mod files;
/// Saved entries and local command pipelines.
#[cfg(feature = "command")]
pub mod runs;
/// Entrypoint for the crate's executable, compiled only with command support.
#[cfg(feature = "command")]
#[doc(hidden)]
#[must_use]
pub fn command_main() -> std::process::ExitCode {
    command::main()
}
