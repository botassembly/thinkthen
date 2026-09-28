//! The three SQL list-member verbs share one question-set and result codec.

use thinkthen::{Annotated, CallOptions, Engine, FailureCause, Question, QuestionSet};

use crate::errors::RowError;

pub(super) fn set(kind: i32, text: &str, members: &[String]) -> Result<QuestionSet, String> {
    if text.starts_with('@') || text.starts_with('{') {
        let message = match kind {
            4 => "choose takes its question as plain text and its options in the list",
            5 => "score takes its question as plain text and its levels in the list",
            6 => "tag takes its question as plain text and its labels in the list",
            _ => "the bridge got an unknown listed kind",
        };
        return Err(RowError::usage(message).text);
    }
    let refused = |error| RowError::from(error).text;
    let question = match kind {
        4 => members
            .iter()
            .try_fold(
                Question::choose_labels(text).map_err(refused)?,
                |built, label| built.label(label, None),
            )
            .and_then(thinkthen::LabelBuilder::build),
        5 => members
            .iter()
            .try_fold(Question::score(text).map_err(refused)?, |built, level| {
                built.level(level, None)
            })
            .and_then(thinkthen::ScoreBuilder::build),
        6 => members
            .iter()
            .try_fold(
                Question::tag_labels(text).map_err(refused)?,
                |built, label| built.label(label, None),
            )
            .and_then(thinkthen::LabelBuilder::build),
        _ => return Err("thinkthen defect: the bridge got an unknown listed kind".to_owned()),
    }
    .map_err(refused)?;
    QuestionSet::builder()
        .question("value", question)
        .and_then(thinkthen::QuestionSetBuilder::build)
        .map_err(refused)
}

fn cause(cause: FailureCause) -> &'static str {
    match cause {
        FailureCause::MissingAnswer => "the reply omitted the answer",
        FailureCause::WrongKind => "the reply answered another kind of question",
        FailureCause::MissingProbability => "an option or level had no probability",
        FailureCause::InvalidProbability => "a probability fell outside zero to one",
        FailureCause::InvalidDistribution => "a distribution did not total one",
        FailureCause::UnexpectedProbability => "a distribution named an option that was not sent",
    }
}

fn string(bytes: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let length = u32::try_from(value.len())
        .map_err(|_| "thinkthen defect: a listed value is too large".to_owned())?;
    bytes.extend_from_slice(&length.to_ne_bytes());
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

pub(super) fn run(
    engine: &Engine,
    set: &QuestionSet,
    texts: Vec<String>,
    options: CallOptions<'_>,
    total: Option<i64>,
) -> Result<Vec<u8>, String> {
    let rows = engine
        .annotate_with(set, texts, options)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| crate::engines::call_error(error, total).text)?;
    let mut bytes = Vec::new();
    for row in rows {
        match row.values().first().map(thinkthen::NamedAnnotation::value) {
            Some(Annotated::Choice(Some(value))) => {
                bytes.push(1);
                string(&mut bytes, value)?;
            }
            Some(Annotated::Score(value)) => {
                bytes.push(2);
                bytes.extend_from_slice(&value.to_ne_bytes());
            }
            Some(Annotated::Tags(values)) => {
                bytes.push(3);
                let count = u32::try_from(values.len())
                    .map_err(|_| "thinkthen defect: too many listed values".to_owned())?;
                bytes.extend_from_slice(&count.to_ne_bytes());
                for value in values {
                    string(&mut bytes, value)?;
                }
            }
            Some(Annotated::Failed(failed)) => {
                return Err(format!(
                    "thinkthen backend: the backend's answer could not be read: {}",
                    cause(failed.cause())
                ));
            }
            Some(Annotated::Choice(None)) | None => bytes.push(0),
            _ => return Err("thinkthen defect: a listed question returned another kind".to_owned()),
        }
    }
    Ok(bytes)
}
