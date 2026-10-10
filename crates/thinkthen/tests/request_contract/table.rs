//! Authorized table reads retain native originals and physical coordinates.
use super::*;
use std::io::Cursor;

#[test]
fn authorized_tables_preserve_strings_order_and_multiline_coordinates() {
    for (format, input) in [
        (
            TableFormat::Csv,
            "\u{feff}id,body,tail\r\n\r\n7,\"a,b\nsaid \"\"yes\"\"\",\r\n8,plain,✓\n",
        ),
        (
            TableFormat::Tsv,
            "\u{feff}id\tbody\ttail\r\n\r\n7\t\"a,b\nsaid \"\"yes\"\"\"\t\r\n8\tplain\t✓\n",
        ),
    ] {
        let rows = TableReader::new("caller-table", Cursor::new(input), format)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!((rows[0].first_line, rows[0].last_line), (3, 4));
        assert_eq!((rows[1].first_line, rows[1].last_line), (5, 5));
        assert_eq!(
            serde_json::to_string(&rows[0].record).unwrap(),
            r#"{"id":"7","body":"a,b\nsaid \"yes\"","tail":""}"#
        );
        assert_eq!(
            serde_json::to_string(&rows[1].record).unwrap(),
            r#"{"id":"8","body":"plain","tail":"✓"}"#
        );
        assert!(format!("{rows:?}").contains("<withheld>"));
    }

    let row = TableReader::new(
        "caller",
        Cursor::new(b" id ,body\r 7 ,odd\"quote"),
        TableFormat::Csv,
    )
    .unwrap()
    .next()
    .unwrap()
    .unwrap();
    assert_eq!((row.first_line, row.last_line), (2, 2));
    assert_eq!(
        serde_json::to_string(&row.record).unwrap(),
        r#"{" id ":" 7 ","body":"odd\"quote"}"#
    );
}

#[test]
fn authorized_table_refusals_hide_values_and_stop_bad_width_tail() {
    for (input, kind, message) in [
        (
            b"".as_slice(),
            ErrorKind::Usage,
            "the CSV header is missing because the input is empty",
        ),
        (
            b"private,private\n",
            ErrorKind::Usage,
            "the CSV header repeats a name",
        ),
        (
            b"private, \n",
            ErrorKind::Usage,
            "the CSV header has a blank name",
        ),
        (
            b"private,\xff\n",
            ErrorKind::Local,
            "the CSV header is not valid UTF-8",
        ),
    ] {
        let error =
            TableReader::new("hidden-file", Cursor::new(input), TableFormat::Csv).unwrap_err();
        assert_eq!(error.kind(), kind);
        assert_eq!(error.to_string(), message);
        assert!(!format!("{error:?}").contains("private"));
        assert!(!format!("{error:?}").contains("hidden-file"));
    }
    let mut rows = TableReader::new(
        "caller",
        Cursor::new(b"id,body\n7,secret,extra\n8,tail\n"),
        TableFormat::Csv,
    )
    .unwrap();
    let error = rows.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.to_string(),
        "the CSV record has 3 fields; its header has 2"
    );
    assert!(rows.next().is_none());
    let mut rows =
        TableReader::new("caller", Cursor::new(b"body\n\xff\n"), TableFormat::Csv).unwrap();
    assert_eq!(rows.next().unwrap().unwrap_err().kind(), ErrorKind::Local);
    assert!(
        TableReader::new("caller", Cursor::new(b"body\n"), TableFormat::Csv)
            .unwrap()
            .next()
            .is_none()
    );
}

#[test]
fn authorized_table_rows_execute_through_native_composed_feeds() {
    let listener = Listener::answering(response).unwrap();
    let request = Request::new(RequestCall::Decide(args(
        Question::decide("Fits?").unwrap().cut().into(),
        RequestInput::Feed {
            name: "table".into(),
            framing: RequestFraming::Document,
            reading: ReaderOptions::default(),
            images: vec![],
        },
    )))
    .admit()
    .unwrap();
    let reading = RecordReading::new(&["/body"], None, None).unwrap();
    let rows = TableReader::new(
        "caller.csv",
        Cursor::new(b"id,body\n7,Alpha\n8,Beta\n"),
        TableFormat::Csv,
    )
    .unwrap();
    let rows = rows.map(|row| {
        let row = row?;
        let location = SourceLocation::new(row.file, Some(row.first_line), Some(row.last_line))?;
        let composed = reading.compose(row.record)?;
        Ok(composed.map_original(|original| original.with_location(location).question_input()))
    });
    let outcome = engine(&listener)
        .execute_request(
            &request,
            RequestEnvironment {
                controls: CallOptions::new(),
                feed: Some(RequestFeed::from_records("table", rows).eager()),
            },
        )
        .unwrap();
    let RequestOutcome::Complete(done) = outcome else {
        panic!("complete table")
    };
    let RequestValue::Decisions(records) = done.value() else {
        panic!("decisions")
    };
    assert_eq!(records.len(), 2);
    let QuestionInput::Record(original) = records[0].original() else {
        panic!("table original")
    };
    assert_eq!(original.location().unwrap().first_line(), Some(2));
    assert_eq!(original.location().unwrap().file(), "caller.csv");
    assert_eq!(
        serde_json::to_string(original.original()).unwrap(),
        r#"{"id":"7","body":"Alpha"}"#
    );
    assert!(listener.count() > 0);
}
