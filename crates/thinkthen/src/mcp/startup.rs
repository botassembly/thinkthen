//! Installed command settings and owned stdio over one native Engine.
use crate::{EngineBuilder, Error};
use clap::Args;
use std::{io::BufReader, path::PathBuf, time::Duration};

const HELP: &str = "Local stdio server with ten judging tools. Tool arguments use exactly one question selector: question (literal text or an authored object), question_file, question_name, or question_reference. Supply evidence text, records, source, or inputs; ordered images may accompany evidence for decide/choose/score. options holds applicable native controls; it never selects credentials or a backend. The compact catalog is guidance; native admission validates the full grammar.

Authored question objects use decide, choose with options, tag with labels, score with levels, find, recognize, or relate. Annotate uses a versioned questions object; rank accepts a decide/score question or an ordered question set. Structured questions retain ordinary saved-question grammar.

Full selectors, question grammar, descriptors, source fields and applicable controls:
https://github.com/botassembly/thinkthen/blob/main/specification/mcp.md
https://github.com/botassembly/thinkthen/blob/main/libraries/mcp/README.md";

#[derive(Args)]
#[command(long_about = HELP)]
pub(crate) struct Arguments {
    /// Select one named backend with its approved environment key.
    #[arg(long)]
    backend: Option<String>,
    /// Use this explicit backend address.
    #[arg(long)]
    url: Option<String>,
    /// Select one literal model.
    #[arg(long)]
    model: Option<String>,
    /// Apply this existing native admission profile.
    #[arg(long)]
    profile: Option<PathBuf>,
    /// Bound one attempt in seconds, from 1 to 86400.
    #[arg(long, default_value_t = 30)]
    timeout: u64,
    /// Bound status retries; transport failures are not retried.
    #[arg(long, default_value_t = crate::cli::args::DEFAULT_MAX_RETRIES)]
    max_retries: u32,
    /// Native concurrency, from 1 to 32.
    #[arg(long)]
    jobs: Option<u8>,
    /// Bound original records admitted per call.
    #[arg(long)]
    max_requests: Option<usize>,
    /// Bound attempts sent by this process.
    #[arg(long)]
    max_requests_total: Option<u64>,
    /// Bound estimated encoded input admission across this process.
    #[arg(long)]
    max_estimated_input_tokens_total: Option<u64>,
    /// Bound encoded request bytes.
    #[arg(long)]
    max_request_bytes: Option<usize>,
    /// Record live exchanges in this explicit folder.
    #[arg(long)]
    record: Option<PathBuf>,
    /// Read only saved exchanges; sends nothing.
    #[arg(long)]
    replay: Option<PathBuf>,
    /// Read and write this explicit cache folder.
    #[arg(long, conflicts_with_all = ["no_cache", "record", "replay"])]
    cache: Option<PathBuf>,
    /// Disable the platform answer cache.
    #[arg(long)]
    no_cache: bool,
    /// Refresh answers through the existing native cache policy.
    #[arg(long)]
    refresh_cache: bool,
}
impl std::fmt::Debug for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("McpArguments").finish_non_exhaustive()
    }
}
impl Arguments {
    fn engine(&self) -> Result<crate::Engine, Error> {
        let (mut builder, config_shared) = EngineBuilder::from_env_with_config_status()?;
        crate::cli::warn_configuration_shared(config_shared, std::io::stderr().lock());
        if let Some(value) = &self.backend {
            builder = builder.backend(value)?;
        }
        if let Some(value) = &self.url {
            builder = builder.base_url(value)?;
        }
        if let Some(value) = &self.model {
            builder = builder.model(value)?;
        }
        if let Some(value) = &self.profile {
            builder = builder.profile(value)?;
        }
        if let Some(value) = self.jobs {
            builder = builder.throttle(value)?;
        }
        if let Some(value) = self.max_requests {
            builder = builder.max_requests(Some(value))?;
        }
        if let Some(value) = self.max_request_bytes {
            builder = builder.max_request_bytes(value)?;
        }
        if self.no_cache {
            builder = builder.no_cache();
        }
        if let Some(value) = &self.cache {
            builder = builder.cache_at(value)?;
        }
        if let Some(value) = &self.record {
            builder = builder.record(value)?;
        }
        if let Some(value) = &self.replay {
            builder = builder.replay(value)?;
        }
        if let Some(value) = self.max_requests_total {
            builder = builder.max_requests_total(Some(value));
        }
        if let Some(value) = self.max_estimated_input_tokens_total {
            builder = builder.max_estimated_input_tokens_total(Some(value));
        }
        builder
            .refresh_cache(self.refresh_cache)
            .timeout(Duration::from_secs(self.timeout))?
            .max_retries(self.max_retries)
            .build()
    }
}
fn run(arguments: &Arguments) -> Result<(), Error> {
    let engine = arguments.engine()?;
    let executor = super::executor::NativeExecutor { engine };
    let (input_file, output_file) =
        super::input::pipes().map_err(|_| Error::local("MCP stdio pipes unavailable"))?;
    #[cfg(unix)]
    let result = super::runtime::serve(
        move |stop| {
            Ok(BufReader::new(super::input::PollInput::new(
                input_file, stop,
            )))
        },
        move |stop| Ok(super::input::PollOutput::new(output_file, stop)),
        executor,
    );
    #[cfg(windows)]
    let result = super::runtime::serve(
        move |stop| super::input::PipeInput::new(input_file, stop).map(BufReader::new),
        move |stop| super::input::PipeOutput::new(output_file, stop),
        executor,
    );
    result.map_err(|_| Error::local("MCP stdio session closed"))
}

pub(crate) fn entry(arguments: &Arguments) -> std::process::ExitCode {
    use std::io::Write as _;
    match run(arguments) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            let _unwritten = writeln!(
                std::io::stderr().lock(),
                "thinkthen: {}",
                error.detail().message()
            );
            std::process::ExitCode::from(match error.kind() {
                crate::ErrorKind::Usage => 2,
                crate::ErrorKind::Local => 5,
                crate::ErrorKind::Backend => 4,
                crate::ErrorKind::Defect => 70,
                crate::ErrorKind::Cancelled => 130,
                crate::ErrorKind::Deadline => 124,
            })
        }
    }
}
