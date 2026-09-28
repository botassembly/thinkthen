//! Describe one answered batch without altering its request or row shares.

use crate::core::batch::Closed;
use crate::core::{BatchMeta, Setting, Usage};

#[derive(Clone, Copy)]
pub(super) struct Description {
    setting: Setting,
    closed: Closed,
    split: bool,
}

impl Description {
    pub(super) const fn new(setting: Setting, closed: Closed, split: bool) -> Self {
        Self {
            setting,
            closed,
            split,
        }
    }

    pub(super) fn row(
        self,
        records: usize,
        position: usize,
        usage: Option<Usage>,
        requests_sent: u64,
    ) -> Option<BatchMeta> {
        (records > 1 || self.split).then(|| {
            let meta = BatchMeta::new(
                self.setting,
                records,
                position,
                self.closed,
                usage,
                requests_sent,
            );
            if self.split { meta.with_split() } else { meta }
        })
    }
}
