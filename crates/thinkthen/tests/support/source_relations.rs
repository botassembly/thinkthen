//! Shared source-record fixtures retain physical identity and fields.
use thinkthen::{RawRecord, RecordEvidence, RecordInput, RecordReading, Relate, SourceLocation};
#[cfg(test)]
pub(crate) fn ask() -> Relate {
    Relate::from_records_json(r#"{"version":1,"relate":{"relations":[{"name":"follows","source":"person","target":"person","reads":"follows"}]}}"#).unwrap()
}
#[cfg(test)]
pub(crate) fn item(
    name: &str,
    ordinal: usize,
    file: &str,
    extra: &str,
) -> RecordInput<RecordEvidence> {
    let raw = RawRecord::json(&format!(
        r#"{{"name":"{name}","kind":"person","private":{extra}}}"#
    ))
    .unwrap();
    let mut input = RecordReading::new(&[], None, None)
        .unwrap()
        .compose(raw)
        .unwrap();
    input.original = input
        .original
        .with_location(SourceLocation::new(file.into(), Some(ordinal), Some(ordinal)).unwrap());
    input
}
