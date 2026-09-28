//! The final recognized row after its ordered question events.

use crate::public::options::Stop;
use crate::public::results::{ObservedRow, RecordObservation};

use super::Recognized;

pub(super) fn observe_row(stop: &Stop<'_>, row: &Recognized) {
    stop.observe(RecordObservation::Row {
        index: 0,
        value: ObservedRow::Recognized(row),
    });
}
