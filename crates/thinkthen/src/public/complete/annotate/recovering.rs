//! Recover independent annotations on the ordinary grouped question scheduler.
use super::{Annotations, complete_row, prepare_record};
use crate::engine::pipeline::Asker as _;
use crate::public::{
    Call, CallOptions, CompleteRecord, Engine, Error, QuestionInput, QuestionSet, RecordInput,
    RequestRecoveredRow, RequestValue, options::Stop, pull,
};
use std::sync::Arc;
impl Engine {
    pub(in crate::public) fn recover_annotations(
        &self,
        questions: &QuestionSet,
        records: Vec<Result<RecordInput<QuestionInput>, Error>>,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<RequestRecoveredRow>>, Error> {
        let setting = crate::public::bulk::selected_set_batch(&questions.0, &options, self.batch)?;
        let engine = Arc::clone(&self.inner);
        let asker = Annotations {
            recover_missing: true,
            require_usable_groups: false,
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let (admitted, rejected) =
            super::super::recovering::prepare(records, |ordinal, record| {
                let (held, prepared) =
                    prepare_record(&questions.0, record, options.context_text(), ordinal)?;
                asker.asks(&prepared)?;
                Ok((held, prepared))
            })?;
        let mut packing = pull::packing(setting, false, true);
        packing.detailed = stop.facts().attempts().is_some();
        super::super::recovering::run(
            (&engine, packing),
            &asker,
            &stop,
            admitted,
            rejected,
            |ordinal, held, row| {
                let outcome =
                    complete_row(&engine, &questions.0, &stop, &held, ordinal, row).map(
                        |result| {
                            RequestValue::Annotations(vec![CompleteRecord {
                                original: held.original,
                                ordinal,
                                result,
                            }])
                        },
                    );
                crate::public::request::recovery_row(ordinal, outcome)
                    .map(|row| row.with_cause(None))
            },
        )
    }
}

struct Original {
    held: super::Held<QuestionInput>,
    prepared: super::Prepared,
}
impl Engine {
    pub(in crate::public) fn recover_annotations_stream<'a, I>(
        &self,
        questions: QuestionSet,
        records: I,
        options: CallOptions<'a>,
        context: Option<String>,
    ) -> Result<crate::Batch<'a, RequestRecoveredRow>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<QuestionInput>, Error>> + 'a,
    {
        let setting = crate::public::bulk::selected_set_batch(&questions.0, &options, self.batch)?;
        let engine = Arc::clone(&self.inner);
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let mut packing = pull::packing(setting, false, true);
        packing.detailed = stop.facts().attempts().is_some();
        let preparing = Annotations {
            recover_missing: false,
            require_usable_groups: false,
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        let preparing_set = questions.0.clone();
        let records = records.into_iter().enumerate().map(move |(at, record)| {
            let (held, prepared) = record
                .and_then(|record| prepare_record(&preparing_set, record, context.as_deref(), at))
                .map_err(|error| error.at_record(at))?;
            preparing
                .asks(&prepared)
                .map_err(|error| error.at_record(at))?;
            Ok(Original { held, prepared })
        });
        let records = self.admit_prepared_stream(records, &options)?;
        let asker = Annotations {
            recover_missing: false,
            require_usable_groups: false,
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        let call = pull::Call {
            engine: Arc::clone(&engine),
            stop,
            packing,
            most: self.most,
            pending_inputs: options.pending_inputs,
        };
        Ok(pull::try_start_prepared(
            call,
            asker,
            records,
            Box::new(|_, original: &Original| {
                Ok(super::Prepared {
                    explicit_context: original.prepared.explicit_context,
                    text: crate::public::asking::Text {
                        at: original.prepared.text.at,
                        input: original.prepared.text.input.clone(),
                    },
                    context: original.prepared.context.clone(),
                })
            }),
            Box::new(move |stop, at, original, row| {
                let row = reader_admission(row).map_err(|error| error.at_record(at))?;
                let original = match original {
                    Some(original) => original,
                    None => {
                        return Err(row
                            .err()
                            .map_or_else(
                                || Error::defect("a recovering annotation lost its original"),
                                super::failed,
                            )
                            .at_record(at));
                    }
                };
                let outcome = complete_row(
                    &engine,
                    &questions.0,
                    stop,
                    &original.held,
                    at,
                    row,
                )
                .map(|result| {
                    RequestValue::Annotations(vec![CompleteRecord {
                        original: original.held.original,
                        ordinal: at,
                        result,
                    }])
                });
                crate::public::request::recovery_row(at, outcome).map(Some)
            }),
        ))
    }
}

fn reader_admission(row: pull::Row<Annotations>) -> Result<pull::Row<Annotations>, Error> {
    match row {
        Err(crate::engine::pipeline::Failed::Asker(error))
            if matches!(
                error.kind(),
                crate::ErrorKind::Usage | crate::ErrorKind::Local
            ) =>
        {
            Err(error)
        }
        row => Ok(row),
    }
}
