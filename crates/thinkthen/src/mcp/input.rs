//! Interruptible local stdio; no engine or credential enters this reader.

#[cfg(unix)]
use nix::poll::{PollFd, PollFlags, poll};
#[cfg(unix)]
use std::io::{self, Read};
#[cfg(unix)]
use std::os::fd::AsFd;

#[cfg(unix)]
pub(super) struct PollInput<R> {
    input: R,
    stop: crate::CancelToken,
}

#[cfg(unix)]
impl<R> std::fmt::Debug for PollInput<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PollInput").finish_non_exhaustive()
    }
}

#[cfg(unix)]
impl<R> PollInput<R> {
    pub(super) fn new(input: R, stop: crate::CancelToken) -> Self {
        Self { input, stop }
    }
}

#[cfg(unix)]
impl<R: Read + AsFd> Read for PollInput<R> {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        if bytes.is_empty() {
            return Ok(0);
        }
        while !self.stop.is_cancelled() {
            let mut descriptors = [PollFd::new(self.input.as_fd(), PollFlags::POLLIN)];
            match poll(&mut descriptors, 50u16) {
                Ok(0) => continue,
                Ok(_) => return self.input.read(bytes),
                Err(nix::errno::Errno::EINTR) => continue,
                Err(_) => return Err(io::Error::other("MCP input unavailable")),
            }
        }
        Ok(0)
    }
}

/// Stoppable stdio pipe output. Limit each write to the POSIX minimum atomic
/// pipe unit after POLLOUT; never block on a complete large JSON string.
#[cfg(unix)]
pub(super) struct PollOutput<W> {
    output: W,
    stop: crate::CancelToken,
}
#[cfg(unix)]
impl<W> PollOutput<W> {
    pub(super) fn new(output: W, stop: crate::CancelToken) -> Self {
        Self { output, stop }
    }
}
#[cfg(unix)]
impl<W> std::fmt::Debug for PollOutput<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PollOutput").finish_non_exhaustive()
    }
}
#[cfg(unix)]
impl<W: std::io::Write + AsFd> std::io::Write for PollOutput<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.is_empty() {
            return Ok(0);
        }
        while !self.stop.is_cancelled() {
            let mut descriptors = [PollFd::new(self.output.as_fd(), PollFlags::POLLOUT)];
            match poll(&mut descriptors, 50u16) {
                Ok(0) => continue,
                Ok(_) => {
                    return self.output.write(
                        bytes
                            .get(..bytes.len().min(512))
                            .ok_or_else(|| io::Error::other("invalid output buffer"))?,
                    );
                }
                Err(nix::errno::Errno::EINTR) => continue,
                Err(_) => return Err(io::Error::other("MCP output unavailable")),
            }
        }
        Err(io::ErrorKind::BrokenPipe.into())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

/// Duplicate the process's stdio pipes into unbuffered owned handles. In
/// particular, avoid a global stdout buffer flushing after cancelled shutdown.
#[cfg(unix)]
pub(super) fn pipes() -> io::Result<(std::fs::File, std::fs::File)> {
    let input = io::stdin().as_fd().try_clone_to_owned()?;
    let output = io::stdout().as_fd().try_clone_to_owned()?;
    Ok((std::fs::File::from(input), std::fs::File::from(output)))
}
