//! The step-1 decode and the span score over the five BILOU tags.

/// The five tags in table order. On equal scores the earlier tag wins.
pub(crate) const TAGS: [&str; 5] = ["BEGIN", "INSIDE", "END", "SINGLE", "OUT"];

const BEGIN: usize = 0;
const INSIDE: usize = 1;
const END: usize = 2;
const SINGLE: usize = 3;

/// A probability under one in a million counts as one in a million.
const FLOOR: f64 = 1e-6;

/// One piece's five tag probabilities, in table order.
pub(crate) type TagRow = [f64; 5];

/// Whether tag `next` may follow tag `previous`, or open the text when `None`.
fn follows(previous: Option<usize>, next: usize) -> bool {
    let inside_name = matches!(previous, Some(BEGIN | INSIDE));
    inside_name == matches!(next, INSIDE | END)
}

/// Whether a text may end on `tag`.
const fn closes(tag: usize) -> bool {
    !matches!(tag, BEGIN | INSIDE)
}

fn weight(row: &TagRow, tag: usize) -> f64 {
    row.get(tag).copied().unwrap_or(0.0).max(FLOOR).ln()
}

/// The pieces of each name on the most likely valid tag sequence, as
/// inclusive `(first, last)` piece places.
pub(crate) fn decode(rows: &[TagRow]) -> Vec<(usize, usize)> {
    let mut scores: Vec<[f64; 5]> = Vec::with_capacity(rows.len());
    let mut back: Vec<[usize; 5]> = Vec::with_capacity(rows.len());
    for row in rows {
        let previous = scores.last().copied();
        let mut score = [f64::NEG_INFINITY; 5];
        let mut from = [0; 5];
        for (tag, (slot, pointer)) in score.iter_mut().zip(from.iter_mut()).enumerate() {
            let best = match previous {
                None => follows(None, tag).then_some((0, 0.0)),
                Some(held) => best_of(
                    held.iter()
                        .enumerate()
                        .filter(|(before, _)| follows(Some(*before), tag))
                        .map(|(before, value)| (before, *value)),
                ),
            };
            if let Some((before, value)) = best {
                *slot = value + weight(row, tag);
                *pointer = before;
            }
        }
        scores.push(score);
        back.push(from);
    }
    let Some(last) = scores.last() else {
        return Vec::new();
    };
    let Some((mut tag, _)) = best_of(
        last.iter()
            .enumerate()
            .filter(|(tag, _)| closes(*tag))
            .map(|(tag, value)| (tag, *value)),
    ) else {
        return Vec::new();
    };
    let mut tags = vec![0; rows.len()];
    for (place, pointers) in back.iter().enumerate().rev() {
        if let Some(slot) = tags.get_mut(place) {
            *slot = tag;
        }
        tag = pointers.get(tag).copied().unwrap_or(0);
    }
    stretches(&tags)
}

/// The highest value, the earliest on a tie.
fn best_of(values: impl Iterator<Item = (usize, f64)>) -> Option<(usize, f64)> {
    values.fold(None, |best, (tag, value)| match best {
        Some((_, held)) if held >= value => best,
        _ => Some((tag, value)),
    })
}

fn stretches(tags: &[usize]) -> Vec<(usize, usize)> {
    let mut names = Vec::new();
    let mut first = 0;
    for (place, tag) in tags.iter().enumerate() {
        match *tag {
            BEGIN => first = place,
            END => names.push((first, place)),
            SINGLE => names.push((place, place)),
            _ => {}
        }
    }
    names
}

/// The forward and backward sums over every valid tag sequence, in log space.
pub(crate) struct SpanOdds<'a> {
    rows: &'a [TagRow],
    forward: Vec<[f64; 5]>,
    backward: Vec<[f64; 5]>,
    total: f64,
}

impl<'a> SpanOdds<'a> {
    /// One forward-backward pass over the floored tag probabilities.
    pub(crate) fn new(rows: &'a [TagRow]) -> Self {
        let mut forward: Vec<[f64; 5]> = Vec::with_capacity(rows.len());
        for row in rows {
            let previous = forward.last().copied();
            forward.push(std::array::from_fn(|tag| match previous {
                None if follows(None, tag) => weight(row, tag),
                None => f64::NEG_INFINITY,
                Some(held) => {
                    weight(row, tag)
                        + log_sum(
                            held.iter()
                                .enumerate()
                                .filter(|(before, _)| follows(Some(*before), tag))
                                .map(|(_, value)| *value),
                        )
                }
            }));
        }
        let mut backward: Vec<[f64; 5]> = Vec::with_capacity(rows.len());
        for place in (0..rows.len()).rev() {
            let row = rows.get(place + 1);
            let after = backward.last().copied();
            backward.push(std::array::from_fn(|tag| match (row, after) {
                (Some(row), Some(after)) => log_sum(
                    after
                        .iter()
                        .enumerate()
                        .filter(|(next, _)| follows(Some(tag), *next))
                        .map(|(next, value)| weight(row, next) + value),
                ),
                _ if closes(tag) => 0.0,
                _ => f64::NEG_INFINITY,
            }));
        }
        backward.reverse();
        let total = log_sum(
            forward
                .last()
                .iter()
                .flat_map(|last| last.iter().enumerate())
                .filter(|(tag, _)| closes(*tag))
                .map(|(_, value)| *value),
        );
        Self {
            rows,
            forward,
            backward,
            total,
        }
    }

    /// P(span): the weight of the valid paths that tag exactly pieces `first`
    /// to `last` as one name, over the weight of all valid paths.
    pub(crate) fn span(&self, first: usize, last: usize) -> f64 {
        let before = match first
            .checked_sub(1)
            .and_then(|place| self.forward.get(place))
        {
            None => 0.0,
            Some(held) => log_sum(
                held.iter()
                    .enumerate()
                    .filter(|(tag, _)| closes(*tag))
                    .map(|(_, value)| *value),
            ),
        };
        let row = |place: usize, tag: usize| {
            self.rows
                .get(place)
                .map_or(f64::NEG_INFINITY, |row| weight(row, tag))
        };
        let inner = if first == last {
            row(first, SINGLE)
        } else {
            row(first, BEGIN)
                + (first + 1..last)
                    .map(|place| row(place, INSIDE))
                    .sum::<f64>()
                + row(last, END)
        };
        let closing = if first == last { SINGLE } else { END };
        let after = self
            .backward
            .get(last)
            .and_then(|held| held.get(closing))
            .copied()
            .unwrap_or(f64::NEG_INFINITY);
        (before + inner + after - self.total).exp()
    }
}

fn log_sum(values: impl Iterator<Item = f64>) -> f64 {
    let values: Vec<f64> = values.filter(|value| value.is_finite()).collect();
    let Some(most) = values.iter().copied().reduce(f64::max) else {
        return f64::NEG_INFINITY;
    };
    most + values
        .iter()
        .map(|value| (value - most).exp())
        .sum::<f64>()
        .ln()
}
