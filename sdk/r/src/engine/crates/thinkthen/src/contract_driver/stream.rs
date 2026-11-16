//! Async rows are their own types, so `StreamExt` and `Iterator` never clash.
use super::*;

/// A function's typed rows as an async [`futures_core::Stream`], from [`Rows::into_stream`].
///
/// It is only a stream, so `while let Some(row) = rows.next().await` works with
/// `StreamExt` imported. Dropping it cancels the call, as dropping [`Rows`] does.
/// ```compile_fail
/// # let engine = thinkthen::Settings::default().engine(|_| None).unwrap();
/// # let refund = thinkthen::Decide::new("Refund?").unwrap();
/// let stream = engine.rows(&refund, ["Refund me."]).unwrap().into_stream();
/// for row in stream {}
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
#[non_exhaustive]
#[derive(Debug)]
pub struct RowStream<F: Function>(Rows<F>);
impl<F: Function> RowStream<F> {
    /// Cancel this call without affecting another call on the same engine.
    pub fn cancel(&self) {
        self.0.cancel();
    }
}
impl<F: Function> Rows<F> {
    /// Read these rows as an async stream instead of a blocking iterator.
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    #[must_use]
    pub fn into_stream(self) -> RowStream<F> {
        RowStream(self)
    }
}
impl<F: Function> futures_core::Stream for RowStream<F> {
    type Item = Result<Row<F>, Error>;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let rows = &mut self.get_mut().0;
        match rows.inner.poll(cx) {
            Poll::Ready(Some(row)) => Poll::Ready(Some(rows.admit(row))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Rows from a question selected at runtime, as an async stream, from [`AnyRows::into_stream`].
#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
#[non_exhaustive]
#[derive(Debug)]
pub struct AnyRowStream(AnyRows);
impl AnyRowStream {
    /// Cancel this call and wake its blocked input and output operations.
    pub fn cancel(&self) {
        self.0.cancel();
    }
}
impl AnyRows {
    /// Read these rows as an async stream instead of a blocking iterator.
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    #[must_use]
    pub fn into_stream(self) -> AnyRowStream {
        AnyRowStream(self)
    }
}
impl futures_core::Stream for AnyRowStream {
    type Item = Result<AnyRow, Error>;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().0.poll(cx)
    }
}
