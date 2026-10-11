//! The listed-answer codec for complete choose, score and tag questions.

use thinkthen::{Engine, Question};

fn string(bytes: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let len = u32::try_from(value.len())
        .map_err(|_| "thinkthen defect: a listed value is too large".to_owned())?;
    bytes.extend_from_slice(&len.to_ne_bytes());
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

pub(super) fn run(
    engine: &Engine,
    question: &Question,
    texts: Vec<String>,
    scope: super::CallScope<'_>,
) -> Result<Vec<u8>, String> {
    let result = super::request::run(
        engine,
        question.clone().into(),
        super::request::texts(texts),
        scope,
    )?
    .success(scope.total)?;
    let rows = result.rows;
    let mut bytes = Vec::new();
    for row in rows {
        match row {
            thinkthen::RequestSessionRow::Choice(row) if row.result().value().is_some() => {
                bytes.push(1);
                string(
                    &mut bytes,
                    row.result()
                        .value()
                        .ok_or("thinkthen defect: choice missing")?,
                )?;
            }
            thinkthen::RequestSessionRow::Choice(_) => bytes.push(0),
            thinkthen::RequestSessionRow::Score(row) => {
                bytes.push(2);
                bytes.extend_from_slice(&row.result().value().to_ne_bytes());
            }
            thinkthen::RequestSessionRow::Tags(row) => {
                let values = row.result().value();
                bytes.push(3);
                let count = u32::try_from(values.len())
                    .map_err(|_| "thinkthen defect: too many listed values".to_owned())?;
                bytes.extend_from_slice(&count.to_ne_bytes());
                for value in values {
                    string(&mut bytes, value)?;
                }
            }
            _ => {
                return Err("thinkthen defect: a listed question returned another kind".to_owned());
            }
        }
    }
    Ok(bytes)
}
