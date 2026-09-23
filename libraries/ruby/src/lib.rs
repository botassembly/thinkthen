//! The Ruby surface: one native door over the contract.
//!
//! This crate holds no rule: no threshold math, no retry, no sending. It
//! converts arguments, calls one contract function with the VM lock
//! released, converts the answer, and maps the six error kinds onto the
//! Ruby error classes. The Ruby-side glue in `lib/thinkthen.rb` owns the
//! keyword shapes and the record handling; everything that crosses into
//! Rust happens here, one crossing a call.
//!
//! The 211 poll shape, ported: a bulk call releases the VM lock for the
//! whole run, and each tick of the engine's wait re-takes the lock through
//! `rb_thread_call_with_gvl` to run the surface's poll. The poll checks
//! the caller's own cancel token and calls MRI's pending-interrupt
//! handling, so a real `Thread#raise`, Ctrl-C, or kill cancels the call's
//! own token, sent requests finish, no new one starts, and the exception
//! is re-raised when the engine call returns. A spurious `Thread#wakeup`
//! and a trapped signal raise nothing and cancel nothing. The engine
//! watches the call's own token, never a token the caller shares, so an
//! interrupt cannot cancel a sibling call; the caller's token is heard by
//! the poll and fired only by the caller's own `cancel`. A token fired
//! without a raise returns the `cancelled` kind.
//!
//! The crossing carries no unblock function: Ruby calls one for every
//! interrupt at all — wake-up, signal, and raise alike — and a callback
//! without the VM lock cannot tell them apart, which is exactly the bug
//! the poll replaces. Single verbs have no poll to run (the contract's
//! single entry points take none), so their cancel channel is the
//! engine's own stop checks on the token the caller gave, and an
//! interrupt that lands during one surfaces when the call returns.

use std::ffi::c_void;
use std::sync::Arc;

use magnus::prelude::*;
use magnus::value::BoxValue;
use magnus::{
    class, define_module, exception, function, method, DataTypeFunctions, Error, ExceptionClass,
    IntoValue, RArray, RClass, RHash, TryConvert, TypedData, Value,
};
use thinkthen_contract::{
    relate_checked, Annotated, Answer, Cancel, Connector as ContractConnector, Details, Edge,
    Engine as ContractEngine, EngineConfig, Error as ContractError, ErrorKind, Found, Judgment,
    Options, Question, QuestionSet, Ranked, Recognize, Recognized, Relate, Scored, Usage,
};
use thinkthen_standin::StandinConnector;

/// A built question: the file's JSON text beside its parsed engine form.
#[derive(Clone, TypedData)]
#[magnus(class = "ThinkThen::Question", free_immediately)]
pub struct QuestionValue {
    json: String,
    question: Question,
}

impl DataTypeFunctions for QuestionValue {}

/// A built question set: the set's JSON text beside its parsed form.
#[derive(Clone, TypedData)]
#[magnus(class = "ThinkThen::QuestionSet", free_immediately)]
pub struct SetValue {
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
}

/// The engine value: one value from the contract's connector, held for
/// the process. It holds no thread between calls.
#[derive(TypedData)]
#[magnus(class = "ThinkThen::Native::Engine", free_immediately)]
pub struct EngineValue {
    engine: Arc<dyn ContractEngine>,
}

impl DataTypeFunctions for EngineValue {}

/// One crossing's answer, or the engine's error carried back as data.
type Crossing<T> = Result<T, ContractError>;

/// Run `body` on this Ruby thread with the VM lock released, over a job
/// the caller keeps. The crossing carries no unblock function: Ruby calls
/// one on every interrupt, including a spurious `Thread#wakeup` and a
/// trapped signal, and a callback without the VM lock cannot tell those
/// from a real `Thread#raise`. Bulk calls hear their host's interrupts
/// through the engine's poll instead, where the VM lock is taken again and
/// the pending interrupt can be judged; the caller's own token rides the
/// same poll.
///
/// The job is borrowed, not boxed across the C boundary: the body writes
/// its answer into the job's slot and takes no ownership, so a raise
/// landing at the GVL reacquire - before this function could consume any
/// handoff - leaks nothing. The whole crossing runs inside one
/// `rb_protect` at the call site (see [`protected`]), with the tick's
/// registration and the job's slot owned outside it, so any jump MRI
/// makes anywhere in the region is caught as data and every drop still
/// runs.
///
/// On return, a real interrupt that arrived between the last poll and
/// the return is drained here with the VM lock held and returned beside
/// the answer; every caller surfaces it before touching the answer.
fn without_gvl<A>(
    job: &mut A,
    body: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
) -> Option<Error> {
    let pointer = job as *mut A as *mut c_void;
    unsafe { rb_sys::rb_thread_call_without_gvl(Some(body), pointer, None, std::ptr::null_mut()) };
    hear_interrupts().err()
}

/// One crossing's whole Ruby-visible length under `rb_protect`: a raise
/// landing anywhere inside - at the GVL reacquire, in the error mapping,
/// in the answer conversion - comes back as `Err` instead of jumping
/// across the Rust frames of the call. The closure converts the answer
/// to a Value under the same protection, because the conversion itself
/// can hit MRI checkpoints. Callers own the tick registration and the
/// job outside this function and drop them after it returns, whatever it
/// returns.
fn protected(build: impl FnOnce() -> Result<Value, Error>) -> Result<Value, Error> {
    // The closure's outcome travels in this slot: `rb_protect` catches
    // only Ruby's unwinding, so the crossing's own answer or failure
    // rides out beside the nil placeholder and re-emerges below, never
    // as a jump.
    let mut carried: Option<Result<Value, Error>> = None;
    let slot = &mut carried;
    let outcome = magnus::rb_sys::protect(move || {
        *slot = Some(build());
        rb_sys::Qnil.into()
    });
    match outcome {
        Ok(_) => carried.take().unwrap_or_else(|| {
            Err(map_error(thinkthen_contract::Error::defect(
                "the protected crossing produced no outcome",
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

/// Hear the host's own interrupts with the VM lock taken: MRI's pending
/// interrupt handling runs here, so a real `Thread#raise`, Ctrl-C, or
/// `Thread#kill` comes back as a caught error while a spurious
/// `Thread#wakeup` and a trapped signal's handler raise nothing. The
/// contract's stop check cannot see a Ruby interrupt, so this poll is the
/// surface's own channel, the shape experiment 211 measured for Python.
///
/// `rb_protect` carries the jump, because a raise here must not unwind
/// across the Rust frames between the poll and the engine.
fn hear_interrupts() -> Result<(), Error> {
    magnus::rb_sys::protect(|| {
        unsafe { rb_sys::rb_thread_check_ints() };
        rb_sys::Qnil.into()
    })
    .map(|_| ())
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

/// What a single-evidence crossing carries in and out.
struct SingleJob {
    engine: Arc<dyn ContractEngine>,
    question: Question,
    evidence: String,
    token: Cancel,
    deadline: Option<f64>,
    which: Single,
    /// The crossing's answer, written by the body on this same thread:
    /// the job is borrowed across the C boundary, never boxed, so no
    /// handoff can leak when a raise lands at the GVL reacquire.
    answer: Option<Crossing<SingleOut>>,
}

/// What a recognize crossing carries in and out. The spec is parsed by the
/// contract's one grammar before the VM lock is released.
struct RecognizeJob {
    engine: Arc<dyn ContractEngine>,
    ask: Recognize,
    text: String,
    token: Cancel,
    deadline: Option<f64>,
    answer: Option<Crossing<Recognized>>,
}

/// What a relate crossing carries in and out: every record at once.
struct RelateJob {
    engine: Arc<dyn ContractEngine>,
    ask: Relate,
    records: Vec<String>,
    token: Cancel,
    deadline: Option<f64>,
    answer: Option<Crossing<Vec<Edge>>>,
}

enum SingleOut {
    Answer(Answer),
    Choice(Option<String>),
    Score(Scored),
    Tags(Vec<String>),
    Details(Details),
}

unsafe extern "C" fn single_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { &mut *(pointer as *mut SingleJob) };
    let which = job.which;
    let engine = job.engine.clone();
    let question = job.question.clone();
    let evidence = job.evidence.clone();
    let deadline = job.deadline;
    let token = job.token.clone();
    let answer: Crossing<SingleOut> = guarded(move || {
        let options = options_for(&token, deadline)?;
        match which {
            Single::Decide => engine
                .decide_opts(&question, &evidence, options)
                .map(SingleOut::Answer),
            Single::Choose => engine
                .choose_opts(&question, &evidence, options)
                .map(SingleOut::Choice),
            Single::Score => engine
                .score_opts(&question, &evidence, options)
                .map(SingleOut::Score),
            Single::Tag => engine
                .tag_opts(&question, &evidence, options)
                .map(SingleOut::Tags),
            Single::Details => engine
                .details_opts(&question, &evidence, options)
                .map(SingleOut::Details),
        }
    });
    job.answer = Some(answer);
    std::ptr::null_mut()
}

/// One recognize crossing, the VM lock released for its whole width.
unsafe extern "C" fn recognize_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { &mut *(pointer as *mut RecognizeJob) };
    let engine = job.engine.clone();
    let ask = job.ask.clone();
    let text = job.text.clone();
    let deadline = job.deadline;
    let token = job.token.clone();
    let answer: Crossing<Recognized> = guarded(move || {
        let options = options_for(&token, deadline)?;
        engine.recognize_opts(&ask, &text, options)
    });
    job.answer = Some(answer);
    std::ptr::null_mut()
}

/// One relate crossing: every record at once, the limit checked inside.
unsafe extern "C" fn relate_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { &mut *(pointer as *mut RelateJob) };
    let engine = job.engine.clone();
    let ask = job.ask.clone();
    let records: Vec<String> = job.records.clone();
    let deadline = job.deadline;
    let token = job.token.clone();
    let answer: Crossing<Vec<Edge>> = guarded(move || {
        let options = options_for(&token, deadline)?;
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        relate_checked(engine.as_ref(), &ask, &slices, options)
    });
    job.answer = Some(answer);
    std::ptr::null_mut()
}

/// Which many-record verb a crossing runs.
enum Bulk {
    DecideMany,
    DecideManyWithProbabilities,
    Filter,
    Rank,
}

/// What a many-record crossing carries in and out.
struct BulkJob {
    engine: Arc<dyn ContractEngine>,
    question: Question,
    records: Vec<String>,
    /// The call's own token: the engine watches this one, and only the
    /// poll fires it. The caller's token is `caller`, checked by the same
    /// poll, so an interrupt never fires a token two calls share.
    token: Cancel,
    caller: Option<Cancel>,
    deadline: Option<f64>,
    which: Bulk,
    /// The caller's block, held in a registered slot for the whole call
    /// so the collector cannot take it while Rust still calls it.
    tick: Option<*const Value>,
    /// The crossing's answer, written by the body on this same thread.
    answer: Option<Crossing<BulkOut>>,
    /// A raise the poll caught mid-batch, riding out beside the answer.
    raised: Option<Error>,
}

/// The poll state a bulk crossing threads into the engine's wait.
struct BulkTick {
    tick: Option<*const Value>,
    caller: Option<Cancel>,
    raised: Option<Error>,
    token: Cancel,
}

impl BulkTick {
    fn poll(&mut self) {
        if self.raised.is_some() {
            return;
        }
        let state = self as *mut BulkTick as *mut c_void;
        // The whole with-gvl body runs under one protection: a raise
        // landing at its GVL reacquire - or inside the caller's tick - is
        // caught here as data instead of jumping across the engine's wait
        // frames with this state pointer dangling behind it. That jump was
        // the drain-boundary SEGV the stress probe caught.
        let outcome = magnus::rb_sys::protect(|| unsafe {
            rb_sys::rb_thread_call_with_gvl(Some(bulk_tick_body), state) as rb_sys::VALUE
        });
        if let Err(raised) = outcome {
            self.token.cancel();
            self.raised = Some(raised);
        }
    }
}

unsafe extern "C" fn bulk_tick_body(pointer: *mut c_void) -> *mut c_void {
    let state = unsafe { &mut *(pointer as *mut BulkTick) };
    // The caller's own token stops this call through the call's own
    // token. Checking it here, never firing it, keeps a token shared by
    // several calls the caller's own gesture: an interrupt elsewhere
    // cannot cancel a sibling call.
    if state.caller.as_ref().is_some_and(Cancel::is_cancelled) {
        state.token.cancel();
        // A raise that landed in the same breath as the caller's cancel
        // must be caught here, with the VM lock held; left pending, MRI
        // delivers it at a later checkpoint - a GVL reacquisition inside
        // the batch - and the jump out skips the scope that joins this
        // batch's workers, which then live on over freed memory (the
        // review's repro: eight `ttb-worker` threads faulting in
        // `Error::guard` after the call had returned).
        if let Err(raised) = hear_interrupts() {
            state.raised = Some(raised);
        }
        return std::ptr::null_mut();
    }
    // Hear the host's own interrupts: a real Thread#raise, Ctrl-C, or
    // kill lands here as a caught error and rides out after the call; a
    // spurious Thread#wakeup and a trapped signal raise nothing.
    if let Err(raised) = hear_interrupts() {
        state.token.cancel();
        state.raised = Some(raised);
        return std::ptr::null_mut();
    }
    // The caller's own tick, when one was given. The slot is registered
    // for the call's length, so this read is the live value.
    if let Some(pointer) = state.tick {
        let tick = unsafe { *pointer };
        let outcome: Result<Value, Error> = tick.funcall("call", ());
        if let Err(raised) = outcome {
            state.token.cancel();
            state.raised = Some(raised);
        }
    }
    // A raise can arrive while the tick runs Ruby code; drain again so it
    // is caught here rather than at an MRI checkpoint later.
    if state.raised.is_none() {
        if let Err(raised) = hear_interrupts() {
            state.token.cancel();
            state.raised = Some(raised);
        }
    }
    std::ptr::null_mut()
}

enum BulkOut {
    Judgments(Vec<Judgment>),
    JudgedPairs(Vec<Judgment>),
    Kept(Vec<usize>),
    Ranked(Vec<Ranked>),
}

unsafe extern "C" fn bulk_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { &mut *(pointer as *mut BulkJob) };
    let engine = job.engine.clone();
    let question = job.question.clone();
    let records: Vec<String> = job.records.clone();
    let deadline = job.deadline;
    let token = job.token.clone();
    let tick = job.tick;
    // The poll always runs: the caller's token and the host's own
    // interrupts are heard there even when no tick was given.
    let mut tick_state = Some(BulkTick {
        tick,
        caller: job.caller.clone(),
        raised: None,
        token: token.clone(),
    });
    let answer: Crossing<BulkOut> = guarded(|| {
        let options = options_for(&token, deadline)?;
        let mut closure = || {
            if let Some(state) = tick_state.as_mut() {
                state.poll();
            }
        };
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        match job.which {
            Bulk::DecideMany => engine
                .decide_many_opts(&question, &slices, options, Some(&mut closure))
                .map(BulkOut::Judgments),
            Bulk::DecideManyWithProbabilities => engine
                .decide_many_opts(&question, &slices, options, Some(&mut closure))
                .map(BulkOut::JudgedPairs),
            Bulk::Filter => engine
                .filter_opts(&question, &slices, options, Some(&mut closure))
                .map(BulkOut::Kept),
            Bulk::Rank => engine
                .rank_opts(&question, &slices, options, Some(&mut closure))
                .map(BulkOut::Ranked),
        }
    });
    job.answer = Some(answer);
    job.raised = tick_state.and_then(|state| state.raised);
    std::ptr::null_mut()
}

/// The annotate crossing, which takes a set instead of one question.
struct AnnotateJob {
    engine: Arc<dyn ContractEngine>,
    set: QuestionSet,
    records: Vec<String>,
    token: Cancel,
    caller: Option<Cancel>,
    deadline: Option<f64>,
    tick: Option<*const Value>,
    answer: Option<Crossing<Vec<Vec<(String, Annotated)>>>>,
    raised: Option<Error>,
}

unsafe extern "C" fn annotate_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { &mut *(pointer as *mut AnnotateJob) };
    let engine = job.engine.clone();
    let set = job.set.clone();
    let records: Vec<String> = job.records.clone();
    let deadline = job.deadline;
    let token = job.token.clone();
    let tick = job.tick;
    let mut tick_state = Some(BulkTick {
        tick,
        caller: job.caller.clone(),
        raised: None,
        token: token.clone(),
    });
    let answer: Crossing<Vec<Vec<(String, Annotated)>>> = guarded(|| {
        let options = options_for(&token, deadline)?;
        let mut closure = || {
            if let Some(state) = tick_state.as_mut() {
                state.poll();
            }
        };
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine.annotate_opts(&set, &slices, options, Some(&mut closure))
    });
    job.answer = Some(answer);
    job.raised = tick_state.and_then(|state| state.raised);
    std::ptr::null_mut()
}

/// The find crossing, which takes units and no tick.
struct FindJob {
    engine: Arc<dyn ContractEngine>,
    question: Question,
    units: Vec<String>,
    token: Cancel,
    deadline: Option<f64>,
    answer: Option<Crossing<Found>>,
}

unsafe extern "C" fn find_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { &mut *(pointer as *mut FindJob) };
    let engine = job.engine.clone();
    let question = job.question.clone();
    let units: Vec<String> = job.units.clone();
    let deadline = job.deadline;
    let token = job.token.clone();
    let answer: Crossing<Found> = guarded(move || {
        let options = options_for(&token, deadline)?;
        let slices: Vec<&str> = units.iter().map(String::as_str).collect();
        engine.find_opts(&question, &slices, options)
    });
    job.answer = Some(answer);
    std::ptr::null_mut()
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

/// A nil-tolerant read of the optional cancel token.
fn optional_cancel(value: Value) -> Result<Option<Cancel>, Error> {
    if value.is_nil() {
        return Ok(None);
    }
    match <&CancelValue as magnus::TryConvert>::try_convert(value) {
        Ok(held) => Ok(Some(held.token.clone())),
        Err(_) => Err(map_error(ContractError::usage(
            "cancel is a ThinkThen::Cancel or nil",
        ))),
    }
}

/// The token a single crossing carries: the caller's own, or a fresh one
/// nothing holds. Single verbs run without a poll, so the engine's own
/// stop checks are the only place a cancel is heard; a fresh token keeps
/// an unarmed call unarmed.
fn single_token(cancel: Value) -> Result<Cancel, Error> {
    Ok(optional_cancel(cancel)?.unwrap_or_default())
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

/// A nil-tolerant read of the optional tick.
fn optional_tick(value: Value) -> Option<Value> {
    (!value.is_nil()).then_some(value)
}

/// The tick a bulk call runs: the caller's block, or the engine's own
/// `@tick` set by `ThinkThen.with_tick`, or none. The documented helper
/// stores its block in the ivar; the argument wins when both are given.
fn tick_from(_rb_self: Value, tick: Value) -> Result<Value, Error> {
    // The wrapper passes this thread's own tick explicitly (Thread.current
    // on the Ruby side); the old shared-ivar fallback is gone, because it
    // let one thread's tick run on another thread's calls.
    Ok(tick)
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
        cancel: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let token = single_token(cancel)?;
        let mut job = SingleJob {
            engine: Arc::clone(&self.engine),
            question: question.question.clone(),
            evidence,
            token: token.clone(),
            deadline: optional_deadline(deadline)?,
            which,
            answer: None,
        };
        // The crossing and its whole conversion run under one protection:
        // a raise anywhere inside - the GVL reacquire, the error mapping,
        // the value building - returns as data. The job lives on this
        // frame, so whatever happens inside, its answer slot drops here.
        protected(|| {
            let pending = without_gvl(&mut job, single_body);
            if let Some(raised) = pending {
                return Err(raised);
            }
            let answer = job
                .answer
                .take()
                .unwrap_or_else(|| Err(thinkthen_contract::Error::defect("the crossing returned no answer")));
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
                    pair.push(scored.value).map_err(|error| error)?;
                    pair.push(scored.nearest).map_err(|error| error)?;
                    pair.as_value()
                }
                SingleOut::Tags(labels) => {
                    let list = RArray::with_capacity(labels.len());
                    for label in labels {
                        list.push(label).map_err(|error| error)?;
                    }
                    list.as_value()
                }
                SingleOut::Details(details) => details_hash(&ruby, &details)?.as_value(),
            };
            Ok(value)
        })
    }

    /// Any many-record verb, one crossing with the lock released and the
    /// tick running each wait interval.
    fn bulk(
        &self,
        which: Bulk,
        question: &QuestionValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<Value, Error> {
        // The engine watches the call's own token, and the poll fires it
        // for the caller's token and for the host's real interrupts, so a
        // token two calls share is only ever fired by its own `cancel`.
        let token = Cancel::new();
        let caller = optional_cancel(cancel)?;
        if caller.as_ref().is_some_and(Cancel::is_cancelled) {
            token.cancel();
        }
        let held_tick = optional_tick(tick).map(BoxValue::new);
        let mut job = BulkJob {
            engine: Arc::clone(&self.engine),
            question: question.question.clone(),
            records,
            token: token.clone(),
            caller,
            deadline: optional_deadline(deadline)?,
            which,
            tick: held_tick.as_ref().map(|held| held.as_ref() as *const Value),
            answer: None,
            raised: None,
        };
        // The tick's registration and the job live on this frame, outside
        // the protection: whatever raise MRI delivers inside, the
        // registration drops and the answer slot empties on this line.
        let outcome = protected(|| {
            let pending = without_gvl(&mut job, bulk_body);
            if let Some(raised) = job.raised.take().or(pending) {
                return Err(raised);
            }
            let answer = job
                .answer
                .take()
                .unwrap_or_else(|| Err(thinkthen_contract::Error::defect("the crossing returned no answer")));
            let answer = answer.map_err(map_error)?;
            match answer {
            BulkOut::Judgments(judgments) => {
                let list = RArray::with_capacity(judgments.len());
                for judgment in judgments {
                    list.push(answer_value(judgment.answer)).map_err(|error| error)?;
                }
                Ok(list.as_value())
            }
            BulkOut::JudgedPairs(judgments) => {
                let list = RArray::with_capacity(judgments.len());
                for judgment in judgments {
                    let pair = RHash::new();
                    pair.aset("answer", answer_value(judgment.answer)).map_err(|error| error)?;
                    pair.aset("probability", judgment.probability).map_err(|error| error)?;
                    list.push(pair).map_err(|error| error)?;
                }
                Ok(list.as_value())
            }
            BulkOut::Kept(places) => {
                let list = RArray::with_capacity(places.len());
                for place in places {
                    list.push(place as i64).map_err(|error| error)?;
                }
                Ok(list.as_value())
            }
            BulkOut::Ranked(ranked) => {
                let list = RArray::with_capacity(ranked.len());
                for one in ranked {
                    let pair = RArray::with_capacity(2);
                    pair.push(one.index as i64).map_err(|error| error)?;
                    pair.push(one.probability).map_err(|error| error)?;
                    list.push(pair).map_err(|error| error)?;
                }
                Ok(list.as_value())
            }
        }
        });
        drop(held_tick);
        outcome
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
        cancel: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let ask = Recognize::from_json(&spec).map_err(map_error)?;
        let token = single_token(cancel)?;
        let mut job = RecognizeJob {
            engine: Arc::clone(&self.engine),
            ask,
            text,
            token: token.clone(),
            deadline: optional_deadline(deadline)?,
            answer: None,
        };
        protected(|| {
            let pending = without_gvl(&mut job, recognize_body);
            if let Some(raised) = pending {
                return Err(raised);
            }
            let answer = job
                .answer
                .take()
                .unwrap_or_else(|| Err(thinkthen_contract::Error::defect("the crossing returned no answer")));
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
        cancel: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let ask = Relate::from_json(&spec).map_err(map_error)?;
        let token = single_token(cancel)?;
        let mut job = RelateJob {
            engine: Arc::clone(&self.engine),
            ask,
            records,
            token: token.clone(),
            deadline: optional_deadline(deadline)?,
            answer: None,
        };
        protected(|| {
            let pending = without_gvl(&mut job, relate_body);
            if let Some(raised) = pending {
                return Err(raised);
            }
            let answer = job
                .answer
                .take()
                .unwrap_or_else(|| Err(thinkthen_contract::Error::defect("the crossing returned no answer")));
            let answer = answer.map_err(map_error)?;
            edges_value(&answer)
        })
    }

    fn decide_many(
        rb_self: Value,
        question: &QuestionValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        engine.bulk(Bulk::DecideMany, question, records, cancel, deadline, tick_from(rb_self, tick)?)
    }

    /// The same one crossing as `decide_many`, with each judgment's
    /// probability carried beside its answer from the same call. No
    /// second request is made merely to expose the numbers the bulk
    /// call already produced.
    fn decide_many_with_probabilities(
        rb_self: Value,
        question: &QuestionValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        engine.bulk(
            Bulk::DecideManyWithProbabilities,
            question,
            records,
            cancel,
            deadline,
            tick_from(rb_self, tick)?,
        )
    }

    fn filter(
        rb_self: Value,
        question: &QuestionValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        engine.bulk(Bulk::Filter, question, records, cancel, deadline, tick_from(rb_self, tick)?)
    }

    fn rank(
        rb_self: Value,
        question: &QuestionValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        engine.bulk(Bulk::Rank, question, records, cancel, deadline, tick_from(rb_self, tick)?)
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
        cancel: Value,
        deadline: Value,
    ) -> Result<Value, Error> {
        let token = single_token(cancel)?;
        let mut job = FindJob {
            engine: Arc::clone(&self.engine),
            question: question.question.clone(),
            units,
            token: token.clone(),
            deadline: optional_deadline(deadline)?,
            answer: None,
        };
        protected(|| {
            let pending = without_gvl(&mut job, find_body);
            if let Some(raised) = pending {
                return Err(raised);
            }
            let found = job
                .answer
                .take()
                .unwrap_or_else(|| Err(thinkthen_contract::Error::defect("the crossing returned no answer")));
            let found = found.map_err(map_error)?;
            let pair = RArray::with_capacity(2);
            match found.index {
                Some(place) => pair.push(place as i64).map_err(|error| error)?,
                None => pair.push(()).map_err(|error| error)?,
            }
            pair.push(found.probability).map_err(|error| error)?;
            Ok(pair.as_value())
        })
    }

    fn annotate(
        rb_self: Value,
        set: &SetValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<Value, Error> {
        let engine: &EngineValue = TryConvert::try_convert(rb_self)?;
        let token = Cancel::new();
        let caller = optional_cancel(cancel)?;
        if caller.as_ref().is_some_and(Cancel::is_cancelled) {
            token.cancel();
        }
        let held_tick = optional_tick(tick_from(rb_self, tick)?).map(BoxValue::new);
        let mut job = AnnotateJob {
            engine: Arc::clone(&engine.engine),
            set: set.set.clone(),
            records,
            token: token.clone(),
            caller,
            deadline: optional_deadline(deadline)?,
            tick: held_tick.as_ref().map(|held| held.as_ref() as *const Value),
            answer: None,
            raised: None,
        };
        // The tick's registration lives outside the protection, so it
        // drops on the next line whatever happened inside.
        let outcome = protected(|| {
            let pending = without_gvl(&mut job, annotate_body);
            if let Some(raised) = job.raised.take().or(pending) {
                return Err(raised);
            }
            let records_out = job
                .answer
                .take()
                .unwrap_or_else(|| Err(thinkthen_contract::Error::defect("the crossing returned no answer")));
            let records_out = records_out.map_err(map_error)?;
            let outer = RArray::with_capacity(records_out.len());
            for one in records_out {
                let inner = RArray::with_capacity(one.len());
                for (name, field) in one {
                    let pair = RArray::with_capacity(2);
                    pair.push(name).map_err(|error| error)?;
                    pair.push(annotated_value(&field)?).map_err(|error| error)?;
                    inner.push(pair).map_err(|error| error)?;
                }
                outer.push(inner).map_err(|error| error)?;
            }
            Ok(outer.as_value())
        });
        drop(held_tick);
        outcome
    }

    fn usage(&self) -> Result<RHash, Error> {
        let Usage { requests, cache_answers, tokens } = self.engine.usage();
        let ruby = magnus::Ruby::get().unwrap();
        let hash = RHash::new();
        hash.aset("requests", requests).map_err(|error| error)?;
        hash.aset("cache_answers", cache_answers).map_err(|error| error)?;
        hash.aset("tokens", tokens).map_err(|error| error)?;
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
            pair.push(scored.value).map_err(|error| error)?;
            pair.push(scored.nearest.clone()).map_err(|error| error)?;
            pair.as_value()
        }
        Annotated::Tags(labels) => {
            let list = RArray::with_capacity(labels.len());
            for label in labels {
                list.push(label.clone()).map_err(|error| error)?;
            }
            list.as_value()
        }
        // The ruled failed-question marker (0054), this host's spelling: a
        // Hash with string keys, the host's own shape for structured data,
        // never `nil`.
        Annotated::Failed(failed) => {
            let inner = RHash::new();
            inner.aset("kind", kind_word(failed.kind)).map_err(|error| error)?;
            inner.aset("cause", cause_word(failed.cause)).map_err(|error| error)?;
            let outer = RHash::new();
            outer.aset("failed", inner).map_err(|error| error)?;
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
    hash.aset("probability", details.probability).map_err(|error| error)?;
    hash.aset("answer", answer_value(details.answer)).map_err(|error| error)?;
    hash.aset("model", details.model.clone()).map_err(|error| error)?;
    hash.aset("digest", details.digest.clone()).map_err(|error| error)?;
    hash.aset("sends", details.sends).map_err(|error| error)?;
    let requests = RArray::with_capacity(details.requests.len());
    for digest in &details.requests {
        requests.push(digest.clone()).map_err(|error| error)?;
    }
    hash.aset("requests", requests).map_err(|error| error)?;
    hash.aset("failed_questions", details.failed_questions).map_err(|error| error)?;
    // The nearest level's name on a score question; nil on every other
    // verb (ADR 0017 pick 6, settled 2026-09-21).
    hash.aset("nearest", details.nearest.clone()).map_err(|error| error)?;
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
        one.aset("id", entity.id as i64).map_err(|error| error)?;
        one.aset("text", entity.text.clone()).map_err(|error| error)?;
        one.aset("kind", entity.kind.clone()).map_err(|error| error)?;
        one.aset("start", entity.start as i64).map_err(|error| error)?;
        one.aset("end", entity.end as i64).map_err(|error| error)?;
        one.aset("strength", entity.strength).map_err(|error| error)?;
        entities.push(one).map_err(|error| error)?;
    }
    let relations = RArray::with_capacity(found.relations.len());
    for relation in &found.relations {
        let one = RHash::new();
        one.aset("name", relation.name.clone()).map_err(|error| error)?;
        one.aset("source", relation.source as i64).map_err(|error| error)?;
        one.aset("target", relation.target as i64).map_err(|error| error)?;
        one.aset("probability", relation.probability).map_err(|error| error)?;
        relations.push(one).map_err(|error| error)?;
    }
    let answer = RHash::new();
    answer.aset("entities", entities).map_err(|error| error)?;
    answer.aset("relations", relations).map_err(|error| error)?;
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
        one.aset("name", edge.name.clone()).map_err(|error| error)?;
        one.aset("source", edge.source as i64).map_err(|error| error)?;
        one.aset("target", edge.target as i64).map_err(|error| error)?;
        one.aset("probability", edge.probability).map_err(|error| error)?;
        if let Some(kind) = &edge.source_kind {
            one.aset("source_kind", kind.clone()).map_err(|error| error)?;
        }
        if let Some(kind) = &edge.target_kind {
            one.aset("target_kind", kind.clone()).map_err(|error| error)?;
        }
        list.push(one).map_err(|error| error)?;
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

    let native = module.define_module("Native")?;
    let engine = native.define_class("Engine", class::object())?;
    engine.define_singleton_method("new", function!(EngineValue::new_engine, 0))?;
    engine.define_method("decide", method!(EngineValue::decide, 4))?;
        engine.define_method("decide_many", method!(EngineValue::decide_many, 5))?;
        engine.define_method(
            "decide_many_with_probabilities",
            method!(EngineValue::decide_many_with_probabilities, 5),
        )?;
    engine.define_method("filter", method!(EngineValue::filter, 5))?;
    engine.define_method("rank", method!(EngineValue::rank, 5))?;
    engine.define_method("choose", method!(EngineValue::choose, 4))?;
    engine.define_method("score", method!(EngineValue::score, 4))?;
    engine.define_method("tag", method!(EngineValue::tag, 4))?;
    engine.define_method("details", method!(EngineValue::details, 4))?;
    engine.define_method("find", method!(EngineValue::find, 4))?;
    engine.define_method("annotate", method!(EngineValue::annotate, 5))?;
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
