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
    let admitted = request(source).unwrap();
    let actual = engine
        .plan_request(&admitted, RequestEnvironment::default())
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
    let RequestOutcome::Complete(done) = engine
        .execute_request(&admitted, RequestEnvironment::default())
        .unwrap()
    else {
        panic!("source decisions");
    };
    let RequestValue::Decisions(rows) = done.value() else {
        panic!("decisions");
    };
    assert_eq!(rows.len(), 4);
    let QuestionInput::Record(original) = rows[0].original() else {
        panic!("located source original");
    };
    assert_eq!(
        (
            original.location().unwrap().first_line(),
            original.location().unwrap().last_line()
        ),
        (Some(2), Some(3))
    );
    assert_eq!(
        serde_json::to_value(original.original()).unwrap(),
        json!({"body":"Alpha.\nBeta."})
    );
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn find_count_range_and_native_admission_share_the_complete_set_boundary() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for (none, expected) in [(false, 2..=255), (true, 1..=254)] {
        let range = find_candidate_count_range(none);
        assert_eq!(range, expected);
        for count in [
            *range.start() - 1,
            *range.start(),
            *range.end(),
            *range.end() + 1,
        ] {
            let request = Request::named(
                "find",
                RequestArguments {
                    question: RequestQuestion::Text {
                        text: "Fits?".to_owned(),
                    },
                    input: RequestInput::Units {
                        items: (0..count).map(|_| item("evidence")).collect(),
                    },
                    options: RequestOptions {
                        none,
                        ..RequestOptions::default()
                    },
                },
            )
            .unwrap()
            .admit();
            let planned = request
                .and_then(|request| engine.plan_request(&request, RequestEnvironment::default()));
            assert_eq!(
                planned.is_ok(),
                range.contains(&count),
                "none={none}, count={count}"
            );
            if let Err(error) = planned {
                assert_eq!(error.kind(), ErrorKind::Usage);
            }
            assert!(listener.requests().is_empty());
            assert_eq!(listener.count(), 0);
        }
    }
}

#[test]
fn saved_rank_set_refusals_retain_native_file_causes() {
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("rank-set-source-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let path = folder.join("question.json");
    for field in ["on", "threshold"] {
        let value = if field == "on" { json!("") } else { json!(0.5) };
        std::fs::write(
            &path,
            json!({"version":1,"questions":{"first":{"decide":"Fits?",field:value}}}).to_string(),
        )
        .unwrap();
        for question in [
            RequestQuestion::File { path: path.clone() },
            RequestQuestion::Reference {
                reference: format!("@{}", path.display()),
            },
        ] {
            let request = Request::for_function(
                RequestFunction::Rank,
                RequestArguments {
                    question,
                    input: RequestInput::Records { items: Vec::new() },
                    options: RequestOptions::default(),
                },
            )
            .admit()
            .unwrap();
            let error = request.resolve_question().unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Local);
            assert_eq!(
                error.to_string(),
                format!("`questions.first.{field}`: rank question sets take no threshold or on")
            );
            assert!(matches!(
                error.diagnostic(),
                Some(ErrorDiagnostic::QuestionSet(_))
            ));
        }
    }
    std::fs::remove_dir_all(folder).unwrap();
}
