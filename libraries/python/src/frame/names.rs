//! Recognized names as an Arrow table.

use crate::arrow::{self, Cells};
use thinkthen::RecognizedEntity;

/// The recognized names as a table: row, text, start, end, length, kind,
/// strength.
pub(super) fn names(found: Vec<Option<Vec<RecognizedEntity>>>) -> Result<arrow::Output, String> {
    let wide = |at: usize| i64::try_from(at).unwrap_or(i64::MAX);
    let (mut row, mut text, mut kind) = (Vec::new(), Vec::new(), Vec::new());
    let (mut start, mut end, mut length) = (Vec::new(), Vec::new(), Vec::new());
    let mut strength = Vec::new();
    let found = (1_i64..)
        .zip(found)
        .flat_map(|(place, row)| row.into_iter().flatten().map(move |one| (place, one)));
    for (place, one) in found {
        row.push(place);
        text.push(Some(one.text().to_owned()));
        start.push(wide(one.start()));
        end.push(wide(one.end()));
        length.push(wide(one.length()));
        kind.push(Some(one.kind().to_owned()));
        strength.push(Some(one.strength()));
    }
    arrow::table(&[
        ("row", Cells::Counts(row)),
        ("text", Cells::Texts(text)),
        ("start", Cells::Counts(start)),
        ("end", Cells::Counts(end)),
        ("length", Cells::Counts(length)),
        ("kind", Cells::Texts(kind)),
        ("strength", Cells::Numbers(strength)),
    ])
}
