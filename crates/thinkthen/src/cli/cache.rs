//! Explicit cache maintenance commands.

use std::io::Write;
use std::process::ExitCode;
use std::time::Duration;

use crate::cli::args::PruneArguments;
use crate::cli::edge;
use crate::cli::edge::Environment;
use crate::failure::Failure;

pub(crate) fn prune(
    arguments: &PruneArguments,
    environment: &Environment,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let max_size = positive(arguments.max_size.as_deref(), "--max-size")?
        .unwrap_or_else(|| environment.cache_bytes());
    let older_than = arguments.older_than.as_deref().map(duration).transpose()?;
    if arguments
        .answered_by_other_than
        .as_ref()
        .is_some_and(|model| model.trim().is_empty())
    {
        return Err(Failure::Usage(
            "--answered-by-other-than takes a nonblank model",
        ));
    }
    let result = crate::engine::cache_prune::run(
        &arguments.directory,
        &crate::engine::cache_prune::Prune {
            max_size,
            older_than,
            answered_by_other_than: arguments.answered_by_other_than.clone(),
        },
    )?;
    edge::write_line(
        writer,
        &format!(
            "removed {} entries and {} bytes; {} entries and {} bytes remain",
            result.removed_entries,
            result.removed_bytes,
            result.remaining_entries,
            result.remaining_bytes
        ),
    )?;
    Ok(ExitCode::SUCCESS)
}

fn positive(value: Option<&str>, option: &'static str) -> Result<Option<u64>, Failure> {
    value
        .map(|text| {
            text.parse::<u64>()
                .ok()
                .filter(|number| *number > 0)
                .ok_or(Failure::Usage(match option {
                    "--max-size" => "--max-size takes a positive base-ten integer",
                    _ => "the option takes a positive base-ten integer",
                }))
        })
        .transpose()
}

fn duration(text: &str) -> Result<Duration, Failure> {
    let (count, unit) = text.split_at(text.len().saturating_sub(1));
    let count = positive(Some(count), "--older-than")?.ok_or(Failure::Usage(
        "--older-than takes a positive integer and one lowercase unit",
    ))?;
    let seconds = match unit {
        "s" => Some(count),
        "m" => count.checked_mul(60),
        "h" => count.checked_mul(3_600),
        "d" => count.checked_mul(86_400),
        _ => None,
    }
    .ok_or(Failure::Usage(
        "--older-than takes a positive integer and one lowercase unit",
    ))?;
    Ok(Duration::from_secs(seconds))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::duration;

    #[test]
    fn duration_takes_one_positive_decimal_count_and_one_lowercase_unit() {
        for (text, seconds) in [("1s", 1), ("2m", 120), ("3h", 10_800), ("4d", 345_600)] {
            assert_eq!(
                duration(text).expect("valid duration"),
                Duration::from_secs(seconds)
            );
        }
        for text in [
            "",
            "0s",
            "1",
            "s",
            "-1s",
            "1.0s",
            "1S",
            "1ss",
            "18446744073709551615d",
        ] {
            assert!(duration(text).is_err(), "{text}");
        }
    }
}
