//! Typed push calls return the admitted input they were sent.
use super::*;
/// Typed push results retaining each original admitted record.
#[derive(Debug)]
pub struct RecordRows<F: Function>(pub(super) Rows<F>);
impl<F: Function> RecordRows<F> {
    /// Cancel this operation without affecting another call.
    pub fn cancel(&self) {
        self.0.cancel();
    }
}
impl<F: Function> Iterator for RecordRows<F> {
    type Item = Result<Row<F, Record>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|row| row.and_then(Row::into_record_result))
    }
}
#[cfg(feature = "stream")]
/// Async push results retaining each original admitted record.
#[derive(Debug)]
pub struct RecordRowStream<F: Function>(RowStream<F>);
#[cfg(feature = "stream")]
impl<F: Function> RecordRows<F> {
    /// Read these results asynchronously; dropping them cancels the call.
    #[must_use]
    pub fn into_stream(self) -> RecordRowStream<F> {
        RecordRowStream(self.0.into_stream())
    }
}
#[cfg(feature = "stream")]
impl<F: Function> RecordRowStream<F> {
    /// Cancel this operation without affecting another call.
    pub fn cancel(&self) {
        self.0.cancel();
    }
}
#[cfg(feature = "stream")]
impl<F: Function> futures_core::Stream for RecordRowStream<F> {
    type Item = Result<Row<F, Record>, Error>;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match futures_core::Stream::poll_next(std::pin::Pin::new(&mut self.get_mut().0), cx) {
            Poll::Ready(Some(row)) => Poll::Ready(Some(row.and_then(Row::into_record_result))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
