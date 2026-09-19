//! The command line, as the clap types that parse it.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use thinkthen_core::BackendValues;

/// Put a decider model in the shell.
#[derive(Debug, Parser)]
#[command(
    name = thinkthen_core::NAME,
    about,
    disable_version_flag = true,
    arg_required_else_help = true
)]
pub(crate) struct Cli {
    /// Print the version and exit.
    #[arg(short = 'V', long = "version")]
    pub(crate) version: bool,

    /// The command to run.
    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

/// The verbs the tool answers to.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Answer a yes/no question about the evidence and set the exit code.
    ///
    /// The answer is a bare `true`, `false`, or `null`, and the exit code is 0
    /// for yes, 1 for no, and 3 for unresolved. Under `set -e` or `set -o
    /// pipefail` a no ends the script, so put the command in an `if`, a `case`,
    /// or a `||` list.
    ///
    /// A single cut answers no when the probability did not reach the mark. It
    /// never says the model is sure of no. A three-way gate takes a band, as
    /// `--threshold 0.1:0.9` writes one.
    Decide(DecideArguments),
}

/// Everything `decide` was asked, before any of it is read.
#[derive(Args, Debug)]
pub(crate) struct DecideArguments {
    /// The question to answer, as one argument naming one visible fact.
    pub(crate) question: String,

    /// The rule: one cut T, or a band LOW:HIGH that leaves a middle unresolved.
    #[arg(long, value_name = "T|LOW:HIGH")]
    pub(crate) threshold: Option<String>,

    /// Print nothing on standard output. The exit code still carries the answer.
    #[arg(long)]
    pub(crate) quiet: bool,

    /// Print the full result object in place of the bare value.
    #[arg(long)]
    pub(crate) details: bool,

    /// Print what would be sent and stop. No key is read and no connection opens.
    #[arg(long)]
    pub(crate) dry_run: bool,

    /// The named backend profile to use.
    #[arg(long, value_name = "NAME")]
    pub(crate) profile: Option<String>,

    /// Where the request is posted, with an adapter and a model.
    #[arg(long, value_name = "URL", hide_short_help = true)]
    pub(crate) url: Option<String>,

    /// The wire format the server speaks.
    #[arg(long, value_name = "NAME", hide_short_help = true)]
    pub(crate) adapter: Option<String>,

    /// The model named in the request.
    #[arg(long, value_name = "NAME", hide_short_help = true)]
    pub(crate) model: Option<String>,

    /// The environment variable that holds the key.
    #[arg(long, value_name = "NAME", hide_short_help = true)]
    pub(crate) key_env: Option<String>,

    /// Call the backend, then write the exchange into DIR. DIR is created when absent.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) record: Option<PathBuf>,

    /// Answer from DIR alone. No connection opens, and no key is read.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) replay: Option<PathBuf>,

    /// Seconds one attempt may take, from connect to the last byte.
    #[arg(
        long,
        value_name = "SECONDS",
        default_value_t = 30,
        hide_short_help = true
    )]
    pub(crate) timeout: u64,

    /// How many times a transport failure or a retried status is sent again.
    #[arg(long, value_name = "N", default_value_t = 2, hide_short_help = true)]
    pub(crate) max_retries: u32,
}

impl DecideArguments {
    /// The five backend values the flags offered.
    pub(crate) fn backend_values(&self) -> BackendValues<'_> {
        BackendValues::new(
            self.profile.as_deref(),
            self.url.as_deref(),
            self.adapter.as_deref(),
            self.model.as_deref(),
            self.key_env.as_deref(),
        )
    }
}
