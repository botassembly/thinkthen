//! Command relation selection and source notices over native saved grammar.
use crate::args::RelateArguments;
use crate::failure::Failure;
use crate::saved_json::Json;
use crate::{AdmittedRequest, Relate, RequestDefinition, RequestFraming as Framing};
use serde::Serialize;

pub(super) struct Settled {
    pub(super) definition: RequestDefinition,
    pub(super) framing: Framing,
    pub(super) fields: Fields,
    pub(super) model: Option<String>,
    pub(super) threshold: Option<crate::RequestThreshold>,
    pub(super) profile: Option<String>,
    pub(super) from: Option<From>,
}
#[derive(Serialize)]
pub(super) struct Fields {
    pub(super) name: String,
    pub(super) kind: String,
}
#[derive(Clone, Copy, Serialize)]
pub(super) struct From {
    question: &'static str,
    threshold: &'static str,
    model: &'static str,
    field: &'static str,
    kind_field: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<&'static str>,
}

pub(super) fn settle(
    arguments: &RelateArguments,
    admitted: &mut AdmittedRequest,
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
    let authored = if let Some(file) = file {
        let reference = crate::QuestionFileReference::reference(&format!("@{file}"))?;
        let text = crate::cli::question_text::selected_text(&reference, Failure::OpenQuestionFile)?;
        let relation = reference
            .parse(
                &text,
                crate::QuestionFileRole::Relate,
                Relate::from_records_json,
            )
            .map_err(|error| {
                // Reference admission reserves Usage for an unambiguous role
                // mismatch; saved parser/name/content failures are Local.
                if error.kind() == crate::ErrorKind::Usage {
                    Failure::Relate(crate::failure::relate::Error::Config {
                        file: true,
                        error: "the question file holds another command's question".to_owned(),
                    })
                } else {
                    Failure::from(error)
                }
            })?;
        *admitted = admitted.clone().with_resolved_definition(relation.into())?;
        Some(Json::parse(&text)?)
    } else {
        None
    };
    let mut definition = admitted.resolve_question()?;
    let RequestDefinition::Relate(_) = &definition else {
        return Err(Failure::Defect("relate lost its definition"));
    };
    let threshold = arguments
        .threshold
        .as_deref()
        .map(|text| {
            text.parse::<crate::Threshold>()
                .ok()
                .and_then(crate::Threshold::cut_value)
                .map(crate::RequestThreshold::Cut)
                .ok_or(Failure::Usage(
                    "the relate threshold is one cut above zero and at most one",
                ))
        })
        .transpose()?;
    let document = Json::parse(&crate::render::json_line(&definition)?)?;
    let saved_fields = document
        .member("relate")
        .and_then(|value| value.member("fields"));
    let field = |name, default: &str| {
        saved_fields
            .and_then(|fields| fields.member(name))
            .and_then(Json::as_str)
            .unwrap_or(default)
            .to_owned()
    };
    let fields = Fields {
        name: arguments
            .common
            .field
            .first()
            .cloned()
            .unwrap_or_else(|| field("name", "/name")),
        kind: arguments
            .kind_field
            .clone()
            .unwrap_or_else(|| field("kind", "/kind")),
    };
    // The native authored parser owns override validation and its relation-specific diagnostics.
    if file.is_some() {
        if let RequestDefinition::Relate(relation) = &mut definition {
            *relation = relation.clone().record_fields(&fields.name, &fields.kind)?;
        }
    }
    let framing = if arguments.common.input.len() > 1
        && arguments.common.unit.is_none()
        && matches!(arguments.common.framing(), Framing::Document)
    {
        Framing::Lines
    } else {
        arguments.common.framing()
    };
    if matches!(framing, Framing::Lines)
        && (!arguments.common.field.is_empty() || arguments.kind_field.is_some())
    {
        return Err(Failure::Usage(
            "--lines takes neither --field nor --kind-field",
        ));
    }
    let top = authored.as_ref();
    let source = |supplied, name| {
        if supplied {
            "command line"
        } else if top.is_some_and(|value| value.member(name).is_some()) {
            "file"
        } else {
            "default"
        }
    };
    let authored_fields = top
        .and_then(|value| value.member("relate"))
        .and_then(|value| value.member("fields"))
        .is_some();
    let from = file.map(|_| From {
        question: "file",
        threshold: source(arguments.threshold.is_some(), "threshold"),
        model: source(arguments.common.model.is_some(), "model"),
        field: if !arguments.common.field.is_empty() {
            "command line"
        } else if authored_fields {
            "file"
        } else {
            "default"
        },
        kind_field: if arguments.kind_field.is_some() {
            "command line"
        } else if authored_fields {
            "file"
        } else {
            "default"
        },
        profile: top
            .is_some_and(|value| value.member("profile").is_some())
            .then_some("file"),
    });
    let model = arguments.common.model.clone().or_else(|| {
        document
            .member("model")
            .and_then(Json::as_str)
            .map(str::to_owned)
    });
    let profile = document
        .member("profile")
        .and_then(Json::as_str)
        .map(str::to_owned);
    Ok(Settled {
        definition,
        framing,
        fields,
        model,
        threshold,
        profile,
        from,
    })
}
