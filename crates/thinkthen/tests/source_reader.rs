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

#[test]
fn reader_debug_withholds_arbitrary_reader_evidence_before_and_after_intake() {
    let mut reader = FileReader::new(
        "notes",
        Cursor::new("PRIVATE_INPUT_MARKER\nsecond\n"),
        ReaderOptions::default(),
    )
    .unwrap();
    for stage in 0..2 {
        let debug = format!("{reader:?}");
        assert!(
            !debug.contains("PRIVATE_INPUT_MARKER"),
            "stage {stage}: {debug}"
        );
        assert!(!debug.contains("second"), "stage {stage}: {debug}");
        if stage == 0 {
            assert_eq!(
                reader.next().unwrap().unwrap().record,
                "PRIVATE_INPUT_MARKER"
            );
        }
    }
}

#[test]
fn image_handles_keep_original_bytes_and_omit_text_line_positions() {
    use thinkthen::{ImageMedia, InputFileReader, InputReaderOptions, ReaderMedia, SourceItem};
    let bytes = include_bytes!("../../../specification/fixtures/images/red.png");
    let options = InputReaderOptions {
        reading: ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
        media: ReaderMedia::Image,
    };
    let mut reader = InputFileReader::new("renamed.jpeg", Cursor::new(bytes), options).unwrap();
    let row = reader.next().unwrap().unwrap();
    let SourceItem::Image(image) = &row else {
        panic!("explicit image mode");
    };
    assert_eq!(image.record.media(), ImageMedia::Png);
    assert_eq!(image.record.bytes(), bytes);
    assert_eq!((image.record.width(), image.record.height()), (32, 32));
    let json = serde_json::to_value(row).unwrap();
    assert!(json.get("first_line").is_none());
    assert!(json.get("last_line").is_none());
    assert!(reader.next().is_none());
    assert!(
        InputFileReader::new(
            "image",
            Cursor::new(bytes),
            InputReaderOptions {
                reading: ReaderOptions::default(),
                media: ReaderMedia::Image
            }
        )
        .is_err()
    );
}

#[test]
fn image_read_failure_is_local_and_the_handle_stops_after_one_error() {
    use std::io::{self, BufRead, Read};
    use thinkthen::{ErrorKind, InputFileReader, InputReaderOptions, ReaderMedia};
    struct Broken;
    impl Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("PRIVATE_IMAGE_BYTES"))
        }
    }
    impl BufRead for Broken {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Err(io::Error::other("PRIVATE_IMAGE_BYTES"))
        }
        fn consume(&mut self, _: usize) {}
    }
    let mut reader = InputFileReader::new(
        "image",
        Broken,
        InputReaderOptions {
            reading: ReaderOptions {
                unit: SourceUnit::File,
                window: None,
            },
            media: ReaderMedia::Image,
        },
    )
    .unwrap();
    let error = reader.next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert!(!format!("{error:?}").contains("PRIVATE_IMAGE_BYTES"));
    assert!(reader.next().is_none());
}
