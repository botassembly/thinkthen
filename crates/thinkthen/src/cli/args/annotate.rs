//! The annotate-only command arguments.

use std::path::PathBuf;

use clap::Args;

use super::{Batching, Common};

/// Everything `annotate` was asked, before the set or evidence is read.
#[derive(Args, Debug)]
pub(crate) struct AnnotateArguments {
    /// A JSON question set containing the named questions to ask.
    pub(crate) questions: PathBuf,

    /// A likely input file written without `--input`.
    #[arg(value_name = "INPUT", hide = true)]
    pub(crate) extra_input: Option<PathBuf>,

    /// Taken so the command can explain that thresholds belong to questions.
    #[arg(long, value_name = "T", hide = true, allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// Taken so the command can explain that every named answer prints.
    #[arg(long, hide = true)]
    pub(crate) quiet: bool,

    /// Taken so the command can explain that its output is always JSON.
    #[arg(long, hide = true)]
    pub(crate) raw: bool,

    /// Continue after a missing question-set `on` pointer, printing an error row.
    /// Requires --jsonl --details --batch 1. Other failures still stop.
    #[arg(long, value_name = "POLICY")]
    pub(crate) on_error: Option<String>,

    /// The record-batch size and request-size limit.
    #[command(flatten)]
    pub(crate) batching: Batching,

    /// The options shared with record-oriented judging commands.
    #[command(flatten)]
    pub(crate) common: Common,
}
