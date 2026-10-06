//! Whole-set relations retain arbitrary originals and use the ordinary entity projection.
use crate::public::{
    Call, CallOptions, CompleteRecord, CompleteRelated, Engine, Error, InputEvidence,
    InputFunction, QuestionInput, RecordInput, Relate,
};
use std::sync::Arc;
impl Engine {
    /// Relate selected native records, retaining the complete ordered original entity set.
    /// # Errors
    /// Invalid fields, duplicate entities, unsupported controls and images refuse before sending.
    #[allow(
        clippy::type_complexity,
        reason = "The whole original set retains a concrete complete relation aggregate"
    )]
    pub fn relate_records_complete_with<I, T>(
        &self,
        ask: &Relate,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteRecord<Vec<T>, CompleteRelated>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        self.try_relate_records_complete_with(ask, records.into_iter().map(Ok), options)
    }
    /// Admit a fallible native whole set with the existing 255-entity boundary.
    /// # Errors
    /// Reader/field failures refuse the whole set; started failures retain final call facts.
    #[allow(
        clippy::type_complexity,
        reason = "The whole original set retains a concrete complete relation aggregate"
    )]
    pub fn try_relate_records_complete_with<I, T>(
        &self,
        ask: &Relate,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<CompleteRecord<Vec<T>, CompleteRelated>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>>,
        T: InputEvidence,
    {
        let mut pairs = Vec::new();
        let mut inputs = Vec::new();
        let mut originals = Vec::new();
        let mut lines = None;
        let mut bytes = 0usize;
        let records = self.try_within_limit(records.into_iter().take(256).enumerate().map(
            |(at, record)| {
                record
                    .and_then(|record| prepare(ask, record, (&mut lines, &mut bytes)))
                    .map_err(|error| error.at_record(at))
            },
        ))?;
        for (original, input, pair) in records {
            pairs.push(pair);
            inputs.push(input);
            originals.push(original);
        }
        let lines = lines.unwrap_or(false);
        if lines {
            ask.0.check_lines().map_err(Error::refused)?;
        }
        let entities = ask.0.admit(&pairs).map_err(Error::refused)?;
        self.relate_admitted_complete(ask, entities, options, lines, &inputs)?
            .try_map(|result| {
                Ok(CompleteRecord {
                    original: originals,
                    ordinal: 0,
                    result,
                })
            })
    }
}
type Prepared<T> = (T, Arc<QuestionInput>, (String, String));
fn prepare<T: InputEvidence>(
    ask: &Relate,
    record: RecordInput<T>,
    (lines, bytes): (&mut Option<bool>, &mut usize),
) -> Result<Prepared<T>, Error> {
    if record.context.is_some() || record.options.is_some() {
        return Err(Error::usage(
            "relate takes one whole-set call context and no per-record controls",
        ));
    }
    let input = Arc::new(record.original.question_input());
    crate::public::images::guard(InputFunction::Relate, &input)?;
    let (pair, literal, size) = match input.as_ref() {
        QuestionInput::Text(text) => ((text.clone(), "*".into()), true, text.len()),
        QuestionInput::Record(record) => {
            let selected = record.selected_record();
            let literal = selected.text().is_some();
            let pair = match selected.text() {
                Some(text) => (text.to_owned(), "*".into()),
                None => ask.0.record_pair(&selected).map_err(Error::refused)?,
            };
            (pair, literal, record.original().retained_bytes()?)
        }
        QuestionInput::Images(_) => return Err(super::super::wrong()),
    };
    if lines.is_some_and(|lines| lines != literal) {
        return Err(Error::usage(
            "relate takes one input mode for its whole entity set",
        ));
    }
    *bytes = bytes
        .checked_add(size)
        .filter(|size| *size <= 16 * 1024 * 1024)
        .ok_or_else(|| Error::usage("relate input exceeds 16 MiB"))?;
    *lines = Some(literal);
    Ok((record.original, input, pair))
}
