//! Bracket annotations preserve decoded text and count scalar positions once.
use super::{ExampleError, RecognitionExampleEntity, RecognitionExampleText};
use std::iter::Peekable;
use std::str::Chars;

pub(super) fn parse(source: &str) -> Result<RecognitionExampleText, ExampleError> {
    let mut chars = source.chars().peekable();
    let mut text = String::new();
    let mut entities = Vec::new();
    let mut scalar = 0;
    while let Some(character) = chars.next() {
        match character {
            '\\' => {
                text.push(escaped(&mut chars)?);
                scalar += 1;
            }
            '[' => match single(&mut chars)? {
                Some((body, kind)) => {
                    let start = scalar;
                    scalar += body.chars().count();
                    text.push_str(&body);
                    entities.push(RecognitionExampleEntity {
                        start,
                        end: scalar,
                        kind,
                    });
                }
                None => {
                    text.push('[');
                    scalar += 1;
                }
            },
            other => {
                text.push(other);
                scalar += 1;
            }
        }
    }
    Ok(RecognitionExampleText {
        text,
        entities,
        kinds: None,
    })
}

fn single(chars: &mut Peekable<Chars<'_>>) -> Result<Option<(String, String)>, ExampleError> {
    let mut scanned = chars.clone();
    let mut before = String::new();
    let mut after = String::new();
    let mut separated = false;
    let mut literal_brackets = Vec::new();
    while let Some(character) = scanned.next() {
        let character = match character {
            '\\' => {
                let decoded = escaped(&mut scanned)?;
                match decoded {
                    '[' => literal_brackets.push(true),
                    ']' => {
                        literal_brackets.pop();
                    }
                    _ => {}
                }
                decoded
            }
            '|' if !separated => {
                if literal_brackets.contains(&false) {
                    return Err(ExampleError::Brackets);
                }
                literal_brackets.clear();
                separated = true;
                continue;
            }
            '|' => return Err(ExampleError::Brackets),
            '[' => {
                literal_brackets.push(false);
                '['
            }
            ']' if !literal_brackets.is_empty() => {
                literal_brackets.pop();
                ']'
            }
            ']' => {
                if !separated {
                    return Ok(None);
                }
                let (text, kind) = (before.trim(), after.trim());
                if text.is_empty() || kind.is_empty() {
                    return Err(ExampleError::Brackets);
                }
                *chars = scanned;
                return Ok(Some((text.to_owned(), kind.to_owned())));
            }
            other => other,
        };
        if separated {
            after.push(character);
        } else {
            before.push(character);
        }
    }
    if separated {
        Err(ExampleError::Brackets)
    } else {
        Ok(None)
    }
}

fn escaped(chars: &mut Peekable<Chars<'_>>) -> Result<char, ExampleError> {
    match chars.next() {
        Some(character @ ('\\' | '[' | ']' | '|')) => Ok(character),
        _ => Err(ExampleError::Brackets),
    }
}
