//! Complete file descriptors are read only through DuckDB-authorized handles.
use super::{BridgeText, Reply, reply_boundary, text};
mod render;
use std::{
    ffi::c_void,
    io::{self, BufReader, Read},
};
use thinkthen::{
    InputFileReader, InputReaderOptions, RequestFraming, RequestSourceReader, SourceItem,
};
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
    items: Box<dyn Iterator<Item = Result<serde_json::Value, thinkthen::Error>>>,
    error: Option<thinkthen::Error>,
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
        let inputs = crate::complete_native::Inputs::parse(source, true)
            .map_err(|e| crate::complete_native::failure(&e).to_string())?;
        let value: serde_json::Value = serde_json::from_str(source)
            .map_err(|_| reject(thinkthen::ErrorKind::Usage, "invalid inputs"))?;
        let Some(files) = value.get("files") else {
            return Ok(Vec::new());
        };
        let paths = inputs
            .file_paths()
            .map_err(|e| crate::complete_native::failure(&e).to_string())?;
        let options: InputReaderOptions = serde_json::from_value(
            files
                .get("options")
                .cloned()
                .unwrap_or_else(|| serde_json::json!({})),
        )
        .map_err(|_| reject(thinkthen::ErrorKind::Usage, "invalid native reader options"))?;
        options
            .validate()
            .map_err(|e| crate::complete_native::failure(&e).to_string())?;
        thinkthen::RequestSource {
            paths: Vec::new(),
            reading: options.reading,
            media: options.media,
            framing: inputs.framing,
        }
        .validate_reading()
        .map_err(|e| crate::complete_native::failure(&e).to_string())?;
        serde_json::to_vec(
            &serde_json::json!({"paths":paths,"options":options,"framing":match inputs.framing {None=>0,Some(RequestFraming::Jsonl)=>1,Some(RequestFraming::Csv)=>2,Some(RequestFraming::Tsv)=>3,_=>return Err(reject(thinkthen::ErrorKind::Defect,"invalid SQL framing"))}}),
        )
        .map_err(|_| "thinkthen defect: source plan did not encode".to_owned())
    })
}
/// Wrap one authorized synchronous handle with the native input reader.
/// # Safety
/// Host retains the callback and writable out range until reader_free.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_reader_new(
    file: BridgeText,
    options: BridgeText,
    framing: i32,
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
        let options: InputReaderOptions = serde_json::from_str(text(options.bytes, options.len)?)
            .map_err(|_| {
            reject(thinkthen::ErrorKind::Usage, "invalid native reader options")
        })?;
        let name = text(file.bytes, file.len)?;
        let host = BufReader::new(Host { context, read });
        let items: Box<dyn Iterator<Item = Result<serde_json::Value, thinkthen::Error>>> =
            if framing == 0 {
                Box::new(match InputFileReader::new(name, host, options) {
                    Ok(reader) => Box::new(reader.map(|item| item.map(physical_descriptor)))
                        as Box<dyn Iterator<Item = Result<serde_json::Value, thinkthen::Error>>>,
                    Err(error) => Box::new(std::iter::once(Err(error))),
                })
            } else {
                let framing = match framing {
                    1 => RequestFraming::Jsonl,
                    2 => RequestFraming::Csv,
                    3 => RequestFraming::Tsv,
                    _ => return Err(reject(thinkthen::ErrorKind::Defect, "invalid SQL framing")),
                };
                thinkthen::RequestSource {
                    paths: Vec::new(),
                    reading: options.reading,
                    media: options.media,
                    framing: Some(framing),
                }
                .validate_reading()
                .map_err(|e| crate::complete_native::failure(&e).to_string())?;
                Box::new(
                    match RequestSourceReader::new(name, host, framing, options.reading) {
                        Ok(reader) => Box::new(reader.map(|item| {
                            item.and_then(crate::complete_native::file_format::descriptor)
                        }))
                            as Box<
                                dyn Iterator<Item = Result<serde_json::Value, thinkthen::Error>>,
                            >,
                        Err(error) => Box::new(std::iter::once(Err(error))),
                    },
                )
            };
        // SAFETY: host lends the out range and takes exclusive reader ownership.
        unsafe {
            out.write(Box::into_raw(Box::new(Reader { items, error: None })));
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
        let descriptor = match item {
            Ok(v) => v,
            Err(e) => {
                let message = crate::complete_native::failure(&e).to_string();
                reader.error = Some(e);
                return Err(message);
            }
        };
        let descriptor = match descriptor_item(descriptor) {
            Ok(v) => v,
            Err(e) => {
                let message = crate::complete_native::failure(&e).to_string();
                reader.error = Some(e);
                return Err(message);
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
fn physical_descriptor(item: SourceItem) -> serde_json::Value {
    match item {
        SourceItem::Text(s) => {
            serde_json::json!({"text":s.record,"source":{"file":s.file,"first_line":s.first_line,"last_line":s.last_line}})
        }
        SourceItem::Image(s) => {
            serde_json::json!({"images":[{"media":match s.record.media(){thinkthen::ImageMedia::Png=>"image/png",thinkthen::ImageMedia::Jpeg=>"image/jpeg"},"bytes":s.record.bytes()}],"source":{"file":s.file}})
        }
    }
}

fn reject(kind: thinkthen::ErrorKind, message: &str) -> String {
    crate::complete_native::failure(&thinkthen::Error::new(kind, message)).to_string()
}
/// Serialize a host admission failure without any invocation facts.
/// # Safety
/// The counted diagnostic range is readable until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_admission_error(
    message: BridgeText,
    kind: BridgeText,
) -> Reply {
    reply_boundary(|| {
        let kind = match text(kind.bytes, kind.len)? {
            "usage" => thinkthen::ErrorKind::Usage,
            "local" => thinkthen::ErrorKind::Local,
            "backend" => thinkthen::ErrorKind::Backend,
            "deadline" => thinkthen::ErrorKind::Deadline,
            "cancelled" => thinkthen::ErrorKind::Cancelled,
            _ => thinkthen::ErrorKind::Defect,
        };
        Ok(reject(kind, text(message.bytes, message.len)?).into_bytes())
    })
}

fn descriptor_item(
    mut descriptor: serde_json::Value,
) -> Result<serde_json::Value, thinkthen::Error> {
    let source = descriptor
        .as_object_mut()
        .ok_or_else(crate::complete_native::defect)?
        .remove("source");
    let item = thinkthen::RequestItem::from_record_descriptor(&descriptor.to_string())?;
    let mut value = serde_json::json!({"item":item});
    if let Some(source) = source {
        crate::complete_native::put(&mut value, "location", source)?;
    }
    Ok(value)
}
/// Transfer the same counted descriptor; Full retains no caller range.
/// # Safety
/// Session is live and unique; counted bytes are readable through return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_feed_push(
    session: *mut thinkthen::RequestSession,
    descriptor: BridgeText,
) -> Reply {
    reply_boundary(|| {
        // SAFETY: the host owns this existing session allocation.
        let session = unsafe { session.as_ref() }
            .ok_or_else(|| "thinkthen defect: missing session".to_owned())?;
        let status = session
            .try_push_json(text(descriptor.bytes, descriptor.len)?)
            .map_err(|e| crate::complete_native::failure(&e).to_string())?;
        Ok(vec![match status {
            thinkthen::RequestSessionPushStatus::Accepted => b'A',
            thinkthen::RequestSessionPushStatus::Full => b'F',
            thinkthen::RequestSessionPushStatus::Closed => b'C',
        }])
    })
}
/// Read one SQL-rendered actual native packet, Pending or End.
/// # Safety
/// Session remains live through return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_feed_read(
    session: *mut thinkthen::RequestSession,
) -> Reply {
    reply_boundary(|| {
        // SAFETY: caller owns the existing allocation throughout this call.
        let session = unsafe { session.as_ref() }
            .ok_or_else(|| "thinkthen defect: missing session".to_owned())?;
        match session.try_read() {
            thinkthen::RequestSessionRead::Pending => Ok(Vec::new()),
            thinkthen::RequestSessionRead::End => Ok(b"E".to_vec()),
            thinkthen::RequestSessionRead::Result(packet) => render::packet(packet)
                .map(|v| v.to_string().into_bytes())
                .map_err(|e| crate::complete_native::failure(&e).to_string()),
        }
    })
}
/// Finish input without waiting, including an exact native reader failure.
/// # Safety
/// Caller uniquely owns both allocations, with reader optional at EOF/host failure.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_feed_finish(
    session: *mut thinkthen::RequestSession,
    reader: *mut Reader,
    host_failure: i32,
) -> Reply {
    reply_boundary(|| {
        // SAFETY: neither allocation is freed or advanced during this operation.
        let session = unsafe { session.as_ref() }
            .ok_or_else(|| "thinkthen defect: missing session".to_owned())?;
        let error = unsafe { reader.as_mut() }.and_then(|r| r.error.take());
        let finished = if let Some(error) = error {
            session.finish_native_reader_error(error)
        } else {
            session.finish(if host_failure == 0 {
                None
            } else {
                Some(thinkthen::RequestReaderFailure::Io { location: None })
            })
        };
        finished
            .map(|()| Vec::new())
            .map_err(|e| crate::complete_native::failure(&e).to_string())
    })
}
/// Cancel and drop the existing endpoint without joining its native worker.
/// # Safety
/// The caller relinquishes this allocation once, after freeing its reader.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_feed_free(
    session: *mut thinkthen::RequestSession,
) {
    let _ = super::panic::caught(|| {
        if !session.is_null() {
            // SAFETY: this is the sole existing allocation returned to the caller.
            drop(unsafe { Box::from_raw(session) });
        }
    });
}
/// Render retained output after native End; no input is passed here.
/// # Safety
/// Counted output packet bytes remain readable through return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_complete_feed_render(
    packets: BridgeText,
    verb: BridgeText,
) -> Reply {
    reply_boundary(|| {
        render::finish(
            text(packets.bytes, packets.len)?,
            text(verb.bytes, verb.len)?,
        )
        .map(|v| v.to_string().into_bytes())
        .map_err(|e| crate::complete_native::failure(&e).to_string())
    })
}
