//! Offline callers inspect the original native representation without reparsing.
use thinkthen::RawRecord;

#[test]
fn borrowed_inspection_preserves_authored_order_false_null_and_exact_numbers() {
    let original = RawRecord::json(r#"{"z":false,"a":null,"a/b":[42,0.5,"x"]}"#).unwrap();
    let content = original.content().unwrap();
    let names = content.fields().unwrap().map(|(name, _)| name).collect::<Vec<_>>();
    assert_eq!(names, ["z", "a", "a/b"]);
    assert_eq!(content.member("z").unwrap().boolean(), Some(false));
    assert!(content.member("a").unwrap().is_null());
    assert!(content.member("absent").is_none());
    assert_eq!(content.pointer("/a~1b/0").unwrap().unwrap().number().unwrap().as_u64(), Some(42));
    let values = content.member("a/b").unwrap().elements().unwrap().collect::<Vec<_>>();
    assert_eq!(values.len(), 3);
    assert_eq!(values[1].number().unwrap().as_f64(), Some(0.5));
    assert_eq!(values[2].text(), Some("x"));
    assert!(content.pointer("/z/anything").unwrap().is_none());
    assert!(content.pointer("$.z").is_err());
    assert!(content.pointer("/a~1b/01").unwrap().is_none());
}

#[test]
fn read_only_json_preserves_shapes_and_shared_borrowed_content_without_execution_admission() {
    use thinkthen::{ParsedJson, RawRecord};
    for text in ["null", "false", "0", "1.25", r#""evidence""#, "[]", "{}", r#"{"z":false,"a":null}"#] {
        let value = ParsedJson::from_json(text).unwrap();
        assert_eq!(value.content().to_json().unwrap(), text);
        let clone = value.clone();
        drop(value);
        assert_eq!(clone.content().to_json().unwrap(), text);
    }
    let marker = "private_read_only_evidence_0639";
    let value = ParsedJson::from_json(&format!(r#"{{"evidence":"{marker}"}}"#)).unwrap();
    assert!(!format!("{value:?}").contains(marker));
    let clone = value.clone();
    let original = value.content().fields().unwrap().next().unwrap().1;
    let shared = clone.content().fields().unwrap().next().unwrap().1;
    assert!(std::ptr::eq(original.text().unwrap().as_ptr(), shared.text().unwrap().as_ptr()));
    let oversized = format!(r#"{{"evidence":"{}"}}"#, "x".repeat(RawRecord::max_bytes()));
    let value = ParsedJson::from_json(&oversized).unwrap();
    assert_eq!(value.content().to_json().unwrap(), oversized);
    assert!(RawRecord::json(&oversized).is_err());
}
