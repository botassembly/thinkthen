use std::fs;

use serde::Serialize;

use crate::args::RelateArguments;
use crate::core::{Framing, ModelName, RelateConfigError, RelateSpec, Source};
use crate::failure::Failure;

pub(super) struct Settled {
    pub(super) spec: RelateSpec,
    pub(super) framing: Framing,
    pub(super) from: Option<From>,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub(super) struct From {
    question: Source,
    threshold: Source,
    model: Source,
    field: Source,
    kind_field: Source,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<Source>,
}

pub(super) fn settle(arguments: &RelateArguments) -> Result<Settled, Failure> {
    if arguments.common.jobs.is_some() {
        return Err(Failure::Usage(
            "`relate` sends its requests in order, so it takes no --jobs",
        ));
    }
    if arguments.common.field.len() > 1 {
        return Err(Failure::Usage("--field takes one pointer on `relate`"));
    }
    let file = match arguments.relations.as_slice() {
        [only] => only.strip_prefix('@'),
        many if many.iter().any(|relation| relation.starts_with('@')) => {
            return Err(Failure::Usage(
                "relate takes inline relation rules or one @FILE, never both",
            ));
        }
        _ => None,
    };
    let (mut spec, mut from) = if let Some(path) = file {
        if arguments.either {
            return Err(config_error(false, RelateConfigError::Relation));
        }
        from_file(path)?
    } else {
        (
            RelateSpec::inline(&arguments.relations, arguments.either)
                .map_err(|error| config_error(false, error))?,
            None,
        )
    };
    if let Some(threshold) = arguments.threshold.as_deref() {
        spec.override_threshold(threshold)
            .map_err(|error| config_error(false, error))?;
        if let Some(sources) = &mut from {
            sources.threshold = Source::CommandLine;
        }
    }
    let name = arguments.common.field.first().map(String::as_str);
    spec.override_fields(name, arguments.kind_field.as_deref())
        .map_err(|error| config_error(false, error))?;
    if let Some(sources) = &mut from {
        if name.is_some() {
            sources.field = Source::CommandLine;
        }
        if arguments.kind_field.is_some() {
            sources.kind_field = Source::CommandLine;
        }
    }
    if let Some(model) = arguments.common.model.as_deref() {
        spec.model = Some(
            ModelName::new(model)
                .map_err(|_| Failure::Usage("--model is text, not white space"))?,
        );
        if let Some(sources) = &mut from {
            sources.model = Source::CommandLine;
        }
    }
    let framing = arguments.common.framing();
    if framing == Framing::Lines {
        lines_only(name.is_some() || arguments.kind_field.is_some(), &spec)?;
    }
    Ok(Settled {
        spec,
        framing,
        from,
    })
}

/// Read the question file and name which values it supplied.
fn from_file(path: &str) -> Result<(RelateSpec, Option<From>), Failure> {
    let text = fs::read_to_string(path).map_err(Failure::OpenQuestionFile)?;
    let spec = RelateSpec::parse(&text).map_err(|error| config_error(true, error))?;
    let presence = spec.presence();
    let from = From {
        question: Source::File,
        threshold: file_or_default(presence.threshold),
        model: file_or_default(presence.model),
        field: file_or_default(presence.fields),
        kind_field: file_or_default(presence.fields),
        profile: spec.profile.is_some().then_some(Source::File),
    };
    Ok((spec, Some(from)))
}

/// Line input has no members to point into and one synthetic kind.
fn lines_only(pointed: bool, spec: &RelateSpec) -> Result<(), Failure> {
    if pointed {
        return Err(Failure::Usage(
            "--lines takes neither --field nor --kind-field",
        ));
    }
    if spec
        .relations
        .iter()
        .any(|rule| rule.source != "*" || rule.target != "*")
    {
        return Err(Failure::Usage(
            "--lines takes only bare relation names or NAME=*:*",
        ));
    }
    Ok(())
}

fn config_error(file: bool, error: RelateConfigError) -> Failure {
    Failure::Relate(crate::failure::relate::Error::Config { file, error })
}

const fn file_or_default(file: bool) -> Source {
    if file { Source::File } else { Source::Default }
}
