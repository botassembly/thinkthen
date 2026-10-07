//! Bounded shared framing over one DuckDB-authorized synchronous handle.

use std::ffi::c_void;
use std::io::{self, BufReader, Read};

use thinkthen::{FileReader, ReaderOptions, SourceRecord};

use super::{Reply, panic, reply_boundary, text};

#[derive(Debug)]
struct HostRead {
    context: *mut c_void,
    read: extern "C" fn(*mut c_void, *mut u8, usize) -> i64,
}

impl Read for HostRead {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let count = (self.read)(self.context, buffer.as_mut_ptr(), buffer.len());
        let count = usize::try_from(count)
            .ok()
            .filter(|&n| n <= buffer.len())
            .ok_or_else(|| io::Error::other("authorized host read failed"))?;
        Ok(count)
    }
}

#[derive(Debug)]
pub(crate) struct Reader {
    records: FileReader<BufReader<HostRead>>,
}

/// Validate reader JSON before the host inspects any operands.
///
/// # Safety
/// The byte range is readable for this synchronous call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_reader_options(
    bytes: *const u8,
    len: usize,
) -> Reply {
    reply_boundary(|| {
        let options = options(text(bytes, len)?)?;
        serde_json::to_vec(&options)
            .map_err(|_| "thinkthen defect: reader options did not encode".to_owned())
    })
}

fn options(json: &str) -> Result<ReaderOptions, String> {
    serde_json::from_str::<ReaderOptions>(json)
        .map_err(|_| {
            "thinkthen usage: reader options must be a unit/window JSON object".to_owned()
        })?
        .validate()
        .map_err(|error| error.to_string())
}

/// Wrap a handle owned and authorized by DuckDB. This opens no path.
///
/// # Safety
/// The host retains context and no-throw callback until reader_free; all byte
/// ranges and the writable out pointer stay live through this call. Calls are
/// sequential on the host's scan thread. The callback never retains the buffer.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_reader_new(
    file: *const u8,
    file_len: usize,
    json: *const u8,
    json_len: usize,
    context: *mut c_void,
    read: Option<extern "C" fn(*mut c_void, *mut u8, usize) -> i64>,
    out: *mut *mut Reader,
) -> Reply {
    reply_boundary(|| {
        if context.is_null() || out.is_null() {
            return Err("thinkthen defect: the reader got a null host handle".to_owned());
        }
        let read =
            read.ok_or_else(|| "thinkthen defect: the reader got no host callback".to_owned())?;
        let records = FileReader::new(
            text(file, file_len)?,
            BufReader::new(HostRead { context, read }),
            options(text(json, json_len)?)?,
        )
        .map_err(|error| error.to_string())?;
        // SAFETY: the C++ owner provided a writable out pointer; ownership
        // transfers only after successful construction, and free occurs once.
        unsafe { out.write(Box::into_raw(Box::new(Reader { records }))) };
        Ok(Vec::new())
    })
}

/// Return one located record as JSON, or empty bytes at EOF.
///
/// # Safety
/// The pointer is live from reader_new and exclusively borrowed for this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_reader_next(reader: *mut Reader) -> Reply {
    reply_boundary(|| {
        // SAFETY: the C++ scan retains and exclusively uses this owned reader.
        let reader = unsafe { reader.as_mut() }
            .ok_or_else(|| "thinkthen defect: the reader pointer is null".to_owned())?;
        reader
            .records
            .next()
            .transpose()
            .map_err(|error| error.to_string())?
            .map_or(Ok(Vec::new()), |record| {
                serde_json::to_vec(&record)
                    .map_err(|_| "thinkthen defect: a source record did not encode".to_owned())
            })
    })
}

/// Free one reader, before the host releases its callback context and handle.
///
/// # Safety
/// This is the unique live reader returned by reader_new, freed exactly once.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_reader_free(reader: *mut Reader) {
    let _ = panic::caught(|| {
        if !reader.is_null() {
            // SAFETY: the C++ owner relinquishes the allocation exactly once.
            drop(unsafe { Box::from_raw(reader) });
        }
    });
}

/// Map recognition's Unicode scalar offsets through the shared source mapper.
///
/// # Safety
/// The record byte range remains readable for this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_span_lines(
    bytes: *const u8,
    len: usize,
    first_line: usize,
    start: usize,
    end: usize,
) -> Reply {
    reply_boundary(|| {
        if first_line == 0 {
            return Err("thinkthen usage: source first_line must be positive".to_owned());
        }
        let content = text(bytes, len)?;
        let last_line = first_line
            .checked_add(content.split_inclusive('\n').count().saturating_sub(1))
            .ok_or_else(|| "thinkthen usage: source line exceeds SQL BIGINT".to_owned())?;
        let record = SourceRecord {
            record: content,
            file: String::new(),
            first_line,
            last_line,
        };
        let (first, last) = record
            .span_lines(start, end)
            .map_err(|error| error.to_string())?;
        serde_json::to_vec(&serde_json::json!({"first_line": first, "last_line": last}))
            .map_err(|_| "thinkthen defect: source lines did not encode".to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::{BufReader, FileReader, HostRead, Read, ReaderOptions, c_void, io};

    extern "C" fn too_many(_: *mut c_void, _: *mut u8, size: usize) -> i64 {
        i64::try_from(size + 1).unwrap()
    }

    extern "C" fn failed(_: *mut c_void, _: *mut u8, _: usize) -> i64 {
        -1
    }

    #[test]
    fn invalid_host_read_counts_fail_without_exposing_a_buffer_range() {
        for read in [too_many, failed] {
            let mut reader = HostRead {
                context: std::ptr::null_mut(),
                read,
            };
            assert!(reader.read(&mut [0; 4]).is_err());
        }
    }

    extern "C" fn short_read(context: *mut c_void, bytes: *mut u8, size: usize) -> i64 {
        // SAFETY: this test lends one exclusive live Cursor and the Read
        // implementation lends a writable size-byte range for this call only.
        let (cursor, buffer) = unsafe {
            (
                context.cast::<io::Cursor<&[u8]>>().as_mut().unwrap(),
                std::slice::from_raw_parts_mut(bytes, size.min(2)),
            )
        };
        i64::try_from(cursor.read(buffer).unwrap()).unwrap()
    }

    #[test]
    fn short_host_reads_preserve_unicode_crlf_and_physical_positions() {
        let mut cursor = io::Cursor::new("café 😀\r\n\r\nAda\n".as_bytes());
        let host = HostRead {
            context: (&raw mut cursor).cast(),
            read: short_read,
        };
        let records = FileReader::new("owned", BufReader::new(host), ReaderOptions::default())
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            records
                .iter()
                .map(|r| (r.record.as_str(), r.first_line, r.last_line))
                .collect::<Vec<_>>(),
            vec![("café 😀", 1, 1), ("Ada", 3, 3)]
        );
        assert_eq!(records[0].span_lines(5, 6).unwrap(), (1, 1));
    }
}
