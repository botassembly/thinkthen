//! Whole-set relations retain arbitrary originals and use the ordinary entity projection.
use crate::public::{
    Call, CallOptions, CompleteRecord, CompleteRelated, Engine, Error, InputEvidence,
    InputFunction, QuestionInput, RecordInput, Relate,
};
use std::sync::Arc;
impl Engine {
    /// Relate selected native records, retaining the complete ordered original entity set.
    /// # Errors
    /// Invalid fields, non-source duplicate entities, unsupported controls and images refuse before sending.
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
        let options = options.started()?;
        options.admission()?;
        let Admitted {
            pairs,
            inputs,
            originals,
            lines,
            located,
            entities,
        } = collect(ask, records, &options, |at| self.check_record_limit(at))?;
        let input_sources = located.then(|| {
            inputs
                .iter()
                .enumerate()
                .filter_map(|(index, input)| {
                    super::super::physical_source(input)
                        .map(|source| crate::core::CompleteIndexedSource { index, source })
                })
                .collect()
        });
        self.relate_admitted_complete(ask, entities, options, lines, &inputs)?
            .try_map(|mut result| {
                result.input_sources = input_sources;
                if located {
                    result.source_edges = Some(crate::public::results::source_relation::expand(
                        &result.value,
                        &inputs,
                        &pairs,
                    )?);
                }
                Ok(CompleteRecord {
                    original: originals,
                    ordinal: 0,
                    result,
                })
            })
    }
}
pub(super) struct Admitted<T> {
    pub(super) pairs: Vec<(String, String)>,
    pub(super) inputs: Vec<Arc<QuestionInput>>,
    pub(super) originals: Vec<T>,
    pub(super) lines: bool,
    pub(super) located: bool,
    pub(super) entities: Vec<crate::core::RelationEntity>,
}
pub(super) fn collect<T: InputEvidence, I>(
    ask: &Relate,
    records: I,
    options: &CallOptions<'_>,
    mut check: impl FnMut(usize) -> Result<(), Error>,
) -> Result<Admitted<T>, Error>
where
    I: IntoIterator<Item = Result<RecordInput<T>, Error>>,
{
    let mut pairs = Vec::new();
    let mut inputs = Vec::new();
    let mut originals = Vec::new();
    let mut lines = None;
    let mut bytes = crate::public::SourceBudget::relate();
    let records = records.into_iter();
    let (lower, upper) = records.size_hint();
    // Eager feeds expose their finite extent; lazy feeds still stop at the refusal boundary.
    let limit = upper
        .filter(|upper| *upper == lower)
        .unwrap_or(crate::core::RelateSpec::MAX_ENTITIES + 1);
    let mut records = records.take(limit).enumerate();
    let mut count = 0;
    loop {
        options.admission()?;
        let Some((at, record)) = records.next() else {
            break;
        };
        let row = record
            .and_then(|record| prepare(ask, record, (&mut lines, &mut bytes)))
            .map_err(|error| error.at_record(at));
        options.admission()?;
        let (original, input, pair) = row?;
        check(at)?;
        count = at + 1;
        if matches!(input.as_ref(), QuestionInput::Record(record) if record.location().is_some()) {
            Relate::admit_record_count(count)?;
        }
        if count <= crate::core::RelateSpec::MAX_ENTITIES + 1 {
            pairs.push(pair);
            inputs.push(input);
            originals.push(original);
        }
    }
    let lines = lines.unwrap_or(false);
    if lines {
        ask.0.check_lines().map_err(Error::refused)?;
    }
    let located = inputs.iter().any(|input| matches!(input.as_ref(), QuestionInput::Record(record) if record.location().is_some()));
    if located && !inputs.iter().all(|input| matches!(input.as_ref(), QuestionInput::Record(record) if record.location().is_some())) {
            return Err(Error::usage("source relate takes a source for every record"));
        }
    if located {
        Relate::admit_record_count(pairs.len())?;
    }
    let distinct = if located {
        unique(&pairs)
    } else {
        pairs.clone()
    };
    let entities = ask.0.admit(&distinct).map_err(|cause| {
        if cause == crate::core::EntitySetError::TooMany {
            Error::refused(cause).with_diagnostic(
                crate::public::error::diagnostic::Diagnostic::RelationEntityCount(count),
            )
        } else {
            Error::refused(cause)
        }
    })?;
    Ok(Admitted {
        pairs,
        inputs,
        originals,
        lines,
        located,
        entities,
    })
}

type Prepared<T> = (T, Arc<QuestionInput>, (String, String));
fn prepare<T: InputEvidence>(
    ask: &Relate,
    record: RecordInput<T>,
    (lines, bytes): (&mut Option<bool>, &mut crate::public::SourceBudget),
) -> Result<Prepared<T>, Error> {
    record.admit_recognition_controls(
        crate::public::request::RequestFunction::from_input(InputFunction::Relate),
        "relate takes one whole-set call context and no per-record controls",
        "relate takes one whole-set call context and no per-record controls",
    )?;
    if record.context.is_some() || record.options.is_some() {
        return Err(Error::usage(
            "relate takes one whole-set call context and no per-record controls",
        ));
    }
    let input = Arc::new(record.original.question_input());
    ask.0.metadata.validate_item(&input)?;
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
    bytes.charge(size)?;
    *lines = Some(literal);
    Ok((record.original, input, pair))
}

fn unique(pairs: &[(String, String)]) -> Vec<(String, String)> {
    let mut distinct = Vec::new();
    for pair in pairs {
        if !distinct.contains(pair) {
            distinct.push(pair.clone());
        }
    }
    distinct
}
