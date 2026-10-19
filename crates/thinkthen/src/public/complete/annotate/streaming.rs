//! Fallible annotation sources share eager preparation and complete row rendering.
use super::{Annotations, Held, Prepared, complete_row, failed, prepare_record};
use crate::engine::pipeline::Asker as _;
use crate::public::{
    Batch, CallOptions, CompleteAnnotated, CompleteRecord, Engine, Error, InputEvidence,
    QuestionSet, RecordInput,
};
use crate::public::{asking::Text, options::Stop, pull};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

struct Original<T> {
    held: Held<T>,
    prepared: Prepared,
    missing_pointer: bool,
}
impl Engine {
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
        self.annotate_stream_project(
            questions,
            records,
            options,
            context,
            recover,
            |original, ordinal, result| {
                Ok(Some(CompleteRecord {
                    original,
                    ordinal,
                    result,
                }))
            },
        )
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "one annotation pipeline retains private reading and recovery projections"
    )]
    pub(crate) fn annotate_stream_project<'a, I, T, R: 'a>(
        &self,
        questions: QuestionSet,
        records: I,
        options: CallOptions<'a>,
        context: Option<String>,
        recover: crate::public::options::AnnotationRecovery<'a>,
        project: impl Fn(T, usize, CompleteAnnotated) -> Result<Option<R>, Error> + 'a,
    ) -> Result<Batch<'a, R>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        let strict_preflight = options.eager_inputs && recover.is_none();
        let setting = crate::public::bulk::selected_set_batch(&questions.0, &options, self.batch)?;
        let engine = Arc::clone(&self.inner);
        let stop = Stop::begin(options)?.with_prices(self.prices);
        let mut packing = pull::packing(setting, false, false);
        packing.detailed = stop.facts().attempts().is_some();
        let preparing = Annotations {
            shared_state_groups: options.shared_state_groups,
            recover_missing: false,
            require_usable_groups: true,
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        let preparing_set = questions.0.clone();
        let records = records.into_iter().enumerate().map(move |(at, record)| {
            let (held, prepared) = record
                .and_then(|record| prepare_record(&preparing_set, record, context.as_deref(), at))
                .map_err(|error| error.at_record(at))?;
            let missing_pointer = match preparing.asks(&prepared) {
                Ok(_) => false,
                Err(error) if strict_preflight || error.missed_pointer().is_none() => {
                    return Err(error.at_record(at));
                }
                Err(_) => true,
            };
            Ok(Original {
                held,
                prepared,
                missing_pointer,
            })
        });
        let records = self.admit_prepared_stream(records, &options)?;
        let asker = Annotations {
            shared_state_groups: options.shared_state_groups,
            recover_missing: recover.is_some(),
            require_usable_groups: true,
            engine: Arc::clone(&engine),
            set: questions.0.clone(),
        };
        let barrier = Arc::new(AtomicBool::new(false));
        let preparing_barrier = Arc::clone(&barrier);
        let call = pull::Call {
            engine: Arc::clone(&engine),
            stop,
            packing,
            most: self.most,
            pending_inputs: false,
            input_barrier: Some(Arc::clone(&barrier)),
        };
        Ok(pull::try_start_prepared(
            call,
            asker,
            records,
            Box::new(move |_, original: &Original<T>| {
                // Arm only when this actual input is fed, never while an eager
                // reader preflights later originals before the pull starts.
                if original.missing_pointer {
                    preparing_barrier.store(true, Ordering::Release);
                }
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
                    && crate::public::options::continue_annotation(recover, at, pointer)
                {
                    barrier.store(false, Ordering::Release);
                    return Ok(None);
                }
                // Decline, panic and all other errors use the existing joined
                // terminal path; no later input can cross the armed barrier.
                let result = complete_row(&engine, &questions.0, stop, &original.held, at, row)
                    .map_err(|error| error.at_record(at))?;
                project(original.held.original, at, result)
            }),
        ))
    }
}
