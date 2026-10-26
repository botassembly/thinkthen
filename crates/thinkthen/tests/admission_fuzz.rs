#![cfg(test)]
//! The routine gate replays the same raw inputs as the coverage-guided fuzzer.
#[path = "../../../fuzz/support/admission.rs"]
mod admission;

#[test]
fn shared_admission_keeps_typed_failures_without_crashes() {
    let corpus =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fuzz/corpus/admission");
    let mut files = std::fs::read_dir(corpus)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    files.sort();
    assert!(!files.is_empty());
    for file in files {
        let input = std::fs::read(&file).unwrap();
        let admitted = admission::exercise(&input);
        use thinkthen::Code;
        let name = file.file_name().unwrap().to_str().unwrap();
        match name {
            "invalid-settings" | "invalid-key" | "number-boundary" => {
                assert!(admitted.settings.unwrap().is_err(), "{name}");
            }
            "duplicate-entries" => assert!(admitted.toolbox.unwrap().is_err(), "{name}"),
            "large-json" | "deep-array" | "deep-object" => {
                assert!(admitted.question.unwrap().is_err(), "{name}");
                assert!(admitted.toolbox.unwrap().is_err(), "{name}");
                assert!(admitted.settings.unwrap().is_err(), "{name}");
            }
            _ => {}
        }
        match name {
            name if name.starts_with("question") => {
                assert_eq!(admitted.question, Some(Ok(())), "{name}");
            }
            "toolbox" => assert_eq!(admitted.toolbox, Some(Ok(()))),
            "settings" => assert_eq!(admitted.settings, Some(Ok(()))),
            "bad-id" | "invalid-utf8" => assert_eq!(admitted.record, Some(Code::RecordInvalid)),
            "null" => assert_eq!(admitted.record, Some(Code::RecordMissing)),
            "large-json" | "deep-array" | "deep-object" => assert!(
                matches!(
                    admitted.record,
                    None | Some(Code::RecordJson | Code::RecordTooLarge)
                ),
                "{name}: {:?}",
                admitted.record
            ),
            "empty" => assert_eq!(admitted.record, Some(Code::RecordJson)),
            _ => assert!(admitted.question.unwrap().is_err(), "{}", file.display()),
        }
    }
}
