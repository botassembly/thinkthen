//! One call per function: the answer goes into the caller's own variable.
#![allow(
    unsafe_code,
    reason = "one-call exports borrow C strings, counted text and exclusive answer slots"
)]
use crate::{
    ClientHandle, HostError, QuestionHandle, RuleInput, SpanView, TextView, ValueHandle,
    builders::relation_rule,
    exports::operation,
    memory::bytes,
    single::{evaluate, status},
};
use std::ffi::{CStr, c_char};
use thinkthen::{
    Code, Error, FunctionName,
    native::{self, QuestionBuilder},
    results::{AnyRow, Outcome},
};

fn missing() -> Error {
    native::argument_error(Code::UsageArgument)
}

// A NUL-terminated C string argument; NULL is a usage error, not a crash.
unsafe fn string<'a>(value: *const c_char) -> Result<&'a str, Error> {
    if value.is_null() {
        return Err(missing());
    }
    /* SAFETY: the caller lends a NUL-terminated string until return. */
    unsafe { CStr::from_ptr(value) }
        .to_str()
        // Text that is not UTF-8 is usage.abi here too, as in every other C function.
        .map_err(|_| native::abi_error())
}

// An array of count NUL-terminated strings.
unsafe fn strings<'a>(values: *const *const c_char, count: usize) -> Result<Vec<&'a str>, Error> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if values.is_null() {
        return Err(missing());
    }
    /* SAFETY: the caller lends count readable string pointers until return. */
    unsafe {
        std::slice::from_raw_parts(values, count)
            .iter()
            .map(|value| string(*value))
            .collect()
    }
}

// Build a question from its wording and one list, through the engine's own admission.
unsafe fn question(
    function: FunctionName,
    wording: &str,
    items: *const *const c_char,
    count: usize,
) -> Result<QuestionBuilder, Error> {
    let mut builder = QuestionBuilder::new(function, wording)?;
    let domain = match function {
        FunctionName::Rank => FunctionName::Score,
        other => other,
    };
    /* SAFETY: the caller lends count readable strings until return. */
    for item in unsafe { strings(items, count) }? {
        builder.add_item(domain, item)?;
    }
    Ok(builder)
}

// Run one text and return the answered row, or the status to return instead.
unsafe fn run(
    client: *const ClientHandle,
    builder: &QuestionBuilder,
    data: *const u8,
    len: usize,
) -> Result<Result<AnyRow, i32>, Error> {
    if client.is_null() {
        return Err(missing());
    }
    /* SAFETY: the caller lends a live client and counted text until return. */
    unsafe {
        let record =
            crate::finite::admit_record(data, len, thinkthen::records::RecordKind::Text.abi())?;
        let row = evaluate(&(*client).0, builder.build()?, record)?;
        match status(&row)? {
            crate::exports::OK => Ok(Ok(row)),
            other => Ok(Err(other)),
        }
    }
}

macro_rules! answered {
    ($row:expr, $variant:ident) => {
        match $row {
            AnyRow::$variant(row) => match row.outcome() {
                Outcome::Answered { answer, .. } => Ok(answer.clone()),
                _ => Err(native::internal_error()),
            },
            _ => Err(native::internal_error()),
        }
    };
}

// Write an answer only when there is one; UNSURE and failures leave the caller's variable alone.
unsafe fn answer<T>(
    error: *mut HostError,
    out: *mut T,
    run: impl FnOnce() -> Result<Result<T, i32>, Error>,
) -> i32 {
    /* SAFETY: the caller lends an exclusive answer slot until return. */
    unsafe {
        operation(error, || {
            if out.is_null() {
                return Err(missing());
            }
            match run()? {
                Ok(value) => {
                    out.write(value);
                    Ok(crate::exports::OK)
                }
                Err(status) => Ok(status),
            }
        })
    }
}

/// Ask a yes-or-no question about one text, and write the answer to *answer.
///
/// Returns THINKTHEN_OK with *answer written; THINKTHEN_UNSURE with *answer unchanged (the
/// default cut never gives it; set a band with thinkthen_question_set_band and
/// thinkthen_run_one); or a negative thinkthen_code with *answer unchanged:
/// -THINKTHEN_CODE_RECORD_EMPTY for empty or blank text (never sent to the backend), a
/// usage code for a bad argument, or a backend code.
/// A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
/// # Safety
/// Any thread may call this, and threads may share the client. question is a NUL-terminated
/// string. text points to len readable bytes (or a NUL-terminated string with
/// THINKTHEN_NUL_TERMINATED). answer is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it
/// again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide(
    client: *const ClientHandle,
    question: *const c_char,
    text: *const u8,
    len: usize,
    answer: *mut bool,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends every input until return. */
    unsafe {
        self::answer(error, answer, || {
            let builder =
                self::question(FunctionName::Decide, string(question)?, std::ptr::null(), 0)?;
            match run(client, &builder, text, len)? {
                Err(status) => Ok(Err(status)),
                Ok(row) => answered!(&row, Decide).map(Ok),
            }
        })
    }
}

/// Choose one of count choices for one text, and write the chosen index to *choice.
///
/// *choice is an index into choices, so the caller's own string names the answer. Returns
/// THINKTHEN_OK with *choice written; THINKTHEN_UNSURE with *choice unchanged when no
/// choice reaches the cut; or a negative thinkthen_code with *choice unchanged:
/// usage.question for fewer than two or repeated choices, -THINKTHEN_CODE_RECORD_EMPTY for
/// blank text, or a backend code.
/// A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
/// # Safety
/// Any thread may call this, and threads may share the client. question and each of the
/// count choices are NUL-terminated strings. text points to len readable bytes (or a NUL-
/// terminated string with THINKTHEN_NUL_TERMINATED). choice is writable. error is NULL or
/// initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it
/// with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_choose(
    client: *const ClientHandle,
    question: *const c_char,
    choices: *const *const c_char,
    count: usize,
    text: *const u8,
    len: usize,
    choice: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends every input until return. */
    unsafe {
        answer(error, choice, || {
            let builder = self::question(FunctionName::Choose, string(question)?, choices, count)?;
            match run(client, &builder, text, len)? {
                Err(status) => Ok(Err(status)),
                Ok(row) => {
                    let label: String = answered!(&row, Choose)?;
                    strings(choices, count)?
                        .iter()
                        .position(|choice| *choice == label)
                        .map(Ok)
                        .ok_or_else(native::internal_error)
                }
            }
        })
    }
}

/// Tag one text with any of count labels, and write whether each applies to selected.
///
/// selected has count entries, in the order of labels; an answer with no labels sets them
/// all to false. Returns THINKTHEN_OK with selected written, or a negative thinkthen_code
/// with selected unchanged: usage.question for no or repeated labels,
/// -THINKTHEN_CODE_RECORD_EMPTY for blank text, or a backend code.
/// A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
/// # Safety
/// Any thread may call this, and threads may share the client. question and each of the
/// count labels are NUL-terminated strings. text points to len readable bytes (or a NUL-
/// terminated string with THINKTHEN_NUL_TERMINATED). selected points to count writable
/// bools. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_tag(
    client: *const ClientHandle,
    question: *const c_char,
    labels: *const *const c_char,
    count: usize,
    text: *const u8,
    len: usize,
    selected: *mut bool,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends every input and count writable answers until return. */
    unsafe {
        operation(error, || {
            if selected.is_null() && count > 0 {
                return Err(missing());
            }
            let builder = self::question(FunctionName::Tag, string(question)?, labels, count)?;
            match run(client, &builder, text, len)? {
                Err(status) => Ok(status),
                Ok(row) => {
                    let chosen: Vec<String> = answered!(&row, Tag)?;
                    for (index, label) in strings(labels, count)?.iter().enumerate() {
                        selected
                            .add(index)
                            .write(chosen.iter().any(|value| value == label));
                    }
                    Ok(crate::exports::OK)
                }
            }
        })
    }
}

/// Score one text on count ordered levels, and write the score to *score.
///
/// The score runs from 0 for the first level to 1 for the last. Returns THINKTHEN_OK with
/// *score written, or a negative thinkthen_code with *score unchanged: usage.question for
/// fewer than two or repeated levels, -THINKTHEN_CODE_RECORD_EMPTY for blank text, or a
/// backend code.
/// A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
/// # Safety
/// Any thread may call this, and threads may share the client. question and each of the
/// count levels are NUL-terminated strings. text points to len readable bytes (or a NUL-
/// terminated string with THINKTHEN_NUL_TERMINATED). score is writable. error is NULL or
/// initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it
/// with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_score(
    client: *const ClientHandle,
    question: *const c_char,
    levels: *const *const c_char,
    count: usize,
    text: *const u8,
    len: usize,
    score: *mut f64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends every input until return. */
    unsafe {
        answer(error, score, || {
            let builder = self::question(FunctionName::Score, string(question)?, levels, count)?;
            match run(client, &builder, text, len)? {
                Err(status) => Ok(Err(status)),
                Ok(row) => answered!(&row, Score).map(Ok),
            }
        })
    }
}

/// Find the part of one text that answers a question, and write it to *found.
///
/// found->present is 1 when a part answers it and 0 when the answer is that none does.
/// found->start and found->end are byte offsets into text, and found->text points into the
/// caller's own text, so it lives as long as text does. Returns THINKTHEN_OK with *found
/// written; THINKTHEN_UNSURE with *found unchanged; or a negative thinkthen_code with
/// *found unchanged: -THINKTHEN_CODE_RECORD_EMPTY for blank text, a usage code for a bad
/// argument, or a backend code.
/// A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
/// # Safety
/// Any thread may call this, and threads may share the client. question is a NUL-terminated
/// string. text points to len readable bytes (or a NUL-terminated string with
/// THINKTHEN_NUL_TERMINATED). found is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it
/// again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_find(
    client: *const ClientHandle,
    question: *const c_char,
    text: *const u8,
    len: usize,
    found: *mut SpanView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends every input until return; the span borrows text. */
    unsafe {
        answer(error, found, || {
            let builder =
                self::question(FunctionName::Find, string(question)?, std::ptr::null(), 0)?;
            match run(client, &builder, text, len)? {
                Err(status) => Ok(Err(status)),
                Ok(row) => Ok(Ok(match answered!(&row, Find)? {
                    None => SpanView::default(),
                    Some(part) => {
                        let input = bytes(text, len);
                        let (start, end) = (part.start(), part.end());
                        let matched = input
                            .get(start..end)
                            .filter(|matched| *matched == part.text().as_bytes())
                            .ok_or_else(native::internal_error)?;
                        SpanView {
                            present: 1,
                            text: TextView {
                                data: matched.as_ptr(),
                                len: matched.len(),
                            },
                            start: start as u64,
                            end: end as u64,
                        }
                    }
                })),
            }
        })
    }
}

// Publish an answered row as a value handle the caller frees.
unsafe fn owner(
    error: *mut HostError,
    out: *mut *mut ValueHandle,
    run: impl FnOnce() -> Result<Result<AnyRow, i32>, Error>,
) -> i32 {
    /* SAFETY: the caller lends an exclusive handle slot until return. */
    unsafe {
        answer(error, out, || {
            Ok(run()?.map(|row| Box::into_raw(Box::new(ValueHandle::from_any(row)))))
        })
    }
}

/// Recognize names of count kinds in one text, and write them to a new value.
///
/// Read the names with thinkthen_value_name_count and thinkthen_value_name_at; their
/// offsets are bytes into text. Free the value with thinkthen_value_free. Returns
/// THINKTHEN_OK with *names set (an answer may hold no names), or a negative thinkthen_code
/// with *names unchanged: usage.question for no or repeated kinds,
/// -THINKTHEN_CODE_RECORD_EMPTY for blank text, or a backend code.
/// A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
/// # Safety
/// Any thread may call this, and threads may share the client. Each of the count kinds is a
/// NUL-terminated string. text points to len readable bytes (or a NUL-terminated string
/// with THINKTHEN_NUL_TERMINATED). names is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it
/// again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_recognize(
    client: *const ClientHandle,
    kinds: *const *const c_char,
    count: usize,
    text: *const u8,
    len: usize,
    names: *mut *mut ValueHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends every input until return. */
    unsafe {
        owner(error, names, || {
            let builder = question(FunctionName::Recognize, "", kinds, count)?;
            run(client, &builder, text, len)
        })
    }
}

/// Find relations of count rules in one text, and write them to a new value.
///
/// Read the relations with thinkthen_value_relation_count and thinkthen_value_relation_at,
/// and the names they join with thinkthen_value_name_count and thinkthen_value_name_at.
/// Free the value with thinkthen_value_free. Returns THINKTHEN_OK with *relations set (an
/// answer may hold no relations), or a negative thinkthen_code with *relations unchanged:
/// usage.question for no rules or a bad rule, -THINKTHEN_CODE_RECORD_EMPTY for blank text,
/// or a backend code.
/// A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
/// # Safety
/// Any thread may call this, and threads may share the client. rules points to count
/// readable rules whose text fields are readable. text points to len readable bytes (or a
/// NUL-terminated string with THINKTHEN_NUL_TERMINATED). relations is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code
/// and message; passing it again frees what it held, so a reused error leaks nothing. Free
/// it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_relate(
    client: *const ClientHandle,
    rules: *const RuleInput,
    count: usize,
    text: *const u8,
    len: usize,
    relations: *mut *mut ValueHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends every input until return. */
    unsafe {
        owner(error, relations, || {
            let mut builder = QuestionBuilder::new(FunctionName::Relate, "")?;
            if count > 0 && rules.is_null() {
                return Err(missing());
            }
            for index in 0..count {
                builder.add_rule(relation_rule(&*rules.add(index))?)?;
            }
            run(client, &builder, text, len)
        })
    }
}

/// Ask count named questions about one text, and write their answers to a new value.
///
/// Read each answer by name with thinkthen_value_annotation_get, or by index with
/// thinkthen_value_annotation_at; each child follows its own function's rules. Free the
/// value with thinkthen_value_free. Returns THINKTHEN_OK with *annotation set, or a
/// negative thinkthen_code with *annotation unchanged: usage.name for repeated or bad
/// names, usage.question for a rank or annotate child, -THINKTHEN_CODE_RECORD_EMPTY for
/// blank text, or a backend code.
/// A string argument that is not UTF-8 returns -THINKTHEN_CODE_USAGE_ABI.
/// # Safety
/// Any thread may call this, and threads may share the client and the questions while no
/// thread changes them. names holds count NUL-terminated strings and questions count live
/// question handles. text points to len readable bytes (or a NUL-terminated string with
/// THINKTHEN_NUL_TERMINATED). annotation is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it
/// again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_annotate(
    client: *const ClientHandle,
    names: *const *const c_char,
    questions: *const *const QuestionHandle,
    count: usize,
    text: *const u8,
    len: usize,
    annotation: *mut *mut ValueHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends every input until return. */
    unsafe {
        owner(error, annotation, || {
            let mut builder = QuestionBuilder::new(FunctionName::Annotate, "")?;
            if count > 0 && questions.is_null() {
                return Err(missing());
            }
            for (index, name) in strings(names, count)?.into_iter().enumerate() {
                let child = *questions.add(index);
                if child.is_null() {
                    return Err(missing());
                }
                builder.add_entry(name, (*child).build()?)?;
            }
            run(client, &builder, text, len)
        })
    }
}
