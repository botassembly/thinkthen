//! One statement counts each affected input once, after host representation conversion.
use thinkthen::{Error, native, results::AnyRow};
#[derive(Debug)]
pub(super) struct Failures {
    total: u64,
    failed: usize,
    first: Option<(u64, Error)>,
    last: Option<(u64, bool)>,
}
impl Failures {
    pub(super) fn new(total: u64) -> Self {
        Self {
            total,
            failed: 0,
            first: None,
            last: None,
        }
    }
    pub(super) fn sent(&mut self) {
        self.total = self.total.saturating_add(1);
    }
    pub(super) fn observe(&mut self, row: &AnyRow) {
        self.last = Some((row.position(), false));
        if let Some(error) = row.failure() {
            self.record(row.position(), error);
        }
    }
    fn record(&mut self, position: u64, error: &Error) {
        if let Some((_, failed)) = &mut self.last {
            if !*failed {
                self.failed = self.failed.saturating_add(1);
            }
            *failed = true;
        }
        if self
            .first
            .as_ref()
            .is_none_or(|(first, _)| position <= *first)
        {
            self.first = Some((position, error.clone()));
        }
    }
    pub(super) fn replace(&mut self, position: u64, error: &Error) -> Result<(), Error> {
        if !error.code().is_record() || self.last.is_none_or(|(last, _)| position != last) {
            return Err(native::abi_error());
        }
        self.record(position, error);
        Ok(())
    }
    pub(super) fn finish(&mut self) {
        self.last = None;
    }
    pub(super) fn error(&self) -> Option<Error> {
        self.first.as_ref().map(|(position, error)| {
            native::records_failed(error, self.failed, self.total, *position)
        })
    }
}
