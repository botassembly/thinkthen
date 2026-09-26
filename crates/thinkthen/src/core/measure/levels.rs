//! Level cuts over a `score` number, and the search that tunes them.
//!
//! A cut is held in hundredths, so a cut of 1.5 is 150. A number on a cut
//! takes the higher level.

use std::cmp::Reverse;

use serde::{Serialize, Serializer};

use crate::core::measure::MeasureError;

/// The most cuts a question of ten levels has.
const MOST: usize = 9;

/// The ascending cuts between the levels of one `score` question, in hundredths.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct LevelCuts {
    cuts: [u32; MOST],
    len: usize,
}

impl LevelCuts {
    /// The cuts halfway between levels, `0.5, 1.5, …`, which read a number as it ran.
    pub(crate) fn midpoints(levels: usize) -> Self {
        let mut cuts = [0; MOST];
        let len = levels.saturating_sub(1).min(MOST);
        for (place, cut) in cuts.iter_mut().take(len).enumerate() {
            *cut = u32::try_from(place).unwrap_or(0) * 100 + 50;
        }
        Self { cuts, len }
    }

    fn at(&self, place: usize) -> u32 {
        self.cuts.get(place).copied().unwrap_or(0)
    }

    fn with(mut self, place: usize, cut: u32) -> Self {
        if let Some(held) = self.cuts.get_mut(place) {
            *held = cut;
        }
        self
    }

    /// The cuts in hundredths.
    pub(crate) fn hundredths(&self) -> &[u32] {
        self.cuts.get(..self.len).unwrap_or_default()
    }
}

impl Serialize for LevelCuts {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let cuts: Vec<f64> = self
            .hundredths()
            .iter()
            .map(|cut| f64::from(*cut) / 100.0)
            .collect();
        cuts.serialize(serializer)
    }
}

/// The zero-based level a number reads as under the cuts.
pub(crate) fn level_at(number: f64, cuts: &LevelCuts) -> usize {
    cuts.hundredths()
        .iter()
        .filter(|cut| number >= f64::from(**cut) / 100.0)
        .count()
}

/// Move each cut in turn, lowest first, to the place between its neighbours
/// that gets the most right answers, until a pass moves nothing.
///
/// A move needs strictly more right answers. A tie between places goes to
/// the one nearest the current cut, then the smaller.
///
/// # Errors
///
/// Returns the error `right` returns.
pub(crate) fn search(
    levels: usize,
    right: impl Fn(&LevelCuts) -> Result<usize, MeasureError>,
) -> Result<LevelCuts, MeasureError> {
    let mut cuts = LevelCuts::midpoints(levels);
    let top = u32::try_from(cuts.len).unwrap_or(0) * 100;
    loop {
        let mut moved = false;
        for place in 0..cuts.len {
            let low = place.checked_sub(1).map_or(0, |below| cuts.at(below)) + 1;
            let high = if place + 1 < cuts.len {
                cuts.at(place + 1)
            } else {
                top
            } - 1;
            let now = cuts.at(place);
            let current = right(&cuts)?;
            let scores = (low..=high)
                .map(|candidate| Ok((right(&cuts.with(place, candidate))?, candidate)))
                .collect::<Result<Vec<_>, MeasureError>>()?;
            let best = scores.into_iter().max_by_key(|(score, candidate)| {
                (
                    *score,
                    Reverse(candidate.abs_diff(now)),
                    Reverse(*candidate),
                )
            });
            if let Some((score, candidate)) = best
                && score > current
            {
                cuts = cuts.with(place, candidate);
                moved = true;
            }
        }
        if !moved {
            return Ok(cuts);
        }
    }
}
