//! One JSON-only output door; native serializers retain their authored order.

use super::protocol::{Fault, Id};
use serde::Serialize;
use serde_json::{json, value::RawValue};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

pub(super) struct Output<W>(Arc<Mutex<W>>);
impl<W> Clone for Output<W> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
impl<W> std::fmt::Debug for Output<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Output(<withheld>)")
    }
}
impl<W: Write> Output<W> {
    pub(super) fn new(writer: W) -> Self {
        Self(Arc::new(Mutex::new(writer)))
    }

    pub(super) fn result(&self, id: &Id, value: impl Serialize) -> io::Result<()> {
        #[derive(Serialize)]
        struct Response<'a, T> {
            jsonrpc: &'static str,
            id: &'a Id,
            result: T,
        }
        self.write(&Response {
            jsonrpc: "2.0",
            id,
            result: value,
        })
    }
    pub(super) fn fault(&self, id: Option<&Id>, fault: Fault) -> io::Result<()> {
        self.write(&json!({"jsonrpc":"2.0", "id":id,
            "error":{"code":fault.code,"message":fault.message}}))
    }
    fn write(&self, value: &impl Serialize) -> io::Result<()> {
        serde_json::to_writer(Counter(1), value)
            .map_err(|_| io::Error::other("MCP output exceeds 192 MiB"))?;
        let mut writer = self
            .0
            .lock()
            .map_err(|_| io::Error::other("MCP output unavailable"))?;
        let mut packet = io::BufWriter::with_capacity(8 * 1024, &mut *writer);
        let result = (|| {
            serde_json::to_writer(&mut packet, value).map_err(|error| {
                io::Error::new(
                    error.io_error_kind().unwrap_or(io::ErrorKind::Other),
                    "MCP output closed",
                )
            })?;
            packet.write_all(b"\n")?;
            packet.flush()
        })();
        // Discard a failed packet tail without retrying output during drop.
        let _parts = packet.into_parts();
        result
    }
}

/// An object produced by an existing native serializer, never reparsed into
/// a sorted JSON map. Construction does not manufacture any result field.
#[derive(Serialize)]
#[serde(transparent)]
pub(super) struct NativeObject(Box<RawValue>);
impl std::fmt::Debug for NativeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NativeObject(<withheld>)")
    }
}
impl NativeObject {
    pub(super) fn new(value: &impl Serialize) -> Result<Self, serde_json::Error> {
        let text = serde_json::to_string(value)?;
        if !text.starts_with('{') {
            return Err(serde::ser::Error::custom(
                "native MCP result must be an object",
            ));
        }
        RawValue::from_string(text).map(Self)
    }
}

#[derive(Serialize)]
pub(super) struct ToolResult {
    content: [TextContent; 1],
    #[serde(rename = "structuredContent")]
    structured: NativeObject,
    #[serde(rename = "isError")]
    failed: bool,
}
#[derive(Serialize)]
struct TextContent {
    #[serde(rename = "type")]
    kind: &'static str,
    text: String,
}

/// False/null and partial failures remain inside the unchanged native object.
pub(super) fn tool_result(value: NativeObject, failed: bool) -> ToolResult {
    ToolResult {
        content: [TextContent {
            kind: "text",
            text: value.0.get().to_owned(),
        }],
        structured: value,
        failed,
    }
}

/// Count before writing, so an excess never emits a truncated protocol line.
struct Counter(usize);
impl Write for Counter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0 = self.0.saturating_add(bytes.len());
        if self.0 > 192 * 1024 * 1024 {
            return Err(io::Error::other("MCP output exceeds 192 MiB"));
        }
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Existing safe errors and observed facts. Admission invents no started facts.
pub(super) fn native_error(error: &crate::Error) -> crate::CompleteError<'_> {
    error.complete()
}

#[cfg(test)]
mod tests {
    use super::Output;
    use std::io::{self, Write};
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    struct Count(Arc<AtomicUsize>);
    impl Write for Count {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.fetch_add(bytes.len(), Ordering::Relaxed);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn outgoing_frame_counts_its_newline_and_refuses_excess_without_partial_output() {
        let limit = 192 * 1024 * 1024;
        let count = Arc::new(AtomicUsize::new(0));
        let output = Output::new(Count(Arc::clone(&count)));
        let mut text = String::with_capacity(limit);
        text.extend(std::iter::repeat_n('x', limit - 3));
        output.write(&text).unwrap();
        assert_eq!(count.load(Ordering::Relaxed), limit);
        text.push('x');
        assert!(output.write(&text).is_err());
        assert_eq!(count.load(Ordering::Relaxed), limit);
    }
    #[derive(Default)]
    struct Sink {
        bytes: Vec<u8>,
        flushes: usize,
        interrupted: bool,
        terminal: Option<io::ErrorKind>,
        failed: bool,
        after_failure: usize,
        flush_failure: bool,
    }
    impl Write for Sink {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.failed {
                self.after_failure += 1;
            } else if !self.interrupted {
                self.interrupted = true;
                return Err(io::ErrorKind::Interrupted.into());
            } else if self.bytes.len() == 14
                && let Some(kind) = self.terminal
            {
                self.failed = true;
                let written = (kind == io::ErrorKind::WriteZero).then_some(0);
                return written.ok_or_else(|| kind.into());
            }
            let count = bytes.len().min(7).min(if self.terminal.is_some() {
                14usize.saturating_sub(self.bytes.len())
            } else {
                usize::MAX
            });
            self.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }
        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            if self.flush_failure {
                self.failed = true;
                Err(io::ErrorKind::PermissionDenied.into())
            } else {
                Ok(())
            }
        }
    }
    fn packet_case(
        repeats: usize,
        terminal: Option<io::ErrorKind>,
        flush_failure: bool,
        error: Option<io::ErrorKind>,
    ) {
        let text = "\"\\\n\t".repeat(repeats);
        let mut expected = serde_json::to_vec(&text).unwrap();
        expected.push(b'\n');
        let output = Output::new(Sink {
            terminal,
            flush_failure,
            ..Sink::default()
        });
        assert_eq!(output.write(&text).err().map(|e| e.kind()), error);
        let sink = output.0.lock().unwrap();
        assert!(sink.interrupted);
        assert_eq!(sink.after_failure, 0, "packet tail retried after {error:?}");
        if terminal.is_some() {
            assert_eq!(sink.flushes, 0);
            assert_eq!(sink.bytes, expected[..14]);
        } else {
            assert_eq!(sink.bytes, expected);
            assert_eq!(sink.bytes.iter().filter(|&&b| b == b'\n').count(), 1);
            assert_eq!(sink.flushes, 1);
        }
    }
    #[test]
    fn packets_handle_short_writes_and_interrupts_without_retrying_terminal_failures() {
        for repeats in [4, 4096] {
            for (terminal, flush_failure, error) in [
                (None, false, None),
                (
                    Some(io::ErrorKind::BrokenPipe),
                    false,
                    Some(io::ErrorKind::BrokenPipe),
                ),
                (
                    Some(io::ErrorKind::WriteZero),
                    false,
                    Some(io::ErrorKind::WriteZero),
                ),
                (None, true, Some(io::ErrorKind::PermissionDenied)),
            ] {
                packet_case(repeats, terminal, flush_failure, error);
            }
        }
    }
}
