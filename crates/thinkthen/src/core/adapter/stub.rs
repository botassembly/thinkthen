//! Deterministic, typed answers from each ask's identity.

use crate::core::ask::{Ask, AskAnswer};
use crate::core::{Error, Probability};

/// The first eight key bytes, interpreted as an unsigned big-endian integer.
fn number(key: &[u8; 32]) -> u64 {
    key.iter()
        .take(8)
        .fold(0, |value, byte| (value << 8) | u64::from(*byte))
}

#[expect(
    clippy::expect_used,
    reason = "the constructed probabilities are in range and the scale is one-hot"
)]
pub(super) fn answer(key: &[u8; 32], ask: &Ask) -> Result<AskAnswer, Error> {
    let value = number(key);
    match ask {
        Ask::WithState { question, .. } => answer(key, question),
        Ask::YesNo { .. } => Ok(AskAnswer::yes_no(
            Probability::new((value % 101) as f64 / 100.0).expect("bounded probability"),
        )),
        Ask::Choice { options, .. } => Ok(AskAnswer::choice(distribution(value, options.len()))),
        Ask::Scale { levels, .. } => AskAnswer::scale(distribution(value, levels.len())),
    }
}

#[expect(clippy::expect_used, reason = "one-hot probabilities are zero or one")]
fn distribution(value: u64, count: usize) -> Vec<Probability> {
    (0..count)
        .map(|place| {
            Probability::new(if place as u64 == value % count as u64 {
                1.0
            } else {
                0.0
            })
            .expect("one-hot probability")
        })
        .collect()
}
