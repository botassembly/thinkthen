//! Saved find follows ordinary file admission and explicit model/field precedence.
use crate::harness::{Canned, Listener, spawn};
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;

const ANSWER: &str = r#"{"model":"actual-model","answers":{"q1":{"type":"choice","probabilities":{"u001":0.2,"u002":0.8}}},"usage":{"input_tokens":887}}"#;

#[test]
fn saved_find_fields_and_model_are_replaced_whole_by_explicit_flags() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("saved-find-overrides");
    fs::create_dir_all(&folder).unwrap();
    let file = folder.join("question.json");
    fs::write(&file, r#"{"find":{"z":"Which?","a":["Read both."]},"on":"/body","model":"saved-model","profile":"saved-profile"}"#).unwrap();
    let argument = format!("@{}", file.display());
    let input = b"{\"id\":1,\"body\":\"first\",\"other\":\"alpha\"}\n{\"id\":2,\"body\":\"second\",\"other\":\"beta\"}\n";
    for (flags, model, evidence) in [
        (vec![], "saved-model", ["first", "second"]),
        (
            vec!["--model", "typed-model", "--field", "/other"],
            "typed-model",
            ["alpha", "beta"],
        ),
    ] {
        let listener = Listener::serving(vec![Canned::ok(ANSWER)]).unwrap();
        let mut arguments = vec!["find", &argument, "--jsonl", "--url", listener.base()];
        arguments.extend(flags);
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "private-find-key")],
            input,
        )
        .unwrap();
        assert_eq!(output.status.code(), Some(0), "{output:?}");
        assert_eq!(
            output.stdout,
            b"{\"id\":2,\"body\":\"second\",\"other\":\"beta\"}\n"
        );
        let requests = listener.requests();
        assert_eq!(requests.len(), 1);
        let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
        assert_eq!(
            body,
            json!({
            "state": format!("[{{\"id\":\"u001\",\"evidence\":\"{}\"}},{{\"id\":\"u002\",\"evidence\":\"{}\"}}]", evidence[0], evidence[1]),
                "model": model,
                "questions": {"q1": {"type":"choice","instructions":{"z":"Which?","a":["Read both."]},"criteria":{"u001":null,"u002":null}}}
            })
        );
        assert!(output.stderr.is_empty());
    }
    let plan = spawn(&["find", &argument, "--jsonl", "--plan"], &[], input).unwrap();
    assert_eq!(plan.status.code(), Some(0));
    let doc: Value =
        serde_json::from_slice(plan.stdout.split(|b| *b == b'\n').next().unwrap()).unwrap();
    assert_eq!(
        doc["from"],
        json!({"question":"file","on":"file","model":"file"})
    );
    fs::remove_dir_all(folder).unwrap();
}

#[test]
fn saved_find_bad_files_are_local_and_never_send_or_repeat_authored_contents() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("saved-find-refusals");
    fs::create_dir_all(&folder).unwrap();
    let file = folder.join("question.json");
    let argument = format!("@{}", file.display());
    for text in [
        r#"{"find":"PRIVATE-FIND-TEXT","threshold":0.8}"#.to_owned(),
        r#"{"find":"PRIVATE-FIND-TEXT","options":["first","second"]}"#.to_owned(),
        "x".repeat(1_048_577),
    ] {
        fs::write(&file, text).unwrap();
        let listener = Listener::serving(Vec::new()).unwrap();
        let output = spawn(
            &["find", &argument, "--url", listener.base()],
            &[("THINKTHEN_API_KEY", "PRIVATE-FIND-KEY")],
            b"first\nsecond\n",
        )
        .unwrap();
        assert_eq!(output.status.code(), Some(5));
        assert!(output.stdout.is_empty());
        let diagnostic = String::from_utf8(output.stderr).unwrap();
        assert!(!diagnostic.contains("PRIVATE-FIND-TEXT"));
        assert!(!diagnostic.contains("PRIVATE-FIND-KEY"));
        assert!(listener.requests().is_empty());
    }
    fs::remove_dir_all(folder).unwrap();
}
