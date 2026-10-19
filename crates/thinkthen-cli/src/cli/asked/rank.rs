//! Rank captures command metadata while the retained Request owns native admission.
use super::{FileTier, Resolved};
use crate::args::RankArguments;
use crate::failure::Failure;

pub(crate) fn rank(_arguments: &RankArguments, admitted: &mut crate::AdmittedRequest)
    -> Result<(Resolved, FileTier, Option<crate::RankSet>), Failure> {
    super::prepare(admitted)
}
