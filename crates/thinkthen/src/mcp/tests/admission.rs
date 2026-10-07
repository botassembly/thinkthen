use super::super::{
    admission::{CallParams, Invocation},
    dispatch::PreparedQuestion,
};
use conformance_backend::Backend;
use serde_json::{Value, json};
use std::fs;

fn admit(name: &str, arguments: Value) -> Result<Invocation, crate::Error> {
    let params: CallParams = serde_json::from_value(json!({"name":name,"arguments":arguments}))
        .map_err(|_| crate::Error::usage("invalid tool arguments"))?;
    Invocation::admit(params)
}

#[test]
fn conflicting_sources_questions_credentials_and_unknown_arguments_send_nothing() {
    let backend = Backend::start().expect("loopback");
    for arguments in [
        json!({"question":"q","question_file":"file","evidence":"x"}),
        json!({"question":"q","evidence":"x","records":[]}),
        json!({"question":"q","source":{"paths":["absent"]},"records":[]}),
        json!({"question":"q","evidence":"x","options":{"api_key":"fake-secret"}}),
        json!({"question":"q","evidence":"x","unrecognized":"fake-secret"}),
        json!({"question":"q","source":{"paths":[],"unit":"line"}}),
        json!({"question":"q","source":{"paths":["absent"],"unit":"file","window":2}}),
    ] {
        let error = admit("decide", arguments).unwrap_err();
        assert_eq!(error.kind(), crate::ErrorKind::Usage);
        assert!(!format!("{error:?}").contains("fake-secret"));
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn seven_text_only_tools_refuse_images_before_open_or_send() {
    let backend = Backend::start().expect("loopback");
    for tool in [
        "tag",
        "filter",
        "rank",
        "find",
        "annotate",
        "recognize",
        "relate",
    ] {
        for arguments in [
            json!({"question":{},"images":["does-not-exist.png"]}),
            json!({"question":{},"source":{"paths":["does-not-exist"],"unit":"file","media":"image"}}),
        ] {
            let error = admit(tool, arguments).unwrap_err();
            assert_eq!(error.detail().message(), "this function takes text only");
            assert_eq!(error.kind(), crate::ErrorKind::Usage);
        }
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn inline_at_prefix_is_text_and_explicit_files_use_native_kind_validation() {
    let backend = Backend::start().expect("loopback");
    let inline = admit(
        "decide",
        json!({"question":"@missing-question-file","evidence":"x"}),
    )
    .unwrap();
    assert!(matches!(
        inline.question().unwrap(),
        PreparedQuestion::Atomic(_)
    ));
    let missing = admit(
        "decide",
        json!({"question_file":"does-not-exist","evidence":"x"}),
    )
    .unwrap();
    assert_eq!(
        missing.question().unwrap_err().kind(),
        crate::ErrorKind::Local
    );
    let wrong = admit("score", json!({"question":{"decide":"q"},"evidence":"x"})).unwrap();
    assert_eq!(
        wrong.question().unwrap_err().detail().message(),
        "question kind does not match the named tool"
    );
    assert_eq!(backend.count(), 0);
}

#[test]
fn source_reader_retains_original_locations_order_and_duplicate_occurrences() {
    let backend = Backend::start().expect("loopback");
    let folder = std::env::temp_dir().join(format!("thinkthen-mcp-source-{}", std::process::id()));
    fs::create_dir(&folder).expect("owned scratch");
    fs::write(folder.join("b.txt"), "beta\n").unwrap();
    fs::write(folder.join("a.txt"), "\nalpha\n").unwrap();
    let call = admit(
        "decide",
        json!({"question":"q","source":{"paths":[folder,folder.join("a.txt")],"unit":"line"}}),
    )
    .unwrap();
    let rows: Vec<_> = call
        .source()
        .unwrap()
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(rows.len(), 3);
    for (row, expected) in rows.iter().zip([("alpha", 2), ("beta", 1), ("alpha", 2)]) {
        let crate::SourceItem::Text(text) = row else {
            panic!("text")
        };
        assert_eq!(text.record, expected.0);
        assert_eq!((text.first_line, text.last_line), (expected.1, expected.1));
    }
    assert!(matches!(rows[0], crate::SourceItem::Text(ref row) if row.file.ends_with("a.txt")));
    assert_eq!(backend.count(), 0);
    fs::remove_dir_all(folder).unwrap();
}

#[test]
fn native_controls_validate_deadline_batch_and_admission_without_sends() {
    let backend = Backend::start().expect("loopback");
    for options in [
        json!({"deadline_ms":-2}),
        json!({"batch":0}),
        json!({"batch":"2"}),
        json!({"field":"invalid-pointer"}),
        json!({"top":1}),
    ] {
        assert_eq!(
            admit(
                "decide",
                json!({"question":"q","evidence":"x","options":options})
            )
            .unwrap_err()
            .kind(),
            crate::ErrorKind::Usage
        );
    }
    let call = admit(
        "decide",
        json!({"question":"q","evidence":"x","options":{"deadline_ms":0,"max_requests_total":0}}),
    )
    .unwrap();
    let token = crate::CancelToken::new();
    assert!(call.controls(&token).is_ok());
    assert_eq!(backend.count(), 0);
}

#[test]
fn explicit_null_or_empty_selectors_are_rejected_without_sends() {
    let backend = Backend::start().unwrap();
    for arguments in [
        json!({"question":"q","evidence":null,"images":["absent.png"]}),
        json!({"question":null,"question_file":"absent","evidence":"x"}),
        json!({"question":"q","source":null,"evidence":"x"}),
        json!({"question_file":"","evidence":"x"}),
        json!({"question":"q","source":{"paths":[""]}}),
        json!({"question":"q","images":[""]}),
        json!({"question":"q","evidence":"x","options":{"deadline_ms":null}}),
    ] {
        assert_eq!(
            admit("decide", arguments).unwrap_err().kind(),
            crate::ErrorKind::Usage
        );
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn all_named_tools_reject_credential_arguments_and_withhold_question_debug() {
    let backend = Backend::start().unwrap();
    for tool in [
        "decide",
        "choose",
        "tag",
        "score",
        "filter",
        "rank",
        "find",
        "annotate",
        "recognize",
        "relate",
    ] {
        let arguments = json!({"question":"private-question","evidence":"private-evidence","options":{"api_key":"fake-secret"}});
        let error = admit(tool, arguments).unwrap_err();
        for secret in ["fake-secret", "private-question", "private-evidence"] {
            assert!(!format!("{error:?}").contains(secret));
        }
        let call = admit(
            tool,
            json!({"question":"private-question","evidence":"private-evidence"}),
        )
        .unwrap();
        assert!(!format!("{call:?}").contains("private-question"));
        assert!(!format!("{:?}", call.arguments).contains("private-evidence"));
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn shared_image_readers_keep_attachment_order_and_original_compressed_bytes() {
    let backend = Backend::start().unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/images");
    let paths = [
        root.join("red.png"),
        root.join("blue.png"),
        root.join("red.png"),
    ];
    let originals: Vec<_> = paths
        .iter()
        .map(|path| std::fs::read(path).unwrap())
        .collect();
    let call = admit(
        "decide",
        json!({"question":"q","images":paths,"evidence":"caption"}),
    )
    .unwrap();
    let attachment = call.attachments().unwrap().unwrap();
    assert_eq!(attachment.text(), Some("caption"));
    assert_eq!(attachment.images().len(), originals.len());
    for (image, bytes) in attachment.images().iter().zip(&originals) {
        assert_eq!(image.bytes(), bytes);
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn malformed_or_conflicting_file_inputs_keep_native_error_kinds_without_sends() {
    let backend = Backend::start().unwrap();
    let call = admit(
        "decide",
        json!({"question":"q","source":{"paths":["does-not-exist"]}}),
    )
    .unwrap();
    assert_eq!(call.source().unwrap_err().kind(), crate::ErrorKind::Local);
    let image = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/images/truncated.png");
    let call = admit(
        "score",
        json!({"question":{"score":"q","levels":["low","high"]},"images":[image]}),
    )
    .unwrap();
    assert_eq!(
        call.attachments().unwrap_err().kind(),
        crate::ErrorKind::Usage
    );
    assert_eq!(backend.count(), 0);
}

#[test]
fn inline_records_keep_original_value_types_and_authored_candidate_order() {
    let params: CallParams=serde_json::from_str(r#"{"name":"decide","arguments":{"question":"q","records":[false,null,{"options":{"zebra":"first","alpha":"second"},"text":"x"}]}}"#).unwrap();
    let call = Invocation::admit(params).unwrap();
    let records = call.arguments.records.unwrap();
    assert_eq!(records[0].get(), "false");
    assert_eq!(records[1].get(), "null");
    assert_eq!(
        records[2].get(),
        r#"{"options":{"zebra":"first","alpha":"second"},"text":"x"}"#
    );
}
