//! The local acceptance policy: a symmetric pass mark, or none at all.

use crate::pass_mark::PassMark;

/// What local policy will do with an answer.
///
/// No pass mark is built in. A pass mark is a measurement for one model, so a
/// user who names none gets an unassessed result rather than a guess.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Policy {
    /// The user named no pass mark, so nothing is accepted.
    Unassessed,
    /// The user named one mark, and it governs a yes and a no alike.
    Symmetric(PassMark),
}

impl Policy {
    /// Read the pass mark back, or `None` when the user named none.
    #[must_use]
    pub const fn min_prob(self) -> Option<PassMark> {
        match self {
            Self::Unassessed => None,
            Self::Symmetric(mark) => Some(mark),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Policy;
    use crate::pass_mark::PassMark;

    #[test]
    fn a_policy_reports_the_mark_it_carries() {
        let mark = PassMark::new(0.9).expect("inside the range");
        assert_eq!(Policy::Symmetric(mark).min_prob(), Some(mark));
        assert_eq!(Policy::Unassessed.min_prob(), None);
    }
}
