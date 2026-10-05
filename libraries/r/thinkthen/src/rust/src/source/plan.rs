//! Bounded planning intake shared by the C and Python source doors.

use thinkthen::{CallOptions, DetailQuestion, Engine, Error, SourceRecord};

/// Preview source evidence with bounded cumulative intake before retaining request bodies.
pub(crate) fn estimate<Q: DetailQuestion + ?Sized>(
    engine: &Engine,
    asked: &Q,
    records: impl IntoIterator<Item = Result<SourceRecord<String>, Error>>,
    options: CallOptions<'_>,
) -> Result<thinkthen::PlanEstimate, Error> {
    let mut bytes = 0usize;
    let records = records.into_iter().map(|record| {
        let record = record?;
        bytes = bytes
            .checked_add(record.record.len())
            .filter(|&bytes| bytes <= 16 * 1024 * 1024)
            .ok_or_else(|| {
                Error::new(
                    thinkthen::ErrorKind::Usage,
                    "source plan input exceeds 16 MiB",
                )
            })?;
        Ok(record)
    });
    engine.try_plan_with(asked, records, options)
}
