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
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .finish()
    }
}
