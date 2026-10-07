//! Resolve the command's framing before it starts reading records.

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
