//! The shared record-batching options of judging verbs.

use std::path::PathBuf;

use clap::Args;

/// The batch size of a judging verb over a stream.
#[derive(Args, Debug)]
pub(crate) struct Batching {
    /// Share the exact UTF-8 contents of FILE as evidence for every batch.
    #[arg(long, value_name = "FILE", hide_short_help = true)]
    pub(crate) context: Option<PathBuf>,

    /// Select each JSON record's separate context; empty text suppresses shared context.
    #[arg(long, value_name = "POINTER", hide_short_help = true)]
    pub(crate) context_field: Option<String>,

    /// Send at most N records of a stream in one request, or `max`. [default: max]
    ///
    /// `max` fills each request to the backend's limits. `--batch 1` asks one
    /// record a request, as before batching. It beats `THINKTHEN_BATCH`, which
    /// beats a question file's `batch`.
    #[arg(long, value_name = "N|max", hide_short_help = true)]
    pub(crate) batch: Option<String>,

    /// Close a batch before its request exceeds N bytes. [default: 96000]
    #[arg(
        long,
        value_name = "N",
        hide_short_help = true,
        allow_negative_numbers = true
    )]
    pub(crate) max_request_bytes: Option<String>,
}
