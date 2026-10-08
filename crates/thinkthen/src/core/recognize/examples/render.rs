//! Answer the actual boundary questions; never restate their wording or piecer.
use super::{ExampleError, RecognitionExample, RecognitionExampleEntity};
use crate::core::{Piece, Question, RecognizeSpec, pieces, step_one_questions};

pub(super) fn one(
    spec: &RecognizeSpec,
    input: &RecognitionExample,
) -> Result<String, ExampleError> {
    let parsed;
    let example = match input {
        RecognitionExample::Brackets(text) => {
            parsed = super::brackets::parse(text)?;
            &parsed
        }
        RecognitionExample::Spans(example) => example,
    };
    let pieces = pieces(&example.text);
    if example.text.trim().is_empty() || pieces.is_empty() {
        return Err(ExampleError::Blank);
    }
    let current: Vec<&str> = spec.kinds.iter().map(|(kind, _)| kind.as_str()).collect();
    let default = if current.is_empty() {
        vec![super::super::ENTITY]
    } else {
        current.clone()
    };
    let vocabulary: Vec<&str> = example
        .kinds
        .as_ref()
        .map_or(default, |kinds| kinds.iter().map(String::as_str).collect());
    validate_vocabulary(&vocabulary)?;
    let mut entities: Vec<_> = example.entities.iter().collect();
    entities.sort_by_key(|entity| entity.start);
    validate_entities(&entities, &pieces, &vocabulary)?;
    let questions = step_one_questions(
        &example.text,
        &pieces,
        0..pieces.len(),
        &current,
        Some(spec),
    )
    .map_err(|_| ExampleError::Render)?;
    let mut rendered = format!("Snippet: {}", quoted(&example.text)?);
    for (piece, question) in pieces.iter().zip(questions) {
        let Question::Choose { text, .. } = question else {
            return Err(ExampleError::Render);
        };
        let entity = entities
            .iter()
            .find(|entity| entity.start <= piece.start && piece.end <= entity.end);
        let selected = entity.filter(|entity| {
            if current.is_empty() {
                entity.kind == super::super::ENTITY
            } else {
                current.contains(&entity.kind.as_str())
            }
        });
        rendered.push_str("\n\n");
        rendered.push_str(text.as_json().as_str().ok_or(ExampleError::Render)?);
        rendered.push_str("\nAnswer: ");
        rendered.push_str(&quoted(tag(piece, selected.copied()))?);
        if let Some(entity) = selected {
            rendered.push_str("; kind: ");
            rendered.push_str(&quoted(&entity.kind)?);
        }
    }
    Ok(rendered)
}

fn quoted(text: &str) -> Result<String, ExampleError> {
    serde_json::to_string(text).map_err(|_| ExampleError::Render)
}

fn validate_vocabulary(kinds: &[&str]) -> Result<(), ExampleError> {
    if kinds.is_empty()
        || kinds.iter().enumerate().any(|(at, kind)| {
            kind.trim().is_empty()
                || kind.chars().any(char::is_control)
                || kinds.iter().take(at).any(|held| held == kind)
        })
    {
        return Err(ExampleError::Vocabulary);
    }
    Ok(())
}

fn validate_entities(
    entities: &[&RecognitionExampleEntity],
    pieces: &[Piece],
    vocabulary: &[&str],
) -> Result<(), ExampleError> {
    let mut previous_end = 0;
    for entity in entities {
        if !vocabulary.contains(&entity.kind.as_str()) {
            return Err(ExampleError::Kind);
        }
        if entity.start >= entity.end
            || !pieces.iter().any(|piece| piece.start == entity.start)
            || !pieces.iter().any(|piece| piece.end == entity.end)
        {
            return Err(ExampleError::Edge);
        }
        if entity.start < previous_end {
            return Err(ExampleError::Overlap);
        }
        previous_end = entity.end;
    }
    Ok(())
}

fn tag(piece: &Piece, entity: Option<&RecognitionExampleEntity>) -> &'static str {
    let Some(entity) = entity else {
        return "OUT";
    };
    match (piece.start == entity.start, piece.end == entity.end) {
        (true, true) => "SINGLE",
        (true, false) => "BEGIN",
        (false, true) => "END",
        (false, false) => "INSIDE",
    }
}
