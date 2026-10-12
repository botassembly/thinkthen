//! Whole-set find retains actual originals after shared selected-input composition.
use crate::public::{
    Call, CallOptions, CompleteFound, Engine, Error, Evidence, InputEvidence, Question,
    QuestionInput, RecordInput,
};

struct Unit<T> {
    original: T,
    text: String,
    input: std::sync::Arc<QuestionInput>,
}
impl<T> Evidence for Unit<T> {
    fn evidence(&self) -> &str {
        &self.text
    }
}
impl Engine {
    pub(crate) fn execute_find_records_complete_with<I, T>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteFound<T>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>>,
        T: InputEvidence,
    {
        let options = options.started()?;
        options.admission()?;
        let units = units
            .into_iter()
            .map(|row| row.and_then(prepare))
            .collect::<Result<Vec<_>, _>>()?;
        let inputs = units
            .iter()
            .map(|unit| std::sync::Arc::clone(&unit.input))
            .collect::<Vec<_>>();
        Ok(self
            .find_complete_inputs(question, units, options, &inputs)?
            .map(|result| result.map_original(|unit| unit.original)))
    }

    pub(in crate::public) fn preview_find_records<I, T>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
    ) -> Result<(crate::core::Find, crate::core::PlanSummary), Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>>,
        T: InputEvidence,
    {
        crate::public::request::native(
            self,
            crate::RequestFunction::Find,
            question.clone().into(),
            units,
            options,
            |engine, definition, units, options| {
                let question = definition.atomic()?;
                let units = units
                    .into_iter()
                    .map(prepare)
                    .collect::<Result<Vec<_>, _>>()?;
                let (units, find, engine) = engine.prepare_find(&question, units, &options)?;
                let mut asks = crate::engine::facade::Asks::default();
                asks.add(engine.backend(), find.plan())
                    .map_err(Error::from)?;
                if let Some(context) = options.context_text() {
                    asks = asks
                        .with_context(engine.backend(), context)
                        .map_err(Error::from)?;
                }
                let prepared = asks
                    .requests(
                        engine.backend(),
                        engine.profile(),
                        crate::engine::facade::Bound::WHOLE,
                    )
                    .map_err(Error::from)?;
                let mut summary = crate::core::PlanSummary::new(false)
                    .with_accounting(engine.backend().accounting());
                let large = || Error::defect("a plan is too large");
                summary.records_added(units.len()).map_err(|_| large())?;
                for request in prepared {
                    summary.request(&request.body).map_err(|_| large())?;
                }
                Ok((find, summary))
            },
        )
    }
}

fn prepare<T: InputEvidence>(unit: RecordInput<T>) -> Result<Unit<T>, Error> {
    let input = unit.original.question_input();
    let text = match &input {
        QuestionInput::Text(text) => text.clone(),
        QuestionInput::Record(record) => record.plain().to_owned(),
        QuestionInput::Images(_) => return Err(super::super::wrong()),
    };
    Ok(Unit {
        original: unit.original,
        text,
        input: std::sync::Arc::new(input),
    })
}
