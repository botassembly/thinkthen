//! Native atomic preparation retains the CLI's expected verb and typed overrides.
use crate::core::{Json, QuestionFile, Resolved, Typed, Verb, resolve};
use crate::{Error, RequestDefinition};

#[derive(Clone, Debug)]
pub(crate) struct Prepared {
    pub(crate) resolved: Resolved,
    pub(crate) batch: Option<Json>,
    pub(crate) tuned: bool,
}

pub(crate) fn inline(verb: Verb, text: &str, typed: &Typed) -> Result<Prepared, Error> {
    Ok(Prepared {
        resolved: resolve(verb, Some(text), None, typed).map_err(Error::refused)?,
        batch: None,
        tuned: false,
    })
}
pub(super) fn saved(verb: Verb, text: &str, typed: &Typed) -> Result<Prepared, Error> {
    let (file, batch) = QuestionFile::parse_top(text).map_err(Error::refused)?;
    Ok(Prepared {
        resolved: resolve(verb, None, Some(&file), typed).map_err(Error::refused)?,
        batch,
        tuned: file.has_threshold(),
    })
}
pub(crate) fn definition(prepared: &Prepared) -> Result<RequestDefinition, Error> {
    let resolved = &prepared.resolved;
    let Some(core) = resolved.question().cloned() else {
        return Ok(crate::RecordChooseQuestion::from_cli_resolved(resolved, prepared.batch.clone(), prepared.tuned).into());
    };
    let threshold = resolved.threshold();
    let kind = match &core {
        crate::core::Question::Decide { .. } if threshold.is_some_and(|v| !v.is_cut()) => crate::public::NativeQuestionKind::Banded,
        crate::core::Question::Decide { .. } => crate::public::NativeQuestionKind::Decide,
        crate::core::Question::Choose { .. } => crate::public::NativeQuestionKind::Choose,
        crate::core::Question::Tag { .. } => crate::public::NativeQuestionKind::Tag,
        crate::core::Question::Score { .. } => crate::public::NativeQuestionKind::Score,
    };
    let q = crate::Question {
        metadata: resolved.metadata().clone(),
        core,
        threshold,
        authored_threshold: prepared.tuned,
        model: (!resolved.sources().model_is_default()).then(|| resolved.model().clone()),
        profile: resolved.profile().cloned(),
        batch: prepared.batch.clone(),
        kind,
    };
    Ok(if kind == crate::public::NativeQuestionKind::Banded {
        crate::LoadedQuestion::Banded(crate::BandedQuestion(q)).into()
    } else {
        q.into()
    })
}
