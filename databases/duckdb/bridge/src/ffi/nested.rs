//! Compact typed values for DuckDB's recognize and relations scalars.

use thinkthen::{CallOptions, Engine, Kind, Recognize};

use crate::errors::RowError;

pub(super) fn ask(
    kind: i32,
    argument: &str,
    members: &[String],
    from_file: bool,
) -> Result<Recognize, String> {
    match kind {
        8 => members
            .iter()
            .try_fold(Recognize::builder(), |built, name| {
                built.kind(Kind::new(name, None)?)
            })
            .and_then(thinkthen::RecognizeBuilder::build)
            .map_err(|error| RowError::from(error).text),
        9 if from_file => Recognize::from_json(argument).map_err(|error| {
            if error.kind() == thinkthen::ErrorKind::Usage {
                RowError::local(error.detail().message()).text
            } else {
                RowError::from(error).text
            }
        }),
        9 if argument.starts_with('@') => {
            Err(RowError::local("the question file was not read by this database").text)
        }
        9 => Recognize::from_json(argument).map_err(|error| RowError::from(error).text),
        _ => Err("thinkthen defect: the bridge got an unknown nested kind".to_owned()),
    }
}

fn count(bytes: &mut Vec<u8>, value: usize) -> Result<(), String> {
    let value =
        u32::try_from(value).map_err(|_| "thinkthen defect: too many nested values".to_owned())?;
    bytes.extend_from_slice(&value.to_ne_bytes());
    Ok(())
}

fn string(bytes: &mut Vec<u8>, value: &str) -> Result<(), String> {
    count(bytes, value.len())?;
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

fn offsets(bytes: &mut Vec<u8>, values: [usize; 3]) {
    for value in values {
        bytes.extend_from_slice(&i64::try_from(value).unwrap_or(i64::MAX).to_ne_bytes());
    }
}

pub(super) fn run(
    engine: &Engine,
    ask: &Recognize,
    texts: Vec<String>,
    deadline_ms: i64,
    kind: i32,
) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    for text in texts {
        let options = if deadline_ms == -1 {
            CallOptions::new()
        } else {
            CallOptions::new()
                .deadline_millis(deadline_ms)
                .map_err(|error| RowError::from(error).text)?
        };
        let found = engine
            .recognize_with(ask, &text, options)
            .map_err(|error| RowError::from(error).text)?;
        if kind == 8 {
            count(&mut bytes, found.entities().len())?;
            for entity in found.entities() {
                string(&mut bytes, entity.text())?;
                offsets(&mut bytes, [entity.start(), entity.end(), entity.length()]);
                string(&mut bytes, entity.kind())?;
                bytes.extend_from_slice(&entity.strength().to_ne_bytes());
            }
        } else {
            let relations = found.relations().unwrap_or_default();
            count(&mut bytes, relations.len())?;
            for relation in relations {
                string(&mut bytes, relation.relation())?;
                string(&mut bytes, relation.source().text())?;
                string(&mut bytes, relation.source().kind())?;
                string(&mut bytes, relation.target().text())?;
                string(&mut bytes, relation.target().kind())?;
                bytes.extend_from_slice(&relation.probability().to_ne_bytes());
            }
        }
    }
    Ok(bytes)
}
