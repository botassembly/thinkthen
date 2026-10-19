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
