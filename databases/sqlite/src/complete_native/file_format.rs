//! Explicit SQL file formats select the shared logical framing grammar.
use thinkthen::{Error, ErrorKind, RequestFraming};
pub(crate) fn framing(format: Option<&str>) -> Result<Option<RequestFraming>, Error> {
    let Some(source) = format else {
        return Ok(None);
    };
    match serde_json::from_str::<String>(source).ok().as_deref() {
        Some("jsonl") => Ok(Some(RequestFraming::Jsonl)),
        Some("csv") => Ok(Some(RequestFraming::Csv)),
        Some("tsv") => Ok(Some(RequestFraming::Tsv)),
        _ => Err(Error::new(
            ErrorKind::Usage,
            "file format is jsonl, csv or tsv",
        )),
    }
}

/// Transport a native original once, retaining ordered JSON and physical coordinates.
#[allow(
    dead_code,
    reason = "only PostgreSQL and DuckDB transport native reader descriptors"
)]
pub(crate) fn descriptor(
    source: thinkthen::SourceRecord<thinkthen::RawRecord>,
) -> Result<serde_json::Value, Error> {
    let original = serde_json::to_string(&source.record)
        .map_err(|_| Error::new(ErrorKind::Defect, "source record did not encode"))?;
    Ok(
        serde_json::json!({"json_text":original,"source":{"file":source.file,"first_line":source.first_line,"last_line":source.last_line}}),
    )
}
