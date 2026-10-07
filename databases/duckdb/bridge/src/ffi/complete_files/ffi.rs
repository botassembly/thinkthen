//! Complete file descriptors are read only through DuckDB-authorized handles.
use super::{BridgeText, Reply, reply_boundary, text};
use std::{
    ffi::c_void,
    io::{self, BufReader, Read},
};
use thinkthen::{InputFileReader, InputReaderOptions, SourceItem};
#[derive(Debug)]
struct Host {
    context: *mut c_void,
    read: extern "C" fn(*mut c_void, *mut u8, usize) -> i64,
}
impl Read for Host {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        usize::try_from((self.read)(self.context, bytes.as_mut_ptr(), bytes.len()))
            .ok()
            .filter(|n| *n <= bytes.len())
            .ok_or_else(|| io::Error::other("authorized source read failed"))
    }
}
pub(crate) struct Reader {
    items: InputFileReader<BufReader<Host>>,
    jsonl: bool,
}
impl std::fmt::Debug for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reader").finish_non_exhaustive()
    }
}
/// Prepare explicit native reader controls without opening any content.
/// # Safety
/// The counted range is readable through return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_file_plan(input: BridgeText) -> Reply {
    reply_boundary(|| {
        let source = text(input.bytes, input.len)?;
        crate::complete_native::Inputs::parse(source, true).map_err(|e| e.to_string())?;
        let value: serde_json::Value = serde_json::from_str(source)
            .map_err(|_| "thinkthen usage: invalid inputs".to_owned())?;
        let Some(files) = value.get("files") else {
            return Ok(Vec::new());
        };
        let options: InputReaderOptions = serde_json::from_value(
            files
                .get("options")
                .cloned()
                .unwrap_or_else(|| serde_json::json!({})),
        )
        .map_err(|_| "thinkthen usage: invalid native reader options".to_owned())?;
        options.validate().map_err(|e| e.to_string())?;
        serde_json::to_vec(&serde_json::json!({"paths":files.get("paths"),"options":options,"jsonl":files.get("format").and_then(serde_json::Value::as_str)==Some("jsonl")})).map_err(|_|"thinkthen defect: source plan did not encode".to_owned())
    })
}
/// Wrap one authorized synchronous handle with the native input reader.
/// # Safety
/// Host retains the callback and writable out range until reader_free.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_reader_new(
    file: BridgeText,
    options: BridgeText,
    jsonl: i32,
    context: *mut c_void,
    read: Option<extern "C" fn(*mut c_void, *mut u8, usize) -> i64>,
    out: *mut *mut Reader,
) -> Reply {
    reply_boundary(|| {
        if context.is_null() || out.is_null() {
            return Err("thinkthen defect: missing authorized source handle".to_owned());
        }
        let read =
            read.ok_or_else(|| "thinkthen defect: missing authorized source callback".to_owned())?;
        let options = serde_json::from_str(text(options.bytes, options.len)?)
            .map_err(|_| "thinkthen usage: invalid native reader options".to_owned())?;
        let items = InputFileReader::new(
            text(file.bytes, file.len)?,
            BufReader::new(Host { context, read }),
            options,
        )
        .map_err(|e| e.to_string())?;
        // SAFETY: host lends the out range and takes exclusive reader ownership.
        unsafe {
            out.write(Box::into_raw(Box::new(Reader {
                items,
                jsonl: jsonl != 0,
            })));
        }
        Ok(Vec::new())
    })
}
/// Read one explicit located descriptor, or empty bytes at EOF.
/// # Safety
/// The live reader is uniquely borrowed until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_reader_next(reader: *mut Reader) -> Reply {
    reply_boundary(|| {
        // SAFETY: host retains exclusive live reader ownership.
        let reader = unsafe { reader.as_mut() }
            .ok_or_else(|| "thinkthen defect: missing source reader".to_owned())?;
        let Some(item) = reader.items.next() else {
            return Ok(Vec::new());
        };
        let descriptor = match item.map_err(|e| e.to_string())? {
            SourceItem::Text(s) => {
                serde_json::json!({if reader.jsonl {"json_text"}else{"text"}:s.record,"source":{"file":s.file,"first_line":s.first_line,"last_line":s.last_line}})
            }
            SourceItem::Image(s) => {
                serde_json::json!({"images":[{"media":match s.record.media(){thinkthen::ImageMedia::Png=>"image/png",thinkthen::ImageMedia::Jpeg=>"image/jpeg"},"bytes":s.record.bytes()}],"source":{"file":s.file}})
            }
        };
        serde_json::to_vec(&descriptor)
            .map_err(|_| "thinkthen defect: source descriptor did not encode".to_owned())
    })
}
/// Relinquish the reader before closing its host handle.
/// # Safety
/// The live reader allocation is relinquished once.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_reader_free(reader: *mut Reader) {
    let _ = super::panic::caught(|| {
        if !reader.is_null() {
            // SAFETY: unique ownership transfers from the host once.
            drop(unsafe { Box::from_raw(reader) });
        }
    });
}
/// Replace explicit files by already authorized descriptors; leave other bytes intact.
/// # Safety
/// Both counted JSON ranges remain readable through return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_file_records(
    input: BridgeText,
    records: BridgeText,
) -> Reply {
    reply_boundary(|| {
        let mut value: serde_json::Value = serde_json::from_str(text(input.bytes, input.len)?)
            .map_err(|_| "thinkthen usage: invalid inputs".to_owned())?;
        value
            .as_object_mut()
            .ok_or_else(|| "thinkthen usage: inputs is one object".to_owned())?
            .remove("files");
        let records = serde_json::from_str(text(records.bytes, records.len)?)
            .map_err(|_| "thinkthen defect: invalid authorized descriptors".to_owned())?;
        value
            .as_object_mut()
            .ok_or_else(|| "thinkthen usage: inputs is one object".to_owned())?
            .insert("records".to_owned(), records);
        serde_json::to_vec(&value)
            .map_err(|_| "thinkthen defect: source descriptors did not encode".to_owned())
    })
}
