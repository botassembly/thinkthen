//! Route-only relation plans share whole-set admission with execution.
use crate::engine::facade;
use crate::public::{CallOptions, Error, RecordInput, Relate};
pub(crate) struct Preview {
    pub(crate) summary: crate::core::PlanSummary,
    pub(crate) prepared: facade::PreparedRelations,
    pub(crate) requests: Vec<facade::Request>,
    pub(crate) requests_per_rule: Vec<usize>,
    pub(crate) entities: usize,
}
pub(in crate::public) fn prepare(
    backend: &crate::core::Backend,
    profile: Option<&crate::core::BackendProfile>,
    ask: &Relate,
    records: impl Iterator<Item = Result<RecordInput<crate::QuestionInput>, Error>>,
    controls: &CallOptions<'_>,
) -> Result<Preview, Error> {
    let admitted = super::records::collect(ask, records, controls, |_| Ok(()))?;
    let mut prepared =
        facade::relations(&admitted.entities, &ask.0, backend, profile).map_err(Error::from)?;
    let context = controls
        .context_text()
        .map(|context| crate::public::RecordContext::resolved(None, Some(context)))
        .transpose()?
        .flatten();
    let requests = if let Some(context) = &context {
        prepared
            .asks
            .clone()
            .with_context_value(backend, &context.as_json())
            .map_err(Error::from)?
            .requests(backend, profile, facade::Bound::pairs(profile))
            .map_err(Error::from)?
    } else {
        std::mem::take(&mut prepared.requests)
    };
    let requests_per_rule = if context.is_none() {
        prepared.requests_per_rule.clone()
    } else {
        contextual_counts(&prepared, &requests)
    };
    let mut summary = crate::core::PlanSummary::new(true).with_accounting(backend.accounting());
    summary
        .records_added(admitted.entities.len())
        .map_err(|_| Error::defect("a plan is too large"))?;
    for request in &requests {
        summary
            .request(&request.body)
            .map_err(|_| Error::defect("a plan is too large"))?;
    }
    Ok(Preview {
        summary,
        prepared,
        requests,
        requests_per_rule,
        entities: admitted.entities.len(),
    })
}

fn contextual_counts(
    prepared: &facade::PreparedRelations,
    requests: &[facade::Request],
) -> Vec<usize> {
    (0..prepared.rules.len())
        .map(|at| {
            let belongs = |place: &usize| {
                prepared
                    .asked
                    .get(*place)
                    .is_some_and(|asked| asked.rule() == at)
            };
            requests
                .iter()
                .filter(|request| request.places.iter().any(&belongs))
                .count()
        })
        .collect()
}
