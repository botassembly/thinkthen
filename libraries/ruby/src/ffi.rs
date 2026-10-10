//! The one module that touches Ruby: the classes, the crossing, and the
//! conversions both ways.
#![allow(
    unsafe_code,
    reason = "the crossing releases Ruby's lock and reads its interrupts through rb-sys"
)]
#![allow(
    missing_docs,
    reason = "magnus::init emits an undocumented Init_ entry point"
)]

use std::ffi::c_void;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Instant;

use magnus::prelude::*;
use magnus::{
    Error, Exception, ExceptionClass, RModule, Ruby, TryConvert, Value, function, method,
};
use thinkthen::{
    BatchSetting, CancelToken, Engine, Entity, LoadedQuestion, Question, QuestionSet, Recognize,
    Relate,
};

use crate::call::Ask;

mod complete;
mod engine;
mod native_result;
mod plan;
mod question_file;
mod register;
mod request;
mod result;
mod results_generated;
use crate::{Controls, Crossing, Fault, Handoff, Settings, Taken, class_name, guarded, start};
use engine::new_engine;
use result::{
    PANICKED, attach_completion, details_value, facts_value, output, protected_completion,
    ruby_json,
};

#[magnus::wrap(class = "ThinkThen::Cancel", free_immediately, size)]
#[derive(Debug, Default)]
pub(crate) struct CancelValue(CancelToken);

impl CancelValue {
    fn cancel(&self) {
        self.0.cancel();
    }

    fn is_cancelled(&self) -> bool {
        self.0.is_cancelled()
    }
}

#[magnus::wrap(class = "ThinkThen::Question", free_immediately, size)]
#[derive(Debug)]
pub(crate) struct QuestionValue {
    json: String,
    loaded: LoadedQuestion,
}

impl QuestionValue {
    fn json(&self) -> String {
        self.json.clone()
    }
}

#[magnus::wrap(class = "ThinkThen::QuestionSet", free_immediately, size)]
#[derive(Debug)]
pub(crate) struct SetValue(QuestionSet);

impl SetValue {
    fn names(&self) -> Vec<String> {
        self.0.members().map(|(name, _)| name.to_owned()).collect()
    }
}

#[magnus::wrap(class = "ThinkThen::Native::Engine", free_immediately, size)]
#[derive(Debug)]
pub(crate) struct EngineValue {
    engine: Engine,
}

#[magnus::wrap(class = "ThinkThen::Native::Completion", free_immediately, size)]
#[derive(Debug)]
struct CompletionValue(Arc<Handoff>);

impl CompletionValue {
    fn done(&self) -> bool {
        !matches!(self.0.take(), Taken::Waiting)
    }

    fn result(ruby: &Ruby, rb_self: &Self, timeout: Option<f64>) -> Result<Option<Value>, Error> {
        let timeout = match timeout {
            Some(value) if !value.is_finite() || value < 0.0 => {
                return Err(raise(
                    ruby,
                    Fault::usage("completion timeout takes a nonnegative number"),
                ));
            }
            value => value,
        };
        let started = Instant::now();
        let pointer = Arc::as_ptr(&rb_self.0).cast_mut().cast::<c_void>();
        loop {
            match rb_self.0.take() {
                Taken::Ready(terminal) => return protected_completion(ruby, &terminal).map(Some),
                Taken::Closed => return ruby_json(ruby, &PANICKED).map(Some),
                Taken::Waiting => {}
            }
            if timeout.is_some_and(|value| started.elapsed().as_secs_f64() >= value) {
                return Ok(None);
            }
            magnus::rb_sys::protect(|| {
                // SAFETY: the Arc keeps this handoff alive across the GVL-free wait.
                unsafe {
                    rb_sys::rb_thread_call_without_gvl(
                        Some(wait_for),
                        pointer,
                        Some(wake),
                        pointer,
                    );
                    rb_sys::rb_thread_check_ints();
                }
                rb_sys::Qnil.into()
            })?;
        }
    }
}

fn batch_of(ruby: &Ruby, value: Value) -> Result<Option<BatchSetting>, Error> {
    if value.is_nil() {
        return Ok(None);
    }
    if let Ok(text) = String::try_convert(value)
        && text == "max"
    {
        return Ok(Some(BatchSetting::Max));
    }
    if let Ok(number) = i64::try_convert(value)
        && let Ok(number) = usize::try_from(number)
        && let Some(number) = NonZeroUsize::new(number)
    {
        return Ok(Some(BatchSetting::Records(number)));
    }
    Err(raise(
        ruby,
        Fault::usage("batch takes max or a whole number of at least 1"),
    ))
}

/// Raise one fault as its `ThinkThen` class, with the kind and retry
/// signal riding the exception.
fn raise(ruby: &Ruby, fault: Fault) -> Error {
    let built = || -> Result<Error, Error> {
        let module: RModule = ruby.class_object().const_get("ThinkThen")?;
        let class: ExceptionClass = module.const_get(class_name(fault.kind))?;
        let made: Value = class.funcall(
            "new",
            (fault.message.as_str(), fault.kind.name(), fault.retryable),
        )?;
        made.funcall::<_, _, Value>(
            "instance_variable_set",
            (
                "@facts",
                fault
                    .facts
                    .as_ref()
                    .map(|facts| facts_value(ruby, facts))
                    .transpose()?,
            ),
        )?;
        made.funcall::<_, _, Value>(
            "instance_variable_set",
            (
                "@details",
                if fault.facts.is_some() {
                    Some(details_value(ruby, &fault.details)?)
                } else {
                    None
                },
            ),
        )?;
        if let Some(snapshot) = fault.native_complete {
            made.funcall::<_, _, Value>("instance_variable_set", ("@native_complete", snapshot))?;
        }
        Exception::from_value(made)
            .map(Error::from)
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "thinkthen failed"))
    };
    built().unwrap_or_else(|error| error)
}

fn checked<T>(ruby: &Ruby, held: Result<T, Fault>) -> Result<T, Error> {
    held.map_err(|fault| raise(ruby, fault))
}

/// Run a conversion under `rb_protect`, so a raise inside it returns as
/// `Err` and never jumps across the Rust frames that hold the answer.
fn protected(ruby: &Ruby, build: impl FnOnce() -> Result<Value, Error>) -> Result<Value, Error> {
    let mut carried: Option<Result<Value, Error>> = None;
    let slot = &mut carried;
    magnus::rb_sys::protect(move || {
        *slot = Some(build());
        rb_sys::Qnil.into()
    })?;
    carried.unwrap_or_else(|| {
        Err(raise(
            ruby,
            Fault::of(
                thinkthen::ErrorKind::Defect,
                "defect: a conversion produced nothing",
            ),
        ))
    })
}

/// One wait slice with the lock released.
unsafe extern "C" fn wait_for(pointer: *mut c_void) -> *mut c_void {
    // SAFETY: `cross` passes a live `Handoff` and keeps it alive across the call.
    let handoff = unsafe { &*pointer.cast::<Handoff>() };
    handoff.wait();
    std::ptr::null_mut()
}

/// Ruby's unblock function. It only wakes the wait. Ruby runs the interrupt
/// itself once the lock is held again.
unsafe extern "C" fn wake(pointer: *mut c_void) {
    // SAFETY: as `wait_for`.
    let handoff = unsafe { &*pointer.cast::<Handoff>() };
    handoff.wake();
}

/// Block asynchronous signals on the worker, so the kernel delivers them to
/// a Ruby thread and never interrupts a send. Faults stay deliverable.
fn quiet_signals() {
    let mut set = std::mem::MaybeUninit::<libc::sigset_t>::uninit();
    // SAFETY: `sigfillset` initializes the set before any other call reads it.
    unsafe {
        libc::sigfillset(set.as_mut_ptr());
        for fault in [
            libc::SIGSEGV,
            libc::SIGBUS,
            libc::SIGFPE,
            libc::SIGILL,
            libc::SIGSYS,
            libc::SIGTRAP,
            libc::SIGABRT,
        ] {
            libc::sigdelset(set.as_mut_ptr(), fault);
        }
        libc::pthread_sigmask(libc::SIG_BLOCK, set.as_ptr(), std::ptr::null_mut());
    }
}

/// Wait for the worker in slices with the lock released and never re-taken
/// inside the wait. Between slices, with the lock held, read Ruby's pending
/// interrupts under `rb_protect`, then the caller's token and the call's own.
/// Any stop fires the call's own token and returns at once. The worker
/// finishes its sent requests alone, and its answer is dropped.
fn cross(
    handoff: &Arc<Handoff>,
    own: &CancelToken,
    caller: Option<&CancelToken>,
) -> Result<Crossing, Error> {
    let pointer = Arc::as_ptr(handoff).cast_mut().cast::<c_void>();
    loop {
        let heard = magnus::rb_sys::protect(|| {
            // SAFETY: `pointer` outlives the call, and both functions only
            // touch the handoff's mutex and condition variable.
            unsafe {
                rb_sys::rb_thread_call_without_gvl(Some(wait_for), pointer, Some(wake), pointer);
                rb_sys::rb_thread_check_ints();
            }
            rb_sys::Qnil.into()
        });
        if let Err(raised) = heard {
            own.cancel();
            return Err(raised);
        }
        if own.is_cancelled() || caller.is_some_and(CancelToken::is_cancelled) {
            own.cancel();
            return Ok(Err(Fault::cancelled()));
        }
        match handoff.take() {
            Taken::Ready(answer) => return Ok(Ok(answer)),
            Taken::Closed => {
                return Ok(Err(Fault::of(
                    thinkthen::ErrorKind::Defect,
                    "defect: the call's worker ended with no answer",
                )));
            }
            Taken::Waiting => {}
        }
    }
}

fn question_of(subject: Value) -> Result<LoadedQuestion, Error> {
    <&QuestionValue>::try_convert(subject).map(|question| question.loaded.clone())
}

/// Read one call's inputs into owned Rust values on the Ruby thread.
fn ask(ruby: &Ruby, verb: &str, subject: Value, input: Value) -> Result<Ask, Error> {
    let text = || String::try_convert(input);
    let records = || Vec::<String>::try_convert(input);
    Ok(match verb {
        "files" => Ask::Files(String::try_convert(subject)?, text()?),
        "decide" => Ask::Decide(question_of(subject)?, text()?),
        "details" => Ask::Details(question_of(subject)?, text()?),
        "score" => Ask::Score(question_of(subject)?, text()?),
        "decide_many" => Ask::DecideMany(question_of(subject)?, records()?),
        "many" => Ask::Many(question_of(subject)?, records()?),
        "filter" => Ask::Filter(question_of(subject)?, records()?),
        "rank" => Ask::Rank(String::try_convert(subject)?, records()?),
        "find" | "find_none" => Ask::Find(
            String::try_convert(subject)?,
            verb == "find_none",
            records()?,
        ),
        "annotate" => Ask::Annotate(<&SetValue>::try_convert(subject)?.0.clone(), records()?),
        "recognize" => {
            let spec = String::try_convert(subject)?;
            Ask::Recognize(
                checked(ruby, Recognize::from_json(&spec).map_err(Fault::from))?,
                text()?,
            )
        }
        "relate" => {
            let spec = String::try_convert(subject)?;
            let ask = checked(ruby, Relate::from_json(&spec).map_err(Fault::from))?;
            let pairs = Vec::<(String, String)>::try_convert(input)?;
            let entities = pairs
                .iter()
                .map(|(name, kind)| Entity::new(name, kind).map_err(Fault::from))
                .collect::<Result<Vec<_>, _>>();
            Ask::Relate(ask, checked(ruby, entities)?)
        }
        _ => return Err(raise(ruby, Fault::usage(format!("{verb} is not a call")))),
    })
}

impl EngineValue {
    /// One call: read the inputs, start the worker, wait, and convert.
    #[expect(
        clippy::too_many_arguments,
        reason = "magnus maps each Ruby argument to one parameter"
    )]
    fn call(
        ruby: &Ruby,
        rb_self: &Self,
        verb: String,
        subject: Value,
        input: Value,
        own: &CancelValue,
        caller: Option<&CancelValue>,
        deadline_ms: Option<i64>,
        batch: Option<Value>,
        context: Option<String>,
    ) -> Result<Value, Error> {
        let asked = if verb == "complete" {
            Ask::Complete(String::try_convert(input)?)
        } else {
            ask(ruby, &verb, subject, input)?
        };
        let batch = batch
            .map(|value| batch_of(ruby, value))
            .transpose()?
            .flatten();
        let caller = caller.map(|token| &token.0);
        if own.0.is_cancelled() || caller.is_some_and(CancelToken::is_cancelled) {
            return Err(raise(ruby, Fault::cancelled()));
        }
        let handoff = checked(
            ruby,
            start(
                rb_self.engine.clone(),
                asked,
                own.0.clone(),
                Controls {
                    deadline_ms,
                    batch,
                    context,
                },
                quiet_signals,
            ),
        )?;
        let answer = match cross(&handoff, &own.0, caller) {
            Ok(answer) => answer,
            Err(error) => {
                attach_completion(ruby, &error, &handoff);
                return Err(error);
            }
        };
        match answer {
            Ok(terminal) => protected(ruby, || match &*terminal {
                Ok(done) => Ok(ruby.into_value((
                    output(ruby, &done.value),
                    facts_value(ruby, &done.facts)?,
                    details_value(ruby, &done.details)?,
                ))),
                Err(fault) => Err(raise(ruby, fault.clone())),
            }),
            Err(fault) => {
                let error = raise(ruby, fault);
                attach_completion(ruby, &error, &handoff);
                Err(error)
            }
        }
    }

    fn usage(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        ruby_json(ruby, &rb_self.engine.usage())
    }
}

fn default_engine(ruby: &Ruby) -> Result<EngineValue, Error> {
    let engine = thinkthen::default_engine().map_err(Fault::from);
    checked(ruby, engine).map(|engine| EngineValue {
        engine: engine.clone(),
    })
}

fn question(ruby: &Ruby, json: String) -> Result<QuestionValue, Error> {
    let loaded = checked(ruby, Question::from_json(&json).map_err(Fault::from))?;
    Ok(QuestionValue { json, loaded })
}

fn file_question(ruby: &Ruby, path: String) -> Result<QuestionValue, Error> {
    let (json, loaded) = checked(ruby, question_file::read(&path))?;
    Ok(QuestionValue { json, loaded })
}

fn file_plan(ruby: &Ruby, path: String, verb: String) -> Result<String, Error> {
    checked(ruby, question_file::read_plan(&path, &verb))
}

fn set_json(ruby: &Ruby, json: String) -> Result<SetValue, Error> {
    checked(ruby, QuestionSet::from_json(&json).map_err(Fault::from)).map(SetValue)
}

fn set_file(ruby: &Ruby, path: String) -> Result<SetValue, Error> {
    checked(ruby, QuestionSet::load(path).map_err(Fault::from)).map(SetValue)
}

struct Poll<'a> {
    session: &'a crate::call::complete::stream::Session,
    result: Option<Result<Option<String>, thinkthen::Error>>,
}
unsafe extern "C" fn poll(data: *mut c_void) -> *mut c_void {
    // SAFETY: the calling frame keeps this owned slot and session alive until this returns.
    let slot = unsafe { &mut *data.cast::<Poll<'_>>() };
    slot.result = thinkthen::contained(|| slot.session.poll());
    std::ptr::null_mut()
}
unsafe extern "C" fn cancel(data: *mut c_void) {
    // SAFETY: Ruby's unblock callback receives the same still-live slot.
    unsafe { &*data.cast::<Poll<'_>>() }.session.cancel();
}
fn complete_poll(
    ruby: &Ruby,
    session: &crate::call::complete::stream::Session,
) -> Result<Result<Option<String>, thinkthen::Error>, Error> {
    let mut slot = Poll {
        session,
        result: None,
    };
    let pointer = std::ptr::from_mut(&mut slot).cast::<c_void>();
    magnus::rb_sys::protect(|| {
        // SAFETY: the frame owns the slot and both callbacks avoid Ruby.
        unsafe {
            rb_sys::rb_thread_call_without_gvl(Some(poll), pointer, Some(cancel), pointer);
            rb_sys::rb_thread_check_ints();
        }
        rb_sys::Qnil.into()
    })?;
    slot.result
        .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "complete poll panicked"))
}
