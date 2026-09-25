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
use std::sync::Arc;

use magnus::prelude::*;
use magnus::{
    Error, Exception, ExceptionClass, RHash, RModule, Ruby, TryConvert, Value, function, method,
};
use thinkthen::{
    CancelToken, Engine, Entity, LoadedQuestion, Question, QuestionSet, Recognize, Relate,
};

use crate::call::{Ask, Output, Value as Judged};
use crate::{Fault, Handoff, Settings, Taken, class_name, guarded, start};

#[magnus::wrap(class = "ThinkThen::Cancel", free_immediately, size)]
#[derive(Debug, Default)]
struct CancelValue(CancelToken);

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
struct QuestionValue {
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
struct SetValue(QuestionSet);

impl SetValue {
    fn names(&self) -> Vec<String> {
        self.0.members().map(|(name, _)| name.to_owned()).collect()
    }
}

#[magnus::wrap(class = "ThinkThen::Native::Engine", free_immediately, size)]
#[derive(Debug)]
struct EngineValue {
    engine: Engine,
    most: Option<usize>,
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
) -> Result<Result<Output, Fault>, Error> {
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
        match handoff.take() {
            Taken::Ready(answer) => return Ok(answer),
            Taken::Closed => {
                return Ok(Err(Fault::of(
                    thinkthen::ErrorKind::Defect,
                    "defect: the call's worker ended with no answer",
                )));
            }
            Taken::Waiting => {}
        }
        if own.is_cancelled() || caller.is_some_and(CancelToken::is_cancelled) {
            own.cancel();
            return Ok(Err(Fault::cancelled()));
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
        "decide" => Ask::Decide(question_of(subject)?, text()?),
        "details" => Ask::Details(question_of(subject)?, text()?),
        "score" => Ask::Score(question_of(subject)?, text()?),
        "decide_many" => Ask::DecideMany(question_of(subject)?, records()?),
        "filter" => Ask::Filter(question_of(subject)?, records()?),
        "rank" => Ask::Rank(String::try_convert(subject)?, records()?),
        "find" => Ask::Find(String::try_convert(subject)?, records()?),
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

fn judged(ruby: &Ruby, value: Judged) -> Value {
    match value {
        Judged::Decision(answer) => ruby.into_value(answer),
        Judged::Choice(pick) => ruby.into_value(pick),
        Judged::Score(position) => ruby.into_value(position),
        Judged::Tags(labels) => ruby.into_value(labels),
    }
}

fn output(ruby: &Ruby, answer: Output) -> Value {
    match answer {
        Output::Answer(answer) => ruby.into_value(answer),
        Output::Score(position) => ruby.into_value(position),
        Output::Details(json, value, nearest) => {
            ruby.into_value((json, judged(ruby, value), nearest))
        }
        Output::Rows(rows) => ruby.into_value(rows),
        Output::Places(places) => ruby.into_value(places),
        Output::Ranked(ranked) => ruby.into_value(ranked),
        Output::Found(found) => ruby.into_value(found),
        Output::Json(json) => ruby.into_value(json),
        Output::JsonRows(rows) => ruby.into_value(rows),
    }
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
        deadline: Option<f64>,
    ) -> Result<Value, Error> {
        let asked = ask(ruby, &verb, subject, input)?;
        checked(ruby, crate::call::within(rb_self.most, &asked))?;
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
                deadline,
                quiet_signals,
            ),
        )?;
        let answer = cross(&handoff, &own.0, caller)?;
        protected(ruby, || match answer {
            Ok(answer) => Ok(output(ruby, answer)),
            Err(fault) => Err(raise(ruby, fault)),
        })
    }

    fn usage(ruby: &Ruby, rb_self: &Self) -> Result<RHash, Error> {
        let counts = rb_self.engine.usage();
        let hash = ruby.hash_new();
        hash.aset(ruby.to_symbol("requests_sent"), counts.requests_sent())?;
        hash.aset(ruby.to_symbol("cache_answers"), counts.cache_answers())?;
        hash.aset(ruby.to_symbol("input_tokens"), counts.input_tokens())?;
        hash.aset(ruby.to_symbol("output_tokens"), counts.output_tokens())?;
        Ok(hash)
    }
}

fn default_engine(ruby: &Ruby) -> Result<EngineValue, Error> {
    let engine = thinkthen::default_engine().map_err(Fault::from);
    checked(ruby, engine).map(|engine| EngineValue {
        engine: engine.clone(),
        most: None,
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "magnus maps each Ruby argument to one parameter"
)]
fn new_engine(
    ruby: &Ruby,
    base_url: Option<String>,
    model: Option<String>,
    throttle: Option<i64>,
    max_requests: Option<i64>,
    cache_at: Option<String>,
    no_cache: bool,
    cache_bytes: Option<i64>,
) -> Result<EngineValue, Error> {
    let settings = Settings {
        base_url,
        model,
        throttle,
        max_requests,
        cache_at,
        no_cache,
        cache_bytes,
    };
    checked(ruby, guarded(|| settings.build())).map(|(engine, most)| EngineValue { engine, most })
}

fn question(ruby: &Ruby, json: String) -> Result<QuestionValue, Error> {
    let loaded = checked(ruby, Question::from_json(&json).map_err(Fault::from))?;
    Ok(QuestionValue { json, loaded })
}

fn set_json(ruby: &Ruby, json: String) -> Result<SetValue, Error> {
    checked(ruby, QuestionSet::from_json(&json).map_err(Fault::from)).map(SetValue)
}

fn set_file(ruby: &Ruby, path: String) -> Result<SetValue, Error> {
    checked(ruby, QuestionSet::load(path).map_err(Fault::from)).map(SetValue)
}

/// Register the public value classes and the private `ThinkThen::Native`
/// functions. `lib/thinkthen.rb` defines the error classes and the verbs.
#[magnus::init(name = "thinkthen")]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("ThinkThen")?;
    let cancel = module.define_class("Cancel", ruby.class_object())?;
    cancel.define_singleton_method("new", function!(CancelValue::default, 0))?;
    cancel.define_method("cancel", method!(CancelValue::cancel, 0))?;
    cancel.define_method("cancelled?", method!(CancelValue::is_cancelled, 0))?;
    let question_class = module.define_class("Question", ruby.class_object())?;
    question_class.define_method("json", method!(QuestionValue::json, 0))?;
    let set_class = module.define_class("QuestionSet", ruby.class_object())?;
    set_class.define_method("names", method!(SetValue::names, 0))?;
    let native = module.define_module("Native")?;
    let engine = native.define_class("Engine", ruby.class_object())?;
    engine.define_method("call", method!(EngineValue::call, 6))?;
    engine.define_method("usage", method!(EngineValue::usage, 0))?;
    native.define_module_function("default_engine", function!(default_engine, 0))?;
    native.define_module_function("engine", function!(new_engine, 7))?;
    native.define_module_function("question", function!(question, 1))?;
    native.define_module_function("set_json", function!(set_json, 1))?;
    native.define_module_function("set_file", function!(set_file, 1))?;
    Ok(())
}
