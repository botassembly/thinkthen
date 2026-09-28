//! Explicit cache maintenance commands.

use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Duration;

use crate::cli::args::{PruneArguments, UnusedArguments};
use crate::cli::edge;
use crate::cli::edge::Environment;
use crate::failure::Failure;

const BAD_DIGEST: &str = "--used takes one lowercase 64-character request digest per nonblank line";

pub(crate) fn unused(
    arguments: &UnusedArguments,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    // Parse the whole manifest before opening the named folder.
    let lines = fs::read_to_string(&arguments.used).map_err(|error| {
        if error.kind() == io::ErrorKind::InvalidData {
            Failure::Usage(BAD_DIGEST)
        } else {
            Failure::UsedManifestUnreadable
        }
    })?;
    let mut used = HashSet::new();
    for line in lines.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.len() != 64
            || !line
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(Failure::Usage(BAD_DIGEST));
        }
        used.insert(line.to_owned());
    }
    let names = crate::engine::cache_prune::unused(&arguments.directory, &used)?;
    edge::write_line(
        &mut writer,
        &format!("unused from supplied digests: {}", names.len()),
    )?;
    for name in names {
        edge::write_line(&mut writer, &format!("unused {name}"))?;
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn prune(
    arguments: &PruneArguments,
    environment: &Environment,
    mut writer: impl Write,
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
    let options = crate::engine::cache_prune::Prune {
        max_size,
        older_than,
        answered_by_other_than: arguments.answered_by_other_than.clone(),
    };
    if arguments.dry_run {
        let preview = crate::engine::cache_prune::preview(&arguments.directory, &options)?;
        edge::write_line(
            &mut writer,
            &format!(
                "selected {} entries and {} bytes; {} entries and {} bytes unselected",
                preview.selected_entries,
                preview.selected_bytes,
                preview.unselected_entries,
                preview.unselected_bytes
            ),
        )?;
        for name in preview.selected_names {
            edge::write_line(&mut writer, &format!("selected {name}"))?;
        }
        if preview.temporary_entries > 0 {
            edge::write_line(
                &mut writer,
                &format!(
                    "selected {} temporary files and {} bytes",
                    preview.temporary_entries, preview.temporary_bytes
                ),
            )?;
            for name in preview.temporary_names {
                edge::write_line(&mut writer, &format!("selected temporary {name}"))?;
            }
        }
        diagnostics(preview.bad_names)?;
        return Ok(ExitCode::SUCCESS);
    }
    let result = crate::engine::cache_prune::run(&arguments.directory, &options)?;
    edge::write_line(
        &mut writer,
        &format!(
            "removed {} entries and {} bytes; {} entries and {} bytes remain",
            result.removed_entries,
            result.removed_bytes,
            result.remaining_entries,
            result.remaining_bytes
        ),
    )?;
    if result.temporary_entries > 0 {
        edge::write_line(
            &mut writer,
            &format!(
                "removed {} temporary files and {} bytes",
                result.temporary_entries, result.temporary_bytes
            ),
        )?;
    }
    diagnostics(result.bad_names)?;
    Ok(ExitCode::SUCCESS)
}

fn diagnostics(bad_names: Vec<String>) -> Result<(), Failure> {
    let stderr = std::io::stderr();
    let mut diagnostic = stderr.lock();
    for name in bad_names {
        edge::write_line(
            &mut diagnostic,
            &format!("thinkthen: cache prune: left `{name}` in place; it is not a valid entry"),
        )?;
    }
    Ok(())
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
