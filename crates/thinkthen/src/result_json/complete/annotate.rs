//! Ordered successful/failed member identities are finalized before host conversion.
use crate::core::RenderError;
use crate::core::image::InputFunction;
use crate::core::{self, AnnotatedEntry, MemberIdentity, Observation, ResultIdentity};
use crate::engine::facade;
use serde::Serialize;
pub(crate) struct AnnotationRow {
    pub(crate) input: core::Record,
    pub(crate) record: usize,
    pub(crate) context_sha256: Option<String>,
    pub(crate) attempts: bool,
}
#[derive(Serialize)]
struct Scope<'a> {
    record: usize,
    member: &'a str,
    position: usize,
}

pub(crate) fn annotation(
    engine: &facade::Engine,
    set: &core::QuestionSet,
    annotation: facade::Annotation,
    row: AnnotationRow,
) -> Result<core::CompleteAnnotation, RenderError> {
    let AnnotationRow {
        input,
        record: at,
        context_sha256,
        attempts,
    } = row;
    if annotation.details.len() != annotation.receipts.len()
        || annotation.details.len() != set.questions().len()
    {
        return Err(RenderError);
    }
    let mut trace = core::LogicalTrace::default();
    let mut members = Vec::new();
    let mut children = Vec::new();
    let mut events = std::collections::BTreeMap::new();
    for (position, ((name, entry), receipt)) in annotation
        .details
        .iter()
        .zip(&annotation.receipts)
        .enumerate()
    {
        let named = set.questions().get(position).ok_or(RenderError)?;
        trace.take(
            "member",
            named.question(),
            &receipt.trace.sources,
            &receipt.trace.observations,
        );
        for event in &receipt.trace.attempts {
            events.insert(event.ordinal(), event.clone());
        }
        let identity = member_identity(named, &receipt.trace, name, at, position, entry)?;
        if let MemberIdentity::Answered(id) = &identity {
            children.push(id.clone());
        }
        members.push((
            name.clone(),
            core::CompleteAnnotationMember {
                declarations: named.metadata().clone(),
                identity,
                legacy: entry.clone(),
                threshold: named.threshold(),
                sources: receipt.trace.sources.clone(),
                observations: receipt.trace.observations.clone(),
                reported_usage: receipt.trace.reply.reported_usage(),
            },
        ));
    }
    let digest = set.sha256().map_err(|_| RenderError)?;
    let identity = trace
        .identity(
            InputFunction::Annotate,
            &core::RecordScope { record: at },
            &digest,
            &children,
        )
        .map_err(|_| RenderError)?;
    let meta = core::AnnotateMeta::new(
        env!("CARGO_PKG_VERSION"),
        digest,
        engine.backend().url().clone(),
        annotation
            .model
            .unwrap_or_else(|| engine.backend().model().clone()),
        annotation.usage,
        core::RequestMeta::new(
            annotation.replayed,
            annotation.requests_sent,
            annotation.requests,
        )
        .with_failed_questions(annotation.failed_questions)
        .with_profile_warning(core::ProfileWarning::between(
            set.profile(),
            engine.profile().map(core::BackendProfile::name),
        )),
    )
    .with_reported_usage(annotation.reported_usage);
    Ok(core::CompleteAnnotation {
        identity,
        legacy: core::AnnotateResult::new(input, annotation.values, annotation.details, meta),
        members,
        context_sha256,
        attempts: attempts.then(|| events.into_values().collect()),
    })
}

fn member_identity(
    named: &core::NamedQuestion,
    receipt: &facade::Answered,
    name: &str,
    at: usize,
    position: usize,
    entry: &AnnotatedEntry,
) -> Result<MemberIdentity, RenderError> {
    Ok(match entry {
        AnnotatedEntry::Answered(_) => {
            let identity = ResultIdentity::of(
                InputFunction::Annotate,
                &Scope {
                    record: at,
                    member: name,
                    position,
                },
                receipt.sources.clone(),
                receipt.observations.clone(),
                &core::AtomicReading {
                    question: named.question(),
                    threshold: named.threshold(),
                    rank_position: None,
                },
                &[],
            )
            .map_err(|_| RenderError)?;
            MemberIdentity::Answered(identity.answer_id().clone())
        }
        AnnotatedEntry::Failed(_) => MemberIdentity::Failed(
            receipt
                .observations
                .iter()
                .find_map(|observation| match observation {
                    Observation::Failed { failure_id } => Some(failure_id.clone()),
                    Observation::Answered { .. } => None,
                })
                .ok_or(RenderError)?,
        ),
    })
}
