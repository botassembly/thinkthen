use serde::Serialize;

use crate::args::RelateArguments;
use crate::core::{Framing, RelateConfigError, RelateSpec, Source};
use crate::edge;
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

pub(super) fn settle(
    arguments: &RelateArguments,
    admitted: &mut crate::AdmittedRequest,
) -> Result<Settled, Failure> {
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
    if file.is_some() && arguments.either {
        return Err(Failure::Usage(
            "`--either` applies only to inline relation rules; a question file sets either on each relation",
        ));
    }
    let crate::RequestDefinition::Relate(ask) = admitted.resolve_once().map_err(Failure::from)?
    else {
        return Err(Failure::Defect("relate lost its definition"));
    };
    let mut spec = ask.0;
    let mut from = file.map(|_| from_file(&spec));
    if file.is_some()
        && let Some(threshold) = arguments.threshold.as_deref()
    {
        spec.override_threshold(threshold)
            .map_err(|error| config_error(false, error))?;
        if let Some(sources) = &mut from {
            sources.threshold = Source::CommandLine;
        }
    }
    let name = arguments.common.field.first().map(String::as_str);
    if file.is_some() {
        spec.override_fields(name, arguments.kind_field.as_deref())
            .map_err(|error| config_error(false, error))?;
    }
    if let Some(sources) = &mut from {
        if name.is_some() {
            sources.field = Source::CommandLine;
        }
        if arguments.kind_field.is_some() {
            sources.kind_field = Source::CommandLine;
        }
    }
    if let Some(model) = arguments.common.model.as_deref() {
        spec.model = Some(edge::model_flag(model)?);
        if let Some(sources) = &mut from {
            sources.model = Source::CommandLine;
        }
    }
    let framing = if arguments.common.input.len() > 1
        && arguments.common.unit.is_none()
        && arguments.common.framing() == Framing::Document
    {
        Framing::Lines
    } else {
        arguments.common.framing()
    };
    if framing == Framing::Lines {
        lines_only(name.is_some() || arguments.kind_field.is_some())?;
    }
    Ok(Settled {
        spec,
        framing,
        from,
    })
}

/// Read the question file and name which values it supplied.
fn from_file(spec: &RelateSpec) -> From {
    let presence = spec.presence();
    From {
        question: Source::File,
        threshold: file_or_default(presence.threshold),
        model: file_or_default(presence.model),
        field: file_or_default(presence.fields),
        kind_field: file_or_default(presence.fields),
        profile: spec.profile.is_some().then_some(Source::File),
    }
}

/// Line input has no members to point into and one synthetic kind.
fn lines_only(pointed: bool) -> Result<(), Failure> {
    if pointed {
        return Err(Failure::Usage(
            "--lines takes neither --field nor --kind-field",
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
