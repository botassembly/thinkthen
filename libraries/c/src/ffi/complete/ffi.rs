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
    ObservationV1, RankViewV1, RecognizeViewV1, RelateViewV1, RowObservationV1, ScoreViewV1,
    SummaryV1, TagViewV1,
};
use crate::ffi::current::read;
use crate::ffi::values as abi;
use crate::ffi::values::{
    THINKTHEN_FUNCTION_ANNOTATE_V1 as ANNOTATE, THINKTHEN_FUNCTION_CHOOSE_V1 as CHOOSE,
    THINKTHEN_FUNCTION_DECIDE_V1 as DECIDE, THINKTHEN_FUNCTION_FILTER_V1 as FILTER,
    THINKTHEN_FUNCTION_FIND_V1 as FIND, THINKTHEN_FUNCTION_RANK_V1 as RANK,
    THINKTHEN_FUNCTION_RECOGNIZE_V1 as RECOGNIZE, THINKTHEN_FUNCTION_RELATE_V1 as RELATE,
    THINKTHEN_FUNCTION_SCORE_V1 as SCORE, THINKTHEN_FUNCTION_TAG_V1 as TAG,
};
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
pub(super) unsafe fn options<'a>(
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

/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_decide_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, DECIDE, question, source, controls, out) }
}
/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_choose_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, CHOOSE, question, source, controls, out) }
}
/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_tag_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, TAG, question, source, controls, out) }
}
/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_score_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, SCORE, question, source, controls, out) }
}
/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_filter_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, FILTER, question, source, controls, out) }
}
/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_rank_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, RANK, question, source, controls, out) }
}
/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_find_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, FIND, question, source, controls, out) }
}
/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_annotate_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, ANNOTATE, question, source, controls, out) }
}
/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_recognize_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, RECOGNIZE, question, source, controls, out) }
}
/// Execute the named judgment with typed inputs and immutable complete results.
/// # Safety
/// Every pointer obeys include/thinkthen.h's counted-storage and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_relate_complete(
    engine: *const Door,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
    // SAFETY: unchanged inputs go through the common validated, guarded edge.
    unsafe { run(engine, RELATE, question, source, controls, out) }
}
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
) -> std::ffi::c_int {
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
) -> std::ffi::c_int {
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
) -> std::ffi::c_int {
    // SAFETY: view validates owner/output and get validates the ordinal.
    unsafe { view(owner, out, |r| r.observations.get(at).copied()) }
}
/// Borrow a final row at its output position, retaining its original input index.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
/// Borrow a final row by output position, retaining its original input index.
/// Nested views borrow until result_free. NULL owner/output or an out-of-range
/// position returns EUSAGE without writing output.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_row(
    owner: *const ResultHandle,
    at: usize,
    out: *mut RowObservationV1,
) -> std::ffi::c_int {
    // SAFETY: view validates owner/output; get validates the final output position.
    unsafe { view(owner, out, |r| r.rows.get(at).copied()) }
}

/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_decide(
    owner: *const ResultHandle,
    at: usize,
    out: *mut DecideViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == DECIDE)
                .map(|row| row.data.decide)
        })
    }
}
/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_choose(
    owner: *const ResultHandle,
    at: usize,
    out: *mut ChooseViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == CHOOSE)
                .map(|row| row.data.choose)
        })
    }
}
/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_tag(
    owner: *const ResultHandle,
    at: usize,
    out: *mut TagViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == TAG)
                .map(|row| row.data.tag)
        })
    }
}
/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_score(
    owner: *const ResultHandle,
    at: usize,
    out: *mut ScoreViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == SCORE)
                .map(|row| row.data.score)
        })
    }
}
/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_filter(
    owner: *const ResultHandle,
    at: usize,
    out: *mut FilterViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == FILTER)
                .map(|row| row.data.filter)
        })
    }
}
/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_rank(
    owner: *const ResultHandle,
    at: usize,
    out: *mut RankViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == RANK)
                .map(|row| row.data.rank)
        })
    }
}
/// Number of saved member judgments; simple rank has none.
/// # Safety
/// Owner and output obey the installed header's storage contract.
/// Saved rank sets retain every ordered member. Simple rank has zero members.
/// Each member view has its own answer ID, complete probabilities and metadata.
/// Views borrow the result; invalid indices return EUSAGE without writing output.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_rank_member_count(
    owner: *const ResultHandle,
    row: usize,
    out: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: view validates the live owner/output and row kind before publication.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(row)
                .filter(|v| v.function == RANK)
                .map(|_| r.rank_members.get(row).map_or(0, Vec::len))
        })
    }
}
/// Borrow an actual saved rank member with its independent identity and probabilities.
/// # Safety
/// Owner and output obey the installed header's storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_rank_member(
    owner: *const ResultHandle,
    row: usize,
    member: usize,
    out: *mut RankViewV1,
) -> std::ffi::c_int {
    // SAFETY: view validates NULLs and checked indices before publication.
    unsafe {
        view(owner, out, |r| {
            r.rank_members.get(row)?.get(member).copied()
        })
    }
}

/// Borrow full member details, including partial usage and source batch sizes.
/// # Safety
/// Owner and output obey the installed header's storage contract.
/// Full member details retain independently present usage dimensions and source
/// batch sizes. Nested views borrow the result until result_free. NULL arguments,
/// wrong function, and invalid row/member indices return EUSAGE without writing.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_rank_member_details(
    owner: *const ResultHandle,
    row: usize,
    member: usize,
    out: *mut DetailsV1,
) -> std::ffi::c_int {
    // SAFETY: view validates NULLs and checked indices before publication.
    unsafe {
        view(owner, out, |r| {
            r.rows.get(row).filter(|row| row.function == RANK)?;
            r.rank_member_details.get(row)?.get(member).copied()
        })
    }
}

/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_find(
    owner: *const ResultHandle,
    at: usize,
    out: *mut FindViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == FIND)
                .map(|row| row.data.find)
        })
    }
}
/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_annotate(
    owner: *const ResultHandle,
    at: usize,
    out: *mut AnnotateViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == ANNOTATE)
                .map(|row| row.data.annotate)
        })
    }
}
/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_recognize(
    owner: *const ResultHandle,
    at: usize,
    out: *mut RecognizeViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == RECOGNIZE)
                .map(|row| row.data.recognize)
        })
    }
}
/// Borrow one complete row; wrong functions and out-of-range ordinals return Usage.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_relate(
    owner: *const ResultHandle,
    at: usize,
    out: *mut RelateViewV1,
) -> std::ffi::c_int {
    // SAFETY: native construction set the union arm from function; owner is immutable.
    unsafe {
        view(owner, out, |r| {
            r.rows
                .get(at)
                .filter(|row| row.function == RELATE)
                .map(|row| row.data.relate)
        })
    }
}
/// Clone the calling thread's saved failure without altering its sticky error slot.
/// # Safety
/// Engine is NULL or live; out is writable when nonnull.
/// Snapshot the calling thread's last failure for engine, or its failed-build
/// slot for engine=NULL. Never clears/replaces that slot.
/// With no saved failure: returns OK and writes *out=NULL (no allocation).
/// With a pre-start failure: returns OK and writes an owned FAILURE result;
/// facts/attempts are absent. With a started failure: returns OK and writes an
/// owned FAILURE result with final facts and opt-in attempts, even if []
/// because no send occurred. Both failure snapshots have absent schema,
/// answer_id and function, count=observation_count=0, and meta.present=0.
/// error is present; no origin/model/request/answer provenance is invented.
/// The return value describes snapshot creation, not the saved failure code.
/// out=NULL returns EUSAGE without changing the saved failure.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_error_complete(
    engine: *const Door,
    out: *mut *mut ResultHandle,
) -> std::ffi::c_int {
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
) -> std::ffi::c_int {
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
) -> std::ffi::c_int {
    // SAFETY: view validates owner/output and get validates the ordinal.
    unsafe { view(owner, out, |r| r.observation_details.get(at).copied()) }
}

/// Borrow the authored metadata of one complete row.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
/// Borrow native author snapshots until result_free. Row/member ordinals
/// follow the existing named result accessors and annotation member order.
/// Set envelopes and generated internal questions carry absent author fields.
/// Invalid owner/output/ordinal or a member on a non-annotation row is Usage
/// and leaves output unchanged. Observation includes both question/row events.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_question_author(
    owner: *const ResultHandle,
    at: usize,
    out: *mut crate::ffi::carriers::QuestionAuthorV1,
) -> std::ffi::c_int {
    // SAFETY: view validates owner/output and get validates the ordinal.
    unsafe { view(owner, out, |r| r.authors.get(at).copied()) }
}
/// Borrow one annotation or rank member's actual authored metadata.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_member_author(
    owner: *const ResultHandle,
    at: usize,
    member: usize,
    out: *mut crate::ffi::carriers::QuestionAuthorV1,
) -> std::ffi::c_int {
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
) -> std::ffi::c_int {
    // SAFETY: view validates owner/output and get validates the ordinal.
    unsafe { view(owner, out, |r| r.observation_authors.get(at).copied()) }
}

/// Borrow native located recognition spans; absent for unlocated input.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_source_recognition(
    owner: *const ResultHandle,
    row: usize,
    out: *mut crate::ffi::carriers::SourceRecognitionV1,
) -> std::ffi::c_int {
    // SAFETY: checked row kind and indices select constructed immutable storage.
    unsafe {
        view(owner, out, |r| {
            r.rows.get(row).filter(|v| v.function == RECOGNIZE)?;
            r.source_recognition.get(row).copied()
        })
    }
}
/// Borrow native expanded relation occurrences; absent for unlocated input.
/// # Safety
/// Owner and output obey the header's lifetime/storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_source_relations(
    owner: *const ResultHandle,
    row: usize,
    out: *mut crate::ffi::carriers::SourceRelationsV1,
) -> std::ffi::c_int {
    // SAFETY: checked row kind and indices select constructed immutable storage.
    unsafe {
        view(owner, out, |r| {
            r.rows.get(row).filter(|v| v.function == RELATE)?;
            r.source_relations.get(row).copied()
        })
    }
}

/// Borrow one recognition row's copied authored task wording.
/// # Safety
/// Owner and output obey the header's lifetime and full-sized storage contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_result_recognition_task_v1(
    owner: *const ResultHandle,
    at: usize,
    out: *mut crate::ffi::carriers::RecognitionTaskV1,
) -> std::ffi::c_int {
    // SAFETY: view validates owner/output; checked row and sidecar access precede publication.
    unsafe {
        view(owner, out, |r| {
            (r.rows.get(at)?.function == abi::THINKTHEN_FUNCTION_RECOGNIZE_V1)
                .then(|| r.recognition_tasks.get(at).copied())
                .flatten()
        })
    }
}
