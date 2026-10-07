//! Complete whole-set selection retains the dedicated find grammar and originals.
use crate::core;
use crate::public::options::Stop;
use crate::public::{Call, CallOptions, CompleteFound, Engine, Error, Evidence, Found, Question};
mod records;

impl Engine {
    /// Select from the whole admitted ordered set, retaining every candidate probability.
    /// # Errors
    /// Uses the same count/evidence admission, observer and cancellation boundaries as find_with.
    pub fn find_complete_with<I>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteFound<I::Item>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence,
    {
        self.try_find_complete_with(question, units.into_iter().map(Ok), options)
    }

    /// Admit a fallible whole set before executing the complete find question.
    /// # Errors
    /// Reader, count or byte failures refuse before sending; started failures retain final facts.
    pub fn try_find_complete_with<I, T>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteFound<T>>, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
        T: Evidence,
    {
        self.find_complete_inputs(question, units, options, &[])
    }
    fn find_complete_inputs<I, T>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
        inputs: &[std::sync::Arc<crate::public::QuestionInput>],
    ) -> Result<Call<CompleteFound<T>>, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
        T: Evidence,
    {
        let options = options.started()?;
        let (units, find, engine) = self.prepare_find(question, units, &options)?;
        for (at, unit) in units.iter().enumerate() {
            let input = inputs.get(at).map(|input| input.as_ref());
            match input {
                Some(input) => question.metadata.validate_item(input),
                None => question
                    .metadata
                    .validate_item(&crate::public::QuestionInput::Text(
                        unit.evidence().to_owned(),
                    )),
            }
            .map_err(|error| error.at_record(at))?;
        }
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
                        declarations: question.metadata.clone(),
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
            })
        })
    }
}
