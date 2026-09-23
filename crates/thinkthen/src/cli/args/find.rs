//! The deliberately smaller option surface of `find`.

use std::path::PathBuf;

use crate::core::DEFAULT_MODEL;
use clap::Args;

use super::Common;

/// Everything `find` was asked before its bounded unit set is read.
#[derive(Args, Debug)]
pub(crate) struct FindArguments {
    /// The question the selected unit best answers.
    pub(crate) question: String,

    /// Let the answer say that no unit fits.
    #[arg(long)]
    pub(crate) none: bool,

    /// Options shared with the request machinery that `find` actually accepts.
    #[command(flatten)]
    pub(crate) common: FindCommon,
}

/// The shared options that apply to one aggregate `find` request.
#[derive(Args, Debug)]
pub(crate) struct FindCommon {
    /// Print the full result object in place of the original selected unit.
    #[arg(long)]
    pub(crate) details: bool,
    /// Read units from FILE instead of standard input.
    #[arg(long, value_name = "FILE")]
    pub(crate) input: Option<PathBuf>,
    /// Take each line as one text unit.
    #[arg(long, conflicts_with = "jsonl")]
    pub(crate) lines: bool,
    /// Take each line as one JSON unit.
    #[arg(long)]
    pub(crate) jsonl: bool,
    /// Send only the part of each JSON unit this RFC 6901 pointer names.
    #[arg(long, value_name = "POINTER")]
    pub(crate) field: Vec<String>,
    /// Print the complete one-request plan and send nothing.
    #[arg(long)]
    pub(crate) dry_run: bool,
    /// The backend base, which outranks THINKTHEN_BASE_URL.
    #[arg(long, value_name = "URL")]
    pub(crate) url: Option<String>,
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
    /// Write the completed exchange into DIR.
    ///
    /// An explicit recording folder suppresses the platform default cache.
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
    /// Positive seconds that bound one attempt from connect to last byte, and each retry wait.
    #[arg(
        long,
        value_name = "SECONDS",
        default_value_t = 30,
        hide_short_help = true
    )]
    pub(crate) timeout: u64,
    /// How many transport or retried-status attempts follow the first.
    #[arg(long, value_name = "N", default_value_t = 2, hide_short_help = true)]
    pub(crate) max_retries: u32,
}

impl FindCommon {
    /// Adapt the narrow parsed surface to the one shared request configuration.
    pub(crate) fn as_common(&self) -> Common {
        Common {
            details: self.details,
            input: self.input.clone(),
            lines: self.lines,
            jsonl: self.jsonl,
            csv: false,
            tsv: false,
            field: self.field.clone(),
            dry_run: self.dry_run,
            url: self.url.clone(),
            profile: self.profile.clone(),
            model: self.model.clone(),
            record: self.record.clone(),
            replay: self.replay.clone(),
            cache: self.cache.clone(),
            no_cache: self.no_cache,
            timeout: self.timeout,
            jobs: None,
            max_retries: self.max_retries,
        }
    }
}
