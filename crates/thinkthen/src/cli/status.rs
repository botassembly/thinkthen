//! Read-only, offline reporting of resolved settings and local counts.

use std::io::Write;
use std::process::ExitCode;

use serde::Serialize;

use crate::cli::args::StatusArguments;
use crate::cli::edge::{self, Environment};
use crate::core::DEFAULT_MODEL;
use crate::failure::Failure;

#[derive(Serialize)]
struct Status {
    schema: &'static str,
    version: &'static str,
    configuration: Configuration,
    backend: Backend,
    cache: Cache,
    usage: UsageStatus,
}

#[derive(Serialize)]
struct Configuration {
    path: Option<String>,
    present: bool,
}

#[derive(Serialize)]
struct Backend {
    url: String,
    url_source: &'static str,
    model: String,
    model_source: &'static str,
    api_key_set: bool,
}

#[derive(Serialize)]
struct Cache {
    enabled: bool,
    enabled_source: &'static str,
    path: Option<String>,
    path_source: &'static str,
    entries: Option<u64>,
    bytes: Option<u64>,
    #[serde(rename = "prune_target_bytes")]
    target_bytes: u64,
    #[serde(rename = "prune_target_source")]
    target_source: &'static str,
}

#[derive(Serialize)]
struct UsageStatus {
    path: Option<String>,
    month: String,
    this_month: Option<StatusCounts>,
    total: Option<StatusCounts>,
}

#[derive(Serialize)]
struct StatusCounts {
    requests_sent: u64,
    retries: u64,
    input_tokens: u64,
    output_tokens: u64,
    cache_answers: u64,
}

impl From<crate::engine::usage::Counts> for StatusCounts {
    fn from(value: crate::engine::usage::Counts) -> Self {
        Self {
            requests_sent: value.requests_sent,
            retries: value.retries,
            input_tokens: value.input_tokens,
            output_tokens: value.output_tokens,
            cache_answers: value.cache_answers,
        }
    }
}

pub(crate) fn run(
    arguments: &StatusArguments,
    environment: &Environment,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let status = gather(environment)?;
    if arguments.json {
        edge::write_line(
            &mut writer,
            &serde_json::to_string(&status)
                .map_err(|_| Failure::Defect("status could not become JSON"))?,
        )?;
    } else {
        write_human(&status, &mut writer)?;
    }
    Ok(ExitCode::SUCCESS)
}

fn gather(environment: &Environment) -> Result<Status, Failure> {
    let config = environment.config();
    let url_source = if environment.base_url_is_environment() {
        "environment"
    } else if config.has_url() {
        "configuration"
    } else {
        "built_in"
    };
    let model_source = if config.has_model() {
        "configuration"
    } else {
        "built_in"
    };
    let enabled_source = if environment.named_cache() {
        "environment"
    } else if config.has_cache() {
        "configuration"
    } else {
        "built_in"
    };
    let target_source = if config.has_cache_bytes() {
        "configuration"
    } else {
        "built_in"
    };
    let cache_counts = environment
        .cache()
        .map(|path| {
            crate::engine::cache_prune::inspect(path, environment.cache_is_platform_default())
        })
        .transpose()
        .map_err(|_| Failure::StatusState)?;
    let month = crate::engine::usage::month_now();
    let usage = environment
        .usage_path()
        .map(|path| crate::engine::usage::read(path, &month))
        .transpose()
        .map_err(|_| Failure::StatusState)?;
    let resolved_backend = crate::core::Backend::resolve(
        None,
        environment.base_url(),
        environment.model().unwrap_or(DEFAULT_MODEL),
    )?;
    Ok(Status {
        schema: "thinkthen.status/1",
        version: env!("CARGO_PKG_VERSION"),
        configuration: Configuration {
            path: environment.config_path().map(display),
            present: config.present(),
        },
        backend: Backend {
            url: resolved_backend.url().as_str().to_owned(),
            url_source,
            model: environment.model().unwrap_or(DEFAULT_MODEL).to_owned(),
            model_source,
            api_key_set: environment.api_key_set(),
        },
        cache: Cache {
            enabled: environment.named_cache() || environment.default_cache_enabled(),
            enabled_source,
            path: environment.cache().map(display),
            path_source: if environment.named_cache() {
                "environment"
            } else {
                "platform"
            },
            entries: cache_counts.as_ref().map(|counts| counts.entries),
            bytes: cache_counts.as_ref().map(|counts| counts.bytes),
            target_bytes: environment.cache_bytes(),
            target_source,
        },
        usage: UsageStatus {
            path: environment.usage_path().map(display),
            month,
            this_month: usage.as_ref().map(|totals| totals.month.into()),
            total: usage.map(|totals| totals.total.into()),
        },
    })
}

fn display(path: &std::path::Path) -> String {
    path.to_string_lossy().into_owned()
}

fn write_human(status: &Status, mut writer: impl Write) -> Result<(), Failure> {
    edge::write_line(&mut writer, &format!("version {}", status.version))?;
    edge::write_line(
        &mut writer,
        &format!(
            "configuration_path {}",
            available(&status.configuration.path)
        ),
    )?;
    edge::write_line(
        &mut writer,
        &format!("configuration_present {}", status.configuration.present),
    )?;
    edge::write_line(&mut writer, &format!("url {}", status.backend.url))?;
    edge::write_line(
        &mut writer,
        &format!("url_source {}", status.backend.url_source),
    )?;
    edge::write_line(&mut writer, &format!("model {}", status.backend.model))?;
    edge::write_line(
        &mut writer,
        &format!("model_source {}", status.backend.model_source),
    )?;
    edge::write_line(
        &mut writer,
        &format!("api_key_set {}", status.backend.api_key_set),
    )?;
    edge::write_line(
        &mut writer,
        &format!("cache_enabled {}", status.cache.enabled),
    )?;
    edge::write_line(
        &mut writer,
        &format!("cache_enabled_source {}", status.cache.enabled_source),
    )?;
    edge::write_line(
        &mut writer,
        &format!("cache_path {}", available(&status.cache.path)),
    )?;
    edge::write_line(
        &mut writer,
        &format!("cache_path_source {}", status.cache.path_source),
    )?;
    optional_line(&mut writer, "cache_entries", status.cache.entries)?;
    optional_line(&mut writer, "cache_bytes", status.cache.bytes)?;
    edge::write_line(
        &mut writer,
        &format!("cache_prune_target_bytes {}", status.cache.target_bytes),
    )?;
    edge::write_line(
        &mut writer,
        &format!("cache_prune_target_source {}", status.cache.target_source),
    )?;
    edge::write_line(
        &mut writer,
        &format!("usage_path {}", available(&status.usage.path)),
    )?;
    edge::write_line(&mut writer, &format!("usage_month {}", status.usage.month))?;
    counts(&mut writer, "month", status.usage.this_month.as_ref())?;
    counts(&mut writer, "total", status.usage.total.as_ref())
}

fn available(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("unavailable")
}

fn optional_line(writer: &mut impl Write, name: &str, value: Option<u64>) -> Result<(), Failure> {
    edge::write_line(
        writer,
        &format!(
            "{name} {}",
            value.map_or_else(|| "unavailable".to_owned(), |number| number.to_string())
        ),
    )?;
    Ok(())
}

fn counts(
    writer: &mut impl Write,
    prefix: &str,
    value: Option<&StatusCounts>,
) -> Result<(), Failure> {
    optional_line(
        writer,
        &format!("{prefix}_requests_sent"),
        value.map(|counts| counts.requests_sent),
    )?;
    optional_line(
        writer,
        &format!("{prefix}_retries"),
        value.map(|counts| counts.retries),
    )?;
    optional_line(
        writer,
        &format!("{prefix}_input_tokens"),
        value.map(|counts| counts.input_tokens),
    )?;
    optional_line(
        writer,
        &format!("{prefix}_output_tokens"),
        value.map(|counts| counts.output_tokens),
    )?;
    optional_line(
        writer,
        &format!("{prefix}_cache_answers"),
        value.map(|counts| counts.cache_answers),
    )
}
