//! Canonical decoding and native header behavior at the request boundary.
use super::*;
#[test]
fn canonical_objects_refuse_duplicate_unknown_and_null_controls() {
    let valid = r#"{"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Does it fit?"},"input":{"kind":"text","text":"yes"}}}"#;
    assert!(Request::from_json(valid).unwrap().admit().is_ok());
    for text in [
        valid.replace(
            "\"schema\":\"thinkthen.request/1\"",
            "\"schema\":\"thinkthen.request/2\"",
        ),
        valid.replace(
            "\"function\":\"decide\"",
            "\"function\":\"decide\",\"function\":\"decide\"",
        ),
        valid.replace(
            "\"kind\":\"text\",\"text\":\"yes\"",
            "\"kind\":\"text\",\"text\":\"yes\",\"secret\":true",
        ),
        valid.replace("\"input\":", "\"options\":{\"context\":null},\"input\":"),
        valid.replace(
            "\"input\":",
            "\"options\":{\"context\":\"\",\"context\":\"\"},\"input\":",
        ),
    ] {
        assert!(Request::from_json(&text).is_err(), "{text}");
    }
}
