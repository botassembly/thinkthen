//! Typed scalar, collection and annotation answers.
#![allow(
    unsafe_code,
    reason = "answer exports borrow live value handles and exclusive output slots"
)]
use super::{NameView, Owned, RelationView, SpanView, ValueHandle, answer_project, views};
use crate::{
    HostError,
    builders::TextView,
    exports::{THINKTHEN_OK, operation},
    memory::live,
};
use thinkthen::{FunctionName, native, questions};
fn names(value: &ValueHandle) -> Result<&[thinkthen::results::Name], thinkthen::Error> {
    match value.function()? {
        FunctionName::Recognize => value.answer::<questions::Recognize>().map(Vec::as_slice),
        FunctionName::Relate => Ok(value
            .detail::<questions::Relate>()?
            .map_or(&[], |detail| detail.names().as_slice())),
        _ => Err(native::argument_error(thinkthen::Code::UsageFunction)),
    }
}
/// Write a choose value's choice to *out.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. *out borrows
/// from value and lasts until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_choice(
    value: *const ValueHandle,
    out: *mut TextView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value owns the string and the caller owns the output slots. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(views::text(live(value)?.answer::<questions::Choose>()?))
        })
    }
}
/// Write a decide or filter value's answer to *out: 1 for yes and 0 for no.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_bool(
    value: *const ValueHandle,
    out: *mut u8,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(u8::from(match live(value)?.function()? {
                FunctionName::Decide => *live(value)?.answer::<questions::Decide>()?,
                FunctionName::Filter => *live(value)?.answer::<questions::Filter>()?,
                _ => return Err(native::argument_error(thinkthen::Code::UsageFunction)),
            }))
        })
    }
}
/// Write a score value's score, from 0 to 1, to *out.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_number(
    value: *const ValueHandle,
    out: *mut f64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the value is borrowed and output slots are exclusive. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(*live(value)?.answer::<questions::Score>()?)
        })
    }
}
/// Write a rank value's rank, 1 for the first, to *out.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_rank(
    value: *const ValueHandle,
    out: *mut u64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(live(value)?.answer::<questions::Rank>()?.get() as u64)
        })
    }
}
/// Write how many labels a tag value applies to *out; 0 is a real answer.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_tag_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(live(value)?.answer::<questions::Tag>()?.len())
        })
    }
}
/// Write a tag value's label at index to *out.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when index is out of range. *out borrows from value and lasts
/// until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_tag_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut TextView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value owns the returned text; output slots are exclusive. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(views::text(
                live(value)?
                    .answer::<questions::Tag>()?
                    .get(index)
                    .ok_or_else(|| native::argument_error(thinkthen::Code::UsageArgument))?,
            ))
        })
    }
}
/// Write a find value's answer to *out; out->present is 0 when the answer is that no part
/// answers it.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's. *out borrows
/// from value and lasts until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_found(
    value: *const ValueHandle,
    out: *mut SpanView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value owns any returned text; output slots are exclusive. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(match live(value)?.answer::<questions::Find>()? {
                None => SpanView::default(),
                Some(found) => SpanView {
                    present: 1,
                    text: views::text(found.text()),
                    start: found.start() as u64,
                    end: found.end() as u64,
                },
            })
        })
    }
}
/// Write how many names a recognize value, or a relate value's detail, holds to *out.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_name_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe { answer_project(value, out, error, || Ok(names(live(value)?)?.len())) }
}
/// Write the name at index, with its kind and probability, to *out.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when index is out of range. *out borrows from value and lasts
/// until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_name_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut NameView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: all text views borrow value until cleanup. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(views::name(names(live(value)?)?.get(index).ok_or_else(
                || native::argument_error(thinkthen::Code::UsageArgument),
            )?))
        })
    }
}
/// Write how many relations a relate value holds to *out.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_relation_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(live(value)?.answer::<questions::Relate>()?.len())
        })
    }
}
/// Write the relation at index, with both names it joins, to *out.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when index is out of range. *out borrows from value and lasts
/// until thinkthen_value_free(value); never free it.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_relation_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut RelationView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: all returned text borrows value until cleanup. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(views::relation(
                live(value)?
                    .answer::<questions::Relate>()?
                    .get(index)
                    .ok_or_else(|| native::argument_error(thinkthen::Code::UsageArgument))?,
            ))
        })
    }
}
/// Write how many named answers an annotate value holds, failed ones included, to *out.
///
/// Returns THINKTHEN_OK with *out written; THINKTHEN_UNSURE with *out unchanged when the value
/// is not sure; or a negative code with *out unchanged: the record's own code when the value
/// failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is a live value. out
/// is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_annotation_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        answer_project(value, out, error, || {
            Ok(live(value)?.answer::<questions::Annotate>()?.len())
        })
    }
}
/// Write the name of the answer at index to *name and a copy of the answer to *out.
///
/// Returns THINKTHEN_OK with *name and *out written; THINKTHEN_UNSURE with *out unchanged when
/// the value is not sure; or a negative code with *out unchanged: the record's own code when
/// the value failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is another function's, or
/// -THINKTHEN_CODE_USAGE_ARGUMENT when index is out of range. *name borrows from value and
/// lasts until thinkthen_value_free(value). The caller owns *out, which follows its own
/// function's rules, and frees it with thinkthen_value_free; an old handle in *out is not
/// freed.
/// # Safety
/// Any thread may call this while no thread changes or frees value. value is live. name and out
/// are writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return
/// it holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_annotation_at(
    value: *const ValueHandle,
    index: usize,
    name: *mut TextView,
    out: *mut *mut ValueHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the borrowed name follows parent lifetime; child receives a unique snapshot. */
    unsafe {
        operation(error, || {
            if live(value)?.status()? == thinkthen::results::Status::Unsure {
                return Ok(crate::THINKTHEN_UNSURE);
            }
            if let Some(error) = live(value)?.error()? {
                return Err(error.clone());
            }
            out.write(std::ptr::null_mut());
            let (key, entry) = live(value)?
                .answer::<questions::Annotate>()?
                .iter()
                .nth(index)
                .ok_or_else(|| native::argument_error(thinkthen::Code::UsageArgument))?;
            name.write(views::text(key));
            out.write(Box::into_raw(Box::new(ValueHandle(
                Owned::Entry(entry.clone()),
                None,
            ))));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Look up one annotation answer by its question's name, and copy it into a new value.
///
/// The new value follows its own function's rules, so read it with that function's accessor.
/// Returns THINKTHEN_OK with *out set; THINKTHEN_UNSURE with *out set to NULL when the
/// annotation itself is not sure; or a negative code with *out set to NULL: the record's own
/// code when it failed, -THINKTHEN_CODE_USAGE_FUNCTION when the value is not an annotation,
/// -THINKTHEN_CODE_USAGE_NAME when no question has that name, or -THINKTHEN_CODE_USAGE_ABI when
/// the text is not UTF-8. The caller owns *out and frees it with thinkthen_value_free; an old
/// handle in *out is not freed.
/// # Safety
/// Any thread may call this while no thread changes or frees value. name points to len readable
/// bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied
/// before return. out is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_annotation_get(
    value: *const ValueHandle,
    name: *const u8,
    len: usize,
    out: *mut *mut ValueHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the parent value and name are borrowed; the child is a unique snapshot. */
    unsafe {
        operation(error, || {
            out.write(std::ptr::null_mut());
            if live(value)?.status()? == thinkthen::results::Status::Unsure {
                return Ok(crate::THINKTHEN_UNSURE);
            }
            if let Some(error) = live(value)?.error()? {
                return Err(error.clone());
            }
            let name = crate::memory::text(name, len)?;
            let entry = live(value)?
                .answer::<questions::Annotate>()?
                .get(name)
                .ok_or_else(|| native::argument_error(thinkthen::Code::UsageName))?;
            out.write(Box::into_raw(Box::new(ValueHandle(
                Owned::Entry(entry.clone()),
                None,
            ))));
            Ok(THINKTHEN_OK)
        })
    }
}
