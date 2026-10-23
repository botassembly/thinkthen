//! Process-wide ownership of declared child groups, including forced exit.

use crate::{Error, ErrorKind};
use command_group::{CommandGroup, GroupChild};
use std::io;
use std::process::{ChildStdin, ChildStdout, Command, ExitStatus};
use std::sync::{Arc, Mutex, MutexGuard, Weak};

struct Groups {
    stopped: bool,
    children: Vec<Weak<Mutex<GroupChild>>>,
}
static GROUPS: Mutex<Groups> = Mutex::new(Groups {
    stopped: false,
    children: Vec::new(),
});

/// Kill and reap every declared command group before a forced process exit.
///
/// This is terminal: subsequent command launches fail. Unrelated processes are
/// never selected or signalled.
///
/// # Errors
/// Returns the first wait error after attempting every owned group.
pub fn force_cleanup() -> Result<(), Error> {
    let mut groups = lock(&GROUPS);
    groups.stopped = true;
    let mut failure = None;
    for child in groups.children.iter().filter_map(Weak::upgrade) {
        let mut child = lock(&child);
        let _ = child.kill();
        if let Err(error) = child.wait() {
            failure.get_or_insert(error);
        }
    }
    failure.map_or(Ok(()), |error| {
        Err(Error::new(
            ErrorKind::Io,
            format!("the declared command could not be waited for: {error}"),
        ))
    })
}

pub(crate) struct OwnedChild(Arc<Mutex<GroupChild>>);
impl OwnedChild {
    pub(crate) fn spawn(command: &mut Command) -> io::Result<Self> {
        let mut groups = lock(&GROUPS);
        if groups.stopped {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "run interrupted",
            ));
        }
        let child = Arc::new(Mutex::new(command.group_spawn()?));
        groups.children.retain(|child| child.strong_count() != 0);
        groups.children.push(Arc::downgrade(&child));
        Ok(Self(child))
    }

    pub(crate) fn pipes(&self) -> (Option<ChildStdin>, Option<ChildStdout>) {
        let mut child = lock(&self.0);
        (child.inner().stdin.take(), child.inner().stdout.take())
    }

    pub(crate) fn try_wait(&self) -> io::Result<Option<ExitStatus>> {
        lock(&self.0).try_wait()
    }

    pub(crate) fn kill(&self) -> io::Result<()> {
        lock(&self.0).kill()
    }

    pub(crate) fn wait(&self) -> io::Result<ExitStatus> {
        lock(&self.0).wait()
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
