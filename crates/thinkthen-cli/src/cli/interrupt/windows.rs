//! Debug-only acknowledgment polling on an ordinary thread.

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};

use super::{Acknowledgment, StartError};
use crate::engine::Cancel;

pub(super) struct Carrier {
    stop: Sender<()>,
    thread: JoinHandle<Result<(), ()>>,
}

impl Carrier {
    pub(super) fn start(cancel: Cancel<'static>, path: PathBuf) -> Result<Self, StartError> {
        let (stop, stopped) = mpsc::channel();
        let acknowledgment = Acknowledgment::new(Some(path));
        let thread = thread::Builder::new()
            .name("thinkthen-sigint-ack".to_owned())
            .spawn(move || poll(cancel, stopped, acknowledgment))
            .map_err(|_error| StartError::Activation)?;
        Ok(Self { stop, thread })
    }

    pub(super) fn cleanup(self) -> Result<(), ()> {
        let stopped = self.stop.send(()).map_err(|_error| ());
        let joined = self
            .thread
            .join()
            .map_err(|_panic| ())
            .and_then(|result| result);
        stopped.and(joined)
    }
}

fn poll(
    cancel: Cancel<'static>,
    stopped: Receiver<()>,
    acknowledgment: Acknowledgment,
) -> Result<(), ()> {
    let mut acknowledged = false;
    loop {
        if cancel.fired() && !acknowledged {
            acknowledgment.write();
            acknowledged = true;
        }
        match stopped.recv_timeout(Cancel::poll()) {
            Ok(()) => {
                if cancel.fired() && !acknowledged {
                    acknowledgment.write();
                }
                return Ok(());
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return Err(()),
        }
    }
}
