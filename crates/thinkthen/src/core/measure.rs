//! The statistics `audit` grades with, as the prototype measurement script defines them.
//!
//! Every function here reproduces one function of that script bit for bit where
//! the golden files can tell, so the generator, the sums, and the rounding
//! follow Python's own arithmetic.

pub(crate) mod answer;
pub(crate) mod audit;
pub(crate) mod diff;
pub(crate) mod key;

use serde::{Serialize, Serializer};

use crate::core::json::Json;

/// The two-sided 95% normal quantile.
const Z: f64 = 1.959_963_984_540_054;
/// The number of equal calibration bins over zero to one.
pub(crate) const BINS: usize = 10;
/// The number of bootstrap resamples behind a calibration interval.
pub(crate) const DRAWS: usize = 1000;
/// What a calibration interval does and does not cover.
pub(crate) const NOTE: &str = "the interval resamples the records; it does not cover rerun noise, so compare two runs of the same records";

/// Why saved answers or a key cannot be graded. A line number is one-based.
///
/// No variant carries a record, an id, a key value, or a path. The command
/// writes each sentence, because the sentence names the command and the input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MeasureError {
    /// A line is not a UTF-8 JSON object.
    NotObject(usize),
    /// A result line has no string or integer id at the pointer.
    NoId(usize),
    /// A line holds an answer of a verb other than `decide` and `choose`.
    Ungradable(usize),
    /// A probability is outside zero to one, or a distribution is empty.
    Probability(usize),
    /// One question holds one record twice.
    Repeated(usize),
    /// A key line lacks an id or a value, repeats an id, or names another part.
    KeyLine(usize),
    /// A key gives a `choose` answer a value that is not text.
    ChooseKey(usize),
    /// Some labeled records carry a part and others do not.
    MixedParts,
    /// One group holds `decide` and `choose` answers.
    TwoVerbs,
    /// A rule was named over answers saved without probabilities.
    NeedsProbabilities,
    /// A band was named over a `choose` answer.
    BandOnChoose,
}

/// One numbered JSON line of an input.
pub(crate) type Line = (usize, Json);

/// Every nonblank line of an input as a JSON object, with its one-based number.
///
/// # Errors
///
/// Returns [`MeasureError::NotObject`] for the first line that is not a UTF-8 JSON object.
pub(crate) fn json_lines(bytes: &[u8]) -> Result<Vec<(usize, Json)>, MeasureError> {
    let mut lines = Vec::new();
    for (place, raw) in bytes.split(|byte| *byte == b'\n').enumerate() {
        let refused = MeasureError::NotObject(place + 1);
        let text = std::str::from_utf8(raw).map_err(|_| refused)?;
        if text.trim().is_empty() {
            continue;
        }
        match Json::parse(text) {
            Ok(row @ Json::Object(_)) => lines.push((place + 1, row)),
            _ => return Err(refused),
        }
    }
    Ok(lines)
}

/// The record id a JSON value names: a string as is, an integer as its decimal text.
pub(crate) fn record_id(value: &Json) -> Option<String> {
    match value {
        Json::String(text) => Some(text.clone()),
        Json::Number(number) if number.is_i64() || number.is_u64() => Some(number.to_string()),
        _ => None,
    }
}

/// SplitMix64, which every language computes bit for bit alike.
#[derive(Clone, Debug)]
pub(crate) struct SplitMix64(u64);

impl SplitMix64 {
    /// Start the generator at a seed.
    pub(crate) const fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// The next 64-bit output.
    pub(crate) const fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A draw from 0 to `n - 1`: the high 64 bits of the next output times `n`.
    pub(crate) fn index(&mut self, n: usize) -> usize {
        let wide = u128::from(self.next()) * n as u128;
        usize::try_from(wide >> 64).unwrap_or(0)
    }
}

/// Fisher-Yates from the last place down.
pub(crate) fn shuffle<T>(items: &mut [T], generator: &mut SplitMix64) {
    for place in (1..items.len()).rev() {
        let other = generator.index(place + 1);
        items.swap(place, other);
    }
}

/// The Wilson interval at 95% for `right` of `answered`, or `None` for no answers.
pub(crate) fn wilson(right: usize, answered: usize) -> Option<[f64; 2]> {
    if answered == 0 {
        return None;
    }
    let n = answered as f64;
    let p = right as f64 / n;
    let square = Z * Z;
    let centre = (p + square / (2.0 * n)) / (1.0 + square / n);
    let half = Z * (p * (1.0 - p) / n + square / (4.0 * n * n)).sqrt() / (1.0 + square / n);
    Some([(centre - half).max(0.0), (centre + half).min(1.0)])
}

/// The chance a true case outranks a false one, ties counting half.
pub(crate) fn auc(pairs: &[(f64, bool)]) -> Option<f64> {
    let yes: Vec<f64> = pairs
        .iter()
        .filter(|pair| pair.1)
        .map(|pair| pair.0)
        .collect();
    let no: Vec<f64> = pairs
        .iter()
        .filter(|pair| !pair.1)
        .map(|pair| pair.0)
        .collect();
    if yes.is_empty() || no.is_empty() {
        return None;
    }
    let mut wins = 0.0;
    for x in &yes {
        for y in &no {
            wins += if x > y {
                1.0
            } else if x == y {
                0.5
            } else {
                0.0
            };
        }
    }
    Some(wins / (yes.len() * no.len()) as f64)
}

/// Python's `sum` over floats, which compensates each step as Neumaier does.
pub(crate) fn python_sum(values: impl IntoIterator<Item = f64>) -> f64 {
    let (mut total, mut compensation) = (0.0_f64, 0.0_f64);
    for value in values {
        let next = total + value;
        compensation += if total.abs() >= value.abs() {
            (total - next) + value
        } else {
            (value - next) + total
        };
        total = next;
    }
    if compensation != 0.0 && compensation.is_finite() {
        total += compensation;
    }
    total
}

/// The binned calibration error: ten equal bins, the last closed at one.
pub(crate) fn calibration_error(pairs: &[(f64, bool)]) -> f64 {
    let mut gap = 0.0;
    for bin in 0..BINS {
        let low = bin as f64 / BINS as f64;
        let high = (bin + 1) as f64 / BINS as f64;
        let inside = |x: f64| (low <= x && x < high) || (bin == BINS - 1 && x == 1.0);
        let held: Vec<&(f64, bool)> = pairs.iter().filter(|pair| inside(pair.0)).collect();
        let trues = held.iter().filter(|pair| pair.1).count() as f64;
        gap += (trues - python_sum(held.iter().map(|pair| pair.0))).abs();
    }
    gap / pairs.len() as f64
}

/// Linear interpolation between the order statistics of sorted values.
pub(crate) fn quantile(sorted: &[f64], q: f64) -> f64 {
    let position = (sorted.len().saturating_sub(1)) as f64 * q;
    let place = position.floor();
    let at = |index: usize| sorted.get(index).copied().unwrap_or(0.0);
    let index = place as usize;
    match sorted.get(index + 1) {
        Some(next) => at(index) + (position - place) * (next - at(index)),
        None => at(index),
    }
}

/// The calibration error and its bootstrap interval.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Calibration {
    /// The error over every pair.
    #[serde(serialize_with = "six")]
    pub(crate) error: f64,
    /// The 2.5% and 97.5% quantiles of the resampled errors.
    #[serde(serialize_with = "six")]
    pub(crate) interval: [f64; 2],
    /// Always [`BINS`].
    pub(crate) bins: usize,
    /// Always [`DRAWS`].
    pub(crate) draws: usize,
    /// The seed of this group's generator.
    pub(crate) seed: u64,
    /// Always [`NOTE`].
    pub(crate) note: &'static str,
}

/// The calibration of these pairs with a fresh generator at the seed.
pub(crate) fn calibration(pairs: &[(f64, bool)], seed: u64) -> Option<Calibration> {
    if pairs.is_empty() {
        return None;
    }
    let mut generator = SplitMix64::new(seed);
    let mut errors: Vec<f64> = (0..DRAWS)
        .map(|_| {
            let drawn: Vec<(f64, bool)> = (0..pairs.len())
                .filter_map(|_| pairs.get(generator.index(pairs.len())).copied())
                .collect();
            calibration_error(&drawn)
        })
        .collect();
    errors.sort_by(f64::total_cmp);
    Some(Calibration {
        error: calibration_error(pairs),
        interval: [quantile(&errors, 0.025), quantile(&errors, 0.975)],
        bins: BINS,
        draws: DRAWS,
        seed,
        note: NOTE,
    })
}

/// The exact two-sided McNemar test: the binomial test of the discordant pairs at one half.
///
/// The terms are summed in log space, so any count works without big integers.
pub(crate) fn mcnemar(a: usize, b: usize) -> f64 {
    let n = a + b;
    let mut term = -(n as f64) * std::f64::consts::LN_2;
    let mut tail = term.exp();
    for i in 0..a.min(b) {
        term += ((n - i) as f64 / (i + 1) as f64).ln();
        tail += term.exp();
    }
    (2.0 * tail).min(1.0)
}

/// A float rounded to six places, as Python's `round(x, 6)` does.
pub(crate) fn rounded(value: f64) -> f64 {
    format!("{value:.6}").parse().unwrap_or(value)
}

/// A float with this many decimals, rounded half to even on its exact binary value, or `-` for none.
pub(crate) fn places(value: Option<f64>, digits: usize) -> String {
    value.map_or_else(|| "-".to_owned(), |value| format!("{value:.digits$}"))
}

/// A float as Python's `str` writes it: the shortest text that reads back,
/// with `.0` on a whole number and an exponent below 1e-4 or from 1e16.
pub(crate) fn python_float_text(value: f64) -> String {
    let scientific = format!("{value:e}");
    let (digits, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let exponent: i32 = exponent.parse().unwrap_or(0);
    if (-4..16).contains(&exponent) {
        let plain = value.to_string();
        return if plain.contains('.') {
            plain
        } else {
            format!("{plain}.0")
        };
    }
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{digits}e{sign}{:02}", exponent.abs())
}

/// A value whose floats print rounded to six places.
pub(crate) trait Rounded {
    /// The value with every float rounded.
    fn rounded(&self) -> Self;
}

impl Rounded for f64 {
    fn rounded(&self) -> Self {
        rounded(*self)
    }
}

impl<T: Rounded> Rounded for Option<T> {
    fn rounded(&self) -> Self {
        self.as_ref().map(T::rounded)
    }
}

impl<T: Rounded> Rounded for [T; 2] {
    fn rounded(&self) -> Self {
        self.each_ref().map(T::rounded)
    }
}

/// Write a value with its floats rounded to six places.
pub(crate) fn six<T: Rounded + Serialize, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    value.rounded().serialize(serializer)
}

#[cfg(test)]
#[path = "measure/tests.rs"]
mod tests;
