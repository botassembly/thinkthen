//! Debug views of parsed addresses never print their raw bytes.

use std::fmt;

use super::Common;
use super::find::FindCommon;

impl fmt::Debug for Common {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Common")
            .field("facts", &self.facts)
            .field("details", &self.details)
            .field("input", &self.input)
            .field("framing", &self.framing())
            .field("field", &self.field)
            .field("dry_run", &self.dry_run)
            .field("url", &self.url.as_ref().map(|_| "<withheld>"))
            .field("profile", &self.profile)
            .field("model", &self.model)
            .field("record", &self.record)
            .field("replay", &self.replay)
            .field("cache", &self.cache)
            .field("no_cache", &self.no_cache)
            .field("refresh_cache", &self.refresh_cache)
            .field("timeout", &self.timeout)
            .field("jobs", &self.jobs)
            .field("max_retries", &self.max_retries)
            .finish()
    }
}

impl fmt::Debug for FindCommon {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FindCommon")
            .field("facts", &self.facts)
            .field("details", &self.details)
            .field("input", &self.input)
            .field("lines", &self.lines)
            .field("jsonl", &self.jsonl)
            .field("field", &self.field)
            .field("dry_run", &self.dry_run)
            .field("url", &self.url.as_ref().map(|_| "<withheld>"))
            .field("profile", &self.profile)
            .field("model", &self.model)
            .field("record", &self.record)
            .field("replay", &self.replay)
            .field("cache", &self.cache)
            .field("no_cache", &self.no_cache)
            .field("refresh_cache", &self.refresh_cache)
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser as _;

    use super::super::Cli;

    #[test]
    #[allow(
        clippy::expect_used,
        reason = "a failed parsed-argument fixture stops the secrecy proof"
    )]
    fn nested_argument_debug_withholds_the_raw_address() {
        for args in [
            vec![
                "thinkthen",
                "decide",
                "a question",
                "--url",
                "http://localhost/debug-secret-0210",
            ],
            vec![
                "thinkthen",
                "find",
                "a question",
                "--url",
                "http://localhost/debug-secret-0210",
            ],
            vec![
                "thinkthen",
                "check",
                "--url",
                "http://localhost/debug-secret-0210",
            ],
        ] {
            let cli = Cli::try_parse_from(args).expect("valid command");
            let shown = format!("{cli:?}");
            assert!(!shown.contains("debug-secret-0210"), "{shown}");
            assert!(shown.contains("<withheld>"), "{shown}");
        }
    }
}
