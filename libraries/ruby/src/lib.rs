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
//! `rb_thread_call_with_gvl` to run the caller's tick. A tick that raises —
//! an interrupt delivered while the lock is taken — first cancels the
//! engine's token, so sent requests finish and no new one starts, and the
//! exception is re-raised when the engine call returns. A token fired
//! without a raise returns the `cancelled` kind.

use std::ffi::c_void;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Arc;
use std::time::Duration;

use magnus::prelude::*;
use magnus::{
    class, define_module, exception, function, method, DataTypeFunctions, Error, ExceptionClass,
    IntoValue, RArray, RClass, RHash, TypedData, Value,
};
use thinkthen_contract::{
    edges_json, relate_checked, Annotated, Answer, Cancel, Details, Edge, Engine as ContractEngine,
    Error as ContractError, ErrorKind, Found, Judgment, Options, Question, QuestionSet, Ranked,
    Recognize, Recognized, Relate, Scored, Usage,
};
use thinkthen_standin::BlockingEngine;

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

/// A cancel token the Ruby side can fire from any thread.
#[derive(Clone, TypedData)]
#[magnus(class = "ThinkThen::Cancel", free_immediately)]
pub struct CancelValue {
    token: Arc<Cancel>,
}

impl DataTypeFunctions for CancelValue {}

impl CancelValue {
    /// Fire the token. Sent requests finish; no new one starts.
    fn cancel(&self) {
        self.token.cancel();
    }
}

/// The engine value. It holds no thread between calls.
#[derive(TypedData)]
#[magnus(class = "ThinkThen::Native::Engine", free_immediately)]
pub struct EngineValue {
    engine: BlockingEngine,
}

impl DataTypeFunctions for EngineValue {}

/// One crossing's answer, or the engine's error carried back as data.
type Crossing<T> = Result<T, ContractError>;

/// Run `body` on this Ruby thread with the VM lock released. The region
/// between the two calls touches no Ruby object.
fn without_gvl<A, R>(job: A, body: unsafe extern "C" fn(*mut c_void) -> *mut c_void) -> R {
    let boxed = Box::into_raw(Box::new(job)) as *mut c_void;
    let answer = unsafe {
        rb_sys::rb_thread_call_without_gvl(Some(body), boxed, None, std::ptr::null_mut())
    };
    unsafe { *Box::from_raw(answer as *mut R) }
}

/// Wrap an engine call so a panic never crosses the lock boundary.
fn guarded<T>(call: impl FnOnce() -> Crossing<T>) -> Crossing<T> {
    panic::catch_unwind(AssertUnwindSafe(call)).unwrap_or_else(|_| {
        Err(ContractError::defect("the engine panicked inside the Ruby shim"))
    })
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
enum Single {
    Decide,
    Choose,
    Score,
    Tag,
    Details,
}

/// What a single-evidence crossing carries in and out.
struct SingleJob {
    engine: *const BlockingEngine,
    question: Question,
    evidence: String,
    token: Option<Arc<Cancel>>,
    deadline: Option<f64>,
    which: Single,
}

/// What a recognize crossing carries in and out. The spec is parsed by the
/// contract's one grammar before the VM lock is released.
struct RecognizeJob {
    engine: *const BlockingEngine,
    ask: Recognize,
    text: String,
    token: Option<Arc<Cancel>>,
    deadline: Option<f64>,
}

/// What a relate crossing carries in and out: every record at once.
struct RelateJob {
    engine: *const BlockingEngine,
    ask: Relate,
    records: Vec<String>,
    token: Option<Arc<Cancel>>,
    deadline: Option<f64>,
}

enum SingleOut {
    Answer(Answer),
    Choice(Option<String>),
    Score(Scored),
    Tags(Vec<String>),
    Details(Details),
}

unsafe extern "C" fn single_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { *Box::from_raw(pointer as *mut SingleJob) };
    let engine = unsafe { &*job.engine };
    let options = options_for(job.token.as_deref(), job.deadline);
    let answer: Crossing<SingleOut> = guarded(|| match job.which {
        Single::Decide => engine
            .decide_opts(&job.question, &job.evidence, options)
            .map(SingleOut::Answer),
        Single::Choose => engine
            .choose_opts(&job.question, &job.evidence, options)
            .map(SingleOut::Choice),
        Single::Score => engine
            .score_opts(&job.question, &job.evidence, options)
            .map(SingleOut::Score),
        Single::Tag => engine
            .tag_opts(&job.question, &job.evidence, options)
            .map(SingleOut::Tags),
        Single::Details => engine
            .details_opts(&job.question, &job.evidence, options)
            .map(SingleOut::Details),
    });
    Box::into_raw(Box::new(answer)) as *mut c_void
}

/// One recognize crossing, the VM lock released for its whole width.
unsafe extern "C" fn recognize_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { *Box::from_raw(pointer as *mut RecognizeJob) };
    let engine = unsafe { &*job.engine };
    let options = options_for(job.token.as_deref(), job.deadline);
    let answer: Crossing<Recognized> =
        guarded(|| engine.recognize_opts(&job.ask, &job.text, options));
    Box::into_raw(Box::new(answer)) as *mut c_void
}

/// One relate crossing: every record at once, the limit checked inside.
unsafe extern "C" fn relate_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { *Box::from_raw(pointer as *mut RelateJob) };
    let engine = unsafe { &*job.engine };
    let options = options_for(job.token.as_deref(), job.deadline);
    let records: Vec<&str> = job.records.iter().map(String::as_str).collect();
    let answer: Crossing<Vec<Edge>> =
        guarded(|| relate_checked(engine, &job.ask, &records, options));
    Box::into_raw(Box::new(answer)) as *mut c_void
}

/// Which many-record verb a crossing runs.
enum Bulk {
    DecideMany,
    Filter,
    Rank,
}

/// What a many-record crossing carries in and out.
struct BulkJob {
    engine: *const BlockingEngine,
    question: Question,
    records: Vec<String>,
    token: Option<Arc<Cancel>>,
    deadline: Option<f64>,
    which: Bulk,
    tick: Option<Value>,
}

/// The poll state a bulk crossing threads into the engine's wait.
struct BulkTick {
    tick: Value,
    raised: Option<Error>,
    token: Option<Arc<Cancel>>,
}

impl BulkTick {
    fn poll(&mut self) {
        let state = self as *mut BulkTick as *mut c_void;
        unsafe { rb_sys::rb_thread_call_with_gvl(Some(bulk_tick_body), state) };
    }
}

unsafe extern "C" fn bulk_tick_body(pointer: *mut c_void) -> *mut c_void {
    let state = unsafe { &mut *(pointer as *mut BulkTick) };
    let outcome: Result<Value, Error> = state.tick.funcall("call", ());
    if let Err(raised) = outcome {
        if let Some(token) = state.token.clone() {
            token.cancel();
        }
        state.raised = Some(raised);
    }
    std::ptr::null_mut()
}

enum BulkOut {
    Judgments(Vec<Judgment>),
    Kept(Vec<usize>),
    Ranked(Vec<Ranked>),
}

unsafe extern "C" fn bulk_body(pointer: *mut c_void) -> *mut c_void {
    let mut job = unsafe { *Box::from_raw(pointer as *mut BulkJob) };
    let engine = unsafe { &*job.engine };
    let options = options_for(job.token.as_deref(), job.deadline);
    let records: Vec<&str> = job.records.iter().map(String::as_str).collect();
    let mut tick_state: Option<BulkTick> = job.tick.map(|tick| BulkTick {
        tick,
        raised: None,
        token: job.token.clone(),
    });
    let answer: Crossing<BulkOut> = guarded(|| {
        let mut closure = || {
            if let Some(state) = tick_state.as_mut() {
                state.poll();
            }
        };
        match job.which {
            Bulk::DecideMany => engine
                .decide_many_opts(&job.question, &records, options, Some(&mut closure))
                .map(BulkOut::Judgments),
            Bulk::Filter => engine
                .filter_opts(&job.question, &records, options, Some(&mut closure))
                .map(BulkOut::Kept),
            Bulk::Rank => engine
                .rank_opts(&job.question, &records, options, Some(&mut closure))
                .map(BulkOut::Ranked),
        }
    });
    let raised = tick_state.and_then(|state| state.raised);
    Box::into_raw(Box::new((answer, raised))) as *mut c_void
}

/// The annotate crossing, which takes a set instead of one question.
struct AnnotateJob {
    engine: *const BlockingEngine,
    set: QuestionSet,
    records: Vec<String>,
    token: Option<Arc<Cancel>>,
    deadline: Option<f64>,
    tick: Option<Value>,
}

unsafe extern "C" fn annotate_body(pointer: *mut c_void) -> *mut c_void {
    let mut job = unsafe { *Box::from_raw(pointer as *mut AnnotateJob) };
    let engine = unsafe { &*job.engine };
    let options = options_for(job.token.as_deref(), job.deadline);
    let records: Vec<&str> = job.records.iter().map(String::as_str).collect();
    let mut tick_state: Option<BulkTick> = job.tick.map(|tick| BulkTick {
        tick,
        raised: None,
        token: job.token.clone(),
    });
    let answer: Crossing<Vec<Vec<(String, Annotated)>>> = guarded(|| {
        let mut closure = || {
            if let Some(state) = tick_state.as_mut() {
                state.poll();
            }
        };
        engine.annotate_opts(&job.set, &records, options, Some(&mut closure))
    });
    let raised = tick_state.and_then(|state| state.raised);
    Box::into_raw(Box::new((answer, raised))) as *mut c_void
}

/// The find crossing, which takes units and no tick.
struct FindJob {
    engine: *const BlockingEngine,
    question: Question,
    units: Vec<String>,
    token: Option<Arc<Cancel>>,
    deadline: Option<f64>,
}

unsafe extern "C" fn find_body(pointer: *mut c_void) -> *mut c_void {
    let job = unsafe { *Box::from_raw(pointer as *mut FindJob) };
    let engine = unsafe { &*job.engine };
    let options = options_for(job.token.as_deref(), job.deadline);
    let units: Vec<&str> = job.units.iter().map(String::as_str).collect();
    let answer: Crossing<Found> = guarded(|| engine.find_opts(&job.question, &units, options));
    Box::into_raw(Box::new(answer)) as *mut c_void
}

/// Build the call options the wrapper's two keywords control.
fn options_for(token: Option<&Cancel>, deadline: Option<f64>) -> Options<'_> {
    let mut options = Options::new().maybe_cancel(token);
    if let Some(seconds) = deadline {
        options = options.deadline_in(Duration::from_secs_f64(seconds.max(0.0)));
    }
    options
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
fn optional_cancel(value: Value) -> Result<Option<Arc<Cancel>>, Error> {
    if value.is_nil() {
        return Ok(None);
    }
    match <&CancelValue as magnus::TryConvert>::try_convert(value) {
        Ok(held) => Ok(Some(held.token.clone())),
        Err(_) => Err(Error::new(exception::arg_error(), "cancel is not a ThinkThen::Cancel")),
    }
}

/// A nil-tolerant read of the optional deadline in seconds.
fn optional_deadline(value: Value) -> Option<f64> {
    if value.is_nil() {
        return None;
    }
    <f64 as magnus::TryConvert>::try_convert(value).ok()
}

/// A nil-tolerant read of the optional tick.
fn optional_tick(value: Value) -> Option<Value> {
    (!value.is_nil()).then_some(value)
}

impl EngineValue {
    fn new_engine() -> Self {
        Self { engine: BlockingEngine::from_env() }
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
        let job = SingleJob {
            engine: &self.engine,
            question: question.question.clone(),
            evidence,
            token: optional_cancel(cancel)?,
            deadline: optional_deadline(deadline),
            which,
        };
        let answer: Crossing<SingleOut> = without_gvl(job, single_body);
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
        let job = BulkJob {
            engine: &self.engine,
            question: question.question.clone(),
            records,
            token: optional_cancel(cancel)?,
            deadline: optional_deadline(deadline),
            which,
            tick: optional_tick(tick),
        };
        let (answer, raised): (Crossing<BulkOut>, Option<Error>) = without_gvl(job, bulk_body);
        if let Some(raised) = raised {
            return Err(raised);
        }
        let answer = answer.map_err(map_error)?;
        match answer {
            BulkOut::Judgments(judgments) => {
                let list = RArray::with_capacity(judgments.len());
                for judgment in judgments {
                    list.push(answer_value(judgment.answer)).map_err(|error| error)?;
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
    /// one JSON string and the Ruby side parses it into its own records,
    /// the pattern the C door uses for a result of no fixed size.
    fn recognize_json(
        &self,
        spec: String,
        text: String,
        cancel: Value,
        deadline: Value,
    ) -> Result<String, Error> {
        let ask = Recognize::from_json(&spec).map_err(map_error)?;
        let job = RecognizeJob {
            engine: &self.engine,
            ask,
            text,
            token: optional_cancel(cancel)?,
            deadline: optional_deadline(deadline),
        };
        let answer: Crossing<Recognized> = without_gvl(job, recognize_body);
        let answer = answer.map_err(map_error)?;
        Ok(answer.to_json())
    }

    /// `relate`: every record crosses at once, the 255-record limit refuses
    /// with the usage kind before any question is asked.
    fn relate_json(
        &self,
        spec: String,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
    ) -> Result<String, Error> {
        let ask = Relate::from_json(&spec).map_err(map_error)?;
        let job = RelateJob {
            engine: &self.engine,
            ask,
            records,
            token: optional_cancel(cancel)?,
            deadline: optional_deadline(deadline),
        };
        let answer: Crossing<Vec<Edge>> = without_gvl(job, relate_body);
        let answer = answer.map_err(map_error)?;
        Ok(edges_json(&answer))
    }

    fn decide_many(
        &self,
        question: &QuestionValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<Value, Error> {
        self.bulk(Bulk::DecideMany, question, records, cancel, deadline, tick)
    }

    fn filter(
        &self,
        question: &QuestionValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<Value, Error> {
        self.bulk(Bulk::Filter, question, records, cancel, deadline, tick)
    }

    fn rank(
        &self,
        question: &QuestionValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<Value, Error> {
        self.bulk(Bulk::Rank, question, records, cancel, deadline, tick)
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
    ) -> Result<RArray, Error> {
        let job = FindJob {
            engine: &self.engine,
            question: question.question.clone(),
            units,
            token: optional_cancel(cancel)?,
            deadline: optional_deadline(deadline),
        };
        let found: Crossing<Found> = without_gvl(job, find_body);
        let found = found.map_err(map_error)?;
        let pair = RArray::with_capacity(2);
        match found.index {
            Some(place) => pair.push(place as i64).map_err(|error| error)?,
            None => pair.push(()).map_err(|error| error)?,
        }
        pair.push(found.probability).map_err(|error| error)?;
        Ok(pair)
    }

    fn annotate(
        &self,
        set: &SetValue,
        records: Vec<String>,
        cancel: Value,
        deadline: Value,
        tick: Value,
    ) -> Result<RArray, Error> {
        let job = AnnotateJob {
            engine: &self.engine,
            set: set.set.clone(),
            records,
            token: optional_cancel(cancel)?,
            deadline: optional_deadline(deadline),
            tick: optional_tick(tick),
        };
        let (answer, raised): (Crossing<Vec<Vec<(String, Annotated)>>>, Option<Error>) =
            without_gvl(job, annotate_body);
        if let Some(raised) = raised {
            return Err(raised);
        }
        let records_out = answer.map_err(map_error)?;
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
        Ok(outer)
    }

    fn usage(&self) -> Result<RHash, Error> {
        let Usage { requests, cache_answers, tokens } = ContractEngine::usage(&self.engine);
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
    })
}

/// The audit trail as a Ruby hash.
fn details_hash(ruby: &magnus::Ruby, details: &Details) -> Result<RHash, Error> {
    let hash = RHash::new();
    hash.aset("probability", details.probability).map_err(|error| error)?;
    hash.aset("answer", answer_value(details.answer)).map_err(|error| error)?;
    hash.aset("model", details.model.clone()).map_err(|error| error)?;
    hash.aset("digest", details.digest.clone()).map_err(|error| error)?;
    hash.aset("sends", details.sends).map_err(|error| error)?;
    let _ = ruby;
    Ok(hash)
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
    CancelValue { token: Arc::new(Cancel::new()) }
}

#[magnus::init(name = "thinkthen")]
fn init() -> Result<(), Error> {
    let module = define_module("ThinkThen")?;
    let standard_error = class::object().const_get("StandardError").and_then(|found| {
        RClass::from_value(found).ok_or_else(|| {
            Error::new(exception::standard_error(), "StandardError is missing")
        })
    })?;
    let arg_error = class::object().const_get("ArgumentError").and_then(|found| {
        RClass::from_value(found).ok_or_else(|| {
            Error::new(exception::standard_error(), "ArgumentError is missing")
        })
    })?;

    module.define_class("UsageError", arg_error)?;
    module.define_class("BackendError", standard_error)?;
    module.define_class("DeadlineError", standard_error)?;
    module.define_class("LocalError", standard_error)?;
    module.define_class("CancelledError", standard_error)?;
    module.define_class("DefectError", standard_error)?;

    let question = module.define_class("Question", class::object())?;
    question.define_method("json", method!(QuestionValue::json, 0))?;

    module.define_class("QuestionSet", class::object())?;

    let cancel = module.define_class("Cancel", class::object())?;
    cancel.define_singleton_method("new", function!(cancel_new, 0))?;
    cancel.define_method("cancel", method!(CancelValue::cancel, 0))?;

    let native = module.define_module("Native")?;
    let engine = native.define_class("Engine", class::object())?;
    engine.define_singleton_method("new", function!(EngineValue::new_engine, 0))?;
    engine.define_method("decide", method!(EngineValue::decide, 4))?;
        engine.define_method("decide_many", method!(EngineValue::decide_many, 5))?;
    engine.define_method("filter", method!(EngineValue::filter, 5))?;
    engine.define_method("rank", method!(EngineValue::rank, 5))?;
    engine.define_method("choose", method!(EngineValue::choose, 4))?;
    engine.define_method("score", method!(EngineValue::score, 4))?;
    engine.define_method("tag", method!(EngineValue::tag, 4))?;
    engine.define_method("details", method!(EngineValue::details, 4))?;
    engine.define_method("find", method!(EngineValue::find, 4))?;
    engine.define_method("annotate", method!(EngineValue::annotate, 5))?;
    engine.define_method("recognize_json", method!(EngineValue::recognize_json, 4))?;
    engine.define_method("relate_json", method!(EngineValue::relate_json, 4))?;
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
            }
            Ok(()) => panic!("the panic must not read as a value"),
        }
    }
}
