//! Caller-side finite admission checks cancellation before pulling another input.
use super::{CallOptions, CancelToken};
use crate::public::{Engine, Error};
type PreparedStream<'a, T> = Box<dyn Iterator<Item = Result<T, Error>> + 'a>;
impl CallOptions<'_> {
    pub(crate) fn reader_admission(&self) -> Result<(), Error> {
        self.admission()?;
        crate::engine::Cancel::default()
            .with_deadline(self.deadline()?)
            .remaining_without_check()
            .map(|_| ())
            .map_err(Error::from)
    }
    pub(crate) fn admission(&self) -> Result<(), Error> {
        if self.proxy.is_some() {
            return Err(Error::usage(
                "proxy activation is reserved and is not supported in 0.2",
            ));
        }
        if CancelToken::any(self.cancel) {
            return Err(Error::cancelled());
        }
        Ok(())
    }
}
impl Engine {
    pub(in crate::public) fn admit_prepared_stream<'a, T: 'a>(
        &self,
        records: impl Iterator<Item = Result<T, Error>> + 'a,
        options: &CallOptions<'_>,
    ) -> Result<PreparedStream<'a, T>, Error> {
        if options.eager_inputs {
            Ok(Box::new(
                self.try_within_admission(records, options)?.map(Ok),
            ))
        } else {
            Ok(Box::new(records))
        }
    }

    pub(in crate::public) fn within_admission<I: IntoIterator>(
        &self,
        records: I,
        options: &CallOptions<'_>,
    ) -> Result<std::vec::IntoIter<I::Item>, Error> {
        self.try_within_admission(records.into_iter().map(Ok), options)
    }
    pub(in crate::public) fn try_within_admission<I, T>(
        &self,
        records: I,
        options: &CallOptions<'_>,
    ) -> Result<std::vec::IntoIter<T>, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
    {
        let mut held = Vec::new();
        let mut records = records.into_iter();
        loop {
            options.admission()?;
            let Some(record) = records.next() else { break };
            options.admission()?;
            let record = record?;
            self.check_record_limit(held.len())?;
            held.push(record);
        }
        Ok(held.into_iter())
    }
}
