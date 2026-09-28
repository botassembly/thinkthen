//! Bounded native work over the process width and ordered worker admission.

use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::engine::workers;

use super::Engine;

impl Engine {
    pub(crate) fn ask_batches_recoverable<W, R, E>(
        &self,
        cancel: &Cancel<'_>,
        items: Vec<W>,
        work: &(impl Fn(W) -> Result<R, Error> + Sync),
        each: impl FnMut(R) -> Result<(), E>,
    ) -> Result<(), E>
    where
        W: Send,
        R: Send,
        E: From<Error>,
    {
        let width = self.state(cancel)?.width;
        workers::ordered(width, items, cancel, work, each)
    }
}
