//! Native bounded readers retain the complete relation set before execution.
use crate::failure::Failure;
use crate::{
    FileReader, RawRecord, ReaderOptions, RecordReading, RequestFraming as Framing, SourceUnit,
    TableFormat, TableReader,
};
use std::io::BufRead;

pub(super) fn read(
    source: Box<dyn BufRead + Send>,
    framing: Framing,
    fields: &super::config::Fields,
) -> Result<Vec<RawRecord>, Failure> {
    match framing {
        Framing::Document => document(source, fields),
        Framing::Lines | Framing::Jsonl => stream(source, framing),
        Framing::Csv => table(source, TableFormat::Csv),
        Framing::Tsv => table(source, TableFormat::Tsv),
    }
}
fn document(
    source: Box<dyn BufRead + Send>,
    fields: &super::config::Fields,
) -> Result<Vec<RawRecord>, Failure> {
    let composition = RecordReading::new(&[&fields.name, &fields.kind], None, None)?;
    let mut reader = FileReader::new(
        "",
        source,
        ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
    )?;
    let bytes = reader
        .next_bytes()
        .transpose()
        .map_err(input_error)?
        .map_or_else(Vec::new, |unit| unit.record);
    let document = composition.read_original(&bytes, Framing::Document)?;
    let elements = document
        .content()
        .and_then(crate::QuestionContent::elements)
        .ok_or(Failure::Usage("the entity document is one JSON array"))?;
    elements
        .map(|element| RawRecord::json(&element.to_json()?).map_err(Failure::from))
        .collect()
}
fn stream(source: Box<dyn BufRead + Send>, framing: Framing) -> Result<Vec<RawRecord>, Failure> {
    let composition = RecordReading::new(&[], None, None)?;
    let mut reader = FileReader::new("", source, ReaderOptions::default())?;
    let mut records = Vec::new();
    while let Some(unit) = reader.next_bytes() {
        records.push(composition.read_original(&unit.map_err(input_error)?.record, framing)?);
    }
    Ok(records)
}
fn table(source: Box<dyn BufRead + Send>, format: TableFormat) -> Result<Vec<RawRecord>, Failure> {
    TableReader::new("", source, format)
        .map_err(input_error)?
        .map(|row| row.map(|row| row.record).map_err(input_error))
        .collect()
}

fn input_error(error: crate::Error) -> Failure {
    if let Some(cause) =
        std::error::Error::source(&error).and_then(|cause| cause.downcast_ref::<std::io::Error>())
    {
        return Failure::Input(std::io::Error::new(cause.kind(), cause.to_string()));
    }
    Failure::from(error)
}
