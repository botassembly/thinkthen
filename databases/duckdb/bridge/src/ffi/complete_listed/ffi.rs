//! The listed-answer codec for complete choose, score and tag questions.

use thinkthen::{Engine, Judgment, Question};

use crate::engines;

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
    options: thinkthen::CallOptions<'_>,
    total: Option<i64>,
) -> Result<Vec<u8>, String> {
    let rows = engine
        .details_many_with(question, texts, options)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| engines::call_error(error, total).text)?;
    let mut bytes = Vec::new();
    for row in rows {
        match row.value().value() {
            Judgment::Choice(Some(value)) => {
                bytes.push(1);
                string(&mut bytes, value)?;
            }
            Judgment::Choice(None) => bytes.push(0),
            Judgment::Score(value) => {
                bytes.push(2);
                bytes.extend_from_slice(&value.to_ne_bytes());
            }
            Judgment::Tags(values) => {
                bytes.push(3);
                let count = u32::try_from(values.len())
                    .map_err(|_| "thinkthen defect: too many listed values".to_owned())?;
                bytes.extend_from_slice(&count.to_ne_bytes());
                for value in values {
                    string(&mut bytes, value)?;
                }
            }
            Judgment::Decision(_) => {
                return Err("thinkthen defect: a listed question returned another kind".to_owned());
            }
        }
    }
    Ok(bytes)
}
