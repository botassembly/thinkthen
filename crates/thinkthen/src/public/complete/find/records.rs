//! Whole-set find retains actual originals after shared selected-input composition.
use crate::public::{
    Call, CallOptions, CompleteFound, Engine, Error, Evidence, InputEvidence, InputFunction,
    Question, QuestionInput, RecordInput,
};

struct Unit<T> {
    original: T,
    text: String,
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
        let mut bytes = 0usize;
        let units = units.into_iter().enumerate().map(|(at, unit)| {
            unit.and_then(|unit| prepare(unit, &mut bytes))
                .map_err(|error| error.at_record(at))
        });
        self.try_find_complete_with(question, units, options)?
            .try_map(|result| {
                Ok(CompleteFound {
                    canonical: result.canonical,
                    found: result.found.map(|unit| unit.original),
                })
            })
    }
}

fn prepare<T: InputEvidence>(unit: RecordInput<T>, bytes: &mut usize) -> Result<Unit<T>, Error> {
    if unit.context.is_some() || unit.options.is_some() {
        return Err(Error::usage(
            "find takes one whole-set call context and no per-record controls",
        ));
    }
    let input = unit.original.question_input();
    crate::public::images::guard(InputFunction::Find, &input)?;
    let (text, size) = match input {
        QuestionInput::Text(text) => {
            let size = text.len();
            (text, size)
        }
        QuestionInput::Record(record) => (
            record.plain().to_owned(),
            record.original().retained_bytes()?,
        ),
        QuestionInput::Images(_) => return Err(super::super::wrong()),
    };
    *bytes = bytes
        .checked_add(size)
        .filter(|size| *size <= 16 * 1024 * 1024)
        .ok_or_else(|| Error::usage("find input exceeds 16 MiB"))?;
    Ok(Unit {
        original: unit.original,
        text,
    })
}
