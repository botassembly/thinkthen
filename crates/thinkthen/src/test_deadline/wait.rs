//! A bounded wait for a spawned test child.
//!
//! The library tests and the backend harness both include this file, so a
//! child that never ends fails its test in one way everywhere.
use std::io::{self, Read};
use std::process::{Child, Output};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// How long a spawned test child may run. An honest run takes a few seconds,
/// and the longest waits out the tool's 30-second request timeout.
pub(crate) const CHILD_DEADLINE: Duration = Duration::from_secs(60);

/// Collect the child's piped output and wait for it to end.
///
/// A child still running at `CHILD_DEADLINE` is killed, and the error names
/// `what`, so a hang fails its test with a message.
pub(crate) fn finish(mut child: Child, what: &str) -> io::Result<Output> {
    let stdout = child.stdout.take().map(drain);
    let stderr = child.stderr.take().map(drain);
    let end = Instant::now() + CHILD_DEADLINE;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if Instant::now() >= end {
            child.kill()?;
            child.wait()?;
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                format!(
                    "{what} ran past {} seconds and was killed",
                    CHILD_DEADLINE.as_secs()
                ),
            ));
        }
        thread::sleep(Duration::from_millis(10));
    };
    Ok(Output {
        status,
        stdout: collected(stdout)?,
        stderr: collected(stderr)?,
    })
}

fn drain(mut pipe: impl Read + Send + 'static) -> JoinHandle<io::Result<Vec<u8>>> {
    thread::spawn(move || {
        let mut bytes = Vec::new();
        pipe.read_to_end(&mut bytes)?;
        Ok(bytes)
    })
}

fn collected(reader: Option<JoinHandle<io::Result<Vec<u8>>>>) -> io::Result<Vec<u8>> {
    reader.map_or_else(
        || Ok(Vec::new()),
        |reader| {
            reader
                .join()
                .map_err(|_| io::Error::other("a pipe reader panicked"))?
        },
    )
}
