//! Counted constructors and named helpers; all foreign pointer reads stay here.
#![allow(
    unsafe_code,
    reason = "the reviewed C lifetime contract permits foreign pointer access"
)]
#![deny(unsafe_op_in_unsafe_fn)]
/// Canonical reviewed C input descriptors and target borrowed views.
pub use crate::ffi::carriers;
#[path = "author/ffi.rs"]
mod author;
#[path = "images/ffi.rs"]
pub mod images;
#[path = "question/ffi.rs"]
mod question;
#[path = "read/ffi.rs"]
pub(crate) mod read;
use crate::Door;
use crate::current::{self, QuestionHandle, Source, SourceHandle};
use crate::failures::{Failure, OK, guard};
use carriers::{QuestionSpecV1, RecordV1, SourceSpecV1, StringV1};

/// Clone an admitted native question from counted descriptors.
/// # Safety
/// All pointers obey the installed header's storage and lifetime contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_new(
    engine: *const Door,
    spec: *const QuestionSpecV1,
    out: *mut *mut QuestionHandle,
) -> i32 {
    // SAFETY: unchanged descriptors pass through the same guarded constructor.
    unsafe { author::new(engine, spec, std::ptr::null(), out) }
}
/// Load a bounded native question or question-set file.
/// # Safety
/// All pointers obey the installed header's storage and lifetime contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_load(
    engine: *const Door,
    path: StringV1,
    out: *mut *mut QuestionHandle,
) -> i32 {
    // SAFETY: counted path and writable output follow the header.
    unsafe {
        super::typed(
            engine,
            |_| {
                read::required(out)?;
                current::load(read::string(path)?)
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
/// Clone every counted original record before returning.
/// # Safety
/// All pointers obey the installed header's storage and lifetime contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_source_records(
    engine: *const Door,
    records: *const RecordV1,
    count: usize,
    out: *mut *mut SourceHandle,
) -> i32 {
    // SAFETY: readable arrays and output have their documented extents.
    unsafe {
        super::typed(
            engine,
            |_| {
                read::required(out)?;
                let records = read::slice(records, count)?
                    .iter()
                    .enumerate()
                    .map(|(index, record)| {
                        if record.images.len > thinkthen::MAX_IMAGES {
                            return Err(Failure::usage("image evidence requires 1 to 8 images"));
                        }
                        let input = current::Input {
                            original: read::optional_content(record.original)?,
                            context: read::optional_content(record.context)?,
                            options: read::choices(record.options)?,
                            images: read::slice(record.images.data, record.images.len)?
                                .iter()
                                .map(|image| read::reference(*image).cloned())
                                .collect::<Result<Vec<_>, Failure>>()?,
                            position: None,
                            index,
                        };
                        input.question_input()?;
                        Ok(input)
                    })
                    .collect::<Result<Vec<_>, Failure>>()?;
                Ok(SourceHandle(Source::Records(records)))
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
/// Clone an explicit native text/image reader selection.
/// # Safety
/// All pointers obey the installed header's storage and lifetime contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_source_files(
    engine: *const Door,
    spec: *const SourceSpecV1,
    out: *mut *mut SourceHandle,
) -> i32 {
    // SAFETY: unchanged descriptors go through the same guarded reader edge.
    unsafe { source_files(engine, spec, out, false) }
}
/// Clone an explicitly image-only native reader, validating physical units before opening paths.
/// # Safety
/// All pointers obey the installed header's storage and lifetime contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_source_image_files(
    engine: *const Door,
    spec: *const SourceSpecV1,
    out: *mut *mut SourceHandle,
) -> i32 {
    // SAFETY: unchanged descriptors go through the same guarded reader edge.
    unsafe { source_files(engine, spec, out, true) }
}
unsafe fn source_files(
    engine: *const Door,
    spec: *const SourceSpecV1,
    out: *mut *mut SourceHandle,
    images: bool,
) -> i32 {
    // SAFETY: counted paths and descriptors are readable through this call.
    unsafe {
        super::typed(
            engine,
            |_| {
                read::required(out)?;
                let spec = read::reference(spec)?;
                if spec.unit == 5 && !images {
                    if spec.window != 0 {
                        return Err(Failure::usage("JSONL sources do not accept windows"));
                    }
                    return Ok(SourceHandle(Source::JsonLines(read::strings(spec.paths)?)));
                }
                let unit = match spec.unit {
                    1 => thinkthen::SourceUnit::Line,
                    2 => thinkthen::SourceUnit::Window,
                    3 | 4 => thinkthen::SourceUnit::File,
                    _ => {
                        return Err(Failure::usage("sources require an explicit reader unit"));
                    }
                };
                let options = thinkthen::InputReaderOptions {
                    reading: thinkthen::ReaderOptions {
                        unit,
                        window: (spec.window != 0).then_some(spec.window),
                    },
                    media: if images || spec.unit == 4 {
                        thinkthen::ReaderMedia::Image
                    } else {
                        thinkthen::ReaderMedia::Text
                    },
                }
                .validate()?;
                let paths = read::strings(spec.paths)?;
                Ok(SourceHandle(Source::Files(paths, options)))
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
macro_rules! free {
    ($name:ident, $ty:ty) => {
        /// Free the owned handle; NULL is harmless.
        /// # Safety
        /// A nonnull handle is live, freed once, with no concurrent borrowers.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(owner: *mut $ty) {
            guard(None, (), || {
                if !owner.is_null() {
                    // SAFETY: only the allocating library frees this live owner, once.
                    drop(unsafe { Box::from_raw(owner) });
                }
            });
        }
    };
}
free!(thinkthen_question_free, QuestionHandle);
free!(thinkthen_source_free, SourceHandle);

#[cfg(test)]
#[path = "tests/ffi.rs"]
mod tests;
