//! A stream's detached interruption receipt, settled by its native worker.

use std::sync::Arc;

use super::{Failure, ReceiptState, Terminal};

pub(crate) fn stream_receipt() -> Arc<ReceiptState> {
    Arc::new(ReceiptState::default())
}

pub(crate) fn finish_stream_receipt(
    receipt: &Arc<ReceiptState>,
    facts: Option<&thinkthen::Facts>,
    failure: Option<Failure>,
    panicked: bool,
) {
    receipt.finish(Terminal {
        outcome: if panicked {
            "panicked"
        } else if failure.is_some() {
            "failed"
        } else {
            "succeeded"
        },
        facts: facts.cloned(),
        details: None,
        failure,
    });
}
