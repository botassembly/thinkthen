//! Bridge-only admission and polling over the ordinary shared driver.
//!
//! Native hosts use one driver. NEEDS_INPUT reserves one queue slot before the
//! host supplies its next record. Row publication and input capacity share a
//! wake source, so neither side requires a second execution producer or polling.
mod builders;
mod recording;
mod sql;
use super::*;
use crate::{ABI_VERSION, Code, questions};
pub use builders::{QuestionBuilder, ToolboxBuilder};
pub use recording::{recording_evidence, validate_recording_answer};
pub use sql::{
    sql_engine, sql_engine_unrestricted, sql_postgres_id_error, sql_postgres_record,
    sql_postgres_row, sql_question, sql_request, sql_threshold,
};
use std::time::Duration;
/// Native driver event; record failures remain ordinary rows.
#[non_exhaustive]
#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    reason = "the 208-byte row crosses a bounded queue without a per-row allocation"
)]
pub enum Event {
    /// An independently owned canonical row.
    Row(AnyRow),
    /// An input slot is reserved until send or end_input.
    NeedsInput,
    /// Completion, including after record failures.
    Finished,
}
impl From<super::driver::Event> for Event {
    fn from(event: super::driver::Event) -> Self {
        match event {
            super::driver::Event::Row(row) => Self::Row(row),
            super::driver::Event::NeedsInput => Self::NeedsInput,
            super::driver::Event::Finished => Self::Finished,
        }
    }
}
/// Native ownership delegates to the same private driver used by command hosts.
#[non_exhaustive]
#[derive(Debug)]
pub struct Call(super::driver::Driver);
impl Call {
    /// Admit and open a canonical request.
    /// # Errors
    /// Returns shared setup or question errors.
    pub fn open(engine: &Engine, request: &Request, offsets: OffsetUnit) -> Result<Self, Error> {
        super::driver::Driver::open(engine, request, offsets).map(Self)
    }
    /// Open a finite collection of already admitted records on the same producer.
    /// The request must have no JSON initial records. Input is closed at creation,
    /// so temporary host scheduling gaps cannot split this collection's batches.
    /// # Errors
    /// Returns usage.abi for competing collections, or a shared setup error.
    pub fn open_records(
        engine: &Engine,
        request: &Request,
        records: Vec<Record>,
        offsets: OffsetUnit,
    ) -> Result<Self, Error> {
        super::driver::Driver::open_records(engine, request, records, offsets).map(Self)
    }
    /// Transfer one record into a reserved input slot.
    /// # Errors
    /// Returns cancellation or usage.abi for a protocol error.
    pub fn send(&mut self, record: Record) -> Result<(), Error> {
        self.0.send(record)
    }
    /// Close input without waiting for output.
    pub fn end_input(&mut self) {
        self.0.end_input();
    }
    /// Cancel the shared worker.
    pub fn cancel(&self) {
        self.0.cancel();
    }
    /// Retain the independently shared concurrent cancellation token.
    #[must_use]
    pub fn cancellation(&self) -> Cancel {
        self.0.cancellation()
    }
    /// Wait for one event.
    /// # Errors
    /// Returns a call error once, then completion.
    pub fn next_event(&mut self) -> Result<Event, Error> {
        self.0.next_event().map(Into::into)
    }
    /// Wait at most timeout for one event.
    /// # Errors
    /// Returns a call error once, then completion.
    pub fn poll(&mut self, timeout: Duration) -> Result<Option<Event>, Error> {
        self.0.poll(timeout).map(|event| event.map(Into::into))
    }
}
/// Reject incompatible native versions before any row or client operation.
/// # Errors
/// Requires equal majors and an engine minor at least the required minor.
pub fn check_version(required: u32, actual: u32) -> Result<(), Error> {
    if required >> 16 != actual >> 16 || (actual & 0xffff) < (required & 0xffff) {
        Err(abi_error())
    } else {
        Ok(())
    }
}
/// The engine's native contract requirement, independent of package version.
#[must_use]
pub const fn version() -> u32 {
    ABI_VERSION
}
/// Construct the shared error for an invalid native layout or protocol.
#[must_use]
pub fn abi_error() -> Error {
    super::abi_error()
}
/// Map a caught native-boundary panic without unwinding through a host.
#[must_use]
pub fn internal_error() -> Error {
    Error::from_contract(Code::InternalBug, "the native operation could not complete")
}

/// Admit an explicit native settings URL without resolving any credentials.
/// # Errors
/// Returns the shared URL policy error.
pub fn validate_url(text: &str) -> Result<(), Error> {
    crate::backend::validate_url(text).map(|_| ())
}
/// Select a canonical named snapshot without constructing a host error.
/// # Errors
/// Returns usage.name for an absent or ambiguous selection.
pub fn select_toolbox(toolbox: &questions::Toolbox, name: Option<&str>) -> Result<Question, Error> {
    let selected = if let Some(name) = name {
        toolbox.get(name)
    } else {
        let mut entries = toolbox.iter();
        let first = entries.next();
        if entries.next().is_none() {
            first.map(|(_, question)| question)
        } else {
            None
        }
    };
    selected.cloned().map(Into::into).ok_or_else(|| {
        Error::from_contract(
            Code::UsageName,
            crate::core::diagnostic::requirement(
                "name",
                "an existing named question",
                "no selected question",
            ),
        )
    })
}
/// Report host settings that cannot enter the canonical representation.
#[must_use]
pub fn settings_conversion_error() -> Error {
    Error::from_contract(
        Code::UsageSettings,
        crate::core::diagnostic::requirement(
            "settings",
            "a JSON-representable settings object",
            "an incompatible host value (contents redacted)",
        ),
    )
}
/// Report a host question that cannot enter the canonical representation.
#[must_use]
pub fn question_conversion_error() -> Error {
    Error::from_contract(
        Code::UsageQuestion,
        crate::core::diagnostic::requirement(
            "question",
            "a JSON-representable question object",
            "an incompatible host value (contents redacted)",
        ),
    )
}
/// Report a host value that could not enter the canonical record representation.
#[must_use]
pub fn record_conversion_error() -> Error {
    Error::from_contract(
        Code::RecordInvalid,
        "the host value cannot be represented as a record",
    )
}
/// Return the engine's own cancellation result after a host releases its call.
#[must_use]
pub fn cancelled_error() -> Error {
    crate::retry::cancelled()
}

/// Report a call attempted after a host released its client owner.
#[must_use]
pub fn closed_client_error() -> Error {
    Error::from_contract(
        Code::UsageAbi,
        crate::core::diagnostic::requirement("client", "an open client", "a closed client"),
    )
}
