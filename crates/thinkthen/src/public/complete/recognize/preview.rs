//! Route-only recognition preparation shares record admission with execution.
use crate::engine::facade;
use crate::public::{Error, Recognize, RecordInput};
pub(crate) struct Preview {
    pub(crate) summary: crate::core::PlanSummary,
    pub(crate) first: Option<First>,
}
pub(crate) struct First {
    pub(crate) pieces: usize,
    pub(crate) requests: Vec<facade::Request>,
    pub(crate) names: usize,
    pub(crate) relations: Option<usize>,
}
pub(in crate::public) fn prepare(
    backend: &crate::core::Backend,
    profile: Option<&crate::core::BackendProfile>,
    ask: &Recognize,
    records: impl Iterator<Item = Result<RecordInput<crate::QuestionInput>, Error>>,
    fallback: Option<&str>,
    limit: usize,
) -> Result<Preview, Error> {
    let mut summary = crate::core::PlanSummary::new(true).with_accounting(backend.accounting());
    let mut first = None;
    for (at, record) in records.enumerate() {
        let (_, _, text, _, context, ask) = super::records::prepare_input(ask, record?, fallback)
            .map_err(|error| error.at_record(at))?;
        let context = context.as_ref().map(crate::core::Evidence::as_json);
        let (pieces, _, requests) =
            facade::step_one_context(backend, profile, &ask.0, &text, limit, context.as_ref())
                .map_err(Error::from)?;
        let large = || Error::defect("a plan is too large");
        summary.record().map_err(|_| large())?;
        for request in &requests {
            summary.request(&request.body).map_err(|_| large())?;
        }
        let names = name_upper_bound(&ask.0, pieces.len())?;
        summary.possible_requests(names).map_err(|_| large())?;
        let relations =
            relation_upper_bound(&ask.0, pieces.len().saturating_add(ask.0.seed_spans.len()));
        if let Some(bound) = relations {
            summary.possible_requests(bound).map_err(|_| large())?;
        }
        if first.is_none() {
            first = Some(First {
                pieces: pieces.len(),
                requests,
                names,
                relations,
            });
        }
    }
    Ok(Preview { summary, first })
}
fn name_upper_bound(spec: &crate::core::RecognizeSpec, pieces: usize) -> Result<usize, Error> {
    if !spec.mode.is_whole() {
        return Ok(0);
    }
    let decoded = pieces.checked_mul(if spec.kinds.is_empty() { 1 } else { 2 });
    decoded
        .and_then(|count| {
            spec.seed_spans
                .len()
                .checked_mul(2)
                .and_then(|seeds| count.checked_add(seeds))
        })
        .ok_or(Error::defect("a plan is too large"))
}

fn relation_upper_bound(spec: &crate::core::RecognizeSpec, tokens: usize) -> Option<usize> {
    if !spec.mode.is_whole() {
        return Some(0);
    }
    (!spec.relations.is_empty()).then(|| {
        let directed = tokens.saturating_mul(tokens.saturating_sub(1));
        spec.relations.iter().fold(0_usize, |total, rule| {
            total.saturating_add(if rule.either { directed / 2 } else { directed })
        })
    })
}
