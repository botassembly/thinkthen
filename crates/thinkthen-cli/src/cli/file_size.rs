//! The command's `SIGXFSZ` policy.
//!
//! Under the default action, a write that reaches the file-size limit ends the
//! process. The command claims the signal once, before input, key lookup, or
//! transport, so the write fails instead and the recording failure runs its
//! normal cleanup. The engine never touches a signal disposition, so an
//! embedding host keeps the one it chose.

use crate::cli::failure::Failure;

/// Install the process-once `SIGXFSZ` handler.
///
/// # Errors
///
/// Returns the fixed recording storage failure when the handler cannot be
/// installed. Nothing has been read or sent by then.
pub(super) fn claim() -> Result<(), Failure> {
    #[cfg(unix)]
    {
        use std::sync::atomic::AtomicBool;
        use std::sync::{Arc, OnceLock};

        static CLAIMED: OnceLock<Result<(), ()>> = OnceLock::new();
        CLAIMED
            .get_or_init(|| {
                signal_hook::flag::register(
                    signal_hook::consts::signal::SIGXFSZ,
                    Arc::new(AtomicBool::new(false)),
                )
                .map(|_id| ())
                .map_err(|_error| ())
            })
            .map_err(|()| Failure::RecordingStorage)
    }
    #[cfg(not(unix))]
    {
        Ok(())
    }
}
