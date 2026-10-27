//! Text projections preserve original bytes while machine output retains typed rows.
use super::{local, output_error};
use crate::results::{AnyRow, Outcome};
use crate::{Cancel, Code, Error};
use std::fs::File;
use std::io::{self, BufWriter, IsTerminal, Read, Seek, SeekFrom, Write};

pub(super) struct Originals {
    data: BufWriter<File>,
    index: BufWriter<File>,
}
impl Originals {
    pub(super) fn new() -> Result<Self, Error> {
        let data = BufWriter::new(
            tempfile::tempfile().map_err(|_| local("original records could not be retained"))?,
        );
        let index = BufWriter::new(
            tempfile::tempfile().map_err(|_| local("original positions could not be retained"))?,
        );
        Ok(Self { data, index })
    }
    pub(super) fn push(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let result = (|| -> io::Result<()> {
            let offset = self.data.seek(SeekFrom::End(0))?;
            self.data.write_all(bytes)?;
            self.index.seek(SeekFrom::End(0))?;
            self.index.write_all(&offset.to_le_bytes())?;
            self.index.write_all(&(bytes.len() as u64).to_le_bytes())
        })();
        result.map_err(|_| local("original records could not be retained"))
    }
    pub(super) fn write(&mut self, position: u64, cancel: &Cancel) -> Result<(), Error> {
        let offset = position
            .checked_sub(1)
            .and_then(|n| n.checked_mul(16))
            .ok_or_else(|| {
                Error::from_contract(
                    Code::InternalBug,
                    "the engine returned an invalid record position",
                )
            })?;
        let result = (|| -> io::Result<()> {
            self.index.seek(SeekFrom::Start(offset))?;
            let mut number = [0; 8];
            self.index.get_mut().read_exact(&mut number)?;
            self.data
                .seek(SeekFrom::Start(u64::from_le_bytes(number)))?;
            self.index.get_mut().read_exact(&mut number)?;
            let length = u64::from_le_bytes(number);
            if io::stdout().is_terminal() {
                let mut text = String::new();
                self.data.get_mut().take(length).read_to_string(&mut text)?;
                let body = super::evidence(&text);
                let mut output = io::stdout().lock();
                write!(output, "{}", terminal_text(body))?;
                if text.ends_with('\n') {
                    writeln!(output)?;
                }
                return Ok(());
            }
            let copied = io::copy(
                &mut self.data.get_mut().take(length),
                &mut io::stdout().lock(),
            )?;
            if copied != length {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
            Ok(())
        })();
        result.map_err(|error| output_error(error, cancel))
    }
}

pub(super) fn row(row: &AnyRow, json: bool, cancel: &Cancel) -> Result<(), Error> {
    if json {
        return json_row(row, cancel);
    }
    let mut output = io::stdout().lock();
    if row.error().is_some() {
        return writeln!(output, "[{}] failed", row.position())
            .map_err(|error| output_error(error, cancel));
    }
    let value = serde_json::to_value(row).map_err(|_| local("the row could not be encoded"))?;
    let answer = value.get("answer");
    let marked = matches!(
        row,
        AnyRow::Tag(_) | AnyRow::Recognize(_) | AnyRow::Relate(_) | AnyRow::Annotate(_)
    );
    let text = if matches!(row, AnyRow::Find(_)) {
        answer
            .and_then(|answer| answer.get("text"))
            .and_then(serde_json::Value::as_str)
            .map_or_else(
                || {
                    if answer.is_some() {
                        String::new()
                    } else {
                        "unsure".to_owned()
                    }
                },
                |text| terminal_text(text).into_owned(),
            )
    } else {
        answer.map_or_else(|| "unsure".to_owned(), text)
    };
    let result = if marked {
        writeln!(
            output,
            "[{}]{}{}",
            row.position(),
            if text.is_empty() { "" } else { "\t" },
            text
        )
    } else {
        writeln!(output, "{text}")
    };
    result.map_err(|error| output_error(error, cancel))
}

pub(super) fn item(
    bytes: &[u8],
    format: super::input::Format,
) -> Result<Box<serde_json::value::RawValue>, Error> {
    let text = std::str::from_utf8(bytes).ok().map(super::evidence);
    if format == super::input::Format::Jsonl
        && let Some(text) = text
        && let Ok(value) = serde_json::from_str(text)
    {
        return Ok(value);
    }
    serde_json::value::to_raw_value(&text)
        .map_err(|_| local("the original item could not be encoded"))
}

pub(super) fn filter_row(
    row: &AnyRow,
    item: &serde_json::value::RawValue,
    cancel: &Cancel,
) -> Result<(), Error> {
    #[derive(serde::Serialize)]
    struct FilterRow<'a> {
        #[serde(flatten)]
        row: &'a AnyRow,
        item: &'a serde_json::value::RawValue,
        kept: bool,
    }
    json_row(
        &FilterRow {
            row,
            item,
            kept: keeps(row),
        },
        cancel,
    )
}

fn json_row(value: &impl serde::Serialize, cancel: &Cancel) -> Result<(), Error> {
    let mut output = io::stdout().lock();
    serde_json::to_writer(&mut output, value).map_err(|error| match error.io_error_kind() {
        Some(kind) => output_error(io::Error::new(kind, error), cancel),
        None => local("the row could not be encoded"),
    })?;
    writeln!(output).map_err(|error| output_error(error, cancel))
}

fn text(value: &serde_json::Value) -> String {
    use serde_json::Value;
    match value {
        Value::Null => String::new(),
        Value::String(s) => terminal_text(&s.replace(['\n', '\r', '\t'], " ")).into_owned(),
        Value::Array(values) => values.iter().map(text).collect::<Vec<_>>().join("\t"),
        Value::Object(fields) => {
            if let Some(label) = fields.get("text") {
                return fields.get("kind").map_or_else(
                    || text(label),
                    |kind| {
                        format!(
                            "{}: {} ({}..{})",
                            text(kind),
                            text(label),
                            fields.get("start").map_or(String::new(), text),
                            fields.get("end").map_or(String::new(), text)
                        )
                    },
                );
            }
            if let (Some(source), Some(relation), Some(target)) = (
                fields.get("source"),
                fields.get("relation"),
                fields.get("target"),
            ) {
                return format!(
                    "{} --{}--> {}",
                    text(source.get("text").unwrap_or(source)),
                    text(relation),
                    text(target.get("text").unwrap_or(target))
                );
            }
            fields
                .iter()
                .filter(|(name, _)| !matches!(name.as_str(), "function" | "detail" | "probability"))
                .map(|(name, value)| format!("{}: {}", terminal_text(name), text(value)))
                .collect::<Vec<_>>()
                .join("\t")
        }
        _ => value.to_string(),
    }
}

pub(super) fn keeps(row: &AnyRow) -> bool {
    matches!(row, AnyRow::Filter(row) if matches!(row.outcome(), Outcome::Answered { answer: true, .. }))
        || matches!(row, AnyRow::Rank(row) if matches!(row.outcome(), Outcome::Answered { .. }))
}

/// Escape record controls only when the output is a terminal.
fn terminal_text(text: &str) -> std::borrow::Cow<'_, str> {
    if !io::stdout().is_terminal() {
        return std::borrow::Cow::Borrowed(text);
    }
    std::borrow::Cow::Owned(crate::escape_diagnostic(text))
}
