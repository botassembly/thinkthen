//! Shared judging-option resolution.

use super::{Common, Framing};

impl Common {
    pub(crate) fn check_plan_name(&self) -> Result<(), crate::failure::Failure> {
        self.check_image_modes()?;
        if self.retired_dry_run {
            return Err(crate::failure::Failure::Usage(
                "--dry-run was renamed --plan",
            ));
        }
        if self.located() && (self.csv || self.tsv) {
            return Err(crate::failure::Failure::Usage(
                "located input needs text or JSONL, not CSV or TSV",
            ));
        }
        if self.unit.is_some() && self.input.is_empty() {
            return Err(crate::failure::Failure::Usage("--unit requires --input"));
        }
        if self.unit.is_some() && (self.jsonl || self.csv || self.tsv || !self.field.is_empty()) {
            return Err(crate::failure::Failure::Usage(
                "--unit needs text without --jsonl, --csv, --tsv or --field",
            ));
        }
        if self.unit.as_deref() == Some("file") && (self.window.is_some() || self.lines) {
            return Err(crate::failure::Failure::Usage(
                "--unit file cannot accompany --window or --lines",
            ));
        }
        Ok(())
    }

    pub(crate) fn located(&self) -> bool {
        self.unit.is_some() || self.window.is_some() || self.input.iter().any(|path| path.is_dir())
    }

    /// The record framing the command line asked for.
    pub(crate) fn framing(&self) -> Framing {
        if self.unit.as_deref() == Some("file") {
            Framing::Document
        } else if self.lines
            || self.window.is_some()
            || self.unit.as_deref() == Some("line")
            || (self.located() && !self.jsonl)
        {
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

impl Common {
    pub(crate) fn images(&self) -> bool {
        !self.image.is_empty() || self.media.as_deref() == Some("image")
    }
    fn check_image_modes(&self) -> Result<(), crate::failure::Failure> {
        use crate::failure::Failure;
        if self.image_media.is_some() && self.image.is_empty() {
            return Err(Failure::Usage(
                "--image-media requires explicit --image attachments",
            ));
        }
        if self.image.len() > crate::public::MAX_IMAGES {
            return Err(Failure::Usage("image evidence requires 1 to 8 images"));
        }
        let framed = self.lines
            || self.jsonl
            || self.csv
            || self.tsv
            || self.window.is_some()
            || !self.field.is_empty();
        if !self.image.is_empty()
            && (self.media.is_some() || self.unit.is_some() || self.window.is_some())
        {
            return Err(Failure::Usage(
                "--image cannot accompany --media, --unit or --window",
            ));
        }
        if self.media.as_deref() == Some("image")
            && (framed || self.input.is_empty() || self.unit.as_deref() != Some("file"))
        {
            return Err(Failure::Usage(
                "--media image requires --input and --unit file without record framing",
            ));
        }
        Ok(())
    }
}
