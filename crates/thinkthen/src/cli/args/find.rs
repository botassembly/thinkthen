//! The deliberately smaller option surface of `find`.

use std::ffi::OsString;
use std::path::PathBuf;

use crate::core::DEFAULT_MODEL;
use clap::Args;

use super::Common;

/// Everything `find` was asked before its bounded unit set is read.
#[derive(Args, Debug)]
pub(crate) struct FindArguments {
    /// The question the selected line or record best answers.
    pub(crate) question: String,

    /// Taken so the command can say where the evidence goes.
    #[arg(value_name = "EVIDENCE", hide = true)]
    pub(crate) extra: Vec<OsString>,

    /// Let the answer say that no line or record fits.
    #[arg(long)]
    pub(crate) none: bool,

    /// Options shared with the request machinery that `find` actually accepts.
    #[command(flatten)]
    pub(crate) common: FindCommon,
}

/// The shared options that apply to one aggregate `find` request.
#[derive(Args)]
pub(crate) struct FindCommon {
    /// Print one machine-readable run-facts line last on standard error.
    #[arg(long, hide_short_help = true)]
    pub(crate) facts: bool,
    /// Refuse live attempts after this process has sent N.
    #[arg(long, value_name = "N", hide_short_help = true)]
    pub(crate) max_requests_total: Option<u64>,
    /// Limit estimated input admission for live final encoded bodies.
    #[arg(long, value_name = "N", hide_short_help = true)]
    pub(crate) max_estimated_input_tokens_total: Option<u64>,

    /// Print the full result object in place of the original selected line or record.
    /// The result names each line or record by its one-based place, zero-padded
    /// to three digits: u001 is the first and u255 the 255th.
    #[arg(long)]
    pub(crate) details: bool,
    /// Read the lines or records from FILE instead of standard input.
    #[arg(long, value_name = "FILE")]
    pub(crate) input: Option<PathBuf>,
    /// Take each line as one text record.
    #[arg(long, conflicts_with = "jsonl")]
    pub(crate) lines: bool,
    /// Take each line as one JSON record.
    #[arg(long)]
    pub(crate) jsonl: bool,
    /// Send only the part of each JSON record this RFC 6901 pointer names.
    #[arg(long, value_name = "POINTER")]
    pub(crate) field: Vec<String>,
    /// Print the complete one-request plan and send nothing.
    #[arg(long = "plan")]
    pub(crate) dry_run: bool,
    /// The removed spelling is parsed only to give the migration sentence.
    #[arg(long = "dry-run", hide = true)]
    pub(crate) retired_dry_run: bool,
    /// The backend base, which outranks THINKTHEN_BASE_URL.
    #[arg(long, value_name = "URL")]
    pub(crate) url: Option<String>,
    /// The named backend: a base with its own key variable and model. It outranks THINKTHEN_BACKEND.
    #[arg(long, value_name = "NAME")]
    pub(crate) backend: Option<String>,
    /// Read enforceable backend limits and a calibration name from FILE.
    #[arg(long, value_name = "FILE")]
    pub(crate) profile: Option<PathBuf>,
    /// The model named in the request.
    #[arg(
        long,
        value_name = "NAME",
        hide_short_help = true,
        help = format!("The model named in the request. [default: {DEFAULT_MODEL}]")
    )]
    pub(crate) model: Option<String>,
    /// Call the backend for every request, then write its exchange into DIR.
    ///
    /// A held request is sent again and may be billed again. A different fresh
    /// answer exits 5 without printing it; the old entry stays. Use --cache
    /// DIR to reuse held answers, or record into a new empty folder for a
    /// deliberate fresh run. An explicit recording folder suppresses the
    /// platform default cache.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) record: Option<PathBuf>,
    /// Answer from DIR alone.
    ///
    /// An explicit replay folder suppresses the platform default cache.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) replay: Option<PathBuf>,
    /// Replay and record through one DIR, overriding THINKTHEN_CACHE and the platform default.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) cache: Option<PathBuf>,
    /// Do not read or write answers in the default cache.
    ///
    /// Answers are otherwise cached by default in the platform cache folder.
    /// Entries contain the judged text. A new default folder is private to its
    /// owner. This option conflicts with --cache but may accompany an explicit
    /// --record or --replay folder.
    #[arg(long, conflicts_with = "cache")]
    pub(crate) no_cache: bool,
    /// Send the requested exchange live and replace its complete cache entry.
    #[arg(long, conflicts_with_all = ["record", "replay", "no_cache"], hide_short_help = true)]
    pub(crate) refresh_cache: bool,
    /// Seconds from 1 to 86400 that bound one attempt from connect to last byte, and each retry wait.
    #[arg(
        long,
        value_name = "SECONDS",
        default_value_t = 30,
        hide_short_help = true
    )]
    pub(crate) timeout: u64,
    /// How many times a retried status is sent again. A transport failure is never sent again.
    #[arg(long, value_name = "N", default_value_t = super::DEFAULT_MAX_RETRIES, hide_short_help = true)]
    pub(crate) max_retries: u32,
}

impl FindCommon {
    /// Adapt the narrow parsed surface to the one shared request configuration.
    pub(crate) fn as_common(&self) -> Common {
        Common {
            facts: self.facts,
            max_requests_total: self.max_requests_total,
            max_estimated_input_tokens_total: self.max_estimated_input_tokens_total,
            details: self.details,
            input: self.input.iter().cloned().collect(),
            window: None,
            lines: self.lines,
            jsonl: self.jsonl,
            csv: false,
            tsv: false,
            field: self.field.clone(),
            dry_run: self.dry_run,
            retired_dry_run: self.retired_dry_run,
            url: self.url.clone(),
            backend: self.backend.clone(),
            profile: self.profile.clone(),
            model: self.model.clone(),
            record: self.record.clone(),
            replay: self.replay.clone(),
            cache: self.cache.clone(),
            no_cache: self.no_cache,
            refresh_cache: self.refresh_cache,
            timeout: self.timeout,
            jobs: None,
            max_retries: self.max_retries,
        }
    }
}
