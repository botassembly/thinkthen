//! Preserve actual image exchanges beside the existing answer store.
use super::{Probe, make_folder};
use crate::core::Url;
use crate::core::adapters::built_in;
use crate::core::recording::Exchange;
use crate::engine::error::Error;
use serde::Serialize;
use serde_json::value::RawValue;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Serialize)]
struct Saved<'a> {
    schema: &'static str,
    adapter: &'static str,
    url: &'a str,
    request: &'a RawValue,
    response: &'a RawValue,
    quoted: bool,
}

impl Probe {
    /// Image live replies only. Cache hits never fabricate an exchange.
    pub(crate) fn image_exchange(
        &self,
        url: &Url,
        request: &[u8],
        response: &[u8],
    ) -> Result<(), Error> {
        if self.private {
            return Ok(());
        }
        let request_json: Box<RawValue> =
            serde_json::from_slice(request).map_err(|_| Error::RecordingStorage)?;
        // Preserve the original decoder failure when no JSON exchange exists.
        let Ok(response_json) = serde_json::from_slice::<Box<RawValue>>(response) else {
            return Ok(());
        };
        let saved = serde_json::to_vec(&Saved {
            schema: "thinkthen.recording/1",
            adapter: built_in::NAME,
            url: url.as_str(),
            request: &request_json,
            response: &response_json,
            quoted: true,
        })
        .map_err(|_| Error::RecordingStorage)?;
        let folder = self.folder.join("exchanges");
        make_folder(&folder)?;
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let temp = folder.join(format!(
            ".image-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let target = folder.join(format!(
            "{}.json",
            Exchange::new(url, request).digest().as_str()
        ));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let mut file = options.open(&temp).map_err(|_| Error::RecordingStorage)?;
        let result = file
            .write_all(&saved)
            .and_then(|()| file.sync_all())
            .and_then(|()| std::fs::rename(&temp, &target))
            .map_err(|_| Error::RecordingStorage);
        if result.is_err() {
            let _removed = std::fs::remove_file(&temp);
        }
        result
    }
}
