//! File-reader behavior over authorized handles and explicit operands.

use std::io::Cursor;
use thinkthen::{FileReader, ReaderOptions, SourceUnit};

#[test]
fn windows_keep_original_endings_and_physical_lines() {
    let options = ReaderOptions {
        unit: SourceUnit::Window,
        window: Some(2),
    };
    let rows: Vec<_> = FileReader::new(
        "notes",
        Cursor::new("one\r\nβeta\r\n\r\n\r\nlast\n"),
        options,
    )
    .unwrap()
    .collect::<Result<_, _>>()
    .unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(
        (&rows[0].record, rows[0].first_line, rows[0].last_line),
        (&"one\r\nβeta".to_owned(), 1, 2)
    );
    assert_eq!(
        (&rows[1].record, rows[1].first_line, rows[1].last_line),
        (&"last".to_owned(), 5, 5)
    );
    assert_eq!(rows[0].span_lines(5, 9).unwrap(), (2, 2));
}

#[test]
fn whole_files_keep_all_bytes_and_span_ranges() {
    let options = ReaderOptions {
        unit: SourceUnit::File,
        window: None,
    };
    let row = FileReader::new("notes", Cursor::new("α\r\nsecond\n"), options)
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    assert_eq!(row.record, "α\r\nsecond\n");
    assert_eq!((row.first_line, row.last_line), (1, 2));
    assert_eq!(row.span_lines(0, 9).unwrap(), (1, 2));
}

#[test]
fn invalid_options_and_content_are_local_without_partial_records() {
    assert!(
        FileReader::new(
            "notes",
            Cursor::new("text"),
            ReaderOptions {
                unit: SourceUnit::Line,
                window: Some(2)
            }
        )
        .is_err()
    );
    let mut rows = FileReader::new(
        "notes",
        Cursor::new(b"good\n\xff\nlast\n"),
        ReaderOptions::default(),
    )
    .unwrap();
    assert_eq!(rows.next().unwrap().unwrap().record, "good");
    assert!(rows.next().unwrap().is_err());
    assert!(rows.next().is_none());
}

#[test]
fn folder_order_and_explicit_operand_order_keep_duplicate_occurrences() {
    let folder =
        std::env::temp_dir().join(format!("thinkthen-source-reader-{}", std::process::id()));
    std::fs::create_dir_all(folder.join("nested")).unwrap();
    std::fs::write(folder.join("z"), "z\n").unwrap();
    std::fs::write(folder.join(".hidden"), "hidden\n").unwrap();
    std::fs::write(folder.join("nested/a"), "a\n").unwrap();
    let paths = [folder.clone(), folder.join("z")];
    let records: Vec<_> = thinkthen::read_files(paths, ReaderOptions::default())
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(
        records
            .iter()
            .map(|r| r.record.as_str())
            .collect::<Vec<_>>(),
        ["hidden", "a", "z", "z"]
    );
    std::fs::remove_dir_all(folder).unwrap();
}
