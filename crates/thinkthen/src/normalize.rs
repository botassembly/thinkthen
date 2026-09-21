//! The one pre-Clap correction for spaced negative threshold values.

use std::ffi::OsString;

/// Join a spaced negative threshold to its option before Clap sees it.
///
/// Clap recognizes some leading-minus floats and splits others as short
/// options. Only a complete Rust `f64` token, or two such tokens separated by
/// one colon, is joined. Every other token keeps Clap's established meaning.
pub(crate) fn arguments(arguments: impl IntoIterator<Item = OsString>) -> Vec<OsString> {
    let mut source = arguments.into_iter().peekable();
    let mut normalized = Vec::new();
    while let Some(argument) = source.next() {
        if argument == "--" {
            normalized.push(argument);
            normalized.extend(source);
            break;
        }
        if argument == "--threshold"
            && let Some(value) = source
                .peek()
                .and_then(|value| value.to_str())
                .filter(|value| negative_threshold(value))
        {
            normalized.push(OsString::from(format!("--threshold={value}")));
            let _joined = source.next();
        } else {
            normalized.push(argument);
        }
    }
    normalized
}

fn negative_threshold(value: &str) -> bool {
    value.starts_with('-')
        && match value.split_once(':') {
            Some((low, high)) => !high.contains(':') && lexical_float(low) && lexical_float(high),
            None => lexical_float(value),
        }
}

fn lexical_float(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::arguments;
    use std::ffi::OsString;

    #[test]
    fn only_a_whole_negative_float_after_threshold_is_joined_before_clap() {
        let cases = [
            ("-0.5", true),
            ("-.5", true),
            ("-1e-1", true),
            ("-inf", true),
            ("-0.2:-1e-1", true),
            ("--dry-run", false),
            ("--bogus", false),
            ("-word", false),
            ("-.config", false),
            ("-infamous", false),
            ("-nanosecond", false),
        ];
        for (value, joined) in cases {
            let got = arguments(
                ["thinkthen", "decide", "q", "--threshold", value]
                    .into_iter()
                    .map(OsString::from),
            );
            assert_eq!(got.len(), if joined { 4 } else { 5 }, "{value}");
            if joined {
                assert_eq!(got[3], OsString::from(format!("--threshold={value}")));
            }
        }

        let after_marker = arguments(
            ["thinkthen", "decide", "--", "--threshold", "-.5"]
                .into_iter()
                .map(OsString::from),
        );
        assert_eq!(after_marker[4], "-.5");
    }
}
