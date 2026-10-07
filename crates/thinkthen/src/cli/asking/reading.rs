//! Resolve the command's framing before it starts reading records.

use super::judged::Held;
use crate::schedule::Placed;

use super::{Common, Failure, Framing, Keeping, Reading, Resolved};

/// Read the framing the command line asked for, over the settled pointers.
/// With no flag, `filter` and `rank` read lines, or JSON Lines under a pointer.
pub(super) fn read_by(
    common: &Common,
    settled: &Resolved,
    keeping: Keeping,
) -> Result<Reading, Failure> {
    let on = settled.on().to_vec();
    let asked = common.framing();
    if asked != Framing::Document
        || !keeping.streams_only()
        || common.unit.as_deref() == Some("file")
    {
        return Ok(
            Reading::new(asked, on)?.with_item_schema(settled.metadata().item_schema.clone())
        );
    }
    let framing = if on.is_empty() {
        Framing::Lines
    } else {
        Framing::Jsonl
    };
    Ok(Reading::new(framing, on)?
        .by_default()
        .with_item_schema(settled.metadata().item_schema.clone()))
}

/// Charge original evidence at source admission, leaving the iterator tail unread.
pub(super) fn charge(reading: &Reading, held: &Held, remaining: &mut usize) -> Result<(), Placed> {
    let record_error = |error| Placed::at(Failure::record(error, reading.streams()), held.at);
    let bytes = match &held.arrived {
        Some(bytes) => reading.as_it_arrived(bytes).map_err(record_error)?.len(),
        None => reading
            .evidence(&held.record)
            .map_err(record_error)?
            .as_text()
            .map_err(|error| Placed::at(Failure::from(error), held.at))?
            .len(),
    };
    *remaining = remaining.checked_sub(bytes).ok_or_else(|| {
        Placed::at(
            Failure::Usage("source rank reads at most 16 MiB across all input records"),
            held.at,
        )
    })?;
    Ok(())
}
