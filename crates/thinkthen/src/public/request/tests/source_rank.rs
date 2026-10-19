//! Original-byte admission must precede selected evidence and leave the refused tail unread.

#[test]
#[ignore = "large-input boundary runs in the release suite"]
fn release_only_source_rank_charges_original_utf8_bytes_before_projection_and_never_reads_the_tail()
{
    use crate::{CallOptions, RecordReading, SourceItem, SourceRecord};
    use std::cell::Cell;
    let envelope = r#"{"public":"x","private":""}"#;
    let padding = crate::core::MAX_RECORD_BYTES - envelope.len();
    let private = "é".repeat(padding / 2) + if padding.is_multiple_of(2) { "" } else { "x" };
    let original = envelope.replacen("\"\"", &format!("\"{private}\""), 1);
    assert_eq!(original.len(), crate::core::MAX_RECORD_BYTES);
    let pulled = Cell::new(0);
    let source = (0..3).map(|at| {
        pulled.set(pulled.get() + 1);
        let record = match at {
            0 => original.clone(),
            1 => "é".to_owned(), // two bytes, and invalid JSON if composed too early
            _ => panic!("source rank read its refused tail"),
        };
        Ok(SourceItem::Text(SourceRecord {
            record,
            file: "source.jsonl".to_owned(),
            first_line: at + 1,
            last_line: at + 1,
        }))
    });
    let mut records = crate::transport::source_records(
        RecordReading::new(&["/public"], None, None).unwrap(),
        source,
        CallOptions::new(),
        false,
        Some(crate::public::SourceBudget::rank()),
    );
    assert!(records.next().unwrap().is_ok());
    assert_eq!(
        records.next().unwrap().unwrap_err().detail().message(),
        "source rank reads at most 16 MiB across all input records"
    );
    assert!(records.next().is_none());
    assert!(records.next().is_none());
    assert_eq!(pulled.get(), 2);
}
