//! Canonical complete execution and borrowed accessors under the existing guard.
#![allow(
    unsafe_code,
    reason = "C handles and outputs obey the reviewed lifetime contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::Door;
use crate::complete::{self, ResultHandle};
use crate::current::{Content, QuestionHandle, SourceHandle};
use crate::failures::{self, DEFECT, Failure, OK, USAGE};
use crate::ffi::carriers::{
    AnnotateViewV1, ChooseViewV1, ControlsV1, DecideViewV1, DetailsV1, FilterViewV1, FindViewV1,
    ObservationV1, RankViewV1, RecognizeViewV1, RelateViewV1, ScoreViewV1, SummaryV1, TagViewV1,
};
use crate::ffi::current::read;
use thinkthen::{CallOptions, Facts};
unsafe fn run(
    engine: *const Door,
    kind: u32,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> i32 {
    // SAFETY: engine is NULL or a live owner through the whole call.
    let Some(held) = (unsafe { super::held(engine) }) else {
        return USAGE;
    };
    let mut completed: Option<Facts> = None;
    failures::guard_completed(held, &mut completed, DEFECT, |completed| {
        let result = (|| {
            read::required(out)?;
            // SAFETY: initialized live descriptors, counted buffers and handles follow the header.
            unsafe {
                let question = read::reference(question)?;
                let source = read::reference(source)?;
                let controls = controls.as_ref();
                let context = controls
                    .map(|c| read::optional_content(c.context))
                    .transpose()?
                    .flatten();
                let context = match &context {
                    Some(Content::Text(text)) => Some(text.as_str()),
                    Some(Content::Json(_)) => {
                        return Err(Failure::usage("shared context requires text"));
                    }
                    None => None,
                };
                let options = options(controls, context)?;
                complete::ask(&held.engine, kind, question, source, options, completed)
            }
        })();
        match held.settle(result.map_err(|failure| failure.completed(completed.as_ref()))) {
            Ok(result) => {
                // SAFETY: validated writable output; published only after every conversion succeeds.
                unsafe {
                    *out = Box::into_raw(Box::new(result));
                }
                OK
            }
            Err(code) => code,
        }
    })
}
unsafe fn options<'a>(
    controls: Option<&'a ControlsV1>,
    context: Option<&'a str>,
) -> Result<CallOptions<'a>, Failure> {
    let Some(c) = controls else {
        return Ok(CallOptions::new().surface(thinkthen::Surface::C));
    };
    let mut options = CallOptions::new()
        .surface(thinkthen::Surface::C)
        .deadline_ms(c.deadline_ms)?
        .attempts(read::flag(c.attempts)?);
    if let Some(context) = context {
        options = options.context(context);
    }
    // SAFETY: optional live cancellation token remains live until the native workers join.
    if let Some(cancel) = unsafe { c.cancel.as_ref() } {
        options = options.cancel(cancel);
    }
    let batch = read::flag(c.batch.present)?;
    let max = read::flag(c.batch_max)?;
    if batch && max {
        return Err(Failure::usage("batch and batch_max conflict"));
    }
    if batch {
        options = options.batch(thinkthen::BatchSetting::Records(
            std::num::NonZeroUsize::new(c.batch.value)
                .ok_or_else(|| Failure::usage("batch records must be positive"))?,
        ));
    }
    if max {
        options = options.batch(thinkthen::BatchSetting::Max);
    }
    // SAFETY: counted surface bytes obey the controls descriptor's extent.
    let surface = unsafe { read::string(c.surface) }?;
    if !surface.is_empty() {
        options = options.surface(
            surface
                .parse()
                .map_err(|_| Failure::usage("unknown surface"))?,
        );
    }
    Ok(options)
}
macro_rules! call {
    ($name:ident, $kind:literal) => {
        /// Execute the named judgment with typed inputs and immutable complete results.
        /// # Safety
        /// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
        #[unsafe(no_mangle)]
        pub(crate) unsafe extern "C" fn $name(
            engine: *const Door,
            question: *const QuestionHandle,
            source: *const SourceHandle,
            controls: *const ControlsV1,
            out: *mut *mut ResultHandle,
        ) -> i32 {
            // SAFETY: unchanged inputs go through the common validated, guarded edge.
            unsafe { run(engine, $kind, question, source, controls, out) }
        }
    };
}
call!(thinkthen_decide_complete, 1);
call!(thinkthen_choose_complete, 2);
call!(thinkthen_tag_complete, 3);
call!(thinkthen_score_complete, 4);
call!(thinkthen_filter_complete, 5);
call!(thinkthen_rank_complete, 6);
call!(thinkthen_find_complete, 7);
call!(thinkthen_annotate_complete, 8);
call!(thinkthen_recognize_complete, 9);
call!(thinkthen_relate_complete, 10);
/// Free an immutable result after every borrowed view is finished. NULL is harmless.
/// # Safety
/// A nonnull result was allocated by this library and has not been freed.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_free(owner: *mut ResultHandle) {
    failures::guard(None, (), || {
        if !owner.is_null() {
            // SAFETY: only this library frees its live owner, once.
            drop(unsafe { Box::from_raw(owner) });
        }
    });
}
unsafe fn view<T: Copy>(
    owner: *const ResultHandle,
    out: *mut T,
    read_view: impl FnOnce(&ResultHandle) -> Option<T>,
) -> i32 {
    failures::guard(None, DEFECT, || {
        if out.is_null() {
            return USAGE;
        }
        // SAFETY: caller supplies NULL or a live immutable result owner.
        let Some(owner) = (unsafe { owner.as_ref() }) else {
            return USAGE;
        };
        let Some(value) = read_view(owner) else {
            return USAGE;
        };
        // SAFETY: a nonnull initialized output is writable; only success publishes.
        unsafe {
            *out = value;
        }
        OK
    })
}
/// Borrow the immutable complete invocation summary.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_summary(
    owner: *const ResultHandle,
    out: *mut SummaryV1,
) -> i32 {
    // SAFETY: view validates NULLs before borrowing/publishing.
    unsafe { view(owner, out, |r| Some(r.summary)) }
}
/// Borrow an actual question or completed-row observation in native event order.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_observation(
    owner: *const ResultHandle,
    at: usize,
    out: *mut ObservationV1,
) -> i32 {
    // SAFETY: view validates owner/output and get validates the ordinal.
    unsafe { view(owner, out, |r| r.observations.get(at).copied()) }
}
macro_rules! row {
    ($name:ident, $ty:ident, $kind:literal, $arm:ident) => {
        /// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
        /// # Safety
        /// Owner and output obey the header's lifetime/storage contract.
        #[unsafe(no_mangle)]
        pub(crate) unsafe extern "C" fn $name(
            owner: *const ResultHandle,
            at: usize,
            out: *mut $ty,
        ) -> i32 {
            // SAFETY: native construction set the union arm from function; owner is immutable.
            unsafe {
                view(owner, out, |r| {
                    r.rows
                        .get(at)
                        .filter(|row| row.function == $kind)
                        .map(|row| row.data.$arm)
                })
            }
        }
    };
}
row!(thinkthen_result_decide, DecideViewV1, 1, decide);
row!(thinkthen_result_choose, ChooseViewV1, 2, choose);
row!(thinkthen_result_tag, TagViewV1, 3, tag);
row!(thinkthen_result_score, ScoreViewV1, 4, score);
row!(thinkthen_result_filter, FilterViewV1, 5, filter);
row!(thinkthen_result_rank, RankViewV1, 6, rank);
row!(thinkthen_result_find, FindViewV1, 7, find);
row!(thinkthen_result_annotate, AnnotateViewV1, 8, annotate);
row!(thinkthen_result_recognize, RecognizeViewV1, 9, recognize);
row!(thinkthen_result_relate, RelateViewV1, 10, relate);
/// Clone the calling thread's saved failure without altering its sticky error slot.
/// # Safety
/// Engine is NULL or live; out is writable when nonnull.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_error_complete(
    engine: *const Door,
    out: *mut *mut ResultHandle,
) -> i32 {
    failures::guard(None, DEFECT, || {
        if out.is_null() {
            return USAGE;
        }
        // SAFETY: engine is NULL or a live owner through snapshot creation.
        let held = unsafe { super::held(engine) };
        match failures::snapshot(held).map(complete::failure).transpose() {
            Ok(result) => {
                // SAFETY: writable output validated; snapshot creation leaves saved error untouched.
                unsafe {
                    *out = result.map_or(std::ptr::null_mut(), |r| Box::into_raw(Box::new(r)));
                }
                OK
            }
            Err(_) => DEFECT,
        }
    })
}

/// Borrow additive full row details, including partial usage and original inputs.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_details(
    owner: *const ResultHandle,
    at: usize,
    out: *mut DetailsV1,
) -> i32 {
    // SAFETY: view validates owner/output and get validates the ordinal.
    unsafe { view(owner, out, |r| r.row_details.get(at).copied()) }
}
/// Borrow the actual resolved observed question and all its native identities.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_observation_details(
    owner: *const ResultHandle,
    at: usize,
    out: *mut DetailsV1,
) -> i32 {
    // SAFETY: view validates owner/output and get validates the ordinal.
    unsafe { view(owner, out, |r| r.observation_details.get(at).copied()) }
}

/// Borrow the authored metadata of one complete row.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_question_author(
    owner: *const ResultHandle,
    at: usize,
    out: *mut crate::ffi::carriers::QuestionAuthorV1,
) -> i32 {
    // SAFETY: view validates owner/output and get validates the ordinal.
    unsafe { view(owner, out, |r| r.authors.get(at).copied()) }
}
/// Borrow one annotation member's actual authored metadata.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_member_author(
    owner: *const ResultHandle,
    at: usize,
    member: usize,
    out: *mut crate::ffi::carriers::QuestionAuthorV1,
) -> i32 {
    // SAFETY: view validates owner/output; both ordinals use checked access.
    unsafe {
        view(owner, out, |r| {
            r.member_authors
                .get(at)
                .and_then(|members| members.get(member))
                .copied()
        })
    }
}
/// Borrow one native observation's actual authored metadata.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_observation_author(
    owner: *const ResultHandle,
    at: usize,
    out: *mut crate::ffi::carriers::QuestionAuthorV1,
) -> i32 {
    // SAFETY: view validates owner/output and get validates the ordinal.
    unsafe { view(owner, out, |r| r.observation_authors.get(at).copied()) }
}
