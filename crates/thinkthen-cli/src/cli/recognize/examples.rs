//! Read caller-owned example files through the public example grammar.
use crate::failure::Failure;
use std::path::Path;

pub(super) fn at_record(mut failure: Failure, at: usize) -> Failure {
    if let Failure::Recognize(crate::failure::recognize::Error::Examples { record, .. }) =
        &mut failure
    {
        *record = Some(at + 1);
    }
    failure
}

pub(super) fn shared(path: Option<&Path>, question: &mut crate::Recognize) -> Result<(), Failure> {
    let Some(path) = path else {
        return Ok(());
    };
    let text = std::fs::read_to_string(path)
        .map_err(|_| Failure::Usage("recognition examples file must be readable UTF-8"))?;
    let lines = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let jsonl = lines
        .first()
        .is_some_and(|line| matches!(line.trim_start().chars().next(), Some('{' | '"')));
    let examples = lines.into_iter().enumerate().map(|(at, line)| {
        if jsonl {
            serde_json::from_str(line).map_err(|_| Failure::Recognize(crate::failure::recognize::Error::Examples {
                record: None,
                error: format!("example {}: recognition examples use bracket text or closed text/entities objects", at + 1),
            }))
        } else { Ok(crate::RecognitionExample::Brackets(line.to_owned())) }
    }).collect::<Result<Vec<_>, Failure>>()?;
    *question = question.clone().with_examples(examples)?;
    Ok(())
}
