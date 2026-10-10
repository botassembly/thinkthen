//! Complete relations keep actual logical successes and recoverable failures.
use super::contextual;
use crate::core;
use crate::engine::facade;
use crate::public::options::Stop;
use crate::public::{
    Call, CallOptions, CompleteRelated, Edge, Engine, Entity, Error, InputFunction, Relate,
};
use std::sync::Arc;
mod records;

impl Engine {
    /// Relate one whole set, retaining rejected and failed logical members.
    /// # Errors
    /// Invalid entities refuse before sending; all-failed calls and transport stops return final facts.
    pub fn relate_complete_with<I>(
        &self,
        ask: &Relate,
        entities: I,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteRelated>, Error>
    where
        I: IntoIterator<Item = Entity>,
    {
        let entities = admitted(ask, entities)?;
        self.relate_admitted_complete(ask, entities, options, false, &[])
    }

    fn relate_admitted_complete(
        &self,
        ask: &Relate,
        entities: Vec<core::RelationEntity>,
        options: CallOptions<'_>,
        lines: bool,
        inputs: &[Arc<crate::public::QuestionInput>],
    ) -> Result<Call<CompleteRelated>, Error> {
        let engine = contextual(self.for_model(ask.0.model.as_ref())?, &options)?;
        let prepared = facade::relations(&entities, &ask.0, engine.backend(), engine.profile())?;
        let threshold = ask.0.threshold.cut_value().unwrap_or(0.5);
        let stop = Stop::begin(options)?.with_prices(self.prices);
        stop.run_call(usize::from(!prepared.asks.is_empty()), |cancel| {
            let mut positions = [0; 4];
            let execution = engine
                .relate_observed(
                    prepared,
                    &entities,
                    threshold,
                    cancel,
                    |question, answered| {
                        crate::public::results::observe_question_inputs(
                            &stop,
                            engine.backend(),
                            (
                                InputFunction::Relate,
                                "relation",
                                question,
                                Some(ask.0.threshold),
                            ),
                            answered,
                            &mut positions,
                            (0, inputs),
                        )
                    },
                )
                .map_err(Error::from)?;
            if execution.failed > 0 && execution.answered == 0 {
                return Err(crate::public::asking::backend_failed());
            }
            let value = execution
                .edges
                .iter()
                .cloned()
                .map(Edge::from_native)
                .collect::<Result<Vec<_>, _>>()?;
            stop.observe(crate::public::RecordObservation::Row {
                index: 0,
                value: crate::public::ObservedRow::Relations(&value),
            });
            Ok((execution, value))
        })?
        .try_map(|(execution, value)| {
            Ok(CompleteRelated {
                canonical: crate::result_json::complete::relation(
                    &engine,
                    &ask.0,
                    &entities,
                    &execution,
                    crate::result_json::complete::RelationRow {
                        lines,
                        context_sha256: options
                            .context_text()
                            .filter(|text| !text.is_empty())
                            .map(|text| core::bytes_sha256(text.as_bytes())),
                        attempts: stop.facts().attempts().map(<[_]>::to_vec),
                    },
                )
                .map_err(|_| super::wrong())?,
                value,
                source_edges: None,
                input_sources: None,
            })
        })
    }
}

fn admitted(
    ask: &Relate,
    entities: impl IntoIterator<Item = Entity>,
) -> Result<Vec<core::RelationEntity>, Error> {
    let pairs = entities
        .into_iter()
        .take(core::RelateSpec::MAX_ENTITIES + 1)
        .map(|entity| (entity.name().to_owned(), entity.kind().to_owned()))
        .collect::<Vec<_>>();
    let entities = ask.0.admit(&pairs).map_err(Error::refused)?;
    ask.validate_pairs(&pairs)?;
    Ok(entities)
}

mod preview;
pub(crate) use preview::Preview as RelationPreview;
pub(in crate::public) use preview::prepare as preview_relations;
