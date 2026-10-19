//! Backend administration keeps the existing check argument contract.

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub(crate) enum BackendCommand {
    /// Check that a backend you name works with this tool, with rich probes and ten minimal function calls.
    ///
    /// It exits 0 only when no finding is critical. Every request is real spend.
    /// The report names the model asked for, the model sent, and the model each
    /// reply names.
    Check(CheckArguments),
}

#[derive(Args)]
pub(crate) struct CheckArguments {
    /// The base the requests are posted under, which outranks THINKTHEN_BASE_URL.
    #[arg(long, value_name = "URL")]
    pub(crate) url: Option<String>,
    /// The named backend: a base with its own key variable and model. It outranks THINKTHEN_BACKEND.
    #[arg(long, value_name = "NAME")]
    pub(crate) backend: Option<String>,
    /// The model named in each request, resolved as every command resolves it.
    #[arg(long, value_name = "NAME")]
    pub(crate) model: Option<String>,
    /// Seconds from 1 to 86400 that bound one attempt from connect to last byte, and each retry wait.
    #[arg(long, value_name = "SECONDS", default_value_t = 30)]
    pub(crate) timeout: u64,
    /// Print rich request bodies, function plans and request bounds. An optional key is checked
    /// against the address; no key is required and nothing is sent.
    #[arg(long = "plan")]
    pub(crate) dry_run: bool,
    /// The removed spelling is parsed only to give the migration sentence.
    #[arg(long = "dry-run", hide = true)]
    pub(crate) retired_dry_run: bool,
}

impl std::fmt::Debug for CheckArguments {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CheckArguments")
            .field("url", &self.url.as_ref().map(|_| "<withheld>"))
            .field("backend", &self.backend.as_ref().map(|_| "<withheld>"))
            .field("model", &self.model)
            .field("timeout", &self.timeout)
            .field("dry_run", &self.dry_run)
            .finish()
    }
}
