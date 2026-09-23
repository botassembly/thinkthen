//! The ThinkThen DuckDB surface: nine SQL functions over the one engine,
//! per ADR 0017.
//!
//! Rule 1 of the database pages holds: no rule lives here. The extension
//! converts SQL values in and out, groups each chunk by its distinct
//! built questions and texts so a repeated row costs one judgment, and
//! maps the contract's six error kinds onto DuckDB's error reporting with
//! the retryable signal carried in the message. The engine owns
//! thresholds, retries, width, the cache, the counters, and answers.
//!
//! The functions are `thinkthen_decide`, `thinkthen_probability`,
//! `thinkthen_choose`, `thinkthen_score`, `thinkthen_tag`,
//! `thinkthen_annotate`, `thinkthen_details`, `thinkthen_usage`, and
//! `thinkthen_warm`. `filter`, `rank`, and `find` get no functions:
//! `WHERE`, `ORDER BY`, and `LIMIT` are those verbs. `NULL` is "not sure"
//! on every answering function, and a failure is an error that never
//! reads as `NULL`.
//!
//! A question argument is the question, as plain text under the grammar's
//! default cut, as a file named with the command's spelling
//! `'@refund.json'`, or as the file grammar's own JSON. A verb whose
//! members ride in a `LIST` argument takes its question as plain text and
//! its members from the list; the file spellings carry their own.
//!
//! The engine builds lazily on the first call and never in the
//! load-time init: a load may not touch the wire. Every engine call
//! carries the process-wide cancel token, and the SIGINT handler below
//! sets it, so the CLI's interrupt reaches the waits between requests.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex, OnceLock};

use duckdb::{
    Connection,
    core::{DataChunkHandle, Inserter, LogicalTypeHandle, LogicalTypeId},
    ffi,
    types::DuckString,
    vscalar::{ScalarFunctionSignature, VScalar},
    vtab::arrow::WritableVector,
};
use thinkthen_contract::{
    Annotated, Answer, Cancel, Connector as _, Engine as ContractEngine, EngineConfig,
    Error as EngineError, ErrorKind, Judgment, Options, Question, QuestionKind, QuestionSet, Scored,
};
use thinkthen_standin::StandinConnector;

mod connections;
mod guard;
mod recognize;
mod relate;
mod relations;
mod usage;
mod warm;

/// The engine this surface calls, built once from the environment through
/// the contract's connector: the stand-in implements the contract today,
/// and the real engine replaces it by naming a different connector in one
/// dependency line. The failure is kept as its verdict, so the first call
/// reports it and no call panics.
static ENGINE: OnceLock<Result<Arc<dyn ContractEngine>, String>> = OnceLock::new();

/// The engine value, built once, on first use and never at load time.
fn engine() -> Result<&'static Arc<dyn ContractEngine>, String> {
    match ENGINE.get_or_init(|| {
        StandinConnector
            .connect(&EngineConfig::from_env())
            .map_err(failure)
    }) {
        Ok(engine) => Ok(engine),
        Err(message) => Err(message.clone()),
    }
}

/// Run one engine call that answers a value, with the in-flight count
/// held for its whole duration, however it ends.
pub(crate) fn with_engine<T>(
    call: impl FnOnce(&dyn ContractEngine) -> T,
) -> Result<T, String> {
    let engine = engine()?;
    let _in_flight = InFlight::new();
    Ok(call(&**engine))
}

/// Run one engine call, its contract failure spelled for the SQL reader.
pub(crate) fn engine_call<T>(
    call: impl FnOnce(&dyn ContractEngine) -> Result<T, EngineError>,
) -> Result<T, String> {
    with_engine(call)?.map_err(failure)
}

/// The options every call carries: the live cancel token.
fn options() -> Options<'static> {
    let token = current_cancel();
    Options::new().maybe_cancel(Some(token))
}

/// The live cancellation token, cleared at the boundaries of the calls
/// that carry it.
///
/// One token served the whole process before, and a one-shot token is
/// spent forever: after one Ctrl-C every later call in that process
/// returned `cancelled` without asking anything (review finding: DuckDB's
/// interrupt poisoned the process). A second review found the surviving
/// half: an interrupt landing near an engine call — just after one
/// returned, or in a chunk gap — still cancelled the next query, four
/// runs in five. The rules now:
///
/// - The handler cancels the live token while an engine call is in
///   flight, or while calls are arriving in a burst — two call starts
///   within [`BURST_MS`] and the latest within it too. A chunked query
///   produces calls that close together, so a signal landing in a gap
///   still stops it at its next call; a signal that lands after a
///   query's last call — a lone call whose next call belongs to the next
///   query — cancels nothing and poisons nothing.
/// - A call takes the cancelled token while another call still runs with
///   it, or while the burst is active: the interrupt belongs to the query
///   still producing calls, and every one of them stops.
/// - A call whose engine returned the cancelled kind marks the token
///   spent, so the next call starts fresh; the last call out also clears
///   a cancelled token once the burst is over.
///
/// What stays unsolvable without a query hook: a signal that lands in the
/// gap between a chunked query's last call and its return, when the next
/// call already belongs to the next query. The C API exposes no
/// per-query callback for scalar functions — `VScalar` carries only
/// `invoke`, and the function-info accessors are `extra_info`,
/// `bind_data`, `init_data`, and `local_init_data` — so the surface sees
/// calls, never queries, and the two shapes above are as close as calls
/// can come to telling them apart. The boundary is pinned in NOTES.md.
/// Tokens are leaked once per interrupt, never per call.
static CANCEL: AtomicPtr<Cancel> = AtomicPtr::new(std::ptr::null_mut());

/// Engine calls in flight: the handler's "a query is running" signal.
static IN_FLIGHT: AtomicUsize = AtomicUsize::new(0);

/// The last two engine-call starts, in milliseconds: two calls that close
/// together are a query producing calls, which is what a signal needs to
/// stop.
static LAST_START: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static PREV_START: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// How close two call starts must be to count as a burst.
const BURST_MS: u64 = 10;

/// A cancellation that reached a call and ended it: the next call starts
/// clean.
static CANCEL_CONSUMED: AtomicBool = AtomicBool::new(false);

/// The monotonic clock the re-arm decisions count on, initialized long
/// before any handler runs.
static CLOCK: OnceLock<std::time::Instant> = OnceLock::new();

/// Milliseconds since the clock's base, never decreasing.
fn now_ms() -> u64 {
    CLOCK.get_or_init(std::time::Instant::now).elapsed().as_millis() as u64
}

/// Whether calls are arriving in a burst: the last two starts close
/// together, and the latest close to now.
fn burst_active() -> bool {
    let last = LAST_START.load(Ordering::SeqCst);
    let prev = PREV_START.load(Ordering::SeqCst);
    last != 0
        && prev != 0
        && last.saturating_sub(prev) < BURST_MS
        && now_ms().saturating_sub(last) < BURST_MS
}

/// When the handler last cancelled a token, on the activity clock: a
/// call starting within one burst window of the signal serves the
/// cancelled token too, so two queries running at the signal both stop
/// even when one was between calls when it landed (review 3, finding
/// 13).
static SIGNAL_AT: AtomicU64 = AtomicU64::new(0);

/// The in-flight guard: counted on entry, uncounted however the call
/// ends, a panic included; the last one out clears a spent or stale
/// interrupt so the next query starts clean.
struct InFlight;

impl InFlight {
    /// Count this call as in flight.
    fn new() -> Self {
        IN_FLIGHT.fetch_add(1, Ordering::SeqCst);
        Self
    }
}

/// The relate scan's in-flight guard: the scan counts as a live call
/// AND installs the cancel token, so the interrupt handler cancels it
/// beside interrupting the busy kept connections, and a relate queued
/// behind the gate can see the cancellation before it starts its own
/// query (review 4, finding 13's queued half).
pub(crate) struct ScanFlight {
    _in_flight: InFlight,
    _token: &'static Cancel,
}

pub(crate) fn scan_in_flight() -> ScanFlight {
    ScanFlight {
        _in_flight: InFlight::new(),
        _token: current_cancel(),
    }
}

impl Drop for InFlight {
    fn drop(&mut self) {
        if IN_FLIGHT.fetch_sub(1, Ordering::SeqCst) == 1 {
            clear_finished_cancel();
        }
    }
}

/// The last call to end clears a cancelled token once the burst is over:
/// an interrupt that reached its query, or reached none, is over with it.
/// Mid-burst the token stays, because the query's next call serves it.
fn clear_finished_cancel() {
    if let Some(token) = live_cancel()
        && token.is_cancelled()
        && !burst_active()
    {
        CANCEL_CONSUMED.store(false, Ordering::SeqCst);
        install_cancel();
    }
}

/// The live token a call carries: the cancelled one while another call
/// still runs with it or the burst is active, a fresh one otherwise.
fn current_cancel() -> &'static Cancel {
    PREV_START.store(LAST_START.load(Ordering::SeqCst), Ordering::SeqCst);
    LAST_START.store(now_ms(), Ordering::SeqCst);
    let Some(token) = live_cancel() else {
        return install_cancel();
    };
    if !token.is_cancelled() {
        return token;
    }
    if CANCEL_CONSUMED.swap(false, Ordering::SeqCst) {
        return install_cancel();
    }
    // This call already counts in flight, so more than one means another
    // call is running with this token and both stop together; a burst
    // means the query that produced the interrupt is still producing
    // calls, and this is one of them; a start within one burst window of
    // the signal belongs to the query the signal aimed at, whichever of
    // the running queries it is.
    let signal = SIGNAL_AT.load(Ordering::SeqCst);
    let in_wave = signal != 0 && now_ms().saturating_sub(signal) < BURST_MS;
    if IN_FLIGHT.load(Ordering::SeqCst) > 1 || burst_active() || in_wave {
        return token;
    }
    install_cancel()
}

/// Whether a cancelled token is live: a call that reaches the gate
/// after the interrupt fired must refuse before running its query, not
/// start six seconds of work nobody asked for anymore (review 4,
/// finding 13's queued half: the bridge stops the RUNNING query, and
/// this check stops the QUEUED one).
pub(crate) fn cancel_pending() -> bool {
    live_cancel().is_some_and(|token| token.is_cancelled())
}

/// The live token, when one exists, without installing anything.
fn live_cancel() -> Option<&'static Cancel> {
    let live = CANCEL.load(Ordering::Acquire);
    if live.is_null() {
        None
    } else {
        Some(unsafe { &*live })
    }
}

/// Install a fresh token, keeping a live one another thread installed
/// first.
fn install_cancel() -> &'static Cancel {
    let fresh = Box::into_raw(Box::new(Cancel::new()));
    loop {
        let live = CANCEL.load(Ordering::Acquire);
        if !live.is_null() && !unsafe { &*live }.is_cancelled() {
            drop(unsafe { Box::from_raw(fresh) });
            return unsafe { &*live };
        }
        match CANCEL.compare_exchange_weak(live, fresh, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return unsafe { &*fresh },
            Err(_) => continue,
        }
    }
}

/// The host's own SIGINT action, taken before ours, so the host's
/// choice still runs after our work: a handler is called with the
/// signature its flags name, `SIG_DFL` is restored and the signal
/// re-raised so the default action still terminates, and `SIG_IGN`
/// stops the install entirely (review 3, finding 13: `libc::signal`
/// replaced the disposition, and an `SA_SIGINFO` host handler received
/// garbage arguments).
static HOST_ACTION: std::sync::OnceLock<Option<libc::sigaction>> = std::sync::OnceLock::new();

/// Whether the handler is already installed, so a second LOAD does not
/// chain a handler to itself.
static HANDLER_SET: AtomicBool = AtomicBool::new(false);

/// Cancel the live token while an engine call is in flight or while calls
/// are arriving in a burst, then run the host's own action with the
/// signature its flags name. A signal that lands after a query's last
/// call is the host's own gesture: cancelling here would only poison the
/// next query.
extern "C" fn on_interrupt(
    signal: libc::c_int,
    info: *mut libc::siginfo_t,
    context: *mut libc::c_void,
) {
    if let Some(token) = live_cancel()
        && !token.is_cancelled()
        && (IN_FLIGHT.load(Ordering::SeqCst) > 0 || burst_active())
    {
        SIGNAL_AT.store(now_ms(), Ordering::SeqCst);
        token.cancel();
    }
    // Every kept connection running a relate query is interrupted too,
    // so one Ctrl-C stops both of two concurrent relates and reaches a
    // slow relate the caller's own interrupt cannot (review 4, finding
    // 13). The handler only wakes the bridge thread with one byte; the
    // registry walk locks and allocates, so it runs there (review 5,
    // finding 1: walking it here deadlocked a thread that held it).
    crate::connections::wake_interrupt_bridge();
    // A OnceLock read locks nothing: taking a mutex in a signal
    // handler can deadlock against the thread the signal interrupted
    // (review 4).
    let Some(Some(host)) = HOST_ACTION.get().map(|action| action.to_owned()) else {
        // Installed without a recorded host action: nothing to chain.
        return;
    };
    if host.sa_sigaction == libc::SIG_DFL {
        // The host ran on the default action: restore it and re-raise,
        // so the signal still terminates the process as the host chose.
        unsafe {
            libc::signal(libc::SIGINT, libc::SIG_DFL);
            libc::raise(signal);
        }
    } else if host.sa_flags & libc::SA_SIGINFO != 0 {
        let chained = unsafe {
            std::mem::transmute::<usize, extern "C" fn(i32, *mut libc::siginfo_t, *mut libc::c_void)>(
                host.sa_sigaction,
            )
        };
        chained(signal, info, context);
    } else if host.sa_sigaction > 1 {
        let chained = unsafe {
            std::mem::transmute::<usize, extern "C" fn(libc::c_int)>(host.sa_sigaction)
        };
        chained(signal);
    }
}

/// Take SIGINT for the token through `sigaction`, keeping the host's own
/// action for the chain. A host that ignored SIGINT is left ignoring it:
/// ours never installs, because the host's choice outranks our
/// interruption.
unsafe fn install_interrupt_handler() {
    if HANDLER_SET.swap(true, Ordering::SeqCst) {
        return;
    }
    // The activity clock starts now, long before any handler can read
    // it, so the handler's read never initializes anything.
    let _ = now_ms();
    crate::connections::start_interrupt_bridge();
    let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
    action.sa_sigaction = on_interrupt as extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void) as usize;
    action.sa_flags = libc::SA_SIGINFO;
    unsafe { libc::sigemptyset(&mut action.sa_mask) };
    let mut previous: libc::sigaction = unsafe { std::mem::zeroed() };
    if unsafe { libc::sigaction(libc::SIGINT, &action, &mut previous) } != 0 {
        return;
    }
    if previous.sa_sigaction == libc::SIG_IGN {
        // The host ignored SIGINT: put its action back untouched.
        unsafe { libc::sigaction(libc::SIGINT, &previous, std::ptr::null_mut()) };
        HANDLER_SET.store(false, Ordering::SeqCst);
        return;
    }
    let _ = HOST_ACTION.set(Some(previous));
}

/// A contract failure as the SQL error a reader sees, with the kind and
/// the retryable signal both carried. A cancellation serves its call and
/// marks the token spent; the next call starts fresh.
fn failure(error: EngineError) -> String {
    if error.kind == ErrorKind::Cancelled {
        CANCEL_CONSUMED.store(true, Ordering::SeqCst);
    }
    let retry = if error.retryable {
        " (a second try could help)"
    } else {
        ""
    };
    format!("thinkthen {}{retry}: {}", error.kind, error.message)
}

/// Resolve a question argument: `'@file.json'` names a file, a leading
/// `{` is the file grammar's own JSON, and anything else is the question
/// as plain text under the grammar's default cut. One grammar, one door.
/// A file read refuses when this database's own `enable_external_access`
/// is off.
fn resolve_question(arg: &str) -> Result<Question, String> {
    if let Some(path) = arg.strip_prefix('@') {
        if let Some(refusal) = file_read_refusal(path) {
            return Err(refusal);
        }
        question_from_file(path)
    } else if arg.starts_with('{') {
        Question::from_json(arg).map_err(failure)
    } else {
        let file = serde_json::json!({ "decide": arg }).to_string();
        Question::from_json(&file).map_err(failure)
    }
}

/// The question files this process has read, keyed by canonical path and
/// modification time: a query naming one file in every row reads it once
/// (review finding: the file was re-read per row, 20,000 opens in one
/// query), and an edited file is read again on its next use.
static QUESTION_FILES: LazyLock<Mutex<HashMap<String, (std::time::SystemTime, Question)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// The question file texts this process has read, keyed by canonical
/// path and modification time: a per-row reader (annotate, relations)
/// reads the file once per query, not once per row (review 3, finding
/// 27: 20,000 rows opened the file 20,000 times).
static QUESTION_TEXTS: LazyLock<Mutex<HashMap<String, (std::time::SystemTime, String)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// One question file's text, read once per file version. The
/// modification time decides whether the cached text still describes the
/// file.
pub(crate) fn question_text_cached(path: &str) -> Result<String, String> {
    let stamp = std::fs::metadata(path).and_then(|meta| meta.modified()).ok();
    let key = std::fs::canonicalize(path)
        .map(|real| real.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_owned());
    if let Some(stamp) = stamp {
        let cached = QUESTION_TEXTS.lock().expect("the file cache");
        if let Some((held, text)) = cached.get(&key)
            && *held == stamp
        {
            return Ok(text.clone());
        }
    }
    let text = connections::read_question_file(path, None)?;
    if let Some(stamp) = stamp {
        QUESTION_TEXTS
            .lock()
            .expect("the file cache")
            .insert(key, (stamp, text.clone()));
    }
    Ok(text)
}

/// One question file's question, read once per file version. The
/// modification time decides whether the cached question still describes
/// the file.
fn question_from_file(path: &str) -> Result<Question, String> {
    let stamp = std::fs::metadata(path).and_then(|meta| meta.modified()).ok();
    let key = std::fs::canonicalize(path)
        .map(|real| real.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_owned());
    if let Some(stamp) = stamp {
        let cached = QUESTION_FILES.lock().expect("the file cache");
        if let Some((held, question)) = cached.get(&key)
            && *held == stamp
        {
            return Ok(question.clone());
        }
    }
    let text = connections::read_question_file(path, None)?;
    let question = Question::from_json(&text).map_err(failure)?;
    if let Some(stamp) = stamp {
        QUESTION_FILES
            .lock()
            .expect("the file cache")
            .insert(key, (stamp, question.clone()));
    }
    Ok(question)
}

/// The refusal a `'@file'` read earns when a loaded database forbids it:
/// external access off, or the path outside `allowed_paths` and
/// `allowed_directories`, or the LocalFileSystem disabled. The settings
/// are read through the DuckDB API, so the databases' own answers decide
/// and no file is touched when the answer is no.
fn file_read_refusal(path: &str) -> Option<String> {
    connections::file_read_refusal(path)
}

/// Build a members verb's question from plain text and the `LIST`
/// argument. The members live in the list; the file spellings belong to
/// the verbs whose members ride inside the question.
fn resolve_member_question(
    arg: &str,
    members: &[String],
    kind: QuestionKind,
) -> Result<Question, String> {
    if arg.starts_with('@') || arg.starts_with('{') {
        return Err(format!(
            "thinkthen usage: {kind} takes its question as plain text and its {} in the list",
            match kind {
                QuestionKind::Choose => "options",
                QuestionKind::Score => "levels",
                _ => "labels",
            }
        ));
    }
    let owned: Vec<&str> = members.iter().map(String::as_str).collect();
    let built = match kind {
        QuestionKind::Choose => Question::choose(arg, &owned).and_then(ChoiceBuilt::build),
        QuestionKind::Score => Question::score(arg, &owned),
        QuestionKind::Tag => Question::tag(arg, &owned),
        QuestionKind::Decide => unreachable!("decide carries no list"),
    };
    built.map_err(failure)
}

/// The builder `Question::choose` returns, named so `build` needs no
/// imported type.
use thinkthen_contract::ChoiceBuilder as ChoiceBuilt;

/// Judge one built question's texts at the engine's width. One text asks
/// once; many cross once through the batch door.
fn judged(question: &Question, texts: &[&str]) -> Result<Vec<Judgment>, String> {
    judged_with(question, texts, None)
}

/// `judged` with a deadline budget for the batch's calls.
fn judged_with(
    question: &Question,
    texts: &[&str],
    budget: Option<f64>,
) -> Result<Vec<Judgment>, String> {
    let opts = options_for(budget)?;
    engine_call(|engine| engine.decide_many_opts(question, texts, opts, None))
}

/// The warm aggregate's judge: resolve the question argument, judge every
/// distinct text once through the batch door, and return how many were
/// asked. A failure is the caller's error: the aggregate raises it as the
/// query's own error, so nothing counts a failure as zero.
fn judge_warm(arg: &str, seen: HashSet<String>) -> Result<usize, String> {
    if seen.is_empty() {
        return Ok(0);
    }
    let mut texts: Vec<String> = seen.iter().cloned().collect();
    texts.sort_unstable();
    let question = resolve_question(arg)?;
    let borrowed: Vec<&str> = texts.iter().map(String::as_str).collect();
    judged(&question, &borrowed)?;
    Ok(texts.len())
}

/// The distinct questions and texts of a chunk, with each row's slot into
/// them, so a repeated (question, text) pair costs one judgment.
struct Distinct {
    /// The built questions, one per distinct digest, in first-seen order.
    questions: Vec<Question>,
    /// The distinct (digest index, text) pairs the rows hold.
    pairs: Vec<(usize, String)>,
    /// Each pair's deadline budget in milliseconds, from the row that
    /// first raised the pair; `None` when the call carries no deadline
    /// column or that row's budget is the no-deadline sentinel.
    budgets: Vec<Option<f64>>,
    /// Each row's pair, or `None` where a NULL made the row NULL.
    slots: Vec<Option<usize>>,
}

/// Group a chunk by its distinct built questions and texts. Questions are
/// keyed by the core's own digest, which covers the text, the threshold,
/// and the members, so two spellings of one question group together.
fn distinct_of(
    args: &[Option<String>],
    members: &[Option<Vec<String>>],
    texts: &[Option<String>],
    kind: Option<QuestionKind>,
    budgets: &[Option<f64>],
) -> Result<Distinct, String> {
    let mut built: HashMap<String, usize> = HashMap::new();
    let mut questions: Vec<Question> = Vec::new();
    let mut pairs: Vec<(usize, String)> = Vec::new();
    let mut pair_budgets: Vec<Option<f64>> = Vec::new();
    let mut pair_index: HashMap<(usize, String), usize> = HashMap::new();
    let mut slots: Vec<Option<usize>> = vec![None; texts.len()];
    for i in 0..texts.len() {
        let (Some(arg), Some(text)) = (args[i].as_deref(), texts[i].as_deref()) else {
            continue;
        };
        let question = match kind {
            None => resolve_question(arg)?,
            Some(kind) => {
                let Some(list) = members[i].as_deref() else {
                    continue;
                };
                resolve_member_question(arg, list, kind)?
            }
        };
        let digest = question.digest();
        let question_at = *built.entry(digest).or_insert_with(|| {
            questions.push(question);
            questions.len() - 1
        });
        let key = (question_at, text.to_owned());
        let budget = budgets.get(i).copied().flatten();
        let slot = *pair_index.entry(key).or_insert_with(|| {
            pairs.push((question_at, text.to_owned()));
            pair_budgets.push(budget);
            pairs.len() - 1
        });
        slots[i] = Some(slot);
    }
    Ok(Distinct {
        questions,
        pairs,
        budgets: pair_budgets,
        slots,
    })
}

/// Read one BIGINT column of a `VScalar` chunk as per-row deadline
/// budgets in milliseconds. A NULL anywhere in the arguments never
/// reaches this door: DuckDB's default NULL handling answers NULL for
/// the whole row before the function runs, the same silence a NULL
/// question or text carries, and the no-deadline spelling is `-1`.
fn read_budgets(input: &mut DataChunkHandle, column: usize) -> Result<Vec<Option<f64>>, String> {
    let len = input.len();
    let vector = input.flat_vector(column);
    let raw = unsafe { vector.as_slice_with_len::<i64>(len) };
    Ok((0..len).map(|i| Some(raw[i] as f64)).collect())
}

/// The chunk's deadline column, as budgets: an empty slice when the
/// call carries no deadline argument (the drawn spelling), the column's
/// rows when it does. The verb names the deadline's position — column 2
/// beside two drawn arguments, column 3 beside the list the choose,
/// score, and tag spellings carry.
fn deadline_column(input: &mut DataChunkHandle, at: usize) -> Result<Vec<Option<f64>>, String> {
    if input.num_columns() <= at {
        return Ok(Vec::new());
    }
    read_budgets(input, at)
}

/// The options one engine call carries: the process-wide cancel token,
/// and the per-call deadline when the pair's row carried one, converted
/// by the contract's one checked door (`-1` none, `0` spent, a positive
/// budget, every other negative refused).
fn options_for(budget: Option<f64>) -> Result<Options<'static>, String> {
    let mut options = options();
    if let Some(millis) = budget {
        options = options.with_deadline_millis(Some(millis)).map_err(failure)?;
    }
    Ok(options)
}

/// Read one VARCHAR column of a `VScalar` chunk as owned strings.
fn read_strings(input: &mut DataChunkHandle, column: usize) -> Vec<Option<String>> {
    let len = input.len();
    let vector = input.flat_vector(column);
    let raw = unsafe { vector.as_slice_with_len::<duckdb::ffi::duckdb_string_t>(len) };
    (0..len)
        .map(|i| {
            if vector.row_is_null(i as u64) {
                None
            } else {
                let mut value = raw[i];
                Some(DuckString::new(&mut value).as_str().to_string())
            }
        })
        .collect()
}

/// Read one LIST(VARCHAR) column of a `VScalar` chunk as owned string
/// lists. A NULL list or a NULL member inside one makes the row NULL, the
/// same semantics a NULL question or text carries.
fn read_list_strings(input: &mut DataChunkHandle, column: usize) -> Vec<Option<Vec<String>>> {
    let len = input.len();
    let lists = input.list_vector(column);
    let child_len = lists.len();
    let child = lists.child(child_len.max(1));
    let raw = unsafe { child.as_slice_with_len::<duckdb::ffi::duckdb_string_t>(child_len.max(1)) };
    (0..len)
        .map(|i| {
            if lists.row_is_null(i as u64) {
                return None;
            }
            let (offset, length) = lists.get_entry(i);
            let mut items = Vec::with_capacity(length);
            for j in 0..length {
                let at = offset + j;
                if at >= child_len || child.row_is_null(at as u64) {
                    return None;
                }
                let mut value = raw[at];
                items.push(DuckString::new(&mut value).as_str().to_string());
            }
            Some(items)
        })
        .collect()
}

/// Read one VARCHAR column of a raw C-API chunk as owned strings. NULL in
/// the validity mask reads as `None`.
pub(crate) unsafe fn read_raw_column(
    input: ffi::duckdb_data_chunk,
    column: usize,
) -> Vec<Option<String>> {
    let len = unsafe { ffi::duckdb_data_chunk_get_size(input) } as usize;
    let vector = unsafe { ffi::duckdb_data_chunk_get_vector(input, column as u64) };
    let data = unsafe { ffi::duckdb_vector_get_data(vector) } as *mut ffi::duckdb_string_t;
    let mask = unsafe { ffi::duckdb_vector_get_validity(vector) };
    (0..len)
        .map(|i| {
            let valid =
                mask.is_null() || unsafe { ffi::duckdb_validity_row_is_valid(mask, i as u64) };
            if !valid {
                None
            } else {
                let mut raw = unsafe { *data.add(i) };
                Some(DuckString::new(&mut raw).as_str().to_string())
            }
        })
        .collect()
}

/// One verb's two spellings: the drawn call, and the same call with the
/// ruled third (or fourth) deadline argument in milliseconds — `-1` none,
/// `0` spent, a positive budget, every other negative refused by the
/// contract's one checked door.
fn drawn_and_deadline(
    mut params: impl FnMut() -> Vec<LogicalTypeHandle>,
    mut return_type: impl FnMut() -> LogicalTypeHandle,
) -> Vec<ScalarFunctionSignature> {
    let mut with_deadline = params();
    with_deadline.push(LogicalTypeId::Bigint.into());
    let drawn = ScalarFunctionSignature::exact(params(), return_type());
    vec![
        drawn,
        ScalarFunctionSignature::exact(with_deadline, return_type()),
    ]
}

/// `thinkthen_decide(question, text)`: the database's boolean, `NULL` for
/// "not sure".
struct DecideScalar;

impl VScalar for DecideScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let budgets = deadline_column(input, 2)?;
        let distinct = distinct_of(&questions, &[], &texts, None, &budgets)?;
        let answers = judged_rows(&distinct)?;
        // Every row reads its own pair's answer through its slot: one
        // judgment per distinct text, one answer per row, so a repeated
        // text answers on every row that carries it and no row reads a
        // neighbor's slot.
        let len = texts.len();
        let row_value = |i: usize| -> Option<bool> {
            distinct.slots[i].and_then(|slot| answers[slot]).and_then(Answer::value)
        };
        {
            let mut out = output.flat_vector();
            let values = unsafe { out.as_mut_slice_with_len::<bool>(len) };
            for (i, place) in values.iter_mut().enumerate() {
                if let Some(value) = row_value(i) {
                    *place = value;
                }
            }
        }
        let mut out = output.flat_vector();
        for i in 0..len {
            if row_value(i).is_none() {
                out.set_null(i);
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        drawn_and_deadline(
            || vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            || LogicalTypeId::Boolean.into(),
        )
    }

    fn volatile() -> bool {
        true
    }
}

/// Every row's decide answer, drawn from the chunk's distinct pairs.
/// Each question group's calls carry the group's first pair's budget, so
/// a deadline column bounds the batch the group becomes.
fn judged_rows(distinct: &Distinct) -> std::result::Result<Vec<Option<Answer>>, String> {
    let mut answers: Vec<Option<Answer>> = vec![None; distinct.pairs.len()];
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for (slot, (question_at, _)) in distinct.pairs.iter().enumerate() {
        groups.entry(*question_at).or_default().push(slot);
    }
    for (question_at, slots) in groups {
        let texts: Vec<&str> = slots
            .iter()
            .map(|slot| distinct.pairs[*slot].1.as_str())
            .collect();
        let budget = slots.first().and_then(|slot| distinct.budgets.get(*slot)).copied().flatten();
        for (slot, judgment) in slots
            .iter()
            .zip(judged_with(&distinct.questions[question_at], &texts, budget)?)
        {
            answers[*slot] = Some(judgment.answer);
        }
    }
    Ok(answers)
}

/// `thinkthen_probability(question, text)`: the probability the backend
/// gave the yes side, and `ORDER BY` on it ranks the rows.
struct ProbabilityScalar;

impl VScalar for ProbabilityScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let budgets = deadline_column(input, 2)?;
        let distinct = distinct_of(&questions, &[], &texts, None, &budgets)?;
        let mut values: Vec<Option<f64>> = vec![None; distinct.pairs.len()];
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for (slot, (question_at, _)) in distinct.pairs.iter().enumerate() {
            groups.entry(*question_at).or_default().push(slot);
        }
        for (question_at, slots) in groups {
            let texts: Vec<&str> = slots
                .iter()
                .map(|slot| distinct.pairs[*slot].1.as_str())
                .collect();
            let budget = slots.first().and_then(|slot| distinct.budgets.get(*slot)).copied().flatten();
            for (slot, judgment) in slots
                .iter()
                .zip(judged_with(&distinct.questions[question_at], &texts, budget)?)
            {
                values[*slot] = Some(judgment.probability);
            }
        }
        // Each row's own slot, never the pair's index: see `judged_rows`.
        let len = texts.len();
        let row_value = |i: usize| -> Option<f64> { distinct.slots[i].and_then(|slot| values[slot]) };
        {
            let mut out = output.flat_vector();
            let slice = unsafe { out.as_mut_slice_with_len::<f64>(len) };
            for (i, place) in slice.iter_mut().enumerate() {
                if let Some(number) = row_value(i) {
                    *place = number;
                }
            }
        }
        let mut out = output.flat_vector();
        for i in 0..len {
            if row_value(i).is_none() {
                out.set_null(i);
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        drawn_and_deadline(
            || vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            || LogicalTypeId::Double.into(),
        )
    }

    fn volatile() -> bool {
        true
    }
}

/// `thinkthen_choose(question, text, options)`: the option that won,
/// `NULL` when nothing cleared the question's own rule.
struct ChooseScalar;

impl VScalar for ChooseScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let option_lists = read_list_strings(input, 2);
        let budgets = deadline_column(input, 3)?;
        let distinct = distinct_of(
            &questions,
            &option_lists,
            &texts,
            Some(QuestionKind::Choose),
            &budgets,
        )?;
        let mut picked: Vec<Option<Option<String>>> = vec![None; distinct.pairs.len()];
        for (slot, (question_at, text)) in distinct.pairs.iter().enumerate() {
            let opts = options_for(distinct.budgets[slot])?;
            let answer =
                engine_call(|engine| engine.choose_opts(&distinct.questions[*question_at], text, opts))?;
            picked[slot] = Some(answer);
        }
        let mut out = output.flat_vector();
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| picked[slot].clone()) {
                Some(Some(choice)) => {
                    out.insert(i, choice.as_str());
                }
                _ => out.set_null(i),
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        drawn_and_deadline(
            || vec![
                LogicalTypeId::Varchar.into(),
                LogicalTypeId::Varchar.into(),
                LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
            ],
            || LogicalTypeId::Varchar.into(),
        )
    }

    fn volatile() -> bool {
        true
    }
}

/// `thinkthen_score(question, text, levels)`: the specification's
/// probability-weighted position from 0 to K-1. The nearest level's name
/// rides in `thinkthen_details`.
struct ScoreScalar;

impl VScalar for ScoreScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let levels = read_list_strings(input, 2);
        let budgets = deadline_column(input, 3)?;
        let distinct = distinct_of(&questions, &levels, &texts, Some(QuestionKind::Score), &budgets)?;
        let mut scored: Vec<Option<Scored>> = vec![None; distinct.pairs.len()];
        for (slot, (question_at, text)) in distinct.pairs.iter().enumerate() {
            let opts = options_for(distinct.budgets[slot])?;
            let answer =
                engine_call(|engine| engine.score_opts(&distinct.questions[*question_at], text, opts))?;
            scored[slot] = Some(answer);
        }
        // Each row's own slot, never the pair's index: see `judged_rows`.
        let len = texts.len();
        let row_value = |i: usize| -> Option<f64> {
            distinct.slots[i].and_then(|slot| scored[slot].as_ref()).map(|score| score.value)
        };
        {
            let mut out = output.flat_vector();
            let slice = unsafe { out.as_mut_slice_with_len::<f64>(len) };
            for (i, place) in slice.iter_mut().enumerate() {
                if let Some(score) = row_value(i) {
                    *place = score;
                }
            }
        }
        let mut out = output.flat_vector();
        for i in 0..len {
            if row_value(i).is_none() {
                out.set_null(i);
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        drawn_and_deadline(
            || vec![
                LogicalTypeId::Varchar.into(),
                LogicalTypeId::Varchar.into(),
                LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
            ],
            || LogicalTypeId::Double.into(),
        )
    }

    fn volatile() -> bool {
        true
    }
}

/// `thinkthen_tag(question, text, labels)`: the labels that held, in the
/// question's own order.
struct TagScalar;

impl VScalar for TagScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let labels = read_list_strings(input, 2);
        let budgets = deadline_column(input, 3)?;
        let distinct = distinct_of(&questions, &labels, &texts, Some(QuestionKind::Tag), &budgets)?;
        let mut held: Vec<Option<Vec<String>>> = vec![None; distinct.pairs.len()];
        for (slot, (question_at, text)) in distinct.pairs.iter().enumerate() {
            let opts = options_for(distinct.budgets[slot])?;
            let answer =
                engine_call(|engine| engine.tag_opts(&distinct.questions[*question_at], text, opts))?;
            held[slot] = Some(answer);
        }
        let len = texts.len();
        let row_held: Vec<Option<Vec<String>>> = (0..len)
            .map(|i| distinct.slots[i].and_then(|slot| held[slot].clone()))
            .collect();
        let total: usize = row_held
            .iter()
            .map(|row| row.as_ref().map_or(0, Vec::len))
            .sum();

        let mut lists = output.list_vector();
        let child = lists.child(total.max(1));
        let mut offset = 0usize;
        for (i, row) in row_held.iter().enumerate() {
            match row {
                None => lists.set_null(i),
                Some(values) => {
                    lists.set_entry(i, offset, values.len());
                    for value in values {
                        child.insert(offset, value.as_str());
                        offset += 1;
                    }
                }
            }
        }
        lists.set_len(total);
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        drawn_and_deadline(
            || vec![
                LogicalTypeId::Varchar.into(),
                LogicalTypeId::Varchar.into(),
                LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
            ],
            || LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
        )
    }

    fn volatile() -> bool {
        true
    }
}

/// `thinkthen_annotate(set, text)`: one request per record, every question
/// with the same evidence riding in it, the evidence billed once.
///
/// The carrier is a JSON object, one field per question in the set's name
/// order, because a scalar's return type is declared at registration and
/// a set's questions are not known there. DuckDB's own JSON functions
/// read the object, so no field is out of reach.
struct AnnotateScalar;

impl VScalar for AnnotateScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let sets = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let budgets = deadline_column(input, 2)?;
        let mut distinct: HashMap<(String, String), usize> = HashMap::new();
        let mut unique: Vec<(String, String)> = Vec::new();
        let mut unique_budgets: Vec<Option<f64>> = Vec::new();
        let mut slots: Vec<Option<usize>> = vec![None; texts.len()];
        for i in 0..texts.len() {
            let (Some(arg), Some(text)) = (sets[i].as_deref(), texts[i].as_deref()) else {
                continue;
            };
            let budget = budgets.get(i).copied().flatten();
            let key = (arg.to_owned(), text.to_owned());
            let fresh = !distinct.contains_key(&key);
            let slot = *distinct.entry(key).or_insert_with(|| {
                unique.push((arg.to_owned(), text.to_owned()));
                unique_budgets.push(budget);
                unique.len() - 1
            });
            let _ = fresh;
            slots[i] = Some(slot);
        }
        let mut judged: Vec<Option<String>> = vec![None; unique.len()];
        for (slot, (arg, text)) in unique.iter().enumerate() {
            let set = if let Some(path) = arg.strip_prefix('@') {
                if let Some(refusal) = file_read_refusal(path) {
                    return Err(refusal.into());
                }
                let text = question_text_cached(path)?;
                QuestionSet::from_json(&text).map_err(failure)?
            } else if arg.starts_with('{') {
                QuestionSet::from_json(arg).map_err(failure)?
            } else {
                return Err(
                    "thinkthen usage: annotate names a question set as '@form.json' or JSON".into(),
                );
            };
            let opts = options_for(unique_budgets[slot])?;
            let records =
                engine_call(|engine| engine.annotate_opts(&set, &[text.as_str()], opts, None))?;
            judged[slot] = Some(annotate_json(&records[0]).map_err(failure)?);
        }
        let mut out = output.flat_vector();
        for (i, slot) in slots.iter().enumerate() {
            match slot.and_then(|slot| judged[slot].as_deref()) {
                Some(object) => {
                    out.insert(i, object);
                }
                None => out.set_null(i),
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        drawn_and_deadline(
            || vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            || LogicalTypeId::Varchar.into(),
        )
    }

    fn volatile() -> bool {
        true
    }
}

/// One annotate record as the JSON object SQL reads.
fn annotate_json(record: &[(String, Annotated)]) -> Result<String, EngineError> {
    let mut object = serde_json::Map::new();
    for (name, answer) in record {
        let value = match answer {
            Annotated::Decision(answer) => match answer.value() {
                Some(true) => serde_json::Value::Bool(true),
                Some(false) => serde_json::Value::Bool(false),
                None => serde_json::Value::Null,
            },
            Annotated::Choice(None) => serde_json::Value::Null,
            Annotated::Choice(Some(winner)) => serde_json::Value::String(winner.clone()),
            Annotated::Score(scored) => serde_json::json!({
                "position": scored.value,
                "nearest": scored.nearest,
            }),
            Annotated::Tags(held) => serde_json::json!(held),
            Annotated::Failed(failed) => serde_json::json!({ "failed": failed }),
        };
        object.insert(name.clone(), value);
    }
    serde_json::to_string(&serde_json::Value::Object(object))
        .map_err(|error| EngineError::defect(error.to_string()))
}

/// One details row as the SQL struct reads it: every member `Option`, so
/// the members that exist only for a decide question read NULL on the
/// others instead of a fake value.
#[derive(Clone)]
struct TrailRow {
    /// The yes-probability, when the question is a decide.
    probability: Option<f64>,
    /// The answer's word, when the question is a decide.
    answer: Option<String>,
    /// The model the request named.
    model: String,
    /// The question's digest with its threshold.
    digest: String,
    /// The nearest level's name, for a score question; NULL for every
    /// other verb. This is the contract's settled `nearest` field, and
    /// the struct member carries that name.
    nearest: Option<String>,
    /// The wire sends that produced the judgment, when the audit door
    /// could count them.
    sends: Option<u64>,
    /// The recording digests of the logical requests that produced the
    /// judgment (0053): always a list, one element for a one-request
    /// result, in construction order; a retry adds no element.
    requests: Vec<String>,
    /// Failed logical questions in this result (0054); always present,
    /// zero on this surface by construction, because a failed single
    /// question is a whole-call error here.
    failed_questions: u32,
}

/// `thinkthen_details(question, text)`: the audit trail, with the sends
/// that produced the judgment and, for a score question, the nearest
/// level's name. A score, choose, or tag reply carries no
/// yes-probability, so for those probability, answer, and sends read
/// NULL, and the audit fields — model, digest, nearest, requests,
/// failed_questions — come from the engine's own details door. A choose
/// or tag reply has no engine details yet, so the engine's own refusal
/// is the answer there.
struct DetailsScalar;

impl VScalar for DetailsScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let len = texts.len();
        let budgets = deadline_column(input, 2)?;
        let distinct = distinct_of(&questions, &[], &texts, None, &budgets)?;
        let mut trail: Vec<Option<TrailRow>> = vec![None; distinct.pairs.len()];
        for (slot, (question_at, text)) in distinct.pairs.iter().enumerate() {
            let question = &distinct.questions[*question_at];
            // Every kind's audit comes through the engine's own details
            // door, the contract's: a score reports its nearest level and
            // its request digests. A choose or tag question has no
            // details door in the engine yet, so the row reads NULL in
            // every audit member rather than failing the whole call
            // (review 3, finding 11: the README promises NULL and the
            // call raised the engine's refusal instead); the digest is
            // the question's own, so the row still names its question.
            // No surface invents an audit the engine did not give it.
            let details = if matches!(question.kind(), QuestionKind::Choose | QuestionKind::Tag) {
                None
            } else {
                let opts = options_for(distinct.budgets[slot])?;
                Some(
                    engine_call(|engine| {
                        ContractEngine::details_opts(engine, question, text, opts)
                    })?,
                )
            };
            let row = match details {
                Some(details) if question.kind() == QuestionKind::Decide => TrailRow {
                    probability: Some(details.probability),
                    answer: Some(details.answer.to_string()),
                    model: details.model,
                    digest: details.digest,
                    nearest: details.nearest,
                    sends: Some(u64::from(details.sends)),
                    requests: details.requests,
                    failed_questions: details.failed_questions,
                },
                Some(details) => TrailRow {
                    // A score reply carries no yes-probability and no
                    // sends, so those read NULL; the question's own audit
                    // fields come from the engine's own details.
                    probability: None,
                    answer: None,
                    model: details.model,
                    digest: details.digest,
                    nearest: details.nearest,
                    sends: None,
                    requests: details.requests,
                    failed_questions: details.failed_questions,
                },
                None => TrailRow {
                    // A choose or tag question: no engine details exist,
                    // so every audit member but the digest reads NULL.
                    probability: None,
                    answer: None,
                    model: String::new(),
                    digest: question.digest(),
                    nearest: None,
                    sends: None,
                    requests: Vec::new(),
                    failed_questions: 0,
                },
            };
            trail[slot] = Some(row);
        }
        {
            let mut parent = output.flat_vector();
            for (i, slot) in distinct.slots.iter().enumerate() {
                if slot.and_then(|slot| trail[slot].as_ref()).is_none() {
                    parent.set_null(i);
                }
            }
        }
        let structs = output.struct_vector();
        {
            let mut child = structs.child(0, len);
            // The member reads NULL everywhere a value does not exist,
            // never the buffer's zero (review 3, finding 11: a score's
            // probability read 0.0) and never NULL where one does
            // (review 4, finding 9: nulling first and writing values
            // after left the validity mask set, so decide's probability
            // and sends read NULL beside real values). The value lands,
            // then the null covers only what stayed absent.
            let values = unsafe { child.as_mut_slice_with_len::<f64>(len) };
            for value in values.iter_mut() {
                *value = 0.0;
            }
            for (i, slot) in distinct.slots.iter().enumerate() {
                if let Some(row) = slot.and_then(|slot| trail[slot].as_ref())
                    && let Some(number) = row.probability
                {
                    values[i] = number;
                }
            }
            // The slice's borrow ends here; the explicit drop was a
            // no-op on a reference (surfaces-review-4).
            for (i, slot) in distinct.slots.iter().enumerate() {
                if slot.and_then(|slot| trail[slot].as_ref()).and_then(|row| row.probability)
                    .is_none()
                {
                    child.set_null(i);
                }
            }
        }
        let mut child = structs.child(1, len);
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| trail[slot].as_ref()) {
                Some(row) => {
                    if let Some(answer) = row.answer.as_deref() {
                        child.insert(i, answer);
                    } else {
                        child.set_null(i);
                    }
                }
                None => child.set_null(i),
            }
        }
        let mut child = structs.child(2, len);
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| trail[slot].as_ref()) {
                // The engine always names a model, so an empty name is
                // the choose-or-tag row's honest NULL, never '' (review
                // 4, finding 9: the README promises NULL there).
                Some(row) if !row.model.is_empty() => {
                    child.insert(i, row.model.as_str());
                }
                _ => child.set_null(i),
            }
        }
        let mut child = structs.child(3, len);
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| trail[slot].as_ref()) {
                Some(row) => {
                    child.insert(i, row.digest.as_str());
                }
                None => child.set_null(i),
            }
        }
        let mut child = structs.child(4, len);
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| trail[slot].as_ref()) {
                Some(row) => match row.nearest.as_deref() {
                    Some(nearest) => {
                        child.insert(i, nearest);
                    }
                    None => child.set_null(i),
                },
                None => child.set_null(i),
            }
        }
        {
            let mut child = structs.child(5, len);
            // A member with no sends reads NULL, never the buffer's zero
            // (review 3, finding 11: a score's sends read 0), and a real
            // send count never reads NULL beside its value (review 4,
            // finding 9): the value lands, then the null covers only
            // what stayed absent.
            let values = unsafe { child.as_mut_slice_with_len::<u64>(len) };
            for value in values.iter_mut() {
                *value = 0;
            }
            for (i, slot) in distinct.slots.iter().enumerate() {
                if let Some(row) = slot.and_then(|slot| trail[slot].as_ref())
                    && let Some(sends) = row.sends
                {
                    values[i] = sends;
                }
            }
            // The slice's guard ends with the block; the explicit drop
            // was a no-op on a reference (surfaces-review-4).
            for (i, slot) in distinct.slots.iter().enumerate() {
                if slot.and_then(|slot| trail[slot].as_ref()).and_then(|row| row.sends)
                    .is_none()
                {
                    child.set_null(i);
                }
            }
        }
        {
            let rows: Vec<Option<&Vec<String>>> = distinct
                .slots
                .iter()
                .map(|slot| {
                    slot.and_then(|slot| trail[slot].as_ref())
                        .map(|row| &row.requests)
                })
                .collect();
            let total: usize = rows.iter().map(|row| row.map_or(0, Vec::len)).sum();
            let mut lists = structs.list_vector_child(6);
            {
                let child = lists.child(total.max(1));
                let mut offset = 0usize;
                for (i, row) in rows.iter().enumerate() {
                    match row {
                        None => lists.set_null(i),
                        Some(values) => {
                            lists.set_entry(i, offset, values.len());
                            for value in values.iter() {
                                child.insert(offset, value.as_str());
                                offset += 1;
                            }
                        }
                    }
                }
            }
            lists.set_len(total);
        }
        {
            let mut child = structs.child(7, len);
            let values = unsafe { child.as_mut_slice_with_len::<u64>(len) };
            for (i, slot) in distinct.slots.iter().enumerate() {
                if let Some(row) = slot.and_then(|slot| trail[slot].as_ref()) {
                    values[i] = u64::from(row.failed_questions);
                }
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        drawn_and_deadline(
            || vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            || LogicalTypeHandle::struct_type(&[
                ("probability", LogicalTypeId::Double.into()),
                ("answer", LogicalTypeId::Varchar.into()),
                ("model", LogicalTypeId::Varchar.into()),
                ("digest", LogicalTypeId::Varchar.into()),
                ("nearest", LogicalTypeId::Varchar.into()),
                ("sends", LogicalTypeId::UBigint.into()),
                (
                    "requests",
                    LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
                ),
                ("failed_questions", LogicalTypeId::UBigint.into()),
            ]),
        )
    }

    fn volatile() -> bool {
        true
    }
}

/// The extension's entrypoint, hand-written where the macro-generated one
/// hides the raw connection the aggregate and the table function need.
/// The entrypoint is contained like the other C boundaries: a panic
/// during load becomes a load error, never an abort.
#[unsafe(no_mangle)]
/// # Safety
///
/// DuckDB calls this at LOAD with its own extension info and access
/// table; the pointers are valid for the call's duration only.
pub unsafe extern "C" fn thinkthen_init_c_api(
    info: ffi::duckdb_extension_info,
    access: *const ffi::duckdb_extension_access,
) -> bool {
    // The entrypoint's own panic guard, through the contract's one
    // panic-to-text: a load panic reports as the load error, never an
    // unwind into the database.
    let mut reported: Result<(), String> = Ok(());
    let guarded = thinkthen_contract::catch_panic("the extension load", || {
        reported = unsafe { init(info, access) }.map_err(|error| error.to_string());
        Ok(())
    });
    match guarded {
        Ok(()) if reported.is_ok() => true,
        Ok(()) => {
            let message = reported.expect_err("the error path");
            unsafe { report_load_error(info, access, &message) };
            false
        }
        Err(error) => {
            unsafe { report_load_error(info, access, &format!("thinkthen defect: {}", error.message)) };
            false
        }
    }
}

/// Report a load failure through the extension access, when it offers the
/// call; DuckDB's own load error names the file either way.
unsafe fn report_load_error(
    info: ffi::duckdb_extension_info,
    access: *const ffi::duckdb_extension_access,
    message: &str,
) {
    unsafe {
        if access.is_null() {
            return;
        }
        if let Some(set_error) = (*access).set_error
            && let Ok(text) = std::ffi::CString::new(message)
        {
            set_error(info, text.as_ptr());
        }
    }
}

unsafe fn init(
    info: ffi::duckdb_extension_info,
    access: *const ffi::duckdb_extension_access,
) -> Result<(), Box<dyn Error>> {
    guard::test_arm("extension load");
    unsafe { install_interrupt_handler() };
    if !unsafe { ffi::duckdb_rs_extension_api_init(info, access, "v1.5.5") }? {
        return Ok(());
    }
    let get_database =
        unsafe { (*access).get_database }.ok_or("the extension access names no get_database")?;
    let database = unsafe { *get_database(info) };
    let connection = unsafe { Connection::open_from_raw(database) }?;
    connection.register_scalar_function::<DecideScalar>("thinkthen_decide")?;
    connection.register_scalar_function::<ProbabilityScalar>("thinkthen_probability")?;
    connection.register_scalar_function::<ChooseScalar>("thinkthen_choose")?;
    connection.register_scalar_function::<ScoreScalar>("thinkthen_score")?;
    connection.register_scalar_function::<TagScalar>("thinkthen_tag")?;
    connection.register_scalar_function::<AnnotateScalar>("thinkthen_annotate")?;
    connection.register_scalar_function::<DetailsScalar>("thinkthen_details")?;
    connection.register_scalar_function::<recognize::RecognizeScalar>("thinkthen_recognize")?;
    connection.register_scalar_function::<relations::RelationsScalar>("thinkthen_relations")?;

    // The relate, usage, and warm functions run on a connection kept per
    // loaded database, opened here: the database pointer `get_database`
    // hands out belongs to the load state and can dangle after init, and
    // one process-global connection used to serve whichever database
    // loaded last (review finding: relate queried the wrong database).
    let raw = connections::register(database)?;
    let table = unsafe { usage::register(raw) };
    let aggregate = unsafe { warm::register(raw) };
    let relate_function = unsafe { relate::register(raw) };
    table?;
    aggregate?;
    relate_function?;
    Ok(())
}

#[cfg(test)]
mod mapping_tests {
    use super::*;

    /// Ruling 2 of the product rulings: the defect kind maps to this
    /// engine's own error surface, with the kind named in the message. No
    /// public door carries a fault hook; this proves the mapping at the
    /// shim level.
    #[test]
    fn the_defect_kind_maps_to_the_engines_error() {
        let text = failure(EngineError::defect("the relate plan lost its bind data"));
        assert!(text.contains("thinkthen defect:"), "{text}");
        assert!(text.contains("the relate plan lost its bind data"), "{text}");
    }
}

#[cfg(test)]
mod cancel_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    /// The fast-backend cancel proof (lane B item 5, the poll-bug shape)
    /// at the engine this surface calls: on the null backend the wait's
    /// channel never idles, so the poll tick must run on the busy arm too
    /// and a token set mid-batch must end the call within about a tick —
    /// not after the whole batch, which is what the bug did (SIGINT one
    /// second into a three-million-record null batch surfaced 8.48 s
    /// later, at batch end). Width 1 stretches the full batch so the
    /// contrast is unambiguous: the broken path drains it all.
    ///
    /// This lives here and not in a CLI script because DuckDB's own
    /// abort ends a native query on SIGINT within a chunk, so a CLI
    /// timing test cannot isolate the engine's tick on a fast backend;
    /// the stub-backed wire suite proves the wire-side shape.
    #[test]
    fn a_fast_backend_hears_a_cancel_within_a_tick() {
        // Before any engine or config is built in this process.
        unsafe {
            std::env::set_var("ENGINE_NULL", "1");
            std::env::set_var("ENGINE_WIDTH", "1");
        }
        let engine = thinkthen_contract::Connector::connect(
            &StandinConnector,
            &EngineConfig::from_env(),
        )
        .expect("the connector answers");
        let question = Question::from_json(r#"{"decide":"Is this a complaint?"}"#)
            .expect("the question parses");
        let texts: Vec<String> = (0..1_000_000).map(|i| format!("record {i}")).collect();
        let records: Vec<&str> = (0..8_000_000).map(|i| texts[i % texts.len()].as_str()).collect();
        let token = Cancel::new();
        let fired = Arc::new(Mutex::new(None::<Instant>));
        let setter = {
            let token = token.clone();
            let fired = Arc::clone(&fired);
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(150));
                token.cancel();
                *fired.lock().expect("the fired stamp") = Some(Instant::now());
            })
        };
        let outcome = engine.decide_many_opts(
            &question,
            &records,
            Options::new().cancel(&token),
            None,
        );
        setter.join().expect("the setter joins");
        let fired_at = fired
            .lock()
            .expect("the fired stamp")
            .expect("the token fired");
        let heard = Instant::now().duration_since(fired_at);
        let error = outcome.expect_err("a cancelled batch returns the cancelled kind");
        assert_eq!(error.kind.to_string(), "cancelled", "{}", error.message);
        assert!(
            heard <= Duration::from_millis(1_500),
            "the interrupt waited {heard:?} past the token; a fast backend starved the poll"
        );
    }
}

#[cfg(test)]
mod signal_tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    /// The host's own handler in this test process: quiet, so the chain
    /// after ours does not end the test binary.
    extern "C" fn quiet_host(_: libc::c_int) {}

    /// Review 5, finding 1: the SIGINT handler walked the kept-connection
    /// registry under its mutex and allocated, so a signal landing on a
    /// thread that held the registry deadlocked that thread against
    /// itself. Two hundred signals land while the raising thread holds
    /// the registry and a second thread churns the same lock; the whole
    /// run must end well inside the timeout.
    #[test]
    fn a_sigint_landing_while_the_registry_is_held_returns() {
        let _alone = connections::SIGNAL_TESTS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        unsafe {
            let mut host: libc::sigaction = std::mem::zeroed();
            host.sa_sigaction = quiet_host as extern "C" fn(libc::c_int) as usize;
            libc::sigemptyset(&mut host.sa_mask);
            libc::sigaction(libc::SIGINT, &host, std::ptr::null_mut());
            install_interrupt_handler();
        }
        let stop = Arc::new(AtomicBool::new(false));
        {
            let stop = Arc::clone(&stop);
            thread::spawn(move || {
                while !stop.load(Ordering::SeqCst) {
                    connections::with_registry_held(thread::yield_now);
                }
            });
        }
        let (done, finished) = mpsc::channel();
        thread::spawn(move || {
            for _ in 0..200 {
                connections::with_registry_held(|| unsafe {
                    libc::raise(libc::SIGINT);
                });
            }
            let _ = done.send(());
        });
        let outcome = finished.recv_timeout(Duration::from_secs(10));
        stop.store(true, Ordering::SeqCst);
        assert!(
            outcome.is_ok(),
            "a SIGINT landing on the thread holding the kept-connection registry deadlocked the handler"
        );
    }
}
