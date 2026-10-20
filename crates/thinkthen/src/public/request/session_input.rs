//! One bounded decoded logical input, shared by readiness and actual composition.
use super::{RequestFraming, RequestSessionDescriptor, framing::Decoder, session_queue::Queue};
use crate::{CallOptions, Error};

pub(super) enum Read {
    Pending,
    Item(Box<Result<RequestSessionDescriptor, Error>>),
    End,
}

pub(super) struct Logical {
    decoder: Decoder,
    staged: Option<Option<Result<RequestSessionDescriptor, Error>>>,
    stopped: bool,
}
impl std::fmt::Debug for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogicalInput")
            .field("staged", &self.staged.is_some())
            .field("stopped", &self.stopped)
            .finish_non_exhaustive()
    }
}
impl Logical {
    pub(super) fn new(framing: RequestFraming) -> Self {
        Self {
            decoder: Decoder::new(framing),
            staged: None,
            stopped: false,
        }
    }
    fn ready(&mut self, queue: &Queue, controls: CallOptions<'_>) -> (bool, bool, bool) {
        if self.staged.is_some() || self.stopped {
            return (true, false, false);
        }
        // Consume at most one raw descriptor. Header/blank progress reschedules
        // another check if input or EOF is already available, without spinning
        // on an empty feed or sleeping over queued work.
        let result = match queue.try_input(controls) {
            Read::Pending => return (false, false, false),
            Read::End => {
                self.stopped = true;
                self.staged = Some(
                    controls
                        .reader_admission()
                        .and_then(|()| self.decoder.finish())
                        .err()
                        .map(Err),
                );
                return (true, false, false);
            }
            Read::Item(value) => (*value).and_then(|value| self.decoder.descriptor(value)),
        };
        match result {
            Ok(None) => (false, false, true),
            Ok(Some(value)) => {
                self.staged = Some(Some(Ok(value)));
                (true, false, false)
            }
            Err(error) => {
                self.stopped = true;
                self.staged = Some(Some(Err(error)));
                (true, true, false)
            }
        }
    }
    fn next(
        &mut self,
        queue: &Queue,
        controls: CallOptions<'_>,
    ) -> Option<Result<RequestSessionDescriptor, Error>> {
        if let Some(staged) = self.staged.take() {
            return staged;
        }
        if self.stopped {
            return None;
        }
        let value = self.decoder.next(queue, controls);
        self.stopped = value.as_ref().is_none_or(Result::is_err);
        value
    }
}
impl Queue {
    pub(super) fn configure_input(&self, framing: RequestFraming) {
        *self
            .logical
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Logical::new(framing));
    }
    pub(super) fn logical_ready(&self, controls: CallOptions<'_>) -> bool {
        let (ready, close, progress) = self
            .logical
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_mut()
            .map_or((false, false, false), |logical| {
                logical.ready(self, controls)
            });
        if close {
            self.close_intake();
        }
        if progress {
            self.reschedule_input();
        }
        ready
    }
    pub(super) fn next_decoded(
        &self,
        controls: CallOptions<'_>,
    ) -> Option<Result<RequestSessionDescriptor, Error>> {
        // Admission remains authoritative before attachment reads/composition,
        // including a descriptor staged while the native host was idle.
        if let Err(error) = controls.reader_admission() {
            self.close_intake();
            return Some(Err(error));
        }
        let value = self
            .logical
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_mut()
            .and_then(|logical| logical.next(self, controls));
        if value.as_ref().is_some_and(Result::is_err) {
            self.close_intake();
        }
        value
    }
}

pub(super) struct Registration<'a>(&'a Queue);
impl<'a> Registration<'a> {
    pub(super) fn new(queue: &'a Queue, wake: crate::public::readiness::Wake) -> Self {
        queue.register_input_wake(Some(wake));
        Self(queue)
    }
}
impl Drop for Registration<'_> {
    fn drop(&mut self) {
        self.0.register_input_wake(None);
    }
}
