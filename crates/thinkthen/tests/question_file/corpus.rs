//! The structural corpus the published schema and the parser both agree on.

#![allow(
    clippy::expect_used,
    reason = "a malformed corpus entry should stop the boundary test"
)]

use std::fs;
use std::path::Path;

use crate::harness::{run, written};

#[test]
fn the_schema_and_the_parser_agree_on_the_shared_corpus() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../specification/fixtures/question-file/corpus.json");
    let corpus: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).expect("the shared corpus file reads"))
            .expect("the shared corpus is JSON");
    let cases = corpus["cases"].as_array().expect("the corpus holds cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("a case name");
        let verb = case["verb"].as_str().expect("a case verb");
        let file = serde_json::to_string(&case["file"]).expect("a case file is JSON");
        let written = written(&format!("corpus-{name}"), &file);
        let has_batch = case["file"].get("batch").is_some();
        let input: &[u8] = if verb == "relate" {
            br#"[{"name":"Ada","kind":"person","label":"Ada","type":"person"},{"name":"Acme","kind":"organization","label":"Acme","type":"organization"}]"#
        } else if has_batch {
            b"Refund me please.\nAnother message.\n"
        } else {
            b"Refund me please."
        };
        let mut arguments = vec![verb, &written, "--dry-run"];
        if has_batch {
            // A file batch is ignored on one document; a stream exercises its value.
            arguments.push("--lines");
        }
        let output = run(&arguments, input).expect("the compiled binary runs");
        if case["valid"].as_bool().expect("a case verdict") {
            assert_eq!(
                output.status.code(),
                Some(0),
                "{name}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        } else {
            assert_eq!(
                output.status.code(),
                Some(5),
                "{name}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
