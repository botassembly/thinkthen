//! Ask typed questions about text and branch on the answer.
//!
//! Build a question, run it on a record, and take the answer with
//! [`Row::into_answer`]. That one step refuses "not sure" and a failed record,
//! so neither can quietly become "no".
//!
//! # Setup
//!
//! [`Engine::from_env`] reads `THINKTHEN_BACKEND`, the backend's key variable
//! (such as `THINKTHEN_API_KEY`) and `THINKTHEN_BATCH`, as the command does.
//! [`Settings`] builds an engine from explicit values instead.
//!
//! ```no_run
//! use thinkthen::{Decide, Engine};
//!
//! let engine = Engine::from_env()?;
//! let refund = Decide::new("Does the customer ask for a refund?")?;
//! let is_refund: bool = engine.run(&refund, "Please refund my order.")?.into_answer()?;
//! if is_refund {
//!     println!("send it to the refund queue");
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! In the examples below, `engine` is an [`Engine`] made this way.
//!
//! # One answer per function
//!
//! ```
//! # use thinkthen::{BackendName, Settings};
//! use thinkthen::{Choose, Decide, Find, Recognize, Relate, RelationRule, Score, Tag};
//! # let engine = Settings::default().with_backend(BackendName::Stub).engine(|_| None)?;
//!
//! let refund = Decide::new("Does the customer ask for a refund?")?;
//! let is_refund: bool = engine.run(&refund, "Please refund my order.")?.into_answer()?;
//!
//! let team = Choose::new("Which team owns this?", ["billing", "shipping", "other"])?;
//! let owner: String = engine.run(&team, "My parcel went to the wrong address.")?.into_answer()?;
//! assert!(["billing", "shipping", "other"].contains(&owner.as_str()));
//!
//! let topics = Tag::new("Which topics does it raise?", ["billing", "shipping"])?;
//! let raised: Vec<String> = engine.run(&topics, "Charged twice, still no parcel.")?.into_answer()?;
//!
//! let urgency = Score::new("How urgent is it?", ["low", "medium", "high"])?;
//! let how_urgent: f64 = engine.run(&urgency, "The site is down for everyone.")?.into_answer()?;
//! assert!((0.0..=1.0).contains(&how_urgent));
//!
//! // Spans are UTF-8 byte offsets, so they slice the text directly.
//! let policy = "Café orders ship daily.\nReturns close after 30 days.";
//! let deadline = Find::new("Which line gives the return deadline?")?;
//! if let Some(line) = engine.run(&deadline, policy)?.into_answer()? {
//!     assert_eq!(&policy[line.start()..line.end()], line.text());
//! }
//!
//! let people = Recognize::new(["person", "place"])?;
//! let names = engine.run(&people, "Ada Lovelace lived in London.")?.into_answer()?;
//! for name in &names {
//!     println!("{}: {}", name.kind(), name.text());
//! }
//!
//! let lives_in = Relate::new([RelationRule::new("lives_in", "person", "place")?])?;
//! let relations = engine.run(&lives_in, "Ada Lovelace lived in London.")?.into_answer()?;
//! # let _ = (is_refund, raised, relations);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Not sure
//!
//! A band threshold leaves a middle where the answer is not sure. `into_answer`
//! returns [`NoAnswer::Unsure`] there, and [`NoAnswer::Failed`] for a failed
//! record. [`Status`] and [`Outcome`] are closed, so a `match`
//! over them needs no catch-all arm.
//!
//! ```
//! # use thinkthen::{BackendName, Settings};
//! use thinkthen::{Decide, NoAnswer, Threshold};
//! # let engine = Settings::default().with_backend(BackendName::Stub).engine(|_| None)?;
//! let refund = Decide::new("Does the customer ask for a refund?")?
//!     .threshold(Threshold::band(0.2, 0.8)?);
//! let next_step = match engine.run(&refund, "I want to send this back.")?.into_answer() {
//!     Ok(true) => "refund queue",
//!     Ok(false) => "no action",
//!     Err(NoAnswer::Unsure { .. }) => "a person reviews it",
//!     Err(NoAnswer::Failed(error)) => return Err(error.into()),
//! };
//! # let _ = next_step;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Many records
//!
//! [`Engine::rows`] takes any owned iterator of records and yields one row per
//! record, in input order; a failed record is a row, and the call goes on.
//! [`Engine::filter`] yields only the kept items, and
//! [`Engine::filter_items`] yields every item with its row. [`Engine::rank`]
//! yields each item with its row, best first.
//!
//! ```
//! # use thinkthen::{BackendName, Settings};
//! use thinkthen::{AnnotatableQuestion, Annotate, Choose, Decide, Rank};
//! # let engine = Settings::default().with_backend(BackendName::Stub).engine(|_| None)?;
//! let refund = Decide::new("Does the customer ask for a refund?")?;
//! let tickets = vec!["Refund me, please.", "Thanks for the quick help!"];
//! let mut refund_requests = Vec::new();
//! for row in engine.rows(&refund, tickets)? {
//!     let is_refund: bool = row?.into_answer()?;
//!     refund_requests.push(is_refund);
//! }
//! assert_eq!(refund_requests.len(), 2);
//!
//! let severity = Rank::new("How severe is the outage?", ["minor", "major", "total"])?;
//! let outages = ["One page is slow.", "Nothing loads."];
//! for ranked in engine.rank(&severity, outages, |outage| *outage)? {
//!     let (outage, row) = ranked?;
//!     println!("{} {outage}", row.answer()?);
//! }
//!
//! let team = Choose::new("Which team owns this?", ["billing", "shipping"])?;
//! let form = Annotate::new([
//!     ("refund", AnnotatableQuestion::from(refund)),
//!     ("team", AnnotatableQuestion::from(team)),
//! ])?;
//! let triage = engine.run(&form, "Refund my lost parcel.")?;
//! println!("{triage}");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Async
//!
//! With the `stream` feature, `Rows::into_stream` turns rows into a
//! `futures_core::Stream` that is only a stream, so `StreamExt` helpers apply
//! without clashing with `Iterator`. No async runtime comes with the crate.
//!
//! # Features
//!
//! - `command` (default) builds the `thinkthen` command. Library users turn it
//!   off with `default-features = false`.
//! - `stream` adds async streams.
#![cfg_attr(docsrs, feature(doc_cfg))]
extern crate self as thinkthen;
/// The engine package version, shared with its command and bindings.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
mod backend;
mod batch;
mod contract_driver;
mod contract_request;
mod core;
mod descriptions;
mod engine;
mod retry;
mod settings;
mod traits;
mod transport;
pub(crate) use backend::BackendTable;
pub use backend::Key;
pub use batch::Batch;
#[cfg(feature = "native")]
#[doc(hidden)]
pub use contract_driver::native;
pub use contract_driver::{
    AnyCall, AnyRows, Call, FilterError, Filtered, Kept, Ranked, RecordSender, Rows,
};
#[cfg(feature = "stream")]
pub use contract_driver::{AnyRowStream, RowStream};
pub use contract_request::Request;
#[doc(hidden)]
pub use core::{ABI_VERSION, OffsetUnit};
pub use core::{Code, Cut, Error, Family, Pointer, Probability, Threshold};
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
pub use settings::{BackendName, Retry, Settings};
#[cfg(feature = "command")]
#[path = "cli/main.rs"]
mod command;
#[cfg(feature = "command")]
pub(crate) mod files;
#[cfg(feature = "command")]
pub(crate) mod runs;
/// Entrypoint for the crate's executable, compiled only with command support.
#[cfg(feature = "command")]
#[doc(hidden)]
#[must_use]
pub fn command_main() -> std::process::ExitCode {
    command::main()
}
