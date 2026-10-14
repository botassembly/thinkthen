//! Typed complete native calls retain dataframe presentation separately.
use crate::public::{Error, RecordInput};
use polars::prelude::Series;

#[allow(
    clippy::type_complexity,
    reason = "native input ownership and original positions remain separate"
)]
pub(super) fn present<T>(
    column: &Series,
    records: Vec<Option<RecordInput<T>>>,
) -> Result<(Vec<RecordInput<T>>, Vec<usize>), Error> {
    if column.len() != records.len() {
        return Err(Error::usage(
            "the typed input column has a different row count",
        ));
    }
    let nulls = column.is_null();
    let mut inputs = Vec::new();
    let mut positions = Vec::new();
    for (at, (missing, record)) in nulls.iter().zip(records).enumerate() {
        if missing != Some(record.is_none()) {
            return Err(Error::usage(
                "the typed input column has different null positions",
            ));
        }
        if let Some(record) = record {
            positions.push(at);
            inputs.push(record);
        }
    }
    Ok((inputs, positions))
}
