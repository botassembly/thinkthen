//! Native atomic preparation retains the CLI's expected verb and typed overrides.
use crate::core::{
    Cutting, Json, QuestionFile, QuestionFileError, QuestionSet, Resolved, Typed, Verb, json_line,
    resolve,
};
use crate::{Error, RequestDefinition};

#[derive(Clone, Debug)]
pub(crate) struct Prepared {
    pub(crate) resolved: Resolved,
    pub(crate) batch: Option<Json>,
    pub(crate) tuned: bool,
    pub(crate) rank_set: Option<crate::core::QuestionSet>,
}

pub(crate) fn inline(verb: Verb, text: &str, typed: &Typed) -> Result<Prepared, Error> {
    Ok(Prepared {
        resolved: resolve(verb, Some(text), None, typed).map_err(Error::refused)?,
        batch: None,
        tuned: false,
        rank_set: None,
    })
}
pub(super) fn saved(verb: Verb, text: &str, typed: &Typed) -> Result<Prepared, Error> {
    let (file, batch) = QuestionFile::parse_top(text).map_err(Error::refused)?;
    Ok(Prepared {
        resolved: resolve(verb, None, Some(&file), typed).map_err(Error::refused)?,
        batch,
        tuned: file.has_threshold(),
        rank_set: None,
    })
}
pub(crate) fn definition(prepared: &Prepared) -> Result<RequestDefinition, Error> {
    if let Some(set) = &prepared.rank_set {
        return Ok(crate::RankSet(set.clone()).into());
    }
    let resolved = &prepared.resolved;
    let Some(core) = resolved.question().cloned() else {
        return Ok(crate::RecordChooseQuestion::from_cli_resolved(
            resolved,
            prepared.batch.clone(),
            prepared.tuned,
        )
        .into());
    };
    let threshold = resolved.threshold();
    let kind = match &core {
        crate::core::Question::Decide { .. } if threshold.is_some_and(|v| !v.is_cut()) => {
            crate::public::NativeQuestionKind::Banded
        }
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

pub(crate) fn rank(text: &str, saved: bool, typed: &Typed) -> Result<Prepared, Error> {
    let (file, batch, set) = read_rank(text, saved)?;
    if set.is_some() && (typed.yes.is_some() || typed.no.is_some()) {
        return Err(Error::usage(
            "`rank` with a question set takes no --true or --false; put meanings in each member",
        ));
    }
    let graded = file.as_ref().is_some_and(|held| held.verb() == Verb::Score);
    if graded && (typed.yes.is_some() || typed.no.is_some()) {
        return Err(Error::usage(
            "`rank` with a `score` question file takes no --true or --false",
        ));
    }
    let mut typed = typed.clone();
    typed.cutting = if graded || set.is_some() {
        Cutting::NoRule
    } else {
        Cutting::RankCut
    };
    let verb = if graded { Verb::Score } else { Verb::Decide };
    let resolved =
        resolve(verb, (!saved).then_some(text), file.as_ref(), &typed).map_err(Error::refused)?;
    Ok(Prepared {
        resolved,
        batch,
        tuned: false,
        rank_set: set,
    })
}

type ReadRank = (Option<QuestionFile>, Option<Json>, Option<QuestionSet>);
fn read_rank(text: &str, saved: bool) -> Result<ReadRank, Error> {
    if !saved {
        return Ok((None, None, None));
    }
    let value = Json::parse(text)
        .map_err(QuestionFileError::from)
        .map_err(Error::refused)?;
    if value.member("questions").is_none() {
        let (file, batch) = QuestionFile::parse_top(text).map_err(Error::refused)?;
        return Ok((Some(file), batch, None));
    }
    let set = QuestionSet::parse_rank(text).map_err(Error::refused)?;
    let Some(Json::Object(members)) = value.member("questions") else {
        return Err(Error::defect("an admitted set has no members"));
    };
    let Some((_, Json::Object(fields))) = members.first() else {
        return Err(Error::defect("an admitted set has no first question"));
    };
    let mut fields = fields.clone();
    if let Some(profile) = value.member("profile") {
        fields.push(("profile".to_owned(), profile.clone()));
    }
    let file = QuestionFile::parse(&json_line(&Json::Object(fields)).map_err(Error::refused)?)
        .map_err(Error::refused)?;
    Ok((Some(file), set.batch().cloned(), Some(set)))
}
