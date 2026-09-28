//! The final relation row after its ordered question events.

use crate::public::options::Stop;
use crate::public::results::{ObservedRow, RecordObservation};

use super::Edge;

pub(super) fn observe_row(stop: &Stop<'_>, edges: &[Edge]) {
    stop.observe(RecordObservation::Row {
        index: 0,
        value: ObservedRow::Relations(edges),
    });
}
