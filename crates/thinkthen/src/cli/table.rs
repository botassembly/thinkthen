//! Bounded CSV and TSV parsing at the process boundary.

use std::fmt;
use std::io::Read;

use crate::core::{MAX_RECORD_BYTES, Record};
use csv_core::{ReadFieldResult, Reader, ReaderBuilder};

use crate::failure::Failure;

const BUFFER_BYTES: usize = 8 * 1024;

/// The table spelling used in diagnostics.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Kind {
    Csv,
    Tsv,
}

impl Kind {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Csv => "CSV",
            Self::Tsv => "TSV",
        }
    }

    const fn delimiter(self) -> u8 {
        match self {
            Self::Csv => b',',
            Self::Tsv => b'\t',
        }
    }
}

/// A table rule that input broke. No variant stores input text.
#[derive(Debug)]
pub(crate) enum Error {
    Empty(Kind),
    HeaderUtf8(Kind),
    HeaderBlank(Kind),
    HeaderControl(Kind),
    HeaderDuplicate(Kind),
    HeaderTooLarge(Kind),
    RecordUtf8(Kind),
    FieldCount {
        kind: Kind,
        expected: usize,
        found: usize,
    },
    RecordTooLarge(Kind),
}

impl Error {
    pub(crate) const fn input_failure(&self) -> bool {
        matches!(self, Self::HeaderUtf8(_) | Self::RecordUtf8(_))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty(kind) => write!(
                formatter,
                "the {} header is missing because the input is empty",
                kind.name()
            ),
            Self::HeaderUtf8(kind) => {
                write!(formatter, "the {} header is not valid UTF-8", kind.name())
            }
            Self::HeaderBlank(kind) => {
                write!(formatter, "the {} header has a blank name", kind.name())
            }
            Self::HeaderControl(kind) => write!(
                formatter,
                "the {} header has a name containing a control character",
                kind.name()
            ),
            Self::HeaderDuplicate(kind) => {
                write!(formatter, "the {} header repeats a name", kind.name())
            }
            Self::HeaderTooLarge(kind) => {
                write!(formatter, "the {} header is over 16 MiB", kind.name())
            }
            Self::RecordUtf8(kind) => {
                write!(formatter, "the {} record is not valid UTF-8", kind.name())
            }
            Self::FieldCount {
                kind,
                expected,
                found,
            } => {
                let noun = if *found == 1 { "field" } else { "fields" };
                write!(
                    formatter,
                    "the {} record has {found} {noun}; its header has {expected}",
                    kind.name()
                )
            }
            Self::RecordTooLarge(kind) => {
                write!(formatter, "the {} record is over 16 MiB", kind.name())
            }
        }
    }
}

/// Parsed table rows, each already a typed object for the pure core.
pub(crate) struct Rows<R> {
    reader: R,
    parser: Reader,
    kind: Kind,
    input: [u8; BUFFER_BYTES],
    start: usize,
    end: usize,
    eof: bool,
    stopped: bool,
    header: Option<Vec<String>>,
}

impl<R: Read> Rows<R> {
    pub(crate) fn new(reader: R, kind: Kind) -> Result<Self, Failure> {
        let mut builder = ReaderBuilder::new();
        builder.delimiter(kind.delimiter());
        let mut rows = Self {
            reader,
            parser: builder.build(),
            kind,
            input: [0; BUFFER_BYTES],
            start: 0,
            end: 0,
            eof: false,
            stopped: false,
            header: None,
        };
        let header = rows
            .parsed_row()?
            .ok_or(Failure::Table(Error::Empty(kind)))?;
        rows.read_header(header)?;
        Ok(rows)
    }

    fn parsed_row(&mut self) -> Result<Option<Vec<Vec<u8>>>, Failure> {
        let mut fields = Vec::new();
        let mut field = Vec::new();
        let mut raw = 0usize;
        let mut last = [0_u8; 2];
        loop {
            if self.start == self.end && !self.eof {
                self.start = 0;
                self.end = self.reader.read(&mut self.input).map_err(Failure::Input)?;
                self.eof = self.end == 0;
            }
            let input = self.input.get(self.start..self.end).unwrap_or_default();
            let mut output = [0_u8; BUFFER_BYTES];
            let (result, consumed, written) = self.parser.read_field(input, &mut output);
            for byte in input.iter().take(consumed) {
                last[0] = last[1];
                last[1] = *byte;
            }
            self.start += consumed;
            raw = raw.saturating_add(consumed);
            field.extend_from_slice(output.get(..written).unwrap_or_default());
            if raw > MAX_RECORD_BYTES + 2 {
                self.stopped = true;
                return Err(self.too_large());
            }
            match result {
                ReadFieldResult::InputEmpty | ReadFieldResult::OutputFull => {}
                ReadFieldResult::Field { record_end: false } => {
                    fields.push(std::mem::take(&mut field));
                }
                ReadFieldResult::Field { record_end: true } => {
                    fields.push(std::mem::take(&mut field));
                    return self.finished_record(fields, raw, last);
                }
                ReadFieldResult::End => return Ok((!fields.is_empty()).then_some(fields)),
            }
        }
    }

    fn finished_record(
        &mut self,
        fields: Vec<Vec<u8>>,
        raw: usize,
        last: [u8; 2],
    ) -> Result<Option<Vec<Vec<u8>>>, Failure> {
        let terminator =
            usize::from(last[1] == b'\n' || last[1] == b'\r') + usize::from(last == [b'\r', b'\n']);
        if raw.saturating_sub(terminator) > MAX_RECORD_BYTES {
            self.stopped = true;
            return Err(self.too_large());
        }
        Ok(Some(fields))
    }

    fn too_large(&self) -> Failure {
        Failure::Table(if self.header.is_none() {
            Error::HeaderTooLarge(self.kind)
        } else {
            Error::RecordTooLarge(self.kind)
        })
    }

    fn read_header(&mut self, fields: Vec<Vec<u8>>) -> Result<(), Failure> {
        let mut names = Vec::with_capacity(fields.len());
        for (place, bytes) in fields.into_iter().enumerate() {
            let bytes = if place == 0 {
                bytes
                    .strip_prefix(&[0xef, 0xbb, 0xbf])
                    .unwrap_or(&bytes)
                    .to_vec()
            } else {
                bytes
            };
            let name = String::from_utf8(bytes)
                .map_err(|_| Failure::Table(Error::HeaderUtf8(self.kind)))?;
            if name.chars().all(char::is_whitespace) {
                return Err(Failure::Table(Error::HeaderBlank(self.kind)));
            }
            if name.chars().any(char::is_control) {
                return Err(Failure::Table(Error::HeaderControl(self.kind)));
            }
            if names.contains(&name) {
                return Err(Failure::Table(Error::HeaderDuplicate(self.kind)));
            }
            names.push(name);
        }
        self.header = Some(names);
        Ok(())
    }
}

impl<R: Read> Iterator for Rows<R> {
    type Item = Result<Record, Failure>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }
        let row = match self.parsed_row() {
            Ok(Some(row)) => row,
            Ok(None) => return None,
            Err(error) => return Some(Err(error)),
        };
        let header = self.header.as_ref()?;
        if row.len() != header.len() {
            self.stopped = true;
            return Some(Err(Failure::Table(Error::FieldCount {
                kind: self.kind,
                expected: header.len(),
                found: row.len(),
            })));
        }
        let values = row
            .into_iter()
            .map(|bytes| {
                String::from_utf8(bytes).map_err(|_| Failure::Table(Error::RecordUtf8(self.kind)))
            })
            .collect::<Result<Vec<_>, _>>();
        Some(
            values
                .map(|values| Record::string_fields(header.iter().cloned().zip(values).collect())),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::io::Cursor;
    use std::io::{self, Read};
    use std::rc::Rc;

    use crate::core::json_line;

    use super::{BUFFER_BYTES, Kind, Rows};

    fn parsed(kind: Kind, input: &[u8]) -> Result<Vec<String>, String> {
        Rows::new(Cursor::new(input), kind)
            .map_err(|error| match error {
                crate::failure::Failure::Table(error) => error.to_string(),
                other => format!("unexpected failure: {other:?}"),
            })?
            .map(|row| row.map(|record| json_line(&record).expect("a record renders")))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| match error {
                crate::failure::Failure::Table(error) => error.to_string(),
                other => format!("unexpected failure: {other:?}"),
            })
    }

    fn header_at(size: usize, bom: bool, ending: &[u8]) -> Vec<u8> {
        let mut header = if bom {
            vec![0xef, 0xbb, 0xbf]
        } else {
            Vec::new()
        };
        header.resize(size, b'h');
        header.extend_from_slice(ending);
        header
    }

    #[test]
    fn csv_core_owns_table_grammar() {
        let csv = concat!(
            "\u{feff} id ,body,tail\r\n",
            " 7 ,\"a,b\nsaid \"\"yes\"\"\",\r\n",
            "\r\n",
            "8,plain,✓\n",
            "9,odd\"quote,z\n",
        );
        assert_eq!(
            parsed(Kind::Csv, csv.as_bytes()).expect("valid CSV"),
            [
                "{\" id \":\" 7 \",\"body\":\"a,b\\nsaid \\\"yes\\\"\",\"tail\":\"\"}",
                "{\" id \":\"8\",\"body\":\"plain\",\"tail\":\"✓\"}",
                "{\" id \":\"9\",\"body\":\"odd\\\"quote\",\"tail\":\"z\"}",
            ]
        );
        assert_eq!(
            parsed(Kind::Tsv, b"a\tb\r\nleft\t\"right\"\r\n").expect("valid TSV"),
            ["{\"a\":\"left\",\"b\":\"right\"}"]
        );
    }

    #[test]
    fn header_and_row_rules_name_no_input_values() {
        let cases: &[(&[u8], &str)] = &[
            (b"", "the CSV header is missing because the input is empty"),
            (b" ,b\n1,2\n", "the CSV header has a blank name"),
            (b"a,a\n1,2\n", "the CSV header repeats a name"),
            (
                b"a,\x01\n1,2\n",
                "the CSV header has a name containing a control character",
            ),
            (b"a,b\n1\n", "the CSV record has 1 field; its header has 2"),
        ];
        for (input, expected) in cases {
            assert_eq!(
                parsed(Kind::Csv, input).expect_err("input is refused"),
                *expected
            );
        }
    }

    #[test]
    fn a_header_without_data_is_an_empty_dataset() {
        assert!(
            parsed(Kind::Csv, b"a,b\n")
                .expect("header is valid")
                .is_empty()
        );
    }

    #[test]
    fn invalid_utf8_names_only_its_location() {
        assert_eq!(
            parsed(Kind::Csv, b"a,\xff\n1,2\n").expect_err("header is refused"),
            "the CSV header is not valid UTF-8"
        );
        assert_eq!(
            parsed(Kind::Csv, b"a,b\n1,\xff\n").expect_err("row is refused"),
            "the CSV record is not valid UTF-8"
        );
    }

    #[test]
    fn encoded_header_and_records_hold_at_the_sixteen_mibibyte_edge() {
        let limit = crate::core::MAX_RECORD_BYTES;
        // The byte order mark counts, and `\r\n` is the widest ending.
        assert!(Rows::new(Cursor::new(header_at(limit, true, b"\r\n")), Kind::Csv).is_ok());
        let error = Rows::new(Cursor::new(header_at(limit + 1, true, b"\r\n")), Kind::Csv)
            .err()
            .expect("oversized header is refused");
        assert!(matches!(
            error,
            crate::failure::Failure::Table(super::Error::HeaderTooLarge(Kind::Csv))
        ));

        // No ending and the widest ending bound the terminator allowance.
        for ending in [b"\r\n".as_slice(), b"".as_slice()] {
            let mut exact = b"value\n".to_vec();
            exact.extend(std::iter::repeat_n(b'x', limit));
            exact.extend_from_slice(ending);
            assert!(
                Rows::new(Cursor::new(exact), Kind::Csv)
                    .expect("valid header")
                    .next()
                    .expect("one record")
                    .is_ok()
            );

            let mut over = b"value\n".to_vec();
            over.extend(std::iter::repeat_n(b'x', limit + 1));
            over.extend_from_slice(ending);
            let error = Rows::new(Cursor::new(over), Kind::Csv)
                .expect("valid header")
                .next()
                .expect("one refusal")
                .expect_err("oversized record is refused");
            assert!(matches!(
                error,
                crate::failure::Failure::Table(super::Error::RecordTooLarge(Kind::Csv))
            ));
        }

        // A quoted line feed and a doubled quote count as raw bytes.
        let quoted = |body: usize| {
            let mut record = b"value\n\"a\n\"\"".to_vec();
            record.extend(std::iter::repeat_n(b'x', body));
            record.push(b'"');
            Rows::new(Cursor::new(record), Kind::Csv)
                .expect("valid header")
                .next()
                .expect("one quoted record")
        };
        assert!(quoted(limit - 6).is_ok());
        assert!(quoted(limit - 5).is_err());
    }

    struct Counted {
        inner: Cursor<Vec<u8>>,
        read: Rc<Cell<usize>>,
    }

    impl Read for Counted {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let read = self.inner.read(buffer)?;
            self.read.set(self.read.get() + read);
            Ok(read)
        }
    }

    #[test]
    fn an_oversized_tail_is_never_a_second_record() {
        let mut input = b"value\n".to_vec();
        input.extend(std::iter::repeat_n(b'x', crate::core::MAX_RECORD_BYTES + 3));
        input.extend_from_slice(b"\ntail\n");
        let consumed = Rc::new(Cell::new(0));
        let reader = Counted {
            inner: Cursor::new(input),
            read: Rc::clone(&consumed),
        };
        let mut rows = Rows::new(reader, Kind::Csv).expect("valid header");
        assert!(rows.next().expect("one refusal").is_err());
        assert!(rows.next().is_none());
        assert!(
            consumed.get() <= crate::core::MAX_RECORD_BYTES + 2 * BUFFER_BYTES,
            "read {} bytes",
            consumed.get()
        );
    }
}
