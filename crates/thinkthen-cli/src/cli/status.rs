//! Read-only, offline reporting of resolved settings and local counts.

use std::io::Write;
use std::process::ExitCode;

use serde::Serialize;

use crate::cli::args::StatusArguments;
use crate::cli::edge::{self, Environment};
use crate::failure::Failure;

#[derive(Serialize)]
struct Status {
    schema: &'static str,
    version: &'static str,
    configuration: Configuration,
    backend: Backend,
    cache: Cache,
    usage: UsageStatus,
    /// Why the usage counts are unavailable, said on standard error.
    #[serde(skip)]
    problem: Option<String>,
}

#[derive(Serialize)]
struct Configuration {
    path: Option<String>,
    present: bool,
}

#[derive(Serialize)]
struct Backend {
    name: Option<String>,
    url: String,
    url_source: &'static str,
    model: String,
    model_source: &'static str,
    key_variable: String,
    api_key_set: bool,
}

#[derive(Serialize)]
struct Cache {
    enabled: bool,
    enabled_source: &'static str,
    path: Option<String>,
    path_source: &'static str,
    /// Answers in the folder's `thinkthen.sqlite`.
    entries: Option<u64>,
    /// The store file's allocated bytes.
    bytes: Option<u64>,
    /// Old digest-named entries; admitted folders have none.
    old_entries: Option<u64>,
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

impl From<crate::Counters> for StatusCounts {
    fn from(value: crate::Counters) -> Self {
        Self {
            requests_sent: value.requests_sent(),
            retries: value.retries(),
            input_tokens: value.input_tokens(),
            output_tokens: value.output_tokens(),
            cache_answers: value.cache_answers(),
        }
    }
}

pub(crate) fn run(
    arguments: &StatusArguments,
    environment: &Environment,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let status = gather(environment, arguments.backend.as_deref())?;
    if arguments.json {
        edge::write_line(
            &mut writer,
            &serde_json::to_string(&status)
                .map_err(|_| Failure::Defect("status could not become JSON"))?,
        )?;
    } else {
        write_human(&status, &mut writer)?;
    }
    if let Some(problem) = &status.problem {
        // Status still reports everything else and exits 0 (ticket 0360).
        let mut stderr = std::io::stderr().lock();
        let _unwritten = writeln!(stderr, "thinkthen: {problem}").and_then(|()| stderr.flush());
    }
    Ok(ExitCode::SUCCESS)
}

fn gather(environment: &Environment, backend: Option<&str>) -> Result<Status, Failure> {
    let resolved = environment.resolve(backend, None, None)?;
    let cache = cache_status(&resolved)?;
    let month = crate::current_usage_month();
    let (usage, problem) = match resolved
        .usage_path
        .as_deref()
        .map(|path| crate::read_usage(path, &month))
    {
        None => (None, None),
        Some(Ok(totals)) => (Some(totals), None),
        Some(Err(failure)) => (None, Some(failure.to_string())),
    };
    Ok(Status {
        schema: "thinkthen.status/2",
        version: env!("CARGO_PKG_VERSION"),
        configuration: Configuration {
            path: resolved.configuration_path.as_deref().map(display),
            present: resolved.configuration_present,
        },
        backend: Backend {
            name: resolved.backend_name,
            url: resolved.url,
            url_source: resolved.url_source,
            model: resolved.model,
            model_source: resolved.model_source,
            key_variable: resolved.key_variable,
            api_key_set: resolved.api_key_set,
        },
        cache,
        usage: UsageStatus {
            path: resolved.usage_path.as_deref().map(display),
            month,
            this_month: usage.as_ref().map(|totals| totals.month.into()),
            total: usage.map(|totals| totals.total.into()),
        },
        problem,
    })
}

fn cache_status(settings: &crate::ResolvedSettings) -> Result<Cache, Failure> {
    let counts = settings
        .cache_path
        .as_deref()
        .map(|path| crate::cache_counts(path, settings.cache_is_platform_default))
        .transpose()
        .map_err(|error| match error.diagnostic() {
            Some(crate::ErrorDiagnostic::StoreVersion(file)) => {
                Failure::StoreVersion(file.to_owned())
            }
            _ => Failure::StatusState,
        })?;
    Ok(Cache {
        enabled: settings.cache_enabled,
        enabled_source: settings.cache_enabled_source,
        path: settings.cache_path.as_deref().map(display),
        path_source: settings.cache_path_source,
        entries: counts.as_ref().map(|counts| counts.answers),
        bytes: counts.as_ref().map(|counts| counts.bytes),
        old_entries: counts.as_ref().map(|counts| counts.old_entries),
        target_bytes: settings.cache_target_bytes,
        target_source: settings.cache_target_source,
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
    edge::write_line(
        &mut writer,
        &format!(
            "backend {}",
            status.backend.name.as_deref().unwrap_or("none")
        ),
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
        &format!("key_variable {}", status.backend.key_variable),
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
    optional_line(&mut writer, "cache_old_entries", status.cache.old_entries)?;
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
