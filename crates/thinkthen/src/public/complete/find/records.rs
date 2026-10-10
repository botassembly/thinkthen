//! Whole-set find retains actual originals after shared selected-input composition.
use crate::public::{
    Call, CallOptions, CompleteFound, Engine, Error, Evidence, InputEvidence, InputFunction,
    Question, QuestionInput, RecordInput,
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
    /// Find within a composed whole set, preserving each original and its source location.
    /// # Errors
    /// Refuses images, per-record controls and invalid later units before sending.
    pub fn find_records_complete_with<I, T>(
        &self,
        question: &Question,
        units: I,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteFound<T>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        self.try_find_records_complete_with(question, units.into_iter().map(Ok), options)
    }

    /// Admit a fallible selected set through the same native find count/byte boundary.
    /// # Errors
    /// A reader or unsupported-input failure refuses the set before any send.
    pub fn try_find_records_complete_with<I, T>(
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
        let units = self.prepare_find_records(question, units, &options)?;
        let inputs = units
            .iter()
            .map(|unit| std::sync::Arc::clone(&unit.input))
            .collect::<Vec<_>>();
        self.find_complete_inputs(question, units.into_iter().map(Ok), options, &inputs)?
            .try_map(|result| {
                Ok(CompleteFound {
                    canonical: result.canonical,
                    sources: result.sources,
                    found: result.found.map(|unit| unit.original),
                })
            })
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
        let units = self.prepare_find_records(question, units, &options)?;
        let (units, find, engine) =
            self.prepare_find(question, units.into_iter().map(Ok), &options)?;
        for (at, unit) in units.iter().enumerate() {
            question
                .metadata
                .validate_item(&unit.input)
                .map_err(|error| error.at_record(at))?;
        }
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
        let mut summary =
            crate::core::PlanSummary::new(false).with_accounting(engine.backend().accounting());
        let large = || Error::defect("a plan is too large");
        summary.records_added(units.len()).map_err(|_| large())?;
        for request in prepared {
            summary.request(&request.body).map_err(|_| large())?;
        }
        Ok((find, summary))
    }

    fn prepare_find_records<I, T>(
        &self,
        question: &Question,
        units: I,
        options: &CallOptions<'_>,
    ) -> Result<Vec<Unit<T>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>>,
        T: InputEvidence,
    {
        let mut bytes = crate::public::SourceBudget::find();
        let maximum =
            crate::core::Find::maximum(question.kind == crate::public::question::Kind::FindNone);
        let units = units
            .into_iter()
            .take(maximum + 1)
            .enumerate()
            .map(|(at, unit)| {
                unit.and_then(|unit| prepare(unit, &mut bytes))
                    .map_err(|error| error.at_record(at))
            });
        self.try_within_admission(units, options)
            .map(Iterator::collect)
    }
}

fn prepare<T: InputEvidence>(
    unit: RecordInput<T>,
    bytes: &mut crate::public::SourceBudget,
) -> Result<Unit<T>, Error> {
    unit.admit_recognition_controls(
        crate::public::request::RequestFunction::from_input(InputFunction::Find),
        "find takes one whole-set call context and no per-record controls",
        "find takes one whole-set call context and no per-record controls",
    )?;
    if unit.context.is_some() || unit.options.is_some() {
        return Err(Error::usage(
            "find takes one whole-set call context and no per-record controls",
        ));
    }
    let input = unit.original.question_input();
    crate::public::images::guard(InputFunction::Find, &input)?;
    let (text, size) = match &input {
        QuestionInput::Text(text) => {
            let size = text.len();
            (text.clone(), size)
        }
        QuestionInput::Record(record) => (
            record.plain().to_owned(),
            record.original().retained_bytes()?,
        ),
        QuestionInput::Images(_) => return Err(super::super::wrong()),
    };
    bytes.charge(size)?;
    Ok(Unit {
        original: unit.original,
        text,
        input: std::sync::Arc::new(input),
    })
}
