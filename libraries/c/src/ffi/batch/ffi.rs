//! Stable owned backing for a native lazy batch; all lifetime extension stays here.
#![allow(
    unsafe_code,
    reason = "the C batch owns stable backing and follows its installed engine lifetime contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::failures::{self, DEFECT, Failure, FailureSnapshot, Held, OK, USAGE};
use crate::ffi::{carriers::ControlsV1, current::read};
use crate::{
    Door,
    complete::{self, ResultHandle, streaming},
    current::{Content, QuestionHandle, SourceHandle},
};
use std::sync::{Arc, Mutex, PoisonError};
use thinkthen::{CancelToken, Facts, RecordObservation};
type Observer = Box<dyn for<'r> Fn(RecordObservation<'r>) + Send + Sync>;
struct Owners {
    question: QuestionHandle,
    source: SourceHandle,
    context: Option<String>,
    cancel: Option<CancelToken>,
    controls: ControlsV1,
    surface: thinkthen::Surface,
    observer: Observer,
}
/// A same-thread native batch. Field order drops/joins the batch before its backing.
pub(crate) struct BatchHandle {
    native: Option<streaming::NativeBatch<'static>>,
    _owners: Box<Owners>,
    held: &'static Held,
    events: streaming::Events,
    thread: std::thread::ThreadId,
    kind: u32,
    facts: Option<Facts>,
    error: Option<FailureSnapshot>,
    ended: bool,
}
impl std::fmt::Debug for BatchHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatchHandle")
            .field("ended", &self.ended)
            .finish_non_exhaustive()
    }
}
impl BatchHandle {
    fn same_thread(&self) -> Result<(), Failure> {
        if self.thread == std::thread::current().id() {
            Ok(())
        } else {
            Err(Failure::usage("a batch stays on its creating thread"))
        }
    }
}
fn backing(
    question: QuestionHandle,
    source: SourceHandle,
    context: Option<String>,
    cancel: Option<CancelToken>,
    controls: ControlsV1,
    surface: thinkthen::Surface,
) -> (Box<Owners>, streaming::Events) {
    let events = Arc::new(Mutex::new(Vec::new()));
    let observed = Arc::clone(&events);
    let observer: Observer = Box::new(move |event| {
        observed
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(event.to_owned())
    });
    (
        Box::new(Owners {
            question,
            source,
            context,
            cancel,
            controls,
            surface,
            observer,
        }),
        events,
    )
}

unsafe fn start(
    engine: *const Door,
    kind: u32,
    question: *const QuestionHandle,
    source: *const SourceHandle,
    controls: *const ControlsV1,
    out: *mut *mut BatchHandle,
) -> i32 {
    // SAFETY: engine remains live through batch_free, as the installed header requires.
    let Some(held) = (unsafe { super::held(engine) }) else {
        return USAGE;
    };
    failures::guard(Some(held), DEFECT, || {
        let result = (|| {
            read::required(out)?;
            // SAFETY: initialized descriptors and their counted buffers are readable until start returns.
            let (question, source, given) = unsafe {
                (
                    read::reference(question)?.clone(),
                    read::reference(source)?.clone(),
                    controls.as_ref().copied(),
                )
            };
            let context = given
                .map(|c| unsafe { read::optional_content(c.context) })
                .transpose()?
                .flatten();
            let context = match context {
                Some(Content::Text(text)) => Some(text),
                Some(Content::Json(_)) => {
                    return Err(Failure::usage("shared context requires text"));
                }
                None => None,
            };
            // SAFETY: a passed token is live during start; cloning shares its native cancellation flag.
            let cancel = given.and_then(|c| unsafe { c.cancel.as_ref() }).cloned();
            let mut controls = given.unwrap_or_else(|| ControlsV1 {
                deadline_ms: -1,
                ..ControlsV1::default()
            });
            // Validate/copy borrowed surface before replacing its pointers with the native C default.
            unsafe { super::complete::options(given.as_ref(), context.as_deref()) }?;
            // SAFETY: counted surface was validated by options above; retain only its parsed enum.
            let text = unsafe { read::string(controls.surface) }?;
            let surface = if text.is_empty() {
                thinkthen::Surface::C
            } else {
                text.parse()
                    .map_err(|_| Failure::usage("unknown surface"))?
            };
            controls.surface = Default::default();
            controls.context = Default::default();
            controls.cancel = std::ptr::null_mut();
            let (owners, events) = backing(question, source, context, cancel, controls, surface);
            // SAFETY: Box storage is stable and never mutated or exposed. native is dropped first,
            // including on early return/unwind, before owners. The engine has the caller's required lifetime.
            let stable: &'static Owners = unsafe { &*std::ptr::from_ref(owners.as_ref()) };
            let mut options = unsafe {
                super::complete::options(Some(&stable.controls), stable.context.as_deref())
            }?;
            if let Some(cancel) = &stable.cancel {
                options = options.cancel(cancel);
            }
            options = options
                .surface(stable.surface)
                .observe(stable.observer.as_ref());
            let native = streaming::begin(
                &held.engine,
                kind,
                &stable.question,
                &stable.source,
                options,
            )?;
            Ok(BatchHandle {
                native: Some(native),
                _owners: owners,
                held,
                events,
                thread: std::thread::current().id(),
                kind,
                facts: None,
                error: None,
                ended: false,
            })
        })();
        match held.settle(result) {
            Ok(batch) => {
                unsafe {
                    *out = Box::into_raw(Box::new(batch));
                }
                OK
            }
            Err(code) => code,
        }
    })
}
macro_rules! start {
    ($name:ident, $kind:literal) => {
        /// Start an owned same-thread native batch without collecting its source.
        /// # Safety
        /// The installed header specifies counted storage, engine and same-thread lifetimes.
        #[unsafe(no_mangle)]
        pub(crate) unsafe extern "C" fn $name(
            e: *const Door,
            q: *const QuestionHandle,
            s: *const SourceHandle,
            c: *const ControlsV1,
            out: *mut *mut BatchHandle,
        ) -> i32 {
            // SAFETY: unchanged pointers pass through the checked common edge.
            unsafe { start(e, $kind, q, s, c, out) }
        }
    };
}
start!(thinkthen_decide_batch_start, 1);
start!(thinkthen_choose_batch_start, 2);
start!(thinkthen_tag_batch_start, 3);
start!(thinkthen_score_batch_start, 4);
start!(thinkthen_filter_batch_start, 5);
start!(thinkthen_annotate_batch_start, 8);
/// Yield one independently owned result, or NULL on exhaustion, then one terminal error.
/// # Safety
/// Batch and output obey the installed same-thread/engine lifetime contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_batch_next(
    owner: *mut BatchHandle,
    out: *mut *mut ResultHandle,
) -> i32 {
    // SAFETY: live exclusive same-thread batch; wrong-thread calls return without pulling.
    let Some(batch) = (unsafe { owner.as_mut() }) else {
        return USAGE;
    };
    failures::guard(Some(batch.held), DEFECT, || {
        let mut terminal_failure = false;
        let result = (|| {
            batch.same_thread()?;
            read::required(out)?;
            let row = thinkthen::contained(|| {
                batch
                    .native
                    .as_mut()
                    .and_then(|native| native.next(&batch.events))
            })
            .unwrap_or_else(|| Some(Err(Failure::defect("a panic crossed the C batch door"))));
            if let Some(native) = &batch.native {
                batch.facts = native.facts().cloned();
            }
            terminal_failure = row.as_ref().is_some_and(Result::is_err);
            if row.is_none() || terminal_failure {
                batch.ended = true;
                drop(batch.native.take());
            }
            row.transpose()
                .map_err(|failure| failure.completed(batch.facts.as_ref()))
        })();
        match batch.held.settle(result) {
            Ok(row) => {
                unsafe {
                    *out = row.map_or(std::ptr::null_mut(), |row| Box::into_raw(Box::new(row)));
                }
                OK
            }
            Err(code) => {
                if terminal_failure {
                    batch.error = failures::snapshot(Some(batch.held));
                }
                code
            }
        }
    })
}
/// Snapshot joined final facts; before termination return Usage without writing output.
/// # Safety
/// Owner/output follow the installed lifetime contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_batch_facts(
    owner: *const BatchHandle,
    out: *mut *mut ResultHandle,
) -> i32 {
    let Some(batch) = (unsafe { owner.as_ref() }) else {
        return USAGE;
    };
    failures::guard(Some(batch.held), DEFECT, || {
        let result = (|| {
            batch.same_thread()?;
            read::required(out)?;
            if !batch.ended {
                return Err(Failure::usage("batch facts are final after termination"));
            }
            if let Some(error) = &batch.error {
                return complete::failure(error.clone());
            }
            let facts = batch
                .facts
                .as_ref()
                .ok_or_else(|| Failure::usage("this batch has no started-call facts"))?;
            streaming::final_result(batch.kind, facts)
        })();
        match batch.held.settle(result) {
            Ok(result) => {
                unsafe {
                    *out = Box::into_raw(Box::new(result));
                }
                OK
            }
            Err(code) => code,
        }
    })
}
/// Drop the native batch and join its workers before releasing owned input backing.
/// # Safety
/// NULL or a live batch on its creating thread, freed exactly once; engine remains live.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_batch_free(owner: *mut BatchHandle) {
    if owner.is_null() {
        return;
    }
    // SAFETY: borrowed live handle is inspected before ownership is taken.
    let batch = unsafe { &*owner };
    failures::guard(Some(batch.held), (), || {
        if let Err(error) = batch.same_thread() {
            batch.held.fail(error);
            return;
        }
        // SAFETY: same-thread unique ownership is returned exactly once.
        drop(unsafe { Box::from_raw(owner) });
    });
}
