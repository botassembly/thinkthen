//! Source coordinates for recognized spans without changing model evidence.

use crate::SourceRecord;
use crate::cli::intake::{self, Position};
use crate::failure::Failure;

pub(super) fn locate(
    line: &mut Option<String>,
    position: &Position,
    text: &str,
    streams: bool,
    details: bool,
) -> Result<(), Failure> {
    let Some(original) = line.as_ref() else {
        return Ok(());
    };
    let mut json: serde_json::Value = serde_json::from_str(original)
        .map_err(|_| Failure::Defect("recognize output is not JSON"))?;
    let value = if streams || details {
        json.get_mut("value")
    } else {
        Some(&mut json)
    }
    .ok_or(Failure::Defect("recognize output has no value"))?;
    let entities = value
        .get_mut("entities")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or(Failure::Defect("recognize output has no entities"))?;
    let record = SourceRecord {
        record: text,
        file: position.file.clone().unwrap_or_default(),
        first_line: position.first,
        last_line: position.last,
    };
    for entity in entities {
        let start = entity
            .get("start")
            .and_then(serde_json::Value::as_u64)
            .and_then(|n| usize::try_from(n).ok());
        let end = entity
            .get("end")
            .and_then(serde_json::Value::as_u64)
            .and_then(|n| usize::try_from(n).ok());
        let (first, last) = record
            .span_lines(
                start.ok_or(Failure::Defect("recognized span has no start"))?,
                end.ok_or(Failure::Defect("recognized span has no end"))?,
            )
            .map_err(|_| Failure::Defect("recognized span is outside its source"))?;
        let entity = entity
            .as_object_mut()
            .ok_or(Failure::Defect("recognized entity is not an object"))?;
        entity.insert(
            "file".to_owned(),
            serde_json::Value::from(record.file.clone()),
        );
        entity.insert("first_line".to_owned(), serde_json::Value::from(first));
        entity.insert("last_line".to_owned(), serde_json::Value::from(last));
    }
    *line = Some(
        serde_json::to_string(&json)
            .map_err(|_| Failure::Defect("located recognition could not be written"))?,
    );
    if streams || details {
        intake::source_members(line, Some(position))?;
    } else if let Some(line) = line {
        *line = intake::source_value(&text, line, position)?;
    }
    Ok(())
}
