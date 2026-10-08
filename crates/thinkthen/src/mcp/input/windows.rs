//! Interrupt only registered MCP pipe I/O, never native execution or other I/O.
use crate::CancelToken;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::windows::io::{AsHandle, OwnedHandle};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

mod ffi;

struct Handles {
    thread: OwnedHandle,
    pipe: Arc<File>,
}
type Registry = Arc<Mutex<Option<Handles>>>;

fn watch_cancellation(registry: Registry, token: CancelToken, completion: mpsc::Receiver<()>) {
    while matches!(
        completion.recv_timeout(Duration::from_millis(20)),
        Err(mpsc::RecvTimeoutError::Timeout)
    ) {
        if token.is_cancelled()
            && let Ok(operation) = registry.lock()
            && let Some(handle) = operation.as_ref()
        {
            // Repeat after ERROR_NOT_FOUND: stop can race entry into
            // the synchronous read/write. The lock also prevents a
            // late cancellation after this operation has returned.
            let _cancel = ffi::cancel(&handle.thread, &handle.pipe);
        }
    }
}

struct Watch {
    active: Registry,
    stop: CancelToken,
    done: Option<mpsc::Sender<()>>,
    worker: Option<JoinHandle<()>>,
}
impl Watch {
    fn new(stop: CancelToken) -> io::Result<Self> {
        let active: Registry = Arc::new(Mutex::new(None));
        let registry = Arc::clone(&active);
        let token = stop.clone();
        let (done, completion) = mpsc::channel();
        let worker = thread::Builder::new()
            .name("thinkthen-mcp-io-stop".into())
            .spawn(move || watch_cancellation(registry, token, completion))?;
        Ok(Self {
            active,
            stop,
            done: Some(done),
            worker: Some(worker),
        })
    }

    fn enter(&self, file: &Arc<File>) -> io::Result<Operation<'_>> {
        let handle = Handles {
            thread: ffi::current_thread()?,
            pipe: Arc::clone(file),
        };
        let mut active = self
            .active
            .lock()
            .map_err(|_| io::Error::other("MCP I/O unavailable"))?;
        if self.stop.is_cancelled() {
            return Err(io::ErrorKind::BrokenPipe.into());
        }
        *active = Some(handle);
        Ok(Operation(&self.active))
    }
}
impl Drop for Watch {
    fn drop(&mut self) {
        self.done.take();
        if let Some(worker) = self.worker.take() {
            let _joined = worker.join();
        }
    }
}
struct Operation<'a>(&'a Registry);
impl Drop for Operation<'_> {
    fn drop(&mut self) {
        if let Ok(mut active) = self.0.lock() {
            *active = None;
        }
    }
}

pub(in crate::mcp) struct PipeInput {
    file: Arc<File>,
    watch: Watch,
}
pub(in crate::mcp) struct PipeOutput {
    file: Arc<File>,
    watch: Watch,
}
impl std::fmt::Debug for PipeInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipeInput").finish_non_exhaustive()
    }
}
impl std::fmt::Debug for PipeOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipeOutput").finish_non_exhaustive()
    }
}
impl PipeInput {
    pub(in crate::mcp) fn new(file: File, stop: CancelToken) -> io::Result<Self> {
        ffi::pipe(&file)?;
        Ok(Self {
            file: Arc::new(file),
            watch: Watch::new(stop)?,
        })
    }
}
impl PipeOutput {
    pub(in crate::mcp) fn new(file: File, stop: CancelToken) -> io::Result<Self> {
        ffi::pipe(&file)?;
        Ok(Self {
            file: Arc::new(file),
            watch: Watch::new(stop)?,
        })
    }
}
impl Read for PipeInput {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        if bytes.is_empty() || self.watch.stop.is_cancelled() {
            return Ok(0);
        }
        let operation = match self.watch.enter(&self.file) {
            Ok(operation) => operation,
            Err(_) if self.watch.stop.is_cancelled() => return Ok(0),
            Err(error) => return Err(error),
        };
        let result = self.file.as_ref().read(bytes);
        drop(operation);
        if self.watch.stop.is_cancelled() {
            Ok(0)
        } else {
            result
        }
    }
}
impl Write for PipeOutput {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.is_empty() {
            return Ok(0);
        }
        let operation = self.watch.enter(&self.file)?;
        let result = self.file.as_ref().write(
            bytes
                .get(..bytes.len().min(65_536))
                .ok_or_else(|| io::Error::other("invalid output buffer"))?,
        );
        drop(operation);
        if self.watch.stop.is_cancelled() {
            Err(io::ErrorKind::BrokenPipe.into())
        } else {
            result
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        // Unbuffered File writes already reached the pipe. FlushFileBuffers
        // would wait for a peer to drain and is deliberately not called.
        Ok(())
    }
}

pub(in crate::mcp) fn pipes() -> io::Result<(File, File)> {
    let input = File::from(io::stdin().as_handle().try_clone_to_owned()?);
    let output = File::from(io::stdout().as_handle().try_clone_to_owned()?);
    ffi::pipe(&input)?;
    ffi::pipe(&output)?;
    Ok((input, output))
}

#[cfg(test)]
mod tests;
