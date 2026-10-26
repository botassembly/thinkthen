//! Typed scalar, collection and annotation answers.
#![allow(
    unsafe_code,
    reason = "answer exports borrow live value handles and exclusive output slots"
)]
use super::{NameView, Owned, RelationView, SpanView, ValueHandle, project, views};
use crate::{
    HostError,
    builders::TextView,
    exports::{OK, operation},
};
use thinkthen::{FunctionName, native, questions};
fn names(value: &ValueHandle) -> Result<&[thinkthen::results::Name], thinkthen::Error> {
    match value.function()? {
        FunctionName::Recognize => value.answer::<questions::Recognize>().map(Vec::as_slice),
        FunctionName::Relate => Ok(value
            .detail::<questions::Relate>()?
            .map_or(&[], |detail| detail.names().as_slice())),
        _ => Err(native::abi_error()),
    }
}
/// Borrow an answered choose label until value cleanup.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_choice(
    value: *const ValueHandle,
    out: *mut TextView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value owns the string and the caller owns the output slots. */
    unsafe {
        project(out, error, || {
            Ok(views::text((*value).answer::<questions::Choose>()?))
        })
    }
}
/// Read an answered decide or filter boolean as zero or one.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_bool(
    value: *const ValueHandle,
    out: *mut u8,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok(u8::from(match (*value).function()? {
                FunctionName::Decide => *(*value).answer::<questions::Decide>()?,
                FunctionName::Filter => *(*value).answer::<questions::Filter>()?,
                _ => return Err(native::abi_error()),
            }))
        })
    }
}
/// Read an answered score without an aggregate probability.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_number(
    value: *const ValueHandle,
    out: *mut f64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the value is borrowed and output slots are exclusive. */
    unsafe { project(out, error, || Ok(*(*value).answer::<questions::Score>()?)) }
}
/// Read an answered rank, independent of its original input position.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_rank(
    value: *const ValueHandle,
    out: *mut u64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok((*value).answer::<questions::Rank>()?.get() as u64)
        })
    }
}
/// Count the answered tag labels, including a legitimate empty answer.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_tag_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok((*value).answer::<questions::Tag>()?.len())
        })
    }
}
/// Borrow one answered tag label until value cleanup.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_tag_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut TextView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value owns the returned text; output slots are exclusive. */
    unsafe {
        project(out, error, || {
            Ok(views::text(
                (*value)
                    .answer::<questions::Tag>()?
                    .get(index)
                    .ok_or_else(native::abi_error)?,
            ))
        })
    }
}
/// Borrow a find answer; present zero means answered null, distinct from unsure.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_found(
    value: *const ValueHandle,
    out: *mut SpanView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value owns any returned text; output slots are exclusive. */
    unsafe {
        project(out, error, || {
            Ok(match (*value).answer::<questions::Find>()? {
                None => SpanView::default(),
                Some(found) => SpanView {
                    present: 1,
                    text: views::text(found.text()),
                    start: *found.start() as u64,
                    end: *found.end() as u64,
                },
            })
        })
    }
}
/// Count recognized names from a recognize answer or relate detail.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_name_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe { project(out, error, || Ok(names(&*value)?.len())) }
}
/// Borrow one recognized name and its typed support probability.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_name_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut NameView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: all text views borrow value until cleanup. */
    unsafe {
        project(out, error, || {
            Ok(views::name(
                names(&*value)?.get(index).ok_or_else(native::abi_error)?,
            ))
        })
    }
}
/// Count answered relations.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_relation_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok((*value).answer::<questions::Relate>()?.len())
        })
    }
}
/// Borrow one answered relation and both typed endpoints.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_relation_at(
    value: *const ValueHandle,
    index: usize,
    out: *mut RelationView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: all returned text borrows value until cleanup. */
    unsafe {
        project(out, error, || {
            Ok(views::relation(
                (*value)
                    .answer::<questions::Relate>()?
                    .get(index)
                    .ok_or_else(native::abi_error)?,
            ))
        })
    }
}
/// Count named annotation outcomes, including failed children.
/// # Safety
/// Value is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_annotation_count(
    value: *const ValueHandle,
    out: *mut usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: value and exclusive output slots remain live until return. */
    unsafe {
        project(out, error, || {
            Ok((*value).answer::<questions::Annotate>()?.len())
        })
    }
}
/// Borrow an entry name and copy its typed outcome into an independent owner.
/// # Safety
/// Value is live; name/out/error are distinct empty writable slots.
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
            out.write(std::ptr::null_mut());
            let (key, entry) = (*value)
                .answer::<questions::Annotate>()?
                .iter()
                .nth(index)
                .ok_or_else(native::abi_error)?;
            name.write(views::text(key));
            out.write(Box::into_raw(Box::new(ValueHandle(
                Owned::Entry(entry.clone()),
                None,
            ))));
            Ok(OK)
        })
    }
}
