//! Bounded native tools. Files drain both streams without pipe capacity or reader joins.
use super::*;
use std::io;

pub(super) fn output(command: &mut Command, tool: &str, timeout: Duration) -> io::Result<Output> {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let folder = scratch(&format!(
        "native-tool-{}",
        RUNS.fetch_add(1, Ordering::Relaxed)
    ));
    let stdout = folder.join("stdout");
    let stderr = folder.join("stderr");
    let mut child = command
        .stdin(Stdio::null())
        .stdout(File::create(&stdout)?)
        .stderr(File::create(&stderr)?)
        .spawn()
        .map_err(|error| io::Error::new(error.kind(), format!("{tool} did not start: {error}")))?;
    let status = wait(&mut child, tool, timeout)?;
    Ok(Output {
        status,
        stdout: std::fs::read(stdout)?,
        stderr: std::fs::read(stderr)?,
    })
}

fn wait(child: &mut Child, tool: &str, timeout: Duration) -> io::Result<std::process::ExitStatus> {
    let started = Instant::now();
    let failure = loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) if started.elapsed() < timeout => {
                std::thread::sleep(Duration::from_millis(20));
            }
            Ok(None) => {
                break io::Error::new(
                    io::ErrorKind::TimedOut,
                    format!("{tool} timed out after {timeout:?}"),
                );
            }
            Err(error) => {
                break io::Error::new(error.kind(), format!("{tool} wait failed: {error}"));
            }
        }
    };
    // Native tools can launch linkers/compiler workers. On Windows stop that
    // owned PID tree before closing its root; never search for processes by name.
    #[cfg(windows)]
    let tree = stop_tree(child);
    // Child::kill targets the process handle/PID we spawned, never a tool name.
    let killed = child.kill();
    let cleanup = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                #[cfg(windows)]
                if let Err(error) = tree {
                    return Err(io::Error::new(
                        failure.kind(),
                        format!("{failure}; {error}"),
                    ));
                }
                return Err(failure);
            }
            Ok(None) if cleanup.elapsed() < Duration::from_secs(5) => {
                std::thread::sleep(Duration::from_millis(20));
            }
            result => {
                return Err(io::Error::new(
                    failure.kind(),
                    format!(
                        "{failure}; owned child {} cleanup failed: kill={killed:?}, status={result:?}",
                        child.id()
                    ),
                ));
            }
        }
    }
}

#[cfg(windows)]
fn stop_tree(owned: &mut Child) -> io::Result<()> {
    if owned.try_wait()?.is_some() {
        return Ok(());
    }
    let mut cleanup = child::command("taskkill.exe", &[])
        .args(["/PID", &owned.id().to_string(), "/T", "/F"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(5) {
        match cleanup.try_wait() {
            Ok(Some(status)) if status.success() => return Ok(()),
            Ok(Some(status)) => {
                return Err(io::Error::other(format!(
                    "owned PID tree cleanup failed: {status}"
                )));
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(_) => break,
        }
    }
    let _ = cleanup.kill();
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(5) {
        if cleanup.try_wait()?.is_some() {
            return Err(io::Error::other("owned PID tree cleanup timed out"));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    Err(io::Error::other(
        "owned PID tree cleanup timed out; cleanup child did not stop",
    ))
}

#[cfg(test)]
mod tests;
