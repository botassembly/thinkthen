//! Caller-owned limits on retained request attachments.
use super::CallOptions;
use crate::Error;

impl CallOptions<'_> {
    /// Bound the total compressed attachment bytes retained by a named call.
    /// Inline bytes are preflighted before admission clones them. Deferred files
    /// share one budget and each occurrence counts, including repeated paths.
    ///
    /// # Errors
    /// Returns Usage when the limit is zero.
    pub fn retained_attachment_limit(mut self, bytes: usize) -> Result<Self, Error> {
        self.attachment_limit = Some(
            crate::public::request::transport::TransportAttachmentLimit::new(bytes)?,
        );
        Ok(self)
    }
}
