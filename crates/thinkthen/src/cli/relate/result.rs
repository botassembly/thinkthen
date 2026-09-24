use std::io::{ErrorKind, Write};

use serde::Serialize;
use sha2::{Digest as _, Sha256};

use crate::core::{
    Answer, AnswerOutcome, Backend, Framing, Meta, ModelName, ProfileWarning, QuestionMap,
    RelateFields, RelateSpec, RelationEdge, RelationEntity, RelationEntityView, RequestMeta, Usage,
    json_line,
};
use crate::failure::Failure;

#[rustfmt::skip]
pub(super) struct Logical {
    pub(super) relation: crate::core::RelationRule, pub(super) mapping: QuestionMap,
    pub(super) outcome: AnswerOutcome, pub(super) request: String,
}

#[derive(Default)]
#[rustfmt::skip]
pub(super) struct Execution {
    pub(super) edges: Vec<RelationEdge<RelationEntity>>, pub(super) logical: Vec<Logical>,
    pub(super) model: Option<ModelName>, pub(super) usage: Option<Usage>, pub(super) replayed: bool,
    pub(super) requests_sent: u64, pub(super) requests: Vec<String>,
    pub(super) failed: usize, pub(super) answered: usize,
}

#[derive(Clone, Copy, Serialize)]
#[rustfmt::skip]
struct QuestionView<'a> {
    verb: &'static str, fields: Option<&'a RelateFields>, relations: &'a [crate::core::RelationRule],
    threshold: crate::core::Threshold,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<&'a crate::core::ProfileName>,
}

#[derive(Serialize)]
#[rustfmt::skip]
struct Endpoint {
    name: String, kind: String,
}

#[rustfmt::skip]
pub(super) struct Output<'a> {
    pub(super) details: bool, pub(super) framing: Framing, pub(super) spec: &'a RelateSpec,
    pub(super) entities: &'a [RelationEntity], pub(super) backend: &'a Backend,
    pub(super) warning: Option<ProfileWarning>, pub(super) execution: Execution,
}

#[rustfmt::skip]
pub(super) fn write(writer: &mut dyn Write, output: Output<'_>) -> Result<(), Failure> {
    let Output { details, framing, spec, entities, backend, warning, execution } = output;
    let output = if details {
        details_line(framing, spec, entities, backend, warning, &execution)? + "\n"
    } else {
        execution
            .edges
            .iter()
            .map(json_line)
            .collect::<Result<Vec<_>, _>>()?
            .join("\n")
            + if execution.edges.is_empty() { "" } else { "\n" }
    };
    write_buffer(writer, output.as_bytes())
}

#[rustfmt::skip]
fn details_line(
    framing: Framing, spec: &RelateSpec, entities: &[RelationEntity], backend: &Backend,
    warning: Option<ProfileWarning>, execution: &Execution,
) -> Result<String, Failure> {
    let question = QuestionView {
        verb: "relate",
        fields: (framing != Framing::Lines).then_some(spec.fields()),
        relations: &spec.relations,
        threshold: spec.threshold,
        profile: spec.profile.as_ref(),
    };
    let question_json = json_line(&question)?;
    let mut hasher = Sha256::new();
    hasher.update(question_json.as_bytes());
    let digest = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let model = execution
        .model
        .clone()
        .unwrap_or_else(|| backend.model().clone());
    let meta = Meta::new(
        env!("CARGO_PKG_VERSION"),
        digest,
        backend.url().clone(),
        model,
        execution.usage,
        RequestMeta::new(
            execution.replayed,
            execution.requests_sent,
            execution.requests.clone(),
        )
        .with_failed_questions(execution.failed)
        .with_profile_warning(warning),
    );
    let entries = execution
        .logical
        .iter()
        .map(|logical| entry(logical, entities, spec.threshold.cut_value().unwrap_or(0.5)))
        .collect::<Result<Vec<_>, _>>()?
        .join(",");
    let meta = json_line(&meta)?.replace(
        "\"failed_questions\":0",
        &format!("\"failed_questions\":{}", execution.failed),
    );
    Ok(format!(
        "{{\"schema\":\"{}\",\"value\":{},\"question\":{},\"answer\":{{\"questions\":[{}]}},\"meta\":{}}}",
        crate::core::RESULT_SCHEMA,
        json_line(&execution.edges)?,
        question_json,
        entries,
        meta
    ))
}

fn entry(
    logical: &Logical,
    entities: &[RelationEntity],
    threshold: f64,
) -> Result<String, Failure> {
    match (&logical.mapping, &logical.outcome) {
        (QuestionMap::Choice { asker, options }, AnswerOutcome::Answered(answer)) => {
            choice(logical, entities, *asker, options, answer, threshold)
        }
        (QuestionMap::Choice { asker, options }, AnswerOutcome::Failed(failure)) => Ok(format!(
            "{}{},\"failure\":{},\"request\":{}}}",
            choice_prefix(logical, entities, *asker, options)?,
            failed_candidates(entities, *asker, options)?,
            json_line(failure)?,
            json_line(&logical.request)?
        )),
        (QuestionMap::Pair { source, target }, AnswerOutcome::Answered(answer)) => {
            let probability = answer
                .yes()
                .ok_or(Failure::Defect("a relation H answer has the wrong kind"))?;
            Ok(format!(
                "{}\"probability\":{probability},\"accepted\":{},\"request\":{}}}",
                pair_prefix(logical, entities, *source, *target)?,
                probability >= threshold,
                json_line(&logical.request)?
            ))
        }
        (QuestionMap::Pair { source, target }, AnswerOutcome::Failed(failure)) => Ok(format!(
            "{}\"failure\":{},\"request\":{}}}",
            pair_prefix(logical, entities, *source, *target)?,
            json_line(failure)?,
            json_line(&logical.request)?
        )),
    }
}

fn choice(
    logical: &Logical,
    entities: &[RelationEntity],
    asker: usize,
    options: &[(String, usize, usize)],
    answer: &Answer,
    threshold: f64,
) -> Result<String, Failure> {
    let probabilities = answer.choice_probabilities().ok_or(Failure::Defect(
        "a relation choice answer has the wrong kind",
    ))?;
    let (_, role, _) = choice_roles(asker, options)?;
    let mut candidates = Vec::new();
    for (label, source, target) in options {
        let probability = probability(&probabilities, label)?;
        let place = if role == "target" { *target } else { *source };
        candidates.push(format!(
            "{{\"role\":{role:?},\"entity\":{},\"probability\":{probability},\"accepted\":{}}}",
            json_line(&endpoint(entities, place)?)?,
            probability >= threshold
        ));
    }
    let none = probability(&probabilities, "none")?;
    candidates.push(format!(
        "{{\"none\":true,\"probability\":{none},\"accepted\":false}}"
    ));
    Ok(format!(
        "{}{}],\"pick\":{},\"request\":{}}}",
        choice_prefix(logical, entities, asker, options)?,
        candidates.join(","),
        pick_json(&probabilities, options, entities, role)?,
        json_line(&logical.request)?
    ))
}

#[rustfmt::skip]
fn choice_prefix(
    logical: &Logical, entities: &[RelationEntity], asker: usize, options: &[(String, usize, usize)],
) -> Result<String, Failure> {
    let (role, _, asker) = choice_roles(asker, options)?;
    Ok(format!(
        "{{\"relation\":{},\"reads\":{},\"method\":\"choice\",\"direction\":{},\"asker\":{{\"role\":{role:?},\"entity\":{}}},\"candidates\":[",
        json_line(&logical.relation.name)?,
        json_line(&logical.relation.reads)?,
        json_line(&direction(logical))?,
        json_line(&endpoint(entities, asker)?)?
    ))
}

#[rustfmt::skip]
fn failed_candidates(
    entities: &[RelationEntity], asker: usize, options: &[(String, usize, usize)],
) -> Result<String, Failure> {
    let (_, role, _) = choice_roles(asker, options)?;
    let mut candidates = options
        .iter()
        .map(|(_, source, target)| {
            let place = if role == "target" { *target } else { *source };
            Ok(format!(
                "{{\"role\":{role:?},\"entity\":{}}}",
                json_line(&endpoint(entities, place)?)?
            ))
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    candidates.push("{\"none\":true}".to_owned());
    Ok(candidates.join(",") + "]")
}

#[rustfmt::skip]
fn pair_prefix(
    logical: &Logical, entities: &[RelationEntity], source: usize, target: usize,
) -> Result<String, Failure> {
    Ok(format!(
        "{{\"relation\":{},\"reads\":{},\"method\":\"yes_no\",\"direction\":{},\"source\":{},\"target\":{},",
        json_line(&logical.relation.name)?,
        json_line(&logical.relation.reads)?,
        json_line(&direction(logical))?,
        json_line(&endpoint(entities, source)?)?,
        json_line(&endpoint(entities, target)?)?
    ))
}

#[rustfmt::skip]
fn choice_roles(asker: usize, options: &[(String, usize, usize)]) -> Result<(&'static str, &'static str, usize), Failure> {
    let (_, source, _) = options.first().ok_or(Failure::Defect("a relation choice maps no candidates"))?;
    if *source == asker { Ok(("source", "target", asker)) } else { Ok(("target", "source", asker)) }
}

#[rustfmt::skip]
fn probability(probabilities: &[(&str, f64)], label: &str) -> Result<f64, Failure> {
    probabilities.iter().find_map(|(held, value)| (*held == label).then_some(*value))
        .ok_or(Failure::Defect("a relation choice lost one probability"))
}

#[rustfmt::skip]
fn pick_json(
    probabilities: &[(&str, f64)], options: &[(String, usize, usize)],
    entities: &[RelationEntity], role: &str,
) -> Result<String, Failure> {
    let (label, _) = probabilities
        .iter()
        .copied()
        .reduce(|best, item| if item.1 > best.1 { item } else { best })
        .ok_or(Failure::Defect("a relation choice has no probabilities"))?;
    if label == "none" {
        return Ok("{\"none\":true}".to_owned());
    }
    let (_, source, target) = options
        .iter()
        .find(|(held, _, _)| held == label)
        .ok_or(Failure::Defect("a relation pick maps no entity"))?;
    let place = if role == "target" { *target } else { *source };
    Ok(format!(
        "{{\"role\":{role:?},\"entity\":{}}}",
        json_line(&endpoint(entities, place)?)?
    ))
}

#[rustfmt::skip]
fn endpoint(entities: &[RelationEntity], place: usize) -> Result<Endpoint, Failure> {
    let entity = entities.get(place).ok_or(Failure::Defect("a relation mapping names no entity"))?;
    Ok(Endpoint {
        name: entity.name().to_owned(), kind: entity.kind().to_owned(),
    })
}

#[rustfmt::skip]
fn direction(logical: &Logical) -> &'static str {
    if logical.relation.either { "either" } else { "source_to_target" }
}

#[rustfmt::skip]
fn write_buffer(writer: &mut dyn Write, bytes: &[u8]) -> Result<(), Failure> {
    match writer.write_all(bytes).and_then(|()| writer.flush()) {
        Err(error) if error.kind() == ErrorKind::BrokenPipe => Ok(()), Err(error) => Err(Failure::Output(error)), Ok(()) => Ok(()),
    }
}
