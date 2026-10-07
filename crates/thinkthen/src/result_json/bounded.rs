//! Measure escaped presentation bytes before retaining an expanded aggregate.
use serde::Serialize;
use std::io::{self, Write};
pub(crate) struct OutputBudget(pub(crate) usize);
impl OutputBudget {
    pub(crate) fn admit(&mut self, value: &impl Serialize, separator: bool) -> Result<(), ()> {
        if separator {
            self.write_all(b",").map_err(|_| ())?;
        }
        serde_json::to_writer(self, value).map_err(|_| ())
    }
}
impl Write for OutputBudget {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0 = self
            .0
            .checked_sub(bytes.len())
            .ok_or_else(|| io::Error::other("aggregate presentation exceeds its bound"))?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
