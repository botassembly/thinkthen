//! Rank flags resolve through retained native question preparation.
use super::{FileTier, fields};
use crate::args::RankArguments;
use crate::core::{Resolved, Typed};
use crate::failure::Failure;

pub(crate) fn rank(
    arguments: &RankArguments,
    admitted: Option<&mut crate::AdmittedRequest>,
) -> Result<(Resolved, FileTier, Option<crate::core::QuestionSet>), Failure> {
    let typed = Typed {
        threshold: arguments.threshold.clone(),
        yes: arguments.meanings.yes.clone(),
        no: arguments.meanings.no.clone(),
        model: arguments.common.model.clone(),
        on: fields(&arguments.common),
        ..Typed::default()
    };
    let prepared = match admitted {
        Some(admitted) => admitted.resolve_cli_rank(&typed),
        None => crate::cli_atomic::rank(&arguments.question, false, &typed),
    }
    .map_err(Failure::from)?;
    Ok((
        prepared.resolved,
        FileTier {
            batch: prepared.batch,
            tuned: prepared.tuned,
        },
        prepared.rank_set,
    ))
}
