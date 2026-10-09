//! Recognition command and question-file precedence.

use crate::args::RecognizeArguments;
use crate::core::{
    Description, RecognizeConfigError, RecognizeKinds, RecognizeSpec, RelationRule, rule_side,
};
use crate::failure::Failure;
use crate::failure::recognize::Error;

pub(super) fn settle(
    arguments: &RecognizeArguments,
    admitted: Option<&mut crate::AdmittedRequest>,
) -> Result<RecognizeSpec, Failure> {
    let file = arguments
        .kinds
        .first()
        .and_then(|kind| kind.strip_prefix('@'));
    if file.is_some()
        && (arguments.kinds.len() != 1
            || !arguments.described.is_empty()
            || !arguments.relations.is_empty())
    {
        return Err(error(true, RecognizeConfigError::Shape));
    }
    let prepared = admitted.is_some();
    let mut spec = if let Some(admitted) = admitted {
        match admitted.resolve_once().map_err(Failure::from)? {
            crate::RequestDefinition::Recognition(ask) => ask.0,
            crate::RequestDefinition::Recognize(file) => file.into_parts().0.0,
            _ => return Err(Failure::Defect("recognize lost its definition")),
        }
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
    if prepared && file.is_none() {
        return Ok(spec);
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
    if !prepared {
        if let Some(value) = arguments.snippet_pieces {
            spec.snippet_pieces = value;
        }
        spec.stage_context.overlay(&crate::RecognitionStageContext {
            boundary: arguments.boundary_context.clone(),
            kind_edge: arguments.kind_edge_context.clone(),
            relation: arguments.relation_context.clone(),
        });
        if let Some(mode) = &arguments.mode {
            spec.mode = crate::RecognitionMode::parse(mode)
                .ok_or(Failure::Usage(crate::RecognitionMode::USAGE))?;
        }
        spec.authored_relation_threshold |= arguments.relation_threshold.is_some();
        spec.validate_mode().map_err(|why| error(false, why))?;
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
