//! Command-owned file capture and setting-source presentation over native admission.
use crate::args::{
    ChooseArguments, DecideArguments, FilterArguments, ScoreArguments, TagArguments,
};
use crate::failure::Failure;
use crate::plan_document::Sources;
use crate::saved_json::{Json, Kind};
use crate::{AdmittedRequest, BatchSetting, InputDeclaration, QuestionKind, RequestDefinition};

pub(crate) struct FileTier {
    pub(crate) batch: Option<BatchSetting>,
    pub(crate) tuned: bool,
}

/// Only command presentation and setup metadata live here. The admitted native
/// definition and RequestOptions own question resolution and every answer identity.
#[derive(Clone)]
pub(crate) struct Resolved {
    pub(crate) kind: QuestionKind,
    pub(crate) dynamic: bool,
    pub(crate) model: Option<String>,
    pub(crate) on: Vec<String>,
    pub(crate) profile: Option<String>,
    pub(crate) literal_question: bool,
    pub(crate) context_schema: Option<InputDeclaration>,
    pub(crate) sources: Sources,
    pub(crate) saved: bool,
}
impl std::fmt::Debug for Resolved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolvedCommandSettings")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

macro_rules! atomic {
    ($name:ident, $arguments:ty) => {
        pub(crate) fn $name(
            _arguments: &$arguments,
            admitted: &mut AdmittedRequest,
        ) -> Result<(Resolved, FileTier), Failure> {
            prepare(admitted).map(|(resolved, tier, _)| (resolved, tier))
        }
    };
}
atomic!(decide, DecideArguments);
atomic!(filter, FilterArguments);
atomic!(choose, ChooseArguments);
atomic!(tag, TagArguments);
atomic!(score, ScoreArguments);
mod rank;
pub(crate) use rank::rank;

fn strings(value: Option<&Json>) -> Vec<String> {
    match value.map(Json::kind) {
        Some(Kind::String(text)) => vec![text.clone()],
        Some(Kind::Array(values)) => values
            .iter()
            .filter_map(Json::as_str)
            .map(str::to_owned)
            .collect(),
        _ => Vec::new(),
    }
}
fn batch(value: Option<&Json>) -> Result<Option<BatchSetting>, Failure> {
    value
        .map(|value| {
            let text = crate::render::json_line(value)?;
            let batch: crate::RequestBatch = serde_json::from_str(&text)
                .map_err(|_| Failure::Defect("admitted file batch lost its native shape"))?;
            batch.native().map_err(Failure::from)
        })
        .transpose()
}

fn prepare(
    admitted: &mut AdmittedRequest,
) -> Result<(Resolved, FileTier, Option<crate::RankSet>), Failure> {
    let function = admitted.request().call.function();
    let authored = authored(admitted)?;
    let saved = authored.is_some();
    let definition = admitted.resolve_question()?;
    let rank_set = match &definition {
        RequestDefinition::RankSet(set) => Some(set.clone()),
        _ => None,
    };
    let options = &admitted.request().call.arguments().options;
    let top = authored.as_ref();
    let member = if rank_set.is_some() {
        top.and_then(|value| value.member("questions"))
            .and_then(|value| match value.kind() {
                Kind::Object(fields) => fields.first().map(|(_, value)| value),
                _ => None,
            })
    } else {
        top
    };
    let (kind, dynamic, context_schema) = definition_details(&definition, member)?;
    admit_rank_meanings(function, options, rank_set.is_some(), kind)?;
    let key = match kind {
        QuestionKind::Choose => "choose",
        QuestionKind::Tag => "tag",
        QuestionKind::Score => "score",
        _ => "decide",
    };
    let literal_question = member
        .and_then(|value| value.member(key))
        .is_none_or(|value| value.as_str().is_some());
    let model = options.model.clone().or_else(|| {
        top.and_then(|value| value.member("model"))
            .and_then(Json::as_str)
            .map(str::to_owned)
    });
    let on = options
        .field
        .clone()
        .unwrap_or_else(|| strings(member.and_then(|value| value.member("on"))));
    let profile = top
        .and_then(|value| value.member("profile"))
        .and_then(Json::as_str)
        .map(str::to_owned);
    let selected_batch = batch(top.and_then(|value| value.member("batch")))?;
    let tuned = function != crate::RequestFunction::Rank
        && member.is_some_and(|value| value.member("threshold").is_some());
    let mut resolved = Resolved {
        kind,
        dynamic,
        model,
        on,
        profile,
        literal_question,
        context_schema,
        sources: Sources::default(),
        saved,
    };
    resolved.sources = resolved.setting_sources(function, options, member, top, tuned);
    Ok((
        resolved,
        FileTier {
            batch: selected_batch,
            tuned,
        },
        rank_set,
    ))
}

impl Resolved {
    fn setting_sources(
        &self,
        function: crate::RequestFunction,
        options: &crate::RequestOptions,
        member: Option<&Json>,
        top: Option<&Json>,
        tuned: bool,
    ) -> Sources {
        let source = |typed: bool, name: &str| {
            if typed {
                "command line"
            } else if member.is_some_and(|value| value.member(name).is_some()) {
                "file"
            } else {
                "default"
            }
        };
        let mut sources = Sources::default()
            .setting("question", if self.saved { "file" } else { "command line" });
        sources = match self.kind {
            QuestionKind::Choose => sources.setting(
                "options",
                source(options.labels.is_some() || self.dynamic, "options"),
            ),
            QuestionKind::Tag => {
                sources.setting("labels", source(options.labels.is_some(), "labels"))
            }
            QuestionKind::Score => {
                sources.setting("levels", source(options.levels.is_some(), "levels"))
            }
            _ => sources
                .setting("true", source(options.yes.is_some(), "true"))
                .setting("false", source(options.no.is_some(), "false")),
        };
        if self.kind != QuestionKind::Score
            && (function != crate::RequestFunction::Rank || options.threshold.is_some() || tuned)
        {
            sources = sources.setting(
                "threshold",
                source(options.threshold.is_some(), "threshold"),
            );
        }
        sources = sources
            .setting("on", source(options.field.is_some(), "on"))
            .setting(
                "model",
                if options.model.is_some() {
                    "command line"
                } else if top.is_some_and(|value| value.member("model").is_some()) {
                    "file"
                } else {
                    "default"
                },
            );
        sources
    }
}

fn authored(admitted: &mut AdmittedRequest) -> Result<Option<Json>, Failure> {
    let function = admitted.request().call.function();
    let selected = match &admitted.request().call.arguments().question {
        crate::RequestQuestion::Reference { reference } => {
            Some(crate::QuestionFileReference::reference(reference)?)
        }
        crate::RequestQuestion::Name { name } => Some(crate::QuestionFileReference::named(name)?),
        crate::RequestQuestion::File { path } => Some(crate::QuestionFileReference::reference(
            &format!("@{}", path.display()),
        )?),
        _ => None,
    };
    Ok(if let Some(reference) = selected {
        let text = super::question_text::selected_text(&reference, Failure::OpenQuestionFile)?;
        // Native saved-file parsing owns all grammar errors. This tentative
        // host selection chooses only the already explicit set role.
        let document = Json::parse(&text).ok();
        let role = if document
            .as_ref()
            .is_some_and(|value| value.member("questions").is_some())
        {
            crate::QuestionFileRole::Set
        } else {
            crate::QuestionFileRole::Atomic
        };
        let dynamic = function == crate::RequestFunction::Choose
            && admitted
                .request()
                .call
                .arguments()
                .options
                .options_field
                .is_some();
        let definition = reference.parse(&text, role, |text| {
            if function == crate::RequestFunction::Rank
                && matches!(role, crate::QuestionFileRole::Set)
            {
                crate::RankSet::from_json(text).map(RequestDefinition::RankSet)
            } else if dynamic {
                crate::RecordChooseQuestion::from_json(text).map(RequestDefinition::DynamicChoose)
            } else {
                RequestDefinition::from_authored_json(text)
            }
        })?;
        mismatch(function, &definition)?;
        let retained = admitted.clone().with_resolved_definition(definition)?;
        *admitted = retained;
        Some(document.ok_or(Failure::Defect(
            "native admitted file lost its ordered JSON",
        ))?)
    } else {
        None
    })
}

fn mismatch(
    function: crate::RequestFunction,
    definition: &RequestDefinition,
) -> Result<(), Failure> {
    let held = match definition {
        RequestDefinition::Atomic(question) => match question.kind() {
            QuestionKind::Decide => "decide",
            QuestionKind::Choose => "choose",
            QuestionKind::Tag => "tag",
            QuestionKind::Score => "score",
            _ => return Ok(()),
        },
        _ => return Ok(()),
    };
    let caller = function.name();
    let matches = match function {
        crate::RequestFunction::Filter => held == "decide",
        crate::RequestFunction::Rank => held == "decide" || held == "score",
        _ => caller == held,
    };
    if matches {
        return Ok(());
    }
    if matches!(
        function,
        crate::RequestFunction::Filter | crate::RequestFunction::Rank
    ) {
        return Err(Failure::QuestionKind {
            command: caller,
            held,
        });
    }
    Err(Failure::Question {
        message: format!(
            "the command is `{caller}` and the question file holds a `{held}` question"
        ),
        from_file: false,
    })
}

#[cfg(test)]
mod tests;

fn admit_rank_meanings(
    function: crate::RequestFunction,
    options: &crate::RequestOptions,
    rank_set: bool,
    kind: QuestionKind,
) -> Result<(), Failure> {
    if function == crate::RequestFunction::Rank && (options.yes.is_some() || options.no.is_some()) {
        if rank_set {
            return Err(Failure::Usage(
                "`rank` with a question set takes no --true or --false; put meanings in each member",
            ));
        }
        if kind == QuestionKind::Score {
            return Err(Failure::Usage(
                "`rank` with a `score` question file takes no --true or --false",
            ));
        }
    }
    Ok(())
}

fn definition_details(
    definition: &RequestDefinition,
    member: Option<&Json>,
) -> Result<(QuestionKind, bool, Option<InputDeclaration>), Failure> {
    Ok(match definition {
        RequestDefinition::Atomic(question) => {
            (question.kind(), false, question.context_schema().cloned())
        }
        RequestDefinition::Rank(question) => (
            if member.is_some_and(|value| value.member("score").is_some()) {
                QuestionKind::Score
            } else {
                QuestionKind::Decide
            },
            false,
            question.context_schema().cloned(),
        ),
        RequestDefinition::DynamicChoose(question) => (
            QuestionKind::Choose,
            true,
            question.context_schema().cloned(),
        ),
        RequestDefinition::RankSet(_) => {
            let member =
                member.ok_or(Failure::Defect("admitted rank set lost its first member"))?;
            let question = crate::Question::from_json(&crate::render::json_line(member)?)?;
            (
                QuestionKind::Decide,
                false,
                question.context_schema().cloned(),
            )
        }
        _ => {
            return Err(Failure::Defect(
                "atomic preparation resolved another function",
            ));
        }
    })
}
