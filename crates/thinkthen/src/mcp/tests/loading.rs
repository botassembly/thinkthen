//! Native grammar/loading boundaries exercised before any engine sends.
use super::super::admission::{CallParams, Invocation};
use conformance_backend::Backend;
use serde_json::json;

fn inline(tool: &str, question: &str) -> Result<Invocation, crate::Error> {
    let text =
        format!(r#"{{"name":"{tool}","arguments":{{"question":{question},"evidence":"x"}}}}"#);
    Invocation::admit(serde_json::from_str::<CallParams>(&text).unwrap())
}

#[test]
fn native_closed_grammars_refuse_duplicate_unknown_and_wrong_kind_without_sends() {
    let backend = Backend::start().unwrap();
    for (tool, question) in [
        ("decide", r#"{"decide":"q","decide":"another"}"#),
        (
            "choose",
            r#"{"choose":"q","options":{"z":"one","z":"two"}}"#,
        ),
        (
            "score",
            r#"{"score":"q","levels":["low","high"],"unknown":true}"#,
        ),
        (
            "annotate",
            r#"{"version":1,"questions":{"a":{"decide":"q"},"a":{"decide":"other"}}}"#,
        ),
        (
            "rank",
            r#"{"version":1,"questions":{"a":{"decide":"q","threshold":0.5}}}"#,
        ),
        (
            "rank",
            r#"{"version":1,"questions":{"a":{"decide":"q","on":""}}}"#,
        ),
        (
            "recognize",
            r#"{"version":1,"recognize":{"kinds":{"person":null}},"extra":true}"#,
        ),
        (
            "relate",
            r#"{"version":1,"relate":{"relations":[{"name":"same","source":"*","target":"*","extra":true}]}}"#,
        ),
        ("tag", r#"{"decide":"q"}"#),
    ] {
        let failure = inline(tool, question).unwrap_err();
        assert_eq!(failure.kind(), crate::ErrorKind::Usage, "{tool}");
    }
    assert_eq!(backend.count(), 0);
}

#[test]
fn explicit_file_loading_keeps_local_errors_and_never_creates_or_rewrites_files() {
    let backend = Backend::start().unwrap();
    let folder = std::env::temp_dir().join(format!("thinkthen-mcp-loading-{}", std::process::id()));
    std::fs::create_dir(&folder).unwrap();
    let path = folder.join("question.json");
    for (tool, bytes) in [
        ("decide", b"{\"decide\":\"   \"}".as_slice()),
        ("annotate", b"{\"version\":2,\"questions\":{}}".as_slice()),
        ("recognize", b"\xff".as_slice()),
        ("relate", b"not JSON".as_slice()),
        ("rank", b"{\"version\":1,\"questions\":{}}".as_slice()),
    ] {
        std::fs::write(&path, bytes).unwrap();
        let params: CallParams = serde_json::from_value(
            json!({"name":tool,"arguments":{"question_file":path,"evidence":"x"}}),
        )
        .unwrap();
        let failure = Invocation::admit(params)
            .unwrap()
            .request()
            .unwrap()
            .0
            .resolve_question()
            .unwrap_err();
        assert_eq!(failure.kind(), crate::ErrorKind::Local, "{tool}");
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
    }
    let absent = folder.join("absent.json");
    let params: CallParams = serde_json::from_value(
        json!({"name":"decide","arguments":{"question_file":absent,"evidence":"x"}}),
    )
    .unwrap();
    let failure = Invocation::admit(params)
        .unwrap()
        .request()
        .unwrap()
        .0
        .resolve_question()
        .unwrap_err();
    assert_eq!(
        failure.detail().message(),
        "the question file could not be read"
    );
    assert!(!absent.exists());
    std::fs::write(&path, vec![b'x'; 1_048_577]).unwrap();
    let params: CallParams = serde_json::from_value(
        json!({"name":"decide","arguments":{"question_file":path,"evidence":"x"}}),
    )
    .unwrap();
    let failure = Invocation::admit(params)
        .unwrap()
        .request()
        .unwrap()
        .0
        .resolve_question()
        .unwrap_err();
    assert_eq!(failure.kind(), crate::ErrorKind::Local);
    assert_eq!(failure.detail().message(), "the question file is too large");
    assert_eq!(backend.count(), 0);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn native_question_set_preparation_keeps_authored_names_and_utf8_order() {
    let call = inline(
        "annotate",
        r#"{"version":1,"questions":{"zebra":{"decide":"café 😀?"},"alpha":{"score":"grade","levels":["low","high"]}}}"#,
    );
    let crate::RequestDefinition::Annotate(set) = call
        .unwrap()
        .request()
        .unwrap()
        .0
        .resolve_question()
        .unwrap()
    else {
        panic!("annotate set")
    };
    assert_eq!(
        set.members().map(|(name, _)| name).collect::<Vec<_>>(),
        ["zebra", "alpha"]
    );
}
