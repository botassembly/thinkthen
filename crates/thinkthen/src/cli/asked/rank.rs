//! Single-read rank dispatch; failed parsing never retries another grammar.
use super::{FileTier, fields, path_of, typed_text};
use crate::args::RankArguments;
use crate::cli::question_text;
use crate::core::{
    Cutting, Json, QuestionFile, QuestionFileError, QuestionSet, Resolved, Typed, Verb, json_line,
    resolve,
};
use crate::failure::Failure;
use std::path::Path;

/// Settle the ordinary yes/no rank or a saved graded score question.
pub(crate) fn rank(
    arguments: &RankArguments,
) -> Result<(Resolved, FileTier, Option<crate::core::QuestionSet>), Failure> {
    let (file, batch, set) = read(&arguments.question)?;
    if set.is_some() && (arguments.meanings.yes.is_some() || arguments.meanings.no.is_some()) {
        return Err(Failure::Usage(
            "`rank` with a question set takes no --true or --false; put meanings in each member",
        ));
    }
    let graded = file.as_ref().is_some_and(|held| held.verb() == Verb::Score);
    if graded && (arguments.meanings.yes.is_some() || arguments.meanings.no.is_some()) {
        return Err(Failure::Usage(
            "`rank` with a `score` question file takes no --true or --false",
        ));
    }
    let typed = Typed {
        threshold: arguments.threshold.clone(),
        yes: arguments.meanings.yes.clone(),
        no: arguments.meanings.no.clone(),
        model: arguments.common.model.clone(),
        on: fields(&arguments.common),
        cutting: if graded || set.is_some() {
            Cutting::NoRule
        } else {
            Cutting::RankCut
        },
        ..Typed::default()
    };
    let verb = if graded { Verb::Score } else { Verb::Decide };
    let resolved = resolve(
        verb,
        typed_text(&arguments.question, file.is_some()),
        file.as_ref(),
        &typed,
    )
    .map_err(|error| match error {
        crate::core::QuestionFileError::VerbMismatch { held, .. } => Failure::QuestionKind {
            command: "rank",
            held: held.word(),
        },
        other => Failure::Question(other),
    })?;
    Ok((
        resolved,
        FileTier {
            batch,
            tuned: false,
        },
        set,
    ))
}

type ReadRank = (Option<QuestionFile>, Option<Json>, Option<QuestionSet>);
fn read(question: &str) -> Result<ReadRank, Failure> {
    let Some(path) = path_of(question) else {
        return Ok((None, None, None));
    };
    let text = question_text::reference(Path::new(path), Failure::OpenQuestionFile)?;
    let value = Json::parse(&text).map_err(QuestionFileError::from)?;
    if value.member("questions").is_none() {
        let (file, batch) = QuestionFile::parse_top(&text)?;
        return Ok((Some(file), batch, None));
    }
    let set = QuestionSet::parse_rank(&text)?;
    let Some(Json::Object(members)) = value.member("questions") else {
        return Err(Failure::Defect("an admitted set has no members"));
    };
    let Some((_, Json::Object(fields))) = members.first() else {
        return Err(Failure::Defect("an admitted set has no first question"));
    };
    let mut fields = fields.clone();
    if let Some(profile) = value.member("profile") {
        fields.push(("profile".to_owned(), profile.clone()));
    }
    let file = QuestionFile::parse(&json_line(&Json::Object(fields))?)?;
    Ok((Some(file), set.batch().cloned(), Some(set)))
}
