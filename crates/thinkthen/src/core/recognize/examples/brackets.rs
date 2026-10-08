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
            '[' if chars.peek() == Some(&'[') => {
                chars.next();
                let kind = label(&mut chars)?;
                let start = scalar;
                let body = body(&mut chars)?;
                scalar += body.chars().count();
                if start == scalar {
                    return Err(ExampleError::Brackets);
                }
                text.push_str(&body);
                entities.push(RecognitionExampleEntity {
                    start,
                    end: scalar,
                    kind,
                });
            }
            ']' if chars.peek() == Some(&']') => return Err(ExampleError::Brackets),
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

fn escaped(chars: &mut Peekable<Chars<'_>>) -> Result<char, ExampleError> {
    match chars.next() {
        Some(character @ ('\\' | '[' | ']' | '|')) => Ok(character),
        _ => Err(ExampleError::Brackets),
    }
}

fn label(chars: &mut Peekable<Chars<'_>>) -> Result<String, ExampleError> {
    let mut label = String::new();
    while let Some(character) = chars.next() {
        match character {
            '|' if !label.trim().is_empty() => return Ok(label),
            '\\' => label.push(escaped(chars)?),
            '[' | ']' | '|' => return Err(ExampleError::Brackets),
            other => label.push(other),
        }
    }
    Err(ExampleError::Brackets)
}

fn body(chars: &mut Peekable<Chars<'_>>) -> Result<String, ExampleError> {
    let mut text = String::new();
    while let Some(character) = chars.next() {
        match character {
            '\\' => text.push(escaped(chars)?),
            '[' if chars.peek() == Some(&'[') => return Err(ExampleError::Brackets),
            ']' if chars.peek() == Some(&']') => {
                chars.next();
                return Ok(text);
            }
            other => text.push(other),
        }
    }
    Err(ExampleError::Brackets)
}
