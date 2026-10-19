//! Offline callers inspect the original native representation without reparsing.
use thinkthen::RawRecord;

#[test]
fn borrowed_inspection_preserves_authored_order_false_null_and_exact_numbers() {
    let original = RawRecord::json(r#"{"z":false,"a":null,"a/b":[42,0.5,"x"]}"#).unwrap();
    let content = original.content().unwrap();
    let names = content
        .fields()
        .unwrap()
        .map(|(name, _)| name)
        .collect::<Vec<_>>();
    assert_eq!(names, ["z", "a", "a/b"]);
    assert_eq!(content.member("z").unwrap().boolean(), Some(false));
    assert!(content.member("a").unwrap().is_null());
    assert!(content.member("absent").is_none());
    assert_eq!(
        content
            .pointer("/a~1b/0")
            .unwrap()
            .unwrap()
            .number()
            .unwrap()
            .as_u64(),
        Some(42)
    );
    let values = content
        .member("a/b")
        .unwrap()
        .elements()
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(values.len(), 3);
    assert_eq!(values[1].number().unwrap().as_f64(), Some(0.5));
    assert_eq!(values[2].text(), Some("x"));
    assert!(content.pointer("/z/anything").unwrap().is_none());
    assert!(content.pointer("$.z").is_err());
    assert!(content.pointer("/a~1b/01").unwrap().is_none());
}

#[test]
fn read_only_json_preserves_shapes_and_shared_borrowed_content_without_execution_admission() {
    use thinkthen::{ParsedJson, RawRecord};
    for text in [
        "null",
        "false",
        "0",
        "1.25",
        r#""evidence""#,
        "[]",
        "{}",
        r#"{"z":false,"a":null}"#,
    ] {
        let value = ParsedJson::from_json(text).unwrap();
        assert_eq!(value.content().to_json().unwrap(), text);
        let clone = value.clone();
        drop(value);
        assert_eq!(clone.content().to_json().unwrap(), text);
    }
    let marker = "private_read_only_evidence_0639";
    let value = ParsedJson::from_json(&format!(r#"{{"evidence":"{marker}"}}"#)).unwrap();
    assert!(!format!("{value:?}").contains(marker));
    let clone = value.clone();
    let original = value.content().fields().unwrap().next().unwrap().1;
    let shared = clone.content().fields().unwrap().next().unwrap().1;
    assert!(std::ptr::eq(
        original.text().unwrap().as_ptr(),
        shared.text().unwrap().as_ptr()
    ));
    let oversized = format!(r#"{{"evidence":"{}"}}"#, "x".repeat(RawRecord::max_bytes()));
    let value = ParsedJson::from_json(&oversized).unwrap();
    assert_eq!(value.content().to_json().unwrap(), oversized);
    assert!(RawRecord::json(&oversized).is_err());
}

#[test]
fn raw_physical_units_share_the_cursor_without_skipping_or_decoding() {
    use std::io::Cursor;
    use thinkthen::{FileReader, ReaderOptions, SourceUnit};
    let mut reader = FileReader::new(
        "input",
        Cursor::new(b"\n \r\nx\r\nlast"),
        ReaderOptions::default(),
    )
    .unwrap();
    let blank = reader.next_bytes().unwrap().unwrap();
    assert_eq!(blank.record, b"\n");
    assert_eq!((blank.first_line, blank.last_line), (1, 1));
    let text = reader.next().unwrap().unwrap();
    assert_eq!(text.record, "x");
    assert_eq!((text.first_line, text.last_line), (3, 3));
    let tail = reader.next_bytes().unwrap().unwrap();
    assert_eq!(tail.record, b"last");
    assert_eq!((tail.first_line, tail.last_line), (4, 4));
    assert!(reader.next_bytes().is_none());
    let mut window = FileReader::new(
        "input",
        Cursor::new(b"\r\n x\r\nz"),
        ReaderOptions {
            unit: SourceUnit::Window,
            window: Some(2),
        },
    )
    .unwrap();
    let first = window.next_bytes().unwrap().unwrap();
    assert_eq!(first.record, b"\r\n x\r\n");
    assert_eq!((first.first_line, first.last_line), (1, 2));
    assert_eq!(window.next().unwrap().unwrap().record, "z");
    let mut empty = FileReader::new(
        "empty",
        Cursor::new(b""),
        ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
    )
    .unwrap();
    let raw = empty.next_bytes().unwrap().unwrap();
    assert!(raw.record.is_empty());
    assert_eq!((raw.first_line, raw.last_line), (1, 1));
    assert!(empty.next().is_none());
}

#[test]
fn raw_original_admission_retains_annotation_fallback_and_size_precedence() {
    use thinkthen::{ErrorDiagnostic, JsonFailure, RawRecord, RecordReading, RequestFraming};
    let reading = RecordReading::new(&[], None, None).unwrap();
    let oversized = vec![0xff; RawRecord::max_bytes() + 2];
    let error = reading
        .read_original(&oversized, RequestFraming::Lines)
        .unwrap_err();
    assert!(error.to_string().contains("over 16 MiB"));
    assert!(!matches!(
        error.diagnostic(),
        Some(ErrorDiagnostic::InvalidUtf8)
    ));
    let blank = reading
        .read_original(b"", RequestFraming::Document)
        .unwrap();
    assert_eq!(blank.literal(), Some(""));
    assert!(reading.read_document(b"").is_err());
    let syntax = reading.read_annotation_document(b"{broken").unwrap();
    assert_eq!(syntax.literal(), Some("{broken"));
    for text in ["false", "null", "2.5", "[]", "{}"] {
        assert!(
            reading
                .read_annotation_document(text.as_bytes())
                .unwrap()
                .content()
                .is_some()
        );
    }
    let deep = format!("{}0{}", "[".repeat(130), "]".repeat(130));
    for (text, expected) in [
        ("{\"x\":1,\"x\":2}".to_owned(), JsonFailure::DuplicateName),
        ("1e9999".to_owned(), JsonFailure::NotFinite),
        (deep, JsonFailure::TooDeep),
    ] {
        let error = reading
            .read_annotation_document(text.as_bytes())
            .unwrap_err();
        assert!(
            matches!(error.diagnostic(), Some(ErrorDiagnostic::RecordJson(actual)) if actual == expected)
        );
    }
    for framing in [RequestFraming::Csv, RequestFraming::Tsv] {
        assert!(reading.read_original(b"", framing).is_err());
    }
}

struct FailingReader {
    remaining: &'static [u8],
}
impl std::io::Read for FailingReader {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        use std::io::BufRead as _;
        let source = self.fill_buf()?;
        let count = source.len().min(output.len());
        let destination = output
            .get_mut(..count)
            .ok_or_else(|| std::io::Error::other("short output buffer"))?;
        let bytes = source
            .get(..count)
            .ok_or_else(|| std::io::Error::other("short input buffer"))?;
        destination.copy_from_slice(bytes);
        self.consume(count);
        Ok(count)
    }
}
impl std::io::BufRead for FailingReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.remaining.is_empty() {
            Err(std::io::Error::other("private-caller-io-marker"))
        } else {
            Ok(self.remaining)
        }
    }
    fn consume(&mut self, count: usize) {
        self.remaining = self.remaining.get(count..).unwrap_or_default();
    }
}
fn assert_caller_io(error: &thinkthen::Error) -> Result<(), Box<dyn std::error::Error>> {
    let cause = std::error::Error::source(error)
        .ok_or("missing caller IO source")?
        .downcast_ref::<std::io::Error>()
        .ok_or("unexpected caller IO source type")?;
    assert_eq!(cause.to_string(), "private-caller-io-marker");
    for shown in [
        format!("{error:?}"),
        error.to_string(),
        format!("{:?}", error.diagnostic()),
        serde_json::to_string(&error.complete())?,
    ] {
        assert!(!shown.contains("private-caller-io-marker"));
    }
    Ok(())
}
#[test]
fn caller_text_and_table_io_causes_are_explicit_and_normal_errors_withhold_them() {
    use thinkthen::{FileReader, ReaderOptions, TableFormat, TableReader};
    let mut text = FileReader::new(
        "input",
        FailingReader { remaining: b"" },
        ReaderOptions::default(),
    )
    .unwrap();
    let error = text.next_bytes().unwrap().unwrap_err();
    assert_caller_io(&error).unwrap();
    assert!(text.next().is_none());
    let error =
        TableReader::new("input", FailingReader { remaining: b"" }, TableFormat::Csv).unwrap_err();
    assert_caller_io(&error).unwrap();
    let mut table = TableReader::new(
        "input",
        FailingReader {
            remaining: b"column\n",
        },
        TableFormat::Csv,
    )
    .unwrap();
    let error = table.next().unwrap().unwrap_err();
    assert_caller_io(&error).unwrap();
    assert!(table.next().is_none());
}

#[test]
fn raw_cut_units_keep_the_native_overread_bound_and_abandon_unterminated_tails() {
    use std::{
        cell::Cell,
        io::{BufRead, Read},
        rc::Rc,
    };
    use thinkthen::{FileReader, RawRecord, ReaderOptions, RecordReading, RequestFraming};
    struct Counted<'a> {
        bytes: &'a [u8],
        consumed: Rc<Cell<usize>>,
    }
    impl Read for Counted<'_> {
        fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
            let count = output.len().min(self.bytes.len());
            output[..count].copy_from_slice(&self.bytes[..count]);
            self.consume(count);
            Ok(count)
        }
    }
    impl BufRead for Counted<'_> {
        fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
            Ok(self.bytes)
        }
        fn consume(&mut self, count: usize) {
            self.bytes = &self.bytes[count..];
            self.consumed.set(self.consumed.get() + count);
        }
    }
    let mut bytes = vec![0xff; RawRecord::max_bytes() + 3];
    bytes.extend_from_slice(b"\nnext\n");
    let consumed = Rc::new(Cell::new(0));
    let mut reader = FileReader::new(
        "input",
        Counted {
            bytes: &bytes,
            consumed: consumed.clone(),
        },
        ReaderOptions::default(),
    )
    .unwrap();
    let raw = reader.next_bytes().unwrap().unwrap();
    assert_eq!(raw.record.len(), RawRecord::max_bytes() + 2);
    assert_eq!(consumed.get(), RawRecord::max_bytes() + 2);
    let error = RecordReading::new(&[], None, None)
        .unwrap()
        .read_original(&raw.record, RequestFraming::Lines)
        .unwrap_err();
    assert!(error.to_string().contains("over 16 MiB"));
    assert!(reader.next_bytes().is_none());
    assert_eq!(consumed.get(), RawRecord::max_bytes() + 2);
}
