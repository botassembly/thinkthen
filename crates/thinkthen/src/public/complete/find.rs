//! Complete whole-set selection retains the dedicated find grammar and originals.
use crate::core;
use crate::public::options::Stop;
use crate::public::{Call, CallOptions, CompleteFound, Engine, Error, Evidence, Found, Question};
mod records;

impl Engine {
    /// Admit a fallible whole set before executing the complete find question.
    /// # Errors
    /// Reader, count or byte failures refuse before sending; started failures retain final facts.
    pub(crate) fn try_find_complete_with<I, T>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteFound<T>>, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
        T: Evidence,
    {
        crate::public::request::native(
            self,
            crate::RequestFunction::Find,
            question.clone().into(),
            units
                .into_iter()
                .map(|unit| unit.map(crate::public::record_input::plain)),
            options,
            |engine, definition, units, controls| {
                let q = definition.atomic()?;
                Ok(crate::public::request::restore_found(
                    engine.execute_find_records_complete_with(
                        &q,
                        units.into_iter().map(Ok),
                        controls,
                    )?,
                ))
            },
        )
    }
    fn find_complete_inputs<T>(
        &self,
        question: &Question,
        units: Vec<T>,
        options: CallOptions<'_>,
        inputs: &[std::sync::Arc<crate::public::QuestionInput>],
    ) -> Result<Call<CompleteFound<T>>, Error>
    where
        T: Evidence,
    {
        let options = options.started()?;
        let (units, find, engine) = self.prepare_find(question, units, &options)?;
        let texts = units
            .iter()
            .map(|unit| unit.evidence().to_owned())
            .collect::<Vec<_>>();
        let stop = Stop::begin(options)?.with_prices(self.prices);
        stop.run_call(1, |cancel| {
            let found = engine.find(&find, cancel).map_err(Error::from)?;
            crate::public::bulk::observation::observe_find_inputs(
                &stop, &engine, question, &find, &found, inputs,
            )?;
            Ok(found)
        })?
        .try_map(|found| {
            let none = question.kind == crate::public::question::Kind::FindNone;
            let picked = found
                .selection
                .selected()
                .map(|at| texts.get(at).ok_or_else(super::wrong))
                .transpose()?
                .map(|text| {
                    core::Reading::new(core::Framing::Document, Vec::new())
                        .map_err(Error::refused)?
                        .record(text.as_bytes())
                        .map_err(Error::refused)
                })
                .transpose()?;
            let public = Found::new(units, none, &found)?;
            Ok(CompleteFound {
                canonical: crate::result_json::complete::find(
                    &engine,
                    &find,
                    found,
                    crate::result_json::complete::FindRow {
                        declarations: question.reading_metadata(),
                        question: question.core.clone(),
                        candidates: texts,
                        input: picked,
                        context_sha256: options
                            .context_text()
                            .filter(|text| !text.is_empty())
                            .map(|text| core::bytes_sha256(text.as_bytes())),
                        attempts: stop.facts().attempts().map(<[_]>::to_vec),
                    },
                )
                .map_err(|_| super::wrong())?,
                found: public,
                sources: inputs
                    .iter()
                    .map(|input| super::physical_source(input))
                    .collect(),
            })
        })
    }
}
