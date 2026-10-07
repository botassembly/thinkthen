//! Pure logical answer framing over typed scopes, readings and constituent identities.
use crate::core::image::InputFunction;
use crate::core::{AnswerId, Observation, Origin, QuestionSource, RenderError, ResultIdentity};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct RecordScope {
    pub(crate) record: usize,
}

#[derive(Serialize)]
pub(crate) struct AtomicReading<'a> {
    pub(crate) question: &'a crate::core::Question,
    pub(crate) threshold: Option<crate::core::Threshold>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rank_position: Option<std::num::NonZeroUsize>,
}

impl ResultIdentity {
    pub(crate) fn of(
        function: InputFunction,
        scope: &impl Serialize,
        sources: Vec<QuestionSource>,
        observations: Vec<Observation>,
        reading: &impl Serialize,
        children: &[AnswerId],
    ) -> Result<Self, RenderError> {
        if sources.len() != observations.len() {
            return Err(RenderError);
        }
        let scope = serde_json::to_string(scope).map_err(|_| RenderError)?;
        let variants = serde_json::to_string(&observations).map_err(|_| RenderError)?;
        let reading = serde_json::to_string(reading).map_err(|_| RenderError)?;
        let children = serde_json::to_string(children).map_err(|_| RenderError)?;
        let digest = super::framing::digest(
            "thinkthen.answer-id/1",
            &[
                function.name().as_bytes(),
                scope.as_bytes(),
                variants.as_bytes(),
                reading.as_bytes(),
                children.as_bytes(),
            ],
        );
        let answer_id = AnswerId::new(crate::core::hex(&digest)).map_err(|_| RenderError)?;
        let origin = aggregate_origin(&sources);
        let answered_by = sources
            .first()
            .filter(|first| {
                sources
                    .iter()
                    .all(|source| source.answered_by() == first.answered_by())
            })
            .map(|source| source.model().clone());
        Ok(Self::resolved(
            answer_id,
            origin,
            sources,
            observations,
            answered_by,
        ))
    }
}

fn aggregate_origin(sources: &[QuestionSource]) -> Option<Origin> {
    [
        Origin::Live,
        Origin::Proxy,
        Origin::Memory,
        Origin::Cache,
        Origin::Replay,
    ]
    .into_iter()
    .find(|origin| sources.iter().any(|source| source.origin() == *origin))
}
