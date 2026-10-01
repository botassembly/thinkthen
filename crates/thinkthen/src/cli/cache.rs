//! Explicit cache maintenance commands.

use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Duration;

use crate::cli::args::{ConvertArguments, PruneArguments, UnusedArguments};
use crate::cli::edge;
use crate::cli::edge::Environment;
use crate::failure::Failure;

const BAD_DIGEST: &str = "--used takes one lowercase 64-character question key per nonblank line";

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
    let keys = crate::engine::store::unused(&arguments.directory, &used)?;
    edge::write_line(
        &mut writer,
        &format!("unused from supplied keys: {}", keys.len()),
    )?;
    for key in keys {
        edge::write_line(&mut writer, &format!("unused {key}"))?;
    }
    Ok(ExitCode::SUCCESS)
}

/// Merge a folder into its fixture, and say on standard error what was
/// written and each old entry that was skipped.
pub(crate) fn convert(arguments: &ConvertArguments) -> Result<ExitCode, Failure> {
    if !fs::metadata(&arguments.directory).is_ok_and(|metadata| metadata.is_dir()) {
        return Err(Failure::ConvertFolder);
    }
    let summary = crate::engine::store::convert(&arguments.directory, arguments.quote)?;
    let stderr = std::io::stderr();
    let mut diagnostic = stderr.lock();
    for (name, why) in &summary.skipped {
        edge::write_line(
            &mut diagnostic,
            &format!("thinkthen: cache convert: skipped `{name}`; {why}"),
        )?;
    }
    edge::write_line(
        &mut diagnostic,
        &format!(
            "thinkthen: cache convert: wrote {} answers to thinkthen.jsonl, {} from old entries; skipped {} entries",
            summary.answers,
            summary.converted,
            summary.skipped.len()
        ),
    )?;
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn prune(
    arguments: &PruneArguments,
    environment: &Environment,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let max_size =
        max_size(arguments.max_size.as_deref())?.unwrap_or_else(|| environment.cache_bytes());
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
    let options = crate::engine::store::Prune {
        max_size,
        older_than,
        answered_by_other_than: arguments.answered_by_other_than.clone(),
    };
    let now = crate::engine::store::now();
    if arguments.dry_run {
        let preview = crate::engine::store::preview(&arguments.directory, &options, now)?;
        edge::write_line(
            &mut writer,
            &format!(
                "selected {} answers and {} bytes; {} answers and {} bytes unselected",
                preview.keys.len(),
                preview.bytes,
                preview.kept,
                preview.kept_bytes
            ),
        )?;
        for key in &preview.keys {
            edge::write_line(&mut writer, &format!("selected {}", crate::core::hex(key)))?;
        }
        return Ok(ExitCode::SUCCESS);
    }
    let result = crate::engine::store::prune(&arguments.directory, &options, now)?;
    edge::write_line(
        &mut writer,
        &format!(
            "removed {} answers and {} bytes; {} answers and {} bytes remain",
            result.removed, result.removed_bytes, result.remaining, result.remaining_bytes
        ),
    )?;
    Ok(ExitCode::SUCCESS)
}

fn max_size(value: Option<&str>) -> Result<Option<u64>, Failure> {
    value
        .map(|text| {
            text.parse::<u64>()
                .ok()
                .filter(|number| *number > 0)
                .ok_or(Failure::Usage(
                    "--max-size takes a positive base-ten integer",
                ))
        })
        .transpose()
}

fn duration(text: &str) -> Result<Duration, Failure> {
    let usage = Failure::Usage("--older-than takes a positive integer and one lowercase unit");
    let mut characters = text.chars();
    let unit = characters.next_back();
    let Some(count) = characters
        .as_str()
        .parse::<u64>()
        .ok()
        .filter(|count| *count > 0)
    else {
        return Err(usage);
    };
    let seconds = match unit {
        Some('s') => Some(count),
        Some('m') => count.checked_mul(60),
        Some('h') => count.checked_mul(3_600),
        Some('d') => count.checked_mul(86_400),
        _ => None,
    }
    .ok_or(usage)?;
    Ok(Duration::from_secs(seconds))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::duration;
    use crate::failure::Failure;

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
            "5\u{e9}",
            "\u{e9}",
            "\u{e9}5s",
            "5\u{1f600}",
            "\u{663}s",
        ] {
            assert!(
                matches!(
                    duration(text),
                    Err(Failure::Usage(
                        "--older-than takes a positive integer and one lowercase unit"
                    ))
                ),
                "{text}"
            );
        }
    }
}
