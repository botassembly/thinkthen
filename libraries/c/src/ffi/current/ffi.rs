//! Counted constructors and named helpers; all foreign pointer reads stay here.
#![allow(
    unsafe_code,
    reason = "the reviewed C lifetime contract permits foreign pointer access"
)]
#![deny(unsafe_op_in_unsafe_fn)]
/// Frozen C layouts for current-native views and reviewed input descriptors.
pub mod carriers;
#[path = "question/ffi.rs"]
mod question;
#[path = "read/ffi.rs"]
mod read;
use crate::Door;
use crate::current::{self, QuestionHandle, ResultHandle, Source, SourceHandle};
use crate::failures::{Failure, OK, USAGE, guard};
use carriers::{
    ControlsV1, CurrentAnnotationV1, CurrentAtomicV1, CurrentAttemptV1, CurrentQuestionV1,
    CurrentRecognitionV1, CurrentRelationsV1, CurrentRowV1, CurrentSummaryV1, QuestionSpecV1,
    RecordV1, SourceSpecV1, StringV1,
};

/// Clone an admitted native question from counted descriptors.
/// # Safety
/// All pointers obey the installed header's storage and lifetime contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_new(
    engine: *const Door,
    spec: *const QuestionSpecV1,
    out: *mut *mut QuestionHandle,
) -> i32 {
    // SAFETY: only null or live descriptors/outputs reach this boundary.
    unsafe {
        super::typed(
            engine,
            |_| {
                read::required(out)?;
                let spec = read::reference(spec)?;
                if spec.kind == 7 || (spec.kind == 6 && spec.members.len == 0) {
                    question::plain(spec)
                } else {
                    current::parse(spec.kind, question::build(spec)?)
                }
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
/// Load a bounded native question or question-set file.
/// # Safety
/// All pointers obey the installed header's storage and lifetime contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_load(
    engine: *const Door,
    path: StringV1,
    out: *mut *mut QuestionHandle,
) -> i32 {
    // SAFETY: counted path and writable output follow the header.
    unsafe {
        super::typed(
            engine,
            |_| {
                read::required(out)?;
                current::load(read::string(path)?)
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
/// Clone every counted original record before returning.
/// # Safety
/// All pointers obey the installed header's storage and lifetime contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_source_records(
    engine: *const Door,
    records: *const RecordV1,
    count: usize,
    out: *mut *mut SourceHandle,
) -> i32 {
    // SAFETY: readable arrays and output have their documented extents.
    unsafe {
        super::typed(
            engine,
            |_| {
                read::required(out)?;
                let records = read::slice(records, count)?
                    .iter()
                    .enumerate()
                    .map(|(index, record)| {
                        read::flag(record.context.present)?;
                        if record.context.present != 0
                            || record.options.len != 0
                            || record.images.len != 0
                        {
                            return Err(Failure::usage(
                                "current sources require text without record context, options or images",
                            ));
                        }
                        let original = read::optional_content(record.original)?
                            .ok_or_else(|| Failure::usage("text records require original content"))?;
                        Ok(current::Input {
                            original,
                            position: None,
                            index,
                        })
                    })
                    .collect::<Result<Vec<_>, Failure>>()?;
                Ok(SourceHandle(Source::Records(records)))
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
/// Clone a text reader selection, validating native reader controls.
/// # Safety
/// All pointers obey the installed header's storage and lifetime contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_source_files(
    engine: *const Door,
    spec: *const SourceSpecV1,
    out: *mut *mut SourceHandle,
) -> i32 {
    // SAFETY: counted paths and descriptors are readable through this call.
    unsafe {
        super::typed(
            engine,
            |_| {
                read::required(out)?;
                let spec = read::reference(spec)?;
                let unit = match spec.unit {
                    1 => thinkthen::SourceUnit::Line,
                    2 => thinkthen::SourceUnit::Window,
                    3 => thinkthen::SourceUnit::File,
                    _ => {
                        return Err(Failure::usage(
                            "current sources require explicit text reader units",
                        ));
                    }
                };
                let options = thinkthen::ReaderOptions {
                    unit,
                    window: (spec.window != 0).then_some(spec.window),
                }
                .validate()?;
                let paths = read::strings(spec.paths)?;
                Ok(SourceHandle(Source::Files(paths, options)))
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
macro_rules! free {
    ($name:ident, $ty:ty) => {
        /// Free the owned handle; NULL is harmless.
        /// # Safety
        /// A nonnull handle is live, freed once, with no concurrent borrowers.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(owner: *mut $ty) {
            guard(None, (), || {
                if !owner.is_null() {
                    // SAFETY: only the allocating library frees this live owner, once.
                    drop(unsafe { Box::from_raw(owner) });
                }
            });
        }
    };
}
free!(thinkthen_question_free, QuestionHandle);
free!(thinkthen_source_free, SourceHandle);
free!(thinkthen_result_free, ResultHandle);

macro_rules! ask {
    ($name:ident, $kind:literal) => {
        /// Execute the native function into owned, typed current-result views.
        /// # Safety
        /// Handles/controls remain live through return; output is writable.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(
            engine: *const Door,
            question: *const QuestionHandle,
            source: *const SourceHandle,
            controls: *const ControlsV1,
            out: *mut *mut ResultHandle,
        ) -> i32 {
            // SAFETY: caller promises live handles and initialized descriptors.
            unsafe {
                super::typed(
                    engine,
                    |held| {
                        read::required(out)?;
                        let question = read::reference(question)?;
                        let source = read::reference(source)?;
                        let controls = controls.as_ref();
                        let context = controls
                            .map(|c| read::optional_content(c.context))
                            .transpose()?
                            .flatten();
                        let mut options = crate::door::options(
                            controls.map_or(-1, |c| c.deadline_ms),
                            controls.and_then(|c| c.cancel.as_ref()),
                        )?;
                        let attempts = if let Some(c) = controls {
                            read::flag(c.batch_max)?;
                            read::flag(c.attempts)?;
                            read::flag(c.batch.present)?;
                            if !read::string(c.surface)?.is_empty() {
                                return Err(Failure::usage(
                                    "current calls cannot transmit surface tokens",
                                ));
                            }
                            if c.batch.present != 0 && c.batch_max != 0 {
                                return Err(Failure::usage("batch and batch_max conflict"));
                            }
                            if c.batch.present != 0 {
                                let batch = std::num::NonZeroUsize::new(c.batch.value)
                                    .ok_or_else(|| Failure::usage("batch must be positive"))?;
                                options = options.batch(thinkthen::BatchSetting::Records(batch));
                            } else if c.batch_max != 0 {
                                options = options.batch(thinkthen::BatchSetting::Max);
                            }
                            c.attempts != 0
                        } else {
                            false
                        };
                        if let Some(context) = context.as_ref() {
                            options = options.context(context.text());
                        }
                        current::ask(&held.engine, $kind, question, source, options, attempts)
                    },
                    |_, value| {
                        *out = Box::into_raw(Box::new(value));
                        OK
                    },
                )
            }
        }
    };
}
ask!(thinkthen_decide_current, 1);
ask!(thinkthen_choose_current, 2);
ask!(thinkthen_tag_current, 3);
ask!(thinkthen_score_current, 4);
ask!(thinkthen_filter_current, 5);
ask!(thinkthen_rank_current, 6);
ask!(thinkthen_find_current, 7);
ask!(thinkthen_annotate_current, 8);
ask!(thinkthen_recognize_current, 9);
ask!(thinkthen_relate_current, 10);

macro_rules! accessor {
    ($name:ident, $ty:ty, $body:expr) => {
        /// Borrow one typed view, leaving output untouched on argument refusal.
        /// # Safety
        /// The result is live through every use of the view; output is writable.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(
            owner: *const ResultHandle,
            index: usize,
            out: *mut $ty,
        ) -> i32 {
            guard(None, USAGE, || {
                // SAFETY: only null or live handles are allowed; refuse before writing.
                let Some(owner) = (unsafe { owner.as_ref() }) else {
                    return USAGE;
                };
                if out.is_null() {
                    return USAGE;
                }
                let get: fn(&ResultHandle, usize) -> Option<$ty> = $body;
                let Some(value) = get(owner, index) else {
                    return USAGE;
                };
                // SAFETY: the checked nonnull output is writable per the header.
                unsafe {
                    *out = value;
                }
                OK
            })
        }
    };
}
accessor!(
    thinkthen_result_current_atomic,
    CurrentAtomicV1,
    |r, i| match r.rows.get(i)? {
        current::Row::Atomic(v) => Some(**v),
        _ => None,
    }
);
accessor!(
    thinkthen_result_current_rank,
    CurrentRowV1,
    |r, i| if r.summary.function == 6 {
        match r.rows.get(i)? {
            current::Row::Original(v) => Some(*v),
            _ => None,
        }
    } else {
        None
    }
);
accessor!(
    thinkthen_result_current_find,
    carriers::CurrentFindV1,
    |r, i| if r.summary.function == 7 {
        match r.rows.get(i)? {
            current::Row::Find(v) => Some(*v),
            _ => None,
        }
    } else {
        None
    }
);
accessor!(
    thinkthen_result_current_annotate,
    CurrentAnnotationV1,
    |r, i| match r.rows.get(i)? {
        current::Row::Annotation(v) => Some(*v),
        _ => None,
    }
);
accessor!(
    thinkthen_result_current_recognize,
    CurrentRecognitionV1,
    |r, i| match r.rows.get(i)? {
        current::Row::Recognition(v) => Some(*v),
        _ => None,
    }
);
accessor!(
    thinkthen_result_current_relate,
    CurrentRelationsV1,
    |r, i| match r.rows.get(i)? {
        current::Row::Relations(v) => Some(*v),
        _ => None,
    }
);
accessor!(
    thinkthen_result_current_question,
    CurrentQuestionV1,
    |r, i| r.questions.get(i).copied()
);
accessor!(
    thinkthen_result_current_attempt,
    CurrentAttemptV1,
    |r, i| r.attempts.get(i).copied()
);
/// Borrow the function, counts and truthful native final facts.
/// # Safety
/// Owner is null or live; output is writable and borrowed fields follow owner lifetime.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_result_current_summary(
    owner: *const ResultHandle,
    out: *mut CurrentSummaryV1,
) -> i32 {
    guard(None, USAGE, || {
        // SAFETY: the caller supplies a live owner or NULL.
        let Some(owner) = (unsafe { owner.as_ref() }) else {
            return USAGE;
        };
        if out.is_null() {
            return USAGE;
        }
        // SAFETY: nonnull output is writable, as the caller promises.
        unsafe {
            *out = owner.summary;
        }
        OK
    })
}

// Filter exposes accepted originals through its own accessor.
accessor!(
    thinkthen_result_current_filter,
    CurrentRowV1,
    |r, i| if r.summary.function == 5 {
        match r.rows.get(i)? {
            current::Row::Original(v) => Some(*v),
            _ => None,
        }
    } else {
        None
    }
);
