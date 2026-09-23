//! The Ruby surface: one native door over the contract.
//!
//! This crate holds no rule: no threshold math, no retry, no sending. It
//! converts arguments, calls one contract function with the VM lock
//! released, converts the answer, and maps the six error kinds onto the
//! Ruby error classes. The Ruby-side glue in `lib/thinkthen.rb` owns the
//! keyword shapes and the record handling; everything that crosses into
//! Rust happens here, one crossing a call.
//!
//! The interrupt story, fifth-review shape: the engine call runs on its
//! own thread while the Ruby thread waits in slices with the VM lock
//! released and an unblock function set (see [`cross`]). Between slices
//! MRI runs the host's interrupts in ordinary Ruby-level frames: Ctrl-C,
//! `Thread#raise`, `Thread#kill`, and a raising trap fire the call's own
//! token, while a trapped signal that raises nothing and a spurious
//! `Thread#wakeup` leave the call alone. Sent requests finish, no new one
//! starts, and the raise surfaces when they have. The call's thread
//! blocks asynchronous signals, so no signal interrupts a send. The
//! Ruby-side watchdog (lib/thinkthen.rb) runs the tick and relays the
//! caller's token; the engine watches only the call's own token, so an
//! interrupt cannot cancel a sibling call.

// The magnus macros write the exported methods and the init function
// themselves; the doc comments above each item name them, and the
// generated wrappers cannot carry docs from here.
#![allow(missing_docs)]

use std::ffi::c_void;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use magnus::prelude::*;
use magnus::{
    class, define_module, exception, function, method, DataTypeFunctions, Error, ExceptionClass,
    IntoValue, RArray, RClass, RHash, TryConvert, TypedData, Value,
};
use thinkthen_contract::{
    relate_checked, Annotated, Answer, Cancel, Connector as ContractConnector, Details, Edge,
    Engine as ContractEngine, EngineConfig, Error as ContractError, ErrorKind, Judgment,
    Options, Question, QuestionSet, Ranked, Recognize, Recognized, Relate, Scored, Usage,
};
use thinkthen_standin::StandinConnector;

/// A built question: the file's JSON text beside its parsed engine form.
#[derive(Clone, Debug, TypedData)]
#[magnus(class = "ThinkThen::Question", free_immediately)]
pub struct QuestionValue {
    json: String,
    question: Question,
}

impl DataTypeFunctions for QuestionValue {}

/// A built question set: the set's JSON text beside its parsed form.
#[derive(Clone, Debug, TypedData)]
#[magnus(class = "ThinkThen::QuestionSet", free_immediately)]
pub struct SetValue {
    /// The JSON the set came from, held for the inspector; no code path
    /// reads it (clippy's dead-field finding, kept deliberately).
    #[allow(dead_code, reason = "held for the inspector")]
    json: String,
    set: QuestionSet,
}

impl DataTypeFunctions for SetValue {}

impl SetValue {
    /// The set's question names, in the file's own order, so the Ruby
    /// half can refuse a column collision before any request is paid.
    fn names(&self) -> Vec<String> {
        self.set.names().to_vec()
    }
}

/// A cancel token the Ruby side can fire from any thread.
#[derive(Clone, TypedData)]
#[derive(Debug)]
#[magnus(class = "ThinkThen::Cancel", free_immediately)]
pub struct CancelValue {
    token: Cancel,
}

impl DataTypeFunctions for CancelValue {}

impl CancelValue {
    /// Fire the token. Sent requests finish; no new one starts.
    fn cancel(&self) {
        self.token.cancel();
    }

    /// Whether the token has been fired. The Ruby-side watchdog polls
    /// this on the caller's token so it can stop the call through the
    /// call's own token, never by firing the caller's.
    fn is_fired(&self) -> bool {
        self.token.is_cancelled()
    }
}

/// The engine value: one value from the contract's connector, held for
/// the process. It holds no thread between calls.
#[derive(TypedData)]
#[magnus(class = "ThinkThen::Native::Engine", free_immediately)]
pub struct EngineValue {
    engine: Arc<dyn ContractEngine>,
}

impl std::fmt::Debug for EngineValue {
    /// The trait object cannot derive, and the class name is the whole
    /// honest description.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ThinkThen::Native::Engine")
    }
}

impl DataTypeFunctions for EngineValue {}

/// One crossing's answer, or the engine's error carried back as data.
type Crossing<T> = Result<T, ContractError>;

/// How long one wait slice holds before the crossing takes the VM lock
/// back and hears the host's interrupts even without a wake-up.
const WAIT_SLICE: Duration = Duration::from_millis(100);

/// What the call's own thread hands back, beside the wake flag Ruby's
/// unblock function sets.
struct Handoff<T> {
    state: Mutex<(Option<Crossing<T>>, bool)>,
    changed: Condvar,
}

impl<T> Handoff<T> {
    fn lock(&self) -> MutexGuard<'_, (Option<Crossing<T>>, bool)> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// One wait slice with the VM lock released: it returns when the answer
/// lands, when Ruby's unblock function wakes it, or after `WAIT_SLICE`.
unsafe extern "C" fn wait_for<T>(pointer: *mut c_void) -> *mut c_void {
    let handoff = unsafe { &*(pointer as *const Handoff<T>) };
    let mut state = handoff.lock();
    if state.0.is_none() && !state.1 {
        state = match handoff.changed.wait_timeout(state, WAIT_SLICE) {
            Ok((held, _)) => held,
            Err(poisoned) => poisoned.into_inner().0,
        };
    }
    state.1 = false;
    std::ptr::null_mut()
}

/// Ruby's unblock function: MRI calls it for every interrupt aimed at the
/// waiting thread - a signal, `Thread#raise`, `Thread#kill`, a spurious
/// `Thread#wakeup`. It only wakes the wait; the judgment happens with the
/// VM lock held, where MRI runs the interrupt itself.
unsafe extern "C" fn wake<T>(pointer: *mut c_void) {
    let handoff = unsafe { &*(pointer as *const Handoff<T>) };
    handoff.lock().1 = true;
    handoff.changed.notify_all();
}

/// Block the asynchronous signals on the call's own thread, so the
/// kernel delivers them to a Ruby thread and never interrupts a send:
/// the fifth review caught one trapped USR1 making the connector send a
/// paid request twice. The engine's worker threads inherit the mask.
/// Faults and aborts stay deliverable so Ruby's crash report sees them.
fn quiet_signals() {
    let mut set = std::mem::MaybeUninit::<libc::sigset_t>::uninit();
    unsafe {
        libc::sigfillset(set.as_mut_ptr());
        let faults = [
            libc::SIGSEGV, libc::SIGBUS, libc::SIGFPE, libc::SIGILL, libc::SIGSYS, libc::SIGTRAP, libc::SIGABRT,
        ];
        for fault in faults {
            libc::sigdelset(set.as_mut_ptr(), fault);
        }
        libc::pthread_sigmask(libc::SIG_BLOCK, set.as_ptr(), std::ptr::null_mut());
    }
}

/// Run one engine call on its own thread while this Ruby thread waits in
/// slices with the VM lock released, never re-taken from inside the wait.
///
/// Between slices the lock is held in ordinary Ruby-level frames and MRI
/// runs the pending interrupts under `rb_protect`: a trap handler that
/// raises nothing, or a spurious `Thread#wakeup`, leaves the call alone;
/// a real raise - Ctrl-C, `Thread#raise`, `Thread#kill`, a raising trap -
/// fires the call's own token, the requests already sent finish, none
/// starts after, and the first raise returns as `Err` when the call's
/// thread is done: nothing is served after the return. A kill or a throw
/// returns at once instead.
///
/// The fourth review's crash came from re-taking the lock through
/// `rb_thread_call_with_gvl` inside the released region; this shape
/// never does, and every jump MRI makes lands in `rb_protect` over frames
/// that own nothing.
fn cross<T: Send + 'static>(
    token: &Cancel,
    work: impl FnOnce() -> Crossing<T> + Send + 'static,
) -> Result<Crossing<T>, Error> {
    let handoff = Arc::new(Handoff { state: Mutex::new((None, false)), changed: Condvar::new() });
    let feed = Arc::clone(&handoff);
    let started = std::thread::Builder::new().name("thinkthen-call".into()).spawn(move || {
        quiet_signals();
        let answer = guarded(work);
        feed.lock().0 = Some(answer);
        feed.changed.notify_all();
    });
    if let Err(error) = started {
        return Ok(Err(ContractError::local(format!("the call's thread could not start: {error}"))));
    }
    let pointer = Arc::as_ptr(&handoff) as *mut c_void;
    let mut first: Option<Error> = None;
    loop {
        let heard = magnus::rb_sys::protect(|| {
            unsafe {
                rb_sys::rb_thread_call_without_gvl(
                    Some(wait_for::<T>),
                    pointer,
                    Some(wake::<T>),
                    pointer,
                );
                rb_sys::rb_thread_check_ints();
            }
            rb_sys::Qnil.into()
        });
        if let Err(raised) = heard {
            token.cancel();
            // A kill or a throw leaves MRI state in the error info that a
            // later catch would clear, so it returns at once; the call's
            // thread owns its data and finishes its sent requests alone.
            if raised.value().is_none() {
                return Err(raised);
            }
            first.get_or_insert(raised);
        }
        if let Some(answer) = handoff.lock().0.take() {
            return first.map_or(Ok(answer), Err);
        }
    }
}

/// The answer conversion under `rb_protect`: an allocation failure or any
/// other raise inside comes back as `Err` instead of jumping across the
/// Rust frames that hold the answer.
fn protected(build: impl FnOnce() -> Result<Value, Error>) -> Result<Value, Error> {
    let mut carried: Option<Result<Value, Error>> = None;
    let slot = &mut carried;
    let outcome = magnus::rb_sys::protect(move || {
        *slot = Some(build());
        rb_sys::Qnil.into()
    });
    match outcome {
        Ok(_) => carried.take().unwrap_or_else(|| {
            Err(map_error(thinkthen_contract::Error::defect(
                "the protected conversion produced no outcome",
            )))
        }),
        Err(raised) => Err(raised),
    }
}

/// Run one engine call behind the contract's shared panic boundary: a
/// panic from anywhere beneath the body comes back as one error of the
/// defect kind, never as an unwind into the host process.
fn guarded<T>(call: impl FnOnce() -> Crossing<T>) -> Crossing<T> {
    thinkthen_contract::catch_panic("the Ruby call", call)
}

/// The error class name of one kind.
const fn exception_name(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::Usage => "UsageError",
        ErrorKind::Backend => "BackendError",
        ErrorKind::Deadline => "DeadlineError",
        ErrorKind::Local => "LocalError",
        ErrorKind::Cancelled => "CancelledError",
        ErrorKind::Defect => "DefectError",
    }
}

/// The error class of one kind, looked up where `init` defined it.
fn exception_class(kind: ErrorKind) -> ExceptionClass {
    let name = exception_name(kind);
    let module = define_module("ThinkThen").expect("the ThinkThen module exists");
    let found = module.const_get(name).expect("the error class exists");
    ExceptionClass::from_value(found).expect("the error class is a class")
}

/// Map one contract failure onto its Ruby raise, with the kind and the
/// retry signal riding the exception object. Runs with the lock held.
fn map_error(error: ContractError) -> Error {
    let class = exception_class(error.kind);
    let built: Result<Value, Error> =
        class.funcall("new", (error.message, error.kind.to_string(), error.retryable));
    match built.ok().and_then(magnus::exception::Exception::from_value) {
        Some(exception) => exception.into(),
        None => Error::new(class, "thinkthen failed"),
    }
}

/// Which single-evidence verb a crossing runs.
#[derive(Clone, Copy)]
enum Single {
    Decide,
    Choose,
    Score,
    Tag,
    Details,
}

enum SingleOut {
    Answer(Answer),
    Choice(Option<String>),
    Score(Scored),
    Tags(Vec<String>),
    Details(Details),
}

/// Which many-record verb a crossing runs.
#[derive(Clone, Copy)]
enum Bulk {
    DecideMany,
    DecideManyWithProbabilities,
    Filter,
    Rank,
}

enum BulkOut {
    Judgments(Vec<Judgment>),
    JudgedPairs(Vec<Judgment>),
    Kept(Vec<usize>),
    Ranked(Vec<Ranked>),
}

/// Build the call options the wrapper's two keywords control: the token
/// the engine watches, and the caller's deadline through the contract's
/// one checked conversion. A NaN, a negative other than the `NO_DEADLINE`
/// sentinel, or an oversized budget is a usage error instead of the panic
/// the unchecked conversion raised inside the host process.
fn options_for(token: &Cancel, deadline: Option<f64>) -> Result<Options<'_>, ContractError> {
    Options::new().cancel(token).with_deadline_seconds(deadline)
}

/// Yes is `true`, no is `false`, unsure is `nil`.
fn answer_value(answer: Answer) -> Value {
    let ruby = magnus::Ruby::get().expect("the lock is held here");
    match answer.value() {
        Some(true) => true.into_value_with(&ruby),
        Some(false) => false.into_value_with(&ruby),
        None => ().into_value_with(&ruby),
    }
}

impl QuestionValue {
    /// The JSON text this question was built from.
    fn json(&self) -> String {
        self.json.clone()
    }
}

/// The token a crossing carries: the one the Ruby side hands in — the
/// call's own, created beside its watchdog row — or a fresh one nothing
/// holds, which keeps an unarmed call unarmed.
fn own_cancel(value: Value) -> Result<Cancel, Error> {
    if value.is_nil() {
        return Ok(Cancel::new());
    }
    match <&CancelValue as magnus::TryConvert>::try_convert(value) {
        Ok(held) => Ok(held.token.clone()),
        Err(_) => Err(map_error(ContractError::usage(
            "cancel is a ThinkThen::Cancel or nil",
        ))),
    }
}

/// A nil-tolerant read of the optional deadline in seconds. A value that
/// is not a number raises the usage error naming the deadline, instead of
/// quietly reading as no deadline at all.
fn optional_deadline(value: Value) -> Result<Option<f64>, Error> {
    if value.is_nil() {
        return Ok(None);
    }
    match <f64 as magnus::TryConvert>::try_convert(value) {
        Ok(held) => Ok(Some(held)),
        Err(_) => Err(map_error(ContractError::usage(
            "the deadline is seconds, a number, or nil for no deadline",
        ))),
    }
}
impl EngineValue {
    fn new_engine() -> Result<Self, Error> {
        let connector = StandinConnector;
        let engine = guarded(|| {
            ContractConnector::connect(&connector, &EngineConfig::from_env())
        })
        .map_err(map_error)?;
        Ok(Self { engine })
    }

    /// Any single-evidence verb, one crossing with the lock released.
    fn single(
        &self,
        which: Single,
        question: &QuestionValue,
        evidence: String,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let token = own_cancel(own)?;
        let deadline = optional_deadline(deadline)?;
        let engine = Arc::clone(&self.engine);
        let question = question.question.clone();
        let call = token.clone();
        let answer = cross(&token, move || {
            let options = options_for(&call, deadline)?;
            match which {
                Single::Decide => engine.decide_opts(&question, &evidence, options).map(SingleOut::Answer),
                Single::Choose => engine.choose_opts(&question, &evidence, options).map(SingleOut::Choice),
                Single::Score => engine.score_opts(&question, &evidence, options).map(SingleOut::Score),
                Single::Tag => engine.tag_opts(&question, &evidence, options).map(SingleOut::Tags),
                Single::Details => engine.details_opts(&question, &evidence, options).map(SingleOut::Details),
            }
        })?;
        protected(|| {
            let answer = answer.map_err(map_error)?;
            let ruby = magnus::Ruby::get().unwrap();
            let value = match answer {
                SingleOut::Answer(one) => answer_value(one),
                SingleOut::Choice(pick) => match pick {
                    Some(name) => name.clone().into_value_with(&ruby),
                    None => ().into_value_with(&ruby),
                },
                SingleOut::Score(scored) => {
                    let pair = RArray::with_capacity(2);
                    pair.push(scored.value)?;
                    pair.push(scored.nearest)?;
                    pair.as_value()
                }
                SingleOut::Tags(labels) => {
                    let list = RArray::with_capacity(labels.len());
                    for label in labels {
                        list.push(label)?;
                    }
                    list.as_value()
                }
                SingleOut::Details(details) => details_hash(&ruby, &details)?.as_value(),
            };
            Ok(value)
        })
    }

    /// Any many-record verb, one crossing with the lock released and the
    /// watchdog watching from the Ruby side.
    fn bulk(
        &self,
        which: Bulk,
        question: &QuestionValue,
        records: Vec<String>,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        // The engine watches the call's own token: the crossing fires it
        // for the host's real interrupts and the watchdog for the
        // caller's token; a token two calls share is only ever fired by
        // its own `cancel`.
        let token = own_cancel(own)?;
        let deadline = optional_deadline(deadline)?;
        let engine = Arc::clone(&self.engine);
        let question = question.question.clone();
        let call = token.clone();
        let answer = cross(&token, move || {
            let options = options_for(&call, deadline)?;
            let slices: Vec<&str> = records.iter().map(String::as_str).collect();
            match which {
                Bulk::DecideMany => engine
                    .decide_many_opts(&question, &slices, options, None)
                    .map(BulkOut::Judgments),
                Bulk::DecideManyWithProbabilities => engine
                    .decide_many_opts(&question, &slices, options, None)
                    .map(BulkOut::JudgedPairs),
                Bulk::Filter => engine.filter_opts(&question, &slices, options, None).map(BulkOut::Kept),
                Bulk::Rank => engine.rank_opts(&question, &slices, options, None).map(BulkOut::Ranked),
            }
        })?;
        protected(|| {
            let answer = answer.map_err(map_error)?;
            match answer {
            BulkOut::Judgments(judgments) => {
                let list = RArray::with_capacity(judgments.len());
                for judgment in judgments {
                    list.push(answer_value(judgment.answer))?;
                }
                Ok(list.as_value())
            }
            BulkOut::JudgedPairs(judgments) => {
                let list = RArray::with_capacity(judgments.len());
                for judgment in judgments {
                    let pair = RHash::new();
                    pair.aset("answer", answer_value(judgment.answer))?;
                    pair.aset("probability", judgment.probability)?;
                    list.push(pair)?;
                }
                Ok(list.as_value())
            }
            BulkOut::Kept(places) => {
                let list = RArray::with_capacity(places.len());
                for place in places {
                    list.push(place as i64)?;
                }
                Ok(list.as_value())
            }
            BulkOut::Ranked(ranked) => {
                let list = RArray::with_capacity(ranked.len());
                for one in ranked {
                    let pair = RArray::with_capacity(2);
                    pair.push(one.index as i64)?;
                    pair.push(one.probability)?;
                    list.push(pair)?;
                }
                Ok(list.as_value())
            }
            }
        })
    }

    fn decide(
        &self,
        question: &QuestionValue,
        evidence: String,
        cancel: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        self.single(Single::Decide, question, evidence, cancel, deadline)
    }



    /// `recognize`: the spec is the contract's one grammar; the answer is
    /// typed Ruby records built here from the engine's own structures, so
    /// no serialized answer JSON is parsed on the Ruby side.
    fn recognize(
        &self,
        spec: String,
        text: String,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let ask = Recognize::from_json(&spec).map_err(map_error)?;
        let token = own_cancel(own)?;
        let deadline = optional_deadline(deadline)?;
        let engine = Arc::clone(&self.engine);
        let call = token.clone();
        let answer = cross(&token, move || {
            let options = options_for(&call, deadline)?;
            engine.recognize_opts(&ask, &text, options)
        })?;
        protected(|| {
            let answer = answer.map_err(map_error)?;
            recognized_value(&answer)
        })
    }

    /// `relate`: every record crosses at once, the 255-record limit refuses
    /// with the usage kind before any question is asked. The edges come
    /// back as typed Ruby records, not a serialized answer to re-parse.
    fn relate(
        &self,
        spec: String,
        records: Vec<String>,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let ask = Relate::from_json(&spec).map_err(map_error)?;
        let token = own_cancel(own)?;
        let deadline = optional_deadline(deadline)?;
        let engine = Arc::clone(&self.engine);
        let call = token.clone();
        let answer = cross(&token, move || {
            let options = options_for(&call, deadline)?;
            let slices: Vec<&str> = records.iter().map(String::as_str).collect();
            relate_checked(engine.as_ref(), &ask, &slices, options)
        })?;
        protected(|| {
            let answer = answer.map_err(map_error)?;
            edges_value(&answer)
        })
    }

    fn decide_many(
        rb_self: Value,
        question: &QuestionValue,
        records: Vec<String>,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        engine.bulk(Bulk::DecideMany, question, records, own, deadline)
    }

    /// The same one crossing as `decide_many`, with each judgment's
    /// probability carried beside its answer from the same call. No
    /// second request is made merely to expose the numbers the bulk
    /// call already produced.
    fn decide_many_with_probabilities(
        rb_self: Value,
        question: &QuestionValue,
        records: Vec<String>,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        engine.bulk(
            Bulk::DecideManyWithProbabilities,
            question,
            records,
            own,
            deadline,
        )
    }

    fn filter(
        rb_self: Value,
        question: &QuestionValue,
        records: Vec<String>,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        engine.bulk(Bulk::Filter, question, records, own, deadline)
    }

    fn rank(
        rb_self: Value,
        question: &QuestionValue,
        records: Vec<String>,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        engine.bulk(Bulk::Rank, question, records, own, deadline)
    }

    fn choose(
        &self,
        question: &QuestionValue,
        evidence: String,
        cancel: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        self.single(Single::Choose, question, evidence, cancel, deadline)
    }

    fn score(
        &self,
        question: &QuestionValue,
        evidence: String,
        cancel: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        self.single(Single::Score, question, evidence, cancel, deadline)
    }

    fn tag(
        &self,
        question: &QuestionValue,
        evidence: String,
        cancel: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        self.single(Single::Tag, question, evidence, cancel, deadline)
    }

    fn details(
        &self,
        question: &QuestionValue,
        evidence: String,
        cancel: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        self.single(Single::Details, question, evidence, cancel, deadline)
    }

    fn find(
        &self,
        question: &QuestionValue,
        units: Vec<String>,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let token = own_cancel(own)?;
        let deadline = optional_deadline(deadline)?;
        let engine = Arc::clone(&self.engine);
        let question = question.question.clone();
        let call = token.clone();
        let found = cross(&token, move || {
            let options = options_for(&call, deadline)?;
            let slices: Vec<&str> = units.iter().map(String::as_str).collect();
            engine.find_opts(&question, &slices, options)
        })?;
        protected(|| {
            let found = found.map_err(map_error)?;
            let pair = RArray::with_capacity(2);
            match found.index {
                Some(place) => pair.push(place as i64)?,
                None => pair.push(())?,
            }
            pair.push(found.probability)?;
            Ok(pair.as_value())
        })
    }

    fn annotate(
        rb_self: Value,
        set: &SetValue,
        records: Vec<String>,
        own: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        let token = own_cancel(own)?;
        let deadline = optional_deadline(deadline)?;
        let engine = Arc::clone(&engine.engine);
        let set = set.set.clone();
        let call = token.clone();
        let records_out = cross(&token, move || {
            let options = options_for(&call, deadline)?;
            let slices: Vec<&str> = records.iter().map(String::as_str).collect();
            engine.annotate_opts(&set, &slices, options, None)
        })?;
        protected(|| {
            let records_out = records_out.map_err(map_error)?;
            let outer = RArray::with_capacity(records_out.len());
            for one in records_out {
                let inner = RArray::with_capacity(one.len());
                for (name, field) in one {
                    let pair = RArray::with_capacity(2);
                    pair.push(name)?;
                    pair.push(annotated_value(&field)?)?;
                    inner.push(pair)?;
                }
                outer.push(inner)?;
            }
            Ok(outer.as_value())
        })
    }

    fn usage(&self) -> Result<RHash, Error> {
        let Usage { requests, cache_answers, tokens } = self.engine.usage();
        let ruby = magnus::Ruby::get().unwrap();
        let hash = RHash::new();
        hash.aset("requests", requests)?;
        hash.aset("cache_answers", cache_answers)?;
        hash.aset("tokens", tokens)?;
        let _ = ruby;
        Ok(hash)
    }
}

/// One annotate field as a Ruby value, typed by its verb.
fn annotated_value(field: &Annotated) -> Result<Value, Error> {
    let ruby = magnus::Ruby::get().unwrap();
    Ok(match field {
        Annotated::Decision(answer) => answer_value(*answer),
        Annotated::Choice(pick) => match pick {
            Some(name) => name.clone().into_value_with(&ruby),
            None => ().into_value_with(&ruby),
        },
        Annotated::Score(scored) => {
            let pair = RArray::with_capacity(2);
            pair.push(scored.value)?;
            pair.push(scored.nearest.clone())?;
            pair.as_value()
        }
        Annotated::Tags(labels) => {
            let list = RArray::with_capacity(labels.len());
            for label in labels {
                list.push(label.clone())?;
            }
            list.as_value()
        }
        // The ruled failed-question marker (0054), this host's spelling: a
        // Hash with string keys, the host's own shape for structured data,
        // never `nil`.
        Annotated::Failed(failed) => {
            let inner = RHash::new();
            inner.aset("kind", kind_word(failed.kind))?;
            inner.aset("cause", cause_word(failed.cause))?;
            let outer = RHash::new();
            outer.aset("failed", inner)?;
            outer.as_value()
        }
    })
}

/// The failure kind's own word; `backend` today.
const fn kind_word(kind: thinkthen_contract::FailureKind) -> &'static str {
    use thinkthen_contract::FailureKind;
    match kind {
        FailureKind::Backend => "backend",
    }
}

/// The closed cause list's own words, spelled once.
const fn cause_word(cause: thinkthen_contract::Cause) -> &'static str {
    use thinkthen_contract::Cause;
    match cause {
        Cause::MissingAnswer => "missing_answer",
        Cause::WrongKind => "wrong_kind",
        Cause::MissingProbability => "missing_probability",
        Cause::InvalidProbability => "invalid_probability",
        Cause::InvalidDistribution => "invalid_distribution",
        Cause::UnexpectedProbability => "unexpected_probability",
    }
}

/// The audit trail as a Ruby hash, with the logical requests' digests
/// (0053) and the failed-question count (0054).
fn details_hash(ruby: &magnus::Ruby, details: &Details) -> Result<RHash, Error> {
    let hash = RHash::new();
    hash.aset("probability", details.probability)?;
    hash.aset("answer", answer_value(details.answer))?;
    hash.aset("model", details.model.clone())?;
    hash.aset("digest", details.digest.clone())?;
    hash.aset("sends", details.sends)?;
    let requests = RArray::with_capacity(details.requests.len());
    for digest in &details.requests {
        requests.push(digest.clone())?;
    }
    hash.aset("requests", requests)?;
    hash.aset("failed_questions", details.failed_questions)?;
    // The nearest level's name on a score question; nil on every other
    // verb (ADR 0017 pick 6, settled 2026-09-21).
    hash.aset("nearest", details.nearest.clone())?;
    let _ = ruby;
    Ok(hash)
}

/// The recognize answer as typed Ruby records: a hash with an `entities`
/// array and a `relations` array, each field converted once from the
/// engine's own structures. No serialized answer JSON is parsed in Ruby.
fn recognized_value(found: &Recognized) -> Result<Value, Error> {
    let ruby = magnus::Ruby::get().expect("the lock is held here");
    let entities = RArray::with_capacity(found.entities.len());
    for entity in &found.entities {
        let one = RHash::new();
        one.aset("id", entity.id as i64)?;
        one.aset("text", entity.text.clone())?;
        one.aset("kind", entity.kind.clone())?;
        one.aset("start", entity.start as i64)?;
        one.aset("end", entity.end as i64)?;
        one.aset("strength", entity.strength)?;
        entities.push(one)?;
    }
    let relations = RArray::with_capacity(found.relations.len());
    for relation in &found.relations {
        let one = RHash::new();
        one.aset("name", relation.name.clone())?;
        one.aset("source", relation.source as i64)?;
        one.aset("target", relation.target as i64)?;
        one.aset("probability", relation.probability)?;
        relations.push(one)?;
    }
    let answer = RHash::new();
    answer.aset("entities", entities)?;
    answer.aset("relations", relations)?;
    let _ = ruby;
    Ok(answer.as_value())
}

/// The relate answer as typed Ruby records: an array of edge hashes with
/// the ruled field names; the kind fields appear only when a rule named
/// one, matching the JSON door's shape.
fn edges_value(edges: &[Edge]) -> Result<Value, Error> {
    let ruby = magnus::Ruby::get().expect("the lock is held here");
    let list = RArray::with_capacity(edges.len());
    for edge in edges {
        let one = RHash::new();
        one.aset("name", edge.name.clone())?;
        one.aset("source", edge.source as i64)?;
        one.aset("target", edge.target as i64)?;
        one.aset("probability", edge.probability)?;
        if let Some(kind) = &edge.source_kind {
            one.aset("source_kind", kind.clone())?;
        }
        if let Some(kind) = &edge.target_kind {
            one.aset("target_kind", kind.clone())?;
        }
        list.push(one)?;
    }
    let _ = ruby;
    Ok(list.as_value())
}

/// Parse the file's grammar into a question value.
fn parse_question(json: String) -> Result<QuestionValue, Error> {
    match Question::from_json(&json) {
        Ok(question) => Ok(QuestionValue { json, question }),
        Err(error) => Err(map_error(error)),
    }
}

/// Parse the set's grammar into a set value.
fn parse_set(json: String) -> Result<SetValue, Error> {
    match QuestionSet::from_json(&json) {
        Ok(set) => Ok(SetValue { json, set }),
        Err(error) => Err(map_error(error)),
    }
}

/// Build a fresh cancel token.
fn cancel_new() -> CancelValue {
    CancelValue { token: Cancel::new() }
}

/// Register the module, the three value classes, and the error
/// hierarchy under `ThinkThen`.
#[magnus::init(name = "thinkthen")]
fn init() -> Result<(), Error> {
    let module = define_module("ThinkThen")?;
    let standard_error = class::object().const_get("StandardError").and_then(|found| {
        RClass::from_value(found).ok_or_else(|| {
            Error::new(exception::standard_error(), "StandardError is missing")
        })
    })?;

    // One base class for every failure this surface raises, so one
    // `rescue ThinkThen::Error` hears the engine's six kinds and the
    // wrapper's own refusals alike.
    let base = module.define_class("Error", standard_error)?;
    module.define_class("UsageError", base)?;
    module.define_class("BackendError", base)?;
    module.define_class("DeadlineError", base)?;
    module.define_class("LocalError", base)?;
    module.define_class("CancelledError", base)?;
    module.define_class("DefectError", base)?;

    let question = module.define_class("Question", class::object())?;
    question.define_method("json", method!(QuestionValue::json, 0))?;

    let set_class = module.define_class("QuestionSet", class::object())?;
    set_class.define_method("names", method!(SetValue::names, 0))?;

    let cancel = module.define_class("Cancel", class::object())?;
    cancel.define_singleton_method("new", function!(cancel_new, 0))?;
    cancel.define_method("cancel", method!(CancelValue::cancel, 0))?;
    cancel.define_method("cancelled?", method!(CancelValue::is_fired, 0))?;

    let native = module.define_module("Native")?;
    let engine = native.define_class("Engine", class::object())?;
    engine.define_singleton_method("new", function!(EngineValue::new_engine, 0))?;
    engine.define_method("decide", method!(EngineValue::decide, 4))?;
        engine.define_method("decide_many", method!(EngineValue::decide_many, 4))?;
        engine.define_method(
            "decide_many_with_probabilities",
            method!(EngineValue::decide_many_with_probabilities, 4),
        )?;
    engine.define_method("filter", method!(EngineValue::filter, 4))?;
    engine.define_method("rank", method!(EngineValue::rank, 4))?;
    engine.define_method("choose", method!(EngineValue::choose, 4))?;
    engine.define_method("score", method!(EngineValue::score, 4))?;
    engine.define_method("tag", method!(EngineValue::tag, 4))?;
    engine.define_method("details", method!(EngineValue::details, 4))?;
    engine.define_method("find", method!(EngineValue::find, 4))?;
    engine.define_method("annotate", method!(EngineValue::annotate, 4))?;
    engine.define_method("recognize", method!(EngineValue::recognize, 4))?;
    engine.define_method("relate", method!(EngineValue::relate, 4))?;
    engine.define_method("usage", method!(EngineValue::usage, 0))?;

    module.define_module_function("_parse_question", function!(parse_question, 1))?;
    module.define_module_function("_parse_set", function!(parse_set, 1))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defect_kind_names_its_error_class() {
        assert_eq!(exception_name(ErrorKind::Defect), "DefectError");
        assert_eq!(exception_name(ErrorKind::Usage), "UsageError");
    }

    #[test]
    fn a_panic_inside_the_shim_becomes_the_defect_kind() {
        let held = guarded(|| -> Crossing<()> { panic!("a shim bug") });
        match held {
            Err(error) => {
                assert_eq!(error.kind, ErrorKind::Defect);
                assert!(!error.retryable);
                assert!(
                    error.message.contains("the Ruby call") && error.message.contains("a shim bug"),
                    "the contract's boundary names itself and carries the panic's words: {error}"
                );
            }
            Ok(()) => panic!("the panic must not read as a value"),
        }
    }

    #[test]
    fn a_hostile_budget_is_a_usage_error_not_a_panic() {
        let token = Cancel::new();
        for held in [f64::NAN, f64::INFINITY, -5.0, 1e300] {
            let failure = match options_for(&token, Some(held)) {
                Err(error) => error,
                Ok(_) => panic!("the budget {held} must be refused"),
            };
            assert_eq!(failure.kind, ErrorKind::Usage, "the budget {held} is refused as usage");
        }
        let none = options_for(&token, Some(-1.0)).expect("the sentinel means no deadline");
        assert!(!none.passed());
        let spent = options_for(&token, Some(0.0)).expect("zero is spent, not refused");
        assert!(spent.passed());
    }
}
