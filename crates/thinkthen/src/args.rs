//! The command line, as the clap types that parse it.

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

/// The command families the tool answers to.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Judge meaning.
    #[command(subcommand)]
    Decide(Decide),
}

/// The verbs `decide` knows.
#[derive(Debug, Subcommand)]
pub(crate) enum Decide {
    /// Ask whether a condition holds for the evidence on standard input.
    If(IfArguments),
}

/// Everything `decide if` was asked, before any of it is read.
#[derive(Args, Debug)]
pub(crate) struct IfArguments {
    /// The condition to judge, as one argument naming one visible fact.
    pub(crate) condition: String,

    /// The pass mark, above 0.5 and at most 1. Without it nothing is accepted.
    #[arg(long, value_name = "P")]
    pub(crate) min_prob: Option<f64>,

    /// Set the exit code from the assessment: 0 yes, 1 no, 3 unsure.
    #[arg(long, requires = "min_prob")]
    pub(crate) status: bool,

    /// Print what would be sent and stop. No key is read and no connection opens.
    #[arg(long)]
    pub(crate) plan: bool,

    /// The named backend profile to use [env: THINKTHEN_BACKEND]
    #[arg(long, value_name = "NAME")]
    pub(crate) backend: Option<String>,

    /// Where the request is posted, with an adapter and a model [env: THINKTHEN_URL]
    #[arg(long, value_name = "URL")]
    pub(crate) url: Option<String>,

    /// The wire format the server speaks [env: THINKTHEN_ADAPTER]
    #[arg(long, value_name = "NAME")]
    pub(crate) adapter: Option<String>,

    /// The model named in the request [env: THINKTHEN_MODEL]
    #[arg(long, value_name = "NAME")]
    pub(crate) model: Option<String>,

    /// The environment variable that holds the key [env: THINKTHEN_KEY_ENV]
    #[arg(long, value_name = "NAME")]
    pub(crate) key_env: Option<String>,

    /// Seconds the whole exchange may take.
    #[arg(long, value_name = "SECONDS", default_value_t = 30)]
    pub(crate) timeout: u64,

    /// How many times a transport failure or a retried status is sent again.
    #[arg(long, value_name = "N", default_value_t = 2)]
    pub(crate) max_retries: u32,
}

impl IfArguments {
    /// The five backend values the flags offered.
    pub(crate) fn backend_values(&self) -> BackendValues<'_> {
        BackendValues::new(
            self.backend.as_deref(),
            self.url.as_deref(),
            self.adapter.as_deref(),
            self.model.as_deref(),
            self.key_env.as_deref(),
        )
    }
}
