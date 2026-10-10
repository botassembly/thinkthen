//! Canonical path framing shares authorized handle decoding and source locations.
use super::*;
use std::io::Cursor;

#[test]
fn authorized_source_framing_preserves_originals_and_physical_lines() {
    for (framing, input, first, last, expected) in [
        (RequestFraming::Document, "a\nb\n", 1, 2, json!("a\nb\n")),
        (RequestFraming::Lines, "\n  a  \r\n", 2, 2, json!("  a  ")),
        (
            RequestFraming::Jsonl,
            "\n{\"ok\":false,\"value\":null}\n",
            2,
            2,
            json!({"ok":false,"value":null}),
        ),
        (
            RequestFraming::Csv,
            "id,body\r\n1,\"a\nb\"\r\n",
            2,
            3,
            json!({"id":"1","body":"a\nb"}),
        ),
        (
            RequestFraming::Tsv,
            "id\tbody\n1\ttext\n",
            2,
            2,
            json!({"id":"1","body":"text"}),
        ),
    ] {
        let reading = ReaderOptions {
            unit: if matches!(framing, RequestFraming::Document) {
                SourceUnit::File
            } else {
                SourceUnit::Line
            },
            window: None,
        };
        let mut reader =
            RequestSourceReader::new("authorized", Cursor::new(input), framing, reading).unwrap();
        let row = reader.next().unwrap().unwrap();
        assert_eq!(
            (row.file.as_str(), row.first_line, row.last_line),
            ("authorized", first, last)
        );
        assert_eq!(serde_json::to_value(row.record).unwrap(), expected);
        assert!(reader.next().is_none());
    }
    let mut reader = RequestSourceReader::new(
        "private",
        Cursor::new("broken\n{}\n"),
        RequestFraming::Jsonl,
        ReaderOptions::default(),
    )
    .unwrap();
    assert_eq!(reader.next().unwrap().unwrap_err().kind(), ErrorKind::Usage);
    assert!(reader.next().is_none());
}

#[test]
fn canonical_source_framing_admits_before_opening_and_plans_table_rows() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("source-framing-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let path = folder.join("records.csv");
    std::fs::write(&path, "body\n\"Alpha.\nBeta.\"\nGamma.\n").unwrap();
    let request = |source: Value| {
        Request::from_json(&json!({"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"text","text":"Fits?"},"input":{"kind":"source","source":source},"options":{"field":["/body"]}}}).to_string()).unwrap().admit()
    };
    let source = json!({"paths":[path,path],"framing":"csv"});
    let actual = engine
        .plan_request(&request(source).unwrap(), RequestEnvironment::default())
        .unwrap();
    let expected = engine
        .plan(
            &Question::decide("Fits?").unwrap().cut(),
            ["Alpha.\nBeta.", "Gamma.", "Alpha.\nBeta.", "Gamma."],
        )
        .unwrap();
    assert_eq!(actual, expected);
    for source in [
        json!({"paths":["missing-private"],"framing":"csv","reading":{"unit":"file"}}),
        json!({"paths":["missing-private"],"framing":"jsonl","reading":{"unit":"window","window":2}}),
        json!({"paths":["missing-private"],"framing":"lines","media":"image","reading":{"unit":"file"}}),
    ] {
        assert_eq!(request(source).unwrap_err().kind(), ErrorKind::Usage);
    }
    assert_eq!(listener.count(), 0);
    std::fs::remove_dir_all(folder).unwrap();
}
