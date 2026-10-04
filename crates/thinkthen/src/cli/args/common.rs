//! Shared judging-option resolution.

use super::{Common, Framing};

impl Common {
    pub(crate) fn check_plan_name(&self) -> Result<(), crate::failure::Failure> {
        if self.retired_dry_run {
            return Err(crate::failure::Failure::Usage(
                "--dry-run was renamed --plan",
            ));
        }
        Ok(())
    }

    /// The record framing the command line asked for.
    pub(crate) const fn framing(&self) -> Framing {
        if self.lines || self.window.is_some() {
            Framing::Lines
        } else if self.jsonl {
            Framing::Jsonl
        } else if self.csv {
            Framing::Csv
        } else if self.tsv {
            Framing::Tsv
        } else {
            Framing::Document
        }
    }
}
