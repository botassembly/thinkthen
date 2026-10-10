//! Fallible annotation sources share eager preparation and complete row rendering.
use super::{Annotations, Held, Prepared, complete_row, failed, prepare_record};
use crate::engine::pipeline::Asker as _;
use crate::public::{
    Batch, CallOptions, CompleteAnnotated, CompleteRecord, Engine, Error, InputEvidence,
    QuestionSet, RecordInput,
};
use crate::public::{asking::Text, options::Stop, pull};
use std::sync::Arc;

struct Original<T> {
    held: Held<T>,
    prepared: Prepared,
}
impl Engine {
    /// Pull fallible original records into ordered complete annotations.
    /// # Errors
    /// Yields the completed prefix then one terminal error; member failures retain identities.
    pub fn try_annotate_records_complete_with<'a, I, T>(
        &'a self,
        questions: &'a QuestionSet,
        records: I,
        options: CallOptions<'a>,
    ) -> Batch<'a, CompleteRecord<T, CompleteAnnotated>>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        Batch::of(
            crate::public::request::pull::native(
                self,
                questions.clone().into(),
                records,
                options,
                crate::RequestCall::Annotate,
            )
            .and_then(|rows| match rows {
                crate::public::request::pull::Rows::Annotations(batch) => Ok(batch),
                _ => Err(Error::defect(
                    "a native annotate batch returned another result kind",
                )),
            }),
        )
    }

    pub(crate) fn annotate_stream<'a, I, T>(
        &self,
        questions: QuestionSet,
        records: I,
        options: CallOptions<'a>,
        context: Option<String>,
        recover: crate::public::options::AnnotationRecovery<'a>,
    ) -> Result<Batch<'a, CompleteRecord<T, CompleteAnnotated>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        let setting = crate::public::bulk::selected_set_batch(&questions.0, &options, self.batch)?;
        let engine = Arc::clone(&self.inner);
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let mut packing = pull::packing(setting, false, false);
        packing.detailed = stop.facts().attempts().is_some();
        let preparing = Annotations {
            recover_missing: false,
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        let preparing_set = questions.0.clone();
        let records = records.into_iter().enumerate().map(move |(at, record)| {
            let (held, prepared) = record
                .and_then(|record| prepare_record(&preparing_set, record, context.as_deref(), at))
                .map_err(|error| error.at_record(at))?;
            if let Err(error) = preparing.asks(&prepared)
                && error.missed_pointer().is_none()
            {
                return Err(error.at_record(at));
            }
            Ok(Original { held, prepared })
        });
        let records = self.admit_prepared_stream(records, &options)?;
        let asker = Annotations {
            recover_missing: recover.is_some(),
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        let call = pull::Call {
            engine: Arc::clone(&engine),
            stop,
            packing,
            most: self.most,
        };
        Ok(pull::try_start_prepared(
            call,
            asker,
            records,
            Box::new(|_, original: &Original<T>| {
                Ok(Prepared {
                    explicit_context: original.prepared.explicit_context,
                    text: Text {
                        at: original.prepared.text.at,
                        input: original.prepared.text.input.clone(),
                    },
                    context: original.prepared.context.clone(),
                })
            }),
            Box::new(move |stop, at, original, row| {
                let original = match original {
                    Some(original) => original,
                    None => {
                        return Err(row
                            .err()
                            .map_or_else(
                                || Error::defect("a pulled annotation lost its original"),
                                failed,
                            )
                            .at_record(at));
                    }
                };
                if let Err(crate::engine::pipeline::Failed::Asker(error)) = &row
                    && let Some(pointer) = error.missed_pointer()
                    && let Some(recover) = recover
                    && recover(at, pointer)
                {
                    return Ok(None);
                }
                let result = complete_row(&engine, &questions.0, stop, &original.held, at, row)
                    .map_err(|error| error.at_record(at))?;
                Ok(Some(CompleteRecord {
                    original: original.held.original,
                    ordinal: at,
                    result,
                }))
            }),
        ))
    }
}
