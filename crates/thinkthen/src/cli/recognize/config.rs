//! Recognition command and question-file precedence.

use std::path::Path;

use crate::args::RecognizeArguments;
use crate::cli::question_text;
use crate::core::{
    Description, RecognizeConfigError, RecognizeKinds, RecognizeSpec, RelationRule, rule_side,
};
use crate::failure::Failure;
use crate::failure::recognize::Error;

pub(super) fn settle(arguments: &RecognizeArguments) -> Result<RecognizeSpec, Failure> {
    let file = arguments
        .kinds
        .first()
        .and_then(|kind| kind.strip_prefix('@'));
    let mut spec = if let Some(path) = file {
        if arguments.kinds.len() != 1
            || !arguments.described.is_empty()
            || !arguments.relations.is_empty()
        {
            return Err(error(true, RecognizeConfigError::Shape));
        }
        let text = question_text::reference(Path::new(path), Failure::OpenQuestionFile)?;
        RecognizeSpec::parse(&text).map_err(|why| error(true, why))?
    } else {
        let kinds = command_kinds(arguments)?;
        let relations = arguments
            .relations
            .iter()
            .map(|held| command_rule(held))
            .collect::<Result<Vec<_>, _>>()?;
        RecognizeSpec::from_parts(
            kinds,
            relations,
            arguments.threshold.as_deref(),
            arguments.relation_threshold.as_deref(),
        )
        .map_err(|why| error(false, why))?
    };
    if file.is_some() {
        if let Some(threshold) = arguments.threshold.as_deref() {
            spec.threshold = threshold
                .parse()
                .map_err(|_| error(false, RecognizeConfigError::Threshold))?;
        }
        if let Some(threshold) = arguments.relation_threshold.as_deref() {
            spec.relation_threshold = threshold
                .parse()
                .map_err(|_| error(false, RecognizeConfigError::Threshold))?;
        }
        if !spec.threshold.is_cut() || !spec.relation_threshold.is_cut() {
            return Err(error(false, RecognizeConfigError::Threshold));
        }
    }
    let wording = |value: &str| {
        crate::core::QuestionText::new(value).map_err(|_| error(false, RecognizeConfigError::Shape))
    };
    if let Some(value) = &arguments.instructions {
        spec.instructions = Some(wording(value)?);
    }
    if let Some(value) = &arguments.entity_definition {
        spec.entity_definition = Some(wording(value)?);
    }
    Ok(spec)
}

fn command_kinds(arguments: &RecognizeArguments) -> Result<RecognizeKinds, Failure> {
    if !arguments.kinds.is_empty() && !arguments.described.is_empty() {
        return Err(error(false, RecognizeConfigError::Kinds));
    }
    if !arguments.described.is_empty() {
        return arguments
            .described
            .iter()
            .map(|entry| {
                let (name, description) = entry
                    .split_once('=')
                    .ok_or(Failure::Recognize(Error::KindWithoutSign))?;
                Ok((name.to_owned(), Some(Description::text(description))))
            })
            .collect();
    }
    Ok(arguments
        .kinds
        .iter()
        .map(|name| (name.clone(), None))
        .collect())
}

/// `NAME` relates any kind to any kind; `NAME=SOURCE:TARGET` names the sides.
fn command_rule(text: &str) -> Result<RelationRule, Failure> {
    let malformed = || error(false, RecognizeConfigError::Relation);
    let (name, source, target) = match text.split_once('=') {
        None if text.contains(':') => return Err(malformed()),
        None => (text, "*".to_owned(), "*".to_owned()),
        Some((name, ends)) => {
            let (source, target) = ends.split_once(':').ok_or_else(malformed)?;
            if source.contains(['=', ':']) || target.contains(['=', ':']) {
                return Err(malformed());
            }
            (name, rule_side(source), rule_side(target))
        }
    };
    Ok(RelationRule {
        name: name.to_owned(),
        source,
        target,
        either: false,
        single: false,
        reads: name.replace('_', " "),
    })
}

pub(super) fn error(file: bool, error: RecognizeConfigError) -> Failure {
    Failure::Recognize(crate::cli::failure::recognize::Error::Config { file, error })
}
